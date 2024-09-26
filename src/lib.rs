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
        earnings_vaults_maps: HashMap<u64, HashMap<NonFungibleLocalId, Vault>>, // Maps panel ids to their respective earnings vaults
        payout_vaults: HashMap<ComponentAddress, Vault>, // Maps accounts to their respective payout vaults
        protocol_collected_fees: Vault, // Vault containing fees collected by the protocol
        admin_badge_address: ResourceAddress,
        id_counter: u64,
        buy_nft_fee: Decimal, // fee to be applied when buying 
        earnings_fee: Decimal, // fee to be applied to the amount being deposited as earnings
    }

    impl Solarix {
        pub fn instantiate(buy_nft_fee: Decimal, earnings_fee: Decimal) -> (Global<Solarix>, NonFungibleBucket) {
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
                    earnings_vaults_maps: HashMap::new(),
                    payout_vaults: HashMap::new(),
                    protocol_collected_fees: Vault::new(XRD),
                    admin_badge_address: admin_badge.resource_address(),
                    id_counter: 0,
                    buy_nft_fee,
                    earnings_fee
                }
                .instantiate()
                .prepare_to_globalize(OwnerRole::Fixed(admin_rule))
                .globalize();
    
            (solarix, admin_badge)

        }

        pub fn create_fractionalized_asset(&mut self, owner_address: ComponentAddress, price_per_nft: Decimal, total_supply: u64) -> u64 {
            let panel_id = self._get_next_id_and_increment();
            let (_panel, nft_bucket) = Panel::new(
                panel_id,
                owner_address,
                price_per_nft,
                total_supply
            );

            self.non_fungible_vaults.insert(panel_id, NonFungibleVault::with_bucket(nft_bucket));
            self.panels.insert(panel_id, _panel);
            self.earnings_vaults_maps.insert(panel_id, HashMap::new());
            self.payout_vaults.insert(owner_address, Vault::new(XRD));
            panel_id
        }

        fn _get_next_id_and_increment(&mut self) -> u64 {
            let id = self.id_counter;
            self.id_counter += 1;
            id
        }

        pub fn buy_nft(&mut self, panel_id: u64, quantity: u32, mut payment: Bucket) -> (NonFungibleBucket, Bucket) {

            assert!(self.panels.contains_key(&panel_id), "{}", MyError::AssetNotFound);

            
            let _panel = self.panels.get(&panel_id).unwrap();
            let vault: &mut NonFungibleVault = self.non_fungible_vaults.get_mut(&panel_id).unwrap();
            let expected_amount = _panel.price_per_nft * quantity;

            assert!(payment.amount() >= (_panel.price_per_nft * quantity), "{}", MyError::InsufficientTokenAmount {
                expected: expected_amount,
                found: payment.amount()
            });
    
            assert!(!vault.is_empty(), "{}", MyError::NonFungibleVaultEmptyError);

            let nfts_ids = vault.as_non_fungible().non_fungible_local_ids(quantity);

            assert!(nfts_ids.len().to_u32().unwrap() >= quantity, "{}", MyError::InsufficientSupply {
                requested: quantity,
                available: nfts_ids.len().to_u32().unwrap()
            });

            let payout_vault: &mut Vault = self.payout_vaults.get_mut(&_panel.payment_receiver).unwrap();
            let mut coins_to_pay: Bucket = payment.take(_panel.price_per_nft * quantity);
            let nft = vault.take_non_fungibles(&nfts_ids);

            let accrued_fee = coins_to_pay.amount() * self.buy_nft_fee;
            let accrued_bucket = coins_to_pay.take(accrued_fee);

            self.protocol_collected_fees.put(accrued_bucket);

            payout_vault.put(coins_to_pay);
            let earnings_vault_map: &mut std::collections::HashMap<NonFungibleLocalId, Vault> = self.earnings_vaults_maps.get_mut(&panel_id).unwrap();

            nfts_ids.iter().for_each(|nft_id| {
                earnings_vault_map.insert(nft_id.clone(), Vault::new(XRD));
            });

            (nft, payment)
        }

        pub fn deposit_earnings(&mut self, panel_id: u64, mut earnings: Bucket) {
            assert!(self.non_fungible_vaults.get(&panel_id).unwrap().is_empty(), "{}", MyError::NonFungibleVaultNotEmptyError);

            let accrued_fee_amount = earnings.amount() * self.earnings_fee;
            self.protocol_collected_fees.put(earnings.take(accrued_fee_amount));

            let vault_map = self.earnings_vaults_maps.get_mut(&panel_id).unwrap();
            let entries_number: u32 = vault_map.len().to_u32().unwrap();
            let amount_to_deposit = earnings.amount() / entries_number;

            vault_map.iter_mut().for_each(|(_nft_id, vault)| {
                vault.put(earnings.take(amount_to_deposit));
            });
        }

        pub fn claim_earnings(&mut self, panel_id: u64, nft_proof: NonFungibleProof) -> Bucket {
            assert!(self.panels.contains_key(&panel_id), "{}", MyError::AssetNotFound);

            let asset = self.panels.get(&panel_id).unwrap();

            assert!(nft_proof.resource_address() == asset.nft_resource_address, "{}", MyError::NotAuthorizedToClaimPayoutError);
            
            let checked_nft: CheckedNonFungibleProof = nft_proof.check_with_message(asset.nft_resource_address, "Invalid proof");

            let vault_map = self.earnings_vaults_maps.get_mut(&panel_id).unwrap();
            let vault: &mut Vault = vault_map.get_mut(&checked_nft.non_fungible_local_id()).unwrap();

            vault.take_all()
        }

        pub fn claim_sales_proceeds(&mut self, account: Global<Account>) -> Bucket {
            Runtime::assert_access_rule(account.get_owner_role().rule);
            assert!(self.payout_vaults.contains_key(&account.address()), "{}", MyError::NotAuthorizedToClaimPayoutError);
            let vault = self.payout_vaults.get_mut(&account.address()).unwrap();
            vault.take_all()
        }
    }
}
