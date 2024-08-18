use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0, space1};
use nom::combinator::{map, opt};
use nom::error::{ErrorKind, ParseError};
use nom::multi::many1;
use nom::sequence::{preceded, terminated, tuple};
use ast::{Else, For, ForIn, IfDecl, While};

use crate::input::Input;
use crate::parsing::parse_node::parse_identity;
use crate::{expression, PineResult};
use crate::parsing::parse_block::parse_block_indent;
use crate::parsing::parse_function::parse_body;
use crate::parsing::parse_identifier::parse_identifier;


pub fn parse_if(input: Input) -> PineResult< IfDecl> {
    map(
        tuple((
            parse_identity,
            terminated(tag("if"), space1),
            expression,
            opt(preceded(many1(line_ending), parse_then_multi)),
            parse_body,
            opt(tuple((line_ending, parse_else))),
        )),
        |(node_id, _if_, cond, _, body, else_)| {
            IfDecl::new(node_id, cond, body, else_.map(|(_, else_)| Box::new(else_)))
        },
    )(input.clone())
}

pub fn parse_then_multi(input:Input) -> PineResult< ()> {
    // NOTE: This is a tweek for then blocks that are at indent 0 (i.e. in the test files)
    let (input, indent) = if input.extra.first_indent.is_some() && input.extra.block_indent > 0 {
        parse_block_indent(input)?
    } else {
        (input, 0)
    };

    if indent == input.extra.block_indent {
        let (input, _) = terminated(tag("then"), space0)(input)?;

        Ok((input, ()))
    } else {
        Err(nom::Err::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Tag,
        )))
    }
}

pub fn parse_else(input: Input) -> PineResult< Else> {
    // NOTE: This is a tweek for else blocks that are at indent 0 (i.e. in the test files)
    let (input, indent) = if input.extra.first_indent.is_some() && input.extra.block_indent > 0 {
        parse_block_indent(input)?
    } else {
        (input, 0)
    };

    if indent == input.extra.block_indent {
        alt((
            map(
                tuple((
                    terminated(tag("else"), space1),
                    terminated(parse_if, space0),
                )),
                |(_, if_)| Else::If(if_),
            ),
            map(
                tuple((
                    terminated(tag("else"), space0),
                    terminated(parse_body, space0),
                )),
                |(_, body)| Else::Body(body),
            ),
        ))(input)
    } else {
        Err(nom::Err::Error(ParseError::from_error_kind(
            input,
            ErrorKind::Tag,
        )))
    }
}

pub fn parse_for(input: Input) -> PineResult< For> {
    alt((map(parse_for_in, For::In), map(parse_while, For::While)))(input)
}

pub fn parse_for_in(input: Input) -> PineResult< ForIn> {
    map(
        tuple((
            terminated(tag("for"), space1),
            terminated(parse_identifier, space0),
            terminated(tag("in"), space0),
            terminated(expression, space0),
            parse_body,
        )),
        |(_, var, _, expr, body)| ForIn::new(var, expr, body),
    )(input)
}

pub fn parse_while(input: Input) -> PineResult< While> {
    map(
        tuple((
            terminated(tag("while"), space1),
            terminated(expression, space0),
            parse_body,
        )),
        |(_, cond, body)| While::new(cond, body),
    )(input)
}
