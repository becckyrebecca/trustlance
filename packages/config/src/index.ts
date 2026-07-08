export const RUNTIME_CONFIG = {
  sessionExpiryMs: 3600000, // 1 hour
  billing: {
    baseConsultationFeeUSD: 50,
    encounterProcessingFeeUSD: 5,
    stellarAssetSymbol: "XLM",
    stellarAssetIssuer: ""
  },
  roles: {
    ADMIN: "admin",
    CLINICIAN: "clinician",
    SUPPORT: "support"
  },
  apiEndpoints: {
    local: "http://localhost:3001",
    production: "https://api.novahealth.org"
  }
};
