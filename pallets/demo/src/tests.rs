use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		assert_ok!(Demo::create_student(Origin::signed(1), "John", 21));
		assert_noop!(Demo::create_student(Origin::signed(1), "Jane", 20), Error::<Test>::TooYoung);
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
			
	
	});
}
