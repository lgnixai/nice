use nom::{
    IResult,
    bytes::complete::take_while1,
    character::complete::one_of,
    sequence::tuple,
    combinator::recognize,
};

fn accepted_operator_chars() -> Vec<char> {
    vec!['+', '-', '/', '*', '|', '<', '>', '=', '!', '$', '@', '&']
}

// Define a parser for a single character from accepted operators
fn parse_operator(input: &str) -> IResult<&str, &str> {
    recognize(one_of(
        accepted_operator_chars()
            .iter()
            .cloned()
            .collect::<String>()
            .as_str(),
    ))(input)
}

fn parse_tuple(input: &str) -> IResult<&str, (&str, &str)> {
    tuple((
        parse_operator,
        recognize(one_of(
            accepted_operator_chars()
                .iter()
                .cloned()
                .collect::<String>()
                .as_str(),
        )),
    ))(input)
}
#[test]
fn main() {
    let input = "+";
    let result = parse_operator(input);

    match result {
        Ok((remaining, a)) => {
            println!("Parsed operator 1: '{}',  Remaining input: '{}'", a, remaining);
        }
        Err(e) => println!("Error parsing tuple: {:?}", e),
    }
}
