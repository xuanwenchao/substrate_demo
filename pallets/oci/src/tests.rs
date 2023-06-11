use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_set_on_chain_data() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		
		let data = vec![1, 2, 3];
		assert_ok!(OCIModule::set_on_chain_data(RuntimeOrigin::signed(1), data.clone()));

		// Assert that the correct event was deposited
		System::assert_last_event(RuntimeEvent::OCIModule(Event::OffChainIndexWriteSuccessful(data, 1)));

		assert_noop!(
			OCIModule::set_on_chain_data(RuntimeOrigin::signed(1), [].into()),
			Error::<Test>::InvalidData
		);

	});
}

