use ast::VariableStatement;
use crate::{
    value::{ErrorCode, Value},
    Runtime,
};

impl Runtime {
    pub fn declare_variable(&mut self, variables: VariableStatement) -> Value {
         let mut last = Value::None;

        for variable in variables.declarations {

            // if !variable.nullable.value && variable.initializer.is_none() {
            //     return Value::error(span, ErrorCode::Type, "expected anything, but got nothing");
            // }

            let (value_span, value) = if let Some(expression) = variable.value {
                (
                    expression.span,
                    match self.eval_expression(expression) {
                        Value::Reference(path, scope) => todo!(),
                        value => value,
                    },
                )
            } else {
                ("", Value::None)
            };

            // if let Some(ty) = variable.ty {
            //     if !value.is_type_of(&ty.value) {
            //         return Value::error(
            //             value_span,
            //             ErrorCode::Type,
            //             format!("expected {}, but got {}", ty.value, value.value_type_of()),
            //         );
            //     }
            // }

            last = self.set_variable(variable.identifier.name,  value);
        }

        last
    }
}
