# Contributing to TrustLance

We welcome open-source contributions from the developer community! Follow this guide to ensure your contributions meet project standards and pass verification pipelines.

---

## 🛠️ Development Setup

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/becckyrebecca/novahealth.git
    cd novahealth
    ```
2.  **Install Node.js dependencies:**
    ```bash
    npm install
    ```
3.  **Compile Next.js build:**
    ```bash
    npm run build
    ```
4.  **Run Soroban unit tests:**
    ```bash
    cargo test
    ```

---

## 🚦 Architectural Enforcement & Guidelines

To ensure the platform is robust, secure, and ready for Stellar open-source submissions:
*   **Contract Integrity:** Do not change smart contract layouts or interfaces without adding accompanying unit tests.
*   **Types & Validations:** Always validate user inputs via React Hook Form and Zod schemas before initiating any transaction or database query.
*   **Clean PR History:** Keep commits grouped cleanly and provide clear descriptions of changes.

---

## 📝 Committing Code

*   Always work in a scoped feature branch (`feature/wallet-auth`, `bugfix/contract-release`).
*   Ensure that all linting passes locally (`npm run lint`) and that both Next.js compiling and Cargo tests succeed before submitting a Pull Request.
