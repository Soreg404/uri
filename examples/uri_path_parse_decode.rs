fn main() {
    let mut decode_arena = {
        let mut v = Vec::<u8>::new();
        v.resize(1000, 0);
        v
    };
    let mut lengths_arena = {
        let mut v = Vec::<usize>::new();
        v.resize(100, 0);
        v
    };

    let result = uri::PathParts::parse_decode(
        b"/////part+1///skip///also+skip//..///..//part+2//parcent-%25////last%20part///",
        &mut decode_arena,
        &mut lengths_arena
    );

    let (path_parts, decode_arena, _) = match result {
        Err(()) => {
            panic!("Not enough space for processing path.");
        }
        Ok(ret) => (
            ret.path_parts,
            ret.decode_arena_rest,
            ret.lengths_arena_rest
        )
    };

    // perfectly usable rest of decode arena
    match uri::codec::decode(b"test%20uri%3a%3adecode%28%29", decode_arena) {
        Err(uri::codec::CodecError::BufferTooSmall(l)) => {
            panic!("Not enough space to decode. \
                required size: {l}, given: {} bytes.",
                decode_arena.len());
        }
        Err(uri::codec::CodecError(e)) => {
            eprintln!("Codec error: {e:?}");
        }
        Ok(codec::CodecOk { decoded, ... }) => {
            println!("Decoded: <<<{}>>>", String::from_utf8_lossy(decoded));
        }
    };

    println!("Processed path parts:");
    for part in path_parts {
        println!("{:?}", String::from_utf8_lossy(part));
    }
}
