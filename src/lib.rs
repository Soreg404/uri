
/*
 *
 * URI RFC: https://datatracker.ietf.org/doc/html/rfc2396
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

fn c_reserved(c: u8) -> bool {
    match c {
        b';' | b'/' | b'?' | b':' | b'@' | b'&' | b'=' | b'+' | b'$' | b',' => true,
            _ => false
    }
}
fn c_unreserved(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c_mark(c)
}
fn c_mark(c: u8) -> bool {
    match c {
        b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')' => true,
        _ => false
    }
}
fn c_scheme_allowed(c: u8, is_first_char: bool) -> bool {
    if is_first_char {
        c.is_ascii_alphabetic()
    } else {
        c.is_ascii_alphanumeric() || match c {
            b'+' | b'-' | b'.' => true,
            _ => false
        }
    }
}

pub fn parse(bytes: &[u8]) -> UrlIndexed {
    let mut idx = 0;
    let mut scheme = None::<Rdx>;
    while idx < bytes.len() {
        let c = bytes[idx];
        if c == b':' {
            if bytes[idx + 1..].starts_with(b"//") {
                // found a scheme
                let s = Rdx::new(0, idx);
                if s.len() == 0 { panic!(); }
                scheme = Some(s);
                idx += 3;
            }
            break;
        }
        if !c_scheme_allowed(c, idx == 0) {
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
    let mut host = Rdx::new(0,0);
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
        if idx + 1 == bytes.len(){
            host = Rdx::new(authority_start, idx + 1);
            break;
        }
        if !c_reserved(c) && !c_unreserved(c) && c != b'%' {
            panic!();
        }
        idx += 1;
    }
    let mut port = None::<u16>;
    if has_port {
        idx += 1;
        let port_start = idx;
        while idx < bytes.len() {
            let c = bytes[idx];
            if c == b'/' || c == b'?' || c == b'#' || idx + 1 == bytes.len() {
                if idx + 1 == bytes.len() {
                    idx += 1;
                }
                let s = &bytes[port_start..idx];
                let mut b = 0u16;
                for c in s {
                    if !c.is_ascii_digit() {
                        panic!();
                    }
                    b *= 10;
                    b += (c - b'0') as u16;
                }
                port = Some(b);
                break;
            }
            idx += 1;
        }
    }

    let path_start = idx;
    let mut path = Rdx::new(0,0);
    while idx < bytes.len() {
        let c = bytes[idx];
        if c == b'?' || c == b'#' {
            path = Rdx::new(path_start, idx);
            break;
        }
        if idx + 1 == bytes.len() {
            idx += 1;
            path = Rdx::new(path_start, idx);
            break;
        }
        if !c_reserved(c) && !c_unreserved(c) && c != b'%' {
            panic!();
        }
        idx += 1;
    }

    let mut query = None::<Rdx>;
    if idx < bytes.len() && bytes[idx] == b'?' {
        idx += 1;
        let qs_start = idx;
        while idx < bytes.len() {
            if idx + 1 == bytes.len() {
                idx += 1;
                query = Some(Rdx::new(qs_start, idx));
                break;
            }
            if bytes[idx] == b'#' {
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
            if !c_reserved(c) && !c_unreserved(c) && c != b'%' {
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

// todo: this is not meant to be pub
pub struct Rdx {
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
