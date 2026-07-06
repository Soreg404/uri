
use super::parse::{ parse, UrlParseState };
use crate::Url;

macro_rules! url_eq {
    ($sample:literal, $expected_uri:expr) => {
        url_eq!($sample, UrlParseState::Scheme, $expected_uri)
    };
    ($sample:literal, $starter_state:expr, $expected_uri:expr) => {
        {
            let sample: &[u8] = { $sample };
            let test_uri =
                parse(sample, { $starter_state })
                .unwrap()
                .as_url(sample);

            let expected_uri: Url = { $expected_uri };

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

            if test_uri.host_raw != expected_uri.host_raw {
                eq = false;
                output.push_str("  host mismatch\n");
                output.push_str(&format!("       test_uri: {:?}\n",
                        test_uri.host_raw.map(str::from_utf8)));
                output.push_str(&format!("   expected_uri: {:?}\n",
                        expected_uri.host_raw.map(str::from_utf8)));
            } else {
                output.push_str(&format!("  hosts match: {:?}\n",
                        test_uri.host_raw.map(str::from_utf8)));
            }

            if test_uri.port != expected_uri.port {
                eq = false;
                output.push_str("  port mismatch\n");
                output.push_str(&format!("       test_uri: {:?}\n",
                        test_uri.port));
                output.push_str(&format!("   expected_uri: {:?}\n",
                        expected_uri.port));
            } else {
                output.push_str(&format!("  ports match: {:?}\n",
                        test_uri.port));
            }

            if test_uri.path_raw != expected_uri.path_raw {
                eq = false;
                output.push_str("  paths mismatch\n");
                output.push_str(&format!("       test_uri: {:?}\n",
                        str::from_utf8(test_uri.path_raw)));
                output.push_str(&format!("   expected_uri: {:?}\n",
                        str::from_utf8(expected_uri.path_raw)));
            } else {
                output.push_str(&format!("  paths match: {:?}\n",
                        str::from_utf8(test_uri.path_raw)));
            }

            if test_uri.query_raw != expected_uri.query_raw {
                eq = false;
                output.push_str("  query mismatch\n");
                output.push_str(&format!("       test_uri: {:?}\n",
                        test_uri.query_raw.map(str::from_utf8)));
                output.push_str(&format!("   expected_uri: {:?}\n",
                        expected_uri.query_raw.map(str::from_utf8)));
            } else {
                output.push_str(&format!("  querys match: {:?}\n",
                        test_uri.query_raw.map(str::from_utf8)));
            }
            if test_uri.fragment_raw != expected_uri.fragment_raw {
                eq = false;
                output.push_str("  fragment mismatch\n");
                output.push_str(&format!("       test_uri: {:?}\n",
                        test_uri.fragment_raw.map(str::from_utf8)));
                output.push_str(&format!("   expected_uri: {:?}\n",
                        expected_uri.fragment_raw.map(str::from_utf8)));
            } else {
                output.push_str(&format!("  fragments match: {:?}\n",
                        test_uri.fragment_raw.map(str::from_utf8)));
            }

            assert!(eq, "urls do not match\n{output}\n");
        }
    }
}

#[test]
fn basic_usage() {
    url_eq!(
        b"http://example.com:80/some/where?query=string#fragment",
        Url {
            scheme: Some(b"http"),
            host_raw: Some(b"example.com"),
            port: Some(80),
            path_raw: b"/some/where",
            query_raw: Some(b"query=string"),
            fragment_raw: Some(b"fragment")
        });
}


#[test]
fn zero_len() {
    url_eq!(
        b"",
        Url::default()
    )
}

#[test]
fn scheme() {
    url_eq!(
        b"word:",
        // expect error
        Url::default()
    );
    url_eq!(
        b"word",
        Url {
            path_raw: b"word",
            ..Default::default()
        }
    );
}

#[test]
fn authority() {
    url_eq!(
        b"s://a",
        Url {
            scheme: Some(b"s"),
            host_raw: Some(b"a"),
            ..Default::default()
        }
    )
}
