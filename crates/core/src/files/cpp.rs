use ligen::generator::{ImplementationVisitor, FileProcessorVisitor, Context, FileSet, FunctionVisitor, ParameterVisitor};
use ligen::ir;
use std::path::PathBuf;
use ligen_c::ast::{Types, Type};

/// Implementation processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ImplementationProcessor;

/// Function processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct FunctionProcessor;

/// Parameter processor.
#[derive(Default, Clone, Copy, Debug)]
pub struct ParameterProcessor;

fn path(implementation: &ImplementationVisitor) -> PathBuf {
    PathBuf::from("src").join(format!("{}.cpp", implementation.current.self_.name))
}

impl FileProcessorVisitor for ImplementationProcessor {
    type Visitor = ImplementationVisitor;

    fn process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor));
        let name = &visitor.current.self_.name;
        let name_lower = name.to_lowercase();
        // includes
        file.writeln(format!("#include <{}.hpp>", name));
        file.writeln("");

        file.writeln(format!("{name}::~{name}() {{", name = name));
        file.writeln(format!("\t{}_drop(*this);", name));
        file.writeln("}");

        file.writeln(format!("{name}::{name}(C{name} {name_lower}) {{", name = name, name_lower = name_lower));
        file.writeln(format!("\tself = {name_lower}.self;", name_lower = name_lower));
        file.writeln("}");
    }

    fn post_process(&self, _context: &Context, _file_set: &mut FileSet, _visitor: &Self::Visitor) {}
}

impl FunctionProcessor {
    /// Generate function name.
    pub fn generate_function_name(&self, visitor: &FunctionVisitor) -> String {
        let object_name = &visitor.parent.current.self_.name;
        let function_name = &visitor.current.identifier.name;
        format!("{}::{}", object_name, function_name)
    }

    /// Generate function output.
    pub fn generate_function_output(&self, output: &Option<ir::Type>) -> String {
        let type_ = output
            .as_ref()
            .map(|type_| {
                let typ_ = Type::from(type_.clone());
                if let Types::Compound(compound) = typ_.type_ {
                    match compound.name.as_str() {
                        "String" => "RString".to_string(),
                        _ => Type::from(type_.clone()).to_string(),
                    }
                } else {
                    Type::from(type_.clone()).to_string()
                }
            })
            .unwrap_or_else(|| "void".into());
        format!("{} ", type_)
    }
}

impl FileProcessorVisitor for FunctionProcessor {
    type Visitor = FunctionVisitor;

    fn process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        if let ir::Visibility::Public = visitor.current.visibility {
            let file = file_set.entry(&path(&visitor.parent));
            if visitor.current.identifier.name == "new" {
                let name = &visitor.parent.current.self_.name;
                file.write(format!("{name}::{name}", name = name));
            } else {
                file.write(self.generate_function_output(&visitor.current.output));
                file.write(self.generate_function_name(&visitor));
            }
            file.write("(");
        }
    }

    fn post_process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        if let ir::Visibility::Public = visitor.current.visibility {
            let object_name = &visitor.parent.current.self_.name;
            let function_name = &visitor.current.identifier.name;
            let arguments = visitor
                .current
                .inputs
                .iter()
                .cloned()
                .map(|input| input.identifier.name)
                .map(|input| {
                    if *input == object_name.to_lowercase() {
                        "*this".into()
                    } else {
                        input
                    }
                })
                .collect::<Vec<String>>()
                .join(", ");
            let file = file_set.entry(&path(&visitor.parent));
            if visitor.current.identifier.name == "new" {
                file.write(format!(") : {}(", object_name));
                file.write(format!("{}_{}({})", object_name, function_name, arguments));
                file.writeln(") {");
            } else {
                file.writeln(") {");
                file.write("\t");
                if visitor.current.output.is_some() {
                    file.write("return ");
                }
                file.writeln(format!("{}_{}({});", object_name, function_name, arguments));
            }
            file.writeln("}");
        }
    }
}

impl FileProcessorVisitor for ParameterProcessor {
    type Visitor = ParameterVisitor;

    fn process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let object_name = &visitor.parent.parent.current.self_.name;
        // FIXME: Find a better way to identify if the function is a method.
        if visitor.current.identifier.name != object_name.to_lowercase() {
            let file = file_set.entry(&path(&visitor.parent.parent));

            let mut type_ = Type::from(visitor.current.type_.clone());
            if let (Some(_pointer), Types::Compound(_type)) = (&type_.pointer, &type_.type_) {
                type_.pointer = None;
            }
            let ident = &visitor.current.identifier.name;
            file.write(format!("{} {}", type_, ident))
        }
    }

    fn post_process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let object_name = &visitor.parent.parent.current.self_.name;
        // FIXME: Find a better way to identify if the function is a method.
        if visitor.current.identifier.name != object_name.to_lowercase() {
            let file = file_set.entry(&path(&visitor.parent.parent));
            file.write(", ");
        }
    }
}
