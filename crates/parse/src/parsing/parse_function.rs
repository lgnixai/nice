
use std::path::PathBuf;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, one_of, space0, space1};
use nom::combinator::{cut, map, opt, recognize, verify};
use nom::error::context;
use nom::error_position;
use nom::multi::{many1, separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, terminated, tuple};
use nom_locate::{LocatedSpan, position};
use ast::{Block, Body, FunctionDecl, FunctionDefinition, Identifier, NodeId, Parameter, Statement};
use crate::input::{Input, input, Span};
use crate::{parse_util, PineResult, sign, statement};
use crate::parse_statement::parse_statement;
use crate::parsing::parse_block::{indent, parse_block_indent};
use crate::parsing::parse_identifier::parse_identifier;
use crate::parsing::parse_node::parse_identity;

use crate::parsing::parse_parameter::parse_parameter_list;


pub fn parse_function(input: Input) -> PineResult< FunctionDecl> {
    map(
        tuple((
            parse_util::position,
            parse_identity,
            terminated(
                tuple((
                    parse_identifier,
                    parse_parameter_list,
                    space0,
                    //separated_list0(tuple((space0, tag(","), space0)), parse_identifier),
                )),
                delimited(space0, tag("=>"), space0),
            ),
            parse_body,
        )),
        |(position,node_id, (name, arguments,_), body)| FunctionDecl {
            node_id,
            name,
            body,
            //signature: FuncType::from_args_nb(arguments.len()), // FIXME: Should not generate random signature
            arguments,
            position:position(),
        },
    )(input)
}

pub fn parse_body(input: Input) -> PineResult< Body> {
    let (input, opt_eol) = opt(many1(line_ending))(input)?; // NOTE: should not fail

    if opt_eol.is_some() {
        indent(map(
            separated_list1(
                many1(line_ending),
                preceded(parse_block_indent, statement),
            ),
            Body::new,
        ))(input)
    } else {
        map(statement, |stmt| Body::new(vec![stmt]))(input)
    }
}



#[test]
fn main() {
    let input2 = r#"add( x, y)=>
    a=x + y
    a

    "#;

    let input23 = r#"geom_average(x, y) =>
    a=x+y
    c=y-2
    b=a+c
genv()"#;
    let input23 = r#" add abc@ "#;

    let result = parse_function(input(&input2));


    //let result = parse_function(Input::new_extra(input2, ctx));

    // let result = parse_function(input);
    //println!("{:?}",input);

    match result {
        Ok((remaining, func_decl)) => {
            println!("Parsed function: {:#?}, Remaining input: {}", func_decl, remaining);
        }
        Err(e) => println!("Error parsing function: {:?}", e),
    }
}
