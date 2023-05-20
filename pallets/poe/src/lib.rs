#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;


#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, BoundedVec};
	use frame_system::{ensure_signed, pallet_prelude::*};
    // use sp_std::prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[pallet::constant]
		type MaxClaimLength: Get<u32>;
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BoundedVec<u8, T::MaxClaimLength>,
		(T::AccountId, T::BlockNumber),
	>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreatedEvent(T::AccountId, BoundedVec<u8,T::MaxClaimLength>),
		ClaimRevokedEvent(T::AccountId,  BoundedVec<u8,T::MaxClaimLength>),
        ClaimTransferEvent(T::AccountId, BoundedVec<u8,T::MaxClaimLength>,T::AccountId), //添加了转移存证的Event
	}

	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ClaimTooLong,
		ClaimNotExist,
		NotClaimOwner,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}


	#[pallet::call]
	impl<T: Config> Pallet<T> {
        //创建存证的功能，
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn create_claim(origin: OriginFor<T>, claim: BoundedVec<u8,T::MaxClaimLength>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			// let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
			// 	.map_err(|_| Error::<T>::ClaimTooLong)?;

			ensure!(!Proofs::<T>::contains_key(&claim), Error::<T>::ProofAlreadyExist);

			Proofs::<T>::insert(
				&claim,
				(sender.clone(), frame_system::Pallet::<T>::block_number()),
			);

			Self::deposit_event(Event::ClaimCreatedEvent(sender, claim));
			Ok(().into())
		}

        //删除存证的功能，
		#[pallet::call_index(1)]
		#[pallet::weight(0)]
		pub fn revoke_claim(origin: OriginFor<T>, claim:  BoundedVec<u8,T::MaxClaimLength>) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?;

			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

			ensure!(owner == sender, Error::<T>::NotClaimOwner);

			Proofs::<T>::remove(&claim);

			Self::deposit_event(Event::ClaimRevokedEvent(sender, claim));

			Ok(().into())
		}

        //转移存证的功能，增加了接收账户的地址做为参数 dest
		#[pallet::call_index(2)]
        #[pallet::weight(0)]
		pub fn transfer_claim(origin: OriginFor<T>, claim: BoundedVec<u8,T::MaxClaimLength>, dest: T::AccountId) -> DispatchResultWithPostInfo {
            let sender = ensure_signed(origin)?;

            // let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone())
			// 	.map_err(|_| Error::<T>::ClaimTooLong)?;

			let (owner, _) = Proofs::<T>::get(&claim).ok_or(Error::<T>::ClaimNotExist)?;

             ensure!(owner == sender, Error::<T>::NotClaimOwner);

			 Proofs::<T>::remove(&claim); //remove proofs from origin owner

			//insert proofs to new dest
             Proofs::<T>::insert(
				&claim,
				(dest.clone(), frame_system::Pallet::<T>::block_number()),
			);

            Self::deposit_event(Event::ClaimTransferEvent(sender, claim, dest));

			Ok(().into())

        }
	}
}
