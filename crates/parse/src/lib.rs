mod combinator;
mod error;
mod input;
mod operations;

pub mod parsing;
mod parser;
mod ty;
mod engine;


use ast::Comment;
pub use error::ParseError;
use input::input;
use crate::error::NomError;
use crate::input::Input;
use crate::parse_util::{comments, module};


const KEYWORDS: &[&str] = &[
    "as", "else", "export", "for", "foreign", "if", "in", "import", "type",
];
const OPERATOR_CHARACTERS: &str = "+-*/=<>&|!?";
const OPERATOR_MODIFIERS: &str = "=";

pub type PineResult<'a, T> = nom::IResult<Input<'a>, T, NomError<'a>>;

pub use self::parsing::*;

pub fn parse(source: &str, path: &str) -> Result<ast::Module, ParseError> {
    module(input(source))
        .map(|(_, module)| module)
        .map_err(|error| ParseError::new(source, path, error))
}

pub fn parse_comments(source: &str, path: &str) -> Result<Vec<Comment>, ParseError> {
    comments(input(source))
        .map(|(_, comments)| comments)
        .map_err(|error| ParseError::new(source, path, error))
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use ast::*;

    use position::{test::PositionFake, Position};

    #[test]
    fn parse_empty_module() {

        let path = "/bot/test/1.pen";
        let code = fs::read_to_string(path);

        let c=parse(code.unwrap().as_str(), "");
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
