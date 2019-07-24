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

#[derive(Debug, PartialEq, Clone)]
pub enum SelectionItem {
    Field {
        name: String,
        selection: Option<Box<SelectionSet>>,
        //arguments: Option<Argu
    },
    FragmentSpread(String),
    InlineFragment,
}

impl SelectionItem {
    pub fn new_field(name: &str) -> SelectionItem {
        return SelectionItem::Field {
            name: name.to_string(),
            selection: None,
        };
    }

    pub fn add(&mut self, new_field: SelectionItem) {
        match self {
            SelectionItem::Field {
                ref mut selection, ..
            } => {
                let new_selection = match selection {
                    Some(fields) => {
                        let mut fields = fields.clone();
                        fields.push(new_field);
                        fields
                    }
                    None => Box::new(vec![new_field]),
                };
                *selection = Some(new_selection);
            }
            _ => return,
        };
    }
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
