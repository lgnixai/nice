use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0},
    combinator::{opt, recognize},
    sequence::{pair, preceded, tuple},

};
use nom::bytes::complete::take_while;
use nom::character::complete::not_line_ending;
use nom::combinator::map;
use nom::sequence::terminated;
use ast::datatype::DeclarationMode;
use crate::input::Input;
use crate::PineResult;

pub fn parse_declaration_mode(input: Input) -> PineResult<DeclarationMode> {
    alt((
        map(tag("varip"), |_| DeclarationMode::Varip),
        map(tag("var"), |_| DeclarationMode::Var),
        map(tag("const"), |_| DeclarationMode::Const),
    ))(input)
}