

mod uri_byte_classes;
pub use uri_byte_classes::UriByte;

mod parser;
pub use parser::ParseError;

pub mod codec;

mod cacheable {
    pub use super::parser::FromTo;

    pub struct UrlCacheable {
        pub scheme: Option<FromTo>,
        pub host: Option<FromTo>,
        pub port: Option<u16>,
        pub path: FromTo,
        pub query: Option<FromTo>,
        pub fragment: Option<FromTo>
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

pub fn parse(bytes: &[u8]) -> Result<Url<'_>, ParseError> {
    let p = crate::parser::parse(bytes)?;
    Ok(Url {
        scheme: p.scheme.map(|ft| &bytes[ft.from..ft.to]),
        host: p.host.map(|ft| &bytes[ft.from..ft.to]),
        port: p.port,
        path: &bytes[p.path.from..p.path.to],
        query: p.query.map(|ft| &bytes[ft.from..ft.to]),
        fragment: p.fragment.map(|ft| &bytes[ft.from..ft.to]),
    })
}

#[expect(unused)]
struct PathIter {

}



