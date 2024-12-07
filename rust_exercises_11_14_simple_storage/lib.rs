#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod simple_storage {
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct SimpleStorage {
        /// Stores a single `bool` value on the storage.
        storage_value: i32,
    }

    impl SimpleStorage {
        /// Constructor that initializes the `i32` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(init_value: i32) -> Self {
            Self {
                storage_value: init_value,
            }
        }

        /// Constructor that initializes the `i32` value to `0`.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new(Default::default())
        }

        /// Updates the stored value with a new i32 value.
        ///
        /// The provided value will replace the existing stored value.
        #[ink(message)]
        pub fn store(&mut self, value: i32) {
            self.storage_value = value;

            if value < 0 {
                panic!("Cannot store a negative value in storage");
            }
        }

        /// Returns the current value stored in the contract.
        #[ink(message)]
        pub fn retrieve(&self) -> i32 {
            self.storage_value
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
            let rust_exercises_11_14_simple_storage = SimpleStorage::default();
            assert_eq!(rust_exercises_11_14_simple_storage.retrieve(), 0);
        }

        /// We test a simple use case of our contract.
        #[ink::test]
        fn it_works() {
            let mut rust_exercises_11_14_simple_storage = SimpleStorage::new(2);
            assert_eq!(rust_exercises_11_14_simple_storage.retrieve(), 2);
            rust_exercises_11_14_simple_storage.store(4);
            assert_eq!(rust_exercises_11_14_simple_storage.retrieve(), 4);
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
        use ink_e2e::ContractsBackend;

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = RustExercises1114SimpleStorageRef::default();

            // When
            let contract = client
                .instantiate(
                    "rust_exercises_11_14_simple_storage",
                    &ink_e2e::alice(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<SimpleStorage>();

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::alice(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        /// We test that we can read and write a value from the on-chain contract.
        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            // Given
            let mut constructor = RustExercises1114SimpleStorageRef::new(false);
            let contract = client
                .instantiate(
                    "rust_exercises_11_14_simple_storage",
                    &ink_e2e::bob(),
                    &mut constructor,
                )
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<SimpleStorage>();

            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), false));

            // When
            let flip = call_builder.flip();
            let _flip_result = client
                .call(&ink_e2e::bob(), &flip)
                .submit()
                .await
                .expect("flip failed");

            // Then
            let get = call_builder.get();
            let get_result = client.call(&ink_e2e::bob(), &get).dry_run().await?;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
