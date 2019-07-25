use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, PartialEq)]
pub enum QueryType {
    Query,
    Mutation,
    Subscription,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Element {
    Str(String),
    BlockStr(String),
    Num(String),
    Name(String),
    Bool(bool),
    Struct(SelectionSet),
}

pub type Args = BTreeMap<Element, Element>;

// impl Args {
//     fn new() -> Self {
//         BTreeMap::new()
//     }
// }

// impl Hash for Args {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         for (key, val) in self.iter() {
//             key.hash(state);
//             val.hash(state);
//         }
//     }
// }

#[derive(Debug, PartialEq)]
pub struct Document {
    definitions: Vec<Definition>,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, Ord, PartialOrd)]
pub enum SelectionItem {
    Field {
        name: Element,
        selection: Option<Box<SelectionSet>>,
        arguments: Option<Args>,
    },
    FragmentSpread(String),
    InlineFragment,
}

impl SelectionItem {
    pub fn new_field(name: &str) -> SelectionItem {
        return SelectionItem::Field {
            name: Element::Name(name.to_string()),
            selection: None,
            arguments: None,
        };
    }

    pub fn add(&mut self, new_field: SelectionItem) {
        if let SelectionItem::Field {
            ref mut selection, ..
        } = self
        {
            match selection {
                Some(fields) => fields.push(new_field),
                None => *selection = Some(Box::new(vec![new_field])),
            }
        }
    }
}

pub type SelectionSet = VecDeque<SelectionItem>;

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
    pub name: Option<String>,
    pub query_type: QueryType,
    pub query_params: Option<Args>,
    pub selection_set: SelectionSet,
}

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub text: String,
}
