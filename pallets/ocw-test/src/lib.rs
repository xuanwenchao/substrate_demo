#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use sp_runtime::offchain::{ http, Duration };
use serde::{Deserialize};
use frame_support::sp_std::vec::Vec;

use frame_system::{
	offchain::{
		AppCrypto, CreateSignedTransaction, SendUnsignedTransaction,
		SignedPayload, Signer, SigningTypes,
	},
};

use sp_core::crypto::KeyTypeId;

pub use pallet::*;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"xuan");

pub mod crypto {
	use super::KEY_TYPE;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		MultiSignature, MultiSigner,
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct SignedPayloadAuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for SignedPayloadAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}


#[frame_support::pallet]
pub mod pallet {
	use super::*;


	#[derive(Debug, Deserialize)]
	struct PriceStructData {
		result: bool,
		price: u32,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct PayloadStruct<Public> {
		price: u64,
		public: Public,
	}
	impl<T: SigningTypes> SignedPayload<T> for PayloadStruct<T::Public> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}


	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Something<T> = StorageValue<_, u64>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored { something: u64, who: T::AccountId },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}

	
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
		pub fn unsigned_extrinsic_with_signed_payload(origin: OriginFor<T>, payload: PayloadStruct<T::Public>, _signature: T::Signature,) -> DispatchResult {
			ensure_none(origin)?;
			let value:u64 = payload.price;
			<Something<T>>::put(value);

            log::info!("### OCW ==> in call unsigned_extrinsic_with_signed_payload: price={:?}", value);

			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T:Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: T::BlockNumber) {
			//通过Http接口获取外部价格
			let price = Self::get_price().unwrap_or(0);
			let value:u64 = price.into();

			let signer = Signer::<T, T::AuthorityId>::any_account();

			if let Some((_, res)) = signer.send_unsigned_transaction(
				// this line is to prepare and return payload
				|acct| PayloadStruct { price: value, public: acct.public.clone() },
				|payload, signature| Call::unsigned_extrinsic_with_signed_payload { payload, signature },
			) {
				match res {
					Ok(()) => {log::info!("### OCW ==> unsigned tx with signed payload successfully sent.");}
					Err(()) => {log::error!("### OCW ==> sending unsigned tx with signed payload failed.");}
				};
			} else {
				// The case of `None`: no account is available for sending
				log::error!("### OCW ==> No local account available");
			}


		}
	}

	#[pallet::validate_unsigned]
		impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			const UNSIGNED_TXS_PRIORITY: u64 = 100;
			let valid_tx = |provide| ValidTransaction::with_tag_prefix("my-pallet")
				.priority(UNSIGNED_TXS_PRIORITY) // please define `UNSIGNED_TXS_PRIORITY` before this line
				.and_provides([&provide])
				.longevity(3)
				.propagate(true)
				.build();
			
			match call {
				Call::unsigned_extrinsic_with_signed_payload {
					ref payload,
					ref signature
				} => {
					if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
						return InvalidTransaction::BadProof.into();
					}
					valid_tx(b"unsigned_extrinsic_with_signed_payload".to_vec())
				},
				_ => InvalidTransaction::Call.into(),
			}
		}
	}

	impl<T:Config> Pallet<T> {
		fn get_price() -> Result<u32,http::Error>{
			let deadline = sp_io::offchain::timestamp().add(Duration::from_millis(2_000));
			
			let request =
			sp_runtime::offchain::http::Request::get("http://mock.apistub.cn/user/gh_1876277/SampleProject/getprice?who=xuan");
			
			let pending = request.deadline(deadline).send().map_err(|_| http::Error::IoError)?;
	
			let response = pending.try_wait(deadline).map_err(|_| http::Error::DeadlineReached)??;
			// Let's check the status code before we proceed to reading the response.
			if response.code != 200 {
				log::warn!("### Unexpected status code: {}", response.code);
				return Err(http::Error::Unknown)
			}
	
			// Next we want to fully read the response body and collect it to a vector of bytes.
			// Note that the return object allows you to read the body in chunks as well
			// with a way to control the deadline.
			let body = response.body().collect::<Vec<u8>>();
	
			// Create a str slice from the body.
			let body_str = sp_std::str::from_utf8(&body).map_err(|_| {
				log::warn!("No UTF8 body");
				http::Error::Unknown
			})?;

			log::warn!("### Got response: body_str= {:?}", body_str);

			let price_struct: PriceStructData = serde_json::from_str(body_str).map_err(|_| {
				log::warn!("No UTF8 body");
				http::Error::Unknown
			})?;
			log::warn!("### Got response: price_struct= {:?}", price_struct);


			let price = price_struct.price;
	
			log::warn!("Got price: {} cents", price);
	
			Ok(price)
		}
	}

}
