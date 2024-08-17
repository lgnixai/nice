use std::collections::HashSet;
use nom::branch::alt;
use nom::combinator::{cut, map, success, verify};
use nom::error::context;
use nom::multi::many0;
use nom::sequence::{preceded, terminated, tuple};
use ast::{Record, RecordDefinition, RecordField, types};
use crate::combinator::{separated_or_terminated_list0, separated_or_terminated_list1};
use crate::input::Input;
use crate::PineResult;
use crate::parse_util::{expression, keyword, position, sign};
use crate::parsing::parse_identifier::{parse_identifier, qualified_identifier};


pub fn parse_record(input: Input) -> PineResult<Record> {
    // TODO Disallow spaces before `{` for disambiguation?
    context(
        "record",
        map(
            tuple((
                position,
                qualified_identifier,
                sign("{"),
                verify(
                    alt((
                        preceded(
                            sign("..."),
                            cut(tuple((
                                map(terminated(expression, sign(",")), Some),
                                separated_or_terminated_list1(sign(","), record_field),
                            ))),
                        ),
                        tuple((
                            success(None),
                            separated_or_terminated_list0(sign(","), record_field),
                        )),
                    )),
                    |(_, fields)| {
                        fields.len()
                            == HashSet::<&str>::from_iter(fields.iter().map(|field| field.name()))
                            .len()
                    },
                ),
                sign("}"),
            )),
            |(position, name, _, (record, fields), _)| {
                Record::new(name, record, fields, position())
            },
        ),
    )(input)
}

fn record_field(input: Input) -> PineResult<RecordField> {
    context(
        "record field",
        map(
            tuple((position, parse_identifier, sign(":"), cut(crate::parsing::parse_util::expression))),
            |(position, ident, _, expression)| RecordField::new(ident.name, expression, position()),
        ),
    )(input)
}
pub fn record_definition(input: Input) -> PineResult<RecordDefinition> {
    context(
        "record definition",
        map(
            tuple((
                position,
                keyword("type"),
                parse_identifier,
                sign("{"),
                cut(tuple((many0(record_field_definition), sign("}")))),
            )),
            |(position, _, ident, _, (fields, _))| RecordDefinition::new(ident.name, fields, position()),
        ),
    )(input)
}

pub fn record_field_definition(input: Input) -> PineResult<types::RecordField> {
    context(
        "record field",
        map(
            tuple((position, parse_identifier, crate::parsing::parse_util::type_)),
            |(position, ident, type_)| types::RecordField::new(ident.name, type_, position()),
        ),
    )(input)
}