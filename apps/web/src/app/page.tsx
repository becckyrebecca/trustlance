import React from "react";
import { RUNTIME_CONFIG } from "@novahealth/config";

export default function Page() {
  return (
    <div style={{ fontFamily: "sans-serif", padding: "40px", backgroundColor: "#f8f9fa", minHeight: "100vh" }}>
      <header style={{ borderBottom: "1px solid #dee2e6", paddingBottom: "20px", marginBottom: "30px" }}>
        <h1 style={{ color: "#0d6efd", margin: "0" }}>NovaHealth</h1>
        <p style={{ color: "#6c757d", margin: "5px 0 0 0" }}>Clinical Operations & Billing Workspace</p>
      </header>

      <main style={{ display: "grid", gridTemplateColumns: "2fr 1fr", gap: "30px" }}>
        <div>
          <section style={{ backgroundColor: "#ffffff", padding: "20px", borderRadius: "8px", boxShadow: "0 2px 4px rgba(0,0,0,0.05)", marginBottom: "30px" }}>
            <h2 style={{ color: "#212529", marginTop: "0" }}>Clinic Operations Dashboard</h2>
            <p>Welcome to the clinic management dashboard. You are currently viewing: <strong>Downtown Family Care</strong>.</p>
            
            <div style={{ display: "flex", gap: "20px", marginTop: "20px" }}>
              <div style={{ flex: "1", border: "1px solid #dee2e6", padding: "15px", borderRadius: "6px", textAlign: "center" }}>
                <span style={{ fontSize: "24px", fontWeight: "bold", display: "block", color: "#198754" }}>1</span>
                <span style={{ color: "#6c757d", fontSize: "14px" }}>Onboarded Clinic</span>
              </div>
              <div style={{ flex: "1", border: "1px solid #dee2e6", padding: "15px", borderRadius: "6px", textAlign: "center" }}>
                <span style={{ fontSize: "24px", fontWeight: "bold", display: "block", color: "#0d6efd" }}>1</span>
                <span style={{ color: "#6c757d", fontSize: "14px" }}>Active Patient Record</span>
              </div>
              <div style={{ flex: "1", border: "1px solid #dee2e6", padding: "15px", borderRadius: "6px", textAlign: "center" }}>
                <span style={{ fontSize: "24px", fontWeight: "bold", display: "block", color: "#fd7e14" }}>1</span>
                <span style={{ color: "#6c757d", fontSize: "14px" }}>Completed Encounter</span>
              </div>
            </div>
          </section>

          <section style={{ backgroundColor: "#ffffff", padding: "20px", borderRadius: "8px", boxShadow: "0 2px 4px rgba(0,0,0,0.05)" }}>
            <h2 style={{ color: "#212529", marginTop: "0" }}>Active Encounters Log</h2>
            <table style={{ width: "100%", borderCollapse: "collapse", marginTop: "15px" }}>
              <thead>
                <tr style={{ borderBottom: "2px solid #dee2e6", textAlign: "left" }}>
                  <th style={{ padding: "8px" }}>Encounter ID</th>
                  <th style={{ padding: "8px" }}>Patient</th>
                  <th style={{ padding: "8px" }}>Diagnosis</th>
                  <th style={{ padding: "8px" }}>Billing</th>
                </tr>
              </thead>
              <tbody>
                <tr style={{ borderBottom: "1px solid #dee2e6" }}>
                  <td style={{ padding: "8px" }}>e1</td>
                  <td style={{ padding: "8px" }}>John Doe</td>
                  <td style={{ padding: "8px" }}>Z00.00 (General Exam)</td>
                  <td style={{ padding: "8px" }}><span style={{ backgroundColor: "#ffc107", padding: "3px 8px", borderRadius: "12px", fontSize: "12px", fontWeight: "bold" }}>Pending</span></td>
                </tr>
              </tbody>
            </table>
          </section>
        </div>

        <aside>
          <div style={{ backgroundColor: "#e9ecef", padding: "20px", borderRadius: "8px", marginBottom: "30px" }}>
            <h3 style={{ marginTop: "0", color: "#495057" }}>Stellar Billing Configuration</h3>
            <ul style={{ paddingLeft: "20px", lineHeight: "1.6" }}>
              <li><strong>Asset Symbol:</strong> {RUNTIME_CONFIG.billing.stellarAssetSymbol}</li>
              <li><strong>Base Consultation Fee:</strong> ${RUNTIME_CONFIG.billing.baseConsultationFeeUSD}</li>
              <li><strong>Encounter Processing Fee:</strong> ${RUNTIME_CONFIG.billing.encounterProcessingFeeUSD}</li>
              <li><strong>Session Expiry Time:</strong> {RUNTIME_CONFIG.sessionExpiryMs / 60000} mins</li>
            </ul>
          </div>

          <div style={{ backgroundColor: "#fff3cd", border: "1px solid #ffe69c", padding: "20px", borderRadius: "8px", color: "#664d03" }}>
            <h3 style={{ marginTop: "0" }}>Frontline Mobile Scoping</h3>
            <p style={{ margin: "0", fontSize: "14px" }}>
              The mobile client is designed to support front-line healthcare workers in lower-connectivity environments. Lookups are cached locally to support offline work.
            </p>
          </div>
        </aside>
      </main>
    </div>
  );
}
