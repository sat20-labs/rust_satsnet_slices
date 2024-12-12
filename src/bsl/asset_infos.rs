use super::len::{parse_len, Len};
use crate::bsl::AssetInfo;
use crate::{Parse, ParseResult, SResult};

/// The transaction outputs of a transaction
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetInfos<'a> {
    slice: &'a [u8],
    n: usize,
}

impl<'a> Parse<'a> for AssetInfos<'a> {
    fn parse(slice: &'a [u8]) -> SResult<Self> {
        let Len { mut consumed, n } = parse_len(slice)?;
        let mut remaining = &slice[consumed..];
        let len = n as usize;
        for _ in 0..len {
            let asset_info = AssetInfo::parse(remaining)?;
            remaining = asset_info.remaining();
            consumed += asset_info.consumed();
            
        }
        Ok(ParseResult::new(
            &slice[consumed..],
            AssetInfos {
                slice: &slice[..consumed],
                n: len,
            },
        ))
    }
}
impl<'a> AssetInfos<'a> {
    /// Creates an empty [`AssetInfos`]
    pub fn empty() -> Self {
        AssetInfos {
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
    /// Returns an iterator over [`AssetInfo`]
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

impl<'a> IntoIterator for &'a AssetInfos<'a> {
    type Item = AssetInfo<'a>;
    type IntoIter = TxOutIterator<'a>;

    /// Returns an iterator over [`AssetInfo`]
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
    tx_outs: &'a AssetInfos<'a>,
}

impl<'a> Iterator for TxOutIterator<'a> {
    type Item = AssetInfo<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.tx_outs.as_ref().len() {
            None
        } else {
            let tx_out =
            AssetInfo::parse(&self.tx_outs.slice[self.offset..]).expect("granted from parsing");
            self.offset += tx_out.consumed();
            Some(tx_out.parsed_owned())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.elements, Some(self.elements))
    }
}

impl<'a> ExactSizeIterator for TxOutIterator<'a> {}

impl<'a> AsRef<[u8]> for AssetInfos<'a> {
    fn as_ref(&self) -> &[u8] {
        self.slice
    }
}


