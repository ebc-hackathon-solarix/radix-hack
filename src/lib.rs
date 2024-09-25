mod panel;

use scrypto::prelude::*;

#[blueprint]
mod solarix {
    struct Solarix {
        id_counter: u64,
    }

    impl Solarix {
        pub fn instantiate() -> Global<Solarix> {
            Self {
                id_counter: 0,
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn create_fractionalized_asset() {

        }

        fn getNextIdAndIncrement(&mut self) -> u64 {
            let id = self.id_counter;
            self.id_counter += 1;
            id
        }


        pub fn buy_nft() {

        }

        pub fn deposit_earnings() {

        }

        pub fn claim_earnings() {

        }

        pub fn claim_payout() {
            
        }
    }
}
