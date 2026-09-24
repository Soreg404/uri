use std::io::{ stdout, stdin, Write, BufRead };

const ARENA_SIZE: usize = 0x100;

fn main() {

    enable_ansi::enable_ansi();

    println!("Playground");
    println!("1) parser");
    println!("2) decode");
    println!("3) encode");

    let select;
    loop {
        println!("Type [123]<Enter>:");
        let mut s = String::new();
        _ = stdin().read_line(&mut s);
        match s.trim().parse::<u8>() {
            Err(_) => continue,
            Ok(v) => {
                select = v;
                break;
            }
        }
    }

    match select {
        1 => pg_parser(),
        2 => pg_decode(),
        3 => pg_encode(),
        _ => unreachable!()
    }

}

fn pg_parser() {
    loop {
        _ = stdout().write_all(b"\x1b[2J\x1b[H\x1bc");
        _ = stdout().flush();

        let mut lines = stdin().lock().lines();

        loop {
            println!("Enter an URI:");
            let line = match lines.next() {
                None => break,
                Some(Err(e)) => {
                    eprintln!("err: {e:?}");
                    continue;
                }
                Some(Ok(v)) => v
            };

            let line_str = line.trim();

            let parsed = match uri::UrlParser::default().parse(line_str.as_bytes()) {
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

fn pg_decode() {
    let mut decode_arena = unsafe { 
        Box::<[u8]>::new_uninit_slice(ARENA_SIZE).assume_init()
    };
    loop {
        _ = stdout().write_all(b"\x1b[2J\x1b[H\x1bc");
        _ = stdout().flush();

        let mut lines = stdin().lock().lines();

        println!("DECODE arena size is {ARENA_SIZE} bytes");
        loop {
            println!("Enter a line to decode:");
            let line = match lines.next() {
                None => break,
                Some(Err(e)) => {
                    eprintln!("err: {e:?}");
                    continue;
                }
                Some(Ok(v)) => v
            };

            let line_str = line.trim();

            match uri::codec::decode(
                line_str.as_bytes(),
                &mut decode_arena,
                true,
                true
            ) {
                Err(e) => {
                    eprintln!("\x1b[91mDecode error!\x1b[0m");
                    eprintln!("{e:?}");
                }
                Ok(v) => {
                    v.rest[0] = b'E';
                    print!("Decoded text, from utf8 lossy ");
                    print_text(v.decoded);
                    print_text(v.rest);
                }
            }
            println!();
        }
    }
}

fn pg_encode() {
    let mut encode_arena = unsafe { 
        Box::<[u8]>::new_uninit_slice(ARENA_SIZE).assume_init()
    };
    loop {
        _ = stdout().write_all(b"\x1b[2J\x1b[H\x1bc");
        _ = stdout().flush();

        let mut lines = stdin().lock().lines();

        println!("ENCODE arena size is {ARENA_SIZE} bytes");
        loop {
            println!("Enter a line to encode:");
            let line = match lines.next() {
                None => break,
                Some(Err(e)) => {
                    eprintln!("err: {e:?}");
                    continue;
                }
                Some(Ok(v)) => v
            };

            let line_str = line.trim();

            match uri::codec::encode(
                line_str.as_bytes(),
                &mut encode_arena
            ) {
                Err(e) => {
                    eprintln!("Encode error: {e:?}");
                }
                Ok(v) => {
                    print!("Encoded text ");
                    print_text(v);
                }
            }
            println!();
        }
    }
}

fn print_text(v: &[u8]) {
    print!("\x1b[94m<<<\x1b[0m");
    print!("{}", String::from_utf8_lossy(v));
    println!("\x1b[94m>>>\x1b[0m");
}
