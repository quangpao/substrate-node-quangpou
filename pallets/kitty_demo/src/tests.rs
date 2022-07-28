use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
// use crate::Kitty;
// use frame_system::Origin;
use crate::KittyNumber;
use frame_system::Config;
use frame_benchmarking::account;
#[test]
fn case1() {
	new_test_ext().execute_with(|| {
        System::set_block_number(10);
		assert_ok!(Kitty::create_kitty(Origin::signed(1), 500));
        assert_ok!(Kitty::create_kitty(Origin::signed(1), 300));
        assert_ok!(Kitty::create_kitty(Origin::signed(1), 200));
        assert_ok!(Kitty::create_kitty(Origin::signed(1), 100));
        assert_ok!(Kitty::create_kitty(Origin::signed(1), 604));
        assert_noop!(Kitty::create_kitty(Origin::signed(1), 300), Error::<Test>::TooManyKitties);

	});
}

#[test]
fn case2() {
	new_test_ext().execute_with(|| {
        System::set_block_number(10);
		assert_ok!(Kitty::create_kitty(Origin::signed(1), 500));
        assert_ok!(Kitty::create_kitty(Origin::signed(1), 300));
        assert_eq!(Kitty::kitty_number(), 2);

	});
}
#[test]
fn case3() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
		assert_ok!(Kitty::create_kitty(Origin::signed(1), 500));
        // let receiver: T::AccountId = account("receiver", 0, 0);
        let kitty_id = Kitty::kitty_number();
        assert_noop!(Kitty::change_owner(Origin::signed(2), kitty_id, account("receiver", 0, 0)), Error::<Test>::NotOwner);


	});
}

#[test]
fn case4() {
    new_test_ext().execute_with(|| {
        System::set_block_number(10);
		assert_ok!(Kitty::create_kitty(Origin::signed(1), 500));
        // let receiver: T::AccountId = account("receiver", 0, 0);
        let kitty_id = Kitty::kitty_number();
        assert_noop!(Kitty::change_owner(Origin::signed(2), 50, account("receiver", 0, 0)), Error::<Test>::KittyNotExist);


	});
}
