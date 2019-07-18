use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum QueryType {
    Query,
    Mutation,
}

#[derive(Debug, PartialEq)]
pub struct Document {
    Definitions: Vec<Definition>,
}

/// [Definition](https://graphql.github.io/graphql-spec/June2018/#Definition)
#[derive(Debug, PartialEq)]
pub enum Definition {
    TypeSystem,
    TypeSystemExtension,
    Operation,
    Fragment,
}

#[derive(Debug, PartialEq)]
pub struct Operation {
    pub QueryType: QueryType,
    pub QueryParams: Option<HashMap<String, String>>,
}
