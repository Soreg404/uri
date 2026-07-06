
/*
 * BNF described in [RFC2396](https://datatracker.ietf.org/doc/html/rfc2396)
 *
 * logic described in uri_parser_state_description.txt
 */

use crate::{ FromTo, UrlCacheable, UriByte };

#[derive(Debug, Clone)]
pub enum UrlParseState {
    Scheme,
    HasScheme,
    OpaquePart,
    NoScheme,
    StartsFromAuthority,
    Authority,
    Port,
    StartsFromPath,
    Path,
    Query,
    Fragment
}

pub fn parse(
    bytes: &[u8],
    starting_state: UrlParseState
) -> Result<UrlCacheable, &'static str> {
    trace!("parse begin");

    let mut ret = UrlCacheable::default();
    if bytes.is_empty() {
        trace!("empty bytes -> done!");
        return Ok(ret);
    }
    let mut state = starting_state;
    let mut i = 0usize;
    let mut loop_terminator = 0usize;
    let mut invalid_scheme_flag = false;
    let mut word_start = 0usize;

    'state_loop: loop {
        trace!(format!("\x1b[90mparse loop, state={: <15} i={i:05}, s={:?}\x1b[0m",
                format!("{:?},", state),
                if bytes[i..].len() > 10 {
                    str::from_utf8(&bytes[i..i + 10])
                } else {
                    str::from_utf8(&bytes[i..])
                }));

        match state.clone() {
            UrlParseState::Scheme => {
                let c = bytes[i];
                if !c.is_uric() && c != b'#' {
                    trace!("scheme: c is not uric -> error!");
                    return Err("illegal byte");
                }
                match c {
                    b':' => {
                        if invalid_scheme_flag {
                            trace!("scheme: invalid -> error!");
                            return Err("invalid scheme");
                        }
                        trace!(format!("scheme: scheme is {:?} -> has_scheme:",
                            str::from_utf8(&bytes[..i])));
                        ret.scheme = Some(FromTo { from: 0, to: i });
                        state = UrlParseState::HasScheme;
                        i += 1;
                    }
                    b'/' | b'?' | b'#' => {
                        trace!("scheme: no scheme found -> no_scheme:");
                        state = UrlParseState::NoScheme;
                    }
                    c => {
                        if !invalid_scheme_flag && !c.is_uri_scheme_allowed(i == 0) {
                            trace!("scheme: set $invalid_scheme flag");
                            invalid_scheme_flag = true;
                        }
                        if i + 1 == bytes.len() {
                            trace!("scheme: eot -> no_scheme:");
                            state = UrlParseState::NoScheme;
                        }
                        i += 1;
                    }
                }
            }
            UrlParseState::HasScheme => {
                match bytes.get(i) {
                    None | Some(b'?') | Some(b'#') => {
                        trace!("has_scheme: c is one of '?' | '#' | $ -> error!");
                        return Err("invalid uri: invalid byte after scheme");
                    }
                    Some(b'/') => {
                        match bytes.get(i + 1) {
                            Some(b'/') => {
                                trace!("has_scheme: starts with '//' -> authority:");
                                state = UrlParseState::Authority;
                                word_start = i + 2;
                                i += 2;
                            }
                            _ => {
                                trace!("has_scheme: starts with '/' -> path:");
                                state = UrlParseState::Path;
                                word_start = i;
                                i += 1;
                            }
                        }
                    },
                    Some(c) => {
                        if !c.is_uric() {
                            trace!("has_scheme: not uric -> error!");
                            return Err("illegal byte");
                        }
                        trace!("has_scheme: starts with uric -> opaque_part:");
                        state = UrlParseState::OpaquePart;
                        word_start = i;
                        i += 1;
                    }
                }
            }
            UrlParseState::OpaquePart => todo!(),
            UrlParseState::NoScheme => {
                if bytes.starts_with(b"//") {
                    trace!("no_scheme: starts with '//' -> authority:");
                    state = UrlParseState::Authority;
                    word_start = i + 2;
                    i += 2;
                } else {
                    trace!("no_scheme: -> starts_from_path:");
                    state = UrlParseState::StartsFromPath;
                }
            }
            UrlParseState::StartsFromAuthority => {
                if !bytes.starts_with(b"//") {
                    trace!("starts_from_authority: did not start with '//' -> error!");
                    return Err("expected authority");
                }
                trace!("starts_from_authority: -> authority:");
                state = UrlParseState::Authority;
                word_start = i + 2;
                i += 2;
            }
            UrlParseState::Authority => {
                let has_port = i < bytes.len() && bytes[i] == b':';
                if i == bytes.len() || match bytes[i] {
                    b':' | b'/' | b'?' | b'#' => true,
                    _ => false
                } {
                    trace!(format!("authority: host={:?}", str::from_utf8(&bytes[word_start..i])));
                    ret.host = Some(FromTo { from: word_start, to: i });
                    if has_port {
                        trace!("authority: -> port:");
                        state = UrlParseState::Port;
                        i += 1;
                    } else {
                        trace!("authority: -> path:");
                        word_start = i;
                        state = UrlParseState::Path
                    }
                } else {
                    i += 1;
                }
            }
            UrlParseState::Port => {
                if i == bytes.len() {
                    trace!("port: eot -> done!");
                    break 'state_loop;
                }
                match bytes[i] {
                    b'/' | b'?' | b'#' => {
                        trace!(format!("port: port is {:?} -> path:", ret.port));
                        word_start = i;
                        state = UrlParseState::Path;
                    }
                    c => {
                        if !c.is_ascii_digit() {
                            trace!("port: c is not digit -> error!");
                            return Err("invalid uri: non-digit in port");
                        }
                        let c = c - b'0';
                        ret.port = Some(ret.port.unwrap_or(0) * 10 + c as u16);
                        i += 1;
                    }
                }
            }
            UrlParseState::StartsFromPath => {
                trace!("starts_from_path: -> path:");
                word_start = 0;
                state = UrlParseState::Path
            }
            UrlParseState::Path => {
                ret.path = FromTo { from: word_start, to: i };
                if i == bytes.len() || bytes[i] == b'?' || bytes[i] == b'#' {
                    trace!(format!("path: path is {:?}", str::from_utf8(&bytes[word_start..i])));
                }
                match bytes.get(i) {
                    None => {
                        trace!("path: eot -> done!");
                        break 'state_loop;
                    }
                    Some(b'?') => {
                        trace!("path: -> query:");
                        state = UrlParseState::Query;
                        word_start = i + 1;
                        i += 1;
                    }
                    Some(b'#') => {
                        trace!("path: -> fragment:");
                        state = UrlParseState::Fragment;
                        word_start = i + 1;
                        i += 1;
                    }
                    Some(c) => {
                        if !c.is_uric() {
                            trace!("path: c is not uric -> error!");
                            return Err("illegal byte");
                        }
                        i += 1;
                    }
                }
            }
            UrlParseState::Query => {
                ret.query = Some(FromTo { from: word_start, to: i });
                if i == bytes.len() || bytes[i] == b'#' {
                    trace!(format!("query: query is {:?}",
                            str::from_utf8(&bytes[word_start..i])));
                }
                match bytes.get(i) {
                    None => {
                        trace!("query: eot -> done!");
                        break 'state_loop;
                    }
                    Some(b'#') => {
                        trace!("query: -> fragment:");
                        state = UrlParseState::Fragment;
                        word_start = i + 1;
                        i += 1;
                    }
                    Some(c) => {
                        if !c.is_uric() {
                            trace!("query: c is not uric -> error!");
                            return Err("illegal byte");
                        }
                        i += 1;
                    }
                }
            }
            UrlParseState::Fragment => {
                if i == bytes.len() {
                    ret.fragment = Some(FromTo { from: word_start, to: i });
                    trace!(format!("fragment: fragment is {:?} -> done!",
                            str::from_utf8(&bytes[word_start..i])));
                    break 'state_loop;
                }
                if !bytes[i].is_uric() {
                    trace!("fragment: c is not uric -> error!");
                    return Err("illegal byte");
                }
                i += 1;
            }
        }

        if loop_terminator == bytes.len() * 2 + 10 {
            panic!("loop terminated (len() * 2 + 10)");
        }
        loop_terminator += 1;
    }

    Ok(ret)
}
