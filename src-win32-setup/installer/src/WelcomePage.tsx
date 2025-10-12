import React from 'react';
import './page.css'; // Assume you have a CSS file for styling

/**
 * A professional Welcome page component for the AHQ Store Installer.
 * @component
 */
const WelcomePage = ({ onStartInstallation }: { onStartInstallation: () => void }) => {
  return (
    <div className="ahq-installer-container">
      <header className="ahq-header">
        <h1>AHQ Store Installer</h1>
        <p className="ahq-tagline">Streamlined Setup for Your AHQ Devices</p>
      </header>

      <main className="ahq-content">
        <section className="ahq-welcome-message">
          <h2>Welcome</h2>
          <p>
            Thank you for choosing the **AHQ Store Installer**. This application provides a comprehensive and efficient solution for setting up your new device. It ensures the correct installation of all necessary drivers, essential software, and optimized configurations curated by the AHQ technical team.
          </p>
          <p>
            Our goal is to get your device operating at **peak performance** with minimal effort.
          </p>
        </section>

        <section className="ahq-process-summary">
          <h3>Installation Process Overview</h3>
          <ul>
            <li>
              <strong>Review:</strong> You will first be presented with a customizable list of recommended and mandatory packages.
            </li>
            <li>
              <strong>Select:</strong> Review the options and confirm your selection. Mandatory items are pre-selected for optimal system stability.
            </li>
            <li>
              <strong>Install:</strong> The installer will automatically download, verify, and install all selected components.
            </li>
          </ul>
        </section>
      </main>

      <footer className="ahq-footer">
        <p>Click **Continue** to review the package list and begin the installation.</p>
        <button
          className="ahq-button-primary"
          onClick={onStartInstallation}
          aria-label="Start Installation Process"
        >
          Continue
        </button>
        <p className="ahq-support-info">
          For technical assistance, please visit the AHQ Support Portal.
        </p>
      </footer>
    </div>
  );
};

export default WelcomePage;