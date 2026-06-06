use serde::{Deserialize, Serialize};

use crate::{
    StudioShellExportPackageBaselineSelectionReport,
    StudioShellExportPackageBaselineSelectionStatus, StudioShellExportPackageComparisonReport,
    StudioShellExportPackageComparisonStatus, StudioShellHandoffAcceptanceBaselineSelectionReport,
    StudioShellHandoffAcceptanceBaselineSelectionStatus,
    StudioShellHandoffAcceptanceComparisonReport, StudioShellHandoffAcceptanceComparisonStatus,
    StudioValidationCheck, StudioValidationStatus,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellReleaseCandidateReviewReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_manifest_schema: String,
    pub manifest_path: Option<String>,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellReleaseCandidateReviewStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub review_owner: String,
    pub command_session_authority: String,
    pub install_launch_evidence_authority: String,
    pub studio_role: String,
    pub handoff_status: StudioValidationStatus,
    pub handoff_ready_count: usize,
    pub handoff_failed_count: usize,
    pub handoff_missing_bundle_count: usize,
    pub acceptance_baseline_selection: StudioShellHandoffAcceptanceBaselineSelectionReport,
    pub acceptance_comparison: Option<StudioShellHandoffAcceptanceComparisonReport>,
    pub export_package_baseline_selection: StudioShellExportPackageBaselineSelectionReport,
    pub export_package_comparison: Option<StudioShellExportPackageComparisonReport>,
    pub checks: Vec<StudioValidationCheck>,
    pub prohibited_actions: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellReleaseCandidateReviewStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellReleaseCandidateReviewManifest {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub candidate_id: String,
    pub label: String,
    pub review_path: String,
    pub review_schema: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellReleaseCandidateReviewStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub review_owner: String,
    pub command_session_authority: String,
    pub install_launch_evidence_authority: String,
    pub studio_role: String,
    pub handoff_ready_count: usize,
    pub handoff_failed_count: usize,
    pub handoff_missing_bundle_count: usize,
    pub acceptance_baseline_status: StudioShellHandoffAcceptanceBaselineSelectionStatus,
    pub acceptance_baseline_id: Option<String>,
    pub acceptance_comparison_status: Option<StudioShellHandoffAcceptanceComparisonStatus>,
    pub export_package_baseline_status: StudioShellExportPackageBaselineSelectionStatus,
    pub export_package_baseline_id: Option<String>,
    pub export_package_comparison_status: Option<StudioShellExportPackageComparisonStatus>,
    pub check_count: usize,
    pub failed_check_count: usize,
    pub prohibited_actions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellReleaseCandidateReviewIndex {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub project_ids: Vec<String>,
    pub manifest_ids: Vec<String>,
    pub default_candidate_id: Option<String>,
    pub candidate_count: usize,
    pub ready_candidate_count: usize,
    pub blocked_candidate_count: usize,
    pub rejected_candidate_count: usize,
    pub entries: Vec<StudioShellReleaseCandidateReviewIndexEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellReleaseCandidateReviewIndexEntry {
    pub candidate_id: String,
    pub label: String,
    pub candidate_manifest_path: Option<String>,
    pub review_path: String,
    pub review_schema: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellReleaseCandidateReviewStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub review_owner: String,
    pub command_session_authority: String,
    pub install_launch_evidence_authority: String,
    pub studio_role: String,
    pub handoff_ready_count: usize,
    pub handoff_failed_count: usize,
    pub handoff_missing_bundle_count: usize,
    pub acceptance_baseline_status: StudioShellHandoffAcceptanceBaselineSelectionStatus,
    pub acceptance_baseline_id: Option<String>,
    pub acceptance_comparison_status: Option<StudioShellHandoffAcceptanceComparisonStatus>,
    pub export_package_baseline_status: StudioShellExportPackageBaselineSelectionStatus,
    pub export_package_baseline_id: Option<String>,
    pub export_package_comparison_status: Option<StudioShellExportPackageComparisonStatus>,
    pub check_count: usize,
    pub failed_check_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellReleaseCandidateReviewSelectionReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_index_schema: String,
    pub index_path: Option<String>,
    pub requested_candidate_id: Option<String>,
    pub default_candidate_id: Option<String>,
    pub selected_candidate_id: Option<String>,
    pub status: StudioShellReleaseCandidateReviewSelectionStatus,
    pub issue_code: Option<String>,
    pub candidate_count: usize,
    pub ready_candidate_count: usize,
    pub blocked_candidate_count: usize,
    pub rejected_candidate_count: usize,
    pub project_ids: Vec<String>,
    pub manifest_ids: Vec<String>,
    pub entries: Vec<StudioShellReleaseCandidateReviewSelectionEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellReleaseCandidateReviewSelectionStatus {
    Selected,
    Missing,
    Empty,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellReleaseCandidateReviewSelectionEntry {
    pub candidate_id: String,
    pub label: String,
    pub selected: bool,
    pub default: bool,
    pub candidate_manifest_path: Option<String>,
    pub review_path: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellReleaseCandidateReviewStatus,
    pub issue_code: Option<String>,
    pub acceptance_baseline_id: Option<String>,
    pub acceptance_comparison_status: Option<StudioShellHandoffAcceptanceComparisonStatus>,
    pub export_package_baseline_id: Option<String>,
    pub export_package_comparison_status: Option<StudioShellExportPackageComparisonStatus>,
    pub check_count: usize,
    pub failed_check_count: usize,
}
