#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_io::offchain_index;
	use frame_support::sp_std::vec::Vec;
	use sp_runtime::offchain::storage::StorageValueRef;

	// use serde::{Serialize, Deserialize};

	const ONCHAIN_TX_KEY: &[u8] = b"offchain_tx_key_for_test_1234567890";

    //定义存储的数据结构
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug)]
	struct OffchainIndexingData{
		pub indexing_data: Vec<u8>,
		pub block_number: u64,
	}


	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		 type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		OffChainIndexWriteSuccessful(Vec<u8>,u64),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		InvalidData,
	}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn set_on_chain_data(origin: OriginFor<T>, data: Vec<u8>) -> DispatchResultWithPostInfo {
			let _who = ensure_signed(origin)?;
			ensure!(!data.is_empty(), Error::<T>::InvalidData);

			let number: u64 = <frame_system::Pallet<T>>::block_number().try_into().unwrap_or(0);
			let data = OffchainIndexingData{indexing_data: data, block_number: number};
			offchain_index::set(ONCHAIN_TX_KEY.clone(), &data.encode());
			Self::deposit_event(Event::OffChainIndexWriteSuccessful(data.indexing_data, data.block_number));
			Ok(().into())
		}
	}

	#[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: T::BlockNumber) {
			let storage_ref = StorageValueRef::persistent(&ONCHAIN_TX_KEY);
			if let Ok(Some(data)) = storage_ref.get::<OffchainIndexingData>() {
				log::info!("read storage data: {:?} - {:?}", data.indexing_data, data.block_number);
			} else {
				log::info!("Error: read storage data");
			}
		}
	}
}
