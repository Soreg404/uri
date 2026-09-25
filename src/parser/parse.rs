
/*
 * BNF described in [RFC2396](https://datatracker.ietf.org/doc/html/rfc2396)
 *
 * logic described in uri_parser_state_description.txt
 */

// todo: maybe change to flow resolver instead of stateful

use crate::{ FromTo, UriCacheable, UriByte };

#[derive(Debug)]
pub enum ParseState {
    Scheme,
    HasScheme,
    Opaque,
    NoScheme,
    StartsFromAuthority,
    Authority,
    Port,
    StartsFromPath,
    Path,
    Query,
    Fragment,
    OpaqueFragment,
}


pub fn parse(
    bytes: &[u8],
    starting_state: ParseState
) -> Result<UriCacheable, &'static str> {
    trace!("parse begin");

    let mut state = starting_state;
    let mut word_start = 0usize;
    let mut i = 0usize;

    /// todo: avoid this ugly excessive invariance
    let mut scheme = None::<&[u8]>;
    let mut host = None::<&[u8]>;
    let mut port = None::<u16>;
    let mut path = &[];
    let mut query = None::<&[u8]>;

    let mut invalid_scheme_flag = false;

    let mut debug_loop_terminator = 0;
    let ret = loop {
        debug_loop_terminator += 1;
        if debug_loop_terminator == bytes.len() * 2 + 10 {
            panic!("loop terminated at bytes.len() * 2 + 10");
        }

        trace!(debug_print_current_state(&state, &bytes));

        state = match state {
            ParseState::Scheme => {
                match bytes.get(i) {
                    None => {
                        trace!("scheme: eot -> no_scheme:");
                        ParseState::NoScheme
                    }
                    Some(c) => {
                        if !c.is_uric() && c != b'#' {
                            trace!("scheme: c is not uric -> error!");
                            break Err("illegal byte");
                        }
                        match c {
                            b':' => {
                                trace!(format!("scheme: scheme is <<<{}>>> -> has_scheme:",
                                        String::from_utf8_lossy(&bytes[..i])));
                                scheme = Some(&bytes[..i]);
                                i += 1;
                                ParseState::HasScheme
                            }
                            b'/' | b'?' | b'#' => {
                                trace!("scheme: no scheme found -> no_scheme:");
                                ParseState::NoScheme
                            }
                            c => {
                                if !c.is_uri_scheme_allowed(i == 0) {
                                    /// todo: should fail if scheme is expected
                                    trace!("scheme: not uri allowe char -> no_scheme:");
                                    ParseState::NoScheme
                                } else {
                                    i += 1;
                                    ParseState::Scheme
                                }
                            }
                        }
                    }
                }
            }
            ParseState::HasScheme => {
                match bytes.get(i) {
                    None | Some(b'?') | Some(b'#') => {
                        trace!("has_scheme: c is one of '?' | '#' | $ -> error!");
                        break Err("invalid uri: invalid byte after scheme");
                    }
                    Some(b'/') => {
                        match bytes.get(i + 1) {
                            Some(b'/') => {
                                trace!("has_scheme: starts with '//' -> authority:");
                                i += 2;
                                word_start = i;
                                ParseState::Authority
                            }
                            _ => {
                                trace!("has_scheme: starts with '/' -> path:");
                                word_start = i;
                                i += 1;
                                ParseState::Path
                            }
                        }
                    },
                    Some(c) => {
                        if !c.is_uric() {
                            trace!("has_scheme: not uric -> error!");
                            break Err("illegal byte");
                        }
                        trace!("has_scheme: starts with uric -> opaque_part:");
                        word_start = i;
                        i += 1;
                        ParseState::OpaquePart
                    }
                }
            }
            ParseState::Opaque => {
                match bytes.get(i) {
                    None | Some(b'#') => {
                        path = &bytes[word_start..i];
                        trace!(format!("opaque_part: path is <<<{}>>>",
                                String::from_utf8_lossy(path)));
                        match bytes.get(i) {
                            Some(b'#') => {
                                trace!("opaque_part: -> opaq_frag:");
                                i += 1;
                                word_start = i;
                                ParseState::OpaqueFragment
                            }
                            _ => {
                                trace!("opaque_part: -> done!");
                                break Ok(todo!("return ok"))
                            }
                        }
                    }
                    Some(c) => {
                        if !c.is_uric() {
                            trace!("opaque_part: c is not uric -> error!");
                            break Err("illegal byte");
                        }
                        i += 1;
                        ParseState::Opaque
                    }
                }
            }
            ParseState::NoScheme => {
                if bytes.starts_with(b"//") {
                    trace!("no_scheme: starts with '//' -> authority:");
                    i += 2;
                    word_start = i;
                    ParseState::Authority
                } else {
                    trace!("no_scheme: -> starts_from_path:");
                    ParseState::StartsFromPath
                }
            }
            ParseState::StartsFromAuthority => {
                if !bytes.starts_with(b"//") {
                    trace!("starts_from_authority: did not start with '//' -> error!");
                    break Err("expected authority");
                }
                trace!("starts_from_authority: -> authority:");
                i += 2;
                word_start = i;
                ParseState::Authority
            }
            ParseState::Authority => {
                match bytes.get(i) {
                    None | Some(b':' | b'/' | b'?' | b'#') => {
                        if i == word_start {
                            trace!("authority: empty host -> error!");
                            break Err("empty host");
                        }
                        trace!(format!("authority: host is <<<{}>>>",
                                String::from_utf8_lossy(&bytes[word_start..i])));
                        host = Some(&bytes[word_start..i]);
                        if let Some(b':') = bytes.get(i) {
                            trace!("authority: -> port:");
                            i += 1;
                            ParseState::Port
                        } else {
                            trace!("authority: -> path:");
                            word_start = i;
                            ParseState::Path
                        }
                    }
                    Some(c) => {
                        if !c.is_uric() {
                            trace!("authority: c is not uric -> error!");
                            break Err("illegal byte");
                        }
                        i += 1;
                        ParseState::Authority
                    }
                }
            }
            ParseState::Port => {
                match bytes.get(i) {
                    None | Some(b'/' | b'?' | b'#') => {
                        match port {
                            None => {
                                trace!("port: eot, empty port -> error!");
                                break Err("empty port")
                            }
                            Some(port) => {
                                trace!(format!("port: port is {} -> path:", port));
                                word_start = i;
                                ParseState::Path
                            }
                        }
                    }
                    Some(c) => {
                        if !c.is_ascii_digit() {
                            trace!("port: c is not digit -> error!");
                            break Err("invalid uri: non-digit in port");
                        }
                        let c = c - b'0';
                        port = Some(port.unwrap_or(0) * 10 + c as u16);
                        i += 1;
                        ParseState::Port
                    }
                }
            }
            ParseState::StartsFromPath => {
                trace!("starts_from_path: -> path:");
                word_start = 0;
                ParseState::Path
            }
            ParseState::Path => {
                match bytes.get(i) {
                    None | Some(b'?' | b'#') => {
                        path = &bytes[word_start..i];
                        trace!(format!("path: path is <<<{}>>>",
                                String::from_utf8_lossy(path)));
                        match bytes.get(i) {
                            None => {
                                trace!("path: eot -> done!");
                                break Ok(todo!("return ok"))
                            }
                            Some(b'?') => {
                                trace!("path: -> query:");
                                i += 1;
                                word_start = i;
                                ParseState::Query
                            }
                            Some(b'#') => {
                                trace!("path: -> fragment:");
                                i += 1;
                                word_start = i;
                                ParseState::Fragment
                            }
                        }
                    }
                    Some(c) => {
                        if !c.is_uric() {
                            trace!("path: c is not uric -> error!");
                            break Err("illegal byte");
                        }
                        i += 1;
                        ParseState::Path
                    }
                }
            }
            ParseState::Query => {
                match bytes.get(i) {
                    None | Some(b'#') => {
                        trace!(format!("query: query is <<<{}>>>",
                                String::from_utf8_lossy(&bytes[word_start..i])));
                        query = Some(&bytes[word_start..i]);
                        match bytes.get(i) {
                            None => {
                                trace!("query: eot -> done!");
                                break Ok(todo!("return ok"))
                            }
                            Some(b'#') => {
                                trace!("query: -> fragment:");
                                i += 1;
                                word_start = i;
                                ParseState::Fragment
                            }
                        }
                    }
                    Some(c) => {
                        if !c.is_uric() {
                            trace!("query: c is not uric -> error!");
                            break Err("illegal byte");
                        }
                        i += 1;
                        ParseState::Query
                    }
                }
            }
            ParseState::Fragment | ParseState::OpaqueFragment => {
                match bytes.get(i) {
                    None => {
                        let frag = &bytes[word_start..i];
                        trace!(format!("fragment: fragment is <<<{}>>> -> done!",
                                String::from_utf8_lossy(frag)));
                        match state {
                            ParseState::Fragment => break Ok(todo!("return ok")),
                            ParseState::OpaqueFragment => break Ok(todo!("return ok")),
                            _ => unreachable!()
                        }
                    }
                    Some(c) => {
                        if !c.is_uric() {
                            trace!("fragment: c is not uric -> error!");
                            break Err("illegal byte");
                        }
                        i += 1;
                        state
                    }
                }
            }
        }
    };

    ret
}


fn debug_print_current_state(state: &ParseState, bytes: &[u8]) -> String {
    let (bytes_str, bytes_more_str) = {
        if bytes.len() > 10 {
            (String::from_utf8_lossy(&bytes[..10]), format!("[+{}]", bytes.len() - 10))
        } else {
            (String::from_utf8_lossy(bytes), "".to_string())
        }
    };
    format!("\x1b[90mparse loop; state={: <15};
        bytes <<<{bytes_str}>>>{bytes_more_str}\x1b[0m",
        format!("{:?},", state))
}
