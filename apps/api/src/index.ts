import express, { Request, Response, NextFunction } from "express";
import { RUNTIME_CONFIG } from "@novahealth/config";
import { User, Clinic, Patient, Encounter } from "@novahealth/types";

const app = express();
const port = 3001;

app.use(express.json());

// In-memory clinical databases
const clinics: Clinic[] = [
  { id: "c1", name: "Downtown Family Care", address: "123 Main St", onboardedAt: new Date().toISOString() }
];

const staff: User[] = [
  { id: "s1", username: "dr_smith", role: "clinician", clinicId: "c1" },
  { id: "s2", username: "admin_jane", role: "admin", clinicId: "c1" }
];

const patients: Patient[] = [
  {
    id: "p1",
    firstName: "John",
    lastName: "Doe",
    dateOfBirth: "1985-05-15",
    medicalHistory: ["Hypertension", "Seasonal Allergies"],
    createdAt: new Date().toISOString()
  }
];

const encounters: Encounter[] = [
  {
    id: "e1",
    patientId: "p1",
    clinicianId: "s1",
    notes: "Routine wellness check. Patient feels well. Advised to stay active.",
    diagnosisCode: "Z00.00",
    timestamp: new Date().toISOString(),
    status: "completed",
    billingStatus: "pending"
  }
];

// Simple logging middleware
app.use((req: Request, res: Response, next: NextFunction) => {
  console.log(`[NovaAPI] ${req.method} ${req.url}`);
  next();
});

// Mock Auth Middleware
const authMock = (req: Request, res: Response, next: NextFunction) => {
  const authHeader = req.headers.authorization;
  if (!authHeader) {
    return res.status(401).json({ error: "Unauthorized. Authentication token required." });
  }
  const username = authHeader.replace("Bearer ", "");
  const user = staff.find((s) => s.username === username);
  if (!user) {
    return res.status(403).json({ error: "Invalid credentials." });
  }
  (req as any).user = user;
  next();
};

// Scoped Permission Middleware
const requireRole = (allowedRoles: string[]) => {
  return (req: Request, res: Response, next: NextFunction) => {
    const user = (req as any).user as User;
    if (!user || !allowedRoles.includes(user.role)) {
      return res.status(403).json({ error: "Forbidden. Insufficient permissions." });
    }
    next();
  };
};

// 1. Auth endpoints
app.post("/api/auth/login", (req: Request, res: Response) => {
  const { username } = req.body;
  const user = staff.find((s) => s.username === username);
  if (!user) {
    return res.status(404).json({ error: "Staff member not found." });
  }
  res.json({
    token: `Bearer ${user.username}`,
    user,
    sessionExpiryMs: RUNTIME_CONFIG.sessionExpiryMs
  });
});

// 2. Clinic Management
app.get("/api/clinics", authMock, requireRole(["admin"]), (req: Request, res: Response) => {
  res.json(clinics);
});

app.post("/api/clinics", authMock, requireRole(["admin"]), (req: Request, res: Response) => {
  const { name, address } = req.body;
  if (!name || !address) {
    return res.status(400).json({ error: "Name and address are required." });
  }
  const newClinic: Clinic = {
    id: `c${clinics.length + 1}`,
    name,
    address,
    onboardedAt: new Date().toISOString()
  };
  clinics.push(newClinic);
  res.status(214).json(newClinic);
});

// 3. Patient Records
app.get("/api/patients", authMock, (req: Request, res: Response) => {
  res.json(patients);
});

app.post("/api/patients", authMock, requireRole(["admin", "clinician"]), (req: Request, res: Response) => {
  const { firstName, lastName, dateOfBirth, medicalHistory } = req.body;
  if (!firstName || !lastName || !dateOfBirth) {
    return res.status(400).json({ error: "firstName, lastName and dateOfBirth are required." });
  }
  const newPatient: Patient = {
    id: `p${patients.length + 1}`,
    firstName,
    lastName,
    dateOfBirth,
    medicalHistory: medicalHistory || [],
    createdAt: new Date().toISOString()
  };
  patients.push(newPatient);
  res.status(214).json(newPatient);
});

// 4. Encounters & Clinical Workflows
app.get("/api/encounters", authMock, (req: Request, res: Response) => {
  res.json(encounters);
});

app.post("/api/encounters", authMock, requireRole(["clinician"]), (req: Request, res: Response) => {
  const { patientId, notes, diagnosisCode } = req.body;
  const user = (req as any).user as User;

  if (!patientId || !notes || !diagnosisCode) {
    return res.status(400).json({ error: "patientId, notes, and diagnosisCode are required." });
  }

  const patientExists = patients.some((p) => p.id === patientId);
  if (!patientExists) {
    return res.status(404).json({ error: "Patient record not found." });
  }

  const newEncounter: Encounter = {
    id: `e${encounters.length + 1}`,
    patientId,
    clinicianId: user.id,
    notes,
    diagnosisCode,
    timestamp: new Date().toISOString(),
    status: "active",
    billingStatus: "pending"
  };

  encounters.push(newEncounter);
  res.status(214).json(newEncounter);
});

app.listen(port, () => {
  console.log(`[NovaAPI] Running on port ${port}`);
  console.log(`[NovaAPI] Configured with Base Consultation Fee: $${RUNTIME_CONFIG.billing.baseConsultationFeeUSD}`);
});
