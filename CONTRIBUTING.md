# Contributing to LuminaHealth

Thank you for your interest in contributing to LuminaHealth! This clinical operations and billing platform is built as a strict monorepo. Please read this guide to understand how to get started and how to maintain the codebase standards.

---

## 🛠 Prerequisites

*   **Node.js:** v20 or newer
*   **npm:** v10 or newer

---

## 🚀 Setting Up the Development Workspace

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/becckyrebecca/stellar-milestone-escrow.git LuminaHealth
    cd LuminaHealth
    ```
2.  **Install dependencies:**
    ```bash
    npm install
    ```
3.  **Compile shared packages and apps:**
    ```bash
    npm run build
    ```

---

## 🚦 Architecture & Boundary Rules

To keep our patient data safe and clinical operations decoupled from financial ledger layers:
1.  **Shared config and types only:** Use `@lumina/config` and `@lumina/types` to share values across client-side and server-side applications.
2.  **No direct backend dependencies:** UI projects (`apps/web` or `apps/mobile`) must **never** import source files from backend services (`apps/api` or `apps/stellar-service`).
3.  **Pre-commit Verification:** Before submitting a Pull Request, you must run the validation checks and ensure they pass successfully:
    ```bash
    npm run check:architecture
    npm run check:boundaries
    ```

---

## 📝 Committing Code

*   Always use descriptive branch names (`feature/onboard-patient`, `bugfix/stellar-tx-timeout`).
*   Ensure that compilation (`npm run build`) is successful and does not generate type errors or warnings.
