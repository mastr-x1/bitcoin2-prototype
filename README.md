
# Bitcoin 2 — Post-Quantum Digital Currency

![Bitcoin 2 Architecture](https://raw.githubusercontent.com/example/bitcoin2-prototype/main/docs/architecture-diagram.png)

---

## 📌 Introduction

Bitcoin 2 is a post-quantum secure cryptocurrency built from the ground up using Rust and Python. It is designed to overcome the limitations of traditional blockchains like Bitcoin by providing:

- Quantum-resistant cryptographic primitives
- Fair mining accessible to any device (PC, mobile)
- A lightweight and scalable decentralized architecture

---

## 🎯 Purpose and Vision

The original Bitcoin opened the door to decentralized digital currency. However, it is not built to withstand quantum computing, and mining has become centralized and energy-intensive.

**Bitcoin 2** aims to:
- Provide a fair mining opportunity for everyone
- Ensure security in the post-quantum era using Dilithium signatures
- Enable scalable and fast transactions through efficient consensus and networking

---

## ⚙️ Architecture Overview

Bitcoin 2 architecture is modular:

```
bitcoin2-prototype/
├── src/
│   ├── blockchain/     # Core blockchain logic
│   ├── consensus/      # Mining (RandomX)
│   ├── crypto/         # Post-quantum cryptography (Dilithium)
│   ├── network/        # P2P networking
│   └── node/           # Node interface and CLI
├── scripts/            # Setup & execution scripts
├── tests/              # Testing modules
```

![Architecture Diagram](https://raw.githubusercontent.com/example/bitcoin2-prototype/main/docs/architecture.png)

---

## 🔐 Security Features

- **Quantum Resistance:** Uses Dilithium post-quantum digital signatures (via liboqs)
- **Decentralized Mining:** Powered by RandomX — CPU-friendly, ASIC-resistant
- **P2P Network:** Libp2p-powered, decentralized node discovery & communication

---

## 🚀 Installation & Setup

### ✅ Requirements
- OS: Linux (Ubuntu 20.04+ recommended)
- Rust 1.60+
- Python 3.8+
- RAM: 2GB minimum

### 📦 Setup
```bash
git clone https://github.com/YOUR_USERNAME/bitcoin2-prototype.git
cd bitcoin2-prototype
./scripts/setup.sh
```

### 🔄 Start Node
```bash
./scripts/start_node.sh
```

### 🧪 Test Network
```bash
./scripts/test_network.sh
```

### 🖥️ Command-Line Interface
```bash
python scripts/cli.py
```

---

## 💡 Key Features

| Feature                   | Description                                           |
|---------------------------|-------------------------------------------------------|
| Quantum-safe signatures   | Dilithium-based, via liboqs                          |
| Accessible mining         | RandomX, runs on any CPU                             |
| P2P decentralized network | libp2p protocol stack                                |
| Fast block time           | 2-minute average block creation                      |
| Clean CLI                 | Easy to use Python-based interface                   |
| Modular Design            | Expandable via Rust modules or Python wrappers       |

---

## 🛣️ Roadmap

- ✅ Prototype release
- 🔄 GUI Wallet interface
- 🔒 Hardware wallet integration
- 🌐 Bridge to other blockchains (via cross-chain tech)
- 📱 Mobile mining clients

---

## 📜 License

This project is licensed under the **MIT License**. See `LICENSE` for details.

---

## 🔗 Useful Links

- GitHub Repository: [github.com/YOUR_USERNAME/bitcoin2-prototype](https://github.com/mastr-x1/bitcoin2-prototype)
- OQS Library: [https://openquantumsafe.org](https://openquantumsafe.org)
- Libp2p Networking: [https://libp2p.io](https://libp2p.io)

---

> ⚠️ Disclaimer: This is a research prototype. Not intended for production deployment yet.
