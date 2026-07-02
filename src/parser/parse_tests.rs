
use super::parse::{ parse, UrlParseState };
use crate::Url;

macro_rules! url_eq {
    ($left:expr, $right:expr) => {
        let left: Url = $left;
        let right: Url = $right;

        let mut eq = true;
        let mut output = String::new();

        if left.scheme != right.scheme {
            eq = false;
            output.push_str("  scheme mismatch\n");
            output.push_str(&format!("    left: {:?}\n", left.scheme.map(str::from_utf8)));
            output.push_str(&format!("   right: {:?}\n", right.scheme.map(str::from_utf8)));
        } else {
            output.push_str(&format!("  schemes match: {:?}\n", left.scheme.map(str::from_utf8)));
        }

        if left.host != right.host {
            eq = false;
            output.push_str("  host mismatch\n");
            output.push_str(&format!("    left: {:?}\n", left.host.map(str::from_utf8)));
            output.push_str(&format!("   right: {:?}\n", right.host.map(str::from_utf8)));
        } else {
            output.push_str(&format!("  hosts match: {:?}\n", left.host.map(str::from_utf8)));
        }

        if left.port != right.port {
            eq = false;
            output.push_str("  port mismatch\n");
            output.push_str(&format!("    left: {:?}\n", left.port));
            output.push_str(&format!("   right: {:?}\n", right.port));
        } else {
            output.push_str(&format!("  ports match: {:?}\n", left.port));
        }

        if left.path != right.path {
            eq = false;
            output.push_str("  paths mismatch\n");
            output.push_str(&format!("    left: {:?}\n", str::from_utf8(left.path)));
            output.push_str(&format!("   right: {:?}\n", str::from_utf8(right.path)));
        } else {
            output.push_str(&format!("  paths match: {:?}\n", str::from_utf8(left.path)));
        }

        if left.query != right.query {
            eq = false;
            output.push_str("  query mismatch\n");
            output.push_str(&format!("    left: {:?}\n", left.query.map(str::from_utf8)));
            output.push_str(&format!("   right: {:?}\n", right.query.map(str::from_utf8)));
        } else {
            output.push_str(&format!("  querys match: {:?}\n", left.query.map(str::from_utf8)));
        }
        if left.fragment != right.fragment {
            eq = false;
            output.push_str("  fragment mismatch\n");
            output.push_str(&format!("    left: {:?}\n", left.fragment.map(str::from_utf8)));
            output.push_str(&format!("   right: {:?}\n", right.fragment.map(str::from_utf8)));
        } else {
            output.push_str(&format!("  fragments match: {:?}\n", left.fragment.map(str::from_utf8)));
        }

        assert!(eq, "urls do not match\n{output}\n");
    }
}

#[test]
fn basic_usage() {
    let sample_abs = b"http://example.com:80/some/where?query=string#fragment";
    let r = parse(sample_abs, UrlParseState::Scheme).unwrap();
    let r = r.as_url(sample_abs);
    url_eq!(r, Url {
        scheme: Some(b"http"),
        host: Some(b"example.com"),
        port: Some(80),
        path: b"/some/where",
        query: Some(b"query=string"),
        fragment: Some(b"fragment")
    });
}

mod edge_cases {

    #[test]
    fn zero_len() {
        todo!()
    }

    #[test]
    fn eof_before_scheme_parsed() {
        let _sample = b"valid-scheme-without-colon";
        // expect ref_path
        todo!()
    }

    mod authority {

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
