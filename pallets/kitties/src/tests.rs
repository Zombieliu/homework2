// use super::*;
// use crate::mock::{new_test_ext, Kitties};
use crate::{mock::{new_test_ext,run_to_block,Test,Kitties}, Error, Event};
use crate::*;
use frame_support::{assert_noop, assert_ok};
use frame_system::EventRecord;
use frame_system::Phase;
// use frame_system::Origin;
use crate::mock::{TestEvent,System,Origin};
// #[test]
// fn owned_kitties_can_append_values() {
// 	new_test_ext().execute_with(|| {
// 		mock::run_to_block(5);
// 		assert_eq!(Kitties::create(mock::Origin::signed(1),),Ok(()));
// 	});
// }

#[test]
fn owner_kitties_can_append_values() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_ok!(Kitties::create(Origin::signed(1)));

		assert_eq!(
			System::events()[1].event,
			// vec![EventRecord {
			//     phase: Phase::Initialization,
			//     event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 0)),
			//     topics: vec![],
			// }]
			TestEvent::kitties_event(Event::<Test>::Created(1u64, 0))
		)
	})
}
#[test]
fn owner_kitties_failed_when_no_enought_money() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		assert_noop!(Kitties::create(Origin::signed(9)), Error::<Test>::NotEnoughMoney);
	})
}

// 测试转让Kitty成功
#[test]
fn transfer_kitty_works() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		let _ = Kitties::create(Origin::signed(1));

		assert_ok!(Kitties::transfer(Origin::signed(1), 2, 0));

		assert_eq!(
			System::events(),
			vec![
				// EventRecord {
				//     phase: Phase::Initialization,
				//     event: TestEvent::kitties_event(Event::<Test>::Reserved(1u64, 5000)),
				//     topics: vec![],
				// },
				EventRecord {
					phase: Phase::Initialization,
					event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 0)),
					topics: vec![],
				},
				EventRecord {
					phase: Phase::Initialization,
					event: TestEvent::kitties_event(Event::<Test>::Transferred(1u64, 2, 0)),
					topics: vec![],
				}
			]
		)
	})
}

// 测试转让Kitty 失败，因为Kitty 不存在
#[test]
fn transfer_kitty_failed_when_no_exists() {
	new_test_ext().execute_with(|| {
		// run_to_block(10);
		// let _ = Kitties::create(Origin::signed(1));

		// assert_ok!(Kitties::transfer(Origin::signed(1), 2, 0));
		assert_noop!(
            Kitties::transfer(Origin::signed(1), 2, 0),
            Error::<Test>::KittyNotExists
        );
	})
}

#[test]
fn transfer_kitty_failed_when_not_owner() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		let _ = Kitties::create(Origin::signed(1));

		assert_noop!(
            Kitties::transfer(Origin::signed(2), 3, 0),
            Error::<Test>::NotKittyOwner
        );
	})
}

// #[test]
// fn transfer_kitty_failed_when_transfer_self() {
// 	new_test_ext().execute_with(|| {
// 		run_to_block(10);
// 		let _ = Kitties::create(Origin::signed(1));
//
// 		assert_noop!(
//             Kitties::transfer(Origin::signed(1), 1, 0),
//             Error::<Test>::TransferToSelf
//         );
// 	})
// }

#[test]
fn breed_kitty_work() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		let _ = Kitties::create(Origin::signed(1));
		let _ = Kitties::create(Origin::signed(1));

		assert_ok!(Kitties::breed(Origin::signed(1), 0, 1));

		assert_eq!(
			System::events(),
			vec![
				// EventRecord {
				//     phase: Phase::Initialization,
				//     event: TestEvent::kitties_event(Event::<Test>::Reserved(1u64, 5000)),
				//     topics: vec![],
				// },
				EventRecord {
					phase: Phase::Initialization,
					event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 0)),
					topics: vec![],
				},
				// EventRecord {
				//     phase: Phase::Initialization,
				//     event: TestEvent::kitties_event(Event::<Test>::Reserved(1u64, 5000)),
				//     topics: vec![],
				// },
				EventRecord {
					phase: Phase::Initialization,
					event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 1)),
					topics: vec![],
				},
				// EventRecord {
				//     phase: Phase::Initialization,
				//     event: TestEvent::kitties_event(Event::<Test>::Reserved(1u64, 5000)),
				//     topics: vec![],
				// },
				EventRecord {
					phase: Phase::Initialization,
					event: TestEvent::kitties_event(Event::<Test>::Created(1u64, 2)),
					topics: vec![],
				}
			]
		)
	})
}

#[test]
fn breed_kitty_fail_when_same() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		let _ = Kitties::create(Origin::signed(1));

		assert_noop!(
            Kitties::breed(Origin::signed(1), 0, 0),
            Error::<Test>::RequireDifferentParent
        );
	})
}

#[test]
fn breed_kitty_fail_when_not_exists() {
	new_test_ext().execute_with(|| {
		assert_noop!(
            Kitties::breed(Origin::signed(1), 0, 1),
            Error::<Test>::KittyNotExists
        );
	})
}

#[test]
fn breed_kitty_work_when_not_owner() {
	new_test_ext().execute_with(|| {
		run_to_block(10);
		let _ = Kitties::create(Origin::signed(1));
		let _ = Kitties::create(Origin::signed(1));

		assert_noop!(
            Kitties::breed(Origin::signed(2), 0, 1),
            Error::<Test>::NotKittyOwner
        );
	})
}