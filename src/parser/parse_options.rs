use crate::{ Url, UrlCacheable };
use super::parse::UrlParseState;

#[derive(Clone)]
pub struct UrlParser {
    starting_state: UrlParseState
}
impl Default for UrlParser {
    fn default() -> Self {
        Self {
            starting_state: UrlParseState::Scheme
        }
    }
}
impl UrlParser {
    pub fn expect_absolute_uri() -> Self {
        Self::default()
    }
    pub fn expect_relative_uri() -> Self {
        Self {
            starting_state: UrlParseState::NoScheme
        }
    }
    pub fn starts_from_authority() -> Self {
        Self {
            starting_state: UrlParseState::StartsFromAuthority
        }
    }
    pub fn starts_from_path() -> Self {
        Self {
            starting_state: UrlParseState::StartsFromPath
        }
    }

    pub fn allow_bckwards_compatible_something_something(&mut self) { todo!() }

    pub fn parse<'a>(self, bytes: &'a [u8]) -> Result<Url<'a>, ()> {
        Ok(
            self.parse_cacheable(bytes)?
            .as_url(bytes)
        )
    }
    pub fn parse_cacheable(self, bytes: &[u8]) -> Result<UrlCacheable, ()> {
        super::parse::parse(
            bytes,
            self.starting_state
        )
    }
}
