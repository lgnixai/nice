use crate::{combinator::{separated_or_terminated_list0, separated_or_terminated_list1}, error::NomError, input::{self, Input}, PineResult, KEYWORDS, operations::{reduce_operations, SuffixOperator}, OPERATOR_CHARACTERS, OPERATOR_MODIFIERS, parse_record, record_definition, parse_import};
use ast::{types::Type, *};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        alpha1, alphanumeric1, char, digit1, multispace0, multispace1, none_of, one_of,
    },
    combinator::{
        all_consuming, cut, into, map, not, opt, peek, recognize, success, value, verify,
    },
    error::context,
    multi::{count, many0, many0_count, many1, separated_list1},
    number::complete::recognize_float,
    sequence::{delimited, pair, preceded, terminated, tuple},
    Parser,
};
use position::Position;
use std::{collections::HashSet, str};
use crate::parsing::parse_identifier::{parse_identifier, qualified_identifier};


pub fn module(input: Input) -> PineResult<Module> {
    map(
        all_consuming(tuple((
            position,
            many0(parse_import),
            many0(foreign_import),
            many0(alt((into(type_alias), into(record_definition)))),
            many0(function_definition),
            blank,
        ))),
        |(position, imports, foreign_imports, type_definitions, definitions, _)| {
            Module::new(
                imports,
                foreign_imports,
                type_definitions,
                definitions,
                position(),
            )
        },
    )(input)
}

pub fn comments(input: Input) -> PineResult<Vec<Comment>> {
    map(
        all_consuming(many0(tuple((
            multispace0,
            alt((
                map(comment, Some),
                map(raw_string_literal, |_| None),
                map(none_of("\"#"), |_| None),
            )),
            multispace0,
        )))),
        |comments| {
            comments
                .into_iter()
                .flat_map(|(_, comment, _)| comment)
                .collect()
        },
    )(input)
}


pub fn unqualified_name(input: Input) -> PineResult<UnqualifiedName> {
    map(
        token(tuple((position, parse_identifier))),
        |(position, parse_identifier)| UnqualifiedName::new(parse_identifier.name   , position()),
    )(input)
}

pub fn module_path(input: Input) -> PineResult<ModulePath> {
    context(
        "module path",
        token(alt((
            into(external_module_path),
            into(internal_module_path),
        ))),
    )(input)
}

fn internal_module_path(input: Input) -> PineResult<InternalModulePath> {
    context(
        "internal module path",
        map(module_path_components(map(parse_identifier,|id|id.name)), InternalModulePath::new),
    )(input)
}

fn external_module_path(input: Input) -> PineResult<ExternalModulePath> {
    context(
        "external module path",
        map(
            tuple((
                parse_identifier,
                cut(module_path_components(public_module_path_component)),
            )),
            |(package, components)| ExternalModulePath::new(package.name, components),
        ),
    )(input)
}

fn module_path_components<'a>(
    component: impl Parser<Input<'a>, String, NomError<'a>>,
) -> impl FnMut(Input<'a>) -> PineResult<'a, Vec<String>> {
    many1(preceded(tag(IDENTIFIER_SEPARATOR), component))
}

fn public_module_path_component(input: Input) -> PineResult<String> {
    context(
        "public module path component",
        map(
            verify(
                parse_identifier,  // 假设 parse_identifier 返回 IResult<Input, Identifier>
                |s: &Identifier| ast::analysis::is_name_public(&s.name),  // 验证 Identifier 的 name 是否为公共
            ),
            |identifier: Identifier| identifier.name  // 将 Identifier 的 name 字段提取为 String
        )
    )(input)
}

fn foreign_import(input: Input) -> PineResult<ForeignImport> {
    context(
        "foreign import",
        map(
            tuple((
                position,
                keyword("import"),
                keyword("foreign"),
                cut(tuple((opt(calling_convention), parse_identifier, type_))),
            )),
            |(position, _, _, (calling_convention, ident, type_))| {
                ForeignImport::new(
                    ident.name,
                    calling_convention.unwrap_or_default(),
                    type_,
                    position(),
                )
            },
        ),
    )(input)
}

fn calling_convention(input: Input) -> PineResult<CallingConvention> {
    context(
        "calling convention",
        value(
            CallingConvention::C,
            verify(string_literal, |string| string.value() == "c"),
        ),
    )(input)
}

pub fn function_definition(input: Input) -> PineResult<FunctionDefinition> {
    context(
        "function definition",
        map(
            tuple((
                position,
                opt(foreign_export),
                parse_identifier,
                sign("="),
                cut(lambda),
            )),
            |(position, foreign_export, ident, _, lambda)| {
                FunctionDefinition::new(ident.name, lambda, foreign_export, position())
            },
        ),
    )(input)
}

fn foreign_export(input: Input) -> PineResult<ForeignExport> {
    context(
        "foreign export",
        map(
            preceded(keyword("foreign"), opt(calling_convention)),
            |calling_convention| ForeignExport::new(calling_convention.unwrap_or_default()),
        ),
    )(input)
}



pub(crate) fn type_alias(input: Input) -> PineResult<TypeAlias> {
    context(
        "type alias",
        map(
            tuple((position, keyword("type"), parse_identifier, sign("="), cut(type_))),
            |(position, _, ident, _, type_)| TypeAlias::new(ident.name, type_, position()),
        ),
    )(input)
}

pub fn type_(input: Input) -> PineResult<Type> {
    context("type", alt((into(function_type), union_type)))(input)
}

fn function_type(input: Input) -> PineResult<types::Function> {
    context(
        "function type",
        map(
            tuple((
                position,
                sign("\\("),
                cut(tuple((
                    separated_or_terminated_list0(sign(","), type_),
                    sign(")"),
                    type_,
                ))),
            )),
            |(position, _, (arguments, _, result))| {
                types::Function::new(arguments, result, position())
            },
        ),
    )(input)
}

fn union_type(input: Input) -> PineResult<Type> {
    map(separated_list1(sign("|"), atomic_type), |types| {
        types
            .into_iter()
            .reduce(|lhs, rhs| types::Union::new(lhs.clone(), rhs, lhs.position().clone()).into())
            .unwrap()
    })(input)
}

fn list_type(input: Input) -> PineResult<types::List> {
    context(
        "list type",
        map(
            tuple((position, sign("["), cut(terminated(type_, sign("]"))))),
            |(position, _, element)| types::List::new(element, position()),
        ),
    )(input)
}

fn map_type(input: Input) -> PineResult<types::Map> {
    context(
        "map type",
        map(
            tuple((
                position,
                sign("{"),
                cut(tuple((type_, sign(":"), type_, sign("}")))),
            )),
            |(position, _, (key, _, value, _))| types::Map::new(key, value, position()),
        ),
    )(input)
}

fn atomic_type(input: Input) -> PineResult<Type> {
    alt((
        into(reference_type),
        into(list_type),
        into(map_type),
        preceded(sign("("), cut(terminated(type_, sign(")")))),
    ))(input)
}

fn reference_type(input: Input) -> PineResult<types::Reference> {
    context(
        "reference type",
        map(
            tuple((position, token(qualified_identifier))),
            |(position, parse_identifier)| types::Reference::new(parse_identifier, position()),
        ),
    )(input)
}

fn block(input: Input) -> PineResult<Block> {
    context(
        "block",
        map(
            tuple((
                position,
                sign("{"),
                cut(terminated(
                    verify(many1(statement), |statements: &[_]| {
                        statements
                            .last()
                            .map(|statement| statement.name().is_none())
                            .unwrap_or_default()
                    }),
                    sign("}"),
                )),
            )),
            |(position, _, statements)| {
                Block::new(
                    statements[..statements.len() - 1].to_vec(),
                    statements.last().unwrap().expression().clone(),
                    position(),
                )
            },
        ),
    )(input)
}

pub fn statement(input: Input) -> PineResult<Statement> {
    context(
        "statement",
        map(
            tuple((position, opt(terminated(parse_identifier, sign("="))), expression)),
            |(position, ident, expression)| Statement::new(ident
                                                               .map(|identifier| identifier.name), expression, position()),
        ),
    )(input)
}

pub fn expression(input: Input) -> PineResult<Expression> {
    context(
        "expression",
        map(
            tuple((
                prefix_operation_like,
                many0(map(
                    tuple((position, binary_operator, cut(prefix_operation_like))),
                    |(position, operator, expression)| (operator, expression, position()),
                )),
            )),
            |(expression, pairs)| reduce_operations(expression, &pairs),
        ),
    )(input)
}

fn binary_operator(input: Input) -> PineResult<BinaryOperator> {
    context(
        "binary operator",
        alt((
            value(BinaryOperator::Add, sign("+")),
            value(BinaryOperator::Subtract, sign("-")),
            value(BinaryOperator::Multiply, sign("*")),
            value(BinaryOperator::Divide, sign("/")),
            value(BinaryOperator::Equal, sign("==")),
            value(BinaryOperator::NotEqual, sign("!=")),
            value(BinaryOperator::LessThanOrEqual, sign("<=")),
            value(BinaryOperator::LessThan, sign("<")),
            value(BinaryOperator::GreaterThanOrEqual, sign(">=")),
            value(BinaryOperator::GreaterThan, sign(">")),
            value(BinaryOperator::And, sign("&")),
            value(BinaryOperator::Or, sign("|")),
        )),
    )(input)
}

fn prefix_operation_like(input: Input) -> PineResult<Expression> {
    alt((into(prefix_operation), into(suffix_operation_like)))(input)
}

fn prefix_operation(input: Input) -> PineResult<UnaryOperation> {
    context(
        "prefix operation",
        map(
            tuple((position, prefix_operator, cut(prefix_operation_like))),
            |(position, operator, expression)| {
                UnaryOperation::new(operator, expression, position())
            },
        ),
    )(input)
}

fn prefix_operator(input: Input) -> PineResult<UnaryOperator> {
    context("prefix operator", value(UnaryOperator::Not, sign("!")))(input)
}

fn suffix_operation_like(input: Input) -> PineResult<Expression> {
    map(
        tuple((atomic_expression, many0(suffix_operator))),
        |(expression, suffix_operators)| {
            suffix_operators
                .into_iter()
                .fold(expression, |expression, operator| match operator {
                    SuffixOperator::Call(arguments, position) => {
                        Call::new(expression, arguments, position).into()
                    }
                    SuffixOperator::RecordField(name, position) => {
                        RecordDeconstruction::new(expression, name, position).into()
                    }
                    SuffixOperator::Try(position) => {
                        UnaryOperation::new(UnaryOperator::Try, expression, position).into()
                    }
                })
        },
    )(input)
}

fn suffix_operator(input: Input) -> PineResult<SuffixOperator> {
    alt((call_operator, record_field_operator, try_operator))(input)
}

fn call_operator(input: Input) -> PineResult<SuffixOperator> {
    // Do not allow any space before parentheses.
    context(
        "call",
        map(
            tuple((
                peek(position),
                tag("("),
                cut(terminated(
                    separated_or_terminated_list0(sign(","), expression),
                    sign(")"),
                )),
            )),
            |(position, _, arguments)| SuffixOperator::Call(arguments, position()),
        ),
    )(input)
}

fn record_field_operator(input: Input) -> PineResult<SuffixOperator> {
    context(
        "record field",
        map(
            tuple((position, sign("."), cut(parse_identifier))),
            |(position, _, identifier)| SuffixOperator::RecordField(identifier.name, position()),
        ),
    )(input)
}

fn try_operator(input: Input) -> PineResult<SuffixOperator> {
    context(
        "try operator",
        map(tuple((position, sign("?"))), |(position, _)| {
            SuffixOperator::Try(position())
        }),
    )(input)
}

fn atomic_expression(input: Input) -> PineResult<Expression> {
    alt((
        into(lambda),
        into(if_type),
        into(if_list),
        into(if_map),
        into(if_),
        into(parse_record),
        into(list_comprehension),
        into(list_literal),
        into(map_literal),
        into(number_literal),
        into(string_literal),
        into(variable),
        delimited(sign("("), expression, sign(")")),
    ))(input)
}

fn lambda(input: Input) -> PineResult<Lambda> {
    context(
        "function",
        map(
            tuple((
                position,
                sign("\\("),
                cut(tuple((
                    separated_or_terminated_list0(sign(","), argument),
                    sign(")"),
                    type_,
                    block,
                ))),
            )),
            |(position, _, (arguments, _, result_type, body))| {
                Lambda::new(arguments, result_type, body, position())
            },
        ),
    )(input)
}

fn argument(input: Input) -> PineResult<Argument> {
    context(
        "argument",
        map(
            tuple((position, parse_identifier, cut(type_))),
            |(position, ident, type_)| Argument::new(ident.name, type_, position()),
        ),
    )(input)
}

fn if_(input: Input) -> PineResult<If> {
    context(
        "if",
        map(
            tuple((
                position,
                keyword("if"),
                cut(tuple((
                    if_branch,
                    many0(preceded(
                        tuple((keyword("else"), keyword("if"))),
                        cut(if_branch),
                    )),
                    keyword("else"),
                    block,
                ))),
            )),
            |(position, _, (first_branch, branches, _, else_block))| {
                If::new(
                    [first_branch].into_iter().chain(branches).collect(),
                    else_block,
                    position(),
                )
            },
        ),
    )(input)
}

fn if_branch(input: Input) -> PineResult<IfBranch> {
    map(tuple((expression, block)), |(expression, block)| {
        IfBranch::new(expression, block)
    })(input)
}

fn if_list(input: Input) -> PineResult<IfList> {
    context(
        "if list",
        map(
            tuple((
                position,
                keyword("if"),
                sign("["),
                cut(tuple((
                    parse_identifier,
                    sign(","),
                    sign("..."),
                    parse_identifier,
                    sign("]"),
                    sign("="),
                    expression,
                    block,
                    keyword("else"),
                    block,
                ))),
            )),
            |(position, _, _, (first_name, _, _, rest_name, _, _, argument, then, _, else_))| {
                IfList::new(argument, first_name.name, rest_name.name, then, else_, position())
            },
        ),
    )(input)
}

fn if_map(input: Input) -> PineResult<IfMap> {
    context(
        "if map",
        map(
            tuple((
                position,
                keyword("if"),
                parse_identifier,
                sign("="),
                expression,
                sign("["),
                cut(tuple((
                    expression,
                    sign("]"),
                    block,
                    keyword("else"),
                    block,
                ))),
            )),
            |(position, _, ident, _, map, _, (key, _, then, _, else_))| {
                IfMap::new(ident.name, map, key, then, else_, position())
            },
        ),
    )(input)
}

fn if_type(input: Input) -> PineResult<IfType> {
    context(
        "if type",
        map(
            tuple((
                position,
                keyword("if"),
                parse_identifier,
                sign("="),
                expression,
                keyword("as"),
                cut(tuple((
                    if_type_branch,
                    many0(preceded(
                        tuple((keyword("else"), keyword("if"))),
                        cut(if_type_branch),
                    )),
                    opt(preceded(keyword("else"), block)),
                ))),
            )),
            |(position, _, parse_identifier, _, argument, _, (first_branch, branches, else_))| {
                IfType::new(
                    parse_identifier.name,
                    argument,
                    [first_branch].into_iter().chain(branches).collect(),
                    else_,
                    position(),
                )
            },
        ),
    )(input)
}

fn if_type_branch(input: Input) -> PineResult<IfTypeBranch> {
    map(tuple((type_, block)), |(type_, block)| {
        IfTypeBranch::new(type_, block)
    })(input)
}



fn number_literal(input: Input) -> PineResult<Number> {
    context(
        "number",
        map(
            token(tuple((
                position,
                alt((binary_literal, hexadecimal_literal, decimal_literal)),
                peek(not(digit1)),
            ))),
            |(position, number, _)| Number::new(number, position()),
        ),
    )(input)
}

fn binary_literal(input: Input) -> PineResult<NumberRepresentation> {
    context(
        "binary literal",
        map(
            preceded(tag("0b"), cut(many1(one_of("01")))),
            |characters| NumberRepresentation::Binary(String::from_iter(characters)),
        ),
    )(input)
}

fn hexadecimal_literal(input: Input) -> PineResult<NumberRepresentation> {
    context(
        "hexadecimal literal",
        map(
            preceded(tag("0x"), cut(many1(hexadecimal_digit))),
            |characters| {
                NumberRepresentation::Hexadecimal(String::from_iter(characters).to_lowercase())
            },
        ),
    )(input)
}

fn hexadecimal_digit(input: Input) -> PineResult<char> {
    one_of("0123456789abcdefABCDEF")(input)
}

fn decimal_literal(input: Input) -> PineResult<NumberRepresentation> {
    context(
        "decimal literal",
        map(recognize_float, |characters: Input| {
            NumberRepresentation::FloatingPoint(
                str::from_utf8(characters.as_bytes()).unwrap().into(),
            )
        }),
    )(input)
}

fn string_literal(input: Input) -> PineResult<ByteString> {
    context("string", token(raw_string_literal))(input)
}

fn raw_string_literal(input: Input) -> PineResult<ByteString> {
    map(
        tuple((
            position,
            preceded(
                char('"'),
                cut(terminated(
                    many0(alt((
                        recognize(none_of("\\\"")),
                        tag("\\\\"),
                        tag("\\\""),
                        tag("\\n"),
                        tag("\\r"),
                        tag("\\t"),
                        recognize(tuple((tag("\\x"), count(hexadecimal_digit, 2)))),
                    ))),
                    char('"'),
                )),
            ),
        )),
        |(position, spans)| {
            ByteString::new(
                spans
                    .iter()
                    .map(|span| str::from_utf8(span.as_bytes()).unwrap())
                    .collect::<Vec<_>>()
                    .concat(),
                position(),
            )
        },
    )(input)
}

fn list_literal(input: Input) -> PineResult<List> {
    context(
        "list",
        map(
            tuple((
                position,
                sign("["),
                cut(tuple((
                    type_,
                    separated_or_terminated_list0(sign(","), list_element),
                    sign("]"),
                ))),
            )),
            |(position, _, (type_, elements, _))| List::new(type_, elements, position()),
        ),
    )(input)
}

fn list_element(input: Input) -> PineResult<ListElement> {
    alt((
        map(
            preceded(sign("..."), cut(expression)),
            ListElement::Multiple,
        ),
        map(expression, ListElement::Single),
    ))(input)
}

fn list_comprehension(input: Input) -> PineResult<Expression> {
    context(
        "list comprehension",
        map(
            tuple((
                position,
                sign("["),
                type_,
                expression,
                many1(list_comprehension_branch),
                sign("]"),
            )),
            |(position, _, type_, element, branches, _)| {
                ListComprehension::new(type_, element, branches, position()).into()
            },
        ),
    )(input)
}

fn list_comprehension_branch(input: Input) -> PineResult<ListComprehensionBranch> {
    context(
        "list comprehension branch",
        map(
            tuple((
                position,
                keyword("for"),
                cut(tuple((
                    separated_or_terminated_list1(sign(","), parse_identifier),
                    keyword("in"),
                    separated_or_terminated_list1(sign(","), expression),
                    opt(preceded(keyword("if"), expression)),
                ))),
            )),
            |(position, _, (element_names, _, iteratees, condition))| {

                let element_names: Vec<String> = element_names
                    .iter()
                    .map(|identifier| identifier.name.clone())  // 提取 name 字段并克隆
                    .collect();

                ListComprehensionBranch::new(element_names, iteratees, condition, position())
            },
        ),
    )(input)
}

fn map_literal(input: Input) -> PineResult<Map> {
    context(
        "map",
        map(
            tuple((
                position,
                sign("{"),
                cut(tuple((
                    type_,
                    sign(":"),
                    type_,
                    separated_or_terminated_list0(sign(","), map_element),
                    sign("}"),
                ))),
            )),
            |(position, _, (key_type, _, value_type, elements, _))| {
                Map::new(key_type, value_type, elements, position())
            },
        ),
    )(input)
}

fn map_element(input: Input) -> PineResult<MapElement> {
    alt((
        map(
            tuple((position, expression, sign(":"), cut(expression))),
            |(position, key, _, value)| MapEntry::new(key, value, position()).into(),
        ),
        map(preceded(sign("..."), cut(expression)), MapElement::Multiple),
    ))(input)
}
pub fn variable(input: Input) -> PineResult<Variable> {
    context(
        "variable",
        map(
            tuple((position, token(qualified_identifier))),
            |(position, parse_identifier)| Variable::new(parse_identifier, position()),
        ),
    )(input)
}





pub fn keyword(name: &'static str) -> impl FnMut(Input) -> PineResult<()> {
    if !KEYWORDS.contains(&name) {
        unreachable!("undefined keyword");
    }

    move |input| {
        context(
            "keyword",
            value(
                (),
                token(tuple((
                    tag(name),
                    peek(not(alt((value((), alphanumeric1), value((), char('_')))))),
                ))),
            ),
        )(input)
    }
}

pub fn sign(sign: &'static str) -> impl Fn(Input) -> PineResult<()> + Clone {
    move |input| {
        let parser = context("sign", token(tag(sign)));

        if sign
            .chars()
            .any(|character| OPERATOR_CHARACTERS.contains(character))
        {
            value((), tuple((parser, peek(not(one_of(OPERATOR_MODIFIERS))))))(input)
        } else {
            value((), parser)(input)
        }
    }
}

pub fn token<'a, O>(
    mut parser: impl Parser<Input<'a>, O, NomError<'a>>,
) -> impl FnMut(Input<'a>) -> PineResult<'a, O> {
    move |input| {
        let (input, _) = blank(input)?;

        parser.parse(input)
    }
}

pub fn blank(input: Input) -> PineResult<()> {
    value(
        (),
        many0_count(alt((value((), multispace1), skipped_comment))),
    )(input)
}

pub fn comment(input: Input) -> PineResult<Comment> {
    context(
        "comment",
        map(
            tuple((comment_position, tag("#"), many0(none_of("\n\r")))),
            |(position, _, characters)| Comment::new(String::from_iter(characters), position),
        ),
    )(input)
}

// Optimize comment parsing by skipping contents.
fn skipped_comment(input: Input) -> PineResult<()> {
    value((), pair(tag("#"), many0_count(none_of("\n\r"))))(input)
}

fn comment_position(input: Input) -> PineResult<Position> {
    let (input, _) = multispace0(input)?;
    let input_clone=input.clone();
    Ok((input, input::position(input_clone)))
}

// Allocate position objects lazily.


pub fn position(input: Input) -> PineResult<impl Fn() -> Position + '_> {
    let (input, _) = blank(input)?;

    // 使用闭包包裹 Position 返回
    Ok((input.clone(), move || input::position(input.clone())))
}