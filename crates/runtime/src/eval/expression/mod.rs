use ast::Expression;
use crate::{value::Value, Runtime};

// pub mod array;
// pub mod arrow_function;
// pub mod binary;
// pub mod call;
// pub mod index;
// pub mod literal;
// pub mod new;
// pub mod switch;
mod list;
mod literal;

impl Runtime {
    pub fn eval_expression(&mut self, expression: Expression) -> Value {
        match expression {
            // Expression::BinaryExpression(expression) => {
            //     self.eval_binary_expression(expression.value)
            // }
            // Expression::IndexExpression(expression) => self.eval_index_expression(expression.value),
            // Expression::MatchExpression(expression) => self.eval_match_expression(expression.value),
            // Expression::Block(block) => self.eval_program(block),
            // Expression::Literal(literal) => self.eval_literal(literal.value),
             Expression::List (list) => self.eval_list(list),

            // Expression::ArrowFunction(func) => self.eval_arrow_function(func.value),
            // Expression::Null => Value::Null,
            // Expression::FunctionCallExpression(call) => self.eval_call(*call),
            // Expression::NewExpression(expression) => self.eval_new_expression(expression),
            // Expression::Ident(name) => Value::Reference(vec![name.value.0], self.scope.clone()),
            // Expression::This => Value::Reference(vec!["this".into()], self.scope.clone()),
            //

            _ => {
                todo!()
            }
        }
    }
}
