# Soroban Escrow Contract Deployment Guide

This guide details the steps to build, deploy, initialize, and integrate the `trustlance-escrow-contract` on the Stellar Testnet using the Soroban CLI.

---

## 🛠️ Prerequisites

1.  **Rust Toolchain:** installed via `rustup` (with target `wasm32-unknown-unknown` enabled).
2.  **Soroban CLI:** v21.0.0 or later installed:
    ```bash
    cargo install --locked soroban-cli
    ```
3.  **Stellar Testnet Account:** Generate and fund a Testnet account via Friendbot:
    ```bash
    soroban keys generate deployer --network testnet
    ```

---

## 🏗️ 1. Build the Smart Contract

To compile the contract to WASM:
```bash
cargo build --target wasm32-unknown-unknown --release
```
The output WASM file will be located at:
`target/wasm32-unknown-unknown/release/trustlance_escrow_contract.wasm`

---

## 🚀 2. Deploy to Stellar Testnet

Deploy the compiled WASM binary to the network:
```bash
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/trustlance_escrow_contract.wasm \
  --source deployer \
  --network testnet
```
This command outputs the **Contract ID** (e.g. `CC...`). Save this value as `CONTRACT_ID`.

---

## 🏁 3. Initialize the Escrow Job

Use the Soroban CLI to initialize the contract. You must specify the address for the **client**, **freelancer**, **arbiter**, **token** (e.g., native asset token ID), the list of **milestone amounts** (in raw units), and the **auto-release window** (in seconds):

```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source deployer \
  --network testnet \
  -- \
  initialize \
  --client "GB_CLIENT_ADDRESS..." \
  --freelancer "GB_FREELANCER_ADDRESS..." \
  --arbiter "GB_ARBITER_ADDRESS..." \
  --token "CAS_TOKEN_ADDRESS..." \
  --milestone_amounts '[100000000, 200000000]' \
  --auto_release_window 86400
```
*(Here `86400` seconds represents a 24-hour auto-release safety window).*

---

## 💸 4. Fund the Escrow Job

The client must call `fund` to transfer the total milestones sum from their balance into the escrow contract.
*Note: The client must authorize the contract to pull the funds.*

```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source client \
  --network testnet \
  -- \
  fund
```

---

## 🚚 5. Clinical Workflow & Action Commands

### Mark Milestone Delivered (Call by Freelancer)
Once a milestone is complete, the freelancer marks it as delivered:
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source freelancer \
  --network testnet \
  -- \
  mark_delivered \
  --milestone_index 0
```

### Approve and Release Payment (Call by Client / Auto-release)
The client releases funds once verified, or it can be called by anyone after the auto-release window has expired:
```bash
soroban contract invoke \
  --id $CONTRACT_ID \
  --source client \
  --network testnet \
  -- \
  approve_milestone \
  --milestone_index 0
```

### Dispute Mediation (Arbiter Intervention)
1.  **Raise Dispute:** Either client or freelancer can flag a milestone:
    ```bash
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source client \
      --network testnet \
      -- \
      raise_dispute \
      --caller "GB_CLIENT_ADDRESS..." \
      --milestone_index 0
    ```
2.  **Arbiter Resolution:** The designated arbiter resolves the dispute. If `favor_freelancer` is `true`, funds release to the freelancer; if `false`, they refund back to the client:
    ```bash
    soroban contract invoke \
      --id $CONTRACT_ID \
      --source arbiter \
      --network testnet \
      -- \
      resolve_dispute \
      --milestone_index 0 \
      --favor_freelancer true
    ```
