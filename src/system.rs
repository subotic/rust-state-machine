/* TODO: You might need to update your imports. */

use std::{collections::BTreeMap, fmt::Debug, ops::AddAssign};

use num::{One, Zero};

pub trait Config {
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + AddAssign + Copy + Debug;
    type Nonce: Zero + One + Copy + Debug;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// The current block number.
    block_number: T::BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        /* TODO: Return a new instance of the `Pallet` struct. */
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        /* TODO: Return the current block number. */
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        /* TODO: Increment the current block number by one. */
        self.block_number += T::BlockNumber::one();
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        /* TODO: Get the current nonce of `who`, and increment it by one. */
        let current_nonce = self.nonce.get(who);
        match current_nonce {
            Some(current_nonce) => self
                .nonce
                .insert(who.clone(), *current_nonce + T::Nonce::one()),
            None => self.nonce.insert(who.clone(), T::Nonce::one()),
        };
    }
}

#[cfg(test)]
mod test {
    use super::Config;
    use super::Pallet;

    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        /* TODO: Create a test which checks the following:
            - Increment the current block number.
            - Increment the nonce of `alice`.
            - Check the block number is what we expect.
            - Check the nonce of `alice` is what we expect.
        */
        let mut system = Pallet::<TestConfig>::new();

        system.inc_block_number();
        assert_eq!(system.block_number(), 1);

        system.inc_nonce(&"alice".to_string());
        assert_eq!(*system.nonce.get(&"alice".to_string()).unwrap(), 1);
    }
}
