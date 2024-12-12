use alloc::string::String;
use crate::{Parse, ParseResult, SResult};
use crate::bsl::VarString;

/// Contains AssetName in an AssetInfo
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetName<'a> {
    slice: &'a [u8],
    /// Required. Examples: ordx, ordinals, brc20, runes, eth, etc.
    pub protocol: VarString<'a>,
    /// Optional. Default is "ft", refer to indexer's definition
    pub type_: VarString<'a>,
    /// If Type is nft, ticker is collection name#inscription number (or satoshi number)
    pub ticker: VarString<'a>,
}
impl<'a> Parse<'a> for AssetName<'a> {
    fn parse(slice: &'a [u8]) -> SResult<Self> {
        let protocol = VarString::parse(slice)?;
        let type_ = VarString::parse(protocol.remaining())?;
        let ticker = VarString::parse(type_.remaining())?;
        let consumed = protocol.consumed() + type_.consumed() + ticker.consumed();
        let remaining = &slice[consumed..];
        let asset_name = AssetName {
            slice: &slice[..consumed],
            protocol: protocol.parsed_owned(),
            type_: type_.parsed_owned(),
            ticker: ticker.parsed_owned(),
        };
        Ok(ParseResult::new(remaining, asset_name))
    }
}

impl<'a> AssetName<'a> {
    /// Returns the protocol
    pub fn protocol(&self) -> String {
        self.protocol.string()
    }

    /// Returns the type
    pub fn type_(&self) -> String {
        self.type_.string()
    }
    /// Returns the ticker
    pub fn ticker(&self) -> String {
        self.ticker.string()
    }
}

impl<'a> AsRef<[u8]> for AssetName<'a> {
    fn as_ref(&self) -> &[u8] {
        self.slice
    }
}

#[cfg(feature = "bitcoin_with_satsnet")]
impl<'a> Into<bitcoin::blockdata::transaction::AssetName> for &AssetName<'a> {
    fn into(self) -> bitcoin::blockdata::transaction::AssetName {
        bitcoin::blockdata::transaction::AssetName {
            protocol: self.protocol(),
            type_: self.type_(),
            ticker: self.ticker(),
        }
    }
}
