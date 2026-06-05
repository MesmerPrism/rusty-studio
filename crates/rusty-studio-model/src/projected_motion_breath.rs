use serde::{Deserialize, Serialize};

use crate::{StudioValidationCheck, StudioValidationStatus};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioManifoldPackageValidationReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub status: StudioValidationStatus,
    pub checks: Vec<StudioManifoldPackageValidationCheck>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioManifoldPackageValidationCheck {
    pub check_id: String,
    pub status: StudioValidationStatus,
    pub evidence: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioPackageEvidenceIntakeReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_report_schema: String,
    pub source_report_path: Option<String>,
    pub target_package_id: String,
    pub status: StudioPackageEvidenceIntakeStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub runtime_authority: String,
    pub authoring_authority: String,
    pub platform_validation_authority: String,
    pub runtime_execution_performed: bool,
    pub platform_execution_performed: bool,
    pub source_report_status: StudioValidationStatus,
    pub source_check_count: usize,
    pub target_package_check_count: usize,
    pub required_check_count: usize,
    pub ready_required_check_count: usize,
    pub blocked_required_check_count: usize,
    pub observed_check_count: usize,
    pub entries: Vec<StudioPackageEvidenceIntakeEntry>,
    pub prohibited_actions: Vec<String>,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioPackageEvidenceIntakeStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioPackageEvidenceIntakeEntry {
    pub check_id: String,
    pub source_status: StudioValidationStatus,
    pub evidence: String,
    pub required_for_studio: bool,
    pub decision: StudioPackageEvidenceIntakeDecision,
    pub next_required_action: String,
    pub issue_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioPackageEvidenceIntakeDecision {
    Ready,
    Observed,
    BlockedByMissingCheck,
    BlockedByFailedCheck,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioProjectedMotionBreathAuthoringReviewReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_intake_schema: String,
    pub source_intake_path: Option<String>,
    pub source_profile_schema: Option<String>,
    pub source_profile_path: Option<String>,
    pub target_package_id: String,
    pub target_module_id: Option<String>,
    pub profile_id: Option<String>,
    pub status: StudioProjectedMotionBreathAuthoringReviewStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub runtime_authority: String,
    pub authoring_authority: String,
    pub platform_validation_authority: String,
    pub runtime_execution_performed: bool,
    pub platform_execution_performed: bool,
    pub package_evidence_status: StudioPackageEvidenceIntakeStatus,
    pub package_required_check_count: usize,
    pub package_ready_required_check_count: usize,
    pub package_blocked_required_check_count: usize,
    pub input_kinds: Vec<String>,
    pub projection_mode: Option<String>,
    pub fallback_projection_mode: Option<String>,
    pub proposed_command_id: String,
    pub proposed_profile_operation: String,
    pub required_package_checks: Vec<String>,
    pub prohibited_actions: Vec<String>,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioProjectedMotionBreathAuthoringReviewStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioProjectedMotionBreathSourceAdapterSelectionReviewReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_authoring_review_schema: String,
    pub source_authoring_review_path: Option<String>,
    pub source_descriptor_schema: Option<String>,
    pub source_descriptor_path: Option<String>,
    pub target_package_id: String,
    pub target_module_id: Option<String>,
    pub profile_id: Option<String>,
    pub selected_adapter_id: String,
    pub selected_source_kind: Option<String>,
    pub selected_input_kind: Option<String>,
    pub selected_output_stream_id: Option<String>,
    pub status: StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub runtime_authority: String,
    pub authoring_authority: String,
    pub platform_validation_authority: String,
    pub runtime_execution_performed: bool,
    pub platform_execution_performed: bool,
    pub source_authoring_review_status: StudioProjectedMotionBreathAuthoringReviewStatus,
    pub source_descriptor_count: usize,
    pub matching_descriptor_count: usize,
    pub proposal_kind: String,
    pub prohibited_actions: Vec<String>,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_selection_review_schema: String,
    pub source_selection_review_path: Option<String>,
    pub source_package_report_schema: String,
    pub source_package_report_path: Option<String>,
    pub source_binding_schema: Option<String>,
    pub source_binding_path: Option<String>,
    pub source_normalization_case_schema: Option<String>,
    pub source_normalization_case_path: Option<String>,
    pub target_package_id: String,
    pub target_module_id: Option<String>,
    pub profile_id: Option<String>,
    pub selected_adapter_id: String,
    pub selected_source_kind: Option<String>,
    pub selected_input_kind: Option<String>,
    pub selected_output_stream_id: Option<String>,
    pub binding_id: Option<String>,
    pub normalization_case_id: Option<String>,
    pub source_payload_kind: Option<String>,
    pub expected_sample_kind: Option<String>,
    pub status: StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub runtime_authority: String,
    pub authoring_authority: String,
    pub platform_validation_authority: String,
    pub runtime_execution_performed: bool,
    pub platform_execution_performed: bool,
    pub source_selection_status: StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus,
    pub adapter_normalization_check_id: String,
    pub adapter_normalization_check_status: Option<StudioValidationStatus>,
    pub source_binding_selected_adapter_match: bool,
    pub deterministic_normalization_evidence: bool,
    pub proposal_kind: String,
    pub prohibited_actions: Vec<String>,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioProjectedMotionBreathShellHandoffReviewReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_evidence_schema: Option<String>,
    pub source_evidence_path: Option<String>,
    pub target_package_id: Option<String>,
    pub handoff_id: Option<String>,
    pub target_host_profile: Option<String>,
    pub shell_app_id: Option<String>,
    pub status: StudioProjectedMotionBreathShellHandoffReviewStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub runtime_authority: String,
    pub authoring_authority: String,
    pub platform_validation_authority: String,
    pub runtime_execution_performed: bool,
    pub platform_execution_performed: bool,
    pub broker_transport_used: bool,
    pub downstream_shell_runtime_used: bool,
    pub legacy_app_dependency_used: bool,
    pub required_binding_count: usize,
    pub ready_required_binding_count: usize,
    pub stream_bindings: Vec<String>,
    pub command_ids: Vec<String>,
    pub transport_ids: Vec<String>,
    pub feedback_receipt_exported: bool,
    pub feedback_sink_provides_receipt: bool,
    pub proposal_kind: String,
    pub prohibited_actions: Vec<String>,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioProjectedMotionBreathShellHandoffReviewStatus {
    Ready,
    Blocked,
    Rejected,
}
