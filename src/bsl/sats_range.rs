use super::len::{parse_len, Len};

use crate::{Parse, ParseResult, SResult};
/// Contains SatsRanges in an output
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SatsRange<'a> {
    slice: &'a [u8],
    /// The start of the range in satoshis.
    pub start: u64,
    /// The size of the range in satoshis.
    pub size: u64,
}
impl<'a> Parse<'a> for SatsRange<'a> {
    fn parse(slice: &'a [u8]) -> SResult<Self> {
        let Len { consumed, n } = parse_len(slice)?;
        let start = n;
        let start_consumed: usize = consumed;
        let mut remaining = &slice[consumed..];

        let Len { consumed, n } = parse_len(remaining)?;
        let size = n;
        let size_consumed: usize = consumed;

        let consumed = start_consumed + size_consumed;
        remaining = &slice[consumed..];
        let sats_range = SatsRange {
            slice: &slice[..consumed],
            start: start,
            size: size,
        };
        Ok(ParseResult::new(remaining, sats_range))
    }
}
impl<'a> SatsRange<'a> {
    /// Return the amount of this output (satoshi)
    pub fn start(&self) -> u64 {
        self.start
    }
    /// Return the script pubkey of this output
    pub fn size(&self) -> u64 {
        self.size
    }
}

impl<'a> AsRef<[u8]> for SatsRange<'a> {
    fn as_ref(&self) -> &[u8] {
        self.slice
    }
}

impl<'a> Into<bitcoin::blockdata::transaction::SatsRange> for &SatsRange<'a> {
    fn into(self) -> bitcoin::blockdata::transaction::SatsRange {
        let start = self.start();
        let size = self.size();
        bitcoin::blockdata::transaction::SatsRange {
            start,
            size,
        }
    }
}
