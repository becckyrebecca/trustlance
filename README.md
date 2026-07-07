# LuminaHealth Clinical Operations Platform

LuminaHealth is a unified clinical operations and billing platform designed to help healthcare facilities manage patients, clinician workflows, medical encounters, and financial billing events in a single, secure, and auditable system.

The platform integrates a dedicated Stellar billing layer to handle transaction processing, receipts, and audit-friendly financial records for clinical encounters.

---

## 🌟 Platform Modules

*   **Authentication & Access Control:** Role-based access control system (`admin`, `clinician`, `support`) specifying scoped permissions per clinic facility.
*   **Clinic & Staff Management:** Multi-facility scaffolding supporting facility onboarding and staff role assignment.
*   **Patient Records:** Traceable registries for longitudinal patient context profiles and medical histories.
*   **Clinical Encounters & Workflows:** Encounter log manager tracking diagnose logs (ICD-10 codes), clinicians notes, and follow-ups.
*   **Billing & Stellar Integration:** Financial processing layer converting clinical encounters into on-chain Stellar ledger payments and transaction tracking.
*   **Mobile Workspace:** Light clinical search dashboard built for frontline workers, designed for offline caching capabilities.

---

## 🏗 System Architecture & Workspaces

LuminaHealth is structured as an npm workspaces monorepo:

| Layer / Package | Location | Technology Stack |
| :--- | :--- | :--- |
| **API** | `apps/api` | Express + TypeScript |
| **Web** | `apps/web` | Next.js (App Router) + React + TypeScript |
| **Mobile** | `apps/mobile` | React + TypeScript (Clinical mobile workspace) |
| **Stellar Service** | `apps/stellar-service` | Node.js + TypeScript + `@stellar/stellar-sdk` |
| **Shared Config** | `packages/config` | Config parameters and billing rates (`@lumina/config`) |
| **Shared Types** | `packages/types` | Clinical domain TypeScript interfaces (`@lumina/types`) |

---

## 🚦 Architecture Validation & Boundaries

The codebase enforces strict modular boundaries:
*   **Architecture Check:** Validates structure directories and configurations.
    ```bash
    npm run check:architecture
    ```
*   **Boundary Check:** Validates that UI applications do not create illegal direct imports from backend logic.
    ```bash
    npm run check:boundaries
    ```

---

## 💻 Getting Started

### Prerequisites
*   [Node.js](https://nodejs.org/) v20+
*   [npm](https://www.npmjs.com/) v10+

### Installation & Builds
1.  Install dependencies:
    ```bash
    npm install
    ```
2.  Compile all packages and applications in the workspaces:
    ```bash
    npm run build
    ```

### Running Development Servers
You can run the API, Web App, and Stellar Service in development mode simultaneously:
```bash
npm run dev
```

---

## 📄 License
This project is licensed under the [MIT License](LICENSE).
