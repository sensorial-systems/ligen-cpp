use ligen::generator::{FileProcessorVisitor, ImplementationVisitor, FunctionVisitor, ParameterVisitor, FileGeneratorVisitors, Context, FileSet};
use crate::Generator;

mod hpp;
mod cpp;

#[derive(Default, Debug, Clone)]
pub struct ImplementationProcessor {
    pub hpp: hpp::ImplementationProcessor,
    pub cpp: cpp::ImplementationProcessor
}

#[derive(Default, Debug, Clone)]
pub struct FunctionProcessor {
    pub hpp: hpp::FunctionProcessor,
    pub cpp: cpp::FunctionProcessor
}

#[derive(Default, Debug, Clone)]
pub struct ParameterProcessor {
    pub hpp: hpp::ParameterProcessor,
    pub cpp: cpp::ParameterProcessor
}

impl FileProcessorVisitor for ImplementationProcessor {
    type Visitor = ImplementationVisitor;

    fn process(&self, context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        self.hpp.process(context, file_set, visitor);
        self.cpp.process(context, file_set, visitor);
    }
    fn post_process(&self, context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        self.hpp.post_process(context, file_set, visitor);
        self.cpp.post_process(context, file_set, visitor);
    }
}

impl FileProcessorVisitor for FunctionProcessor {
    type Visitor = FunctionVisitor;

    fn process(&self, context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        self.hpp.process(context, file_set, visitor);
        self.cpp.process(context, file_set, visitor);
    }
    fn post_process(&self, context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        self.hpp.post_process(context, file_set, visitor);
        self.cpp.post_process(context, file_set, visitor);
    }
}

impl FileProcessorVisitor for ParameterProcessor {
    type Visitor = ParameterVisitor;

    fn process(&self, context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        self.hpp.process(context, file_set, visitor);
        self.cpp.process(context, file_set, visitor);
    }
    fn post_process(&self, context: &Context, file_set: &mut FileSet, visitor: &Self::Visitor) {
        self.hpp.post_process(context, file_set, visitor);
        self.cpp.post_process(context, file_set, visitor);
    }
}

impl FileGeneratorVisitors for Generator {
    type ImplementationProcessor = ImplementationProcessor;
    type FunctionProcessor = FunctionProcessor;
    type ParameterProcessor = ParameterProcessor;
}
