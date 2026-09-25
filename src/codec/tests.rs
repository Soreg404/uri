mod decode {
    use crate::codec::*;

    #[test]
    fn basic_usage() {
        let mut buf = [0u8; 100];
        let r = decode(b"%20%20hello+world+%23%24%25", &mut buf, false, false)
            .unwrap();
        assert_eq!(r.decoded, b"  hello world #$%");
        assert_eq!(r.decode_arena_remainder.len(), 100 - r.decoded.len());
    }

    #[test]
    fn catch_unsafe_byte() {
        todo!()
    }

    #[test]
    fn catch_invalid_sequence() {
        todo!()
    }

    #[test]
    fn edge_case_buffer_too_small() {
        assert!(matches!(
                decode(b"1", &mut [], false, false),
                Err(CodecError::BufferTooSmall(1))
        ));

        // simple cutoff
        let mut buf = [0u8; 2];
        assert!(matches!(
                decode(b"12345", &mut buf, false, false),
                Err(CodecError::BufferTooSmall(5))
        ));
        assert_eq!(buf.as_slice(), b"12");
    }

    #[test] 
    fn edge_case_buffer_too_small_combined_with_other_error() {
        let mut buf = [0u8; 2];
        assert!(matches!(
                decode(b"123%xx45", &mut buf, true, true),
                Err(CodecError::BufferTooSmall(5))
        ));
        assert_eq!(buf.as_slice(), b"12");


        let mut buf = [0u8; 2];
        assert!(matches!(
                decode(b"123\r45", &mut buf, true, true),
                Err(CodecError::BufferTooSmall(6))
        ));
        assert_eq!(buf.as_slice(), b"12");
    }
}
