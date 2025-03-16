import type { Metadata } from "next"
import "./globals.css"
import { Dashboard } from "@shared/Dashboard";

export const metadata: Metadata = {
  title: "Quiz",
  description: "Backoffice",
}

export default function RootLayout({ children, }: Readonly<{ children: React.ReactNode; }>) {
  return (
    <html lang="en">
    <body>
    <Dashboard>
      {children}
    </Dashboard>
    </body>
    </html>
  )
}
