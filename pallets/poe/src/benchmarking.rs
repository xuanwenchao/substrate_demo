use crate::*;
use frame_benchmarking::v1::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::vec;

benchmarks! {
	create_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim =  BoundedVec::try_from(vec![0; d as usize]).unwrap();
		let caller:T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Signed(caller), claim.clone())

	revoke_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim =  BoundedVec::try_from(vec![0; d as usize]).unwrap();
		let caller:T::AccountId = whitelisted_caller();
		assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());
	}: _(RawOrigin::Signed(caller), claim.clone())

	transfer_claim {
		let d in 0 .. T::MaxClaimLength::get();
		let claim = BoundedVec::try_from(vec![0; d as usize]).unwrap();
		let caller: T::AccountId = whitelisted_caller();
		assert!(Pallet::<T>::create_claim(RawOrigin::Signed(caller.clone()).into(), claim.clone()).is_ok());
		let dest:T::AccountId = account("dest", 0, 0);
	}: _(RawOrigin::Signed(caller), claim.clone(), dest.clone())

	impl_benchmark_test_suite!(PoeModule, crate::mock::new_test_ext(), crate::mock::Test);
}
