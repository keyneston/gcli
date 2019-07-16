extern crate nom;
use std::fs;

use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
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
    Ok((
        input,
        GraphqlQuery {
            queryType: QueryType::Query,
        },
    ))
}
