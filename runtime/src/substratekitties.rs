/// A runtime module template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references


/// For more guidance on Substrate modules, see the example module
/// https://github.com/paritytech/substrate/blob/gav-template/srml/example/src/lib.rs

use parity_codec::Encode;
use support::{decl_module, decl_storage, decl_event, ensure, StorageMap, StorageValue, dispatch::Result};
use { balances, system::ensure_signed };
use parity_codec_derive::{Encode, Decode};
use runtime_primitives::traits::{As, Hash};

/// The module's configuration trait.
pub trait Trait: balances::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

#[derive(Encode, Decode, Default, Clone, PartialEq)]
pub struct Kitty<Hash, Balance> {
	id: Hash,
	dna: Hash,
	price: Balance,
	gen: u64
}

/// This module's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as KittyStorage {
		pub Kitties get(kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
		pub KittyOwner get(kitty_owner): map T::Hash => T::AccountId;
		pub OwnedKitty get(owned_kitty): map T::AccountId => Kitty<T::Hash, T::Balance>;
		pub Nonce get(nonce): u64;
	}
}

decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing events
		// this is needed only if you are using events in your module
		fn deposit_event<T>() = default;

		fn create_kitty(origin) -> Result {
		   let sender = ensure_signed(origin)?;
		   let nonce = <Nonce<T>>::get();
		   let random_seed = <system::Module<T>>::random_seed();
		   let random_hash = (random_seed, &sender, nonce).using_encoded(<T as system::Trait>::Hashing::hash);
		   let initial_price = <T::Balance as As<u64>>::sa(0);

		   ensure!(!<Kitties<T>>::exists(random_hash), "This new id already exists.");

		   let new_kitty = Kitty {
			   id: random_hash,
			   dna: random_hash,
			   price: initial_price,
			   gen: 0,
		   };

		   <Kitties<T>>::insert(&random_hash, &new_kitty);
		   <KittyOwner<T>>::insert(random_hash, &sender);
		   <OwnedKitty<T>>::insert(&sender, new_kitty);

			Self::deposit_event(RawEvent::KittyCreated(sender, random_hash));

		   <Nonce<T>>::mutate(|n| *n += 1);

		   Ok(())
	   }
	}
}

decl_event!(
	/// An event in this module.
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId, Hash = <T as system::Trait>::Hash {
		KittyCreated(AccountId, Hash),
	}
);

/// tests for this module
#[cfg(test)]
mod tests {
	use super::*;

	use runtime_io::with_externalities;
	use primitives::{H256, Blake2Hasher};
	use support::{impl_outer_origin, assert_ok};
	use runtime_primitives::{
		BuildStorage,
		traits::{BlakeTwo256, IdentityLookup},
		testing::{Digest, DigestItem, Header}
	};

	impl_outer_origin! {
		pub enum Origin for Test {}
	}

	// For testing the module, we construct most of a mock runtime. This means
	// first constructing a configuration type (`Test`) which `impl`s each of the
	// configuration traits of modules we want to use.
	#[derive(Clone, Eq, PartialEq)]
	pub struct Test;
	impl system::Trait for Test {
		type Origin = Origin;
		type Index = u64;
		type BlockNumber = u64;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type Digest = Digest;
		type AccountId = u64;
		type Lookup = IdentityLookup<u64>;
		type Header = Header;
		type Event = ();
		type Log = DigestItem;
	}
	impl Trait for Test {
		type Event = ();
	}
	type TemplateModule = Module<Test>;

	// This function basically just builds a genesis storage key/value store according to
	// our desired mockup.
	fn new_test_ext() -> runtime_io::TestExternalities<Blake2Hasher> {
		system::GenesisConfig::<Test>::default().build_storage().unwrap().0.into()
	}

	#[test]
	fn it_works_for_default_value() {
		with_externalities(&mut new_test_ext(), || {
			// Just a dummy test for the dummy funtion `do_something`
			// calling the `do_something` function with a value 42
			assert_ok!(TemplateModule::do_something(Origin::signed(1), 42));
			// asserting that the stored value is equal to what we stored
			assert_eq!(TemplateModule::something(), Some(42));
		});
	}
}
