mod panel;

use scrypto::prelude::*;
use crate::panel::panel::Panel;

#[blueprint]
mod solarix {
    enable_method_auth! {
        methods {
            buy_nft => PUBLIC;
            claim_sales_proceeds => PUBLIC;
            create_fractionalized_asset => restrict_to: [OWNER];
            deposit_earnings => restrict_to: [OWNER];
            claim_earnings => PUBLIC;
        }
    }

    struct Solarix {
        non_fungible_vaults: HashMap<u64, NonFungibleVault>, // Maps panel ids to non fungible vaults containing their NFTs
        panels: HashMap<u64, Panel>, // Maps panel ids to their respective panel struct
        earnings_vaults_map: HashMap<u64, HashMap<NonFungibleLocalId, Vault>>, // Maps panel ids to their respective earnings vaults
        payout_vaults: HashMap<ComponentAddress, Vault>, // Maps accounts to their respective payout vaults
        protocol_collected_fees: Vault, // Vault containing fees collected by the protocol
        admin_badge_address: ResourceAddress,
        id_counter: u64
    }

    impl Solarix {
        pub fn instantiate() -> (Global<Solarix>, NonFungibleBucket) {
            let admin_badge = ResourceBuilder::new_integer_non_fungible(OwnerRole::None)
                .metadata(metadata! {
                    init {
                        "name" => "Admin Badge NFT", locked;
                        "description" => "A non-fungible token representing admin privileges in the solarix system.", locked;
                    }
                })
                .mint_initial_supply([(1.into(), {})]);

                let solarix: Global<Solarix> = Self {
                    non_fungible_vaults: HashMap::new(),
                    panels: HashMap::new(),
                    earnings_vaults_map: HashMap::new(),
                    payout_vaults: HashMap::new(),
                    protocol_collected_fees: Vault::new(XRD),
                    admin_badge_address: admin_badge.resource_address(),
                    id_counter: 0
                }
                .instantiate()
                .prepare_to_globalize(OwnerRole::None)
                .globalize();
    
                (solarix, admin_badge)

        }

        // Not tested, please check.
        pub fn create_fractionalized_asset(&mut self, owner: ComponentAddress,  pricePerNft: Decimal, totalSupply: u64) -> u64 { {
            let id = self._get_next_id_and_increment();
            let (panel, nft_bucket) = Panel::new(id, owner, pricePerNft, totalSupply);
            self.non_fungible_vaults.insert(id, NonFungibleVault::with_bucket(nft_bucket));
            self.panels.insert(id, panel);
            self.earnings_vaults_map.insert(id, HashMap::new());
            self.panels.get(&id).unwrap().price_per_nft;

            // initialise an earnings vault for every localid? Maybe gas inefficient.  

            let total_supply = self.panels.get(&id).unwrap().total_supply;
            for i in 0..total_supply {
                let local_id = NonFungibleLocalId::integer(i);
                self.earnings_vaults_map.get_mut(&id).unwrap().insert(local_id, Vault::new(XRD));
            }
            id
        }

        fn _get_next_id_and_increment(&mut self) -> u64 {
            let id = self.id_counter;
            self.id_counter += 1;
            id
        }

        pub fn buy_nft(&mut self) {
            todo!()
        }

        pub fn deposit_earnings(&mut self) {
            todo!()
        }

        pub fn claim_earnings(&mut self) {
            todo!()
        }

        pub fn claim_sales_proceeds(&mut self) {
            todo!()
        }
    }
}
