use std::{
    convert::Infallible,
    fmt,
    ops::{Add, AddAssign},
    str::FromStr,
};

/// Null terminated UTF-16 string.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Utf16String {
    bytes: Vec<u16>,
}

impl Utf16String {
    pub fn new() -> Self {
        Self { bytes: vec![0] }
    }

    pub fn len(&self) -> usize {
        self.bytes.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn as_ptr(&self) -> *const u16 {
        self.bytes.as_ptr()
    }

    pub fn as_mut_ptr(&mut self) -> *mut u16 {
        self.bytes.as_mut_ptr()
    }
}

impl From<&str> for Utf16String {
    fn from(string: &str) -> Self {
        let mut bytes = Vec::from_iter(string.encode_utf16());
        bytes.push(0);
        Self { bytes }
    }
}

impl From<&String> for Utf16String {
    fn from(string: &String) -> Self {
        Self::from(&string[..])
    }
}

impl fmt::Display for Utf16String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match String::from_utf16(&self.bytes[..self.len()]) {
            Ok(utf8_str) => {
                write!(f, "{}", utf8_str)?;
                Ok(())
            }
            Err(_) => Err(fmt::Error),
        }
    }
}

impl Add<&Utf16String> for Utf16String {
    type Output = Self;

    fn add(mut self, rhs: &Utf16String) -> Self::Output {
        self.bytes.pop();
        self.bytes.extend(rhs.bytes.iter());
        self
    }
}

impl AddAssign<&Utf16String> for Utf16String {
    fn add_assign(&mut self, rhs: &Utf16String) {
        self.bytes.pop();
        self.bytes.extend(rhs.bytes.iter());
    }
}

impl FromIterator<char> for Utf16String {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut bytes = vec![];
        let mut buffer = [0; 2];
        for ch in iter {
            ch.encode_utf16(&mut buffer);
            bytes.extend(buffer);
        }
        bytes.push(0);
        Self { bytes }
    }
}

impl FromStr for Utf16String {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(s))
    }
}

pub type WideString = Utf16String;
