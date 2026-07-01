#[derive(Debug)]
pub enum UrlPathError {
    ConstraintViolation(UrlPathErrorConstraint),

}
#[derive(Debug)]
pub enum UrlPathErrorConstraint {
    SinglePartSizeExceeded {
        part_index: usize,
        required_size: usize
    },
    PathTooLong {
        required_size: usize
    },
    TooManyParts {
        num_parts: usize
    }
}
pub struct PathParts<'a> {
    parts_arena: &'a mut [u8],
    lengths_arena: &'a mut [u8],
    num_parts: usize,
}
impl PathParts<'_> {
    pub fn new<'a, 'b>(
        bytes: &'a [u8],
        single_part_decode_scratch_buffer: &'a mut [u8],
        all_parts_arena: &'b mut [u8],
        parts_lengths_arena: &'b mut [usize]
    ) -> Result<UrlPath<'b>, UrlPathError>  {

    }
}
