# 🚀 TrustLance
### Freelance with Confidence. Get Paid Securely.

TrustLance is a Web3-powered freelancing platform built on the Stellar blockchain. It protects both freelancers and clients using blockchain-based escrow payments, milestone tracking, and transparent dispute resolution.

By leveraging Stellar's fast and low-cost network, TrustLance ensures that payments are secure, transparent, and instant, removing the need for traditional intermediaries and high platform fees.

---

## 🌐 Project Features

*   **Trustless Escrow:** Payments are held in smart contracts and only released when milestones are met.
*   **Instant Payouts:** Once approved, funds are transferred instantly via the Stellar network.
*   **Zero Platform Fees:** We believe freelancers should keep what they earn.
*   **Transparent Track Record:** All reviews and project histories are stored on the blockchain, ensuring a verified reputation system.

---

## 🛠️ Tech Stack

### Frontend & UI
*   **Framework:** Next.js 15 (App Router)
*   **Styling:** Tailwind CSS
*   **UI Components:** Radix UI & custom primitives
*   **Icons:** Lucide React
*   **Forms/Validation:** React Hook Form & Zod

### Backend & Database
*   **Database:** Neon Postgres (Serverless)
*   **Client/ORM:** `@neondatabase/serverless`

### Blockchain (Stellar)
*   **Contracts:** Soroban Smart Contracts (Rust)
*   **WASM Builder:** Cargo wasm32 targets

---

## 📂 Folder Structure
```
trust_lance/
├── app/                              # Next.js App Router pages and global styles
├── components/                       # Reusable UI components
├── contracts/
│   └── trustlance-escrow-contract/   # Soroban milestone escrow contract (Rust)
├── docs/                             # Contract deployment and configuration guides
├── lib/                              # Shared database and utility configurations
├── public/                           # Static assets (images, icons, etc.)
├── scripts/                          # Database migrations (Postgres tables setup)
├── styles/                           # Global CSS and theme configurations
├── .env.example                      # Template for database environmental variables
├── Cargo.toml                        # Rust workspace configuration
├── package.json                      # Next.js workspace scripts and dependencies
└── tsconfig.json                     # TypeScript compiler configuration
```

---

## 🚀 Getting Started

### Prerequisites
*   [Node.js](https://nodejs.org/) v18+
*   [Rust](https://www.rust-lang.org/) and cargo toolchain (target `wasm32-unknown-unknown` enabled)
*   [Neon Database Account](https://neon.tech/)

### Installation
1.  **Clone the repository:**
    ```bash
    git clone https://github.com/becckyrebecca/novahealth.git TrustLance
    cd TrustLance
    ```
2.  **Install dependencies:**
    ```bash
    npm install
    ```
3.  **Set up environment variables:** Copy `.env.example` to `.env` and fill in your database credentials:
    ```bash
    cp .env.example .env
    ```
4.  **Initialize the database:** Execute the SQL script in `scripts/001-create-tables.sql` against your Neon database instance.
5.  **Run the development server:**
    ```bash
    npm run dev
    ```

---

## 🧪 Smart Contract Testing

To verify contract execution:
```bash
cargo test
```

---

## 🛣️ Roadmap
*   [x] **Phase 1:** Core landing page and mockup escrow dashboard
*   [x] **Phase 2:** Soroban milestone escrow smart contract development
*   [ ] **Phase 3:** Live Freighter Wallet authentication & signing integration
*   [ ] **Phase 4:** Dispute resolution UI & DAO governance
*   [ ] **Phase 5:** Freelancer verified reputations on-chain

---

## 🤝 Contributing
Contributions are always welcome:
1.  Fork the repository.
2.  Create a feature branch (`git checkout -b feature/name`).
3.  Commit changes (`git commit -m 'add feature'`).
4.  Push to branch (`git push origin feature/name`).
5.  Open a Pull Request.

---

## 📄 License
This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
