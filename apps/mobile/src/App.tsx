import React from "react";
import { RUNTIME_CONFIG } from "@lumina/config";
import { Patient } from "@lumina/types";

export function MobileApp() {
  const [searchQuery, setSearchQuery] = React.useState("");
  const [offlineSyncPending, setOfflineSyncPending] = React.useState(false);

  const testPatient: Patient = {
    id: "p1",
    firstName: "John",
    lastName: "Doe",
    dateOfBirth: "1985-05-15",
    medicalHistory: ["Hypertension"],
    createdAt: new Date().toISOString()
  };

  return {
    render: () => `
      Mobile Frontline Workspace: LuminaHealth
      Active Clinic Server: ${RUNTIME_CONFIG.apiEndpoints.production}
      Patient Lookup Query: ${searchQuery}
      Offline Sync Status: ${offlineSyncPending ? "Pending Connection" : "Synchronized"}
      Test Patient: ${testPatient.firstName} ${testPatient.lastName} (DOB: ${testPatient.dateOfBirth})
    `
  };
}
