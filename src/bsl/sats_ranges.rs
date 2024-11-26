use super::len::{parse_len, Len};
use crate::bsl::SatsRange;
use crate::{Parse, ParseResult, SResult};

/// The transaction outputs of a transaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SatsRanges<'a> {
    slice: &'a [u8],
    n: usize,
}

impl<'a> Parse<'a> for SatsRanges<'a> {
    fn parse(slice: &'a [u8]) -> SResult<Self> {
        let Len { mut consumed, n } = parse_len(slice)?;
        let mut remaining = &slice[consumed..];
        let total_sats_ranges = n as usize;
        for _ in 0..total_sats_ranges {
            let sats_range = SatsRange::parse(remaining)?;
            remaining = sats_range.remaining();
            consumed += sats_range.consumed();
            
        }
        Ok(ParseResult::new(
            &slice[consumed..],
            SatsRanges {
                slice: &slice[..consumed],
                n: total_sats_ranges,
            },
        ))
    }
}
impl<'a> SatsRanges<'a> {
    /// Creates an empty [`SatsRanges`]
    pub fn empty() -> Self {
        SatsRanges {
            slice: &[],
            n: 0,
        }
    }
    
    /// If there are no outputs.
    pub fn is_empty(&self) -> bool {
        self.slice[0] == 0
    }
    /// The number of outputs.
    pub fn n(&self) -> usize {
        self.n
    }
    /// Returns an iterator over [`SatsRange`]
    ///
    /// If possible is better to use [`Visitor::visit_tx_out`] to avoid double pass, however, it may
    /// be conveniet to iterate in case you already have validated the slice, for example some data
    /// in a db.
    pub fn iter(&self) -> TxOutIterator<'_> {
        let len = parse_len(self.slice).expect("len granted by parsing");
        TxOutIterator {
            elements: len.n() as usize,
            offset: len.consumed(),
            tx_outs: self,
        }
    }
}

impl<'a> IntoIterator for &'a SatsRanges<'a> {
    type Item = SatsRange<'a>;
    type IntoIter = TxOutIterator<'a>;

    /// Returns an iterator over [`SatsRange`]
    ///
    /// If possible is better to use [`Visitor::visit_tx_out`] to avoid double pass, however, it may
    /// be conveniet to iterate in case you already have validated the slice, for example some data
    /// in a db.
    fn into_iter(self) -> TxOutIterator<'a> {
        self.iter()
    }
}

pub struct TxOutIterator<'a> {
    elements: usize,
    offset: usize,
    tx_outs: &'a SatsRanges<'a>,
}

impl<'a> Iterator for TxOutIterator<'a> {
    type Item = SatsRange<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.tx_outs.as_ref().len() {
            None
        } else {
            let tx_out =
            SatsRange::parse(&self.tx_outs.slice[self.offset..]).expect("granted from parsing");
            self.offset += tx_out.consumed();
            Some(tx_out.parsed_owned())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.elements, Some(self.elements))
    }
}

impl<'a> ExactSizeIterator for TxOutIterator<'a> {}

impl<'a> AsRef<[u8]> for SatsRanges<'a> {
    fn as_ref(&self) -> &[u8] {
        self.slice
    }
}


