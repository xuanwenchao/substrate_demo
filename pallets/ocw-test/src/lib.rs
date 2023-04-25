#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::{
	offchain::{AppCrypto, CreateSignedTransaction, SendSignedTransaction, Signer, SubmitTransaction},
	pallet_prelude::*,
};
use sp_core::crypto::KeyTypeId;

pub use pallet::*;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"xuan");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		// traits::Verify,
		MultiSignature,
		MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

#[frame_support::pallet]
pub mod pallet {

	use super::*;
	use frame_support::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something_signed)]
	pub type SomethingSigned<T> = StorageValue<_, u64>;

	#[pallet::storage]
	#[pallet::getter(fn something_unsigned)]
	pub type SomethingUnSigned<T> = StorageValue<_, u64>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SendSignedSomething(u64, T::AccountId),
		SendUnSignedSomething(u64),
	}

	#[pallet::error]
	pub enum Error<T> {
		OffchainunsignedTransactionError,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn submit_send_signed_something(
			//签名交易函数
			origin: OriginFor<T>,
			blocknumber: u64,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			log::info!(
				"### submit_send_signed_something who:[{:?}], number:[{:?}]",
				who,
				blocknumber
			);
			<SomethingSigned<T>>::put(blocknumber);
			Self::deposit_event(Event::SendSignedSomething(blocknumber, who));
			Ok(())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn submit_send_unsigned_something(
			//未签名交易函数
			origin: OriginFor<T>,
			blocknumber: u64,
		) -> DispatchResult {
			ensure_none(origin)?;
			<SomethingUnSigned<T>>::put(blocknumber);
			Self::deposit_event(Event::SendUnSignedSomething(blocknumber));
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			let number: u64 = block_number.try_into().unwrap_or(0);
			log::info!("### offchain_worker signer number:[{:?}]", number);

			//-----------------------------未签名交易函数的调用--------------------------			
			let call = Call::submit_send_unsigned_something { blocknumber:number };
			if let Err(e) =
				SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
					.map_err(|_| <Error<T>>::OffchainunsignedTransactionError)
			{
				log::error!(target:"ocw-test","### offchain_worker submit unsigned transaction error:{:?}",e);
			} else {
				log::info!(target:"ocw-test","### offchain_worker submit unsigned transaction successful!");
			}

			//-----------------------------签名交易函数的调用--------------------------
			let signer = Signer::<T, T::AuthorityId>::any_account();
			log::info!("### offchain_worker signer can sign:[{:?}]", signer.can_sign());
			let result = signer.send_signed_transaction(|_account| {
				Call::submit_send_signed_something { blocknumber: number }
			});

			if let Some((_acc, res)) = result {
				if res.is_err() {
					if let Err(e) = res {
						log::info!("### Error! send_signed_transaction:[{:?}]", e);
					} else {
						log::info!("### Error! Unknow100");
					}
				} else {
					log::info!("### send_signed_transaction Successful!");
				}
			} else {
				log::info!("### Error! NoAccountForSigning!");
			}
		}
	}

   //需要实现未签名交易的验证trait
	#[pallet::validate_unsigned]
	impl<T:Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source:TransactionSource, call:&Self::Call) -> TransactionValidity{
			if let Call::submit_send_unsigned_something{ blocknumber:_ } = call{
				ValidTransaction::with_tag_prefix("OcwUnsigtx")
				.priority(TransactionPriority::max_value())
				.longevity(1)
				.propagate(false)
				.build()
			} else {
				InvalidTransaction::Call.into()
			}
		}
	}
}
