extern crate nom;
use std::fs;

// TODO: figure this out
#[path = "../ast/mod.rs"]
mod ast;

use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_while1, take_while_m_n},
    character::complete::{char, space0},
    character::is_alphabetic,
    character::is_alphanumeric,
    combinator::{map_res, opt, peek},
    error,
    error::VerboseError,
    sequence::tuple,
    AsChar, IResult,
};

use ast::{Operation, QueryType};

pub fn format_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename).unwrap();

    return format_string(&contents);
}

pub fn format_string(input: &str) -> String {
    let res = parse_query(input);

    match res {
        Ok((_, query)) => println!("{:#?}", query),
        _ => return "".to_string(),
    }

    return "".to_string();
}

fn parse_query(input: &str) -> IResult<&str, Operation> {
    let (input, queryType) = parse_query_type(input)?;

    Ok((
        input,
        Operation {
            QueryType: queryType,
            QueryParams: None,
        },
    ))
}

fn parse_query_type(input: &str) -> IResult<&str, QueryType> {
    let (input, _) = space0(input)?;
    let (input, queryType) = opt(alt((tag_no_case("query"), tag_no_case("mutation"))))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('{')(input)?;

    let queryType = queryType.map(|s: &str| {
        let mut s = s.to_string();
        s.make_ascii_lowercase();
        s
    });

    match queryType.as_ref().map(String::as_str) {
        Some("query") => Ok((input, QueryType::Query)),
        Some("mutation") => Ok((input, QueryType::Mutation)),
        Some(queryType) => panic!("don't know how to handle queryType {}", queryType), // TODO: make a custom error format
        None => Ok((input, QueryType::Query)),
    }
}

fn parse_query_params(input: &str) -> IResult<&str, ()> {
    //delimited(char('\"'), parse_str, char('\"'))(input)
    unimplemented!()
}

fn parse_variable_name(input: &str) -> IResult<&str, String> {
    let (input, sigil) = char('$')(input)?;
    let (input, name) = take_while1(AsChar::is_alphanum)(input)?;

    let mut s = sigil.to_string();
    s.push_str(name);

    return Ok((input, s));
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parse_var_name() {
        let input = "$foo";

        let (_, varName) = parse_variable_name(input).unwrap();
        assert_eq!(varName, "$foo");
    }

    #[test]
    fn mutation_parse_query_type() {
        let input = "mutation {";
        let (_, queryType) = parse_query_type(input).unwrap();

        assert_eq!(queryType, QueryType::Mutation);
    }

    #[test]
    fn query_parse_query_type() {
        let input = "query {";
        let (_, queryType) = parse_query_type(input).unwrap();

        assert_eq!(queryType, QueryType::Query);
    }

    #[test]
    fn casesensitive_query_parse_query_type() {
        let input = "QUERY {";
        let (_, queryType) = parse_query_type(input).unwrap();

        assert_eq!(queryType, QueryType::Query);
    }

    #[test]
    fn default_parse_query_type() {
        let input = "{";
        let (_, queryType) = parse_query_type(input).unwrap();

        assert_eq!(queryType, QueryType::Query);
    }
}
