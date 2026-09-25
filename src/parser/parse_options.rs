use crate::{ UriVariant, Uri, UriOpaque };
use super::parse::ParseState;

pub struct ParseOptions {
    starting_state: UriParseState
}
impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            starting_state: UriParseState::Scheme
        }
    }
}
impl ParseOptions {
    pub fn absolute_uri() -> Self {
        Self::default()
    }
    pub fn relative_uri() -> Self {
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
        todo!()
    }
}
