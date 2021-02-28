#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {

    use ink_storage::collections::HashMap as StorageHashMap;

    #[ink(storage)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: StorageHashMap<AccountId, Balance>,
        allowance: StorageHashMap<(AccountId, AccountId), Balance>,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        balance: Balance
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InSufficientBalance
    }
    pub type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = StorageHashMap::new();
            balances.insert(caller, total_supply);
            let instance = Self {
                total_supply,
                balances,
                allowance: StorageHashMap::new()
            };
            instance
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            *self.balances.get(&owner).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            *self.allowance.get(&(owner, spender)).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, balance: Balance) -> Result<()> {
            let who = Self::env().caller();
            self.transfer_help(who, to, balance)
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, balance: Balance) -> Result<()> {
            let who = Self::env().caller();
            let allowance = self.allowance(from, who);
            if allowance < balance {
                return Err(Error::InSufficientBalance)
            }
            self.transfer_help(from, to, balance)?;
            self.allowance.insert((from, who), allowance - balance);
            Ok(())
        }

        fn transfer_help(&mut self, from: AccountId, to: AccountId, balance: Balance) -> Result<()> {
            let from_balance = self.balance_of(from);
            if from_balance < balance {
                return Err(Error::InSufficientBalance)
            }
            self.balances.insert(from, from_balance - balance);
            let to_balance = self.balance_of(to);
            self.balances.insert(to, to_balance + balance);
            self.env().emit_event(Transfer {
                from,
                to,
                balance
            });
            Ok(())
        }

    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn create_contract_work() {
            let erc20 = Erc20::new(1000);
            assert_eq!(erc20.total_supply(), 1000);
        }

    }
}
