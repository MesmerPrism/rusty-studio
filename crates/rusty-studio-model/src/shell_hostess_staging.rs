use serde::{Deserialize, Serialize};

use crate::{
    StudioProjectedMotionBreathShellHandoffReviewStatus, StudioShellTargetKind,
    StudioValidationCheck,
};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingPreviewManifest {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_intake_schema: String,
    pub source_handoff_manifest_schema: Option<String>,
    pub intake_path: Option<String>,
    pub package_path: Option<String>,
    pub handoff_manifest_path: Option<String>,
    pub selected_candidate_id: Option<String>,
    pub manifest_id: Option<String>,
    pub project_id: Option<String>,
    pub project_revision: Option<u64>,
    pub status: StudioShellHostessStagingPreviewStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub staging_owner: String,
    pub command_session_authority: Option<String>,
    pub install_launch_evidence_authority: Option<String>,
    pub studio_role: Option<String>,
    pub assignment_count: usize,
    pub ready_assignment_count: usize,
    pub blocked_assignment_count: usize,
    pub ready_group_count: usize,
    pub blocked_group_count: usize,
    pub expected_artifact_count: usize,
    pub groups: Vec<StudioShellHostessStagingPreviewGroup>,
    pub prohibited_actions: Vec<String>,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingPreviewStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingPreviewGroup {
    pub action_id: String,
    pub owner: String,
    pub request_kind: String,
    pub route_kind: String,
    pub status: StudioShellHostessStagingPreviewGroupStatus,
    pub issue_code: Option<String>,
    pub source: String,
    pub next_required_action: String,
    pub prohibited_in_studio: bool,
    pub expected_artifact_count: usize,
    pub target_kinds: Vec<String>,
    pub graph_ids: Vec<String>,
    pub expected_artifacts: Vec<StudioShellHostessStagingPreviewArtifact>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingPreviewGroupStatus {
    Ready,
    Blocked,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingPreviewArtifact {
    pub artifact_kind: String,
    pub path: String,
    pub target_kind: Option<StudioShellTargetKind>,
    pub graph_id: Option<String>,
    pub consumer_id: Option<String>,
    pub route_hint: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingFilePlan {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_preview_schema: String,
    pub preview_path: Option<String>,
    pub intake_path: Option<String>,
    pub package_path: Option<String>,
    pub handoff_manifest_path: Option<String>,
    pub selected_candidate_id: Option<String>,
    pub manifest_id: Option<String>,
    pub project_id: Option<String>,
    pub project_revision: Option<u64>,
    pub status: StudioShellHostessStagingFilePlanStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub staging_owner: String,
    pub command_session_authority: Option<String>,
    pub install_launch_evidence_authority: Option<String>,
    pub studio_role: Option<String>,
    pub preview_group_count: usize,
    pub ready_preview_group_count: usize,
    pub blocked_preview_group_count: usize,
    pub source_artifact_count: usize,
    pub planned_file_count: usize,
    pub duplicate_artifact_count: usize,
    pub request_count: usize,
    pub ready_request_count: usize,
    pub blocked_request_count: usize,
    pub target_request_count: usize,
    pub shared_request_count: usize,
    pub requests: Vec<StudioShellHostessStagingFileRequest>,
    pub prohibited_actions: Vec<String>,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingFilePlanStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingFileRequest {
    pub request_id: String,
    pub request_kind: String,
    pub owner: String,
    pub status: StudioShellHostessStagingFileRequestStatus,
    pub issue_code: Option<String>,
    pub target_key: String,
    pub target_kind: Option<StudioShellTargetKind>,
    pub graph_id: Option<String>,
    pub consumer_id: Option<String>,
    pub destination_root: String,
    pub action_ids: Vec<String>,
    pub route_kinds: Vec<String>,
    pub planned_file_count: usize,
    pub planned_files: Vec<StudioShellHostessStagingPlannedFile>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingFileRequestStatus {
    Ready,
    Blocked,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingPlannedFile {
    pub artifact_kind: String,
    pub source_path: String,
    pub destination_path: String,
    pub target_kind: Option<StudioShellTargetKind>,
    pub graph_id: Option<String>,
    pub consumer_id: Option<String>,
    pub route_hints: Vec<String>,
    pub source_action_ids: Vec<String>,
    pub source_route_kinds: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingHandoffEnvelope {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_file_plan_schema: String,
    pub file_plan_path: Option<String>,
    pub preview_path: Option<String>,
    pub intake_path: Option<String>,
    pub package_path: Option<String>,
    pub handoff_manifest_path: Option<String>,
    pub selected_candidate_id: Option<String>,
    pub envelope_id: String,
    pub manifest_id: Option<String>,
    pub project_id: Option<String>,
    pub project_revision: Option<u64>,
    pub status: StudioShellHostessStagingHandoffEnvelopeStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub handoff_owner: String,
    pub staging_owner: String,
    pub command_session_authority: Option<String>,
    pub install_launch_evidence_authority: Option<String>,
    pub studio_role: Option<String>,
    pub planned_file_count: usize,
    pub request_count: usize,
    pub ready_request_count: usize,
    pub blocked_request_count: usize,
    pub target_request_count: usize,
    pub shared_request_count: usize,
    pub instruction_count: usize,
    pub ready_instruction_count: usize,
    pub blocked_instruction_count: usize,
    pub provenance: StudioShellHostessStagingHandoffProvenance,
    pub request_summaries: Vec<StudioShellHostessStagingHandoffRequestSummary>,
    pub owner_instructions: Vec<StudioShellHostessStagingHandoffInstruction>,
    pub prohibited_actions: Vec<String>,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingHandoffEnvelopeStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingHandoffProvenance {
    pub checksum_algorithm: String,
    pub plan_checksum: String,
    pub source_artifact_kinds: Vec<String>,
    pub source_action_ids: Vec<String>,
    pub source_route_kinds: Vec<String>,
    pub target_keys: Vec<String>,
    pub destination_roots: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingHandoffRequestSummary {
    pub request_id: String,
    pub request_kind: String,
    pub owner: String,
    pub status: StudioShellHostessStagingFileRequestStatus,
    pub target_key: String,
    pub target_kind: Option<StudioShellTargetKind>,
    pub graph_id: Option<String>,
    pub consumer_id: Option<String>,
    pub destination_root: String,
    pub planned_file_count: usize,
    pub route_kinds: Vec<String>,
    pub action_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingHandoffInstruction {
    pub instruction_id: String,
    pub owner: String,
    pub status: StudioShellHostessStagingHandoffInstructionStatus,
    pub issue_code: Option<String>,
    pub instruction_kind: String,
    pub route_kind: String,
    pub source: String,
    pub next_required_action: String,
    pub prohibited_in_studio: bool,
    pub expected_input_path: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingHandoffInstructionStatus {
    Ready,
    Blocked,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingAcceptanceChecklistReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_handoff_schema: String,
    pub handoff_path: Option<String>,
    pub file_plan_path: Option<String>,
    pub preview_path: Option<String>,
    pub intake_path: Option<String>,
    pub package_path: Option<String>,
    pub handoff_manifest_path: Option<String>,
    pub selected_candidate_id: Option<String>,
    pub envelope_id: String,
    pub manifest_id: Option<String>,
    pub project_id: Option<String>,
    pub project_revision: Option<u64>,
    pub status: StudioShellHostessStagingAcceptanceStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub checklist_owner: String,
    pub handoff_owner: String,
    pub staging_owner: String,
    pub command_session_authority: Option<String>,
    pub install_launch_evidence_authority: Option<String>,
    pub studio_role: Option<String>,
    pub request_count: usize,
    pub ready_request_count: usize,
    pub blocked_request_count: usize,
    pub instruction_count: usize,
    pub ready_instruction_count: usize,
    pub blocked_instruction_count: usize,
    pub checksum_algorithm: String,
    pub plan_checksum: String,
    pub ready_item_count: usize,
    pub blocked_item_count: usize,
    pub rejected_item_count: usize,
    pub prohibited_actions: Vec<String>,
    pub handoff_checks: Vec<StudioValidationCheck>,
    pub entries: Vec<StudioShellHostessStagingAcceptanceChecklistEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingAcceptanceStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingAcceptanceChecklistEntry {
    pub item_id: String,
    pub owner: String,
    pub status: StudioShellHostessStagingAcceptanceStatus,
    pub issue_code: Option<String>,
    pub item_kind: String,
    pub route_kind: String,
    pub source: String,
    pub evidence: String,
    pub next_required_action: String,
    pub prohibited_in_studio: bool,
    pub expected_input_path: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingAcceptanceManifest {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub acceptance_id: String,
    pub label: String,
    pub checklist_path: String,
    pub checklist_schema: String,
    pub envelope_id: String,
    pub manifest_id: Option<String>,
    pub project_id: Option<String>,
    pub project_revision: Option<u64>,
    pub status: StudioShellHostessStagingAcceptanceStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub checklist_owner: String,
    pub handoff_owner: String,
    pub staging_owner: String,
    pub command_session_authority: Option<String>,
    pub install_launch_evidence_authority: Option<String>,
    pub studio_role: Option<String>,
    pub request_count: usize,
    pub ready_request_count: usize,
    pub blocked_request_count: usize,
    pub instruction_count: usize,
    pub ready_instruction_count: usize,
    pub blocked_instruction_count: usize,
    pub checksum_algorithm: String,
    pub plan_checksum: String,
    pub ready_item_count: usize,
    pub blocked_item_count: usize,
    pub rejected_item_count: usize,
    pub prohibited_actions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingAcceptanceIndex {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub project_ids: Vec<String>,
    pub envelope_ids: Vec<String>,
    pub manifest_ids: Vec<String>,
    pub default_acceptance_id: Option<String>,
    pub acceptance_count: usize,
    pub ready_acceptance_count: usize,
    pub blocked_acceptance_count: usize,
    pub rejected_acceptance_count: usize,
    pub entries: Vec<StudioShellHostessStagingAcceptanceIndexEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingAcceptanceIndexEntry {
    pub acceptance_id: String,
    pub label: String,
    pub acceptance_manifest_path: Option<String>,
    pub checklist_path: String,
    pub checklist_schema: String,
    pub envelope_id: String,
    pub manifest_id: Option<String>,
    pub project_id: Option<String>,
    pub project_revision: Option<u64>,
    pub status: StudioShellHostessStagingAcceptanceStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub checklist_owner: String,
    pub handoff_owner: String,
    pub staging_owner: String,
    pub command_session_authority: Option<String>,
    pub install_launch_evidence_authority: Option<String>,
    pub studio_role: Option<String>,
    pub request_count: usize,
    pub ready_request_count: usize,
    pub blocked_request_count: usize,
    pub instruction_count: usize,
    pub ready_instruction_count: usize,
    pub blocked_instruction_count: usize,
    pub checksum_algorithm: String,
    pub plan_checksum: String,
    pub ready_item_count: usize,
    pub blocked_item_count: usize,
    pub rejected_item_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingAcceptanceSelectionReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_index_schema: String,
    pub index_path: Option<String>,
    pub requested_acceptance_id: Option<String>,
    pub default_acceptance_id: Option<String>,
    pub selected_acceptance_id: Option<String>,
    pub status: StudioShellHostessStagingAcceptanceSelectionStatus,
    pub issue_code: Option<String>,
    pub acceptance_count: usize,
    pub ready_acceptance_count: usize,
    pub blocked_acceptance_count: usize,
    pub rejected_acceptance_count: usize,
    pub project_ids: Vec<String>,
    pub envelope_ids: Vec<String>,
    pub manifest_ids: Vec<String>,
    pub entries: Vec<StudioShellHostessStagingAcceptanceSelectionEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingAcceptanceSelectionStatus {
    Selected,
    Missing,
    Empty,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingAcceptanceSelectionEntry {
    pub acceptance_id: String,
    pub label: String,
    pub selected: bool,
    pub default: bool,
    pub acceptance_manifest_path: Option<String>,
    pub checklist_path: String,
    pub envelope_id: String,
    pub project_id: Option<String>,
    pub project_revision: Option<u64>,
    pub status: StudioShellHostessStagingAcceptanceStatus,
    pub issue_code: Option<String>,
    pub ready_item_count: usize,
    pub blocked_item_count: usize,
    pub rejected_item_count: usize,
    pub request_count: usize,
    pub instruction_count: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingAcceptanceComparisonReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub baseline_identity_schema: Option<String>,
    pub baseline_acceptance_id: Option<String>,
    pub baseline_label: Option<String>,
    pub baseline_checklist_path: Option<String>,
    pub baseline_index_schema: Option<String>,
    pub baseline_index_path: Option<String>,
    pub baseline_index_default_acceptance_id: Option<String>,
    pub baseline_index_selected_acceptance_id: Option<String>,
    pub baseline_schema: String,
    pub candidate_schema: String,
    pub baseline_envelope_id: String,
    pub candidate_envelope_id: String,
    pub baseline_manifest_id: Option<String>,
    pub candidate_manifest_id: Option<String>,
    pub baseline_project_id: Option<String>,
    pub candidate_project_id: Option<String>,
    pub baseline_project_revision: Option<u64>,
    pub candidate_project_revision: Option<u64>,
    pub baseline_status: StudioShellHostessStagingAcceptanceStatus,
    pub candidate_status: StudioShellHostessStagingAcceptanceStatus,
    pub status: StudioShellHostessStagingAcceptanceComparisonStatus,
    pub issue_code: Option<String>,
    pub baseline_ready_item_count: usize,
    pub candidate_ready_item_count: usize,
    pub ready_item_delta: isize,
    pub baseline_blocked_item_count: usize,
    pub candidate_blocked_item_count: usize,
    pub blocked_item_delta: isize,
    pub baseline_rejected_item_count: usize,
    pub candidate_rejected_item_count: usize,
    pub rejected_item_delta: isize,
    pub checks: Vec<StudioValidationCheck>,
    pub entries: Vec<StudioShellHostessStagingAcceptanceComparisonEntry>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingAcceptanceComparisonStatus {
    Improved,
    Unchanged,
    Regressed,
    Incomparable,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingAcceptanceComparisonEntry {
    pub item_id: String,
    pub owner: String,
    pub baseline_status: Option<StudioShellHostessStagingAcceptanceStatus>,
    pub candidate_status: Option<StudioShellHostessStagingAcceptanceStatus>,
    pub change: StudioShellHostessStagingAcceptanceComparisonChange,
    pub score_delta: isize,
    pub baseline_route_kind: Option<String>,
    pub candidate_route_kind: Option<String>,
    pub baseline_issue_code: Option<String>,
    pub candidate_issue_code: Option<String>,
    pub issue_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingAcceptanceComparisonChange {
    Added,
    Removed,
    Improved,
    Unchanged,
    Regressed,
    Changed,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingExecutionRequestReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub request_id: String,
    pub source_acceptance_index_schema: Option<String>,
    pub acceptance_index_path: Option<String>,
    pub selected_acceptance_id: String,
    pub acceptance_manifest_path: Option<String>,
    pub acceptance_schema: String,
    pub acceptance_checklist_path: String,
    pub acceptance_checklist_schema: String,
    pub source_acceptance_status: StudioShellHostessStagingAcceptanceStatus,
    pub source_handoff_schema: String,
    pub handoff_path: Option<String>,
    pub envelope_id: String,
    pub manifest_id: Option<String>,
    pub project_id: Option<String>,
    pub project_revision: Option<u64>,
    pub selected_candidate_id: Option<String>,
    pub file_plan_path: Option<String>,
    pub preview_path: Option<String>,
    pub intake_path: Option<String>,
    pub package_path: Option<String>,
    pub handoff_manifest_path: Option<String>,
    #[serde(default)]
    pub pmb_shell_handoff_review_required: bool,
    pub pmb_shell_handoff_review_path: Option<String>,
    pub source_pmb_shell_handoff_review_schema: Option<String>,
    pub source_pmb_shell_handoff_review_status:
        Option<StudioProjectedMotionBreathShellHandoffReviewStatus>,
    pub source_pmb_shell_handoff_review_issue_code: Option<String>,
    pub source_pmb_shell_handoff_id: Option<String>,
    pub source_pmb_shell_app_id: Option<String>,
    #[serde(default)]
    pub pmb_shell_handoff_review_ready: bool,
    #[serde(default)]
    pub hostess_operator_start_preflight_cli_args: Vec<String>,
    pub status: StudioShellHostessStagingExecutionRequestStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub adapter_owner: String,
    pub requester_role: String,
    pub command_session_authority: Option<String>,
    pub install_launch_evidence_authority: Option<String>,
    pub studio_role: Option<String>,
    pub request_count: usize,
    pub ready_request_count: usize,
    pub blocked_request_count: usize,
    pub instruction_count: usize,
    pub ready_instruction_count: usize,
    pub blocked_instruction_count: usize,
    pub checksum_algorithm: String,
    pub plan_checksum: String,
    pub prohibited_studio_actions: Vec<String>,
    pub adapter_action_count: usize,
    pub ready_adapter_action_count: usize,
    pub blocked_adapter_action_count: usize,
    pub actions: Vec<StudioShellHostessStagingExecutionAction>,
    pub checks: Vec<StudioValidationCheck>,
    pub ack_template: StudioShellHostessStagingExecutionAck,
    pub reject_template: StudioShellHostessStagingExecutionReject,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingExecutionRequestStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingExecutionAction {
    pub action_id: String,
    pub owner: String,
    pub status: StudioShellHostessStagingExecutionActionStatus,
    pub issue_code: Option<String>,
    pub action_kind: String,
    pub route_kind: String,
    pub source_item_id: String,
    pub responsible_authority: String,
    pub expected_input_path: Option<String>,
    pub next_required_action: String,
    pub ack_required: bool,
    pub execution_in_studio: bool,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingExecutionActionStatus {
    Ready,
    Blocked,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingExecutionAck {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub request_id: String,
    pub accepted_by: String,
    pub ack_status: StudioShellHostessStagingExecutionAckStatus,
    pub execution_in_studio: bool,
    pub command_session_authority: Option<String>,
    pub install_launch_evidence_authority: Option<String>,
    pub required_action_ids: Vec<String>,
    pub accepted_action_ids: Vec<String>,
    pub required_evidence_kinds: Vec<String>,
    pub issue_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingExecutionAckStatus {
    Pending,
    Accepted,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessStagingExecutionReject {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub request_id: String,
    pub rejected_by: String,
    pub reject_status: StudioShellHostessStagingExecutionRejectStatus,
    pub execution_in_studio: bool,
    pub request_action_ids: Vec<String>,
    pub rejected_action_ids: Vec<String>,
    pub reason_code: Option<String>,
    pub next_required_action: String,
    pub issue_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessStagingExecutionRejectStatus {
    Pending,
    Rejected,
}
