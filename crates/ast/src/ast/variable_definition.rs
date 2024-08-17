use super::{foreign_export::ForeignExport, lambda::Lambda};
use position::Position;
use crate::ast::identifier::Identifier;
use crate::datatype::{DataType, DeclarationMode};
use crate::Expression;

#[derive(Clone, Debug, PartialEq)]
pub struct VariableDefinition {
    // name: String,
    // lambda: Lambda,
    // foreign_export: Option<ForeignExport>,

    pub declaration_mode: Option<DeclarationMode>,
    pub var_type: Option<DataType>,
    pub identifier: Identifier,
    pub value: Expression,
    pub position: Position,
}

impl VariableDefinition {
    pub fn new(
        declaration_mode: Option<DeclarationMode>,
        var_type:Option<DataType>,
        identifier:Identifier,
        value:Expression,
        position: Position,
    ) -> Self {
        Self {
            declaration_mode,
            var_type,
            identifier,
            value,
            position,
        }
    }


    pub fn value(&self) -> &Expression {
        &self.value
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn var_type(&self) -> Option<&DataType> {
        self.var_type.as_ref()
    }

    pub fn position(&self) -> &Position {
        &self.position
    }
}
