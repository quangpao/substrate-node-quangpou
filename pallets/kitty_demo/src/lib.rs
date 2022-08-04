#![cfg_attr(not(feature = "std"), no_std)]


pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;
use frame_support::inherent::Vec;
use frame_support::dispatch::fmt;
use frame_support::log;
use frame_support::traits::Randomness;
use frame_support::traits::Currency;
use frame_support::traits::UnixTime;
// use frame_support::traits::StorageInstance;
type BalanceOf<T> = <<T as Config>::KittyCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {

	pub use super::*;

	#[derive(Clone, Encode, Decode, PartialEq, TypeInfo)]
	#[scale_info(skip_type_params(T))]
	pub struct Kitties<T: Config> {
		id: Id,
		dna: Vec<u8>,
		owner: T::AccountId,
		price: BalanceOf<T>,
		gender: Gender,
		create_date: u64,
	}

	impl<T: Config> fmt::Debug for Kitties<T>{
		fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
			f.debug_struct("Kitty")
				.field("id", &self.id)
				.field("dna", &self.dna)
				.field("owner", &self.owner)
				.field("price", &self.price)
				.field("gender", &self.gender)
				.field("create date", &self.create_date)
				.finish()
		}
	}


	pub type Id = u32;

	#[derive(Clone, Encode, Decode, PartialEq, Copy, TypeInfo, MaxEncodedLen, Debug)]
	pub enum Gender {
		Male,
		Female,
	}

	impl Default for Gender{
		fn default() -> Self {
			Gender::Male
		}
	}


	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config{
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type KittyCurrency: Currency<Self::AccountId>;
		type TimeProvider: UnixTime;
		type MyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
		#[pallet::constant]
		type MaxAddend: Get<u32>;
		// type ClearFrequency: Get<Self::BlockNumber>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);


	//
	//
	//
	#[pallet::storage]
	#[pallet::getter(fn kitty_number)]
	pub type KittyNumber<T> = StorageValue<_, Id, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn nonce_number)]
	pub type Nonce<T> = StorageValue<_, u32, ValueQuery>;



	#[pallet::storage]
	#[pallet::getter(fn kitty)]
	pub(super) type Kitty<T: Config> = StorageMap<_, Blake2_128Concat, Id, Kitties<T>, OptionQuery>;


	#[pallet::storage]
	#[pallet::getter(fn kitty_owner)]
	pub(super) type KittyOwner<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Kitties<T>>, OptionQuery>;

	//
	

	//

	//
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]	
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		KittyStore(Vec<u8>, u32),
		KittyTransfer(Id, T::AccountId),

	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		/// Errors should have helpful documentation associated with them.
		TooManyKitties,
		KittyNotExist,
		// KittyAlreadyExist,
		NotOwner,

	}

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config>{
		pub kitties: Vec<(T::AccountId, Vec<u8>)>,
		pub time: u64,
	}

	#[cfg(feature = "std")]
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> GenesisConfig<T> {
			GenesisConfig {
				kitties: Vec::new(),
				time: 0u64,
			}
		}

	}

	#[pallet::genesis_build]
	impl<T:Config> GenesisBuild<T> for GenesisConfig<T>{
		fn build(&self) {
			KittyNumber::<T>::put(self.kitties.len() as Id);
			let mut i = 0;
			for(owner, dna) in self.kitties.iter(){
				i+= 1;
				let gender = Pallet::<T>::gen_gender(dna.clone()).unwrap();
				let kitty = Kitties::<T>{
					id: i,
					dna: dna.clone(),
					owner: owner.clone(),
					price: 0u32.into(),
					gender: gender,
					create_date: T::TimeProvider::now().as_secs(),
				};
				<Kitty<T>>::insert(i, kitty);
				let kitty_owner = <KittyOwner<T>>::get(owner.clone());
				let mut kitty_owner = match kitty_owner {
					Some(k) => k,
					None => Vec::new(),
				};
				let kitty = <Kitty<T>>::get(i);
				kitty_owner.push(kitty.unwrap());
				<KittyOwner<T>>::insert(owner.clone(), kitty_owner);
			}
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {


		#[pallet::weight(50_700_000 + T::DbWeight::get().reads_writes(3, 2))]
		pub fn create_kitty(origin: OriginFor<T>, price: u32) -> DispatchResultWithPostInfo {

			let who = ensure_signed(origin)?;
			
			//Check if the kitties are full or not
			let kitty_owner = <KittyOwner<T>>::get(who.clone());
			let mut kitty_owner = match kitty_owner {
				Some(k) => k,
				None => Vec::new(),
			};
			ensure!(kitty_owner.len() < <T as Config>::MaxAddend::get().try_into().unwrap(), Error::<T>::TooManyKitties);

			//check the total balance of who for debug
			log::info!("total_balance: {:?}", T::KittyCurrency::total_balance(&who));

			let nonce = Self::get_and_increase_nonce();
			let (randomValue, _) = T::MyRandomness::random(&nonce);
			let dna = randomValue.as_ref().to_vec();

			let current_id = <KittyNumber<T>>::get() + 1;
			let gender = Self:: gen_gender(dna.clone()).unwrap();


			let kitty = Kitties {
				id: current_id,
				dna: dna.clone(),
				owner: who.clone(),
				price: price.into(),
				gender: gender,
				create_date: T::TimeProvider::now().as_secs(),
			};
			log::info!("Kitty : {:?}", &kitty);
			<Kitty<T>>::insert(current_id, kitty);
			<KittyNumber<T>>::put(current_id);


			let kitty = <Kitty<T>>::get(current_id);
			kitty_owner.push(kitty.unwrap());
			<KittyOwner<T>>::insert(who.clone(), kitty_owner);
			Self::deposit_event(Event::KittyStore(dna, 0u32.into()));

			Ok(().into())


		}

		

		#[pallet::weight(21_900_000 + T::DbWeight::get().reads_writes(3, 2))]
		pub fn change_owner(origin: OriginFor<T>, kitty_id: Id, new_owner: T::AccountId) -> DispatchResult {
			let sender = ensure_signed(origin)?;
			let kitty = <Kitty<T>>::get(kitty_id);
			ensure!(kitty.is_some(), Error::<T>::KittyNotExist);
			let mut kitty = kitty.unwrap();
			ensure!(kitty.owner == sender, Error::<T>::NotOwner);
			let kitty_owner = <KittyOwner<T>>::get(sender.clone());
			let mut kitty_owner = match kitty_owner{
				Some(k) => k,
				None => Vec::new(),
			};

			// Hàm retain dùng để giữ lại những giá trị thoả điều kiệu -> Loại bỏ giá trị không thoả
			kitty_owner.retain(|k| k.id != kitty.id.clone());
			kitty.owner = new_owner.clone();
			<KittyOwner<T>>::insert(sender.clone(), kitty_owner);
			let kitty_owner = <KittyOwner<T>>::get(new_owner.clone());
			let mut kitty_owner = match kitty_owner{
				Some(k) => k,
				None => Vec::new(),
			};
			ensure!(kitty_owner.len() < <T as Config>::MaxAddend::get().try_into().unwrap(), Error::<T>::TooManyKitties);
			kitty_owner.push(kitty);
			<KittyOwner<T>>::insert(new_owner.clone(), kitty_owner);
			Self::deposit_event(Event::KittyTransfer(kitty_id, new_owner));
			Ok(())
		}


		



	}


	
}

impl<T: Config> Pallet<T> {
	fn gen_gender(name: Vec<u8>) -> Result<Gender, Error<T>>{
		let mut res = Gender::Male;
		if name.len() % 2 == 1 {
			res = Gender::Female;
		}
		Ok(res)
	}

	fn get_and_increase_nonce() -> Vec<u8> {
		let nonce = Nonce::<T>::get();
		Nonce::<T>::put(nonce + 1);
		nonce.encode()
	}
}

