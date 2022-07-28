//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as Kitties;
use frame_benchmarking::{benchmarks, whitelisted_caller, account};
use frame_system::RawOrigin;

benchmarks!{

	create_kitty {
		let caller: T::AccountId = whitelisted_caller();
		let price = 50;
	}: create_kitty(RawOrigin::Signed(caller), price)

	verify {
		assert_eq!(KittyNumber::<T>::get(), 1);
	}

	change_owner {
		let caller: T::AccountId = whitelisted_caller();
		let price= 50;
		let caller_origin = <T as frame_system::Config>::Origin::from(RawOrigin::Signed(caller.clone()));
		Kitties::<T>::create_kitty(caller_origin, price);
		let kitty_id = KittyNumber::<T>::get();
		let receiver: T::AccountId = account("receiver", 0, 0);
		
	}: change_owner(RawOrigin::Signed(caller), kitty_id, receiver.clone())

	verify {
		let kitties = <KittyOwner<T>>::get(receiver.clone());
		assert_eq!(kitties.unwrap().len(), 1);
	}

	impl_benchmark_test_suite!(Kitties, crate::mock::new_test_ext(), crate::mock::Test);


	
}
