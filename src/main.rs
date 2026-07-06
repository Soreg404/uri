use std::io::{ Write, BufRead };

fn main() {

    loop {
        _ = std::io::stdout().write_all(b"\x1b[2J\x1b[H\x1bc");
        _ = std::io::stdout().flush();

        let mut lines = std::io::stdin().lock().lines();

        loop {
            println!("Input an URI:");
            let line = match lines.next() {
                None => break,
                Some(Err(e)) => {
                    eprintln!("err: {e:?}");
                    continue;
                }
                Some(Ok(v)) => v
            };

            let line_str = line.trim();

            let parsed = match url::UrlParser::default().parse(line_str.as_bytes()) {
                Err(e) => {
                    eprintln!("err: {e:?}");
                    println!();
                    continue;
                }
                Ok(v) => v
            };

            println!("{parsed:#?}");
            println!();
        }
    }
}
