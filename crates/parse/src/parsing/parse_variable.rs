use nom::combinator::map;
use nom::error::context;
use nom::sequence::tuple;
use ast::Variable;
use crate::input::Input;
use crate::PineResult;
use crate::parse_util::{position, token};
use crate::parsing::parse_identifier::qualified_identifier;

pub fn variable(input: Input) -> PineResult<Variable> {
    context(
        "variable",
        map(
            tuple((position, token(qualified_identifier))),
            |(position, identifier)| Variable::new(identifier, position()),
        ),
    )(input)
}
