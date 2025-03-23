import type { Metadata } from "next"
import "./globals.css"
import { Dashboard } from "@shared/Dashboard";
import { Work_Sans } from "next/font/google"

const mainFont = Work_Sans({
  weight: ["200", "300", "400", "600", "700", "900"],
  subsets: ['latin-ext']
})

export const metadata: Metadata = {
  title: "Quiz",
  description: "Backoffice",
}

export default function RootLayout({ children, }: Readonly<{ children: React.ReactNode; }>) {
  return (
    <html lang="en">
    <body className={mainFont.className}>
    <Dashboard>
      {children}
    </Dashboard>
    </body>
    </html>
  )
}
