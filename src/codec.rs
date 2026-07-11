use crate::uri_byte_classes::UriByte;

#[derive(Debug, Eq, PartialEq)]
pub enum CodecError<'src, 'dest> {
    BufferTooSmall(usize),
    UnsafeByte {
        decoded: &'dest [u8],
        error_index: usize,
        rest: &'src [u8]
    },
    InvalidSequence {
        decoded: &'dest [u8],
        error_index: usize,
        error_length: usize,
        rest: &'src [u8]
    },
}

pub struct CodecOk<'arena> {
    pub decoded: &'arena [u8],
    pub rest: &'arena mut [u8]
}

/*
 * uri_unsafe_bytes are all besides is_uri_unreserved
 */
pub fn decode<'src, 'dest>(
    bytes: &'src [u8],
    dest_buffer: &'dest mut [u8],
    catch_invalid_sequences: bool,
    catch_uri_unsafe_bytes: bool
)
-> Result<CodecOk<'dest>, CodecError<'src, 'dest>> {
    let mut buffer_too_small = false;
    let mut dest_head = 0usize;
    let mut i = 0;
    while i < bytes.len() {
        let b = match decode_next_byte(&bytes[i..]) {
            DNBResult::Byte(b) => {
                i += 1;
                b
            }
            DNBResult::UnsafeByte(b) => {
                if !buffer_too_small && catch_uri_unsafe_bytes {
                    return Err(CodecError::UnsafeByte {
                        decoded: &dest_buffer[..dest_head],
                        error_index: i,
                        rest: &bytes[i + 1 ..]
                    });
                }
                i += 1;
                b
            }
            DNBResult::Sequence(b) => {
                i += 3;
                b
            }
            DNBResult::InvalidSequence(len) => {
                if !buffer_too_small && catch_invalid_sequences {
                    return Err(CodecError::InvalidSequence {
                        decoded: &dest_buffer[..dest_head],
                        error_index: i,
                        error_length: len,
                        rest: &bytes[i + len ..]
                    });
                }
                i += len;
                continue;
            }
        };

        if !buffer_too_small {
            if dest_head == dest_buffer.len() {
                buffer_too_small = true;
            } else {
                dest_buffer[dest_head] = b;
            }
        }
        dest_head += 1;
    }

    if buffer_too_small {
        Err(CodecError::BufferTooSmall(dest_head))
    } else {
        let (decoded, rest) = dest_buffer.split_at_mut(dest_head);
        Ok(CodecOk {
            decoded,
            rest
        })
    }
}

pub fn decode_to_vec<'src, 'dest>(
    bytes: &'src [u8],
    dest: &'dest mut Vec<u8>,
    catch_invalid_sequences: bool,
    catch_uri_unsafe_bytes: bool
) 
-> Result<&'dest [u8], CodecError<'src, 'dest>> {
    let dest_start = dest.len();
    let mut i = 0;
    while i < bytes.len() {
        let b = match decode_next_byte(&bytes[i..]) {
            DNBResult::Byte(b) => {
                i += 1;
                b
            }
            DNBResult::UnsafeByte(b) => {
                if catch_uri_unsafe_bytes {
                    return Err(CodecError::UnsafeByte {
                        decoded: &dest[dest_start..],
                        error_index: i,
                        rest: &bytes[i + 1 ..]
                    });
                }
                i += 1;
                b
            }
            DNBResult::Sequence(b) => {
                i += 3;
                b
            }
            DNBResult::InvalidSequence(len) => {
                if catch_invalid_sequences {
                    return Err(CodecError::InvalidSequence {
                        decoded: &dest[dest_start..],
                        error_index: i,
                        error_length: len,
                        rest: &bytes[i + len ..]
                    });
                }
                i += len;
                continue;
            }
        };

        dest.push(b);
    }
    Ok(&dest[dest_start..])
}

enum DNBResult {
    Byte(u8),
    UnsafeByte(u8),
    Sequence(u8),
    InvalidSequence(usize),
}
fn decode_next_byte(src: &[u8]) -> DNBResult {
    use DNBResult::*;
    match src.first() {
        None => panic!("tbd error message"),
        Some(b'+') => Byte(b' '),
        Some(b'%') => {
            if src.len() < 3 {
                InvalidSequence(src.len())
            } else {
                match hex(src[1], src[2]) {
                    None => InvalidSequence(3),
                    Some(v) => Sequence(v)
                }
            }
        },
        Some(b) => {
            if b.is_uri_unreserved() {
                Byte(*b)
            } else {
                UnsafeByte(*b)
            }
        }
    }
}

// todo: handle TooSmallBuffer error
pub fn encode<'src, 'dest>(
    bytes: &'src [u8],
    dest_buffer: &'dest mut [u8]
) -> Result<&'dest [u8], usize> {
    #[expect(unused)]
    let mut buffer_too_small = false;
    let mut dest_head = 0usize;
    let mut i = 0;
    while i < bytes.len() {
        let c = bytes[i];
        i += 1;
        if c.is_uri_unreserved() {
            // push c
            dest_buffer[dest_head] = c;
            dest_head += 1;
        } else {
            // push encode
            let lo = c & 0x0f;
            let hi = c >> 4;

            fn hexit(b: u8) -> u8 { 
                if b < 10 {
                    b'0' + b
                } else {
                    b'A' + (b - 10)
                }
            }
            let lo = hexit(lo);
            let hi = hexit(hi);

            dest_buffer[dest_head..dest_head + 3]
                .copy_from_slice(&[b'%', hi, lo]);
            dest_head += 3;
        }
    }

    Ok(&dest_buffer[..dest_head])
}

#[expect(unused)]
pub fn encode_to_vec(bytes: &[u8]) -> Vec<u8> {
    todo!()
}

fn hex(b1: u8, b2: u8) -> Option<u8> {
    let hi = match b1 {
        b'a'..=b'f' => b1 - b'a' + 10,
        b'A'..=b'F' => b1 - b'A' + 10,
        b'0'..=b'9' => b1 - b'0',
        _ => return None
    };
    let lo = match b2 {
        b'a'..=b'f' => b2 - b'a' + 10,
        b'A'..=b'F' => b2 - b'A' + 10,
        b'0'..=b'9' => b2 - b'0',
        _ => return None
    };
    Some(hi << 4 | lo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex() {
        assert_eq!(hex(b'2', b'0'), Some(0x20));
        assert_eq!(hex(b'a', b'a'), Some(0xaa));
        assert_eq!(hex(b'C', b'C'), Some(0xcc));
        assert_eq!(hex(b'z', b'1'), None);
    }

    mod decode {
        use crate::codec::*;

        fn simple_comp(from: &[u8], to: &[u8]) {
            let mut arr_buffer = [0u8; 20];
            assert!(from.len() <= 20);

            let r = decode(from, &mut arr_buffer, false, false);
            assert_eq!(r, Ok(to));
            assert!(arr_buffer.starts_with(to));

            let mut vec_buffer = Vec::<u8>::new();
            let r = decode_to_vec(from, &mut vec_buffer, false, false);
            assert_eq!(r, Ok(to));
            assert_eq!(vec_buffer.as_slice(), to);
        }

        #[test]
        fn basic_usage() {
            simple_comp(b"+", b" ");
            simple_comp(b"1+2+3", b"1 2 3");
            simple_comp(b"+1+", b" 1 ");
            simple_comp(b"+++1+++1+++", b"   1   1   ");

            simple_comp(b"%20", b" ");
            simple_comp(b"1%202%203", b"1 2 3");
            simple_comp(b"%e7%8c%ab", b"\xe7\x8c\xab");
        }

        #[test]
        fn catch_unsafe_byte() {
            let mut buf = [0u8; 20];
            let r = decode(b"123\r45", &mut buf, true, true).unwrap_err();
            assert_eq!(r, CodecError::UnsafeByte {
                decoded: b"123",
                error_index: 3,
                rest: b"45"
            });

            let r = decode(b"123 45", &mut buf, true, true).unwrap_err();
            assert_eq!(r, CodecError::UnsafeByte {
                decoded: b"123",
                error_index: 3,
                rest: b"45"
            });
        }

        #[test]
        fn catch_invalid_sequence() {
            let mut buf = [0u8; 20];
            let sample = b"123_%20_%xx_45_%x";
            let r = decode(sample, &mut buf, true, true).unwrap_err();
            assert_eq!(r, CodecError::InvalidSequence {
                decoded: b"123_ _",
                error_index: 8,
                error_length: 3,
                rest: &sample[11..]
            });

            let sample = &sample[11..];
            let r = decode(sample, &mut buf[6..], true, true).unwrap_err();
            assert_eq!(r, CodecError::InvalidSequence {
                decoded: b"_45_",
                error_index: 4,
                error_length: 2,
                rest: &sample[sample.len()..]
            });
        }

        #[test]
        fn edge_case_buffer_too_small() {
            assert_eq!(
                decode(b"1", &mut [], false, false),
                Err(CodecError::BufferTooSmall(1))
            );

            // simple cutoff
            let mut buf = [0u8; 2];
            assert_eq!(
                decode(b"12345", &mut buf, false, false),
                Err(CodecError::BufferTooSmall(5))
            );
            assert_eq!(buf.as_slice(), b"12");
        }

        #[test] 
        fn edge_case_buffer_too_small_combined_with_other_error() {
            let mut buf = [0u8; 2];
            assert_eq!(
                decode(b"123%xx45", &mut buf, true, true),
                Err(CodecError::BufferTooSmall(5))
            );
            assert_eq!(buf.as_slice(), b"12");


            let mut buf = [0u8; 2];
            assert_eq!(
                decode(b"123\r45", &mut buf, true, true),
                Err(CodecError::BufferTooSmall(6))
            );
            assert_eq!(buf.as_slice(), b"12");
        }
    }
}
