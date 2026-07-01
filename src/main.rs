fn main() {
    let sample = b"https://example.com:443/path/to/resource?query=string#frag";
/*
    let parsed = url::parse(sample).unwrap();

    println!(
        "scheme: {:?}\nhost: {:?}\nport: {:?}\npath: {:?}\n\
        query: {:?}\nfrag: {:?}",
        parsed.scheme.map(|s| str::from_utf8(s)),
        parsed.host.map(|s| str::from_utf8(s)),
        parsed.port,
        str::from_utf8(parsed.path),
        parsed.query.map(|s| str::from_utf8(s)),
        parsed.fragment.map(|s| str::from_utf8(s)),
    );
*/
}
