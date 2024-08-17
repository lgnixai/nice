use nom::branch::alt;
use nom::combinator::{cut, map};
use nom::error::context;
use nom::sequence::{preceded, tuple};
use ast::{Map, MapElement, MapEntry};
use crate::combinator::separated_or_terminated_list0;
use crate::input::Input;
use crate::PineResult;
use crate::parse_util::{expression, position, sign, type_};

pub fn map_literal(input: Input) -> PineResult<Map> {
    context(
        "map",
        map(
            tuple((
                position,
                sign("{"),
                cut(tuple((
                    type_,
                    sign(":"),
                    type_,
                    separated_or_terminated_list0(sign(","), map_element),
                    sign("}"),
                ))),
            )),
            |(position, _, (key_type, _, value_type, elements, _))| {
                Map::new(key_type, value_type, elements, position())
            },
        ),
    )(input)
}

fn map_element(input: Input) -> PineResult<MapElement> {
    alt((
        map(
            tuple((position, expression, sign(":"), cut(expression))),
            |(position, key, _, value)| MapEntry::new(key, value, position()).into(),
        ),
        map(preceded(sign("..."), cut(expression)), MapElement::Multiple),
    ))(input)
}

