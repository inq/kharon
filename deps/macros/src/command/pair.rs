use std::str::FromStr;

use failure;
use proc_macro::TokenTree;

use common::{Command, Key};

#[derive(Debug)]
pub struct Pair {
    pub keys: Vec<Key>,
    pub command: Command<String>,
}

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    #[fail(display = "invalid input token: {}", _0)]
    InvalidToken(String),
    #[fail(display = "empty token")]
    Empty,
}

impl Error {
    fn wrap(self) -> failure::Error {
        failure::Error::from(self)
    }

    fn invalid_token(cur: Option<proc_macro::TokenTree>) -> failure::Error {
        let cur_str = format!("{:?}", cur.map(|inner| inner.to_string()));
        Error::InvalidToken(cur_str).wrap()
    }
}

impl Pair {
    pub fn parse(it: &mut proc_macro::token_stream::IntoIter) -> Result<Self, failure::Error> {
        let keys = match it.next() {
            Some(TokenTree::Literal(lit)) => Self::parse_keys(lit.to_string().trim_matches('"')),
            None => Err(Error::Empty.wrap()),
            cur => Err(Error::invalid_token(cur)),
        }?;
        match it.next() {
            Some(TokenTree::Punct(ref punc)) if punc.as_char() == ':' => Ok(()),
            cur => Err(Error::invalid_token(cur)),
        }?;
        let command = match it.next() {
            Some(TokenTree::Group(group)) => Self::parse_attrs(group.stream()),
            cur => Err(Error::invalid_token(cur)),
        }?;
        Ok(Self {
            keys: keys,
            command: command,
        })
    }

    fn parse_keys(sequence_str: &str) -> Result<Vec<common::Key>, failure::Error> {
        let res: Result<_, _> = sequence_str.split(' ').map(FromStr::from_str).collect();
        Ok(res?)
    }

    fn parse_attrs(stream: proc_macro::TokenStream) -> Result<Command<String>, failure::Error> {
        let mut it = stream.into_iter();
        let name = match it.next() {
            Some(TokenTree::Literal(lit)) => Ok(lit.to_string().trim_matches('"').to_string()),
            cur => Err(Error::invalid_token(cur)),
        }?;
        match it.next() {
            Some(TokenTree::Punct(ref punc)) if punc.as_char() == ',' => Ok(()),
            cur => Err(Error::invalid_token(cur)),
        }?;
        let function = match it.next() {
            Some(TokenTree::Literal(lit)) => Ok(lit.to_string().trim_matches('"').to_string()),
            cur => Err(Error::invalid_token(cur)),
        }?;
        Ok(Command::new(name.to_string(), function.to_string()))
    }
}
