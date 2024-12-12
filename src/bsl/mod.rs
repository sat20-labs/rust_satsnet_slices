//! Contains all blockhain objects parsed from byte slices.
//!
//! Everyone of this objects keep the byte slice from which they were parsed, so "deserialization"
//! is free.
//!
//! Everyone of this object doesn't offer iteration of the contents on purpose, doing so would mean
//! deserialize the object more than once or allocating. Instead if you want to operate on the
//! parsed values you need to implement a [`crate::Visitor`].
//!
//! Other than the slice from they have been created these object may contain fields that are needed
//! from the caller without requiring re-parsing.

mod block;
mod block_header;
mod len;
mod out_point;
mod script;
mod var_string;

#[cfg(feature = "bitcoin_with_satsnet")]
mod asset_name;
#[cfg(feature = "bitcoin_with_satsnet")]
mod asset_info;
#[cfg(feature = "bitcoin_with_satsnet")]
mod asset_infos;

mod transaction;
mod tx_in;
mod tx_ins;
mod tx_out;
mod tx_outs;
mod witness;
mod witnesses;

pub use block::Block;
pub use block_header::BlockHeader;
pub use len::parse_len;
pub use len::Len;
pub use out_point::OutPoint;
pub use script::Script;
pub use var_string::VarString;

#[cfg(feature = "bitcoin_with_satsnet")]
pub use asset_name::AssetName;
#[cfg(feature = "bitcoin_with_satsnet")]
pub use asset_info::AssetInfo;
#[cfg(feature = "bitcoin_with_satsnet")]
pub use asset_infos::AssetInfos;

pub use transaction::Transaction;
pub use tx_in::TxIn;
pub use tx_ins::TxIns;
pub use tx_out::TxOut;
pub use tx_outs::TxOuts;
pub use witness::Witness;
pub use witnesses::Witnesses;

#[cfg(all(feature = "bitcoin", feature = "sha2"))]
pub use block::visitor::FindTransaction;
