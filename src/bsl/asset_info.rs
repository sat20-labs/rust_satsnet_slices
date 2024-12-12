use crate::{
    bsl::AssetName,
    Parse, ParseResult, SResult,
};

use super::len::{parse_var_int, VarInt};

// use log::{error, warn, info, debug, trace};

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
        let var_int_amount: VarInt<'_> = parse_var_int(name.remaining())?;
        let var_int_binding_sat = parse_var_int(var_int_amount.remaining)?;
        let consumed = name.consumed() + var_int_amount.consumed + var_int_binding_sat.consumed;
        let remaining = &slice[consumed..];
        let asset_info = AssetInfo {
            slice: &slice[..consumed],
            name: name.parsed_owned(),
            amount: var_int_amount.n as i64,
            binding_sat: var_int_binding_sat.n as u16,
        };
        Ok(ParseResult::new(remaining, asset_info))
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
