# Solana TrustLock - Decentralized Escrow Contract

TrustLock is a decentralized escrow smart contract built on the Solana blockchain, designed to enable secure and trustless asset transfers for eCommerce orders and services. It ensures that payments are only released once predefined conditions are met, providing transparency and safety for both buyers and sellers.

## Project Status

- **Current Phase**: Smart Contract Complete + Testing Done

## Key Features

- **Escrow Mechanism**: Funds are securely held in escrow until conditions like order fulfillment and approval are met.
- **Claim Functionality**: Allows the order fulfiller to claim funds if specific conditions are met, especially in cases where a release time is not set.
- **Owner Review & Approval**: The order creator can review and approve order completion, triggering the release of assets.
- **Order Closure**: Safely closes orders after all conditions are fulfilled, ensuring a clear contract lifecycle.

## How It Works

1. **Order Creation**: A buyer initiates an escrow order by transferring funds to the contract.
2. **Fulfiller's Role**: Once the fulfiller delivers the order, they update the order status to fulfill the delivery requirement.
3. **Approval and Release**: The order creator reviews the order and, upon satisfaction, approves it to release funds.
4. **Claiming Funds**: If the release conditions aren't fully set, the fulfiller has the option to claim the funds after fulfilling certain criteria.

## Development Roadmap

- [x] Implement core escrow logic
- [x] Code review and testing for smart contracts
- [ ] Frontend development for user interaction
- [ ] Fuzz testing for edge case resilience

## Usage Guide

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/preetsinghmakkar/TrustLock.git
   cd TrustLock
   ```

2. **Deploy the Smart Contract**:

   - Follow the Solana program deployment steps, ensuring to update keys and PDAs in your configuration.

3. **Run Tests**:

   - Run `anchor test` to ensure all functionalities are working correctly.

4. **Frontend (Coming Soon)**:
   - The frontend interface will provide user-friendly interactions for creating, approving, and claiming orders.

## License

This project is licensed under the MIT License.
