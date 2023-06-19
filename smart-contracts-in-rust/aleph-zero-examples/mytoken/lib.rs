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
    use ink::{storage::Mapping, primitives::AccountId};
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
        pub fn balance_of(&self, account: AccountId) -> Balance {
            // retrieve a value from the balances mapping for a given account
            // result comes wrapped in an Option struct
            // unwrap_or_default() retrieves the actual value 
            // or default for the Balance type (which is 0)
            self.balances.get(&account).unwrap_or_default()
        }
        
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[ink::test]
        fn default_works() {
            let mytoken = Mytoken::default();
            assert_eq!(mytoken.get(), false);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut mytoken = Mytoken::new(false);
            assert_eq!(mytoken.get(), false);
            mytoken.flip();
            assert_eq!(mytoken.get(), true);
        }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::build_message;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = MytokenRef::default();

            // When
            let contract_account_id = client
                .instantiate("mytoken", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            // Then
            let get = build_message::<MytokenRef>(contract_account_id.clone())
                .call(|mytoken| mytoken.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let constructor = MytokenRef::new(false);
            let contract_account_id = client
                .instantiate("mytoken", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<MytokenRef>(contract_account_id.clone())
                .call(|mytoken| mytoken.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = build_message::<MytokenRef>(contract_account_id.clone())
                .call(|mytoken| mytoken.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            // Then
            let get = build_message::<MytokenRef>(contract_account_id.clone())
                .call(|mytoken| mytoken.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
