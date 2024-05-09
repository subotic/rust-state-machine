use std::{collections::BTreeMap, fmt::Debug};

use num::{CheckedAdd, CheckedSub, Zero};

use crate::support::DispatchResult;

pub trait Config: crate::system::Config {
    type Balance: Zero + CheckedSub + CheckedAdd + Copy + Debug;
}

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
    balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the balances module.
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }
    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &T::AccountId, amount: T::Balance) {
        /* Insert `amount` into the BTreeMap under `who`. */
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &T::AccountId) -> T::Balance {
        /* Return the balance of `who`, returning zero if `None`. */
        *self.balances.get(who).unwrap_or(&T::Balance::zero())
    }
}

#[macros::call]
impl<T: Config> Pallet<T> {
    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: T::AccountId,
        to: T::AccountId,
        amount: T::Balance,
    ) -> DispatchResult {
        // - Get the balance of account `caller`.
        let caller_balance = self.balance(&caller);

        // - Get the balance of account `to`.
        let to_balance = self.balance(&to);

        // - Use safe math to calculate a `new_caller_balance`.
        let new_caller_balance = caller_balance
            .checked_sub(&amount)
            .ok_or("Not enough funds")?;

        // - Use safe math to calculate a `new_to_balance`.
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        // - Insert the new balance of `caller`.
        self.set_balance(&caller, new_caller_balance);

        // - Insert the new balance of `to`.
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use super::Pallet;

    struct TestConfig;
    impl Config for TestConfig {
        type Balance = u128;
    }
    impl crate::system::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_balances() {
        /* TODO: Create a mutable variable `balances`, which is a new instance of `Pallet`. */
        let mut balances = Pallet::<TestConfig>::new();

        /* TODO: Assert that the balance of `alice` starts at zero. */
        assert_eq!(balances.balance(&"alice".to_string()), 0);

        /* TODO: Set the balance of `alice` to 100. */
        balances.set_balance(&"alice".to_string(), 100);

        /* TODO: Assert the balance of `alice` is now 100. */
        assert_eq!(balances.balance(&"alice".to_string()), 100);

        /* TODO: Assert the balance of `bob` has not changed and is 0. */
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        /* TODO: Create a test that checks the following:
            - That `alice` cannot transfer funds she does not have.
            - That `alice` can successfully transfer funds to `bob`.
            - That the balance of `alice` and `bob` is correctly updated.
        */
        let mut balances = Pallet::<TestConfig>::new();
        assert_eq!(
            balances
                .transfer("alice".to_string(), "bob".to_string(), 100)
                .is_err(),
            true
        );
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(
            balances
                .transfer("alice".to_string(), "bob".to_string(), 100)
                .is_ok(),
            true
        );
        assert_eq!(balances.balance(&"alice".to_string()), 0);
        assert_eq!(balances.balance(&"bob".to_string()), 100);
    }
}
