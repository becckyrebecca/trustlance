export interface User {
  id: string;
  username: string;
  role: "admin" | "clinician" | "support";
  clinicId: string;
}

export interface Clinic {
  id: string;
  name: string;
  address: string;
  onboardedAt: string;
}

export interface Patient {
  id: string;
  firstName: string;
  lastName: string;
  dateOfBirth: string;
  medicalHistory: string[];
  createdAt: string;
}

export interface Encounter {
  id: string;
  patientId: string;
  clinicianId: string;
  notes: string;
  diagnosisCode: string;
  timestamp: string;
  status: "active" | "completed" | "disputed";
  billingStatus: "pending" | "billed" | "failed";
}

export interface BillingRecord {
  encounterId: string;
  amountUSD: number;
  stellarTransactionHash?: string;
  status: "unbilled" | "pending" | "paid" | "failed";
  timestamp: string;
}
