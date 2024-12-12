use crate::{slice::read_slice, Parse, ParseResult, SResult};
use super::len::{parse_len, Len};
use alloc::string::String;

/// Contains a parsed string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarString<'a> {
    slice: &'a [u8],
}

impl<'a> AsRef<[u8]> for VarString<'a> {
    fn as_ref(&self) -> &[u8] {
        self.slice
    }
}

impl<'a> Parse<'a> for VarString<'a> {
    /// Parse string
    fn parse(slice: &'a [u8]) -> SResult<Self> {
        let Len { consumed, n } = parse_len(slice)?;
        let str_slice = read_slice(&slice[consumed..], n as usize)?;
        Ok(ParseResult::new(
            str_slice.remaining(),
            VarString {
                slice: str_slice.parsed_owned(),
            },
        ))
    }
}
impl<'a> VarString<'a> {
    /// Returns the string parsed
    pub fn string(&self) -> String {
        String::from_utf8(self.slice.to_vec()).unwrap()
    }
}



