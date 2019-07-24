extern crate nom;
use std::collections::HashMap;
use std::fs;

// TODO: figure this out
#[path = "../ast/mod.rs"]
#[allow(dead_code)]
mod ast;

use nom::{
    branch::alt,
    bytes::complete::tag_no_case,
    bytes::complete::take_while1,
    character::complete::{char, line_ending, multispace0, not_line_ending, one_of, space0},
    combinator::opt,
    multi::{many1, separated_list},
    AsChar, IResult,
};

use ast::{Operation, QueryType, SelectionItem, SelectionItem::*, SelectionSet};

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
    let (input, query_params) = parse_query_params(input)?;

    Ok((
        input,
        Operation {
            query_type: query_type,
            query_params: Some(query_params),
        },
    ))
}

fn parse_selection_set(input: &str) -> IResult<&str, SelectionSet> {
    let (input, _) = parse_sigil('{')(input)?;
    let (input, res) = separated_list(seperator, parse_selection_item)(input)?;
    let (input, _) = multispace0(input)?; // consume any lingering whitespace.
    let (input, _) = parse_sigil('}')(input)?;

    Ok((input, res))
}

fn parse_selection_item(input: &str) -> IResult<&str, SelectionItem> {
    let (input, _) = multispace0(input)?;
    let (input, name) = take_while1(AsChar::is_alphanum)(input)?;
    let (input, sub_selection) = opt(parse_selection_set)(input)?;

    let new_field = Field {
        name: name.to_string(),
        selection: match sub_selection {
            Some(sub) => Some(Box::new(sub)),
            None => None,
        },
    };

    Ok((input, new_field))
}

fn parse_query_type(input: &str) -> IResult<&str, QueryType> {
    let (input, _) = space0(input)?;
    let (input, query_type) = opt(alt((
        tag_no_case("query"),
        tag_no_case("mutation"),
        tag_no_case("subscription"),
    )))(input)?;
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
        Some("subscription") => Ok((input, QueryType::Subscription)),
        Some(query_type) => panic!("don't know how to handle query_type {}", query_type), // TODO: make a custom error format
        None => Ok((input, QueryType::Query)),
    }
}

fn parse_comment(input: &str) -> IResult<&str, ast::Comment> {
    let (input, _) = space0(input)?;
    let (input, _) = char('#')(input)?;
    let (input, comment_text) = not_line_ending(input)?;
    let (input, _) = line_ending(input)?;

    return Ok((
        input,
        ast::Comment {
            text: comment_text.to_string(),
        },
    ));
}

fn parse_query_params(input: &str) -> IResult<&str, HashMap<ast::VarName, String>> {
    let (input, _) = parse_sigil('(')(input)?;
    let (input, var_name) = parse_variable_name(input)?;
    let (input, _) = parse_sigil(':')(input)?;
    let (input, thing) = take_while1(AsChar::is_alphanum)(input)?;
    let (input, _) = parse_sigil(')')(input)?;

    let mut hash: HashMap<ast::VarName, String> = HashMap::new();
    hash.insert(var_name, thing.to_string());

    Ok((input, hash))
}

fn parse_variable_name(input: &str) -> IResult<&str, String> {
    let (input, sigil) = char('$')(input)?;
    let (input, name) = take_while1(AsChar::is_alphanum)(input)?;

    let mut s = sigil.to_string();
    s.push_str(name);

    return Ok((input, s));
}

fn parse_sigil(sigil: char) -> impl Fn(&str) -> IResult<&str, ()> {
    move |input: &str| -> IResult<&str, ()> {
        let (input, _) = space0(input)?;
        let (input, _) = char(sigil)(input)?;
        let (input, _) = space0(input)?;

        Ok((input, ()))
    }
}

fn seperator(input: &str) -> IResult<&str, ()> {
    let (input, _) = space0(input)?;
    let (input, _) = many1(one_of("\n\r,"))(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

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
    fn subscription_parse_query_type() {
        let input = "subscription {";
        let (_, query_type) = parse_query_type(input).expect("Error calling parse_query_type");

        assert_eq!(query_type, QueryType::Subscription);
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

    #[test]
    fn test_parse_selection_set() {
        let input = "{id, foo, bar}";
        let (_, selection_set) = parse_selection_set(input).unwrap();

        assert_eq!(
            selection_set,
            vec!(
                SelectionItem::new_field("id"),
                SelectionItem::new_field("foo"),
                SelectionItem::new_field("bar"),
            )
        )
    }

    #[test]
    fn newline_sepearted_test_parse_selection_set() {
        let input = "{id\nfoo\nbar}";
        let (_, selection_set) = parse_selection_set(input).unwrap();

        assert_eq!(
            selection_set,
            vec!(
                SelectionItem::new_field("id"),
                SelectionItem::new_field("foo"),
                SelectionItem::new_field("bar"),
            )
        )
    }

    #[test]
    fn recursive_test_parse_selection_set() {
        let input = "{
            id
            foo { baz, qaz }
            bar
        }";

        let mut recursive = SelectionItem::new_field("foo");
        recursive.add(SelectionItem::new_field("baz"));
        recursive.add(SelectionItem::new_field("qaz"));

        let (_, selection_set) = parse_selection_set(input).unwrap();

        assert_eq!(
            vec!(
                SelectionItem::new_field("id"),
                recursive,
                SelectionItem::new_field("bar"),
            ),
            selection_set,
        )
    }

    #[test]
    fn fields_test_parse_selection_item() {
        let input = "foo { a, b } blah }";
        // Mostly we are checking that this succeeds at all.
        let (remaining, field) = parse_selection_item(input).unwrap();

        let mut expected = SelectionItem::new_field("foo");
        expected.add(SelectionItem::new_field("a"));
        expected.add(SelectionItem::new_field("b"));

        assert_eq!(field, expected);
        assert_eq!(remaining, "blah }");
    }
}
