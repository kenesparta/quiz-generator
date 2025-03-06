import { createRoot } from 'react-dom/client'
import App from './App.tsx'
import { StrictMode } from 'react'

const root = createRoot(document.getElementById('root')!)
root.render(
  <StrictMode>
    <App/>
  </StrictMode>,
)
