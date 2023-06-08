use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

mod create_kitty_test {
	use super::*;

	#[test]
	fn it_works_for_create() {
		new_test_ext().execute_with(|| {
			let kitty_id = 0;
			let account_id = 1;

			assert_eq!(KittyModule::next_kitty_id(), kitty_id);
			assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id), *b"alex_123"));

			assert_eq!(KittyModule::next_kitty_id(), kitty_id + 1);
			assert_eq!(KittyModule::kitties(kitty_id).is_some(), true);

			assert_eq!(KittyModule::kitty_owner(kitty_id), Some(account_id));
			assert_eq!(KittyModule::kitty_parents(kitty_id), None);

			//To check if a custom event has been successfully sent,
			//we can use the System::events() function to get the list of events that have been
			// emitted.
			let kitty = KittyModule::kitties(kitty_id).unwrap();

			let expected_event = RuntimeEvent::KittyModule(crate::Event::KittyCreated {
				who: (account_id),
				kitty_id: (kitty_id),
				kitty,
			});

			System::assert_last_event(expected_event.into());
		});
	}

	#[test]
	fn failed_when_create() {
		new_test_ext().execute_with(|| {
			let account_id = 1;
			crate::NextKittyId::<Test>::set(crate::KittyId::max_value());

			assert_noop!(
				KittyModule::create(RuntimeOrigin::signed(account_id), *b"alex_123"),
				Error::<Test>::InvalidKittyID
			);
		});
	}
}

mod breed_kitty_test {
	use super::*;

	#[test]
	fn it_works_for_breed() {
		new_test_ext().execute_with(|| {
			let kitty_id = 0;
			let account_id = 1;

			assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id), *b"alex_123"));
			assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id), *b"alex_123"));

			assert_eq!(KittyModule::next_kitty_id(), kitty_id + 2);

			assert_ok!(KittyModule::breed(
				RuntimeOrigin::signed(account_id),
				kitty_id,
				kitty_id + 1,
				*b"alex_123"
			));

			let breed_kitty_id = kitty_id + 2;
			assert_eq!(KittyModule::next_kitty_id(), breed_kitty_id + 1);
			assert_eq!(KittyModule::kitties(breed_kitty_id).is_some(), true);
			assert_eq!(KittyModule::kitty_owner(breed_kitty_id), Some(account_id));
			assert_eq!(KittyModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));

			//To check if a custom event has been successfully sent,
			//we can use the System::events() function to get the list of events that have been
			// emitted.
			let breed_kitty = KittyModule::kitties(breed_kitty_id).unwrap();
			let expected_event = RuntimeEvent::KittyModule(crate::Event::KittyBreeded {
				who: account_id,
				kitty_id: (breed_kitty_id),
				kitty: breed_kitty,
			});
			System::assert_last_event(expected_event.into());
		});
	}

	#[test]
	fn failed_when_breed() {
		new_test_ext().execute_with(|| {
			let kitty_id = 0;
			let account_id = 1;

			assert_noop!(
				KittyModule::breed(
					RuntimeOrigin::signed(account_id),
					kitty_id,
					kitty_id,
					*b"alex_123"
				),
				Error::<Test>::SameKittyId
			);

			assert_noop!(
				KittyModule::breed(
					RuntimeOrigin::signed(account_id),
					kitty_id,
					kitty_id + 1,
					*b"alex_123"
				),
				Error::<Test>::InvalidKittyID
			);
		});
	}
}

mod transfer_kitty_test {
	use super::*;

	#[test]
	fn it_works_for_transfer() {
		new_test_ext().execute_with(|| {
			let kitty_id = 0;
			let account_id = 1;
			let recipient = 2;

			assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id), *b"alex_123"));
			assert_eq!(KittyModule::kitty_owner(kitty_id), Some(account_id));

			assert_ok!(KittyModule::transfer(
				RuntimeOrigin::signed(account_id),
				recipient,
				kitty_id
			));
			assert_eq!(KittyModule::kitty_owner(kitty_id), Some(recipient));
			assert_ok!(KittyModule::transfer(
				RuntimeOrigin::signed(recipient),
				account_id,
				kitty_id
			));
			assert_eq!(KittyModule::kitty_owner(kitty_id), Some(account_id));

			//To check if a custom event has been successfully sent,
			//we can use the System::events() function to get the list of events that have been
			// emitted.
			let expected_event = RuntimeEvent::KittyModule(crate::Event::KittyTransfered {
				who: recipient,
				recipient: account_id,
				kitty_id,
			});
			System::assert_last_event(expected_event.into());
		});
	}

	#[test]
	fn failed_when_transfer() {
		new_test_ext().execute_with(|| {
			let kitty_id = 0;
			let account_id = 1;
			let recipient = 2;
			assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id), *b"alex_123"));

			assert_noop!(
				KittyModule::transfer(RuntimeOrigin::signed(recipient), recipient, kitty_id),
				Error::<Test>::NotKittyOwner
			);
		});
	}
}

mod sale_kitty_test {
	use super::*;

	#[test]
	fn it_works_for_sale() {
		new_test_ext().execute_with(|| {
			let kitty_id = 0;
			let account_id = 1;
			assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id), *b"alex_123"));
			assert_eq!(KittyModule::kitty_owner(kitty_id), Some(account_id));

			assert_ok!(KittyModule::sale(RuntimeOrigin::signed(account_id), kitty_id));

			assert_eq!(KittyModule::kitty_on_sale(kitty_id).is_some(), true);

			//To check if a custom event has been successfully sent,
			//we can use the System::events() function to get the list of events that have been
			// emitted.
			let expected_event =
				RuntimeEvent::KittyModule(crate::Event::KittyOnSale { who: account_id, kitty_id });
			System::assert_last_event(expected_event.into());
		});
	}

	#[test]
	fn failed_when_sale() {
		new_test_ext().execute_with(|| {
			let kitty_id = 0;
			let account_id = 1;
			let recipient = 2;
			assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id), *b"alex_123"));
			assert_ok!(KittyModule::sale(RuntimeOrigin::signed(account_id), kitty_id));
			assert_noop!(
				KittyModule::sale(RuntimeOrigin::signed(recipient), kitty_id),
				Error::<Test>::NotKittyOwner
			);

			assert_noop!(
				KittyModule::sale(RuntimeOrigin::signed(account_id), kitty_id+1),
				Error::<Test>::InvalidKittyID
			);

			assert_noop!(
				KittyModule::sale(RuntimeOrigin::signed(account_id), kitty_id),
				Error::<Test>::AlreadyOnSale
			);
		});
	}
}

mod buy_kitty_test {
	use super::*;

	#[test]
	fn it_works_for_buy() {
		new_test_ext().execute_with(|| {
			let kitty_id = 0;
			let account_id = 1;
			let buyer_id = 2;

			assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id), *b"alex_123"));
			assert_eq!(KittyModule::kitty_owner(kitty_id), Some(account_id));

			assert_ok!(KittyModule::sale(RuntimeOrigin::signed(account_id), kitty_id));

			assert_eq!(KittyModule::kitty_on_sale(kitty_id).is_some(), true);

			assert_ok!(KittyModule::buy(RuntimeOrigin::signed(buyer_id), kitty_id));

			assert_eq!(KittyModule::kitty_owner(kitty_id), Some(buyer_id));
			assert_eq!(KittyModule::kitty_on_sale(kitty_id).is_none(), true);

			//To check if a custom event has been successfully sent,
			//we can use the System::events() function to get the list of events that have been
			// emitted.
			let expected_event =
				RuntimeEvent::KittyModule(crate::Event::KittyBought { who: buyer_id, kitty_id });
			System::assert_last_event(expected_event.into());
		});
	}

	#[test]
	fn failed_when_buy() {
		new_test_ext().execute_with(|| {
			let kitty_id = 0;
			let account_id = 1;
			let buyer_id = 2;
			assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id), *b"alex_123"));
			assert_noop!(
				KittyModule::buy(RuntimeOrigin::signed(buyer_id), kitty_id+1),
				Error::<Test>::InvalidKittyID
			);
			assert_noop!(
				KittyModule::buy(RuntimeOrigin::signed(account_id), kitty_id),
				Error::<Test>::AlreadyOwned
			);
            assert_noop!(
				KittyModule::buy(RuntimeOrigin::signed(buyer_id), kitty_id),
				Error::<Test>::NotOnSale
			);
		});
	}
}
