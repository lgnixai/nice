use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, alphanumeric1, char, one_of};
use nom::combinator::{cut, map, opt, recognize, value, verify};
use nom::error::context;
use nom::multi::{many0_count, many1};
use nom::sequence::tuple;
use ast::{Identifier, IDENTIFIER_SEPARATOR};
use crate::input::{Input, Span};
use crate::{PineResult, KEYWORDS};
use crate::parse_util::{ token};
use std::{collections::HashSet, str};
use crate::parsing::parse_node::new_identity;


pub  fn parse_identifier(input: Input) -> PineResult<Identifier> {
    context("identifier", token(raw_identifier))(input)
}

fn raw_identifier(input: Input) -> PineResult<Identifier> {
    verify(unchecked_identifier, |identifier| {
        !KEYWORDS.contains(&&*identifier.name)
    })(input)
}

fn unchecked_identifier(mut input: Input) -> PineResult<Identifier> {


    let (input, ident_parsed) =
        recognize(many1(one_of("abcdefghijklmnopqrstuvwxyz_0123456789")))(input)?;

    let (input, node_id) = new_identity(input, &ident_parsed);

    Ok((
        input,
        Identifier {
            name: ident_parsed.to_string(),
            node_id,
        },
    ))
    // map(
    //     recognize(tuple((
    //         alt((value((), alpha1::<Input, _>), value((), char('_')))),
    //         many0_count(alt((value((), alphanumeric1), value((), char('_'))))),
    //     ))),
    //     |(span)| {
    //
    //         let node_id = input.extra.new_identity(Span::from(parsed.clone()));
    //
    //         let s: String = str::from_utf8(span.as_bytes()).unwrap().to_string();
    //
    //         let (input, node_id) = new_identity(input, &ident_parsed);
    //
    //         Identifier::new(s, 0)
    //     }
    //
    // )(input)
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
