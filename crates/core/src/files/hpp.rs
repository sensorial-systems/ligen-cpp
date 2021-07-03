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
    PathBuf::from("include").join(format!("{}.hpp", implementation.current.self_.name))
}

impl FileProcessorVisitor for ImplementationProcessor {
    type Visitor = ImplementationVisitor;

    fn process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor));
        let name = &visitor.current.self_.name;
        // includes
        file.writeln("#pragma once");
        file.writeln("");
        file.writeln("#include <RString.hpp>");
        file.writeln("#include <stdint.h>");
        file.writeln(format!("#define {name} C{name}", name = name));
        file.writeln(format!("#include <{}.h>", name));
        file.writeln(format!("#undef {}", name));
        file.writeln("");

        // class
        file.writeln(format!("class {name}: public C{name} {{", name = name));
        file.writeln("public:");
        file.writeln(format!("\t~{}();", name));
        file.writeln(format!("\t{name}(C{name} {name_lower});", name = name, name_lower = name.to_lowercase()));
    }

    fn post_process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        let file = file_set.entry(&path(&visitor));

        file.writeln("};");
    }
}

impl FunctionProcessor {
    /// Generate function name.
    pub fn generate_function_name(&self, visitor: &FunctionVisitor) -> String {
        visitor.current.identifier.name.clone()
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
            file.write("\t");
            if visitor.current.identifier.name == "new" {
                file.write(&visitor.parent.current.self_.name);
            } else {
                file.write(self.generate_function_output(&visitor.current.output));
                file.write(self.generate_function_name(&visitor));
            }
            file.write("(");
        }
    }

    fn post_process(&self, _context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        if let ir::Visibility::Public = visitor.current.visibility {
            let file = file_set.entry(&path(&visitor.parent));
            file.writeln(");");
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
