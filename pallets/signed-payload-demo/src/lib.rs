#![cfg_attr(not(feature = "std"), no_std)]

use frame_system::{
	offchain::{AppCrypto, CreateSignedTransaction, SignedPayload, Signer, SigningTypes, SendUnsignedTransaction},
	pallet_prelude::*,
};
use sp_core::crypto::KeyTypeId;

pub use pallet::*;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"asdf");

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
	pub type SomethingSigned<T> = StorageValue<_, (u64, u64)>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SendUnSignedSomething(u64, u64),
	}

	#[pallet::error]
	pub enum Error<T> {
		OffchainunsignedTransactionError,
	}

	//定义payload的结构
	#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
	pub struct SomethingPayload<Public, BlockNumber> {
		block_number: BlockNumber,
		something: u64,
		public: Public,
	}

	//为自定义的payload的结构实现 SignedPayload trait
	impl<T: SigningTypes> SignedPayload<T> for SomethingPayload<T::Public, T::BlockNumber> {
		fn public(&self) -> T::Public {
			self.public.clone()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		//提交仅内容签名的、交易未签名的函数
		pub fn submit_signed_payload_with_unsigned_something(
			origin: OriginFor<T>,
			something_payload: SomethingPayload<T::Public, T::BlockNumber>,
			_signature: T::Signature,
		) -> DispatchResult {
			log::info!("@@@ submit_signed_payload_with_unsigned_something start .............");
			ensure_none(origin)?;
			let number: u64 = something_payload.block_number.try_into().unwrap_or(0);
			let something: u64 = something_payload.something.try_into().unwrap_or(0);

			<SomethingSigned<T>>::put((number, something));
			Self::deposit_event(Event::SendUnSignedSomething(number, something));

			log::info!(
				"@@@ submit_signed_payload_with_unsigned_something: number={:?}, somthing={:?}",
				number,
				something
			);
			Ok(())
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			let number: u64 = block_number.try_into().unwrap_or(0);
			log::info!("@@@ offchain_worker signer number:[{:?}]", number);

		    let signer = Signer::<T, T::AuthorityId>::any_account();
			let _ = signer.send_unsigned_transaction(
				|account| SomethingPayload {
					//构造payload
					block_number,
					something: number,
					public: account.public.clone(),
				},
				|payload, signature| Call::submit_signed_payload_with_unsigned_something {
					something_payload: payload,
					signature,
				},
			);
		}
	}

	//需要实现未签名交易的验证trait
	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Call::submit_signed_payload_with_unsigned_something {
				something_payload: ref payload,
				ref signature,
			} = call
			{
				let signature_valid =
					SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone());
				if !signature_valid {
					return InvalidTransaction::BadProof.into()
				}

				ValidTransaction::with_tag_prefix("OcwUnsigtxPayload")
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
