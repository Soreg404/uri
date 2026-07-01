mod uri_byte_classes;
pub use uri_byte_classes::UriByte;

mod parser;

pub mod codec;

#[derive(Default)]
struct FromTo {
    pub from: usize,
    pub to: usize
}
#[derive(Default)]
struct UrlCacheable {
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

pub struct Url<'a> {
    pub scheme: Option<&'a [u8]>,
    pub host: Option<&'a [u8]>,
    pub port: Option<u16>,
    pub path: &'a [u8],
    pub query: Option<&'a [u8]>,
    pub fragment: Option<&'a [u8]>
}

