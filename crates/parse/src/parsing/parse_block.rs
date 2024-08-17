use std::path::PathBuf;
use nom::character::complete::{line_ending, space1};
use nom::combinator::map;
use nom::error::{ErrorKind, ParseError};
use nom::IResult;
use nom::multi::{many1, separated_list1};
use nom::sequence::preceded;
use crate::error::NomError;
use crate::input::Input;
use crate::PineResult;

pub fn indent<'a, O, E, F>(mut parser: F) -> impl FnMut(Input<'a>) -> IResult<Input<'a>, O, E>
    where
        F: nom::Parser<Input<'a>, O, E>,
{
    move |mut input: Input<'a>| {
        if let Some(indent) = input.extra.first_indent {
            input.extra.block_indent += indent;
        }

        let (mut input, output) = parser.parse(input)?;

        if let Some(indent) = input.extra.first_indent {
            input.extra.block_indent -= indent;
        }

        Ok((input, output))
    }
}

pub fn parse_block_indent(input: Input) -> PineResult<usize> {
    let (mut input, indent) = space1(input)?;
    let indent_len = indent.fragment().len();
    println!("indent_len {:?}",indent_len);
    if input.extra.first_indent == None {
        input.extra.first_indent = Some(indent_len);
        input.extra.block_indent = indent_len;
    }

    if indent_len == input.extra.block_indent {
        Ok((input, indent_len))
    } else {
        // todo()
        Err(nom::Err::Error(NomError::from_error_kind(input.clone(), ErrorKind::Tag)))

    }
}

pub fn parse_block_indent_plus_one(input: Input) -> PineResult<usize> {
    let (mut input, indent) = space1(input)?;
    let indent_len = indent.fragment().len();

    if input.extra.first_indent == None {
        input.extra.first_indent = Some(indent_len);
        input.extra.block_indent = indent_len;
    }

    if indent_len == input.extra.block_indent + input.extra.first_indent.unwrap() {
        Ok((input, indent_len))
    } else {
        // todo()
        Err(nom::Err::Error(NomError::from_error_kind(input.clone(), ErrorKind::Tag)))

        // Err(nom::Err::Error(ParseError::from_error_kind(
        //     input,
        //     ErrorKind::Tag,
        // )))
    }
}
// //
// // pub fn parse_body(input: Input) -> PineResult<Body> {
// //
// //     println!("1==={:?}",input );
// //
// //     // let (input, opt_eol) = opt(many1(line_ending))(input)?; // NOTE: should not fail
// //     // println!("2==={:?}",opt_eol.is_some());
// //     // if opt_eol.is_some() {
// //     //     indent(map(
// //     //         separated_list1(
// //     //             many1(line_ending),
// //     //             preceded(parse_block_indent, parse_stmt),
// //     //         ),
// //     //         Body::new,
// //     //     ))(input)
// //     // } else {
// //     //     println!("单行");
// //     //     // let b=String::from(input.fragment().to_string());
// //     //     map(parse_stmt, |stmt| Body::new(vec![stmt]))(input)
// //     // }
// //
// //     indent(map(
// //         separated_list1(
// //             many1(line_ending),
// //             preceded(parse_block_indent, parse_stmt),
// //         ),
// //         Body::new,
// //     ))(input)
// // }
//
// #[test]
// fn main() {
//     let input = r#"genv(x,y)=>
//
//     a=x+3
//     c=a+b
//
//     "#;
//     let mut path = PathBuf::new();
//     let ctx=ParserCtx::new(path);
//
//     let result = parse_function(Input::new_extra(input,ctx));
//     match result {
//         Ok((remaining, enum_decl)) => {
//             println!("Parsed enum: {:?}, Remaining input: {}", enum_decl, remaining);
//         }
//         Err(e) => println!("Error parsing enum: {:?}", e),
//     }
// }