use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char};
use nom::combinator::{cut, map, opt, recognize, value, verify};
use nom::error::context;
use nom::multi::many0_count;
use nom::sequence::tuple;
use ast::{Identifier, IDENTIFIER_SEPARATOR};
use crate::input::Input;
use crate::{PineResult, KEYWORDS};
use crate::parse_util::{ token};
use std::{collections::HashSet, str};



pub(crate) fn parse_identifier(input: Input) -> PineResult<Identifier> {
    context("identifier", token(raw_identifier))(input)
}

fn raw_identifier(input: Input) -> PineResult<Identifier> {
    verify(unchecked_identifier, |identifier| {
        !KEYWORDS.contains(&&*identifier.name)
    })(input)
}

fn unchecked_identifier(input: Input) -> PineResult<Identifier> {
    map(
        recognize(tuple((
            alt((value((), alpha1::<Input, _>), value((), char('_')))),
            many0_count(alt((value((), alphanumeric1), value((), char('_'))))),
        ))),
        |(span)| {
            let s: String = str::from_utf8(span.as_bytes()).unwrap().to_string();
            Identifier::new(s, 0)
        }

    )(input)
}

// pub fn identifier(input: Input) -> PineResult<String> {
//     context("identifier", token(raw_identifier))(input)
// }
//

pub fn qualified_identifier(input: Input) -> PineResult<String> {
    map(
        recognize(tuple((
            raw_identifier,
            opt(tuple((tag(IDENTIFIER_SEPARATOR), cut(raw_identifier)))),
        ))),
        |span| str::from_utf8(span.as_bytes()).unwrap().into(),
    )(input)
}
