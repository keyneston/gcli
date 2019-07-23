use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum QueryType {
    Query,
    Mutation,
    Subscription,
}

pub type VarName = String;

#[derive(Debug, PartialEq)]
pub struct Document {
    definitions: Vec<Definition>,
}

#[derive(Debug, PartialEq)]
pub enum SelectionItem {
    Field(String),
    FragmentSpread(String),
    InlineFragment,
}

pub type SelectionSet = Vec<SelectionItem>;

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
    pub query_type: QueryType,
    pub query_params: Option<HashMap<VarName, String>>,
}

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub text: String,
}
