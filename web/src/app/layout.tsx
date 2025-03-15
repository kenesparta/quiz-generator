import type { Metadata } from "next"
import { Header } from "@shared/Header"
import { Footer } from "@shared/Footer";
import "./globals.css"

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
    <Footer/>
    </body>
    </html>
  )
}
