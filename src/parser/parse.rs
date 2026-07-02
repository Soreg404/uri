
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
) -> Result<UrlCacheable, ()> {
    let mut ret = UrlCacheable::default();
    if bytes.is_empty() {
        return Ok(ret);
    }
    let mut state = starting_state;
    let mut i = 0usize;
    let mut loop_terminator = 0usize;
    let mut invalid_scheme_flag = false;
    let mut word_start = 0usize;

    macro_rules! trace {
        ($ctx:expr) => {
            {
                #![cfg(any(test, trace))]
                println!("\x1b[36mtrace!\x1b[0m ({:03}): {}", line!(), $ctx);
            }
        }
    }

    'state_loop: loop {
        trace!(format!("\x1b[90mparse loop, state={: <15} i={i:05}, s={:?}\x1b[0m",
                format!("{:?},", state),
                if bytes[i..].len() > 20 {
                    str::from_utf8(&bytes[i..i + 20])
                } else {
                    str::from_utf8(&bytes[i..])
                }));

        match state.clone() {
            UrlParseState::Scheme => {
                let c = bytes[i];
                if !c.is_uric() {
                    trace!("scheme: c is not uric -> error!");
                    return Err(());
                }
                match c {
                    b':' => {
                        if invalid_scheme_flag {
                            trace!("scheme: invalid -> error!");
                            return Err(());
                        }
                        trace!(format!("scheme: scheme is {:?} -> has_scheme:",
                            str::from_utf8(&bytes[..i])));
                        ret.scheme = Some(FromTo { from: 0, to: i });
                        state = UrlParseState::HasScheme;
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
                    }
                }
                if i + 1 == bytes.len() {
                    trace!("scheme: eot -> no_scheme:");
                    state = UrlParseState::NoScheme;
                }
                i += 1;
            }
            UrlParseState::HasScheme => {
                match bytes.get(i) {
                    None => unreachable!(),
                    Some(b'?') | Some(b'#') => {
                        trace!("has_scheme: c is one of '?' | '#' -> error!");
                        return Err(());
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
                        if c.is_uric() {
                            trace!("has_scheme: starts with uric -> opaque_part:");
                            state = UrlParseState::OpaquePart;
                            word_start = i;
                            i += 1;
                        } else {
                            trace!("has_scheme: not uric -> error!");
                            return Err(());
                        }
                    }
                }
            }
            UrlParseState::OpaquePart => todo!(),
            UrlParseState::NoScheme => {
                if bytes[i..].starts_with(b"//") {
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
                    return Err(());
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
                            return Err(());
                        }
                        let c = c - b'0';
                        ret.port = Some(ret.port.unwrap_or(0) * 10 + c as u16);
                    }
                }
                i += 1;
            }
            UrlParseState::StartsFromPath => {
                trace!("starts_from_path: -> path:");
                word_start = 0;
                state = UrlParseState::Path
            }
            UrlParseState::Path => {
                match bytes.get(i) {
                    None | Some(b'?') | Some(b'#') => {
                        trace!(format!("path: path is {:?}",
                                str::from_utf8(&bytes[word_start..i])));
                        ret.path = FromTo { from: word_start, to: i };
                        if i == bytes.len() {
                            trace!("path: eot -> done!");
                            break 'state_loop;
                        }
                        trace!("path: -> query:");
                        state = UrlParseState::Query;
                        word_start = i + 1;
                        i += 1;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            UrlParseState::Query => {
                match bytes.get(i) {
                    None | Some(b'#') => {
                        trace!(format!("query: query is {:?}",
                                str::from_utf8(&bytes[word_start..i])));
                        ret.query = Some(FromTo { from: word_start, to: i });
                        if i == bytes.len() {
                            trace!("query: eot -> done!");
                            break 'state_loop;
                        }
                        trace!("query: -> fragment:");
                        state = UrlParseState::Fragment;
                        word_start = i + 1;
                        i += 1;
                    }
                    _ => {
                        i += 1;
                    }
                }
            }
            UrlParseState::Fragment => {
                i = bytes.len();
                trace!(format!("fragment: fragment is {:?} -> done!",
                        str::from_utf8(&bytes[word_start..i])));
                ret.fragment = Some(FromTo { from: word_start, to: i });
                break 'state_loop;
            }
        }

        if i == bytes.len() {
            panic!("loophole, failed to get done");
        } else if loop_terminator == bytes.len() {
            panic!("loop terminated");
        }
        loop_terminator += 1;
    }

    Ok(ret)
}
