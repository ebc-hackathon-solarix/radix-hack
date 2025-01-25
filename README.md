# Radix Hackathon Project

Welcome to the Radix Hackathon Project! This project leverages the Radix blockchain to simplify full-stack development by providing a robust framework for creating decentralized applications (dApps) with ease.

## Overview

The project is built using the Radix platform, which is designed to make blockchain development more accessible and efficient. It includes a smart contract module called `Solarix`, which facilitates the creation and management of fractionalized assets in the form of NFTs (Non-Fungible Tokens).

## Features

- **Fractionalized Asset Creation**: Easily create fractionalized assets with specified total supply and price per NFT.
- **NFT Purchase and Management**: Buy NFTs and manage them through a secure vault system.
- **Earnings and Fees Management**: Deposit and claim earnings, with automatic fee deductions for protocol maintenance.
- **Sales Proceeds and Fee Claims**: Claim sales proceeds and protocol fees securely.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/) programming language
- [Scrypto](https://docs.radixdlt.com/main/scrypto/introduction.html) for Radix smart contract development

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/radix-hackathon.git
   cd radix-hackathon
   ```

2. Build the project:

   ```bash
   scrypto build
   ```

3. Deploy the smart contract:

   ```bash
   scrypto publish
   ```

## Usage

The `Solarix` module provides several public methods for interacting with the smart contract:

- `instantiate`: Initializes the contract with specified fees.
- `create_fractionalized_asset`: Creates a new fractionalized asset.
- `buy_nft`: Allows users to purchase NFTs.
- `deposit_earnings`: Deposits earnings into the contract.
- `claim_earnings`: Claims earnings for a specific NFT.
- `claim_sales_proceeds`: Claims sales proceeds for a panel.
- `claim_fees`: Claims protocol fees.
- `get_available_nfts`: Retrieves available NFTs for a panel.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request with your changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.