# NovaHealth System Architecture

NovaHealth is designed as a modular, structured monorepo using npm workspaces to maintain clean separations of concerns between different services, applications, and shared domain libraries.

## 🏗 Directory Structure
```
novahealth/
│
├── apps/
│   ├── api/                # Core backend API (Express + TypeScript)
│   ├── web/                # Admin & clinic web application (Next.js App Router)
│   ├── mobile/             # Clinical mobile workspace (React Native)
│   └── stellar-service/    # Billing & Stellar Network integration service
│
├── packages/
│   ├── config/             # Shared runtime configuration, billing rates, and roles
│   └── types/              # Shared domain TypeScript contracts and interfaces
│
└── docs/               # System architecture and roadmap documentation
```

---

## 🔒 Layered Architecture & Service Boundaries

Each service in the monorepo operates with strict service boundaries:

### 1. API Backend (`apps/api`)
Acts as the central router for clinical operations, patient registries, and clinical encounters. It is isolated from the blockchain/billing processing logic. To trigger billing, it communicates with the `stellar-service` asynchronously or via scoped API calls.

### 2. Stellar Service (`apps/stellar-service`)
Integrates directly with the Stellar Network (Horizon RPCs). It manages keypair operations, constructs payment/billing transactions using `@stellar/stellar-sdk`, and handles on-chain verification. No UI packages or Express API components should directly reference Stellar secrets; all operations run through this isolated microservice.

### 3. Web & Mobile Client Workspaces (`apps/web` & `apps/mobile`)
Web (Next.js) and Mobile (React Native) are strictly client-side presentation layers. They import configuration data and type structures from shared packages but have **zero direct dependencies** on backend services (`api` or `stellar-service`) source code.

---

## 🚦 Architectural Enforcement
NovaHealth enforces boundary separations via local validation tools:
*   **Architecture Check:** `npm run check:architecture` validates that the required project modules and configurations exist.
*   **Boundary Check:** `npm run check:boundaries` parses files in client workspaces to ensure there are no illegal direct imports from `apps/api` or `apps/stellar-service`.

---

## 🔗 Shared Packages
To avoid code duplication and import issues:
*   `@novahealth/config` exports a global config object `RUNTIME_CONFIG` for session rules, consultation fees, and roles.
*   `@novahealth/types` exports type interfaces for `User`, `Clinic`, `Patient`, `Encounter`, and `BillingRecord`.
