use crate as pallet_kitty;
use frame_support::traits::{ConstU16, ConstU64, ConstU32, ConstU128};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, IdentityLookup},
};




type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Kitty: pallet_kitty::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage},
		Balance: pallet_balances::{Pallet, Call, Storage, Event<T>, Config<T>},
		RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet},
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type Origin = Origin;
	type Call = Call;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type Event = Event;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u128>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

impl pallet_kitty::Config for Test {
	type Event = Event;
	type KittyCurrency = Balance;
	type TimeProvider = Timestamp;
	type MaxAddend = ConstU32<5>;
	type MyRandomness = RandomnessCollectiveFlip;
}

impl pallet_timestamp::Config for Test {
	type Moment = u64;
	type OnTimestampSet = ();
	type MinimumPeriod = ();
	type WeightInfo = ();
}

impl pallet_balances::Config for Test {
	type Event = Event;
	type Balance = u128;
	type DustRemoval = ();
	type MaxReserves = ();
	type MaxLocks = ConstU32<50>;
	type WeightInfo = ();
	type ReserveIdentifier = [u8; 8];
	type ExistentialDeposit = ConstU128<500>;
	type AccountStore = System;
}

impl pallet_randomness_collective_flip::Config for Test {

}


// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}
