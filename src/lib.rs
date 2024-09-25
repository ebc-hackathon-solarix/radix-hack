mod panel;
mod errors;

use scrypto::prelude::*;
use crate::panel::panel::Panel;
use crate::errors::MyError;

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

            let admin_rule = rule!(require(admin_badge.resource_address()));

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
                .prepare_to_globalize(OwnerRole::Fixed(admin_rule))
                .globalize();
    
            (solarix, admin_badge)

        }

        // Not tested, please check.
        pub fn create_fractionalized_asset(&mut self, owner_address: ComponentAddress, price_per_nft: Decimal, total_supply: u64) -> u64 {
            let panel_id = self._get_next_id_and_increment();
            let (_panel, nft_bucket, _panel_owner_badge) = Panel::new(
                panel_id,
                owner_address,
                price_per_nft,
                total_supply
            );
            self.non_fungible_vaults.insert(panel_id, NonFungibleVault::with_bucket(nft_bucket));
            self.panels.insert(panel_id, _panel);
            self.earnings_vaults_map.insert(panel_id, HashMap::new());
            self.payout_vaults.insert(owner_address, Vault::new(XRD));
            panel_id
        }

        fn _get_next_id_and_increment(&mut self) -> u64 {
            let id = self.id_counter;
            self.id_counter += 1;
            id
        }

        pub fn buy_nft(&mut self, panel_id: u64, quantity: u32, mut payment: Bucket) -> Result<(NonFungibleBucket, Bucket), MyError> {
            assert!(self.panels.contains_key(&panel_id), "Asset not found");
            let _panel = self.panels.get(&panel_id).unwrap();
            let vault: &mut NonFungibleVault = self.non_fungible_vaults.get_mut(&panel_id).unwrap();

            if payment.amount() >= (_panel.price_per_nft * quantity) {
                let expected = _panel.price_per_nft * quantity;
                return Err(MyError::InsufficientTokenAmount { expected, found: payment.amount() });
            }

            assert!(!vault.is_empty(), "Non fungible vault is empty");

            let nfts_ids = vault.as_non_fungible().non_fungible_local_ids(quantity);

            assert!(nfts_ids.len().to_u32().unwrap() >= quantity, "Not enough NFTs to buy");

            let payout_vault: &mut Vault = self.payout_vaults.get_mut(&_panel.payment_receiver).unwrap();
            let coins_to_pay: Bucket = payment.take(_panel.price_per_nft * quantity);
            let nft = vault.take_non_fungibles(&nfts_ids);

            payout_vault.put(coins_to_pay);
            let earnings_vault_map: &mut std::collections::HashMap<NonFungibleLocalId, Vault> = self.earnings_vaults_map.get_mut(&panel_id).unwrap();

            nfts_ids.iter().for_each(|nft_id| {
                earnings_vault_map.insert(nft_id.clone(), Vault::new(XRD));
            });

            Ok((nft, payment))
        }

        pub fn deposit_earnings(&mut self) {
            todo!()
        }

        pub fn claim_earnings(&mut self) {
            todo!()
        }

        pub fn claim_sales_proceeds(&mut self, panel_id: u64) {
            todo!()
        }
    }
}
