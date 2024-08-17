use std::fs;
use std::path::PathBuf;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::{map, opt};
use nom::error::context;
use nom::sequence::{preceded, tuple};
use ast::{FunctionDefinition, Variable, VariableDefinition};
use crate::input::{Input, input, spaced};
use crate::{expression, PineResult};
use crate::parse_util::{position, token, type_alias};
use crate::parsing::parse_data_type::parse_type;
use crate::parsing::parse_declaration_mode::parse_declaration_mode;
use crate::parsing::parse_identifier::{parse_identifier, qualified_identifier};


/**
[<declaration_mode>] [<type>] <identifier> = <expression> | <structure>
 */


pub fn parse_variable(input: Input) -> PineResult<VariableDefinition> {
    context(
        "variableDefinition",
        map(
            tuple((
                position,
                opt(preceded(
                    multispace0,
                    parse_declaration_mode,
                    //alt(( const_tag,var_tag,varip_tag)),
                    //alt((parse_declaration_mode, map(tag(""), |_| None))),
                )),
                opt(preceded(multispace0, parse_type)),
                preceded(multispace0, parse_identifier),
                opt(preceded(spaced(tag("=")), expression)),
            )),|(position,declaration_mode, var_type, identifier, value)| {
                VariableDefinition::new(declaration_mode, var_type, identifier, value.unwrap(), position())
            },
        ),
    )(input)
}

#[test]
fn main() {
    let script = "var int a= 3 + 4";
    // let path = "/bot/test/1.pen";
    // let code = fs::read_to_string(path);
    // let i=code.unwrap().as_str();
    let i = "var int a= 3 + 4*(5+6)";

    //let i=r#"a =x+1"#;
    let c=parse_variable(input(&i));
    match c {
        Ok((remaining, parsed)) => {
            println!("Parsed identifier: {:#?}, Remaining: {}", parsed, remaining);
        }
        Err(err) => {
            println!("Failed to parse '{}': {:?}", script, err);
        }
    }
}
