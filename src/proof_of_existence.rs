use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
	/// The type which represents the content that can be claimed using this pallet.
	type Content: Debug + Ord;
}

/// This module allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
	/// A simple storage map from content to the owner of that content.
	/// Accounts can make multiple different claims, but each claim can only have one owner.
	claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
	/// Create a new instance of the Proof of Existence Module.
	pub fn new() -> Self {
		Self { claims: BTreeMap::new() }
	}

	/// Get the owner (if any) of a claim.
	pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
		self.claims.get(&claim)
	}
}

#[macros::call]
impl<T: Config> Pallet<T> {
	/// Create a new claim on behalf of the `caller`.
	pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		if self.claims.contains_key(&claim) {
			return Err(&"this content is already claimed");
		}
		self.claims.insert(claim, caller);
		Ok(())
	}

	/// This function should only succeed if the caller is the owner of an existing claim.
	pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
		let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
		if caller != *owner {
			return Err(&"this content is owned by someone else");
		}
		self.claims.remove(&claim);
		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;

	impl super::Config for TestConfig {
		type Content = &'static str;
	}

	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn basic_proof_of_existence() {
		let mut poe = super::Pallet::<TestConfig>::new();
		assert_eq!(poe.get_claim(&"Hello, world!"), None);
		assert_eq!(poe.create_claim(&"alice", &"Hello, world!"), Ok(()));
		assert_eq!(poe.get_claim(&"Hello, world!"), Some(&"alice"));
		assert_eq!(
			poe.create_claim(&"bob", &"Hello, world!"),
			Err("this content is already claimed")
		);
		assert_eq!(poe.revoke_claim(&"alice", &"Hello, world!"), Ok(()));
		assert_eq!(poe.create_claim(&"bob", &"Hello, world!"), Ok(()));
	}
}