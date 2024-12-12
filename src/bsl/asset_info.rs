use crate::{
    bsl::AssetName,
    number::{U16,I64},
    Parse, ParseResult, SResult,
};

/// Contains AssetInfo in an output
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetInfo<'a> {
    slice: &'a [u8],
    name: AssetName<'a>,
    amount: i64,
    binding_sat: u16,
}

impl<'a> Parse<'a> for AssetInfo<'a> {
    fn parse(slice: &'a [u8]) -> SResult<Self> {
        let name = AssetName::parse(slice)?;
        let amount = I64::parse(name.remaining())?;
        let binding_sat = U16::parse(amount.remaining())?;
        let consumed = name.consumed() + amount.consumed() + binding_sat.consumed();
        let asset_info = AssetInfo {
            slice: &slice[..consumed],
            name: name.parsed_owned(),
            amount: amount.parsed_owned().into(),
            binding_sat: binding_sat.parsed().into(),
        };
        Ok(ParseResult::new(binding_sat.remaining(), asset_info))
    }
}
impl<'a> AssetInfo<'a> {
    /// Returns name
    pub fn name(&self) -> &AssetName {
        &self.name
    }
    /// Return the amount
    pub fn amount(&self) -> i64 {
        self.amount
    }
    /// Returns the binding_sat
    pub fn binding_sat(&self) -> u16 {
        self.binding_sat
    }
}

impl<'a> AsRef<[u8]> for AssetInfo<'a> {
    fn as_ref(&self) -> &[u8] {
        self.slice
    }
}

#[cfg(feature = "bitcoin_with_satsnet")]
impl<'a> Into<bitcoin::blockdata::transaction::AssetInfo> for &AssetInfo<'a> {
    fn into(self) -> bitcoin::blockdata::transaction::AssetInfo {
        bitcoin::blockdata::transaction::AssetInfo {
            name: (&self.name).into(),
            amount: self.amount,
            binding_sat: self.binding_sat,
        }
    }
}
