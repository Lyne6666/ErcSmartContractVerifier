# ErcSmartContractVerifier

## Description

This repository houses a novel NFT minting and management platform leveraging Merkle tree-based whitelisting for gas-efficient airdrops and on-chain royalty enforcement via dynamically generated, SVG-based metadata stored directly on the Ethereum blockchain.

## Features

- Deploys bytecode analysis to identify potential vulnerabilities like reentrancy and integer overflow.
- Integrates with Truffle and Hardhat development environments via a command-line interface.
- Generates a detailed report comparing the deployed bytecode with the source code's compiled bytecode, highlighting discrepancies.
- Verifies contract source code on multiple Ethereum Virtual Machine (EVM) compatible blockchains using standardized API endpoints.
- Supports various compiler versions (solc, vyper) and optimization levels to ensure accurate verification.
- Utilizes a Merkle tree-based data structure to efficiently store and compare bytecode hashes.
- Provides a REST API for programmatic verification requests, allowing integration with CI/CD pipelines.
- Implements a caching mechanism to reduce redundant bytecode analysis and improve verification speed.
## Installation

```bash
pip install git+https://github.com/Lyne6666/ErcSmartContractVerifier.git
```

## Usage

```bash
python -m ercsmartcontractverifier --verbose
```

## Contributing

We welcome contributions! Here's how to get started:

1. Fork this repository
2. Create a new branch for your feature (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add some awesome feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Open a Pull Request

## License

Distributed under the MIT License. See `LICENSE` for more information.
