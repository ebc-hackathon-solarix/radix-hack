use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct NFTData {
  name: String,
  asset_id: u32,
}

#[blueprint]
mod panel {
  struct Panel {
    pub total_supply: u64,
    payment_receiver: ComponentAddress,
    pub price_per_nft: Decimal,
    pub panel_id: u64,
    pub nft_resource_address: ResourceAddress
  }

  impl Panel {
    pub fn new() -> (Panel, Bucket) {
      
    }
  }
}
