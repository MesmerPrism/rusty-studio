use serde::{Deserialize, Serialize};

use crate::{
    StudioShellHostRoutes, StudioShellRuntimeAuthority, StudioShellTargetKind,
    StudioValidationCheck,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_manifest_schema: String,
    pub source_runbook_schema: String,
    pub package_id: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub bundle_root: String,
    pub status: StudioShellExportPackageStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub review_owner: String,
    pub command_session_authority: String,
    pub install_launch_evidence_authority: String,
    pub studio_role: String,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub descriptor_count: usize,
    pub template_manifest_count: usize,
    pub runbook_entry_count: usize,
    pub target_summaries: Vec<StudioShellExportPackageTargetSummary>,
    pub prohibited_actions: Vec<String>,
    pub entries: Vec<StudioShellExportPackageEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellExportPackageStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageTargetSummary {
    pub target_kind: StudioShellTargetKind,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub descriptor_count: usize,
    pub template_manifest_count: usize,
    pub graph_ids: Vec<String>,
    pub consumer_ids: Vec<String>,
    pub responsible_owners: Vec<String>,
    pub issue_codes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageEntry {
    pub export_bundle_id: String,
    pub graph_id: String,
    pub display_name: String,
    pub target_host_profile: String,
    pub target_kind: StudioShellTargetKind,
    pub status: StudioShellExportPackageStatus,
    pub issue_code: Option<String>,
    pub responsible_owner: String,
    pub execution_policy: String,
    pub consumer_id: String,
    pub runtime_route_kind: String,
    pub next_required_action: String,
    pub bundle_dir: String,
    pub descriptor: Option<StudioShellExportPackageDescriptorRef>,
    pub template_manifest: Option<StudioShellExportPackageTemplateRef>,
    pub runbook_cli_request: Vec<String>,
    pub host_routes: StudioShellHostRoutes,
    pub package_ids: Vec<String>,
    pub module_ids: Vec<String>,
    pub operator_shell_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageDescriptorRef {
    pub descriptor_path: String,
    pub descriptor_id: String,
    pub graph_id: String,
    pub shell_id: String,
    pub target_host_profile: String,
    pub package_count: usize,
    pub module_count: usize,
    pub command_binding_count: usize,
    pub stream_binding_count: usize,
    pub validation_slot_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageTemplateRef {
    pub template_index_path: String,
    pub template_manifest_path: String,
    pub template_id: String,
    pub artifact_id: String,
    pub graph_id: String,
    pub shell_id: String,
    pub target_host_profile: String,
    pub host_routes: StudioShellHostRoutes,
    pub runtime_authority: StudioShellRuntimeAuthority,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageBaselineManifest {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub baseline_id: String,
    pub label: String,
    pub package_path: String,
    pub package_schema: String,
    pub package_id: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellExportPackageStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub review_owner: String,
    pub command_session_authority: String,
    pub install_launch_evidence_authority: String,
    pub studio_role: String,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub descriptor_count: usize,
    pub template_manifest_count: usize,
    pub runbook_entry_count: usize,
    pub target_count: usize,
    pub prohibited_actions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageBaselineIndex {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub project_ids: Vec<String>,
    pub package_ids: Vec<String>,
    pub manifest_ids: Vec<String>,
    pub default_baseline_id: Option<String>,
    pub baseline_count: usize,
    pub ready_baseline_count: usize,
    pub blocked_baseline_count: usize,
    pub rejected_baseline_count: usize,
    pub entries: Vec<StudioShellExportPackageBaselineIndexEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageBaselineIndexEntry {
    pub baseline_id: String,
    pub label: String,
    pub baseline_manifest_path: Option<String>,
    pub package_path: String,
    pub package_schema: String,
    pub package_id: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellExportPackageStatus,
    pub issue_code: Option<String>,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub descriptor_count: usize,
    pub template_manifest_count: usize,
    pub runbook_entry_count: usize,
    pub target_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageBaselineSelectionReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_index_schema: String,
    pub index_path: Option<String>,
    pub requested_baseline_id: Option<String>,
    pub default_baseline_id: Option<String>,
    pub selected_baseline_id: Option<String>,
    pub status: StudioShellExportPackageBaselineSelectionStatus,
    pub issue_code: Option<String>,
    pub baseline_count: usize,
    pub ready_baseline_count: usize,
    pub blocked_baseline_count: usize,
    pub rejected_baseline_count: usize,
    pub project_ids: Vec<String>,
    pub package_ids: Vec<String>,
    pub manifest_ids: Vec<String>,
    pub entries: Vec<StudioShellExportPackageBaselineSelectionEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellExportPackageBaselineSelectionStatus {
    Selected,
    Missing,
    Empty,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageBaselineSelectionEntry {
    pub baseline_id: String,
    pub label: String,
    pub selected: bool,
    pub default: bool,
    pub baseline_manifest_path: Option<String>,
    pub package_path: String,
    pub package_id: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellExportPackageStatus,
    pub issue_code: Option<String>,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub descriptor_count: usize,
    pub template_manifest_count: usize,
    pub runbook_entry_count: usize,
    pub target_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageComparisonReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub baseline_identity_schema: Option<String>,
    pub baseline_id: Option<String>,
    pub baseline_label: Option<String>,
    pub baseline_package_path: Option<String>,
    pub baseline_index_schema: Option<String>,
    pub baseline_index_path: Option<String>,
    pub baseline_index_default_baseline_id: Option<String>,
    pub baseline_index_selected_baseline_id: Option<String>,
    pub baseline_schema: String,
    pub candidate_schema: String,
    pub baseline_package_id: String,
    pub candidate_package_id: String,
    pub baseline_manifest_id: String,
    pub candidate_manifest_id: String,
    pub baseline_project_id: String,
    pub candidate_project_id: String,
    pub baseline_project_revision: u64,
    pub candidate_project_revision: u64,
    pub baseline_status: StudioShellExportPackageStatus,
    pub candidate_status: StudioShellExportPackageStatus,
    pub status: StudioShellExportPackageComparisonStatus,
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
    pub baseline_descriptor_count: usize,
    pub candidate_descriptor_count: usize,
    pub descriptor_delta: isize,
    pub baseline_template_manifest_count: usize,
    pub candidate_template_manifest_count: usize,
    pub template_manifest_delta: isize,
    pub baseline_runbook_entry_count: usize,
    pub candidate_runbook_entry_count: usize,
    pub runbook_entry_delta: isize,
    pub checks: Vec<StudioValidationCheck>,
    pub entries: Vec<StudioShellExportPackageComparisonEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellExportPackageComparisonStatus {
    Improved,
    Unchanged,
    Regressed,
    Incomparable,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellExportPackageComparisonEntry {
    pub graph_id: String,
    pub target_kind: Option<StudioShellTargetKind>,
    pub baseline_status: Option<StudioShellExportPackageStatus>,
    pub candidate_status: Option<StudioShellExportPackageStatus>,
    pub change: StudioShellExportPackageComparisonChange,
    pub score_delta: isize,
    pub baseline_consumer_id: Option<String>,
    pub candidate_consumer_id: Option<String>,
    pub baseline_descriptor_present: bool,
    pub candidate_descriptor_present: bool,
    pub baseline_template_manifest_present: bool,
    pub candidate_template_manifest_present: bool,
    pub baseline_runbook_cli_request_present: bool,
    pub candidate_runbook_cli_request_present: bool,
    pub baseline_issue_code: Option<String>,
    pub candidate_issue_code: Option<String>,
    pub issue_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellExportPackageComparisonChange {
    Added,
    Removed,
    Improved,
    Unchanged,
    Regressed,
    Changed,
}
