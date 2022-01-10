use scrypto::prelude::*;

#[derive(NftData)]
pub struct ProductContribution {
    pub amount: u32,
    pub task_description: String,
    pub external_reference: String,
}

blueprint! {
    struct CollaborativeProduct {
        // Metadata
        pub name: String,
        pub external_reference: String,
        pub product_description: String,

        // Define what resources and data will be managed by ProductContribution components

        // Time contributed to products
        product_contribution_time_vault: Vault,

        // The badge for controlling who is super admin
        admin_badge: ResourceDef,
        admin_badge_vault: Vault,

        // The badge for controlling who is allowed to spend time on products
        product_contribution_badge : ResourceDef,
        product_contribution_badge_vault : Vault,
    }

    impl CollaborativeProduct {
        // Implement the functions and methods which will manage those resources and data

        // This is a function, and can be called directly on the blueprint once deployed
        pub fn new(name: String, external_reference: String, product_description: String,) -> Component {

            // Create resource definitions for badges / access control

            let admin_badge: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name","ProductContributionTime general admin badge")
                .flags(FREELY_BURNABLE)
                .initial_supply_fungible(1);
            let product_contribution_badge: Bucket = ResourceBuilder::new_fungible(DIVISIBILITY_NONE)
                .metadata("name","ProductContributionTime time booking badge")
                .flags(FREELY_BURNABLE)
                .initial_supply_fungible(1);

            // Create a new token called "ProductContributionTime" with no initial supply
            let product_contribution_time_def: ResourceDef = ResourceBuilder::new_non_fungible()
                .metadata("name", "ProductContributionTime")
                .metadata("symbol", "PCT")
                .flags(SHARED_METADATA_MUTABLE)
                .mutable_flags(MINTABLE | SHARED_METADATA_MUTABLE)
                .badge(
                    admin_badge.resource_def(),
                    ALL_PERMISSIONS
                )
                .badge(
                    product_contribution_badge.resource_def(),
                    MAY_MINT | MAY_TRANSFER
                )
                .no_initial_supply();

            // Instantiate a ProductContribution component, populating its vault with our definition
            Self {
                name: name,
                external_reference: external_reference,
                product_description: product_description,
                product_contribution_time_vault: Vault::new(product_contribution_time_def),
                product_contribution_badge: product_contribution_badge.resource_def(),
                product_contribution_badge_vault: Vault::with_bucket(product_contribution_badge),
                admin_badge: admin_badge.resource_def(),
                admin_badge_vault: Vault::with_bucket(admin_badge),
            }
            .instantiate()
        }

        // This method may only be called if admin or product_contribution badge is present
        #[auth(admin_badge,product_contribution_badge,keep_auth)]
        pub fn book_product_contribution_time(&mut self, amount: u32, task_description: String,external_reference: String) {
            info!("My balance is: {} minutes on product {}. Now booking additional {} minutes for task {}!",
                self.product_contribution_time_vault.amount(), self.name, amount, task_description);
            let product_contribution_time: Bucket = self.product_contribution_badge_vault.authorize(|auth| {
                    self.product_contribution_time_vault.resource_def().mint_nft(
                        // The NFT id
                        Uuid::generate(),
                        // The NFT data
                        ProductContributionTime {
                            amount: amount,
                            task_description: task_description,
                            external_reference: external_reference,
                        },
                        // authorization to mint
                        auth
                    )
                });
            self.product_contribution_time_vault.put(product_contribution_time);
        }
    }
}
