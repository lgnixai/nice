use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::{all_consuming, eof, into, map, opt};
use nom::error::context;

use nom::multi::{many0, separated_list0};
use nom::sequence::tuple;
use ast::{Main, Module, Statement, Statements, TypeDefinition};
use crate::input::Input;
use crate::{blank, function_definition, PineResult, parsing, record_definition};
use crate::parsing::parse_function::parse_function;
use parsing::parse_util::position;
pub fn parse_statement(input: Input) -> PineResult<Main>{
    map(
        all_consuming(tuple((
            position,
            many0(alt((into(crate::parsing::parse_util::type_alias), into(record_definition)))),
            many0(parse_function),
            blank,
        ))),
        |(position,  type_definitions,definitions, _)| {
            Main::new(

                type_definitions,

                definitions,
                position(),

            )
        },
    )(input)
}
//
// pub fn parse_statement2(i: Input) -> PineResult<Main> {
//     // STMT <<; | \n> STMT>* [;]
//     // map(
//     //     tuple((position, many0(parse_program_statement), eof)),
//     //     |(start, program, end)| start.between(Span::from(end)).wrap(program),
//     // ).parse(input)
//
//     let (i, mut stmts) =  many0(single_statement)(i)?;
//
//     let (i, stmt) = if stmts.len() == 1 {
//         let stmt = stmts.pop().expect("vec should have length 1");
//         (i, Main {
//             statement: vec![stmt]
//         })
//     } else {
//         let statement = stmts;
//         (i, Main {
//             statement
//         })
//     };
//
//     Ok((i, stmt))
// }

pub fn single_statement(i: Input<'_>) -> PineResult<Statements> {
    println!("input :::{:?}", i);
    context(
        "statement",
        map(
            alt((
                map(parsing::expression, Statements::Expression),

                // map(parsing::parse_import, Statements::Import),
                map(parsing::parse_record, Statements::Record),
                map(parsing::map_literal, Statements::Map),
                map(parsing::function_definition, Statements::Func),
            )),
            |statement: Statements| {
                match statement {
                    Statements::Import(stmt) => Statements::Import(stmt),  // 返回 Import 枚举
                    Statements::Record(stmt) => Statements::Record(stmt),  // 返回 Record 枚举
                    Statements::Map(stmt) => Statements::Map(stmt),  // 返回 Record 枚举
                    Statements::Func(stmt) => Statements::Func(stmt),  // 返回 Record 枚举
                    Statements::Expression(stmt) => Statements::Expression(stmt),  // 返回 Record 枚举
                    _ => unimplemented!(),  // 处理其他情况，必要时可以添加更多分支
                }
            },
        ),
    )(i)
}
//
// pub fn single_statement(i: Input<'_>) -> PineResult<Statements> {
//     println!("input :::{:?}", i);
//     context(
//         "statement",
//             (
//                 map(
//                     alt((
//                         map(parsing::parse_import, Statements::Import),
//                         map(
//                             parsing::parse_record,
//                             Statements::Record,
//                         ),
//                     )),
//                     |statement: Statement| {
//                         match statement {
//                             Statements::Import(stmt) => Ok(stmt),  // 处理 Import 类型
//                             Statements::Record(rec) => Ok(rec),  // 处理其他类型
//                         }
//                 },
//             ),
//         )
//
//     )(i)
// }