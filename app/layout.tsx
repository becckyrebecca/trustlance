import type { Metadata } from "next";
import "../styles/globals.css";

export const metadata: Metadata = {
  title: "TrustLance | Web3 Freelancing & Trustless Escrow Platform",
  description: "Secure, milestone-based freelance contracts powered by the Stellar blockchain and Soroban smart contracts.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <head>
        <link rel="preconnect" href="https://fonts.googleapis.com" />
        <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="anonymous" />
        <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;500;600;700;800&display=swap" rel="stylesheet" />
      </head>
      <body style={{ fontFamily: "'Outfit', sans-serif" }}>
        {children}
      </body>
    </html>
  );
}
