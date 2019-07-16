extern crate clap;
use clap::{App, Arg, ArgMatches, SubCommand};
use std::fs;

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
        .arg(
            Arg::with_name("queryFile")
                .help("The name of the file to load the query from")
                .required(true),
        )
        .get_matches();

    let config = create_config(matches);

    //read_query_file(&config.queryFile);

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
