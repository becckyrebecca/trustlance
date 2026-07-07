import express, { Request, Response } from "express";
import { RUNTIME_CONFIG } from "@lumina/config";
import { BillingRecord } from "@lumina/types";
import * as StellarSdk from "@stellar/stellar-sdk";

const app = express();
const port = 3002;

app.use(express.json());

// In-memory billing records database
const billingRecords: BillingRecord[] = [];

// Process clinical billing event on Stellar
app.post("/api/billing/process", async (req: Request, res: Response) => {
  const { encounterId, amountUSD } = req.body;

  if (!encounterId || typeof amountUSD !== "number") {
    return res.status(400).json({ error: "encounterId and amountUSD are required." });
  }

  // Check if already processed
  const existing = billingRecords.find((r) => r.encounterId === encounterId);
  if (existing) {
    return res.status(400).json({ error: "Billing record already exists for this encounter." });
  }

  const record: BillingRecord = {
    encounterId,
    amountUSD,
    status: "pending",
    timestamp: new Date().toISOString()
  };

  try {
    // Generate a temporary Keypair to simulate key authorization
    const sourceKeypair = StellarSdk.Keypair.random();
    
    // In production, we build, sign, and submit the transaction to Horizon:
    // const transaction = new StellarSdk.TransactionBuilder(...)
    // Here we generate a cryptographically structured mock transaction hash to represent success:
    const randomHash = StellarSdk.Keypair.random().publicKey().toLowerCase().substring(0, 32);
    const mockTxHash = `st_tx_${randomHash}`;

    record.status = "paid";
    record.stellarTransactionHash = mockTxHash;
    billingRecords.push(record);

    console.log(`[StellarService] Processed billing for Encounter ${encounterId}: $${amountUSD} via transaction ${mockTxHash}`);

    res.json({
      success: true,
      record
    });
  } catch (error: any) {
    record.status = "failed";
    billingRecords.push(record);
    res.status(500).json({ error: "Stellar transaction failed", details: error.message });
  }
});

app.get("/api/billing/records", (req: Request, res: Response) => {
  res.json(billingRecords);
});

app.listen(port, () => {
  console.log(`[StellarService] Running on port ${port}`);
  console.log(`[StellarService] Configured Asset Symbol: ${RUNTIME_CONFIG.billing.stellarAssetSymbol}`);
});
