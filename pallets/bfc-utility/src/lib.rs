#![cfg_attr(not(feature = "std"), no_std)]

pub mod migrations;
mod pallet;
pub mod weights;

use frame_support::traits::Currency;
pub use pallet::{pallet::*, *};
use weights::WeightInfo;

use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

use scale_info::prelude::string::String;
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

/// The type that indicates the index of a general proposal
pub type PropIndex = u32;

#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug, TypeInfo)]
/// A value placed in storage that represents the current version of the BFC Utility storage. This
/// value is used by the `on_runtime_upgrade` logic to determine whether we run storage migration
/// logic.
enum Releases {
	V1_0_0,
	V2_0_0,
}

impl Default for Releases {
	fn default() -> Self {
		Releases::V2_0_0
	}
}

#[derive(Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
/// The information of a general proposal
pub struct Proposal {
	/// The hexadecimal hash of the proposal data
	pub proposal_hex: String,
	/// The index of this proposal
	pub proposal_index: PropIndex,
}
