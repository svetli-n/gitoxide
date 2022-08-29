#![deny(rust_2018_idioms)]
#![forbid(unsafe_code)]

use bstr::{BStr, BString};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The current platform has no implementation for prompting in the terminal")]
    UnsupportedPlatform,
    #[error("Failed to open terminal at {:?} for writing prompt, or to write it", unix::TTY_PATH)]
    TtyWrite { source: std::io::Error },
}

#[derive(Default, Copy, Clone)]
pub struct Options {
    /// If true, what's prompted is a secret and thus should be hidden.
    pub secret: bool,
}

///
pub mod unix;
#[cfg(unix)]
use unix::imp;

#[cfg(not(unix))]
mod imp {
    use crate::{Error, Options};
    use bstr::{BStr, BString};

    /// Not implemented on this platform
    pub fn ask(_prompt: &BStr, _opts: Options) -> Result<BString, Error> {
        Err(Error::UnsupportedPlatform)
    }
}
#[cfg(not(unix))]
pub use imp::ask;

/// Ask for information typed by the user into the terminal after showing the prompt`, like `"Username: `.
pub fn openly<'a>(prompt: impl Into<&'a BStr>) -> Result<BString, Error> {
    imp::ask(
        prompt.into(),
        Options {
            secret: false,
            ..Default::default()
        },
    )
}

/// Ask for information _securely_ after showing the `prompt` (like `"password: "`) by not showing what's typed.
pub fn securely<'a>(prompt: impl Into<&'a BStr>) -> Result<BString, Error> {
    imp::ask(
        prompt.into(),
        Options {
            secret: true,
            ..Default::default()
        },
    )
}
