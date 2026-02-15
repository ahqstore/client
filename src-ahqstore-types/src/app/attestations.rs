use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attestations {
  // What these will resemble will be defined later!
  pub oidc: String,
  pub ed25519: String,

  // Will be defined later!
  pub workflow: String,

  // SHA of the Build Workflow Content
  // If this remains same, CI Integrity report is skipped
  pub workflowSha: String,

  pub security_reports: SecurityReports,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecurityReports {
  pub windowsDefenderStatus: SecurityReportStatus,
  pub clamAVReport: ClamAVReport,
  pub mobSFReport: MobSFReport,
  /// Not absolute authority, but a weighted heuristic
  ///
  /// Especially if the score is `< 100/400`, it triggers a
  /// manual review no matter the score from others.
  pub ciIntegrity: LLMCICDReport,
  /// If this is set of true, it overrides workflowSha boolean
  pub ciReferencesSecondaryScripts: bool,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
/// This uses an LLM to verify and attest that the building
/// CI CD indeed builds the codebase on the fly of the exact repository
pub struct LLMCICDReport {
  pub status: SecurityReportStatus,
  pub integrityScore: u64,
  pub intentScore: u64,
  pub bestPracticesScore: u64,
  pub totalScore: u64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClamAVReport {
  pub status: SecurityReportStatus,
  pub goodFiles: Vec<String>,
  pub badFiles: Vec<String>,
  pub virusNames: Vec<String>,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MobSFReport {
  pub status: SecurityReportStatus,
  pub score: u64,
}

#[allow(non_snake_case)]
#[cfg_attr(feature = "export", derive(specta::Type))]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SecurityReportStatus {
  Passed,
  Failed,
  Unknown,
}
