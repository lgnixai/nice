use nom_locate::position;
use ast::NodeId;
use crate::input::{Input, Span};
use crate::PineResult;

pub fn new_identity<'a>(mut input: Input<'a>, parsed: &Input<'a>) -> (Input<'a>, NodeId) {
    let node_id = input.extra.new_identity(Span::from(parsed.clone()));

    (input, node_id)
}

pub fn parse_node_id(input: Input) -> PineResult< NodeId> {
    let (input, pos) = position(input)?;

    let (input, node_id) = new_identity(input, &pos);

    Ok((input, node_id))
}

pub fn parse_identity(input: Input) -> PineResult<NodeId> {
    let (input, pos) = position(input)?;

    let (input, node_id) = new_identity(input, &pos);
    println!("node_id{:?}",node_id);
    Ok((input, node_id))
}
