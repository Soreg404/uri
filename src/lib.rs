
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

pub fn parse(bytes: &[u8]) -> Url<'_> {
    let mut idx = 0;
    let mut scheme = None::<&[u8]>;
    while idx < bytes.len() {
        let c = bytes[idx];
        if c == b':' {
            if bytes[idx + 1..].starts_with(b"//") {
                // found a scheme
                let s = &bytes[..idx];
                scheme = Some(s);
                if s.len() == 0 { panic!(); }
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
    let mut host: &[u8] = &[];
    while idx < bytes.len() {
        let c = bytes[idx];
        if c == b':' {
            has_port = true;
            host = &bytes[authority_start..idx];
            break;
        }
        if c == b'/' || c == b'?' || c == b'#' {
            host = &bytes[authority_start..idx];
            break;
        }
        if idx + 1 == bytes.len(){
            host = &bytes[authority_start..];
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
    let mut path: &[u8] = &[];
    while idx < bytes.len() {
        let c = bytes[idx];
        if c == b'?' || c == b'#' {
            path = &bytes[path_start..idx];
            break;
        }
        if idx + 1 == bytes.len() {
            path = &bytes[path_start..];
            idx += 1;
            break;
        }
        if !c_reserved(c) && !c_unreserved(c) && c != b'%' {
            panic!();
        }
        idx += 1;
    }

    let mut query = None::<&[u8]>;
    if idx < bytes.len() && bytes[idx] == b'?' {
        idx += 1;
        let qs_start = idx;
        while idx < bytes.len() {
            if idx + 1 == bytes.len() {
                query = Some(&bytes[qs_start..]);
                idx += 1;
                break;
            }
            if bytes[idx] == b'#' {
                query = Some(&bytes[qs_start..idx]);
                break;
            } 
            idx += 1;
        }
    }
    
    let mut fragment = None::<&[u8]>;
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
        fragment = Some(&bytes[frag_start..]);
    }

    Url {
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
    fn new(from: usize, to: usize) -> Self {
        assert!(from <= to);
        Self {
            from,
            to
        }
    }
    fn len(&self) -> usize {
        self.to - self.from
    }
    fn as_slice_of<'a>(&self, bytes: &'a [u8]) -> &'a [u8] {
        &bytes[self.from..self.to]
    }
    fn translate(&mut self, offset: usize) -> &mut Self {
        self.from += offset;
        self.to += offset;
        self
    }
}

// pub struct UrlIndexed {
//     scheme: Option<Scheme>,
//     host: Rdx,
//     port: Option<u16>,
//     path: Rdx,
//     query: Option<Rdx>,
//     fragment: Option<Rdx>
// }

pub struct Url<'a> {
    pub scheme: Option<&'a [u8]>,
    pub host: Option<&'a [u8]>,
    pub port: Option<u16>,
    pub path: &'a [u8],
    pub query: Option<&'a [u8]>,
    pub fragment: Option<&'a [u8]>
}
