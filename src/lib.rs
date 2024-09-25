mod panel;

use scrypto::prelude::*;
use crate::panel::panel::Panel;

#[blueprint]
mod solarix {
    struct Solarix {
        non_fungible_vaults: HashMap<u64, NonFungibleVault>, // Maps panel ids to non fungible vaults containing their NFTs
        panels: HashMap<u64, Panel>, // Maps panel ids to their respective panel struct
        earnings_vaults_map: HashMap<u64, HashMap<NonFungibleLocalId, Vault>>, // Maps panel ids to their respective earnings vaults
        payout_vaults: HashMap<ComponentAddress, Vault>, // Maps accounts to their respective payout vaults
        protocol_collected_fees: Vault, // Vault containing fees collected by the protocol
        admin_badge_address: ResourceAddress 
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
