use super::*;
use crate::mock::{new_test_ext, Kitties};

#[test]
fn owned_kitties_can_append_values() {
	new_test_ext().execute_with(|| {
		mock::run_to_block(5);
		assert_eq!(Kitties::create(mock::Origin::signed(1),),Ok(()));
	});
}

