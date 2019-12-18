use std::collections::BTreeMap;

use proc_macro2::{Ident, TokenStream, Span};

use common::{Command, Key};
use crate::command::pair::Pair;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "the key vector is empty")]
    EmptyKey,
    #[fail(display = "duplicated leaf node: {}", _0)]
    Duplicated(String),
    #[fail(display = "cannot insert into a leaf node")]
    LeafNode,
}

#[derive(Debug)]
pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            root: Node::new_inner("R".to_string())
        }
    }

    pub fn add_pair(&mut self, pair: Pair) -> Result<(), Error> {
        self.root.add_pair(&pair.keys, pair.command)?;
        Ok(())
    }

    pub fn ids(&self) -> Vec<Ident> {
        self.root.ids()
    }

    pub fn transitions(&self) -> Vec<(Ident, TokenStream, TokenStream)> {
        self.root.transitions(None)
    }
}

#[derive(Debug)]
enum Node {
    Inner {
        id: String,
        children: BTreeMap<Key, Self>,
    },
    Leaf {
        id: String,
        command: Command<String>,
    },
}

impl Node {
    fn new_inner(id: String) -> Self {
        Node::Inner { id, children: BTreeMap::new() }
    }

    fn add_pair(&mut self, keys: &[Key], command: Command<String>) -> Result<(), Error> {
        if keys.is_empty() {
            return Err(Error::EmptyKey);
        }
        let (children, prefix) = if let Node::Inner{ref mut children, ref id} = self {
            (children, id)
        } else {
            return Err(Error::LeafNode);
        };
        let (ref head, ref tail) = keys.split_at(1);
        let key = head[0];
        let id = format!("{}{}", prefix, key.identifier());

        if tail.is_empty() {
            let child = Node::Leaf { id, command };
            if let Some(_existing) = children.insert(key, child) {
                return Err(Error::Duplicated(format!("{:?}", head)))
            };
        } else if let Some(child) = children.get_mut(&key) {
            child.add_pair(tail, command)?;
        } else {
            let mut child = Self::new_inner(id);
            child.add_pair(tail, command)?;
            children.insert(key, child);
        }
        Ok(())
    }

    fn ids(&self) -> Vec<Ident> {
        match self {
            Node::Inner { id, children } => {
                let mut res = vec![Ident::new(id, Span::call_site())];
                res.extend(children.values().flat_map(|n| n.ids()).collect::<Vec<_>>());
                res
            }
            Node::Leaf { .. } => vec![]
        }
    }

    fn transitions(&self, parent: Option<(Ident, Key)>) -> Vec<(Ident, TokenStream, TokenStream)> {
        match self {
            Node::Inner { id, children } => {
                let mut res = vec![];
                if let Some((from, key)) = parent {
                    let to = format!("Response::More(State::{})", id).parse().unwrap();
                    let key_stream = format!("Key::{:?}", key).parse().unwrap();
                    res.push((from, key_stream, to));
                }
                let to = Ident::new(id, Span::call_site());
                res.extend(
                    children
                        .iter()
                        .flat_map(|(k, v)| v.transitions(Some((to.clone(), *k))))
                        .collect::<Vec<_>>());
                res
            }
            Node::Leaf { command, .. } => {
                let to = format!(
                    r#"Response::Done(Command::new("{}".to_string(), Action::{}))"#,
                    command.name, 
                    command.action).parse().unwrap();
                if let Some((from, key)) = parent {
                    let key_stream = format!("Key::{:?}", key).parse().unwrap();
                    vec![(from, key_stream, to)]
                } else {
                    vec![]
                }
            }
        }
    }
}