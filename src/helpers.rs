// todo: maybe change to $args:tt*
macro_rules! trace {
    ($ctx:expr) => {{
        if option_env!("URI_TRACE") == Some("1") {
            println!("\x1b[36mtrace!\x1b[0m ({}:{:04}) \
                [uri_parse] {}", file!(), line!(), $ctx);
        }
    }}
}
