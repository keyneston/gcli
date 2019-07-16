extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs;

mod fmt;
mod graphqlclient;

static version: &'static str = "0.1";

struct Config {
    colour: bool,
    queryFile: String,
}

fn create_config(matches: clap::ArgMatches) -> Config {
    return Config {
        colour: true,
        queryFile: matches
            .value_of("queryFile")
            .unwrap_or("query.graphql")
            .to_string(),
    };
}

fn main() {
    let matches = App::new("gcli")
        .version(version)
        .subcommand(
            SubCommand::with_name("fmt")
                .about("format graphql files")
                .arg(
                    Arg::with_name("fmtFile")
                        .help("The name of the file to format")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("query")
                .about("make graphql query")
                .arg(
                    Arg::with_name("queryFile")
                        .help("The name of the file to load the query from")
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("fmt") => fmt_command(matches.subcommand_matches("fmt").unwrap()),
        Some("query") => query_command(matches.subcommand_matches("query").unwrap()),
        None => {
            println!("Must set subcommand");
        }
        _ => {
            println!("Unknown subcommand");
        }
    }
}

fn fmt_command(matches: &ArgMatches) {
    let file = matches.value_of("fmtFile").unwrap();
    println!("calling fmt on: {}", file);

    fmt::format_file(file);
}

fn query_command(matches: &ArgMatches) {
    let client = graphqlclient::Client::new("http://localhost:8080/query");
    client.get_schema();
}

fn make_graphql_query() {
    unimplemented!();
}

fn read_query_file(filename: &str) -> String {
    println!("reading query file: {}", filename);
    let query = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("query: {}", query);

    return query;
}
