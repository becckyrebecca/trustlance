# NovaHealth MVP Scope

The platform development is broken down into structured, clinical-centric milestones to ensure progressive hardening, auditability, and safety before full-scale deployment.

---

## 📅 MVP Delivery Plan

### Milestone 1: Authentication & Access Control
*   Implement role-based access controls (`admin`, `clinician`, `support`).
*   Establish secure verification tokens and session boundaries based on `@novahealth/config` rules.

### Milestone 2: Clinic & Staff Management
*   Enable onboarding of multi-facility environments.
*   Establish scoped permissions per clinic to restrict clinician oversight to their designated clinics.

### Milestone 3: Patient Records System
*   Deploy a consistent longitudinal patient context profile.
*   Secure medical histories and ensure traceabilities of all reads/updates.

### Milestone 4: Clinical Encounter Workflows
*   Support creation of interactive clinical encounters (diagnoses, symptoms, physician notes).
*   Enforce structured encounter timelines.

### Milestone 5: Billing & Stellar Integration
*   Map clinical encounters to billing events.
*   Process billing payments using Stellar asset networks and record transaction hashes for audit trails.

### Milestone 6: Mobile Clinical Workflows
*   Deploy frontline clinical screens for patient lookup.
*   Incorporate offline-first synchronization models for lower-connectivity settings.

### Milestone 7: Production Hardening & Audits
*   Implement dependency auditing, security headers, and static analysis verification checks in the CI.
