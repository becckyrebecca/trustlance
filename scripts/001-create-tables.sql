-- Migration: Create initial tables for TrustLance Web3 platform
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    wallet_address VARCHAR(56) UNIQUE NOT NULL,
    role VARCHAR(20) NOT NULL DEFAULT 'freelancer',
    username VARCHAR(50),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS jobs (
    id SERIAL PRIMARY KEY,
    title VARCHAR(100) NOT NULL,
    description TEXT NOT NULL,
    client_address VARCHAR(56) NOT NULL,
    freelancer_address VARCHAR(56),
    arbiter_address VARCHAR(56) NOT NULL,
    token_address VARCHAR(56) NOT NULL,
    contract_id VARCHAR(56),
    total_amount NUMERIC(20, 7) NOT NULL,
    is_funded BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS milestones (
    id SERIAL PRIMARY KEY,
    job_id INTEGER REFERENCES jobs(id) ON DELETE CASCADE,
    index INTEGER NOT NULL,
    amount NUMERIC(20, 7) NOT NULL,
    state VARCHAR(20) NOT NULL DEFAULT 'Pending',
    delivered_at TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
