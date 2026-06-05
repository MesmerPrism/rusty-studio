use serde::{Deserialize, Serialize};

use crate::{
    StudioShellHandoffIntakeDecision, StudioShellTargetKind, StudioValidationCheck,
    StudioValidationStatus,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceChecklistReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_intake_schema: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellHandoffAcceptanceStatus,
    pub issue_code: Option<String>,
    pub prohibited_actions: Vec<String>,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub intake_checks: Vec<StudioValidationCheck>,
    pub entries: Vec<StudioShellHandoffAcceptanceChecklistEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHandoffAcceptanceStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceChecklistEntry {
    pub graph_id: String,
    pub target_kind: StudioShellTargetKind,
    pub consumer_id: String,
    pub runtime_route_kind: String,
    pub source_decision: StudioShellHandoffIntakeDecision,
    pub status: StudioShellHandoffAcceptanceStatus,
    pub issue_code: Option<String>,
    pub next_required_action: String,
    pub command_session_authority: String,
    pub install_launch_evidence_authority: String,
    pub studio_role: String,
    pub checks: Vec<StudioShellHandoffAcceptanceCheck>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceCheck {
    pub check_id: String,
    pub owner: String,
    pub status: StudioValidationStatus,
    pub evidence: String,
    pub issue_code: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceSummaryReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub checklist_schema: String,
    pub checklist_path: Option<String>,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellHandoffAcceptanceStatus,
    pub issue_code: Option<String>,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub entry_count: usize,
    pub intake_check_count: usize,
    pub failed_intake_check_count: usize,
    pub prohibited_actions: Vec<String>,
    pub targets: Vec<StudioShellHandoffAcceptanceTargetSummary>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceTargetSummary {
    pub target_kind: StudioShellTargetKind,
    pub graph_count: usize,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub graph_ids: Vec<String>,
    pub consumer_ids: Vec<String>,
    pub route_kinds: Vec<String>,
    pub issue_codes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceBaselineManifest {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub baseline_id: String,
    pub label: String,
    pub checklist_path: String,
    pub summary: StudioShellHandoffAcceptanceSummaryReport,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceBaselineIndex {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub project_ids: Vec<String>,
    pub manifest_ids: Vec<String>,
    pub default_baseline_id: Option<String>,
    pub baseline_count: usize,
    pub ready_baseline_count: usize,
    pub blocked_baseline_count: usize,
    pub rejected_baseline_count: usize,
    pub entries: Vec<StudioShellHandoffAcceptanceBaselineIndexEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceBaselineIndexEntry {
    pub baseline_id: String,
    pub label: String,
    pub baseline_manifest_path: Option<String>,
    pub checklist_path: String,
    pub summary_schema: String,
    pub checklist_schema: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellHandoffAcceptanceStatus,
    pub issue_code: Option<String>,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub entry_count: usize,
    pub target_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceBaselineSelectionReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_index_schema: String,
    pub index_path: Option<String>,
    pub requested_baseline_id: Option<String>,
    pub default_baseline_id: Option<String>,
    pub selected_baseline_id: Option<String>,
    pub status: StudioShellHandoffAcceptanceBaselineSelectionStatus,
    pub issue_code: Option<String>,
    pub baseline_count: usize,
    pub ready_baseline_count: usize,
    pub blocked_baseline_count: usize,
    pub rejected_baseline_count: usize,
    pub project_ids: Vec<String>,
    pub manifest_ids: Vec<String>,
    pub entries: Vec<StudioShellHandoffAcceptanceBaselineSelectionEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHandoffAcceptanceBaselineSelectionStatus {
    Selected,
    Missing,
    Empty,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceBaselineSelectionEntry {
    pub baseline_id: String,
    pub label: String,
    pub selected: bool,
    pub default: bool,
    pub baseline_manifest_path: Option<String>,
    pub checklist_path: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellHandoffAcceptanceStatus,
    pub issue_code: Option<String>,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub entry_count: usize,
    pub target_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceComparisonReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub baseline_identity_schema: Option<String>,
    pub baseline_id: Option<String>,
    pub baseline_label: Option<String>,
    pub baseline_checklist_path: Option<String>,
    pub baseline_index_schema: Option<String>,
    pub baseline_index_path: Option<String>,
    pub baseline_index_default_baseline_id: Option<String>,
    pub baseline_index_selected_baseline_id: Option<String>,
    pub baseline_schema: String,
    pub candidate_schema: String,
    pub baseline_manifest_id: String,
    pub candidate_manifest_id: String,
    pub baseline_project_id: String,
    pub candidate_project_id: String,
    pub baseline_project_revision: u64,
    pub candidate_project_revision: u64,
    pub baseline_status: StudioShellHandoffAcceptanceStatus,
    pub candidate_status: StudioShellHandoffAcceptanceStatus,
    pub status: StudioShellHandoffAcceptanceComparisonStatus,
    pub issue_code: Option<String>,
    pub baseline_ready_count: usize,
    pub candidate_ready_count: usize,
    pub ready_delta: isize,
    pub baseline_blocked_count: usize,
    pub candidate_blocked_count: usize,
    pub blocked_delta: isize,
    pub baseline_rejected_count: usize,
    pub candidate_rejected_count: usize,
    pub rejected_delta: isize,
    pub checks: Vec<StudioValidationCheck>,
    pub entries: Vec<StudioShellHandoffAcceptanceComparisonEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHandoffAcceptanceComparisonStatus {
    Improved,
    Unchanged,
    Regressed,
    Incomparable,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffAcceptanceComparisonEntry {
    pub graph_id: String,
    pub target_kind: Option<StudioShellTargetKind>,
    pub baseline_status: Option<StudioShellHandoffAcceptanceStatus>,
    pub candidate_status: Option<StudioShellHandoffAcceptanceStatus>,
    pub change: StudioShellHandoffAcceptanceComparisonChange,
    pub score_delta: isize,
    pub baseline_consumer_id: Option<String>,
    pub candidate_consumer_id: Option<String>,
    pub baseline_route_kind: Option<String>,
    pub candidate_route_kind: Option<String>,
    pub baseline_issue_code: Option<String>,
    pub candidate_issue_code: Option<String>,
    pub issue_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHandoffAcceptanceComparisonChange {
    Added,
    Removed,
    Improved,
    Unchanged,
    Regressed,
    Changed,
}
