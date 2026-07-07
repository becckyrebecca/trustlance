# Contributing to Stellar Milestone Escrow

First off, thank you for taking the time to contribute! 🎉 We welcome contributions from everyone to help make this contract more robust, secure, and useful for the Stellar ecosystem.

This document provides a set of guidelines and instructions to help you get started with contributing.

---

## 🗺 Table of Contents
1. [Code of Conduct](#code-of-conduct)
2. [How Can I Contribute?](#how-can-i-contribute)
3. [Development Environment Setup](#development-environment-setup)
4. [Coding Standards & Guidelines](#coding-standards--guidelines)
5. [Testing & Verification](#testing--verification)
6. [Submitting a Pull Request](#submitting-a-pull-request)

---

## 🤝 Code of Conduct
We expect all contributors to adhere to a respectful, welcoming, and harassment-free community environment. Please be constructive and polite in all issues, pull requests, and discussions.

---

## 💡 How Can I Contribute?

*   **Reporting Bugs:** Open an issue describing the bug, including steps to reproduce, expected behavior, and host/environment details.
*   **Suggesting Enhancements:** Open an issue describing the feature, why it is useful, and potential implementation paths.
*   **Submitting Pull Requests:** Fix open issues or implement features. Please link your PR to the corresponding issue.

---

## 🛠 Development Environment Setup

To compile and run tests for this Soroban smart contract, you will need the following tools:

1.  **Rust Toolchain:** Install Rust (v1.79.0+) via [rustup](https://rustup.rs/):
    ```bash
    rustup update stable
    ```
2.  **WebAssembly Target:** Add the `wasm32-unknown-unknown` target:
    ```bash
    rustup target add wasm32-unknown-unknown
    ```
3.  **Soroban/Stellar CLI:** (Optional, for deploying and CLI interactions):
    ```bash
    cargo install --locked stellar-cli --features opt
    ```

---

## 📏 Coding Standards & Guidelines

To ensure the smart contract remains solid and review processes are smooth, please follow these rules:

*   **Documentation:** Maintain clear comments and documentation. Keep `README.md` updated if you change function signatures or behavior.
*   **Crate Naming:** Keep crate names aligned with workspace configuration (`stellar-milestone-escrow`).
*   **Formatting:** All Rust files must be formatted with the standard settings. Format before committing:
    ```bash
    cargo fmt --all
    ```
*   **Lints & Quality:** All code must compile with clippy with **zero warnings**. Run clippy with strict warning flags:
    ```bash
    cargo clippy --all-targets --all-features -- -D warnings
    ```
*   **Security Principles:** 
    *   Verify all caller permissions via `Address::require_auth()`.
    *   Explicitly handle errors using the contract's custom `ContractError` enum.
    *   Sanitize all numbers to prevent overflow/underflow or negative amounts.

---

## 🧪 Testing & Verification

Every functional change or bug fix **must** be accompanied by unit tests in `src/test.rs`.

*   **Run local unit tests:**
    ```bash
    cargo test --all
    ```
*   **Compile to WebAssembly:** Ensure that the contract compiles cleanly to guest WASM target before submitting:
    ```bash
    cargo build --release --target wasm32-unknown-unknown --all
    ```

---

## 🚀 Submitting a Pull Request

1.  **Fork the Repository:** Create a fork of the repository and clone it locally.
2.  **Create a Branch:** Create a branch for your changes:
    ```bash
    git checkout -b feature/my-cool-feature
    ```
3.  **Make Changes:** Implement your feature or bug fix.
4.  **Verify & Format:** Run formatting checks, clippy, and unit tests to ensure everything is passing.
5.  **Commit Changes:** Commit using clear, descriptive commit messages:
    ```bash
    git commit -m "feat: add auto-dispute expiry mechanism"
    ```
6.  **Push and Open PR:** Push to your fork and submit a Pull Request to the `main` branch. Provide a detailed summary of your changes and reference any related issues.

Thank you for contributing to the Stellar open-source community!
