use super::{
    function_definition::FunctionDefinition, type_definition::TypeDefinition, ForeignImport, Import,
};
use position::Position;
use crate::{Expression, Map, Record, Statement};

#[derive(Clone, Debug, PartialEq)]
pub struct Main {
    type_definitions: Vec<TypeDefinition>,
    function_definitions: Vec<FunctionDefinition>,
    position: Position,
}

impl Main {
    pub fn new(
        type_definitions: Vec<TypeDefinition>,

        function_definitions: Vec<FunctionDefinition>,
        position: Position,
    ) -> Self {
        Self {
            type_definitions,
            function_definitions,
            position,
        }
    }


    pub fn function_definitions(&self) -> &[FunctionDefinition] {
        &self.function_definitions
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}


#[derive(Debug, Clone)]
pub enum Statements {
    Compound(Statement),
    Record(Record),
    Import(Import),
    Map(Map),
    Func(FunctionDefinition),
    Expression(Expression),
}