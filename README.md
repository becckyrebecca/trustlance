# NovaHealth Clinical Operations Platform

NovaHealth is a clinical operations platform designed to help healthcare facilities manage patients, staff workflows, encounters, and billing in a unified system.

The platform combines web, mobile, and backend services with a dedicated Stellar integration layer for financial and billing workflows, enabling a modern healthcare system that is structured, auditable, and scalable.

NovaHealth is built as a monorepo to support fast iteration across clinical, administrative, and operational features.

---

## 🌟 Overview
Healthcare systems often operate across fragmented tools — patient records in one system, scheduling in another, billing handled separately, and mobile workflows poorly integrated.

NovaHealth addresses this by providing a unified platform for:
*   clinic and staff management,
*   patient records and histories,
*   clinical encounters and workflows,
*   billing and payment processing,
*   mobile-first access for healthcare workers,
*   structured, auditable data across all operations.

The system is designed to support both administrative and frontline clinical use cases.

---

## 🏗 Platform Modules

### Authentication & Access Control
NovaHealth begins with a secure authentication and role system. It supports:
*   staff login and identity verification,
*   role-based access (`admin`, `clinician`, `support` staff),
*   secure session management,
*   scoped permissions per clinic or organization.

This forms the foundation for all clinical and administrative actions.

### Clinic & Staff Management
The platform supports structured clinic organization management. Capabilities include:
*   clinic onboarding,
*   staff assignment and roles,
*   multi-clinic support,
*   access scoping per facility,
*   administrative oversight tools.

This ensures healthcare environments can be modeled accurately within the system.

### Patient Records
Patient data is central to NovaHealth. The system provides:
*   patient profiles,
*   medical history tracking,
*   visit records,
*   structured clinical data storage,
*   longitudinal patient context,
*   secure access controls.

All patient data is designed to be consistent, traceable, and easy to extend.

### Encounters & Clinical Workflows
Encounters represent interactions between clinicians and patients. The system supports:
*   visit creation and updates,
*   diagnosis and notes,
*   treatment workflows,
*   structured encounter timelines,
*   follow-up tracking,
*   clinician collaboration.

This creates a clear record of clinical activity over time.

### Billing & Stellar Integration
NovaHealth uses Stellar to support financial workflows within healthcare operations. The Stellar service enables:
*   billing event processing,
*   payment tracking,
*   transaction receipts,
*   audit-friendly financial records,
*   integration between clinical actions and billing events.

This provides a transparent and programmable layer for healthcare billing workflows.

### Mobile & Offline Support
The mobile workspace is designed for frontline healthcare environments. It supports:
*   patient lookup and access,
*   encounter documentation,
*   offline-first workflows (future phase),
*   sync when connectivity is restored,
*   lightweight clinical data entry.

This ensures clinicians can operate even in low-connectivity environments.

---

## 🏗 System Architecture & Workspaces

NovaHealth is built as a strict monorepo with clear boundaries between services.

| Layer / Package | Location | Technology Stack |
| :--- | :--- | :--- |
| **API** | `apps/api` | Express + TypeScript |
| **Web** | `apps/web` | Next.js (App Router) + React + TypeScript |
| **Mobile** | `apps/mobile` | React + TypeScript (Clinical mobile workspace) |
| **Stellar Service** | `apps/stellar-service` | Node.js + TypeScript + `@stellar/stellar-sdk` |
| **Shared Config** | `packages/config` | Config parameters and billing rates (`@novahealth/config`) |
| **Shared Types** | `packages/types` | Clinical domain TypeScript interfaces (`@novahealth/types`) |

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
