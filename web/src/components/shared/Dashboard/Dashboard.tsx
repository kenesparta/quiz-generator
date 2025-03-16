import { Header } from "@shared/Header";
import React from "react";

interface DashboardLayoutProps {
  children: React.ReactNode;
}

export const Dashboard = ({ children }: DashboardLayoutProps) => {
  return (
    <div className="min-h-screen grid grid-cols-1 grid-rows-[auto_1fr]">
      <Header/>
      <main className="">
        <div className="max-w-7xl mx-auto px-6 py-6">
          {children}
        </div>
      </main>
    </div>
  )
}