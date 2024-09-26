#!/usr/bin/env sh
#set -x
set -e

echo "Resetting environment"
resim reset

echo "Setting up the ledger"
export XRD="resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3"

echo "Creating accounts"
OP1=$(resim new-account)
export account_priv_key=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account_owner_badge=$(echo "$OP1" | sed -nr "s/Owner badge: ([[:alnum:]_]+)/\1/p")
export account=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
echo "Account = " $account

OP1=$(resim new-account)
export account2_priv_key=$(echo "$OP1" | sed -nr "s/Private key: ([[:alnum:]_]+)/\1/p")
export account2_owner_badge=$(echo "$OP1" | sed -nr "s/Owner badge: ([[:alnum:]_]+)/\1/p")
export account2=$(echo "$OP1" | sed -nr "s/Account component address: ([[:alnum:]_]+)/\1/p")
echo "Account 2 = " $account2
echo "Account 2 owner badge = " $account2_owner_badge
echo "Account 2 private key = " $account2_priv_key

echo "Publishing dapp"
export package=$(resim publish . | sed -nr "s/Success! New Package: ([[:alnum:]_]+)/\1/p")
echo "Package = " $package

instantiate() {
    echo "Starting instantiation"

    result=$(resim call-function $package Solarix instantiate 0.1 0.2)

    export component=$(echo "$result" | sed -nr "s/.*Component: ([[:alnum:]_]+)/\1/p")
    export admin_badge_resource_address=$(echo "$result" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p")
    echo "Component = $component"
    echo "Badge Resource = $admin_badge_resource_address"
    echo "Finished instantiation."
}

create_fractionalized_asset() {
    echo "Starting create_fractionalized_asset"
    result=$(resim call-method $component create_fractionalized_asset $account2 550 15 --proofs $admin_badge_resource_address:#1#)

    export asset_tokenized_nft_resource=$(echo "$result" | sed -nr "s/.*Resource: ([[:alnum:]_]+)/\1/p")
    echo "Asset Tokenized NFT Resource Address = $asset_tokenized_nft_resource"
}

instantiate
create_fractionalized_asset

# echo "Starting buy_nft"
# # account 1 buys 1
# result=$(resim call-method $component buy_nft 0 5 $XRD:3000)

# resim set-default-account $account2 $account2_priv_key $account2_owner_badge

# # account 2 buys 14
# result=$(resim call-method $component buy_nft 0 10 $XRD:9500)

# resim show $account2

# # claim payout
# result=$(resim call-method $component claim_sales_proceeds $account2)

# resim show $account2

# resim set-default-account $account $account_priv_key $account_owner_badge

# resim call-method $component deposit_earnings 0 $XRD:5000 --proofs $admin_badge_resource_address:#1#

