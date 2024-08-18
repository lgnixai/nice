use ast::{FunctionDecl};
use ast::types::Type;
use parse::Span;
use crate::{value::{Function,   Value, Visibility}, Runtime};
use crate::value::Parameter;


impl Runtime {
    pub fn declare_function(&mut self, function: FunctionDecl) -> Value {
        //let (span, function) = function.unpack();
        let mut visibility = Visibility::Private;
        let mut is_async = false;
        let mut is_static = false;

        // for modifier in function.modifiers {
        //     let modifier = modifier.value;
        //
        //     match modifier {
        //         Modifier::Public => visibility = Visibility::Public,
        //         Modifier::Private => visibility = Visibility::Private,
        //         Modifier::Protected => visibility = Visibility::Protected,
        //         Modifier::Async => is_async = true,
        //         Modifier::Static => is_static = true,
        //     }
        // }

        if let Some(body) = function.body {
            let parameters = function
                .arguments
                .into_iter()
                .map(|param| Parameter {
                    name: param.name.name,
                    nullable: true,
                   // ty: Type::Function(function),
                    default: param
                        .default_value.map(|expression| Box::new(self.eval_expression(expression))),

                })
                .collect();

            self.set_variable(
                function.name.name.clone(),
                Span::empty().wrap(Value::Function(Function {
                    visibility,
                    overloads: Vec::default(),
                    is_async,
                    is_static,
                    name: function.name.name,
                    parameters,
                    ty: Type::Function(Type::Function),
                    body,
                })),
            );
        }

        Value::None
    }
}