use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{char, multispace0},
    combinator::{opt, recognize},
    sequence::{pair, preceded, tuple},
};
use nom::bytes::complete::take_while;
use nom::character::complete::not_line_ending;
use nom::combinator::map;
use nom::sequence::terminated;
use ast::datatype::DataType;
use crate::input::Input;
use crate::PineResult;


pub fn parse_type(input: Input) -> PineResult<DataType> {
    alt((
        map(tag("int"), |_| DataType::Int),
        map(tag("float"), |_| DataType::Float),
        map(tag("bool"), |_| DataType::Bool),
        map(tag("color"), |_| DataType::Color),
        map(tag("string"), |_| DataType::String),
        map(tag("line"), |_| DataType::Line),
        map(tag("linefill"), |_| DataType::LineFill),
        map(tag("label"), |_| DataType::Label),
        map(tag("box"), |_| DataType::Box),
        map(tag("table"), |_| DataType::Table),
        map(tag("UDF"), |_| DataType::UDF),
        // 添加对 array<int> 等复杂类型的支持
        map(preceded(tag("array<"), terminated(parse_type, tag(">"))), |t| {
            DataType::Array(Box::new(t))
        }),
        map(preceded(tag("matrix<"), terminated(parse_type, tag(">"))), |t| {
            DataType::Matrix(Box::new(t))
        }),
    ))(input)
}