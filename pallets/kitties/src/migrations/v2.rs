use frame_support::{
	migration::storage_key_iter, pallet_prelude::*, storage::StoragePrefixedMap,
	traits::GetStorageVersion, weights::Weight, Blake2_128Concat,
};

use crate::{Config, Kitties, Kitty, KittyId, Pallet};

#[derive(Encode, Decode, Clone, Debug, TypeInfo, MaxEncodedLen, PartialEq, Eq)]
pub struct OldKitty0(pub [u8; 16]);

#[derive(Encode, Decode, Clone, Debug, TypeInfo, MaxEncodedLen, PartialEq, Eq)]
pub struct OldKitty1 {
	pub dna: [u8; 16],
	pub name: [u8; 4],
}

pub fn migrate<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();
	log::info!(
		"### on_chain_version={:?}, current_version={:?}",
		on_chain_version,
		current_version
	);

	if on_chain_version == 0 && current_version == 2 {
		migrate_from_v0_to_v2::<T>();
	} else if on_chain_version == 1 && current_version == 2 {
		migrate_from_v1_to_v2::<T>();
	} else {
		return Weight::zero()
	}

	Weight::zero()
}

fn migrate_from_v0_to_v2<T: Config>() {
	// 版本 0 到版本 2 的迁移逻辑
	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (index, kitty) in
		storage_key_iter::<KittyId, OldKitty0, Blake2_128Concat>(module, item).drain()
	{
		let new_kitty = Kitty { dna: kitty.0, name: *b"abcdefgh" };

		Kitties::<T>::insert(index, &new_kitty);
	}
}

fn migrate_from_v1_to_v2<T: Config>() {
	// 版本 1 到版本 2 的迁移逻辑
	let module = Kitties::<T>::module_prefix();
	let item = Kitties::<T>::storage_prefix();

	for (index, old_kitty) in
		storage_key_iter::<KittyId, OldKitty1, Blake2_128Concat>(module, item).drain()
	{
		let new_kitty = Kitty {
			dna: old_kitty.dna,
			name: {
                let mut name = [0u8; 8];
                name[..4].copy_from_slice(&old_kitty.name[..4]);
                name[4..].copy_from_slice(b"efgh");
                name
            },
		};

		Kitties::<T>::insert(index, &new_kitty);
	}
}
