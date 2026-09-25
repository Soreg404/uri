use crate::UriVariant;

pub fn parse<'src>(bytes: &'src [u8]) -> Result<UriVariant<'src>, &'static str> {
    self::ParseOptions::auto().parse(bytes)
}

pub struct ParseOptions {
    starting_state: super::parse::ParseState
}

impl ParseOptions {
    pub fn parse<'src>(self, bytes: &'src [u8]) -> Result<UriVariant<'src>, &'static str> {
        super::parse::parse(bytes, self.starting_state)
    }
}

impl ParseOptions {
    pub fn auto() -> Self {
        // temp
        // todo: detect stage -- Scheme / OptionScheme / RequireScheme or something
        //       or ExpectAbsolute / ExpectRelative
        Self {
            starting_state: super::parse::ParseState::Scheme
        }
    }
    pub fn absolute_uri() -> Self {
        Self {
            starting_state: super::parse::ParseState::Scheme
        }
    }
    pub fn relative_uri() -> Self {
        Self {
            starting_state: super::parse::ParseState::NoScheme
        }
    }
    pub fn starts_from_authority() -> Self {
        Self {
            starting_state: super::parse::ParseState::StartsFromAuthority
        }
    }
    pub fn starts_from_path() -> Self {
        Self {
            starting_state: super::parse::ParseState::StartsFromPath
        }
    }

    /// RFC mentioned some backwards compatible `scheme:path` configuration
    /// maybe look into it someday
    pub fn allow_bckwards_compatible_something_something(&mut self) { todo!() }
}
