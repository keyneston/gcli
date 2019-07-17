extern crate nom;
use std::fs;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, space0},
    combinator::{map_res, opt, peek},
    error::make_error,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub enum QueryType {
    Query,
    Mutation,
}

struct QueryAST {}

#[derive(Debug, PartialEq)]
struct GraphqlQuery {
    queryType: QueryType,
}

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

fn parse_query(input: &str) -> IResult<&str, GraphqlQuery> {
    let (input, queryType) = parse_query_type(input)?;

    Ok((
        input,
        GraphqlQuery {
            queryType: queryType,
        },
    ))
}

fn parse_query_type(input: &str) -> IResult<&str, QueryType> {
    let (input, _) = space0(input)?;
    let (input, queryType) = opt(alt((tag("query"), tag("mutation"))))(input)?;
    let (input, _) = space0(input)?;
    let (input, _) = char('{')(input)?;

    match queryType {
        Some("query") => Ok((input, QueryType::Query)),
        Some("mutation") => Ok((input, QueryType::Mutation)),
        Some(queryType) => unimplemented!(),
        None => Ok((input, QueryType::Query)),
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

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

    fn default_parse_query_type() {
        let input = "{";
        let (_, queryType) = parse_query_type(input).unwrap();

        assert_eq!(queryType, QueryType::Query);
    }
}
