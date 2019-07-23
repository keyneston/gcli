extern crate nom;
use std::fs;

// TODO: figure this out
#[path = "../ast/mod.rs"]
mod ast;

use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_till, take_while, take_while1, take_while_m_n},
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
    let (input, query_type) = parse_query_type(input)?;

    Ok((
        input,
        Operation {
            query_type: query_type,
            query_params: None,
        },
    ))
}

fn parse_query_type(input: &str) -> IResult<&str, QueryType> {
    let (input, _) = space0(input)?;
    let (input, query_type) = opt(alt((tag_no_case("query"), tag_no_case("mutation"))))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('{')(input)?;

    let query_type = query_type.map(|s: &str| {
        let mut s = s.to_string();
        s.make_ascii_lowercase();
        s
    });

    match query_type.as_ref().map(String::as_str) {
        Some("query") => Ok((input, QueryType::Query)),
        Some("mutation") => Ok((input, QueryType::Mutation)),
        Some(query_type) => panic!("don't know how to handle query_type {}", query_type), // TODO: make a custom error format
        None => Ok((input, QueryType::Query)),
    }
}

fn is_eol(c: char) -> bool {
    c == '\n'
}

fn is_whitespace(c: char) -> bool {
    c == '\n' || c == ' ' || c == '\t'
}

fn take_whitespace(input: &str) -> IResult<&str, ()> {
    let (input, _) = take_while(is_whitespace)(input)?;

    Ok((input, ()))
}

fn parse_comment(input: &str) -> IResult<&str, ast::Comment> {
    let (input, _) = take_whitespace(input)?;
    let (input, _) = char('#')(input)?;
    let (input, comment_text) = take_till(is_eol)(input)?;
    let (input, _) = take_whitespace(input)?;

    return Ok((
        input,
        ast::Comment {
            text: comment_text.to_string(),
        },
    ));
}

fn parse_query_params(input: &str) -> IResult<&str, ()> {
    let (input, _) = parse_seperator('(')(input)?;
    let (input, var_name) = parse_variable_name(input)?;
    let (input, _) = parse_seperator(':')(input)?;
    let (input, _thing) = take_while1(AsChar::is_alphanum)(input)?;
    let (input, _) = parse_seperator(')')(input)?;

    Ok((input, ()))
}

fn parse_variable_name(input: &str) -> IResult<&str, String> {
    let (input, sigil) = char('$')(input)?;
    let (input, name) = take_while1(AsChar::is_alphanum)(input)?;

    let mut s = sigil.to_string();
    s.push_str(name);

    return Ok((input, s));
}

fn parse_seperator(sep: char) -> impl Fn(&str) -> IResult<&str, ()> {
    move |input: &str| -> IResult<&str, ()> {
        let (input, _) = take_whitespace(input)?;
        let (input, _) = char(sep)(input)?;
        let (input, _) = take_whitespace(input)?;

        Ok((input, ()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_var_name() {
        let input = "$foo: baz_bar";

        let (_, var_name) = parse_variable_name(input).unwrap();
        assert_eq!(var_name, "$foo");
    }

    #[test]
    fn mutation_parse_query_type() {
        let input = "mutation {";
        let (_, query_type) = parse_query_type(input).unwrap();

        assert_eq!(query_type, QueryType::Mutation);
    }

    #[test]
    fn query_parse_query_type() {
        let input = "query {";
        let (_, query_type) = parse_query_type(input).unwrap();

        assert_eq!(query_type, QueryType::Query);
    }

    #[test]
    fn casesensitive_query_parse_query_type() {
        let input = "QUERY {";
        let (_, query_type) = parse_query_type(input).unwrap();

        assert_eq!(query_type, QueryType::Query);
    }

    #[test]
    fn default_parse_query_type() {
        let input = "{";
        let (_, query_type) = parse_query_type(input).unwrap();

        assert_eq!(query_type, QueryType::Query);
    }

    #[test]
    fn test_parse_comment() {
        let input = "# This is a comment\nThis is not a comment";
        let (remaining, comment) = parse_comment(input).unwrap();

        assert_eq!(
            comment,
            ast::Comment {
                text: " This is a comment".to_string()
            }
        );
        assert_eq!(remaining, "This is not a comment");
    }
}
