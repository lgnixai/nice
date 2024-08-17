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

    declaration_mode: Option<DeclarationMode>,
    var_type: Option<DataType>,
    identifier: Identifier,
    value: Expression,
    position: Position,
}
//
// impl VariableDefinition {
//     pub fn new(
//         name: impl Into<String>,
//         lambda: Lambda,
//         foreign_export: Option<ForeignExport>,
//         position: Position,
//     ) -> Self {
//         Self {
//             name: name.into(),
//             lambda,
//             foreign_export,
//             position,
//         }
//     }
//
//     pub fn name(&self) -> &str {
//         &self.name
//     }
//
//     pub fn lambda(&self) -> &Lambda {
//         &self.lambda
//     }
//
//     pub fn foreign_export(&self) -> Option<&ForeignExport> {
//         self.foreign_export.as_ref()
//     }
//
//     pub fn position(&self) -> &Position {
//         &self.position
//     }
// }
