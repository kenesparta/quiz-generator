import * as React from "react";
import { createContext, ReactNode, useState, useContext } from 'react';

interface FilterState {
  category: string;
  minPrice: number;
}

interface FiltersContextType {
  filters: FilterState;
  setFilters: React.Dispatch<React.SetStateAction<FilterState>>;
}

const FiltersContext = createContext<FiltersContextType | undefined>(undefined);

interface FilterProviderProps {
  children: ReactNode;
}

export function FilterProvider({ children }: FilterProviderProps) {
  const [filters, setFilters] = useState<FilterState>({
    category: 'all',
    minPrice: 0,
  });

  return (
    <FiltersContext.Provider value={{ filters, setFilters }}>
      {children}
    </FiltersContext.Provider>
  );
}

// Custom hook to use the FiltersContext
export function useFilters() {
  const context = useContext(FiltersContext);
  if (!context) {
    throw new Error('useFilters must be used within a FilterProvider');
  }

  return context;
}