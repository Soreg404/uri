use crate::{UriVariant, Uri, UriCacheable};
use super::parse::UriParseState;

#[derive(Clone)]
pub struct UriParser {
    starting_state: UriParseState
}
impl Default for UriParser {
    fn default() -> Self {
        Self {
            starting_state: UriParseState::Scheme
        }
    }
}
impl UriParser {
    pub fn expect_absolute_uri() -> Self {
        Self::default()
    }
    pub fn expect_relative_uri() -> Self {
        Self {
            starting_state: UriParseState::NoScheme
        }
    }
    pub fn starts_from_authority() -> Self {
        Self {
            starting_state: UriParseState::StartsFromAuthority
        }
    }
    pub fn starts_from_path() -> Self {
        Self {
            starting_state: UriParseState::StartsFromPath
        }
    }

    pub fn allow_bckwards_compatible_something_something(&mut self) { todo!() }

    pub fn parse<'a>(self, bytes: &'a [u8]) -> Result<UriVariant<'a>, &'static str> {
        Ok(
            self.parse_cacheable(bytes)?
            .as_uri(bytes)
        )
    }
    pub fn parse_cacheable(self, bytes: &[u8]) -> Result<UriCacheable, &'static str> {
        super::parse::parse(
            bytes,
            self.starting_state
        )
    }
}
