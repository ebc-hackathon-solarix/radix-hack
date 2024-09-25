use scrypto::prelude::*;

#[derive(ScryptoSbor, NonFungibleData)]
pub struct NFTData {
  name: String,
  asset_id: u32,
}

#[blueprint]
mod panel {
  struct Panel {
  }

  impl Panel {
    pub fn new() -> (Panel, Bucket) {
      
    }
  }
}
