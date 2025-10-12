import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import WelcomePage from './WelcomePage'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <WelcomePage onStartInstallation={() => { }} />
  </StrictMode>,
)
