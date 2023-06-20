#![cfg_attr(not(feature = "std"), no_std, no_main)]
/// example contract following tutorial at 
/// https://docs.alephzero.org/aleph-zero/build/aleph-zero-smart-contracts-basics/creating-your-first-contract
/// simple version of ERC20 token
/// contract when intantiated will create pool of fungible tokens that can be transferred between accounts
/// (1) contract will hold a registry of accounts with balances
/// (2) contract will provide methods to query balances and transfer tokens

/// smart contract written in ink! is regular Rust code that makes use of ink! macros
/// the macros modify the compilation process in order to produce WASM code

/// the #![cfg_attr(not(feature = "std"), no_std, no_main)] macro at the top 
/// instructs the compiler to not use the standards library
/// in ink! smart contracts you cannot use data structures from the standard library

#[ink::contract]
/// module prefixed with the main ink! macro
/// sets type aliases like Balance and Account
/// #[ink::contract] macro tells ink! that module is a definition of a smart contract
/// a contract needs to have exactly one struct marked as #[ink::storage]
/// a contract needs to have at least one function marked as #[ink::constructor]
mod mytoken {
    use ink::{storage::Mapping};
    /// Defines the storage of your contract.
    /// contains data that's stored on the blockchain
    /// holds the state of the contract
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Mytoken {
        /// ink! will create an implementation of the Default trait
        /// total number of tokens
        total_supply: Balance,
        /// mapping between users and the number of tokens they own
        /// mapping data structure is provided by the ink::storage crate
        balances: Mapping<AccountId, Balance>,
    }
    
    /// defining a custom Error struct that we'll use as the Err part of the Result
    /// Error type enum with just one variant that takes no arguments
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
    }

    impl Mytoken {
        /// implementing a constructor for the MyToken contract
        /// contract can have an arbitrary non-zero number of constructors
        /// takes single argument: the initial supply of newly created token
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            // creating an empty Mapping
            let mut balances = Mapping::default();
            // access the address of an account that called contract
            // in context of the constructor this will be the contract's creator/owner
            let caller = Self::env().caller();
            // deposits all that supply to the account of the contract creator
            balances.insert(caller, &total_supply);
            Self {
                total_supply,
                balances
            }
        }
        
        /// the callable methods (messages) of our contract 
        /// a message can be called on instantiated contracts
        /// two methods for accessing the storage of the contract
        /// both are read-only: don't modify the contract storage 
        /// & can be called without submitting a transaction
        /// 
        /// reading the total supply
        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }
        
        /// number of tokens held by a particular account
        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            // retrieve a value from the balances mapping for a given account
            // result comes wrapped in an Option struct
            // unwrap_or_default() retrieves the actual value 
            // or default for the Balance type (which is 0)
            self.balances.get(&owner).unwrap_or_default()
        }

        /// method for transferring tokens between accounts
        /// method can be called by any user to transfer tokens to a chosen recipient
        /// this method modifies the contract storage
        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<(), Error> {
            let from = self.env().caller();
            // query the balance of the caller
            let from_balance = self.balance_of(from);
            // if user tries to transfer more tokens than they own
            // method exits without performing any changes
            if from_balance < value {
                return Err(Error::InsufficientBalance);
            }
            
            // transfer modifies the contract storage
            self.balances.insert(from, &(from_balance - value));
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            Ok(())
        }
        
    }

    /// test section copied from Aleph Zero documentation
    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn total_supply_works() {
            let mytoken = Mytoken::new(100);
            assert_eq!(mytoken.total_supply(), 100);
        }

        #[ink::test]
        fn balance_of_works() {
            let mytoken = Mytoken::new(100);
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
            assert_eq!(mytoken.balance_of(accounts.alice), 100);
            assert_eq!(mytoken.balance_of(accounts.bob), 0);
        }

        #[ink::test]
        fn transfer_works() {
            let mut mytoken = Mytoken::new(100);
            let accounts =
                ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

            assert_eq!(mytoken.balance_of(accounts.bob), 0);
            assert_eq!(mytoken.transfer(accounts.bob, 10), Ok(()));
            assert_eq!(mytoken.balance_of(accounts.bob), 10);
        }
    }
    
}
