
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
 * uric = reserved | unreserved
 *
 * reserved = ";" | "/" | "?" | ":" | "@" | "&" | "=" | "+" | "$" | ","
 *
 * unreserved = alphanum | mark
 *
 * mark = "-" | "_" | "." | "!" | "~" | "*" | "'" | "(" | ")"
 *
 */

use crate::uri_byte_classes::UriByte;

#[derive(Default)]
pub struct FromTo {
    pub from: usize,
    pub to: usize
}

pub struct UrlParts {
    pub scheme: Option<FromTo>,
    pub host: Option<FromTo>,
    pub port: Option<u16>,
    pub path: FromTo,
    pub query: Option<FromTo>,
    pub fragment: Option<FromTo>
}

#[derive(Debug)]
pub enum ParseError {
    TBD(usize, String)
}

pub fn parse(bytes: &[u8]) -> Result<UrlParts, ParseError> {
    let mut idx = 0;
    let mut scheme = None::<FromTo>;
    while idx < bytes.len() {
        let c = bytes[idx];
        if c == b':' {
            // technically, a uri scheme does not have to be followed by "//"
            // but http uris ussually (if not always) have it
            //
            // todo: to be uri compliant, add support for net_path, abs_path and opaque_part
            // ref: ^1 RFC2396, section 3.
            if bytes[idx + 1..].starts_with(b"//") {
                if idx == 0 { panic!(); }
                // found a scheme
                let s = FromTo {
                    from: 0,
                    to: idx
                };
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
    let mut host = FromTo::default();
    while idx < bytes.len() {
        let c = bytes[idx];
        if c == b':' {
            has_port = true;
            host = FromTo {
                from: authority_start,
                to: idx
            };
            break;
        }
        if c == b'/' || c == b'?' || c == b'#' {
            host = FromTo {
                from: authority_start,
                to: idx
            };
            break;
        }
        if !c.is_uric() {
            panic!();
        }
        if idx + 1 == bytes.len(){
            idx += 1;
            host = FromTo {
                from: authority_start,
                to: idx
            };
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
    let mut path = FromTo::default();
    while idx < bytes.len() {
        let c = bytes[idx];
        if c == b'?' || c == b'#' {
            path = FromTo {
                from: path_start, 
                to: idx
            };
            break;
        }
        if !c.is_uric() {
            panic!();
        }
        if idx + 1 == bytes.len() {
            idx += 1;
            path = FromTo {
                from: path_start, 
                to: idx
            };
            break;
        }
        idx += 1;
    }

    let mut query = None::<FromTo>;
    if idx < bytes.len() && bytes[idx] == b'?' {
        idx += 1;
        let qs_start = idx;
        while idx < bytes.len() {
            let c = bytes[idx];
            if c == b'#' {
                query = Some(FromTo { from: qs_start, to: idx});
                break;
            }
            if !c.is_uric() {
                panic!();
            }
            if idx + 1 == bytes.len() {
                idx += 1;
                query = Some(FromTo { from: qs_start, to: idx});
                break;
            }
            idx += 1;
        }
    }
    
    let mut fragment = None::<FromTo>;
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
        fragment = Some(FromTo { from: frag_start, to: idx});
    }

    Ok(UrlParts {
        scheme,
        host: Some(host),
        port,
        path,
        query,
        fragment
    })
}
