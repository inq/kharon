#![recursion_limit="256"]
#![feature(slice_patterns)]

#[macro_use]
extern crate failure;
extern crate proc_macro;

mod command;

use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn command_map(input: TokenStream) -> TokenStream {
    use proc_macro2::{Ident, Span};

    let mut it = input.into_iter();
    let mut tree = command::Tree::new();
    let mut actions = Vec::new();
    loop {
        match command::Pair::parse(&mut it) {
            Err(ref err)
                if err.downcast_ref::<command::pair::Error>() == Some(&command::pair::Error::Empty) =>
            {
                break;
            }
            Err(err) => panic!("{}", err),
            Ok(res) => {
                actions.push(Ident::new(&res.command.action, Span::call_site()));
                tree.add_pair(res).unwrap();
            }
        }
    }
    // TODO(inkyu): Find a better way
    let froms = tree.transitions().iter().map(|x| x.0.clone()).collect::<Vec<_>>();
    let keys = tree.transitions().iter().map(|x| x.1.clone()).collect::<Vec<_>>();
    let nexts = tree.transitions().iter().map(|x| x.2.clone()).collect::<Vec<_>>();
    let idents = tree.ids();
    quote!(
        use hidden::{Response, State};
        pub use hidden::Action;
        mod hidden {
            use common::{Command, Key};

            #[derive(Debug)]
            pub enum Response {
                More(State),
                Done(Command<Action>),
                Empty,
            }

            #[derive(Debug, Clone, Copy)]
            pub enum State {
                #(#idents,)*
            }

            #[derive(Debug, Clone, Copy)]
            pub enum Action {
                #(#actions,)*
            }

            impl State {
                pub fn initial() -> Self {
                    State::R
                }

                pub fn handle_key(self, key: Key) -> Response {
                    match (self, key) {
                        #((State::#froms, #keys) => #nexts,)*
                        _ => Response::Empty,
                    }
                }
            }
        }
    ).into()
}
