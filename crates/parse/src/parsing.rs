pub mod main;
pub mod parse_statement;
mod parse_import;
pub mod parse_util;
mod parse_identifier;
mod parse_variable;
mod parse_record;
mod parse_map;
mod parse_assign;
mod parse_declaration_mode;
mod parse_data_type;
mod parse_node;

pub use parse_import::*;
pub use parse_record::*;
pub use parse_map::*;
pub use parse_util::*;
use crate::parse;


#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use ast::*;

    use position::{test::PositionFake, Position};
    use crate::input::input;
    use crate::main::parse_ast;
    use crate::parse_util::expression;

    #[test]
    fn parse_empty_module() {

        let path = "/bot/test/1.pen";
        let code = fs::read_to_string(path);
        let i=r#"a =x+1"#;
       //let c=expression(input(i,"root"));
        //let c=expression(input(i,"root"));
        let c=parse_ast(code.unwrap().as_str(), "root");

        match c {
            Ok(module) => {
                println!("{:#?}", module)
            }
            Err(err) => {
                println!("{:?}", err.to_string())
            }
        }
        // assert_eq!(
        //     // parse(code.unwrap().as_str(), ""),
        //     // Ok(Module::new(
        //     //     vec![],
        //     //     vec![],
        //     //     vec![],
        //     //     vec![],
        //     //     Position::fake()
        //     // ))
        // );
    }
}
