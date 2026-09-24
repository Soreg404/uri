pub struct PathParts<'arena> {
    parts: &'arena [u8],
    lengths: &'arena [usize],
}

#[derive(Copy, Clone)]
pub struct PathPartsIter<'a> {
    target: &'a PathParts<'a>,
    offs: usize,
    n_part: usize
}
impl<'a> Iterator for PathPartsIter<'a> {
    type Item = &'a [u8];
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_part >= self.target.lengths.len() {
            None
        } else {
            let c_len = self.target.lengths[self.n_part];
            let part = &self.target.parts[self.offs..self.offs + c_len];
            self.n_part += 1;
            self.offs += c_len;
            Some(part)
        }
    }
}
impl PathParts<'_> {
    pub fn iter(&self) -> PathPartsIter<'_> {
        PathPartsIter {
            target: self,
            offs: 0,
            n_part: 0
        }
    }
}

impl PathParts<'_> {
    pub fn eq(&self, other: &[&[u8]]) -> bool {
        let mut other_iter = other.iter();
        for p in self.iter() {
            match other_iter.next() {
                None => return false,
                Some(v) => {
                    if p != *v {
                        return false
                    }
                }
            }
        }
        true
    }
}
#[test]
fn test_path_parts_eq() {
    let test = PathParts {
        parts: b"loremipsumdolorsitamet",
        lengths: &[5, 5, 5, 3, 4]
    };
    assert!(test.eq(&[b"lorem", b"ipsum", b"dolor", b"sit", b"amet"]));
    assert!(!test.eq(&[b"it", b"is", b"bananas"]))
}

pub struct PathPartsRet<'arena> {
    pub path_parts: PathParts<'arena>,
    pub decode_arena_rest: &'arena mut [u8],
    pub lengths_arena_rest: &'arena mut [usize]
}

impl PathParts<'_> {
    pub fn parse_decode<'a, 'tmp, 'persistent>(
        path_raw: &'a [u8],
        single_part_decode_scratch_buffer: &'tmp mut [u8],
        decoded_path_parts_arena: &'persistent mut [u8],
        parts_lengths_arena: &'persistent mut [usize],
        catch_invalid_sequences: bool,
        catch_uri_unsafe_bytes: bool
    ) -> Result<PathPartsRet<'persistent>, ()> {

        /*
         * idk about prerformance, maybe better not to loop buffers backwards?
         * cuz cache and that stuff
         *
         * and, todo, used part of the arena better to be at start
         */

        let src = path_raw;
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
                catch_invalid_sequences,
                catch_uri_unsafe_bytes
            ) {
                Err(_e) => {
                    // codec error
                    return Err(());
                },
                Ok(v) => v.decoded
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

        let (a_decode_rest, a_decode_used) = dest.split_at_mut(dest_head);
        let (a_lengths_rest, a_lengths_used) = lengths.split_at_mut(lengths.len() - part_counter);

        Ok(PathPartsRet {
            path_parts: PathParts {
                parts: a_decode_used,
                lengths: a_lengths_used,
            },
            decode_arena_rest: a_decode_rest,
            lengths_arena_rest: a_lengths_rest
        })
    }
}

#[cfg(test)]
mod tests {
    use super::PathParts;

    macro_rules! path_parts_eq {
        ($left:expr, $paths:expr, $lengths:expr) => {{
            let mut decode_scratch_buffer = { let mut v = Vec::new(); v.resize(100, 0u8); v };

            let mut decode_arena = { let mut v = Vec::new(); v.resize(1000, 0u8); v };
            let mut lengths_arena = { let mut v = Vec::new(); v.resize(100, 0usize); v };
            
            let r = PathParts::parse_decode(
                $left,
                &mut decode_scratch_buffer,
                &mut decode_arena,
                &mut lengths_arena
            )
                .unwrap();

            assert_eq!(r.path_parts.parts, $paths);
            assert_eq!(r.path_parts.lengths, $lengths);
            r.decode_arena_rest[0..5] = b"hello";
            r.lengths_arena_rest[0..1] = [0, 1];
        }}
    }

    #[test]
    fn simple_path() {
        path_parts_eq!(
            b"/lorem/ipsum/dolor/sit/amet",
            b"loremipsumdolorsitamet",
            &[5, 5, 5, 3, 4]
        );
    }

    #[test]
    fn excessive_slashes() {
        path_parts_eq!(
            b"//////hello/////world///////",
            b"helloworld",
            &[5, 5]
        );
    }

    #[test]
    fn normalize_path() {
        path_parts_eq!(
            b"/lorem/ipsum/../../jolly/cooperation",
            b"jollycooperation",
            &[5, 11]
        );
    }

    #[test]
    fn normalize_path_edge_case() {
        path_parts_eq!(
            b"////silly/../..////../../hi",
            b"hi",
            &[2]
        );
    }

    #[test]
    fn decode_and_normalize() {
        path_parts_eq!(
            b"ab+/ab%20/%25%61%62/%20/%E7%8C%AB",
            b"ab ab %ab \xe7\x8c\xab",
            &[3, 3, 3, 1, 3]
        );
    }

    #[test]
    fn handle_errors() {
        todo!()
    }

}
