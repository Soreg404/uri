#![forbid(unsafe_code)]

macro_rules! trace {
    ($ctx:expr) => {
        {
            if matches!(option_env!("URI_TRACE"), Some(s) if s != "0") {
                println!("\x1b[36mtrace!\x1b[0m ({}:{:04}) [uri_parse] {}", file!(), line!(), $ctx);
            }
        }
    }
}

mod uri_byte_classes;
pub use uri_byte_classes::UriByte;

pub mod codec;

mod parser;
pub use parser::UriParser;

mod uri_path;
mod uri_query;

mod uri_debug_view;

#[derive(Default)]
pub struct UriAuthority<'a> {
    pub host: &'a [u8],
    pub port: Option<u16>,
}
#[derive(Default)]
pub struct Uri<'a> {
    pub scheme: Option<&'a [u8]>,
    pub authority: Option<UriAuthority<'a>>,
    pub path: &'a [u8],
    pub query: Option<&'a [u8]>,
    pub fragment: Option<&'a [u8]>
}

#[derive(Default)]
pub struct UriOpaque<'a> {
    pub scheme: &'a [u8],
    pub path: &'a [u8],
    pub fragment: Option<&'a [u8]>
}

pub enum UriVariant<'a> {
    Hier(Uri<'a>),
    Opaq(UriOpaque<'a>)
}
#[derive(Debug, Default, Eq, PartialEq)]
struct FromTo {
    pub from: usize,
    pub to: usize
}
#[derive(Debug, Default, Eq, PartialEq)]
pub struct UriCacheable {
    scheme: Option<FromTo>,
    host: Option<FromTo>,
    port: Option<u16>,
    path: FromTo,
    query: Option<FromTo>,
    fragment: Option<FromTo>
}
impl UriCacheable {
    pub fn as_uri<'a, 'b>(&'a self, original_bytes: &'b [u8]) -> UriVariant<'b> {
        let b = original_bytes;
        let path = &b[self.path.from..self.path.to];

        if let Some(scheme) = &self.scheme && !path.starts_with(b"/") {
            UriVariant::Opaq(UriOpaque {
                scheme: &b[scheme.from..scheme.to],
                path,
                fragment: self.fragment.as_ref().map(|v| &b[v.from..v.to]),
            })
        } else {
            let authority = if let Some(ft) = &self.host {
                Some(UriAuthority {
                    host: &b[ft.from..ft.to],
                    port: self.port,
                })
            }
            else { None };
            UriVariant::Hier(Uri {
                scheme: self.scheme.as_ref().map(|v| &b[v.from..v.to]),
                authority,
                path,
                query: self.query.as_ref().map(|v| &b[v.from..v.to]),
                fragment: self.fragment.as_ref().map(|v| &b[v.from..v.to]),
            })
        }
    }
}

