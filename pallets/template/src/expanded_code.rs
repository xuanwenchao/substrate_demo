#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;
/**
			The module that hosts all the
			[FRAME](https://docs.substrate.io/main-docs/build/events-errors/)
			types needed to add this pallet to a
			runtime.
			*/
pub mod pallet {
    use codec::Codec;
    use frame_support::{
        pallet_prelude::*, sp_runtime::traits::AtLeast32BitUnsigned, sp_std::fmt::Debug,
    };
    use frame_system::pallet_prelude::*;
    /**
			The [pallet](https://docs.substrate.io/reference/frame-pallets/#pallets) implementing
			the on-chain logic.
			*/
    pub struct Pallet<T>(frame_support::sp_std::marker::PhantomData<(T)>);
    const _: () = {
        impl<T> core::clone::Clone for Pallet<T> {
            fn clone(&self) -> Self {
                Self(core::clone::Clone::clone(&self.0))
            }
        }
    };
    const _: () = {
        impl<T> core::cmp::Eq for Pallet<T> {}
    };
    const _: () = {
        impl<T> core::cmp::PartialEq for Pallet<T> {
            fn eq(&self, other: &Self) -> bool {
                true && self.0 == other.0
            }
        }
    };
    const _: () = {
        impl<T> core::fmt::Debug for Pallet<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                fmt.debug_tuple("Pallet").field(&self.0).finish()
            }
        }
    };
    /// Configure the pallet by specifying the parameters and types on which it depends.
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type RuntimeEvent: From<Event<Self>>
            + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Balance: Member
            + Parameter
            + AtLeast32BitUnsigned
            + Codec
            + From<u32>
            + Into<u32>
            + Copy
            + Debug
            + Default
            + MaxEncodedLen
            + MaybeSerializeDeserialize;
    }
    #[allow(type_alias_bounds)]
    pub type Something<T> = StorageValue<_GeneratedPrefixForStorageSomething<T>, u32>;
    #[allow(type_alias_bounds)]
    pub type AccountMap<T: Config> = StorageMap<
        _GeneratedPrefixForStorageAccountMap<T>,
        Blake2_128Concat,
        T::AccountId,
        T::Balance,
    >;
    /**
			The [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted
			by this pallet.
			*/
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored { something: u32, who: T::AccountId },
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T)>, frame_support::Never),
    }
    const _: () = {
        impl<T: Config> core::clone::Clone for Event<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::SomethingStored { ref something, ref who } => {
                        Self::SomethingStored {
                            something: core::clone::Clone::clone(something),
                            who: core::clone::Clone::clone(who),
                        }
                    }
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(
                            core::clone::Clone::clone(_0),
                            core::clone::Clone::clone(_1),
                        )
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Event<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Event<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (
                        Self::SomethingStored { something, who },
                        Self::SomethingStored { something: _0, who: _1 },
                    ) => true && something == _0 && who == _1,
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (Self::SomethingStored { .. }, Self::__Ignore { .. }) => false,
                    (Self::__Ignore { .. }, Self::SomethingStored { .. }) => false,
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::fmt::Debug for Event<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::SomethingStored { ref something, ref who } => {
                        fmt.debug_struct("Event::SomethingStored")
                            .field("something", &something)
                            .field("who", &who)
                            .finish()
                    }
                    Self::__Ignore(ref _0, ref _1) => {
                        fmt.debug_tuple("Event::__Ignore").field(&_0).field(&_1).finish()
                    }
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::codec::Encode for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Event::SomethingStored { ref something, ref who } => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                        ::codec::Encode::encode_to(something, __codec_dest_edqy);
                        ::codec::Encode::encode_to(who, __codec_dest_edqy);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::codec::EncodeLike for Event<T>
        where
            T::AccountId: ::codec::Encode,
            T::AccountId: ::codec::Encode,
        {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T: Config> ::codec::Decode for Event<T>
        where
            T::AccountId: ::codec::Decode,
            T::AccountId: ::codec::Decode,
        {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Event`, failed to read variant byte")
                    })?
                {
                    __codec_x_edqy if __codec_x_edqy
                        == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Event::<T>::SomethingStored {
                            something: {
                                let __codec_res_edqy = <u32 as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e
                                                .chain(
                                                    "Could not decode `Event::SomethingStored::something`",
                                                ),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                            who: {
                                let __codec_res_edqy = <T::AccountId as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Event::SomethingStored::who`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    _ => {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into("Could not decode `Event`, variant doesn't exist"),
                        )
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Event<T>
        where
            T::AccountId: ::scale_info::TypeInfo + 'static,
            frame_support::sp_std::marker::PhantomData<
                (T),
            >: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Event", "pallet_template::pallet"))
                    .type_params(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(
                        &[
                            "\n\t\t\tThe [event](https://docs.substrate.io/main-docs/build/events-errors/) emitted\n\t\t\tby this pallet.\n\t\t\t",
                        ],
                    )
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "SomethingStored",
                                |v| {
                                    v
                                        .index(0usize as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::named()
                                                .field(|f| f.ty::<u32>().name("something").type_name("u32"))
                                                .field(|f| {
                                                    f.ty::<T::AccountId>().name("who").type_name("T::AccountId")
                                                }),
                                        )
                                        .docs_always(
                                            &[
                                                "Event documentation should end with an array that provides descriptive names for event",
                                                "parameters. [something, who]",
                                            ],
                                        )
                                },
                            ),
                    )
            }
        }
    };
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    /**
			Custom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)
			of this pallet.
			*/
    pub enum Error<T> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T)>, frame_support::Never),
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T> ::codec::Encode for Error<T> {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Error::NoneValue => {
                        __codec_dest_edqy.push_byte(0usize as ::core::primitive::u8);
                    }
                    Error::StorageOverflow => {
                        __codec_dest_edqy.push_byte(1usize as ::core::primitive::u8);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T> ::codec::EncodeLike for Error<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[automatically_derived]
        impl<T> ::codec::Decode for Error<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Error`, failed to read variant byte")
                    })?
                {
                    __codec_x_edqy if __codec_x_edqy
                        == 0usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::NoneValue)
                    }
                    __codec_x_edqy if __codec_x_edqy
                        == 1usize as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Error::<T>::StorageOverflow)
                    }
                    _ => {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into("Could not decode `Error`, variant doesn't exist"),
                        )
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T> ::scale_info::TypeInfo for Error<T>
        where
            frame_support::sp_std::marker::PhantomData<
                (T),
            >: ::scale_info::TypeInfo + 'static,
            T: 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Error", "pallet_template::pallet"))
                    .type_params(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(
                        &[
                            "\n\t\t\tCustom [dispatch errors](https://docs.substrate.io/main-docs/build/events-errors/)\n\t\t\tof this pallet.\n\t\t\t",
                        ],
                    )
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "NoneValue",
                                |v| {
                                    v
                                        .index(0usize as ::core::primitive::u8)
                                        .docs_always(&["Error names should be descriptive."])
                                },
                            )
                            .variant(
                                "StorageOverflow",
                                |v| {
                                    v
                                        .index(1usize as ::core::primitive::u8)
                                        .docs_always(
                                            &[
                                                "Errors should have helpful documentation associated with them.",
                                            ],
                                        )
                                },
                            ),
                    )
            }
        }
    };
    const _: () = {
        impl<T> frame_support::traits::PalletError for Error<T> {
            const MAX_ENCODED_SIZE: usize = 1;
        }
    };
    /**
					Can be used to configure the
					[genesis state](https://docs.substrate.io/v3/runtime/chain-specs#the-genesis-state)
					of this pallet.
					*/
    #[cfg(feature = "std")]
    #[serde(rename_all = "camelCase")]
    #[serde(deny_unknown_fields)]
    #[serde(bound(serialize = ""))]
    #[serde(bound(deserialize = ""))]
    #[serde(crate = "frame_support::serde")]
    pub struct GenesisConfig<T: Config> {
        pub something: u32,
        pub accounts: Vec<(T::AccountId, T::Balance)>,
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use frame_support::serde as _serde;
        #[automatically_derived]
        impl<T: Config> frame_support::serde::Serialize for GenesisConfig<T> {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> frame_support::serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: frame_support::serde::Serializer,
            {
                let mut __serde_state = match _serde::Serializer::serialize_struct(
                    __serializer,
                    "GenesisConfig",
                    false as usize + 1 + 1,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "something",
                    &self.something,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                match _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "accounts",
                    &self.accounts,
                ) {
                    _serde::__private::Ok(__val) => __val,
                    _serde::__private::Err(__err) => {
                        return _serde::__private::Err(__err);
                    }
                };
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        use frame_support::serde as _serde;
        #[automatically_derived]
        impl<'de, T: Config> frame_support::serde::Deserialize<'de>
        for GenesisConfig<T> {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> frame_support::serde::__private::Result<Self, __D::Error>
            where
                __D: frame_support::serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                enum __Field {
                    __field0,
                    __field1,
                }
                struct __FieldVisitor;
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"field index 0 <= i < 2",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "something" => _serde::__private::Ok(__Field::__field0),
                            "accounts" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"something" => _serde::__private::Ok(__Field::__field0),
                            b"accounts" => _serde::__private::Ok(__Field::__field1),
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_field(__value, FIELDS),
                                )
                            }
                        }
                    }
                }
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                struct __Visitor<'de, T: Config> {
                    marker: _serde::__private::PhantomData<GenesisConfig<T>>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                impl<'de, T: Config> _serde::de::Visitor<'de> for __Visitor<'de, T> {
                    type Value = GenesisConfig<T>;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct GenesisConfig",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match match _serde::de::SeqAccess::next_element::<
                            u32,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct GenesisConfig with 2 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match match _serde::de::SeqAccess::next_element::<
                            Vec<(T::AccountId, T::Balance)>,
                        >(&mut __seq) {
                            _serde::__private::Ok(__val) => __val,
                            _serde::__private::Err(__err) => {
                                return _serde::__private::Err(__err);
                            }
                        } {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct GenesisConfig with 2 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(GenesisConfig {
                            something: __field0,
                            accounts: __field1,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<u32> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<
                            Vec<(T::AccountId, T::Balance)>,
                        > = _serde::__private::None;
                        while let _serde::__private::Some(__key)
                            = match _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map) {
                                _serde::__private::Ok(__val) => __val,
                                _serde::__private::Err(__err) => {
                                    return _serde::__private::Err(__err);
                                }
                            } {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "something",
                                            ),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<u32>(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "accounts",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        match _serde::de::MapAccess::next_value::<
                                            Vec<(T::AccountId, T::Balance)>,
                                        >(&mut __map) {
                                            _serde::__private::Ok(__val) => __val,
                                            _serde::__private::Err(__err) => {
                                                return _serde::__private::Err(__err);
                                            }
                                        },
                                    );
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("something") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                match _serde::__private::de::missing_field("accounts") {
                                    _serde::__private::Ok(__val) => __val,
                                    _serde::__private::Err(__err) => {
                                        return _serde::__private::Err(__err);
                                    }
                                }
                            }
                        };
                        _serde::__private::Ok(GenesisConfig {
                            something: __field0,
                            accounts: __field1,
                        })
                    }
                }
                const FIELDS: &'static [&'static str] = &["something", "accounts"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "GenesisConfig",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<GenesisConfig<T>>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                something: Default::default(),
                accounts: Default::default(),
            }
        }
    }
    #[cfg(feature = "std")]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            Something::<T>::put(self.something);
            for (account, balance) in &self.accounts {
                AccountMap::<T>::insert(account, balance);
            }
        }
    }
    impl<T: Config> Pallet<T> {
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                let who = ensure_signed(origin)?;
                <Something<T>>::put(something);
                Self::deposit_event(Event::SomethingStored {
                    something,
                    who,
                });
                Ok(())
            })
        }
        /// An example dispatchable that may throw a custom error.
        pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
            frame_support::storage::with_storage_layer(|| {
                let _who = ensure_signed(origin)?;
                match <Something<T>>::get() {
                    None => return Err(Error::<T>::NoneValue.into()),
                    Some(old) => {
                        let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
                        <Something<T>>::put(new);
                        Ok(())
                    }
                }
            })
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_documentation_metadata() -> frame_support::sp_std::vec::Vec<
            &'static str,
        > {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn pallet_constants_metadata() -> frame_support::sp_std::vec::Vec<
            frame_support::metadata::PalletConstantMetadata,
        > {
            ::alloc::vec::Vec::new()
        }
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn error_metadata() -> Option<frame_support::metadata::PalletErrorMetadata> {
            Some(frame_support::metadata::PalletErrorMetadata {
                ty: frame_support::scale_info::meta_type::<Error<T>>(),
            })
        }
    }
    /// Type alias to `Pallet`, to be used by `construct_runtime`.
    ///
    /// Generated by `pallet` attribute macro.
    #[deprecated(note = "use `Pallet` instead")]
    #[allow(dead_code)]
    pub type Module<T> = Pallet<T>;
    impl<T: Config> frame_support::traits::GetStorageVersion for Pallet<T> {
        fn current_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::default()
        }
        fn on_chain_storage_version() -> frame_support::traits::StorageVersion {
            frame_support::traits::StorageVersion::get::<Self>()
        }
    }
    impl<T: Config> frame_support::traits::OnGenesis for Pallet<T> {
        fn on_genesis() {
            let storage_version = frame_support::traits::StorageVersion::default();
            storage_version.put::<Self>();
        }
    }
    impl<T: Config> frame_support::traits::PalletInfoAccess for Pallet<T> {
        fn index() -> usize {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn module_name() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::module_name::<
                Self,
            >()
                .expect(
                    "Pallet is part of the runtime because pallet `Config` trait is \
						implemented by the runtime",
                )
        }
        fn crate_version() -> frame_support::traits::CrateVersion {
            frame_support::traits::CrateVersion {
                major: 4u16,
                minor: 0u8,
                patch: 0u8,
            }
        }
    }
    impl<T: Config> frame_support::traits::PalletsInfoAccess for Pallet<T> {
        fn count() -> usize {
            1
        }
        fn infos() -> frame_support::sp_std::vec::Vec<
            frame_support::traits::PalletInfoData,
        > {
            use frame_support::traits::PalletInfoAccess;
            let item = frame_support::traits::PalletInfoData {
                index: Self::index(),
                name: Self::name(),
                module_name: Self::module_name(),
                crate_version: Self::crate_version(),
            };
            <[_]>::into_vec(#[rustc_box] ::alloc::boxed::Box::new([item]))
        }
    }
    impl<T: Config> frame_support::traits::StorageInfoTrait for Pallet<T> {
        fn storage_info() -> frame_support::sp_std::vec::Vec<
            frame_support::traits::StorageInfo,
        > {
            #[allow(unused_mut)]
            let mut res = ::alloc::vec::Vec::new();
            {
                let mut storage_info = <Something<
                    T,
                > as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            {
                let mut storage_info = <AccountMap<
                    T,
                > as frame_support::traits::StorageInfoTrait>::storage_info();
                res.append(&mut storage_info);
            }
            res
        }
    }
    use frame_support::traits::{
        StorageInfoTrait, TrackedStorageKey, WhitelistedStorageKeys,
    };
    impl<T: Config> WhitelistedStorageKeys for Pallet<T> {
        fn whitelisted_storage_keys() -> frame_support::sp_std::vec::Vec<
            TrackedStorageKey,
        > {
            use frame_support::sp_std::vec;
            ::alloc::vec::Vec::new()
        }
    }
    mod warnings {}
    #[doc(hidden)]
    pub mod __substrate_call_check {
        #[doc(hidden)]
        pub use __is_call_part_defined_0 as is_call_part_defined;
    }
    ///Contains one variant per dispatchable that can be called by an extrinsic.
    #[codec(encode_bound())]
    #[codec(decode_bound())]
    #[scale_info(skip_type_params(T), capture_docs = "always")]
    #[allow(non_camel_case_types)]
    pub enum Call<T: Config> {
        #[doc(hidden)]
        #[codec(skip)]
        __Ignore(frame_support::sp_std::marker::PhantomData<(T,)>, frame_support::Never),
        /// An example dispatchable that takes a singles value as a parameter, writes the value to
        /// storage and emits an event. This function must be dispatched by a signed extrinsic.
        #[codec(index = 0u8)]
        do_something { #[allow(missing_docs)] something: u32 },
        /// An example dispatchable that may throw a custom error.
        #[codec(index = 1u8)]
        cause_error {},
    }
    const _: () = {
        impl<T: Config> core::fmt::Debug for Call<T> {
            fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::fmt::Result {
                match *self {
                    Self::__Ignore(ref _0, ref _1) => {
                        fmt.debug_tuple("Call::__Ignore").field(&_0).field(&_1).finish()
                    }
                    Self::do_something { ref something } => {
                        fmt.debug_struct("Call::do_something")
                            .field("something", &something)
                            .finish()
                    }
                    Self::cause_error {} => {
                        fmt.debug_struct("Call::cause_error").finish()
                    }
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::clone::Clone for Call<T> {
            fn clone(&self) -> Self {
                match self {
                    Self::__Ignore(ref _0, ref _1) => {
                        Self::__Ignore(
                            core::clone::Clone::clone(_0),
                            core::clone::Clone::clone(_1),
                        )
                    }
                    Self::do_something { ref something } => {
                        Self::do_something {
                            something: core::clone::Clone::clone(something),
                        }
                    }
                    Self::cause_error {} => Self::cause_error {},
                }
            }
        }
    };
    const _: () = {
        impl<T: Config> core::cmp::Eq for Call<T> {}
    };
    const _: () = {
        impl<T: Config> core::cmp::PartialEq for Call<T> {
            fn eq(&self, other: &Self) -> bool {
                match (self, other) {
                    (Self::__Ignore(_0, _1), Self::__Ignore(_0_other, _1_other)) => {
                        true && _0 == _0_other && _1 == _1_other
                    }
                    (
                        Self::do_something { something },
                        Self::do_something { something: _0 },
                    ) => true && something == _0,
                    (Self::cause_error {}, Self::cause_error {}) => true,
                    (Self::__Ignore { .. }, Self::do_something { .. }) => false,
                    (Self::__Ignore { .. }, Self::cause_error { .. }) => false,
                    (Self::do_something { .. }, Self::__Ignore { .. }) => false,
                    (Self::do_something { .. }, Self::cause_error { .. }) => false,
                    (Self::cause_error { .. }, Self::__Ignore { .. }) => false,
                    (Self::cause_error { .. }, Self::do_something { .. }) => false,
                }
            }
        }
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Encode for Call<T> {
            fn encode_to<__CodecOutputEdqy: ::codec::Output + ?::core::marker::Sized>(
                &self,
                __codec_dest_edqy: &mut __CodecOutputEdqy,
            ) {
                match *self {
                    Call::do_something { ref something } => {
                        __codec_dest_edqy.push_byte(0u8 as ::core::primitive::u8);
                        ::codec::Encode::encode_to(something, __codec_dest_edqy);
                    }
                    Call::cause_error {} => {
                        __codec_dest_edqy.push_byte(1u8 as ::core::primitive::u8);
                    }
                    _ => {}
                }
            }
        }
        #[automatically_derived]
        impl<T: Config> ::codec::EncodeLike for Call<T> {}
    };
    #[allow(deprecated)]
    const _: () = {
        #[allow(non_camel_case_types)]
        #[automatically_derived]
        impl<T: Config> ::codec::Decode for Call<T> {
            fn decode<__CodecInputEdqy: ::codec::Input>(
                __codec_input_edqy: &mut __CodecInputEdqy,
            ) -> ::core::result::Result<Self, ::codec::Error> {
                match __codec_input_edqy
                    .read_byte()
                    .map_err(|e| {
                        e.chain("Could not decode `Call`, failed to read variant byte")
                    })?
                {
                    __codec_x_edqy if __codec_x_edqy == 0u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::do_something {
                            something: {
                                let __codec_res_edqy = <u32 as ::codec::Decode>::decode(
                                    __codec_input_edqy,
                                );
                                match __codec_res_edqy {
                                    ::core::result::Result::Err(e) => {
                                        return ::core::result::Result::Err(
                                            e.chain("Could not decode `Call::do_something::something`"),
                                        );
                                    }
                                    ::core::result::Result::Ok(__codec_res_edqy) => {
                                        __codec_res_edqy
                                    }
                                }
                            },
                        })
                    }
                    __codec_x_edqy if __codec_x_edqy == 1u8 as ::core::primitive::u8 => {
                        ::core::result::Result::Ok(Call::<T>::cause_error {})
                    }
                    _ => {
                        ::core::result::Result::Err(
                            <_ as ::core::convert::Into<
                                _,
                            >>::into("Could not decode `Call`, variant doesn't exist"),
                        )
                    }
                }
            }
        }
    };
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        impl<T: Config> ::scale_info::TypeInfo for Call<T>
        where
            frame_support::sp_std::marker::PhantomData<
                (T,),
            >: ::scale_info::TypeInfo + 'static,
            T: Config + 'static,
        {
            type Identity = Self;
            fn type_info() -> ::scale_info::Type {
                ::scale_info::Type::builder()
                    .path(::scale_info::Path::new("Call", "pallet_template::pallet"))
                    .type_params(
                        <[_]>::into_vec(
                            #[rustc_box]
                            ::alloc::boxed::Box::new([
                                ::scale_info::TypeParameter::new(
                                    "T",
                                    ::core::option::Option::None,
                                ),
                            ]),
                        ),
                    )
                    .docs_always(
                        &[
                            "Contains one variant per dispatchable that can be called by an extrinsic.",
                        ],
                    )
                    .variant(
                        ::scale_info::build::Variants::new()
                            .variant(
                                "do_something",
                                |v| {
                                    v
                                        .index(0u8 as ::core::primitive::u8)
                                        .fields(
                                            ::scale_info::build::Fields::named()
                                                .field(|f| f.ty::<u32>().name("something").type_name("u32")),
                                        )
                                        .docs_always(
                                            &[
                                                "An example dispatchable that takes a singles value as a parameter, writes the value to",
                                                "storage and emits an event. This function must be dispatched by a signed extrinsic.",
                                            ],
                                        )
                                },
                            )
                            .variant(
                                "cause_error",
                                |v| {
                                    v
                                        .index(1u8 as ::core::primitive::u8)
                                        .fields(::scale_info::build::Fields::named())
                                        .docs_always(
                                            &["An example dispatchable that may throw a custom error."],
                                        )
                                },
                            ),
                    )
            }
        }
    };
    impl<T: Config> Call<T> {
        ///Create a call with the variant `do_something`.
        pub fn new_call_variant_do_something(something: u32) -> Self {
            Self::do_something { something }
        }
        ///Create a call with the variant `cause_error`.
        pub fn new_call_variant_cause_error() -> Self {
            Self::cause_error {}
        }
    }
    impl<T: Config> frame_support::dispatch::GetDispatchInfo for Call<T> {
        fn get_dispatch_info(&self) -> frame_support::dispatch::DispatchInfo {
            match *self {
                Self::do_something { ref something } => {
                    let __pallet_base_weight = 10_000
                        + T::DbWeight::get().writes(1).ref_time();
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<
                        (&u32,),
                    >>::weigh_data(&__pallet_base_weight, (something,));
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<
                        (&u32,),
                    >>::classify_dispatch(&__pallet_base_weight, (something,));
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<
                        (&u32,),
                    >>::pays_fee(&__pallet_base_weight, (something,));
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::cause_error {} => {
                    let __pallet_base_weight = 10_000
                        + T::DbWeight::get().reads_writes(1, 1).ref_time();
                    let __pallet_weight = <dyn frame_support::dispatch::WeighData<
                        (),
                    >>::weigh_data(&__pallet_base_weight, ());
                    let __pallet_class = <dyn frame_support::dispatch::ClassifyDispatch<
                        (),
                    >>::classify_dispatch(&__pallet_base_weight, ());
                    let __pallet_pays_fee = <dyn frame_support::dispatch::PaysFee<
                        (),
                    >>::pays_fee(&__pallet_base_weight, ());
                    frame_support::dispatch::DispatchInfo {
                        weight: __pallet_weight,
                        class: __pallet_class,
                        pays_fee: __pallet_pays_fee,
                    }
                }
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["internal error: entered unreachable code: "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &::core::fmt::Arguments::new_v1(
                                        &["__Ignore cannot be used"],
                                        &[],
                                    ),
                                ),
                            ],
                        ),
                    )
                }
            }
        }
    }
    #[allow(deprecated)]
    impl<T: Config> frame_support::weights::GetDispatchInfo for Call<T> {}
    impl<T: Config> frame_support::dispatch::GetCallName for Call<T> {
        fn get_call_name(&self) -> &'static str {
            match *self {
                Self::do_something { .. } => "do_something",
                Self::cause_error { .. } => "cause_error",
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["internal error: entered unreachable code: "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &::core::fmt::Arguments::new_v1(
                                        &["__PhantomItem cannot be used."],
                                        &[],
                                    ),
                                ),
                            ],
                        ),
                    )
                }
            }
        }
        fn get_call_names() -> &'static [&'static str] {
            &["do_something", "cause_error"]
        }
    }
    impl<T: Config> frame_support::traits::UnfilteredDispatchable for Call<T> {
        type RuntimeOrigin = frame_system::pallet_prelude::OriginFor<T>;
        fn dispatch_bypass_filter(
            self,
            origin: Self::RuntimeOrigin,
        ) -> frame_support::dispatch::DispatchResultWithPostInfo {
            frame_support::dispatch_context::run_in_context(|| {
                match self {
                    Self::do_something { something } => {
                        let __within_span__ = {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "do_something",
                                        "pallet_template::pallet",
                                        ::tracing::Level::TRACE,
                                        Some("pallets/template/src/lib.rs"),
                                        Some(17u32),
                                        Some("pallet_template::pallet"),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::TRACE
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        let __tracing_guard__ = __within_span__.enter();
                        <Pallet<T>>::do_something(origin, something)
                            .map(Into::into)
                            .map_err(Into::into)
                    }
                    Self::cause_error {} => {
                        let __within_span__ = {
                            use ::tracing::__macro_support::Callsite as _;
                            static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                                static META: ::tracing::Metadata<'static> = {
                                    ::tracing_core::metadata::Metadata::new(
                                        "cause_error",
                                        "pallet_template::pallet",
                                        ::tracing::Level::TRACE,
                                        Some("pallets/template/src/lib.rs"),
                                        Some(17u32),
                                        Some("pallet_template::pallet"),
                                        ::tracing_core::field::FieldSet::new(
                                            &[],
                                            ::tracing_core::callsite::Identifier(&CALLSITE),
                                        ),
                                        ::tracing::metadata::Kind::SPAN,
                                    )
                                };
                                ::tracing::callsite::DefaultCallsite::new(&META)
                            };
                            let mut interest = ::tracing::subscriber::Interest::never();
                            if ::tracing::Level::TRACE
                                <= ::tracing::level_filters::STATIC_MAX_LEVEL
                                && ::tracing::Level::TRACE
                                    <= ::tracing::level_filters::LevelFilter::current()
                                && {
                                    interest = CALLSITE.interest();
                                    !interest.is_never()
                                }
                                && ::tracing::__macro_support::__is_enabled(
                                    CALLSITE.metadata(),
                                    interest,
                                )
                            {
                                let meta = CALLSITE.metadata();
                                ::tracing::Span::new(
                                    meta,
                                    &{ meta.fields().value_set(&[]) },
                                )
                            } else {
                                let span = ::tracing::__macro_support::__disabled_span(
                                    CALLSITE.metadata(),
                                );
                                {};
                                span
                            }
                        };
                        let __tracing_guard__ = __within_span__.enter();
                        <Pallet<T>>::cause_error(origin)
                            .map(Into::into)
                            .map_err(Into::into)
                    }
                    Self::__Ignore(_, _) => {
                        let _ = origin;
                        ::core::panicking::panic_fmt(
                            ::core::fmt::Arguments::new_v1(
                                &["internal error: entered unreachable code: "],
                                &[
                                    ::core::fmt::ArgumentV1::new_display(
                                        &::core::fmt::Arguments::new_v1(
                                            &["__PhantomItem cannot be used."],
                                            &[],
                                        ),
                                    ),
                                ],
                            ),
                        );
                    }
                }
            })
        }
    }
    impl<T: Config> frame_support::dispatch::Callable<T> for Pallet<T> {
        type RuntimeCall = Call<T>;
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn call_functions() -> frame_support::metadata::PalletCallMetadata {
            frame_support::scale_info::meta_type::<Call<T>>().into()
        }
    }
    impl<T: Config> frame_support::sp_std::fmt::Debug for Error<T> {
        fn fmt(
            &self,
            f: &mut frame_support::sp_std::fmt::Formatter<'_>,
        ) -> frame_support::sp_std::fmt::Result {
            f.write_str(self.as_str())
        }
    }
    impl<T: Config> Error<T> {
        #[doc(hidden)]
        pub fn as_str(&self) -> &'static str {
            match &self {
                Self::__Ignore(_, _) => {
                    ::core::panicking::panic_fmt(
                        ::core::fmt::Arguments::new_v1(
                            &["internal error: entered unreachable code: "],
                            &[
                                ::core::fmt::ArgumentV1::new_display(
                                    &::core::fmt::Arguments::new_v1(
                                        &["`__Ignore` can never be constructed"],
                                        &[],
                                    ),
                                ),
                            ],
                        ),
                    )
                }
                Self::NoneValue => "NoneValue",
                Self::StorageOverflow => "StorageOverflow",
            }
        }
    }
    impl<T: Config> From<Error<T>> for &'static str {
        fn from(err: Error<T>) -> &'static str {
            err.as_str()
        }
    }
    impl<T: Config> From<Error<T>> for frame_support::sp_runtime::DispatchError {
        fn from(err: Error<T>) -> Self {
            use frame_support::codec::Encode;
            let index = <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::index::<
                Pallet<T>,
            >()
                .expect("Every active module has an index in the runtime; qed") as u8;
            let mut encoded = err.encode();
            encoded.resize(frame_support::MAX_MODULE_ERROR_ENCODED_SIZE, 0);
            frame_support::sp_runtime::DispatchError::Module(frame_support::sp_runtime::ModuleError {
                index,
                error: TryInto::try_into(encoded)
                    .expect(
                        "encoded error is resized to be equal to the maximum encoded error size; qed",
                    ),
                message: Some(err.as_str()),
            })
        }
    }
    pub use __tt_error_token_1 as tt_error_token;
    #[doc(hidden)]
    pub mod __substrate_event_check {
        #[doc(hidden)]
        pub use __is_event_part_defined_2 as is_event_part_defined;
    }
    impl<T: Config> Pallet<T> {
        pub(super) fn deposit_event(event: Event<T>) {
            let event = <<T as Config>::RuntimeEvent as From<Event<T>>>::from(event);
            let event = <<T as Config>::RuntimeEvent as Into<
                <T as frame_system::Config>::RuntimeEvent,
            >>::into(event);
            <frame_system::Pallet<T>>::deposit_event(event)
        }
    }
    impl<T: Config> From<Event<T>> for () {
        fn from(_: Event<T>) {}
    }
    impl<T: Config> Pallet<T> {
        #[doc(hidden)]
        pub fn storage_metadata() -> frame_support::metadata::PalletStorageMetadata {
            frame_support::metadata::PalletStorageMetadata {
                prefix: <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                    Pallet<T>,
                >()
                    .expect(
                        "No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                    ),
                entries: {
                    #[allow(unused_mut)]
                    let mut entries = ::alloc::vec::Vec::new();
                    {
                        <Something<
                            T,
                        > as frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(
                            ::alloc::vec::Vec::new(),
                            &mut entries,
                        );
                    }
                    {
                        <AccountMap<
                            T,
                        > as frame_support::storage::StorageEntryMetadataBuilder>::build_metadata(
                            ::alloc::vec::Vec::new(),
                            &mut entries,
                        );
                    }
                    entries
                },
            }
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn something() -> Option<u32> {
            <Something<T> as frame_support::storage::StorageValue<u32>>::get()
        }
    }
    impl<T: Config> Pallet<T> {
        pub fn accounts<KArg>(k: KArg) -> Option<T::Balance>
        where
            KArg: frame_support::codec::EncodeLike<T::AccountId>,
        {
            <AccountMap<
                T,
            > as frame_support::storage::StorageMap<T::AccountId, T::Balance>>::get(k)
        }
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageSomething<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
    for _GeneratedPrefixForStorageSomething<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
                .expect(
                    "No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                )
        }
        const STORAGE_PREFIX: &'static str = "Something";
    }
    #[doc(hidden)]
    pub struct _GeneratedPrefixForStorageAccountMap<T>(core::marker::PhantomData<(T,)>);
    impl<T: Config> frame_support::traits::StorageInstance
    for _GeneratedPrefixForStorageAccountMap<T> {
        fn pallet_prefix() -> &'static str {
            <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Pallet<T>,
            >()
                .expect(
                    "No name found for the pallet in the runtime! This usually means that the pallet wasn't added to `construct_runtime!`.",
                )
        }
        const STORAGE_PREFIX: &'static str = "AccountMap";
    }
    #[doc(hidden)]
    pub mod __substrate_inherent_check {
        #[doc(hidden)]
        pub use __is_inherent_part_defined_3 as is_inherent_part_defined;
    }
    /// Hidden instance generated to be internally used when module is used without
    /// instance.
    #[doc(hidden)]
    pub type __InherentHiddenInstance = ();
    impl<
        T: Config,
    > frame_support::traits::Hooks<<T as frame_system::Config>::BlockNumber>
    for Pallet<T> {}
    impl<
        T: Config,
    > frame_support::traits::OnFinalize<<T as frame_system::Config>::BlockNumber>
    for Pallet<T> {
        fn on_finalize(n: <T as frame_system::Config>::BlockNumber) {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_finalize",
                            "pallet_template::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/template/src/lib.rs"),
                            Some(17u32),
                            Some("pallet_template::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::on_finalize(n)
        }
    }
    impl<
        T: Config,
    > frame_support::traits::OnIdle<<T as frame_system::Config>::BlockNumber>
    for Pallet<T> {
        fn on_idle(
            n: <T as frame_system::Config>::BlockNumber,
            remaining_weight: frame_support::weights::Weight,
        ) -> frame_support::weights::Weight {
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::on_idle(n, remaining_weight)
        }
    }
    impl<
        T: Config,
    > frame_support::traits::OnInitialize<<T as frame_system::Config>::BlockNumber>
    for Pallet<T> {
        fn on_initialize(
            n: <T as frame_system::Config>::BlockNumber,
        ) -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_initialize",
                            "pallet_template::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/template/src/lib.rs"),
                            Some(17u32),
                            Some("pallet_template::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::on_initialize(n)
        }
    }
    impl<T: Config> frame_support::traits::OnRuntimeUpgrade for Pallet<T> {
        fn on_runtime_upgrade() -> frame_support::weights::Weight {
            let __within_span__ = {
                use ::tracing::__macro_support::Callsite as _;
                static CALLSITE: ::tracing::callsite::DefaultCallsite = {
                    static META: ::tracing::Metadata<'static> = {
                        ::tracing_core::metadata::Metadata::new(
                            "on_runtime_update",
                            "pallet_template::pallet",
                            ::tracing::Level::TRACE,
                            Some("pallets/template/src/lib.rs"),
                            Some(17u32),
                            Some("pallet_template::pallet"),
                            ::tracing_core::field::FieldSet::new(
                                &[],
                                ::tracing_core::callsite::Identifier(&CALLSITE),
                            ),
                            ::tracing::metadata::Kind::SPAN,
                        )
                    };
                    ::tracing::callsite::DefaultCallsite::new(&META)
                };
                let mut interest = ::tracing::subscriber::Interest::never();
                if ::tracing::Level::TRACE <= ::tracing::level_filters::STATIC_MAX_LEVEL
                    && ::tracing::Level::TRACE
                        <= ::tracing::level_filters::LevelFilter::current()
                    && {
                        interest = CALLSITE.interest();
                        !interest.is_never()
                    }
                    && ::tracing::__macro_support::__is_enabled(
                        CALLSITE.metadata(),
                        interest,
                    )
                {
                    let meta = CALLSITE.metadata();
                    ::tracing::Span::new(meta, &{ meta.fields().value_set(&[]) })
                } else {
                    let span = ::tracing::__macro_support::__disabled_span(
                        CALLSITE.metadata(),
                    );
                    {};
                    span
                }
            };
            let __tracing_guard__ = __within_span__.enter();
            let pallet_name = <<T as frame_system::Config>::PalletInfo as frame_support::traits::PalletInfo>::name::<
                Self,
            >()
                .unwrap_or("<unknown pallet name>");
            {
                let lvl = ::log::Level::Debug;
                if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                    ::log::__private_api_log(
                        ::core::fmt::Arguments::new_v1(
                            &["\u{2705} no migration for "],
                            &[::core::fmt::ArgumentV1::new_display(&pallet_name)],
                        ),
                        lvl,
                        &(
                            frame_support::LOG_TARGET,
                            "pallet_template::pallet",
                            "pallets/template/src/lib.rs",
                            17u32,
                        ),
                        ::log::__private_api::Option::None,
                    );
                }
            };
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::on_runtime_upgrade()
        }
    }
    impl<
        T: Config,
    > frame_support::traits::OffchainWorker<<T as frame_system::Config>::BlockNumber>
    for Pallet<T> {
        fn offchain_worker(n: <T as frame_system::Config>::BlockNumber) {
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::offchain_worker(n)
        }
    }
    impl<T: Config> frame_support::traits::IntegrityTest for Pallet<T> {
        fn integrity_test() {
            <Self as frame_support::traits::Hooks<
                <T as frame_system::Config>::BlockNumber,
            >>::integrity_test()
        }
    }
    #[cfg(feature = "std")]
    impl<T: Config> frame_support::sp_runtime::BuildModuleGenesisStorage<T, ()>
    for GenesisConfig<T> {
        fn build_module_genesis_storage(
            &self,
            storage: &mut frame_support::sp_runtime::Storage,
        ) -> std::result::Result<(), std::string::String> {
            frame_support::BasicExternalities::execute_with_storage(
                storage,
                || {
                    <Self as frame_support::traits::GenesisBuild<T>>::build(self);
                    Ok(())
                },
            )
        }
    }
    #[doc(hidden)]
    pub mod __substrate_genesis_config_check {
        #[doc(hidden)]
        pub use __is_genesis_config_defined_4 as is_genesis_config_defined;
        #[doc(hidden)]
        pub use __is_std_macro_defined_for_genesis_4 as is_std_enabled_for_genesis;
    }
    #[doc(hidden)]
    pub mod __substrate_origin_check {
        #[doc(hidden)]
        pub use __is_origin_part_defined_5 as is_origin_part_defined;
    }
    #[doc(hidden)]
    pub mod __substrate_validate_unsigned_check {
        #[doc(hidden)]
        pub use __is_validate_unsigned_part_defined_6 as is_validate_unsigned_part_defined;
    }
    pub use __tt_default_parts_7 as tt_default_parts;
}
