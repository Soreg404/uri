#[non_exhaustive]
#[derive(Debug, Eq, PartialEq)]
pub enum PathPartsError {
    BufferTooSmall,
}

pub fn path_parts<'src, 'arena>(
    path_bytes: &'src [u8],
    parts_arena: &'arena mut [&'src [u8]],
) -> Result<&'arena [&'src [u8]], PathPartsError> {
    // starting from end to easier normalize the path
    let mut src_i = path_bytes.len();
    let mut part_counter = 0;

    let mut part_end;

    let mut skip_parts_counter = 0;

    while src_i > 0 {

        while src_i > 0 && path_bytes[src_i - 1] == b'/' {
            src_i -= 1;
        }

        part_end = src_i;
        while src_i > 0 && path_bytes[src_i - 1] != b'/' {
            src_i -= 1;
        }
        if part_end == src_i {
            break;
        }
        let c_part = &path_bytes[src_i..part_end];

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

        if part_counter == parts_arena.len() {
            return Err(PathPartsError::BufferTooSmall)
        }
        parts_arena[part_counter] = c_part;
        part_counter += 1;
    }

    let parts = &mut parts_arena[..part_counter];
    parts.reverse();
    Ok(parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let b = b"/sample/path/with/many/parts";
        let mut arena: [&[u8]; 20] = [&[]; 20];
        assert_eq!(
            path_parts(b, arena.as_mut_slice()),
            Ok([b"sample".as_slice(), b"path", b"with", b"many", b"parts"].as_slice())
        );
    }

    #[test]
    fn simple_relative_path() {
        let b = b"sample/path/with/many/parts";
        let mut arena: [&[u8]; 20] = [&[]; 20];
        assert_eq!(
            path_parts(b, arena.as_mut_slice()),
            Ok([b"sample".as_slice(), b"path", b"with", b"many", b"parts"].as_slice())
        );
    }

    #[test]
    fn edge_case_excessive_slashes() {
        let b = b"/////sample/////path/with/////many/parts//////";
        let mut arena: [&[u8]; 20] = [&[]; 20];
        assert_eq!(
            path_parts(b, arena.as_mut_slice()),
            Ok([b"sample".as_slice(), b"path", b"with", b"many", b"parts"].as_slice())
        );
    }

    #[test]
    fn skips() {
        let b = b"/sample/path/with/many/parts/../../../../text";
        let mut arena: [&[u8]; 20] = [&[]; 20];
        assert_eq!(
            path_parts(b, arena.as_mut_slice()),
            Ok([b"sample".as_slice(), b"text"].as_slice())
        );
    }

    #[test]
    fn edge_case_skips_past_root() {
        let b = b"/one/two/three/four/../../../../../";
        let mut arena: [&[u8]; 1] = [&[]; 1];
        assert_eq!(
            path_parts(b, arena.as_mut_slice()),
            Ok([].as_slice())
        );
    }

    #[test]
    fn edge_case_skips_over_too_small_buffer() {
        let b = b"one/two/three/../../../four/five/six/seven/eight/../../../../";
        let mut arena: [&[u8]; 1] = [&[]; 1];
        assert_eq!(
            path_parts(b, arena.as_mut_slice()),
            Ok([b"four".as_slice()].as_slice())
        );
    }

    #[test]
    fn error_too_small_buffer() {
        let b = b"one/two/three";
        let mut arena: [&[u8]; 2] = [&[]; 2];
        assert_eq!(
            path_parts(b, arena.as_mut_slice()),
            Err(PathPartsError::BufferTooSmall)
        );
    }
}
