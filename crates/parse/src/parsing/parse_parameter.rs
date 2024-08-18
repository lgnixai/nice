use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::opt;
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded};
use ast::Parameter;
use crate::input::Input;
use crate::parsing::parse_identifier::parse_identifier;
use crate::{expression, PineResult, position};

pub fn parse_parameter(input:Input) -> PineResult<Parameter> {
    let (input, position) = position(input)?;
    let (input, ident) = parse_identifier(input)?;
    let (input, default_value) = opt(preceded(tag("="), expression))(input)?;
    Ok((input, Parameter::new(ident, default_value,position())))
}

pub fn parse_parameter_list(input:Input) -> PineResult< Vec<Parameter>> {
    delimited(
        tag("("),
        separated_list0(
            delimited(multispace0, tag(","), multispace0),
            parse_parameter,
        ),
        tag(")"),
    )(input)
}