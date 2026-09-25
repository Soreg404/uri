// todo: maybe change to $args:tt*
macro_rules! trace {
    ($ctx:expr) => {{
        #[cfg(feature = "debug-trace")]
        {
        println!("\x1b[36mtrace!\x1b[0m ({}:{:04}) [uri_parse] {}", file!(), line!(), $ctx);
        }
    }}
}
