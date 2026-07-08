import React from "react";

export const metadata = {
  title: "NovaHealth Admin Portal",
  description: "Unified Clinical Operations Platform",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}
