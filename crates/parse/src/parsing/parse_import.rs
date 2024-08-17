use nom::combinator::{cut, map, opt};
use nom::error::context;
use nom::sequence::{delimited, preceded, tuple};
use ast::{Identifier, Import};
use crate::combinator::separated_or_terminated_list1;
use crate::input::{Input};
use crate::PineResult;
use crate::parse_util::{keyword, module_path, position, sign, unqualified_name};
use crate::parsing::parse_identifier::{ parse_identifier};


pub fn parse_import(input: Input) -> PineResult<Import> {
    context(
        "import",
        map(
            tuple((
                position,
                keyword("import"),
                module_path,
                cut(tuple((
                    opt(preceded(keyword("as"), parse_identifier)),
                    opt(delimited(
                        sign("{"),
                        separated_or_terminated_list1(sign(","), unqualified_name),
                        sign("}"),
                    )),
                ))),
            )),
            |(position, _, path, (prefix, names))| {
                Import::new(path, prefix.and_then(|identifier| Some(identifier.name)), names.unwrap_or_default(), position())
            },
        ),
    )(input)
}
