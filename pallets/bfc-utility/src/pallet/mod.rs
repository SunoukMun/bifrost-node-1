use crate::{PropIndex, Proposal, Releases, WeightInfo};

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

use impl_serde::serialize::to_hex;
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {

	use super::*;

	/// Pallet for bfc utility
	#[pallet::pallet]
	#[pallet::generate_store(pub(crate) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configuration trait of this pallet
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Overarching event type
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(crate) fn deposit_event)]
	pub enum Event<T: Config> {
		/// A motion has been proposed by a public account.
		Proposed { proposal_index: PropIndex },
	}

	#[pallet::storage]
	/// Storage version of this pallet.
	pub(crate) type StorageVersion<T: Config> = StorageValue<_, Releases, ValueQuery>;

	#[pallet::storage]
	/// Storage for accepted proposals. Proposal passed by governance will be stored here.
	pub type AcceptedProposals<T: Config> = StorageValue<_, Vec<Proposal>, ValueQuery>;

	#[pallet::storage]
	/// Storage for proposal index. Whenever proposal is accepted, index will be increased.
	pub type ProposalIndex<T: Config> = StorageValue<_, PropIndex, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig {}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self {}
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			StorageVersion::<T>::put(Releases::V1_0_0);
			ProposalIndex::<T>::put(0);
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(<T as Config>::WeightInfo::community_proposal())]
		/// General Proposal
		/// ####
		/// General community proposal without changes on codes.
		pub fn community_proposal(
			origin: OriginFor<T>,
			proposal: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			ensure_root(origin)?;

			let mut proposal_index = ProposalIndex::<T>::get();
			let proposal = Proposal { proposal_hex: to_hex(&proposal[..], true), proposal_index };
			let mut proposals = AcceptedProposals::<T>::get();
			proposals.push(proposal);
			AcceptedProposals::<T>::put(proposals);
			proposal_index += 1;
			ProposalIndex::<T>::put(proposal_index);

			Self::deposit_event(Event::Proposed { proposal_index });
			Ok(().into())
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn decode_works() {
		let proposal = b"This is test proposal";
		let hex = to_hex(proposal, true);
		let decode = sp_core::bytes::from_hex(hex.as_str()).unwrap();
		assert_eq!(decode, "This is test proposal".as_bytes());
	}
}
