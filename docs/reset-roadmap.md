# LuminaHealth Reset Roadmap

This document maps out the post-foundational development roadmap to transition LuminaHealth from the initial monorepo workspace scaffolding into a hardened, production-grade clinical platform.

---

## 🚀 Post-MVP Progression

### Phase 1: Real-time Database Integration
*   Replace in-memory database stores in `apps/api` with an ORM (Prisma / TypeORM) connecting to a PostgreSQL instance.
*   Enforce encrypted data storage at rest for patient records to comply with medical standards.

### Phase 2: Live Stellar Network Integration
*   Configure environment variables (`apps/stellar-service/.env`) to use secret keys of funded Testnet accounts.
*   Transition the transaction builder in `apps/stellar-service` from mock hashes to live transaction submissions using `@stellar/stellar-sdk` and tracking payments via Horizon.

### Phase 3: Mobile Offline-first Engine
*   Integrate a local database (SQLite / WatermelonDB) into `apps/mobile` for offline caching.
*   Build a conflict-free sync engine that uploads queued patient records/encounters as soon as active network connectivity is detected.

### Phase 4: Medical Compliance & Audit Trails
*   Implement end-to-end audit logging for all patient record read/write operations to maintain HIPAA-style compliance.
*   Conduct standard security audits of the Stellar Service's transaction signing and key storage mechanisms.
