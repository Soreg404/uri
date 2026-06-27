use crate::uri_byte_classes::UriByte;

#[derive(Debug)]
pub enum CodecError<'src, 'dest> {
    BufferTooSmall(usize),
    InvalidSequence {
        decoded: &'dest [u8],
        error_index: usize,
        error_length: usize,
        rest: &'src [u8]
    },
    UnsafeByte {
        decoded: &'dest [u8],
        error_index: usize,
        rest: &'src [u8]
    }
}

/*
 * uri_unsafe_bytes are all besides is_uri_unreserved
 */
pub fn decode<'src, 'dest>(
    bytes: &'src [u8],
    dest_buffer: &'dest mut [u8],
    _catch_invalid_sequences: bool,
    _catch_uri_unsafe_bytes: bool
)
-> Result<&'dest [u8], CodecError<'src, 'dest>> {
    #![deny(unused)]
    #![allow(unused_mut)]
    let mut buffer_too_small = false;
    let mut dest_head = 0usize;
    let mut i = 0;
    while i < bytes.len() {

        match decode_next_byte(
            bytes,
            &mut i,
            dest_buffer,
            dest_head,
            false, false
        ) {
            Err(_e) => {
                return Err(CodecError::UnsafeByte {
                    decoded: dest_buffer,
                    error_index: 0,
                    rest: bytes
                });
            },
            _ => {}
        };
        dest_buffer[0] = 0;

        /*if !buffer_too_small {
            if dest_head == dest_buffer.len() {
                buffer_too_small = true;
            } else {
                if let Some(b) = b {
                    dest_buffer[dest_head] = b;
                    dest_head += 1;
                }
            }
        }*/
    }

    if buffer_too_small {
        Err(CodecError::BufferTooSmall(dest_head))
    } else {
        Ok(&dest_buffer[..dest_head])
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
        // todo: lifetimes not happy
        //
        // let b = decode_next_byte(
        //     bytes,
        //     &mut i,
        //     &dest[dest_start..],
        //     dest.len() - dest_start,
        //     catch_invalid_sequences,
        //     catch_uri_unsafe_bytes
        // )?;
        let b = Some(b'a');
        if let Some(b) = b {
            dest.push(b);
        }
    }
    Ok(&dest[dest_start..])
}

fn decode_next_byte<'decode, 'src, 'dest>(
    src: &'src [u8],
    src_i: &'decode mut usize,
    dest: &'dest [u8],
    dest_head: usize,
    catch_invalid_sequences: bool,
    catch_uri_unsafe_bytes: bool
) -> Result<Option<u8>, CodecError<'src, 'dest>> {
    if src[*src_i] == b'+' {
        *src_i += 1;
        return Ok(Some(b' '));
    } else if src[*src_i] == b'%' {
        let mut invalid_sequence = false;
        if *src_i + 2 >= src.len() {
            invalid_sequence = true;
        } else {
            match hex(src[*src_i + 1], src[*src_i + 2]) {
                None => invalid_sequence = true,
                Some(v) => {
                    *src_i += 3;
                    return Ok(Some(v));
                }
            }
        }
        if invalid_sequence && catch_uri_unsafe_bytes {
            let error_length = std::cmp::min(src.len() - *src_i, 3);
            return Err(CodecError::InvalidSequence {
                decoded: dest,
                error_index: *src_i,
                error_length,
                rest: &src[*src_i + error_length..] 
            });
        }
        Ok(None)
    } else {
        if !src[*src_i].is_uri_unreserved() && catch_uri_unsafe_bytes {
            return Err(CodecError::UnsafeByte {
                decoded: dest,
                error_index: *src_i,
                rest: &src[*src_i + 1..] 
            });
        }
        let tmp_src_i = *src_i;
        *src_i += 1;
        Ok(Some(src[tmp_src_i]))
    }
}

pub fn encode(bytes: &[u8], dest_buffer: &mut [u8]) -> Result<usize, usize> {
    let mut buffer_too_small = false;
    let mut buf_head = 0usize;
    let mut count = 0usize;
    let mut i = 0;
    while i < bytes.len() {
        // match bytes[i] {
        //
        // }
    }
    todo!()
}

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

    #[test]
    fn decode() {
        let mut buf = [0u8; 20];
        assert_eq!(decode(b"hello%20world", &mut buf), Ok(b"hello world".as_slice()));
        assert_eq!(decode(b"123456789%20123456789%20XYZ", &mut buf), Err(23));
        assert_eq!(&buf[0..20], b"123456789 123456789 ");
        assert_eq!(decode(b"%20%20%20", &mut buf), Ok(b"   ".as_slice()));
        assert_eq!(decode(b"a+b+c", &mut buf), Ok(b"a b c".as_slice()));

        let mut v = Vec::new();
        assert_eq!(decode_to_vec(b"hello%20world", &mut v), b"hello world");
        assert_eq!(decode_to_vec(b"123456789%20123456789%20XYZ", &mut v),
        b"123456789 123456789 XYZ");
        assert_eq!(decode_to_vec(b"%20%20%20", &mut v), b"   ");
        assert_eq!(decode_to_vec(b"cze%C5%9B%C4%87", &mut v), "cześć".as_bytes());
        assert_eq!(decode_to_vec(b"%22", &mut v), b"\"");
    }
}
