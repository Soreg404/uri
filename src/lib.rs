
/*
 *
 * ^1 URI RFC2396: https://datatracker.ietf.org/doc/html/rfc2396
 *
 *
 * full-url = scheme "://" url-without-scheme
 *
 * url-without-scheme = authority absolute-path url-rest
 *
 * authority = host [ ":" port ]
 *
 * absolute-path = "/" relative-path
 *
 * relative-path = [ segment ] [ / relative-path ]
 *
 * path = absolute-path / relative-path
 *
 *
 * segment = *( unreserved | escaped |
 *              ":" | "@" | "&" | "=" | "+" | "$" | "," )
 * 
 * url-rest = [ "?" query-string ] [ "#" fragment ]
 *
 *        
 * reserved    = ";" | "/" | "?" | ":" | "@" | "&" | "=" | "+" | "$" | ","
 *
 * unreserved = alphanum | mark
 *
 * mark = "-" | "_" | "." | "!" | "~" | "*" | "'" | "(" | ")"
 *
 */

mod uri_byte_classes;
pub use uri_byte_classes::UriByte;

pub mod codec;

pub fn parse(bytes: &[u8]) -> UrlIndexed {
    let mut idx = 0;
    let mut scheme = None::<Rdx>;
    while idx < bytes.len() {
        let c = bytes[idx];
        if c == b':' {
            // technically, a uri scheme does not have to be followed by "//"
            // but http uris ussually (if not always) have it
            //
            // todo: to be uri compliant, add support for net_path, abs_path and opaque_part
            // ref: ^1 RFC2396, section 3.
            if bytes[idx + 1..].starts_with(b"//") {
                // found a scheme
                let s = Rdx::new(0, idx);
                if s.len() == 0 { panic!(); }
                scheme = Some(s);
                idx += 3;
            }
            break;
        }
        if !c.is_uri_scheme_allowed(idx == 0) {
            // not a scheme
            break;
        }
        idx += 1;
    }

    if scheme.is_none() {
        // error if scheme is needed
        idx = 0;
    }

    // authority
    // todo: userinfo
    let authority_start = idx;
    let mut has_port = false;
    let mut host = Rdx::new(0, 0);
    while idx < bytes.len() {
        let c = bytes[idx];
        if c == b':' {
            has_port = true;
            host = Rdx::new(authority_start, idx);
            break;
        }
        if c == b'/' || c == b'?' || c == b'#' {
            host = Rdx::new(authority_start, idx);
            break;
        }
        if !c.is_uric() {
            panic!();
        }
        if idx + 1 == bytes.len(){
            idx += 1;
            host = Rdx::new(authority_start, idx);
            break;
        }
        idx += 1;
    }
    let mut port = None::<u16>;
    if has_port {
        idx += 1;
        let mut port_tmp = 0u16;
        while idx < bytes.len() {
            let c = bytes[idx];
            if c == b'/' || c == b'?' || c == b'#' || idx + 1 == bytes.len() {
                if idx + 1 == bytes.len() {
                    idx += 1;
                }
                port = Some(port_tmp);
                break;
            }
            if !c.is_ascii_digit() {
                panic!("non-digit byte in port");
            }
            port_tmp *= 10;
            port_tmp += (c - b'0') as u16;
            idx += 1;
        }
        if bytes[idx - 1] == b':' {
            panic!("no port given after ':'");
        }
    }

    let path_start = idx;
    let mut path = Rdx::new(0, 0);
    while idx < bytes.len() {
        let c = bytes[idx];
        if c == b'?' || c == b'#' {
            path = Rdx::new(path_start, idx);
            break;
        }
        if !c.is_uric() {
            panic!();
        }
        if idx + 1 == bytes.len() {
            idx += 1;
            path = Rdx::new(path_start, idx);
            break;
        }
        idx += 1;
    }

    let mut query = None::<Rdx>;
    if idx < bytes.len() && bytes[idx] == b'?' {
        idx += 1;
        let qs_start = idx;
        while idx < bytes.len() {
            let c = bytes[idx];
            if c == b'#' {
                query = Some(Rdx::new(qs_start, idx));
                break;
            }
            if !c.is_uric() {
                panic!();
            }
            if idx + 1 == bytes.len() {
                idx += 1;
                query = Some(Rdx::new(qs_start, idx));
                break;
            }
            idx += 1;
        }
    }
    
    let mut fragment = None::<Rdx>;
    if idx < bytes.len() && bytes[idx] == b'#' {
        idx += 1;
        let frag_start = idx;
        while idx < bytes.len() {
            let c = bytes[idx];
            if !c.is_uric() {
                panic!();
            }
            idx += 1;
        }
        fragment = Some(Rdx::new(frag_start, idx));
    }

    UrlIndexed {
        scheme,
        host: Some(host),
        port,
        path,
        query,
        fragment
    }
}

struct Rdx {
    from: usize,
    to: usize
}
impl Rdx {
    pub fn new(from: usize, to: usize) -> Self {
        assert!(from <= to);
        Self {
            from,
            to
        }
    }
    pub fn len(&self) -> usize {
        self.to - self.from
    }
    pub fn as_slice_of<'a>(&self, bytes: &'a [u8]) -> &'a [u8] {
        &bytes[self.from..self.to]
    }
    pub fn translate(&mut self, offset: usize) -> &mut Self {
        self.from += offset;
        self.to += offset;
        self
    }
}

pub struct UrlIndexed {
    pub scheme: Option<Rdx>,
    pub host: Option<Rdx>,
    pub port: Option<u16>,
    pub path: Rdx,
    pub query: Option<Rdx>,
    pub fragment: Option<Rdx>
}

pub struct Url<'a> {
    pub scheme: Option<&'a [u8]>,
    pub host: Option<&'a [u8]>,
    pub port: Option<u16>,
    pub path: &'a [u8],
    pub query: Option<&'a [u8]>,
    pub fragment: Option<&'a [u8]>
}

#[expect(unused)]
struct PathIter {

}



