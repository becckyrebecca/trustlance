"use client";

import React, { useState } from "react";
import { 
  ShieldCheck, 
  Coins, 
  UserCheck, 
  ArrowRight, 
  Wallet, 
  CheckCircle2, 
  HelpCircle, 
  Clock, 
  FileText, 
  Scale, 
  User, 
  Check, 
  AlertTriangle 
} from "lucide-react";

export default function Page() {
  const [walletConnected, setWalletConnected] = useState(false);
  const [walletAddress, setWalletAddress] = useState("");
  const [selectedTab, setSelectedTab] = useState<"client" | "freelancer">("client");

  // Mock Active Jobs Data
  const [jobs, setJobs] = useState([
    {
      id: 1,
      title: "DeFi Aggregator UI Redesign",
      client: "GAX4...9K2W",
      freelancer: "GCT3...8PL4",
      arbiter: "GDD5...7QQ2",
      amount: "4,500 XLM",
      isFunded: true,
      milestones: [
        { name: "Wireframes & UX Architecture", amount: "1,500 XLM", state: "Released" },
        { name: "High-Fidelity UI Mockups", amount: "1,500 XLM", state: "Delivered" },
        { name: "Frontend Next.js Components", amount: "1,500 XLM", state: "Pending" }
      ]
    },
    {
      id: 2,
      title: "Soroban AMM Smart Contract Audit",
      client: "GD2L...3LL1",
      freelancer: "GAA7...5P3O",
      arbiter: "GDD5...7QQ2",
      amount: "8,000 XLM",
      isFunded: true,
      milestones: [
        { name: "Security Check & Vulnerability Audit", amount: "4,000 XLM", state: "Released" },
        { name: "Gas Optimization & Test Coverage", amount: "4,000 XLM", state: "Disputed" }
      ]
    }
  ]);

  const connectWallet = () => {
    if (!walletConnected) {
      setWalletConnected(true);
      setWalletAddress("GB2K3R...9WZ4");
    } else {
      setWalletConnected(false);
      setWalletAddress("");
    }
  };

  const getStatusBadge = (state: string) => {
    switch (state) {
      case "Released":
        return <span className="px-3 py-1 text-xs font-semibold rounded-full bg-emerald-500/10 text-emerald-400 border border-emerald-500/20 flex items-center gap-1"><CheckCircle2 className="w-3.5 h-3.5" /> Released</span>;
      case "Delivered":
        return <span className="px-3 py-1 text-xs font-semibold rounded-full bg-amber-500/10 text-amber-400 border border-amber-500/20 flex items-center gap-1"><Clock className="w-3.5 h-3.5" /> Delivered (Auto-Release active)</span>;
      case "Disputed":
        return <span className="px-3 py-1 text-xs font-semibold rounded-full bg-rose-500/10 text-rose-400 border border-rose-500/20 flex items-center gap-1"><AlertTriangle className="w-3.5 h-3.5" /> Disputed</span>;
      default:
        return <span className="px-3 py-1 text-xs font-semibold rounded-full bg-slate-500/10 text-slate-400 border border-slate-500/20 flex items-center gap-1"><HelpCircle className="w-3.5 h-3.5" /> Pending</span>;
    }
  };

  return (
    <div className="min-height-screen text-slate-100 flex flex-col justify-between">
      {/* Navigation Header */}
      <header className="glass sticky top-0 z-50 px-6 py-4 flex items-center justify-between">
        <div className="flex items-center gap-2">
          <div className="bg-sky-500 p-2 rounded-lg glow">
            <ShieldCheck className="w-6 h-6 text-slate-950 font-bold" />
          </div>
          <span className="text-xl font-bold tracking-wider bg-gradient-to-r from-sky-400 to-indigo-400 bg-clip-text text-transparent">
            TRUSTLANCE
          </span>
        </div>

        <button 
          onClick={connectWallet}
          className="flex items-center gap-2 px-4 py-2 rounded-lg font-medium text-sm transition-all duration-300 bg-slate-900 border border-sky-500/30 hover:border-sky-400 hover:shadow-[0_0_15px_rgba(56,189,248,0.2)]"
        >
          <Wallet className="w-4 h-4 text-sky-400" />
          {walletConnected ? `${walletAddress}` : "Connect Freighter Wallet"}
        </button>
      </header>

      {/* Main Content */}
      <main className="flex-1 max-w-7xl mx-auto px-6 py-12 w-full grid grid-cols-1 lg:grid-cols-12 gap-12">
        {/* Left Side: Landing Content */}
        <section className="lg:col-span-5 flex flex-col justify-center space-y-6">
          <div className="inline-flex items-center gap-2 px-3 py-1 rounded-full bg-sky-500/10 text-sky-400 text-xs font-semibold border border-sky-500/20">
            <Coins className="w-3.5 h-3.5" /> Powered by Stellar & Soroban
          </div>
          <h1 className="text-4xl md:text-5xl font-extrabold tracking-tight leading-tight">
            Freelance with{" "}
            <span className="bg-gradient-to-r from-sky-400 via-blue-400 to-indigo-400 bg-clip-text text-transparent">
              Confidence
            </span>.
            <br /> Get Paid Securely.
          </h1>
          <p className="text-slate-400 text-lg leading-relaxed">
            TrustLance protects both freelancers and clients using trustless smart contract escrows. Funds are secured instantly and released dynamically as predefined milestones are met.
          </p>

          <div className="space-y-4 pt-4">
            <div className="flex items-start gap-3">
              <div className="bg-sky-500/10 p-2 rounded-lg text-sky-400 border border-sky-500/20">
                <ShieldCheck className="w-5 h-5" />
              </div>
              <div>
                <h3 className="font-semibold text-slate-200">Trustless Escrow Accounts</h3>
                <p className="text-slate-400 text-sm">Payments are secured inside a Soroban smart contract, never held by third-party intermediaries.</p>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="bg-indigo-500/10 p-2 rounded-lg text-indigo-400 border border-indigo-500/20">
                <Clock className="w-5 h-5" />
              </div>
              <div>
                <h3 className="font-semibold text-slate-200">Auto-Release Safeguards</h3>
                <p className="text-slate-400 text-sm">Completed milestones automatically release funds after a custom safety window if no dispute is raised.</p>
              </div>
            </div>
            <div className="flex items-start gap-3">
              <div className="bg-purple-500/10 p-2 rounded-lg text-purple-400 border border-purple-500/20">
                <Scale className="w-5 h-5" />
              </div>
              <div>
                <h3 className="font-semibold text-slate-200">Arbiter Mediation</h3>
                <p className="text-slate-400 text-sm">Designate a trusted third-party arbiter to mediate disputes and safely allocate refund or payouts.</p>
              </div>
            </div>
          </div>
        </section>

        {/* Right Side: Interactive Smart Escrow Simulator Dashboard */}
        <section className="lg:col-span-7 space-y-6">
          <div className="glass rounded-2xl p-6 glow flex flex-col space-y-6">
            <div className="flex items-center justify-between border-b border-slate-800 pb-4">
              <div>
                <h2 className="text-lg font-bold text-slate-200">Escrow Dashboard</h2>
                <p className="text-xs text-slate-400">Simulation of your active Soroban smart contract milestones</p>
              </div>
              <div className="flex bg-slate-950 p-1 rounded-lg border border-slate-800">
                <button 
                  onClick={() => setSelectedTab("client")}
                  className={`px-3 py-1.5 rounded-md text-xs font-semibold transition-all ${selectedTab === "client" ? "bg-sky-500 text-slate-950 shadow-md" : "text-slate-400 hover:text-slate-200"}`}
                >
                  I'm a Client
                </button>
                <button 
                  onClick={() => setSelectedTab("freelancer")}
                  className={`px-3 py-1.5 rounded-md text-xs font-semibold transition-all ${selectedTab === "freelancer" ? "bg-sky-500 text-slate-950 shadow-md" : "text-slate-400 hover:text-slate-200"}`}
                >
                  I'm a Freelancer
                </button>
              </div>
            </div>

            {/* Simulated Workspace Jobs */}
            <div className="space-y-6">
              {jobs.map((job) => (
                <div key={job.id} className="bg-slate-950/60 border border-slate-800/80 rounded-xl p-5 space-y-4">
                  <div className="flex items-start justify-between">
                    <div>
                      <h3 className="font-bold text-slate-200 text-base">{job.title}</h3>
                      <div className="flex flex-wrap gap-x-4 gap-y-1 mt-1 text-xs text-slate-400">
                        <span className="flex items-center gap-1"><User className="w-3 h-3" /> Client: {job.client}</span>
                        <span className="flex items-center gap-1"><User className="w-3 h-3" /> Freelancer: {job.freelancer}</span>
                      </div>
                    </div>
                    <div className="text-right">
                      <span className="text-sky-400 font-bold block">{job.amount}</span>
                      <span className="text-[10px] text-emerald-400 flex items-center justify-end gap-1"><Check className="w-3 h-3" /> Funded</span>
                    </div>
                  </div>

                  {/* Milestones Timeline */}
                  <div className="space-y-3 pt-2">
                    <div className="text-xs font-semibold text-slate-400 tracking-wider uppercase">Contract Milestones</div>
                    <div className="space-y-2">
                      {job.milestones.map((milestone, idx) => (
                        <div key={idx} className="flex items-center justify-between p-3 rounded-lg bg-slate-900 border border-slate-800">
                          <div className="space-y-1">
                            <div className="text-sm font-medium text-slate-300">{milestone.name}</div>
                            <div className="text-xs text-slate-500 font-medium">Value: {milestone.amount}</div>
                          </div>
                          <div className="flex items-center gap-3">
                            {getStatusBadge(milestone.state)}
                            {selectedTab === "client" && milestone.state === "Delivered" && (
                              <button className="px-3 py-1 text-xs font-semibold rounded bg-sky-500 text-slate-950 hover:bg-sky-400 transition-all shadow-sm">
                                Approve Release
                              </button>
                            )}
                            {selectedTab === "freelancer" && milestone.state === "Pending" && (
                              <button className="px-3 py-1 text-xs font-semibold rounded bg-indigo-500/20 text-indigo-400 border border-indigo-500/30 hover:bg-indigo-500/30 transition-all">
                                Request Review
                              </button>
                            )}
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                </div>
              ))}
            </div>

            {/* Create Contract Mock Form */}
            <div className="border-t border-slate-800 pt-6 space-y-4">
              <h3 className="font-bold text-slate-200 text-sm">Deploy New Milestone Escrow</h3>
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <input 
                  type="text" 
                  placeholder="Freelancer Wallet Address" 
                  className="bg-slate-900 border border-slate-800 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-sky-500 text-slate-200"
                />
                <input 
                  type="text" 
                  placeholder="Arbiter Address (Disputes)" 
                  className="bg-slate-900 border border-slate-800 rounded-lg px-3 py-2 text-sm focus:outline-none focus:border-sky-500 text-slate-200"
                />
              </div>
              <button 
                onClick={() => {
                  if (!walletConnected) {
                    alert("Please connect your Freighter wallet first.");
                  } else {
                    alert("Mock transaction initiated: deploying Soroban milestone escrow contract!");
                  }
                }}
                className="w-full py-3 rounded-lg font-bold text-sm bg-gradient-to-r from-sky-500 to-indigo-500 hover:from-sky-400 hover:to-indigo-400 text-slate-950 transition-all flex items-center justify-center gap-2 shadow-[0_0_20px_rgba(56,189,248,0.3)] hover:shadow-[0_0_25px_rgba(56,189,248,0.4)]"
              >
                Create Escrow Contract <ArrowRight className="w-4 h-4 text-slate-950" />
              </button>
            </div>
          </div>
        </section>
      </main>

      {/* Footer */}
      <footer className="border-t border-slate-900 bg-slate-950/40 px-6 py-6 text-center text-xs text-slate-500">
        © 2026 TrustLance Contributors. Built for the Stellar Community. Open source and MIT licensed.
      </footer>
    </div>
  );
}
