use super::parser::UrlParseState;

#[derive(Copy, Clone)]
pub struct Parser {
    starting_state: UrlParseState
}
impl Default for Parser {
    fn default() -> Self {
        Self {
            start: ParseStart::Discover;
        }
    }
}
impl Parser {
    pub fn starts_with_authority() -> Self {
        Self {
            start: ParseStart::StartsWithAuthority;
        }
    }
    pub fn starts_with_path() -> Self {
        Self {
            start: ParseStart::StartsWithPath;
        }
    }
    pub fn expect_absolute_uri() -> Self {
        Self {
            start: ParseStart::ExpectAbsoluteUri;
        }
    }
    pub fn expect_relative_uri() -> Self {
        Self {
            start: ParseStart::ExpectRelativeUri;
        }
    }
    pub fn parse(self, bytes: &[u8]) -> Result<Url, ()> {
        self.parse_cacheable()?
            .as_url(bytes)
    }
    pub fn parse_cacheable(self, bytes: &[u8]) -> Result<UrlCached, ()> {
        let url_parts = super::parse::parse(

        )?
    }
}
