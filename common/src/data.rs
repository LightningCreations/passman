use std::{
    borrow::{Borrow, BorrowMut},
    num::ParseIntError,
    ops::{Deref, DerefMut, Index, IndexMut},
    slice::SliceIndex,
    str::FromStr,
};

mod serde;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Bytes(Vec<u8>);

impl From<Vec<u8>> for Bytes {
    fn from(value: Vec<u8>) -> Self {
        Bytes(value)
    }
}

impl From<&[u8]> for Bytes {
    fn from(value: &[u8]) -> Self {
        Bytes(value.to_owned())
    }
}

impl Bytes {
    pub const fn new(underlying: Vec<u8>) -> Bytes {
        Self(underlying)
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.0
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for Bytes {
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.0[index]
    }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for Bytes {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Deref for Bytes {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Bytes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl AsRef<[u8]> for Bytes {
    fn as_ref(&self) -> &[u8] {
        self
    }
}

impl AsMut<[u8]> for Bytes {
    fn as_mut(&mut self) -> &mut [u8] {
        self
    }
}

impl Borrow<[u8]> for Bytes {
    fn borrow(&self) -> &[u8] {
        self
    }
}

impl BorrowMut<[u8]> for Bytes {
    fn borrow_mut(&mut self) -> &mut [u8] {
        self
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Version(u32);

impl Version {
    pub const fn from_encoded(val: u32) -> Self {
        Self(val)
    }

    pub const fn encoded(self) -> u32 {
        self.0
    }

    pub const fn from_parts(major: u8, minor: u8, revision: u16) -> Self {
        Self((major as u32) << 24 | (minor as u32) << 16 | (revision as u32))
    }

    pub const fn major(self) -> u8 {
        (self.0 >> 24) as u8
    }

    pub const fn minor(self) -> u8 {
        (self.0 >> 16) as u8
    }

    pub const fn revision(self) -> u16 {
        self.0 as u16
    }
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{}.{}.{}",
            self.major(),
            self.minor(),
            self.revision()
        ))
    }
}

pub enum VersionFromStringError {
    MissingComponents,
    TooManyComponents,
    InvalidComponent(ParseIntError),
}

impl FromStr for Version {
    type Err = VersionFromStringError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut components = s.split(".");
        let major = components
            .next()
            .ok_or(VersionFromStringError::MissingComponents)
            .and_then(|v| v.parse().map_err(VersionFromStringError::InvalidComponent))?;
        let minor = components
            .next()
            .ok_or(VersionFromStringError::TooManyComponents)
            .and_then(|v| v.parse().map_err(VersionFromStringError::InvalidComponent))?;
        let revision = components
            .next()
            .ok_or(VersionFromStringError::TooManyComponents)
            .and_then(|v| v.parse().map_err(VersionFromStringError::InvalidComponent))?;

        if components.next().is_some() {
            Err(VersionFromStringError::TooManyComponents)
        } else {
            Ok(Self::from_parts(major, minor, revision))
        }
    }
}
