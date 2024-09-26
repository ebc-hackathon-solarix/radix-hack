use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct NFTData {
  name: String,
  asset_id: u64,
}

#[blueprint]
mod panel {
  struct Panel {
    pub total_supply: u64,
    pub payment_receiver: ComponentAddress,
    pub price_per_nft: Decimal,
    pub panel_id: u64,
    pub nft_resource_address: ResourceAddress
  }

  impl Panel {
    pub fn new(
      id: u64,
      payment_receiver: ComponentAddress,
      price_per_nft: Decimal,
      supply: u64
    ) -> (Panel, NonFungibleBucket, NonFungibleBucket) {
      let panel_owner_badge = ResourceBuilder::new_integer_non_fungible(OwnerRole::None)
        .metadata(metadata! {
          init {
            "name" => "Panel Owner Badge", locked;
            "description" => "A non-fungible token representing ownership of a panel. Needed to redeem sales proceeds", locked;
          }
        })
        .mint_initial_supply([(1.into(), {})]);

      let nft_bucket = ResourceBuilder::new_integer_non_fungible(OwnerRole::None)
        .metadata(metadata! {
          init {
            "name" => format!("Panel #{} NFT", id), locked;
            "description" => "A non-fungible token representing ownership of a panel in the solarix system.", locked;
          }
        })
        .mint_initial_supply((0..supply).map(|nft_id: u64| (nft_id.into(), NFTData { name: format!("Panel #{} NFT", id), asset_id: id })));

      let panel = Self {
        total_supply: supply,
        payment_receiver: payment_receiver,
        price_per_nft,
        panel_id: id,
        nft_resource_address: nft_bucket.resource_address()
      };

      (panel, nft_bucket, panel_owner_badge)
    }
  }
}
