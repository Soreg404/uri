use super::parse::{parse, UriParseState};
use crate::{Uri, UriAuthority};

macro_rules! uri_eq {
    ($sample:literal, $expected_uri:expr) => {
        uri_eq!($sample, UriParseState::Scheme, $expected_uri)
    };
    ($sample:literal, $starter_state:expr, $expected_uri:expr) => {
        {
            let sample: &[u8] = { $sample };
            let test_uri =
                parse(sample, { $starter_state })
                .unwrap()
                .as_uri(sample);

            let expected_uri: Uri = { $expected_uri };

            let mut eq = true;
            let mut output = String::new();

            if test_uri.scheme != expected_uri.scheme {
                eq = false;
                output.push_str("  scheme mismatch\n");
                output.push_str(&format!("       test_uri: {:?}\n",
                        test_uri.scheme.map(str::from_utf8)));
                output.push_str(&format!("   expected_uri: {:?}\n",
                        expected_uri.scheme.map(str::from_utf8)));
            } else {
                output.push_str(&format!("  schemes match: {:?}\n",
                        test_uri.scheme.map(str::from_utf8)));
            }

            if test_uri.authority.is_some() && expected_uri.authority.is_some() {
                let t = test_uri.authority.as_ref().unwrap();
                let x = expected_uri.authority.as_ref().unwrap();
                if t.host != x.host {
                    eq = false;
                    output.push_str("  host mismatch\n");
                    output.push_str(&format!("       test_uri: {:?}\n",
                            str::from_utf8(t.host)));
                    output.push_str(&format!("   expected_uri: {:?}\n",
                            str::from_utf8(x.host)));
                } else {
                    output.push_str(&format!("  hosts match: {:?}\n",
                            str::from_utf8(t.host)));
                }
                if t.port != x.port {
                    eq = false;
                    output.push_str("  port mismatch\n");
                    output.push_str(&format!("       test_uri: {:?}\n",
                            t.port));
                    output.push_str(&format!("   expected_uri: {:?}\n",
                            x.port));
                } else {
                    output.push_str(&format!("  ports match: {:?}\n",
                            t.port));
                }
            } else if test_uri.authority.is_none() && expected_uri.authority.is_some() {
                eq = false;
                output.push_str("  authority mismatch\n");
                output.push_str("       test_uri: None\n");
                output.push_str("   expected_uri: Some(...)\n");
            } else if test_uri.authority.is_some() && expected_uri.authority.is_none() {
                eq = false;
                output.push_str("  authority mismatch\n");
                output.push_str("       test_uri: Some(...)\n");
                output.push_str("   expected_uri: None\n");
            } else {
                output.push_str("  authorities match: (both None)\n");
            }

            if test_uri.path != expected_uri.path {
                eq = false;
                output.push_str("  paths mismatch\n");
                output.push_str(&format!("       test_uri: {:?}\n",
                        str::from_utf8(test_uri.path)));
                output.push_str(&format!("   expected_uri: {:?}\n",
                        str::from_utf8(expected_uri.path)));
            } else {
                output.push_str(&format!("  paths match: {:?}\n",
                        str::from_utf8(test_uri.path)));
            }

            if test_uri.query != expected_uri.query {
                eq = false;
                output.push_str("  query mismatch\n");
                output.push_str(&format!("       test_uri: {:?}\n",
                        test_uri.query.map(str::from_utf8)));
                output.push_str(&format!("   expected_uri: {:?}\n",
                        expected_uri.query.map(str::from_utf8)));
            } else {
                output.push_str(&format!("  querys match: {:?}\n",
                        test_uri.query.map(str::from_utf8)));
            }

            if test_uri.fragment != expected_uri.fragment {
                eq = false;
                output.push_str("  fragment mismatch\n");
                output.push_str(&format!("       test_uri: {:?}\n",
                        test_uri.fragment.map(str::from_utf8)));
                output.push_str(&format!("   expected_uri: {:?}\n",
                        expected_uri.fragment.map(str::from_utf8)));
            } else {
                output.push_str(&format!("  fragments match: {:?}\n",
                        test_uri.fragment.map(str::from_utf8)));
            }

            assert!(eq, "uris do not match\n{output}\n");
        }
    }
}

#[test]
fn basic_usage() {
    uri_eq!(
        b"http://example.com:80/some/where?query=string#fragment",
        Uri {
            scheme: Some(b"http"),
            authority: UriAuthority {
                host: Some(b"example.com"),
                port: Some(80),
            },
            path: b"/some/where",
            query: Some(b"query=string"),
            fragment: Some(b"fragment")
        });
}

#[test]
fn zero_len() {
    uri_eq!(
        b"",
        Uri::default()
    )
}

#[test]
fn invalid_scheme() {
    assert_eq!(
        parse(b"word:", UriParseState::Scheme),
        // tmp, until no proper errors
        Err("invalid uri: invalid byte after scheme")
    );
}

#[test]
fn no_scheme_one_word() {
    uri_eq!(
        b"word",
        Uri {
            path: b"word",
            ..Default::default()
        }
    );
}

#[test]
fn no_scheme_more() {
    uri_eq!(
        b"path/to/resource?query=string",
        Uri {
            path: b"path/to/resource",
            query: Some(b"query=string"),
            ..Default::default()
        }
    );
}

#[test]
fn has_scheme_and_authority() {
    uri_eq!(
        b"scheme://domain.com:190//",
        Uri {
            scheme: Some(b"scheme"),
            authority: UriAuthority {
                host: Some(b"domain.com"),
                port: Some(190),
            },
            path: b"//",
            ..Default::default()
        }
    );
}
#[test]
fn no_scheme_authority() {
    uri_eq!(
        b"//domain.tld:90",
        Uri {
            authority: UriAuthority {
                host: Some(b"domain.tld"),
                port: Some(90),
            },
            ..Default::default()
        }
    );
}
#[test]
fn no_scheme_authority_empty() {
    uri_eq!(
        b"/////empty_auth",
        Uri {
            authority: UriAuthority {
                host: Some(b""),
                path: b"///empty_auth",
            },
            ..Default::default()
        }
    );
}
