#![forbid(unsafe_code)]

macro_rules! trace {
    ($ctx:expr) => {
        {
            #![cfg(any(test, trace))]
            println!("\x1b[36mtrace!\x1b[0m ({}:{:04}) [uri_parse] {}", file!(), line!(), $ctx);
        }
    }
}

mod uri_byte_classes;
pub use uri_byte_classes::UriByte;

pub mod codec;

mod parser;
pub use parser::UriParser;

mod uri_conversions;

mod uri_path;
mod uri_query;

mod uri_debug_view;

pub struct UriAuthority<'a> {
    pub host: &'a [u8],
    pub port: Option<u16>
}
pub struct Uri<'a> {
    pub scheme: Option<&'a [u8]>,
    pub authority: Option<UriAuthority<'a>>,
    pub path: uri_path::PathParts,
    pub query: Option<&'a [u8]>,
    pub fragment: Option<&'a [u8]>
}
pub struct UriOpaque<'a> {
    pub scheme: &'a [u8],
    pub path: &'a [u8],
    pub fragment: Option<&'a [u8]>
}


#[derive(Default)]
pub struct UriAuthorityRaw<'a> {
    pub host_raw: &'a [u8],
    pub port: Option<u16>,
}
#[derive(Default)]
pub struct UriRaw<'a> {
    pub scheme: Option<&'a [u8]>,
    pub authority_raw: Option<UriAuthorityRaw<'a>>,
    pub path_raw: &'a [u8],
    pub query_raw: Option<&'a [u8]>,
    pub fragment_raw: Option<&'a [u8]>
}
impl UriRaw<'_> {
    pub fn is_abs_path(&self) -> bool {
        self.path_raw.starts_with(b"/")
    }
}
impl<'a> Uri<'a> {
    /// convenience wrapper for `UriPath::parse_decode` on `self.path_raw`
    pub fn path_parse_decode<'tmp, 'persistent>(
        &'a self,
        decode_scratch_buffer: &'tmp mut [u8],
        arena: &'persistent mut [u8]
    ) -> 
}

#[derive(Default)]
pub struct UriOpaqueRaw<'a> {
    pub scheme: Option<&'a [u8]>,
    pub path_raw: &'a [u8],
    pub fragment_raw: Option<&'a [u8]>
}
impl UriOpaqueRaw<'a> {
    pub fn to_decoded<'tmp, 'persistent>(
        &'a self,
        decode_scratch_bufer: &'tmp mut [u8],
        all_arena: &'persistent mut [u8]
    ) -> UriOpaque<'persistent> {
    }
    /// convenience wrapper for `codec::decode` on `self.path_raw`
    pub fn path_decode<'persistent>(
        &'a self,
        arena: &'persistent mut [u8]
    ) -> &'persistent [u8] {
        codec::decode(self.path_raw, arena)
    }
}

pub enum UriVariant<'a> {
    Hier(UriRaw<'a>),
    Opaq(UriOpaqueRaw<'a>)
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

        if self.scheme.is_some() && !self.path.starts_with(b"/") {
            UriVariant::Opaq(UriOpaqueRaw {
                scheme: &b[self.scheme.from..self.scheme.to],
                path_raw: &b[self.path.from..self.path.to],
                fragment_raw: self.fragment.as_ref().map(|v| &b[v.from..v.to]),
            })
        } else {
            let authority_raw = if let Some(ft) = self.host {
                Some(UriAuthorityRaw {
                    host_raw: &b[ft.from..ft.to],
                    port: self.port,
                })
            }
            else { None }
            UriVariant::Hier(UriRaw {
                scheme: self.scheme.as_ref().map(|v| &b[v.from..v.to]),
                authority_raw,
                path_raw: &b[self.path.from..self.path.to],
                query_raw: self.query.as_ref().map(|v| &b[v.from..v.to]),
                fragment_raw: self.fragment.as_ref().map(|v| &b[v.from..v.to]),
            })
        }
    }
}

