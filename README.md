# Sanctum Cranker

A Rust-based automation tool for cranking [Sanctum Reserve](https://www.sanctum.so/) pools on the Solana blockchain using a CLI interface. It is designed to simplify and automate the process of sending cranking transactions using Solana RPC and a designated wallet.

---

## 🧪 Requirements

- Rust (latest stable version)
- Solana CLI (for local key management, optional)
- Docker (required, for containerized usage and production deployment)

---

## 🔧 Environment Configuration

Create a `.env` file or export the following environment variables:

```env
PORT=3000
RPC_URL="https://api.mainnet-beta.solana.com"
PAYER_PRIVATE_KEY=''  # Your Solana wallet's private key (Base64 or JSON)
```

<!-- ```` -->

> ⚠️ **Do not expose your `PAYER_PRIVATE_KEY` in public repos.** Use secrets managers or environment variables for secure deployment.

---

## 📁 Project Structure

```
.
├── Cargo.lock               # Cargo dependency lock file
├── Cargo.toml               # Package manifest
├── Dockerfile               # Docker container configuration
├── nixpacks.toml            # Nixpacks config for deployment
├── README.md                # Project documentation
└── src
    ├── config.rs            # Loads and validates environment config
    ├── main.rs              # CLI entry point
    ├── routes               # (Planned) API routes
    │   ├── cranker_route.rs
    │   └── mod.rs
    ├── services             # Core logic for cranking
    │   ├── cranker_service.rs
    │   └── mod.rs
    ├── tx_utils.rs          # Utility functions for Solana transactions
    └── update.rs            # State update and refresh logic
```

---

## 🛠️ Building & Running

### Build Locally

```bash
cargo build --release
```

### Run with CLI

```bash
PORT=3000 \
RPC_URL="https://api.mainnet-beta.solana.com" \
PAYER_PRIVATE_KEY='<your-private-key>' \
cargo run --release
```

---

## 🐳 Docker Support

### Build Docker Image

```bash
docker build -t sanctum-cranker .
```

### Run Docker Container

```bash
docker run -e PORT=3000 \
           -e RPC_URL="https://api.mainnet-beta.solana.com" \
           -e PAYER_PRIVATE_KEY='<your-private-key>' \
           sanctum-cranker
```

---

## 🧭 Roadmap

- [x] RESTful API server interface
- [ ] Retry logic and error reporting
- [ ] Unit and integration tests

---

## 🤝 Contributing

We welcome contributions from the community! To get started:

1. Fork the repo
2. Create your feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push to the branch (`git push origin feature/your-feature`)
5. Open a pull request

---

## 📝 License

This project is licensed under the [Apache 2.0 License](LICENSE).

---

## 👨‍💻 Author

Built and maintained by the Brew Labs team.
