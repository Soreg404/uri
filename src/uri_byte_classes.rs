pub trait UriByte {
    fn is_uric(&self) -> bool;
    fn is_uri_reserved(&self) -> bool;
    fn is_uri_unreserved(&self) -> bool;
    fn is_uri_mark(&self) -> bool;
    fn is_uri_scheme_allowed(&self, is_first_byte: bool) -> bool;
}

impl UriByte for u8 {
    fn is_uric(&self) -> bool {
        self.is_uri_reserved() || self.is_uri_unreserved() || *self == b'%'
    }
    fn is_uri_reserved(&self) -> bool {
        match &self {
            b';' | b'/' | b'?' | b':' | b'@' | b'&' | b'=' | b'+' | b'$' | b',' => true,
            _ => false
        }
    }
    fn is_uri_unreserved(&self) -> bool {
        self.is_ascii_alphanumeric() || self.is_uri_mark()
    }
    fn is_uri_mark(&self) -> bool {
        match self {
            b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')' => true,
            _ => false
        }
    }
    fn is_uri_scheme_allowed(&self, is_first_byte: bool) -> bool {
        if is_first_byte {
            self.is_ascii_alphabetic()
        } else {
            self.is_ascii_alphanumeric() || match self {
                b'+' | b'-' | b'.' => true,
                _ => false
            }
        }
    }
}

