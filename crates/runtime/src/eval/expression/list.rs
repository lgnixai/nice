use ast::List;
use parse::util::ArraySize;
use crate::{value::Value, Runtime};

impl Runtime {
    pub fn eval_list(
        &mut self,
        list: List,
    ) -> Value {

        let elements=list.elements()
            .iter()
            .map(|element| {
                match element {
                    ast::ListElement::Multiple(element) => {
                        match self.eval_expression(**element) {
                            Value::Reference(path, scope) => todo!(),
                            value => value,
                        }
                    }
                    ast::ListElement::Single(element) => {
                        match self.eval_expression(**element) {
                            Value::Reference(path, scope) => todo!(),
                            value => value,
                        }
                    }
                }
            })
            .collect::<Vec<_>>();


        let size = elements.len();

        Value::Array(
            elements,
            elements
                .map(ArraySize::Dynamic, ArraySize::Fixed(size)),
        )
    }
}
