# Solana Game API (Rust)

## Overview
SOLANA_GAME_API_RUST is a backend application written in Rust using the Actix Web framework. It provides an API to interact with a Solana-based program. The API integrates Swagger for documentation and testing of Solana instructions.
This project is currently **in progress** and under active development.

## Features
- Exposes endpoints to execute instructions on the Solana blockchain.
- Provides Swagger documentation for easy testing and interaction.
- Utilizes Actix Web for high-performance backend services.
- Modular design for easy extension and maintenance.

## Project Structure
```
SOLANA-GAME-API-RUST
├── src
│   ├── config
│   ├── controllers
│   │   ├── game_controller.rs
│   │   ├── solana_controller.rs
│   │   ├── mod.rs
│   ├── models
│   │   ├── mod.rs
│   ├── services
│   │   ├── game_service.rs
│   │   ├── solana_service.rs
│   │   ├── mod.rs
│   ├── main.rs
├── target
├── wallets
├── .env
├── .gitignore
├── Cargo.lock
├── Cargo.toml
└── README.md
```

## Solana Program Instructions
The API interacts with a Solana program designed for escrow-based gambling, and its key instructions include:
- **close_game.rs**: Closes the game and cleans up resources.
- **create_game.rs**: Initializes a new game.
- **fetch_price.rs**: Fetches the price of assets.
- **join_game.rs**: Allows a user to join an existing game.
- **settle_game.rs**: Finalizes the game and distributes rewards.
- **withdraw_funds.rs**: Allows users to withdraw their funds.

## Requirements
- Rust (latest stable version)
- Cargo (Rust's package manager)
- Solana CLI
- Actix Web framework
- Swagger integration

## Setup
1. **Clone the Repository:**
   ```bash
   git clone https://github.com/dariusjvc/solana-game-api-rust.git
   cd SOLANA-GAME-API-RUST
   ```

2. **Install Dependencies:**
   Ensure you have Rust installed. If not, [install Rust here](https://www.rust-lang.org/tools/install):
   ```bash
   cargo build
   ```

3. **Configure Environment Variables:**
   Create a `.env` file in the root directory and configure the following:
   ```env
   SOLANA_RPC_URL=https://api.devnet.solana.com
   WALLET_PATH=/path/to/wallet.json
   ```

4. **Run the API:**
   ```bash
   cargo run
   ```

5. **Access Swagger Documentation:**
   Open your browser and navigate to:
   ```
   http://localhost:8080/swagger-ui/#
   ```

## Usage
The API provides endpoints for all the Solana program instructions:
- **POST /api/game/create**: Triggers the `create_game` instruction.
- **POST /api/game/join**: Triggers the `join_game` instruction.
- **POST /api/game/settle**: Triggers the `settle_game` instruction.
- **POST /api/game/close**: Triggers the `close_game` instruction.
- **POST /api/funds/withdraw**: Triggers the `withdraw_funds` instruction.
- **GET /api/price**: Fetches price details from the Solana program.

Refer to the Swagger documentation for more details about the request and response structures.

## Contributing
Contributions are welcome! Please follow the steps below:
1. Fork the repository.
2. Create a new branch.
3. Make your changes and commit them.
4. Push to your fork and submit a pull request.

