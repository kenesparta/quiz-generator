import type { Metadata } from "next"
import "./globals.css"
import Header from "@/app/header"

export const metadata: Metadata = {
  title: "Quiz",
  description: "Backoffice",
}

export default function RootLayout({ children, }: Readonly<{ children: React.ReactNode; }>) {
  return (
    <html lang="en">
    <body>
    <Header/>
    {children}
    </body>
    </html>
  )
}
