#![deny(warnings)]
#![warn(unsafe_code)]
#![warn(clippy::panic)]

#[macro_use]
mod helpers;

mod byte_classes;

pub mod codec;

mod parser;
pub use parser::parse_options::{
    ParseOptions,
    parse,
};

mod path;
pub use path::path_parts;

mod query;

mod debug_view;

// todo: bring back UriCacheable somehow (core::range preferably)

pub struct Uri<'a> {
    pub scheme: Option<&'a [u8]>,
    pub authority: Option<UriAuthority<'a>>,
    pub path: &'a [u8],
    pub query: Option<&'a [u8]>,
    pub fragment: Option<&'a [u8]>
}
pub struct UriAuthority<'a> {
    pub host: &'a [u8],
    pub port: Option<u16>,
}

pub struct UriOpaque<'a> {
    pub scheme: &'a [u8],
    pub path: &'a [u8],
    // todo: add query
    pub fragment: Option<&'a [u8]>
}

#[derive(Debug)]
pub enum UriVariant<'a> {
    Hier(Uri<'a>),
    Opaq(UriOpaque<'a>)
}

