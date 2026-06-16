fn main() {
    let sample = b"https://example.com:443/path/to/resource?qery=string#frag";

    let parsed = url::parse(sample);

    println!(
        "scheme: {:?}\nhost: {:?}\nport: {:?}\npath: {:?}\n\
        query: {:?}\nfrag: {:?}",
        parsed.scheme.map(|s| String::from_utf8_lossy(s)),
        parsed.host.map(|s| String::from_utf8_lossy(s)),
        parsed.port,
        String::from_utf8_lossy(parsed.path),
        parsed.query.map(|s| String::from_utf8_lossy(s)),
        parsed.fragment.map(|s| String::from_utf8_lossy(s)),
    );
}
