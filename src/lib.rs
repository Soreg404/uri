#![forbid(unsafe_code)]

// todo: fix name inconsistencies: Ur(l) / Ur(i)

mod uri_byte_classes;
pub use uri_byte_classes::UriByte;

mod parser;
pub use parser::UrlParser;

pub mod codec;

#[derive(Default)]
struct FromTo {
    pub from: usize,
    pub to: usize
}
#[derive(Default)]
pub struct UrlCacheable {
    scheme: Option<FromTo>,
    host: Option<FromTo>,
    port: Option<u16>,
    path: FromTo,
    query: Option<FromTo>,
    fragment: Option<FromTo>
}
impl UrlCacheable {
    pub fn as_url<'a, 'b>(&'a self, original_bytes: &'b [u8]) -> Url<'b> {
        let b = original_bytes;
        Url {
            scheme: self.scheme.as_ref().map(|v| &b[v.from..v.to]),
            host_raw: self.host.as_ref().map(|v| &b[v.from..v.to]),
            port: self.port,
            path_raw: &b[self.path.from..self.path.to],
            query_raw: self.query.as_ref().map(|v| &b[v.from..v.to]),
            fragment_raw: self.fragment.as_ref().map(|v| &b[v.from..v.to]),
        }
    }
}

#[derive(Default)]
pub struct Url<'a> {
    pub scheme: Option<&'a [u8]>,
    pub host_raw: Option<&'a [u8]>,
    pub port: Option<u16>,
    pub path_raw: &'a [u8],
    pub query_raw: Option<&'a [u8]>,
    pub fragment_raw: Option<&'a [u8]>
}
impl Url<'_> {
    pub fn is_abs_path(&self) -> bool {
        self.path_raw.starts_with(b"/")
    }
}

mod uri_debug_view;

// todo: url_path? uri_path? change later
mod url_path;
