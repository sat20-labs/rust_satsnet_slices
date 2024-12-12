use crate::{slice::read_slice, Parse, ParseResult, SResult};
use super::len::{parse_len, Len};

#[cfg(feature = "alloc")]
use alloc::string::String;

/// Contains a parsed string
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VarString<'a> {
    str_index: usize,
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
        let Len { consumed , n } = parse_len(slice)?;
        let var_str_slice = read_slice(&slice[0..], n as usize + consumed)?;
        Ok(ParseResult::new(
            var_str_slice.remaining(),
            VarString {
                str_index: consumed,
                slice: var_str_slice.parsed_owned(),
            },
        ))
    }
}

#[cfg(feature = "alloc")]
impl<'a> VarString<'a> {
    /// Returns the string parsed
    pub fn string(&self) -> String {
        let str_slice = &self.slice[self.str_index..];
        String::from_utf8(str_slice.to_vec()).unwrap()
    }
}



