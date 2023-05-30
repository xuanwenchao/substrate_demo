use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_eq!(KittyModule::next_kitty_id(), kitty_id);
		assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittyModule::next_kitty_id(), kitty_id + 1);
		assert_eq!(KittyModule::kitties(kitty_id).is_some(), true);

		assert_eq!(KittyModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittyModule::kitty_parents(kitty_id), None);

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());

		assert_noop!(
			KittyModule::create(RuntimeOrigin::signed(account_id)),
			Error::<Test>::InvalidKittyID
		);


		//To check if a custom event has been successfully sent, 
        //we can use the System::events() function to get the list of events that have been emitted.
        let kitty = crate::Kitty(Default::default());
        let expected_event = RuntimeEvent::KittyModule(crate::Event::KittyCreated { who: (account_id), kitty_id: (kitty_id), kitty:kitty  });
		let events = System::events();
        let last_event = events.last().unwrap_or_else(|| unreachable!("Expected an event"));
        assert_eq!(
            last_event.event,
            expected_event
        );

    });
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;

		assert_noop!(
			KittyModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id),
			Error::<Test>::SameKittyId
		);

		assert_noop!(
			KittyModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1),
			Error::<Test>::InvalidKittyID
		);

		assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id)));
		assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id)));

		assert_eq!(KittyModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(KittyModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1));

		let breed_kitty_id = kitty_id + 2;
		assert_eq!(KittyModule::next_kitty_id(), breed_kitty_id + 1);
		assert_eq!(KittyModule::kitties(breed_kitty_id).is_some(), true);
		assert_eq!(KittyModule::kitty_owner(breed_kitty_id), Some(account_id));
		assert_eq!(KittyModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));

        //To check if a custom event has been successfully sent, 
        //we can use the System::events() function to get the list of events that have been emitted.
        let breed_kitty = KittyModule::kitties(breed_kitty_id).unwrap();
        let expected_event = RuntimeEvent::KittyModule(crate::Event::KittyBreeded { who: account_id, kitty_id: (breed_kitty_id), kitty:breed_kitty  });
		let events = System::events();
        let last_event = events.last().unwrap_or_else(|| unreachable!("Expected an event"));
        assert_eq!(
            last_event.event,
            expected_event
        );
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let recipient = 2;

		assert_ok!(KittyModule::create(RuntimeOrigin::signed(account_id)));
		assert_eq!(KittyModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(
			KittyModule::transfer(RuntimeOrigin::signed(recipient), recipient, kitty_id),
			Error::<Test>::NotKittyOwner
		);

		assert_ok!(KittyModule::transfer(RuntimeOrigin::signed(account_id), recipient, kitty_id));
		assert_eq!(KittyModule::kitty_owner(kitty_id), Some(recipient));
		assert_ok!(KittyModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id));
		assert_eq!(KittyModule::kitty_owner(kitty_id), Some(account_id));

        //To check if a custom event has been successfully sent, 
        //we can use the System::events() function to get the list of events that have been emitted.
        let expected_event = RuntimeEvent::KittyModule(crate::Event::KittyTransfered { who: recipient, recipient: account_id, kitty_id:kitty_id  });
		let events = System::events();
        let last_event = events.last().unwrap_or_else(|| unreachable!("Expected an event"));
        assert_eq!(
            last_event.event,
            expected_event
        );
	});
}
