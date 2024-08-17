// use nom::{
//     branch::alt,
//     bytes::complete::{tag, take_while1},
//     character::complete::{char, multispace0},
//     combinator::{opt, recognize},
//     sequence::{pair, preceded, tuple},
//
// };
// use nom::bytes::complete::take_while;
// use nom::character::complete::not_line_ending;
// use nom::combinator::map;
// use nom::sequence::terminated;
// use crate::input::Input;
// use crate::PineResult;
//
//
// pub fn parse_assignment(input: Input) -> PineResult<Variable> {
//     map(
//         tuple((
//             parse_variable_declaration,
//
//             preceded(multispace0, char('=')),
//             preceded(multispace0, not_line_ending),
//         )),
//         |(variable, _, expr)| Variable {
//             declaration_mode: variable.declaration_mode,
//             identifier: variable.identifier,
//             var_type: variable.var_type,
//             //identifier: identifier.to_string(),
//
//             value: parse_expr(expr).unwrap().1, // 使用 parse_expr 解析表达式
//         })(input)
//
//     //
//     //  let (input,  variable) = parse_variable_declaration(input)?;
//     //  let (input, _) = preceded(multispace0, char('='))(input)?;  // 解析等号
//     //
//     //  //let (input, expr) = preceded(multispace0, take_while1(|c| c != '\n'))(input)?;  // 解析表达式
//     //  let (input, expr) = preceded(multispace0, not_line_ending)(input)?;  // 解析表达式
//     //
//     //  // Ok((input, (mode, var_type, identifier, expr)))
//     // // variable.value=parse_expr(expr).unwrap().1;
//     //  let variable1 = Stmt::VariableDeclaration  {
//     //      declaration_mode: variable.declaration_mode,
//     //      //identifier:variable.identifier,
//     //      var_type:variable.var_type,
//     //      //identifier: identifier.to_string(),
//     //      value: parse_expr(expr).unwrap().1 // 使用 parse_expr 解析表达式
//     //  };
//     //  Ok((input, variable1))
// }
//
