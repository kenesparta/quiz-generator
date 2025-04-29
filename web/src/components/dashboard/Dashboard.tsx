import React from "react";

interface DashboardLayoutProps {
  children: React.ReactNode;
}

export const Dashboard = ({ children }: DashboardLayoutProps) => {
  return (
    <>
      {children}
    </>
  )
}