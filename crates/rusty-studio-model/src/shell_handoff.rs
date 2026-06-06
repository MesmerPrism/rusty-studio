use serde::{Deserialize, Serialize};

use crate::{
    StudioShellBundleValidationReport, StudioShellHostRoutes, StudioShellRuntimeAuthority,
    StudioShellTargetKind, StudioValidationCheck, StudioValidationStatus,
};

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub revision: u64,
    pub graph_id: String,
    pub status: StudioValidationStatus,
    pub issue_code: Option<String>,
    pub message: String,
    pub handoff_kind: StudioShellHandoffKind,
    pub consumer_id: String,
    pub target_kind: StudioShellTargetKind,
    pub bundle_dir: String,
    pub descriptor_path: String,
    pub artifact_manifest_path: String,
    pub template_index_path: String,
    pub template_manifest_path: String,
    pub consumer_args: Vec<String>,
    pub runtime_authority: Option<StudioShellRuntimeAuthority>,
    pub validation: StudioShellBundleValidationReport,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHandoffKind {
    DesktopShell,
    PhoneShell,
    QuestShell,
    UnknownShell,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffReadinessReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub revision: u64,
    pub bundle_root: String,
    pub status: StudioValidationStatus,
    pub graph_count: usize,
    pub ready_count: usize,
    pub failed_count: usize,
    pub missing_bundle_count: usize,
    pub target_summaries: Vec<StudioShellHandoffReadinessTargetSummary>,
    pub entries: Vec<StudioShellHandoffReadinessEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffReadinessTargetSummary {
    pub target_kind: StudioShellTargetKind,
    pub graph_count: usize,
    pub ready_count: usize,
    pub failed_count: usize,
    pub missing_bundle_count: usize,
    pub package_count: usize,
    pub module_count: usize,
    pub operator_shell_count: usize,
    pub graph_ids: Vec<String>,
    pub consumer_ids: Vec<String>,
    pub issue_codes: Vec<String>,
    pub bundle_dirs: Vec<String>,
    pub ready_bundle_dirs: Vec<String>,
    pub failed_bundle_dirs: Vec<String>,
    pub missing_bundle_dirs: Vec<String>,
    pub template_index_paths: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffReadinessEntry {
    pub export_bundle_id: String,
    pub graph_id: String,
    pub display_name: String,
    pub target_host_profile: String,
    pub target_kind: StudioShellTargetKind,
    pub package_ids: Vec<String>,
    pub module_ids: Vec<String>,
    pub operator_shell_ids: Vec<String>,
    pub package_count: usize,
    pub module_count: usize,
    pub operator_shell_count: usize,
    pub status: StudioValidationStatus,
    pub issue_code: Option<String>,
    pub message: String,
    pub handoff_kind: StudioShellHandoffKind,
    pub consumer_id: String,
    pub bundle_dir: String,
    pub template_index_path: String,
    pub consumer_args: Vec<String>,
    pub runtime_authority: Option<StudioShellRuntimeAuthority>,
    pub validation_status: StudioValidationStatus,
    pub failed_check_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffManifest {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub source_readiness_schema: String,
    pub bundle_root: String,
    pub status: StudioValidationStatus,
    pub graph_count: usize,
    pub ready_count: usize,
    pub failed_count: usize,
    pub missing_bundle_count: usize,
    pub runtime_authority: StudioShellRuntimeAuthority,
    pub targets: Vec<StudioShellHandoffManifestTarget>,
    pub handoffs: Vec<StudioShellHandoffManifestEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffManifestTarget {
    pub target_kind: StudioShellTargetKind,
    pub graph_count: usize,
    pub ready_count: usize,
    pub failed_count: usize,
    pub missing_bundle_count: usize,
    pub package_count: usize,
    pub module_count: usize,
    pub operator_shell_count: usize,
    pub graph_ids: Vec<String>,
    pub consumer_ids: Vec<String>,
    pub issue_codes: Vec<String>,
    pub bundle_dirs: Vec<String>,
    pub ready_bundle_dirs: Vec<String>,
    pub failed_bundle_dirs: Vec<String>,
    pub missing_bundle_dirs: Vec<String>,
    pub template_index_paths: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffManifestEntry {
    pub export_bundle_id: String,
    pub graph_id: String,
    pub display_name: String,
    pub target_host_profile: String,
    pub target_kind: StudioShellTargetKind,
    pub status: StudioValidationStatus,
    pub issue_code: Option<String>,
    pub message: String,
    pub handoff_kind: StudioShellHandoffKind,
    pub consumer_id: String,
    pub bundle_dir: String,
    pub template_index_path: String,
    pub consumer_args: Vec<String>,
    pub runtime_authority: Option<StudioShellRuntimeAuthority>,
    pub package_ids: Vec<String>,
    pub module_ids: Vec<String>,
    pub operator_shell_ids: Vec<String>,
    pub validation_status: StudioValidationStatus,
    pub failed_check_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffManifestValidationReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub manifest_id: String,
    pub status: StudioValidationStatus,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffIntakeReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellHandoffIntakeStatus,
    pub issue_code: Option<String>,
    pub command_session_authority: String,
    pub install_launch_evidence_authority: String,
    pub studio_role: String,
    pub accepted_count: usize,
    pub blocked_count: usize,
    pub target_summaries: Vec<StudioShellHandoffIntakeTargetSummary>,
    pub entries: Vec<StudioShellHandoffIntakeEntry>,
    pub validation: StudioShellHandoffManifestValidationReport,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHandoffIntakeStatus {
    Accepted,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffIntakeTargetSummary {
    pub target_kind: StudioShellTargetKind,
    pub accepted_count: usize,
    pub blocked_count: usize,
    pub graph_ids: Vec<String>,
    pub consumer_ids: Vec<String>,
    pub bundle_dirs: Vec<String>,
    pub template_index_paths: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHandoffIntakeEntry {
    pub export_bundle_id: String,
    pub graph_id: String,
    pub display_name: String,
    pub target_host_profile: String,
    pub target_kind: StudioShellTargetKind,
    pub handoff_kind: StudioShellHandoffKind,
    pub consumer_id: String,
    pub handoff_status: StudioValidationStatus,
    pub issue_code: Option<String>,
    pub decision: StudioShellHandoffIntakeDecision,
    pub handoff_request_kind: String,
    pub runtime_route_kind: String,
    pub next_required_action: String,
    pub bundle_dir: String,
    pub template_index_path: String,
    pub consumer_args: Vec<String>,
    pub command_session_authority: String,
    pub install_launch_evidence_authority: String,
    pub studio_role: String,
    pub package_ids: Vec<String>,
    pub module_ids: Vec<String>,
    pub operator_shell_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHandoffIntakeDecision {
    ReadyForRuntimeOwner,
    BlockedByManifestIssue,
    BlockedByHandoffIssue,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellRunbookReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_manifest_schema: String,
    pub source_intake_schema: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub bundle_root: String,
    pub status: StudioShellRunbookStatus,
    pub issue_code: Option<String>,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub target_summaries: Vec<StudioShellRunbookTargetSummary>,
    pub prohibited_actions: Vec<String>,
    pub entries: Vec<StudioShellRunbookEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellRunbookStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellRunbookTargetSummary {
    pub target_kind: StudioShellTargetKind,
    pub ready_count: usize,
    pub blocked_count: usize,
    pub rejected_count: usize,
    pub graph_ids: Vec<String>,
    pub consumer_ids: Vec<String>,
    pub responsible_owners: Vec<String>,
    pub runtime_route_kinds: Vec<String>,
    pub issue_codes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellRunbookEntry {
    pub export_bundle_id: String,
    pub graph_id: String,
    pub display_name: String,
    pub target_host_profile: String,
    pub target_kind: StudioShellTargetKind,
    pub handoff_kind: StudioShellHandoffKind,
    pub status: StudioShellRunbookStatus,
    pub issue_code: Option<String>,
    pub decision: StudioShellHandoffIntakeDecision,
    pub responsible_owner: String,
    pub handoff_request_kind: String,
    pub runtime_route_kind: String,
    pub next_required_action: String,
    pub execution_policy: String,
    pub command_session_authority: String,
    pub install_launch_evidence_authority: String,
    pub studio_role: String,
    pub consumer_id: String,
    pub bundle_dir: String,
    pub template_index_path: String,
    pub consumer_args: Vec<String>,
    pub cli_request: Vec<String>,
    pub host_routes: StudioShellHostRoutes,
    pub route_status: StudioValidationStatus,
    pub route_issue_code: Option<String>,
    pub package_ids: Vec<String>,
    pub module_ids: Vec<String>,
    pub operator_shell_ids: Vec<String>,
}
