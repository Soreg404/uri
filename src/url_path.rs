#[allow(dead_code)]
pub struct PathParts<'arena> {
    parts: &'arena [u8],
    lengths: &'arena [usize],
}
impl PathParts<'_> {
    pub fn tmp_parts(&self) -> &[u8] {
        self.parts
    }
    pub fn tmp_lengths(&self) -> &[usize] {
        self.lengths
    }
}

impl<'a> super::Url<'a> {
    pub fn get_decoded_path<'tmp, 'persistent>(
        &'a self,
        single_part_decode_scratch_buffer: &'tmp mut [u8],
        decoded_path_parts_arena: &'persistent mut [u8],
        parts_lengths_arena: &'persistent mut [usize]
    ) -> Result<PathParts<'persistent>, ()> {

        /*
         * idk about prerformance, maybe better not to loop buffers backwards?
         * cuz cache and that stuff
         */

        let src = self.path_raw;
        // starting from end to easier normalize the path
        let mut src_i = src.len();
        let mut part_counter = 0;

        #[allow(unused_assignments)]
        let mut part_end = 0;

        let dest = decoded_path_parts_arena;
        assert!(dest.len() > 0);
        let mut dest_head = dest.len();

        let lengths = parts_lengths_arena;
        assert!(lengths.len() > 0);

        let mut skip_parts_counter = 0;

        while src_i > 0 {

            while src_i > 0 && src[src_i - 1] == b'/' {
                src_i -= 1;
            }

            part_end = src_i;
            while src_i > 0 && src[src_i - 1] != b'/' {
                src_i -= 1;
            }
            if part_end == src_i {
                break;
            }
            let c_part = &src[src_i..part_end];

            if c_part == b"." {
                continue;
            }
            if c_part == b".." {
                skip_parts_counter += 1;
                continue;
            }

            if skip_parts_counter > 0 {
                skip_parts_counter -= 1;
                continue;
            }

            let c_part = match crate::codec::decode(
                c_part,
                single_part_decode_scratch_buffer,
                false, false
            ) {
                Err(crate::codec::CodecError::BufferTooSmall(_l)) => {
                    // buffer too small - single part too big ({l})
                    return Err(());
                },
                Err(_) => unreachable!(),
                Ok(v) => v
            };

            if dest_head < c_part.len() {
                // dest arena too small - path too long
                return Err(());
            }
            dest[dest_head - c_part.len()..dest_head].copy_from_slice(c_part);
            dest_head -= c_part.len();
            part_counter += 1;
            if lengths.len() < part_counter {
                // lengths arena too small - too many parts
                return Err(());
            }
            lengths[lengths.len() - part_counter] = c_part.len();
        }

        Ok(PathParts {
            parts: &dest[dest_head..],
            lengths: &lengths[lengths.len() - part_counter..],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PathParts;
    use crate::Url;

    fn prep_buffers() -> (Vec<u8>, Vec<usize>) {
        let a1 = { let mut v = Vec::new(); v.resize(1000, 0u8); v };
        let a2 = { let mut v = Vec::new(); v.resize(1000, 0usize); v };
        (a1, a2)
    }
    fn decode_helper<'a>(
        url: &'a Url<'a>,
        buffers: &'a mut (Vec<u8>, Vec<usize>)
    ) -> Result<PathParts<'a>, ()> {
        let mut sb = { let mut v = Vec::new(); v.resize(1000, 0u8); v };
        url.get_decoded_path(&mut sb, &mut buffers.0, &mut buffers.1)
    }

    #[test]
    fn simple_path() {
        let url = Url {
            path: b"/lorem/ipsum/dolor/sit/amet",
            ..Default::default()
        };
        let mut b = prep_buffers();
        let p = decode_helper(&url, &mut b).unwrap();
        assert_eq!(p.parts, b"loremipsumdolorsitamet");
        assert_eq!(p.lengths, &[5, 5, 5, 3, 4]);
    }

    #[test]
    fn excessive_slashes() {
        let url = Url {
            path: b"//////hello/////world///////",
            ..Default::default()
        };
        let mut b = prep_buffers();
        let p = decode_helper(&url, &mut b).unwrap();
        assert_eq!(p.parts, b"helloworld");
        assert_eq!(p.lengths, &[5, 5]);
    }

    #[test]
    fn normalize_path() {
        let url = Url {
            path: b"/lorem/ipsum/../../jolly/cooperation",
            ..Default::default()
        };
        let mut b = prep_buffers();
        let p = decode_helper(&url, &mut b).unwrap();
        assert_eq!(p.parts, b"jollycooperation");
        assert_eq!(p.lengths, &[5, 11]);
    }

    #[test]
    fn normalize_path_edge_case() {
        let url = Url {
            path: b"////silly/../..////../../hi",
            ..Default::default()
        };
        let mut b = prep_buffers();
        let p = decode_helper(&url, &mut b).unwrap();
        assert_eq!(p.parts, b"hi");
        assert_eq!(p.lengths, &[2]);
    }

    #[test]
    fn decode_and_normalize() {
        let url = Url {
            path: b"ab+/ab%20/%25%61%62/%20/%E7%8C%AB",
            ..Default::default()
        };
        let mut b = prep_buffers();
        let p = decode_helper(&url, &mut b).unwrap();
        assert_eq!(p.parts, b"ab ab %ab \xe7\x8c\xab");
        assert_eq!(p.lengths, &[3, 3, 3, 1, 3]);
    }

}
