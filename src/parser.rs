
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
    StartsWithAuthority,
    Authority,
    Port,
    StartsWithPath,
    ContinueWithPath,
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
            println!("\x1b[36mtrace!\x1b[0m ({:03}): {}", line!(), $ctx);
        }
    }

    'state_loop: loop {
        trace!(format!("state loop, state={:?}", state));

        match state.clone() {
            UrlParseState::Scheme => {
                let c = bytes[i];

                trace!(format!("scheme begin, i={}, c={:?}, s={:?}",
                        i,
                        c as char,
                        str::from_utf8(&bytes[i..])));
                if !c.is_uric() {
                    return Err(());
                }
                match c {
                    b':' => {
                        trace!(format!("scheme c={:?}, invalid_scheme_flag={:?}",
                                c as char,
                                invalid_scheme_flag));
                        if invalid_scheme_flag {
                            return Err(());
                        }
                        trace!(format!("scheme, saving scheme: {:?}",
                            str::from_utf8(&bytes[..i])));
                        ret.scheme = Some(FromTo { from: 0, to: i });
                        state = UrlParseState::HasScheme;
                    }
                    b'/' | b'?' | b'#' => {
                        state = UrlParseState::NoScheme;
                    }
                    _ => {}
                }
                if !invalid_scheme_flag && !c.is_uri_scheme_allowed(i == 0) {
                    invalid_scheme_flag = true;
                }
                if i + 1 == bytes.len() {
                    state = UrlParseState::NoScheme;
                }
                i += 1;
            }
            UrlParseState::HasScheme => {
                trace!("has_scheme begin");
                match bytes.get(i) {
                    None => unreachable!(),
                    Some(b'?') | Some(b'#') => return Err(()),
                    Some(b'/') => {
                        match bytes.get(i + 1) {
                            Some(b'/') => {
                                state = UrlParseState::Authority;
                                word_start = i + 2;
                                i += 2;
                            }
                            _ => {
                                state = UrlParseState::Path;
                                word_start = i;
                                i += 1;
                            }
                        }
                    },
                    Some(c) => {
                        if c.is_uric() {
                            state = UrlParseState::OpaquePart;
                            word_start = i;
                            i += 1;
                        } else {
                            return Err(());
                        }
                    }
                }
            }
            UrlParseState::OpaquePart => todo!(),
            UrlParseState::NoScheme => {
                trace!("no_scheme begin");
                if bytes[i..].starts_with(b"//") {
                    state = UrlParseState::Authority;
                    word_start = i + 2;
                    i += 2;
                } else {
                    state = UrlParseState::StartsWithPath;
                }
            }
            UrlParseState::StartsWithAuthority => {
                if !bytes.starts_with(b"//") {
                    return Err(());
                }
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
                    ret.host = Some(FromTo { from: word_start, to: i });
                    if has_port {
                        state = UrlParseState::Port;
                        i += 1;
                    } else {
                        state = UrlParseState::Path
                    }
                } else {
                    i += 1;
                }
            }
            UrlParseState::Port => {
                if i == bytes.len() {
                    break 'state_loop;
                }
                match bytes[i] {
                    b'/' | b'?' | b'#' => {
                        state = UrlParseState::Path;
                    }
                    c => {
                        if !c.is_ascii_digit() {
                            return Err(());
                        }
                        let c = c - b'0';
                        ret.port = Some(ret.port.unwrap_or(0) * 10 + c as u16);
                    }
                }
                i += 1;
            }
            UrlParseState::StartsWithPath => {
                word_start = 0;
                state = UrlParseState::Path
            }
            UrlParseState::ContinueWithPath => {
                word_start = i;
                state = UrlParseState::Path
            }
            UrlParseState::Path => {
                match bytes.get(i) {
                    None | Some(b'?') | Some(b'#') => {
                        ret.path = FromTo { from: word_start, to: i };
                        if i == bytes.len() {
                            break 'state_loop;
                        }
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
                        ret.query = Some(FromTo { from: word_start, to: i });
                        if i == bytes.len() {
                            break 'state_loop;
                        }
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_eq_utf8_opt {
        ($e:expr, $to:literal) => {
            let _: &Option<&[u8]> = &$e;
            assert_eq!($e.map(|v| str::from_utf8(v)), Some(Ok($to)));
        }
    }

    #[test]
    fn basic_usage() {
        let sample_abs = b"http://example.com:80/some/where?query=string#fragment";
        let r = parse(sample_abs, UrlParseState::Scheme).unwrap();
        let r = r.as_url(sample_abs);
        assert_eq_utf8_opt!(r.scheme, "http");
    }

    mod edge_cases {
        use super::*;

        #[test]
        fn zero_len() {
            todo!()
        }

        #[test]
        fn eof_before_scheme_parsed() {
            let sample = b"valid-scheme-without-colon";
            // expect ref_path
            todo!()
        }

        mod authority {
            use super::*;

            #[test]
            fn abcd() {
                // with / without scheme
                // with / without port
                // with / without path following
                // with no authority given
                todo!()
            }
        }
    } 
}
