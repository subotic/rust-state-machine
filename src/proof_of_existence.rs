use core::fmt::Debug;
use std::collections::BTreeMap;

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    /// The type which represents the content that can be claimed using this pallet.
    /// Could be the content directly as bytes, or better yet the hash of that content.
    /// We leave that decision to the runtime developer.
    type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// A simple storage map from content to the owner of that content.
    /// Accounts can make multiple different claims, but each claim can only have one owner.
    /* TODO: Add a field `claims` which is a `BTreeMap` fom `T::Content` to `T::AccountId`. */
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    /// Get the owner (if any) of a claim.
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }

    /// Create a new claim on behalf of the `caller`.
    /// This function will return an error if someone already has claimed that content.
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        /* TODO: Check that a `claim` does not already exist. If so, return an error. */
        /* TODO: `insert` the claim on behalf of `caller`. */
        if self.claims.contains_key(&claim) {
            return Err("this content is already claimed");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    /// Revoke an existing claim on some content.
    /// This function should only succeed if the caller is the owner of an existing claim.
    /// It will return an error if the claim does not exist, or if the caller is not the owner.
    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let current_owner = self.get_claim(&claim).ok_or("Claim not existing")?;
        if current_owner != &caller {
            return Err(&"Cannot revoke claim that is not owned by caller");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}

// A public enum which describes the calls we want to expose to the dispatcher.
// We should expect that the caller of each call will be provided by the dispatcher,
// and not included as a parameter of the call.
pub enum Call<T: Config> {
    CreateClaim { claim: T::Content },
    RevokeClaim { claim: T::Content },
}

/// Implementation of the dispatch logic, mapping from `POECall` to the appropriate underlying
/// function we want to execute.
impl<T: Config> crate::support::Dispatch for Pallet<T> {
    type Caller = T::AccountId;
    type Call = Call<T>;

    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult {
        match call {
            Call::CreateClaim { claim } => self.create_claim(caller, claim)?,
            Call::RevokeClaim { claim } => self.revoke_claim(caller, claim)?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::Pallet;

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
        let content = "something";
        let alice = "alice";
        let bob = "bob";

        let mut pallet = Pallet::<TestConfig>::new();

        assert_eq!(pallet.get_claim(&content), None);
        assert_eq!(pallet.create_claim(alice, &content), Ok(()));
        assert_eq!(pallet.get_claim(&content), Some(&alice));
        assert_eq!(
            pallet.create_claim(bob, &content),
            Err("this content is already claimed")
        );
        assert_eq!(
            pallet.revoke_claim(alice, &"something else"),
            Err("Claim not existing")
        );
        assert_eq!(
            pallet.revoke_claim(bob, &content),
            Err("Cannot revoke claim that is not owned by caller")
        );
        assert_eq!(pallet.revoke_claim(alice, &content), Ok(()));
    }
}
