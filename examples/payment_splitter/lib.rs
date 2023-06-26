#![cfg_attr(not(feature = "std"), no_std)]

#[openbrush::implementation(PaymentSplitter)]
#[openbrush::contract]
pub mod my_payment_splitter {
    use ink::prelude::vec::Vec;
    use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        splitter: payment_splitter::Data,
    }

    impl Contract {
        #[ink(constructor)]
        pub fn new(payees_and_shares: Vec<(AccountId, Balance)>) -> Self {
            let mut instance = Self::default();
            payment_splitter::Internal::_init(&mut instance, payees_and_shares).expect("Should init");
            instance
        }

        /// Payout all payees at once.
        /// Delete this method if you don't want this functionality in your version of the payment splitter.
        #[ink(message)]
        pub fn release_all(&mut self) -> Result<(), PaymentSplitterError> {
            // `_release_all()` is an internal method defined by the `payment_splitter::Internal` trait
            payment_splitter::Internal::_release_all(self)
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    pub mod tests {
        use openbrush::contracts::payment_splitter::paymentsplitter_external::PaymentSplitter;

        #[rustfmt::skip]
        use super::*;
        #[rustfmt::skip]
        use ink_e2e::{build_message, PolkadotConfig};

        use test_helpers::{
            address_of,
            get_shares,
            method_call,
        };

        type E2EResult<T> = Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn init_values(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(vec![(address_of!(bob), 40), (address_of!(alice), 60)]);
            let address = client
                .instantiate("my_payment_splitter", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let bob_shares = get_shares!(client, address, bob);

            assert_eq!(bob_shares, 40);

            let alice_shares = get_shares!(client, address, alice);

            assert_eq!(alice_shares, 60);

            let total_shares = method_call!(client, address, total_shares);

            assert_eq!(total_shares, 100);

            let payee_0 = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.payee(0));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            assert_eq!(payee_0, Some(address_of!(bob)));

            let payee_1 = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.payee(1));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            assert_eq!(payee_1, Some(address_of!(alice)));

            Ok(())
        }

        #[ink_e2e::test]
        async fn release_native_token(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(vec![(address_of!(bob), 40), (address_of!(alice), 60)]);
            let address = client
                .instantiate("my_payment_splitter", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let total_released_before = method_call!(client, address, total_released);

            assert_eq!(total_released_before, 0);

            let _receive_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.receive());
                client
                    .call(&ink_e2e::alice(), _msg, 1000000000000, None)
                    .await
                    .expect("call failed")
                    .return_value()
            };

            let release_bob = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.release(address_of!(bob)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
                    .return_value()
            };

            assert!(release_bob.is_ok());

            let release_alice = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.release(address_of!(alice)));
                client
                    .call(&ink_e2e::alice(), _msg, 0, None)
                    .await
                    .expect("call failed")
                    .return_value()
            };

            assert!(release_alice.is_ok());

            let total_released = method_call!(client, address, total_released);

            let bob_released = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.released(address_of!(bob)));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            let alice_released = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.released(address_of!(alice)));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            assert!(alice_released > bob_released);

            assert_eq!(bob_released, (total_released * 40) / 100);

            assert_eq!(alice_released, (total_released * 60) / 100);

            assert_eq!(alice_released + bob_released, total_released);

            Ok(())
        }

        #[ink_e2e::test]
        async fn release_native_token_using_release_all(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = ContractRef::new(vec![(address_of!(bob), 40), (address_of!(alice), 60)]);
            let address = client
                .instantiate("my_payment_splitter", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let total_released_before = method_call!(client, address, total_released);

            assert_eq!(total_released_before, 0);

            let _receive_tx = {
                let _msg = build_message::<ContractRef>(address.clone()).call(|contract| contract.receive());
                client
                    .call(&ink_e2e::alice(), _msg, 1000000000000, None)
                    .await
                    .expect("call failed")
                    .return_value()
            };

            let release_all = method_call!(client, address, release_all);

            assert!(release_all.is_ok());

            let total_released = method_call!(client, address, total_released);

            let bob_released = {
                let _msg =
                    build_message::<ContractRef>(address.clone()).call(|contract| contract.released(address_of!(bob)));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            let alice_released = {
                let _msg = build_message::<ContractRef>(address.clone())
                    .call(|contract| contract.released(address_of!(alice)));
                client
                    .call_dry_run(&ink_e2e::alice(), &_msg, 0, None)
                    .await
                    .return_value()
            };

            assert!(alice_released > bob_released);

            assert_eq!(bob_released, (total_released * 40) / 100);

            assert_eq!(alice_released, (total_released * 60) / 100);

            assert_eq!(alice_released + bob_released, total_released);

            Ok(())
        }
    }
}
