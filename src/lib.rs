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
            host: self.host.as_ref().map(|v| &b[v.from..v.to]),
            port: self.port,
            path: &b[self.path.from..self.path.to],
            query: self.query.as_ref().map(|v| &b[v.from..v.to]),
            fragment: self.fragment.as_ref().map(|v| &b[v.from..v.to]),
        }
    }
}

#[derive(Default)]
pub struct Url<'a> {
    scheme: Option<&'a [u8]>,
    host: Option<&'a [u8]>,
    port: Option<u16>,
    path: &'a [u8],
    query: Option<&'a [u8]>,
    fragment: Option<&'a [u8]>
}
impl Url<'_> {
    pub fn scheme(&self) -> Option<&[u8]> {
        self.scheme
    }
    pub fn host(&self) -> Option<&[u8]> {
        self.host
    }
    pub fn port(&self) -> Option<u16> {
        self.port
    }
    pub fn path_encoded_raw(&self) -> &[u8] {
        self.path
    }
    pub fn query_encoded_raw(&self) -> Option<&[u8]> {
        self.query
    }
    pub fn fragment(&self) -> Option<&[u8]> {
        self.fragment
    }

    pub fn is_abs_path(&self) -> bool {
        self.path.starts_with(b"/")
    }
}

mod url_path;
