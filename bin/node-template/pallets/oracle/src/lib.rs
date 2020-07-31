#![cfg_attr(not(feature = "std"), no_std)]

/// A FRAME pallet template with necessary imports

/// Feel free to remove or edit this file as needed.
/// If you change the name of this file, make sure to update its references in runtime/src/lib.rs
/// If you remove this file, you can remove those references

/// For more guidance on Substrate FRAME, see the example pallet
/// https://github.com/paritytech/substrate/blob/master/frame/example/src/lib.rs

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::{self as system, ensure_signed};
use frame_support::traits::Get;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// The pallet's configuration trait.
pub trait Trait: system::Trait {
	// Add other types and constants required to configure this pallet.

	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as system::Trait>::Event>;
}

use sp_std::{marker::PhantomData, prelude::*};
use codec::Decode;
pub struct DataFeedGet<Key: Get<[u8; 32]>, Type:Decode + Sized, Value: Get<Type> >(
	PhantomData<Key>,
	PhantomData<Type>,
	PhantomData<Value>,
);
impl<Type:Decode + Sized, Key: Get<[u8; 32]>, Value: Get<Type>> Get<Type> for DataFeedGet<Key, Type, Value> {
	fn get() -> Type {
		frame_support::storage::unhashed::get_or(Key::get().as_ref(), Value::get())
	}
}

// The pallet's events
decl_event!(
	pub enum Event<T> where AccountId = <T as system::Trait>::AccountId {
		/// Just a dummy event.
		/// Event `Something` is declared with a parameter of the type `u32` and `AccountId`
		/// To emit this event, we call the deposit function, from our runtime functions
		SomethingStored(u32, AccountId),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Value was None
		NoneValue,
		/// Value reached maximum and cannot be incremented further
		StorageOverflow,
	}
}

// This pallet's storage items.
decl_storage! {
	trait Store for Module<T: Trait> as TemplateModule {
		StorageKeys: Vec<Vec<u8>>;

		Something get(fn something): Option<u32>;
	}
}

impl<T: Trait> Module<T> {
	fn set_storage_value(key: Vec<u8>, value: Vec<u8>) {
		// e.g. receive to set key b":Fxck:" in period end, value is u64.encode()
		frame_support::storage::unhashed::put(&key, &value);
	}
}
decl_module! {
	/// The module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		// Initializing events
		// this is needed only if you are using events in your pallet
		fn deposit_event() = default;

		#[weight = 0]
		pub fn register_storage_key(origin, key: Vec<u8>) {
			// parse origin
			// e.g. the key is b":Fxck:"
			StorageKeys::mutate(|v| {
				if !v.contains(&key) {
					v.push(key)
				}
			});
		}

		/// Just a dummy entry point.
		/// function that can be called by the external world as an extrinsics call
		/// takes a parameter of the type `AccountId`, stores it, and emits an event
		#[weight = 0]
		pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let who = ensure_signed(origin)?;

			// Code to execute when something calls this.
			// For example: the following line stores the passed in u32 in the storage
			Something::put(something);

			// Here we are raising the Something event
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			Ok(())
		}

		/// Another dummy entry point.
		/// takes no parameters, attempts to increment storage value, and possibly throws an error
		#[weight = 0]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			// Check it was signed and get the signer. See also: ensure_root and ensure_none
			let _who = ensure_signed(origin)?;

			match Something::get() {
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					Something::put(new);
					Ok(())
				},
			}
		}
	}
}
