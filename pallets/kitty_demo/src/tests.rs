use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
// use crate::Kitty;
// use frame_system::Origin;

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
        System::set_block_number(10);
		assert_ok!(Kitty::create_kitty(Origin::signed(1)));
        assert_ok!(Kitty::create_kitty(Origin::signed(1)));
        assert_ok!(Kitty::create_kitty(Origin::signed(1)));
        assert_ok!(Kitty::create_kitty(Origin::signed(1)));
        assert_ok!(Kitty::create_kitty(Origin::signed(1)));
        assert_noop!(Kitty::create_kitty(Origin::signed(1)), Error::<Test>::StorageOverflow);

	});
}

// #[test]
// fn correct_error_for_none_value() {
// 	new_test_ext().execute_with(|| {
// 		// Ensure the expected error is thrown when no value is present.
// 		assert_noop!(TemplateModule::cause_error(Origin::signed(1)), Error::<Test>::NoneValue);
// 	});
// }
