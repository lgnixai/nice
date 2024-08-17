use std::path::PathBuf;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{line_ending, space0};
use nom::combinator::{cut, map, opt, verify};
use nom::error::context;
use nom::multi::{many1, separated_list0};
use nom::sequence::{preceded, terminated, tuple};
use ast::{Block, FunctionDec, FunctionDefinition};
use crate::input::{Input, input};
use crate::{PineResult, position, sign, statement};
use crate::parse_statement::parse_statement;
use crate::parsing::parse_block::{indent, parse_block_indent};
use crate::parsing::parse_identifier::parse_identifier;
use crate::parsing::parse_parameter::parse_parameter_list;
use crate::parsing::parse_variable::parse_variable;

//
fn parse_multiline(input: Input) -> PineResult<Block> {
    println!("多行");
    map(
        tuple((
            position,
            indent(
                separated_list0(
                    line_ending,
                    preceded(
                        parse_block_indent,
                        statement,
                        // verify(many1(statement), |statements: &[_]| {
                        //     statements
                        //         .last()
                        //         .map(|statement| statement.name().is_none())
                        //         .unwrap_or_default()
                        // }),
                    ),
                )))), |(position, statements)|{

            println!("44444");
            Block::new(
                statements[..statements.len() - 1].to_vec(),
                statements.last().unwrap().expression().clone(),
                position(),
            )
        }

    )(input)
}
//

fn parse_single(input: Input) -> PineResult<Block> {
    println!("单行");
    map(
        tuple((
            position,
            tag("=>"),
            space0,
            cut(terminated(
                verify(many1(statement), |statements: &[_]| {
                    statements
                        .last()
                        .map(|statement| statement.name().is_none())
                        .unwrap_or_default()
                }),
                alt((tag("\n"), line_ending)),
            )),
        )),
        |(position, _, _, statements)| {
            Block::new(
                statements[..statements.len() - 1].to_vec(),
                statements.last().unwrap().expression().clone(),
                position(),
            )
        })(input)
}

fn parse_body_decls(input: Input) -> PineResult<Block> {
    println!("多行");
    println!("input {:?}",input);
    map(
        tuple((
            position,
            indent(separated_list0(
                line_ending,
                preceded(
                    parse_block_indent,
                    statement,
                ),
            ))
        )), |(position, statements)| {
            Block::new(
                statements[..statements.len() - 1].to_vec(),
                statements.last().unwrap().expression().clone(),
                position(),
            )
        },
    )(input)
}


pub fn parse_function(input: Input) -> PineResult<FunctionDec> {
    map(
        tuple((
            position,
            parse_identifier,
            parse_parameter_list,
            space0,
            preceded(terminated(tag("=>"), line_ending), parse_body_decls),
            // alt((
            //
            //     //parse_multiline,
            //     //
            //     //     preceded( take_until("\n"),parse_stmt), |stmt| (Body::new(vec![Stmt::ExprStmt(stmt)]), false)),
            //
            //     // 处理多行函数体
            //
            //     parse_single,
            // ))
        )),
        |(position, name, params,_, block)| {
            FunctionDec::new(name, params, block, position())
            // let mut body = body;
            // //单行处理
            // // if !is_multi_line {
            // //     print!("单行函数")
            // //     // if let Some(last_stmt) = body.pop() {
            // //     //     body.push( Stmt::Return(match last_stmt {
            // //     //         Statement::Expression(expr) => expr,
            // //     //         _ => format!("{}", last_stmt),
            // //     //     }));
            // //     // }
            // // } else{
            // //     print!("多行函数")
            // // }
            // FunctionDeclaration { name: ident.name, params, body }
        },
    )(input)
}

//
// fn parse_function2(input: Input) -> PineResult<Block> {
//     context(
//         "block",
//         map(
//             tuple((
//                 position,
//                 sign("{"),
//                 cut(terminated(
//                     verify(many1(crate::parsing::parse_util::statement), |statements: &[_]| {
//                         statements
//                             .last()
//                             .map(|statement| statement.name().is_none())
//                             .unwrap_or_default()
//                     }),
//                     sign("}"),
//                 )),
//             )),
//             |(position, _, statements)| {
//                 Block::new(
//                     statements[..statements.len() - 1].to_vec(),
//                     statements.last().unwrap().expression().clone(),
//                     position(),
//                 )
//             },
//         ),
//     )(input)
// }


#[test]
fn main() {
    let input2 = r#"geom_average(x, y) =>x+2
    "#;

    let input2 = r#"geom_average(x, y) =>
    a=x+y
    c=y-2
    b=a+c
genv()"#;


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