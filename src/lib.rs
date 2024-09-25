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
    }
}
