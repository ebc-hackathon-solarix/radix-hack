mod panel;

use scrypto::prelude::*;

#[blueprint]
mod solarix {
    struct Solarix {
    }

    impl Solarix {
        pub fn instantiate() -> Global<Solarix> {
            Self {
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::None)
            .globalize()
        }

        pub fn create_fractionalized_asset() {

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
