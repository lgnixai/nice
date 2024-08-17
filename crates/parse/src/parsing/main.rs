use nom::{combinator::eof, sequence::terminated};
use nom::Err::Error;
use nom::error::ErrorKind;
use ast::Main;
use crate::input::{Input, input};
use crate::ParseError;

use crate::parsing::parse_statement::parse_statement;

pub fn parse_ast(source: &str, path: &str) -> Result<ast::Main, ParseError> {
    // module(input(source, path))
    //     .map(|(_, module)| module)
    //     .map_err(|error| ParseError::new(source, path, error))

    let i=  input(source);
    match terminated(parse_statement, eof)(i) {
        Ok((_, statement)) => Ok(statement),
        Err(e) =>  {
            Err(ParseError::new(source, path, e))
        },
    }

}

//
// #[cfg(test)]
// mod tests {
//     use std::assert_matches::assert_matches;
//
//     use crate::{ast::node::statement::StmtValue, parser::new_input};
//
//     use super::module;
//
//     #[test]
//     fn test_module() {
//         let m = module(new_input("struct S {}\nlet x = false")).unwrap();
//
//         match m.stmt.value {
//             StmtValue::Compound(cmp) => {
//                 assert_eq!(cmp.len(), 2)
//             }
//             _ => assert!(false),
//         }
//     }
//
//     #[test]
//     fn test_invalid_end() {
//         let m = module(new_input("let x = 2 INVALID"));
//         assert_matches!(m, Err(_));
//     }
// }
