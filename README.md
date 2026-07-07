# Stellar Milestone Escrow Contract

[![CI](https://github.com/becckyrebecca/STELLARR/actions/workflows/ci.yml/badge.svg)](https://github.com/becckyrebecca/STELLARR/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A production-ready Soroban smart contract on the Stellar network that enables trustless, milestone-based escrow payments between a **Client**, a **Freelancer**, and a designated **Arbiter**.

---

## 🌟 Overview

The contract allows a **Client** to set up a job divided into multiple **Milestones**. The total funding is locked inside the contract when the job is funded. Funds for each individual milestone are released to the **Freelancer** only upon client approval, or automatically when a pre-configured **Auto-Release Window** expires. In case of disagreement, either party can raise a dispute, which is resolved by an independent **Arbiter**.

### Key Features
*   **Multi-Milestone Jobs:** Supports setting up jobs with arbitrary numbers of milestones, each with custom payment amounts.
*   **Security & Safety:** All input parameters (addresses, milestone amounts, and auto-release windows) are strictly validated during initialization to prevent contract misconfiguration.
*   **Auto-Release Mechanism:** Prevents funds from being locked indefinitely if the client disappears after milestone delivery.
*   **Built-in Arbitration:** Fully decentralized dispute-resolution flow handled by a designated arbiter address.
*   **Comprehensive Testing:** 100% test coverage for all success scenarios and error handling with full execution state verification.

---

## 🛠 Contract Functions

| Function | Caller | Description |
| :--- | :--- | :--- |
| `initialize` | Anyone | Set up job with client, freelancer, arbiter, token addresses, milestone amounts, and auto-release window. |
| `fund` | Client | Deposit the total job amount into the contract. |
| `mark_delivered` | Freelancer | Mark a milestone index as delivered. |
| `approve_milestone` | Client / Freelancer* | Release milestone funds. Early release requires Client signature. After the auto-release window passes, client signature is not required. |
| `raise_dispute` | Client or Freelancer | Freeze a milestone index for arbitration. |
| `resolve_dispute` | Arbiter | Resolve a disputed milestone, releasing funds to the Freelancer or refunding the Client. |
| `get_job` | Anyone | View current job state (participants, milestone states, funding status, etc.). |

---

## 🔄 State Machine & Flows

### Milestone States
```
         +---------+
         | Pending |
         +----+----+
              |
              | (mark_delivered)
              v
        +-----------+
        | Delivered |
        +--+---+----+
           |   |
           |   | (approve_milestone / auto-release window expires)
           |   v
           | +----------+
           | | Released | (Funds -> Freelancer)
           | +----------+
           |
           | (raise_dispute by Client/Freelancer)
           v
        +----------+
        | Disputed |
        +----+-----+
             |
             +-----------------------+
             | (favor_freelancer)    | (favor_client)
             v                       v
       +----------+            +----------+
       | Released |            | Refunded | (Funds -> Client)
       +----------+            +----------+
```

---

## ⚠️ Contract Error Codes

The contract uses standard Rust `contracterror` representation and returns explicit errors for incorrect operations:

| Code | Error Name | Cause |
| :--- | :--- | :--- |
| `1` | `AlreadyInitialized` | Contract has already been initialized. |
| `2` | `NotInitialized` | Contract functions called prior to initialization. |
| `3` | `EmptyMilestones` | Rejects empty milestone amount lists on initialization. |
| `4` | `InvalidMilestoneAmount` | Rejects zero or negative milestone amounts. |
| `5` | `ZeroAutoReleaseWindow` | Rejects `0` value for the auto-release window parameter. |
| `6` | `ZeroAddress` | Rejects zero-bytes/dummy address inputs. |
| `7` | `NotFunded` | Action attempted on an unfunded job. |
| `8` | `AlreadyFunded` | Client attempts to fund the contract twice. |
| `9` | `InvalidMilestoneIndex` | Milestone index is out of bounds. |
| `10` | `InvalidMilestoneState` | Action attempted on a milestone in an invalid state. |
| `11` | `Unauthorized` | Caller is not authorized to perform the operation. |

---

## 💻 Developer Setup & Installation

### Prerequisites
*   [Rust Toolchain](https://rustup.rs/) (v1.79+)
*   Target `wasm32-unknown-unknown` installed:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
*   (Optional) Stellar CLI:
    ```bash
    cargo install --locked stellar-cli --features opt
    ```

### Compilation
Build the contract into WebAssembly (`.wasm`) format from the root of the workspace:
```bash
cargo build --release --target wasm32-unknown-unknown -p stellar-milestone-escrow
```
The optimized WebAssembly binary will be generated under:
`target/wasm32-unknown-unknown/release/stellar_milestone_escrow.wasm`

### Unit Testing
Run the comprehensive test suite to verify contract workflows:
```bash
cargo test -p stellar-milestone-escrow
```

---

## 🚀 Deployment Guide (Testnet)

You can deploy the compiled WASM binary using the Stellar CLI:

1.  **Add Testnet network configuration:**
    ```bash
    stellar network add \
      --global testnet \
      --rpc-url https://soroban-testnet.stellar.org:443 \
      --network-passphrase "Test Stellar Network ; September 2015"
    ```

2.  **Generate a deployment identity:**
    ```bash
    stellar keys generate --global deployer --network testnet
    ```

3.  **Deploy the contract:**
    ```bash
    stellar contract deploy \
      --wasm target/wasm32-unknown-unknown/release/stellar_milestone_escrow.wasm \
      --network testnet \
      --source deployer
    ```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).