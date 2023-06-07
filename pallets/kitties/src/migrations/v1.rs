// use frame_support::{
//     pallet_prelude::*,
//     storage::StoragePrefixedMap,
//     traits::GetStorageVersion,
//     weights::Weight,
//     migration::storage_key_iter,
//     Blake2_128Concat,
// };

// use crate::{Config, Pallet, Kitty, KittyId, Kitties};


// #[derive(Encode, Decode, Clone, Debug, TypeInfo, MaxEncodedLen, PartialEq, Eq)]
// pub struct OldKitty(pub [u8; 16]);

// pub fn migrate<T: Config>() -> Weight {
//     let on_chain_storage_version = Pallet::<T>::on_chain_storage_version();
//     let current_version: StorageVersion = Pallet::<T>::current_storage_version();
// 	log::info!("### on_chain_storage_version={:?}, current_version={:?}", on_chain_storage_version, current_version);

//     if on_chain_storage_version != 0 {
//         return Weight::zero();
//     }

//     if current_version != 1 {
//         return Weight::zero();
//     }

//     let module = Kitties::<T>::module_prefix();
//     let item = Kitties::<T>::storage_prefix();

//     for(index,kitty) in storage_key_iter::<KittyId, OldKitty,Blake2_128Concat>(module, item).drain(){
  
//         let new_kitty = Kitty{
//             dna: kitty.0,
//             name: *b"abcd",
//         };

//         Kitties::<T>::insert(index, &new_kitty);
//     }
//     Weight::zero()
// }