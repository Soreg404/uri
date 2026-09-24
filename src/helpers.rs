macro_rules! trace {
    ($ctx:expr) => {
        {
            if matches!(option_env!("URI_TRACE"), Some(s) if s != "0") {
                println!("\x1b[36mtrace!\x1b[0m ({}:{:04}) [uri_parse] {}", file!(), line!(), $ctx);
            }
        }
    }
}
