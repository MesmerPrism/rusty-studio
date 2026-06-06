use serde::{Deserialize, Serialize};

mod shell_hostess_staging;
pub use shell_hostess_staging::*;

mod shell_export_package;
pub use shell_export_package::*;

mod shell_handoff;
pub use shell_handoff::*;
mod shell_handoff_acceptance;
pub use shell_handoff_acceptance::*;

mod shell_release_candidate;
pub use shell_release_candidate::*;

mod projected_motion_breath;
pub use projected_motion_breath::*;

pub const PROJECT_SCHEMA: &str = "rusty.studio.project.v1";
pub const VALIDATION_REPORT_SCHEMA: &str = "rusty.studio.validation_report.v1";
pub const RESOLVED_PROJECT_SCHEMA: &str = "rusty.studio.resolved_project.v1";
pub const EXPORT_PLAN_SCHEMA: &str = "rusty.studio.export_plan.v1";
pub const VIEW_MODEL_SCHEMA: &str = "rusty.studio.view_model.v1";
pub const EDIT_REPORT_SCHEMA: &str = "rusty.studio.edit_report.v1";
pub const SHELL_DESCRIPTOR_SCHEMA: &str = "rusty.studio.shell_descriptor.v1";
pub const SHELL_DESCRIPTOR_REPORT_SCHEMA: &str = "rusty.studio.shell_descriptor_report.v1";
pub const SHELL_DESCRIPTOR_VALIDATION_REPORT_SCHEMA: &str =
    "rusty.studio.shell_descriptor_validation_report.v1";
pub const SHELL_ARTIFACT_MANIFEST_SCHEMA: &str = "rusty.studio.shell_artifact_manifest.v1";
pub const SHELL_ARTIFACT_REPORT_SCHEMA: &str = "rusty.studio.shell_artifact_report.v1";
pub const SHELL_ARTIFACT_MANIFEST_VALIDATION_REPORT_SCHEMA: &str =
    "rusty.studio.shell_artifact_manifest_validation_report.v1";
pub const SHELL_TEMPLATE_INDEX_SCHEMA: &str = "rusty.studio.shell_template_index.v1";
pub const SHELL_TEMPLATE_MANIFEST_SCHEMA: &str = "rusty.studio.shell_template_manifest.v1";
pub const SHELL_TEMPLATE_REPORT_SCHEMA: &str = "rusty.studio.shell_template_report.v1";
pub const SHELL_TEMPLATE_INDEX_VALIDATION_REPORT_SCHEMA: &str =
    "rusty.studio.shell_template_index_validation_report.v1";
pub const SHELL_BUNDLE_REPORT_SCHEMA: &str = "rusty.studio.shell_bundle_report.v1";
pub const SHELL_BUNDLE_VALIDATION_REPORT_SCHEMA: &str =
    "rusty.studio.shell_bundle_validation_report.v1";
pub const SHELL_HANDOFF_REPORT_SCHEMA: &str = "rusty.studio.shell_handoff_report.v1";
pub const SHELL_HANDOFF_READINESS_REPORT_SCHEMA: &str =
    "rusty.studio.shell_handoff_readiness_report.v1";
pub const SHELL_HANDOFF_MANIFEST_SCHEMA: &str = "rusty.studio.shell_handoff_manifest.v1";
pub const SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA: &str =
    "rusty.studio.shell_handoff_manifest_validation_report.v1";
pub const SHELL_HANDOFF_INTAKE_REPORT_SCHEMA: &str = "rusty.studio.shell_handoff_intake_report.v1";
pub const SHELL_RUNBOOK_REPORT_SCHEMA: &str = "rusty.studio.shell_runbook_report.v1";
pub const SHELL_EXPORT_PACKAGE_REPORT_SCHEMA: &str = "rusty.studio.shell_export_package_report.v1";
pub const SHELL_EXPORT_PACKAGE_COMPARISON_SCHEMA: &str =
    "rusty.studio.shell_export_package_comparison.v1";
pub const SHELL_EXPORT_PACKAGE_BASELINE_MANIFEST_SCHEMA: &str =
    "rusty.studio.shell_export_package_baseline_manifest.v1";
pub const SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA: &str =
    "rusty.studio.shell_export_package_baseline_index.v1";
pub const SHELL_EXPORT_PACKAGE_BASELINE_SELECTION_SCHEMA: &str =
    "rusty.studio.shell_export_package_baseline_selection.v1";
pub const SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA: &str =
    "rusty.studio.shell_handoff_acceptance_checklist.v1";
pub const SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA: &str =
    "rusty.studio.shell_handoff_acceptance_summary.v1";
pub const SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA: &str =
    "rusty.studio.shell_handoff_acceptance_baseline_manifest.v1";
pub const SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA: &str =
    "rusty.studio.shell_handoff_acceptance_baseline_index.v1";
pub const SHELL_HANDOFF_ACCEPTANCE_BASELINE_SELECTION_SCHEMA: &str =
    "rusty.studio.shell_handoff_acceptance_baseline_selection.v1";
pub const SHELL_HANDOFF_ACCEPTANCE_COMPARISON_SCHEMA: &str =
    "rusty.studio.shell_handoff_acceptance_comparison.v1";
pub const SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA: &str =
    "rusty.studio.shell_release_candidate_review.v1";
pub const SHELL_RELEASE_CANDIDATE_REVIEW_MANIFEST_SCHEMA: &str =
    "rusty.studio.shell_release_candidate_review_manifest.v1";
pub const SHELL_RELEASE_CANDIDATE_REVIEW_INDEX_SCHEMA: &str =
    "rusty.studio.shell_release_candidate_review_index.v1";
pub const SHELL_RELEASE_CANDIDATE_REVIEW_SELECTION_SCHEMA: &str =
    "rusty.studio.shell_release_candidate_review_selection.v1";
pub const SHELL_HOSTESS_HANDOFF_PACKAGE_SCHEMA: &str =
    "rusty.studio.shell_hostess_handoff_package.v1";
pub const SHELL_HOSTESS_OWNER_INTAKE_SCHEMA: &str = "rusty.studio.shell_hostess_owner_intake.v1";
pub const SHELL_HOSTESS_STAGING_PREVIEW_MANIFEST_SCHEMA: &str =
    "rusty.studio.shell_hostess_staging_preview_manifest.v1";
pub const SHELL_HOSTESS_STAGING_FILE_PLAN_SCHEMA: &str =
    "rusty.studio.shell_hostess_staging_file_plan.v1";
pub const SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA: &str =
    "rusty.studio.shell_hostess_staging_handoff_envelope.v1";
pub const SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA: &str =
    "rusty.studio.shell_hostess_staging_acceptance_checklist.v1";
pub const SHELL_HOSTESS_STAGING_ACCEPTANCE_MANIFEST_SCHEMA: &str =
    "rusty.studio.shell_hostess_staging_acceptance_manifest.v1";
pub const SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA: &str =
    "rusty.studio.shell_hostess_staging_acceptance_index.v1";
pub const SHELL_HOSTESS_STAGING_ACCEPTANCE_SELECTION_SCHEMA: &str =
    "rusty.studio.shell_hostess_staging_acceptance_selection.v1";
pub const SHELL_HOSTESS_STAGING_ACCEPTANCE_COMPARISON_SCHEMA: &str =
    "rusty.studio.shell_hostess_staging_acceptance_comparison.v1";
pub const SHELL_HOSTESS_STAGING_EXECUTION_REQUEST_SCHEMA: &str =
    "rusty.studio.shell_hostess_staging_execution_request.v1";
pub const SHELL_HOSTESS_STAGING_EXECUTION_ACK_SCHEMA: &str =
    "rusty.studio.shell_hostess_staging_execution_ack.v1";
pub const SHELL_HOSTESS_STAGING_EXECUTION_REJECT_SCHEMA: &str =
    "rusty.studio.shell_hostess_staging_execution_reject.v1";
pub const MANIFOLD_PACKAGE_VALIDATION_REPORT_SCHEMA: &str =
    "rusty.manifold.package.validation_report.v1";
pub const PACKAGE_EVIDENCE_INTAKE_REPORT_SCHEMA: &str =
    "rusty.studio.package_evidence_intake_report.v1";
pub const MOTION_BREATH_PROFILE_SCHEMA: &str = "rusty.motion_breath_profile.v1";
pub const PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_DESCRIPTOR_SCHEMA: &str =
    "rusty.manifold.projected_motion_breath.source_adapter_descriptors.v1";
pub const PROJECTED_MOTION_BREATH_SOURCE_BINDING_SCHEMA: &str =
    "rusty.manifold.projected_motion_breath.source_binding.v1";
pub const PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_CASE_SCHEMA: &str =
    "rusty.manifold.projected_motion_breath.adapter_normalization_case.v1";
pub const PROJECTED_MOTION_BREATH_AUTHORING_REVIEW_SCHEMA: &str =
    "rusty.studio.projected_motion_breath_authoring_review.v1";
pub const PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_SELECTION_REVIEW_SCHEMA: &str =
    "rusty.studio.projected_motion_breath_source_adapter_selection_review.v1";
pub const PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_EVIDENCE_REVIEW_SCHEMA: &str =
    "rusty.studio.projected_motion_breath_adapter_normalization_evidence_review.v1";
pub const PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA: &str =
    "rusty.studio.projected_motion_breath_shell_handoff_review.v1";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StudioProject {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub project_id: String,
    pub revision: u64,
    pub display_name: String,
    pub package_catalog_path: String,
    pub host_run_profile_paths: Vec<String>,
    pub graphs: Vec<StudioGraph>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct StudioGraph {
    pub graph_id: String,
    pub display_name: String,
    pub target_host_profile: String,
    pub nodes: Vec<StudioNode>,
    pub edges: Vec<StudioEdge>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub layout: Option<StudioGraphLayout>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioNode {
    pub node_id: String,
    pub kind: StudioNodeKind,
    pub reference_id: String,
    pub label: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioNodeKind {
    Package,
    Module,
    HostProfile,
    ValidationSlot,
    OperatorShell,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioEdge {
    pub edge_id: String,
    pub kind: StudioEdgeKind,
    pub source_node_id: String,
    pub target_node_id: String,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioEdgeKind {
    PackageProvidesModule,
    StreamBinding,
    CommandBinding,
    ValidationSlotUsesPackage,
    ShellTargetsHostProfile,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioBindingKind {
    Stream,
    Command,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioGraphLayout {
    pub layout_id: String,
    pub coordinate_space: String,
    pub nodes: Vec<StudioNodeLayout>,
    pub edges: Vec<StudioEdgeLayout>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioNodeLayout {
    pub node_id: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioEdgeLayout {
    pub edge_id: String,
    pub route: StudioEdgeRouteKind,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioEdgeRouteKind {
    Direct,
    Orthogonal,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioValidationReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub revision: u64,
    pub status: StudioValidationStatus,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioValidationCheck {
    pub check_id: String,
    pub status: StudioValidationStatus,
    pub evidence: String,
    pub issue_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edge_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reference_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioValidationStatus {
    Pass,
    Fail,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioEditReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub original_revision: u64,
    pub resulting_revision: u64,
    pub operation: StudioEditOperation,
    pub status: StudioEditStatus,
    pub issue_code: Option<String>,
    pub message: String,
    pub graph_id: String,
    pub requested_reference_id: String,
    pub requested_host_profile: String,
    pub changed_fields: Vec<String>,
    pub validation: StudioValidationReport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioEditOperation {
    RetargetHost,
    AddModule,
    RemoveModule,
    AddBinding,
    RemoveBinding,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioEditStatus {
    Applied,
    Rejected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellDescriptorReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub revision: u64,
    pub status: StudioShellDescriptorStatus,
    pub issue_code: Option<String>,
    pub message: String,
    pub graph_id: String,
    pub validation: StudioValidationReport,
    pub descriptor: Option<StudioShellDescriptor>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellDescriptorStatus {
    Exported,
    Rejected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellDescriptorValidationReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub descriptor_id: String,
    pub status: StudioValidationStatus,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellDescriptor {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub descriptor_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub graph_id: String,
    pub display_name: String,
    pub shell_id: String,
    pub shell_label: String,
    pub target_host_profile: String,
    pub host_profile: StudioShellHostProfile,
    pub package_ids: Vec<String>,
    pub module_ids: Vec<String>,
    pub validation_slot_ids: Vec<String>,
    pub stream_bindings: Vec<StudioShellBinding>,
    pub command_bindings: Vec<StudioShellBinding>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostProfile {
    pub profile_id: String,
    pub host_profile: Option<String>,
    pub app_id: Option<String>,
    pub install_route: Option<String>,
    pub launch_route: Option<String>,
    pub command_bridge: Option<String>,
    pub evidence_pull_route: Option<String>,
    pub required_permissions: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellBinding {
    pub binding_id: String,
    pub source_node_id: String,
    pub target_node_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellArtifactReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub revision: u64,
    pub status: StudioShellArtifactStatus,
    pub issue_code: Option<String>,
    pub message: String,
    pub validation: StudioValidationReport,
    pub manifest: Option<StudioShellArtifactManifest>,
    pub descriptors: Vec<StudioShellDescriptor>,
    pub rejections: Vec<StudioShellArtifactRejection>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellArtifactStatus {
    Exported,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellArtifactManifest {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub artifacts: Vec<StudioShellArtifact>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellArtifact {
    pub artifact_id: String,
    pub graph_id: String,
    pub shell_id: String,
    pub target_kind: StudioShellTargetKind,
    pub target_host_profile: String,
    pub host_profile_class: Option<String>,
    pub descriptor_path: String,
    pub app_id: Option<String>,
    pub install_route: Option<String>,
    pub launch_route: Option<String>,
    pub command_bridge: Option<String>,
    pub evidence_pull_route: Option<String>,
    pub package_ids: Vec<String>,
    pub module_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellTargetKind {
    Desktop,
    Phone,
    Quest,
    Unknown,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellArtifactRejection {
    pub graph_id: String,
    pub issue_code: Option<String>,
    pub message: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellArtifactManifestValidationReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub manifest_id: String,
    pub status: StudioValidationStatus,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellTemplateReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub status: StudioShellTemplateStatus,
    pub issue_code: Option<String>,
    pub message: String,
    pub validation: StudioShellArtifactManifestValidationReport,
    pub index: Option<StudioShellTemplateIndex>,
    pub templates: Vec<StudioShellTemplateManifest>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellTemplateStatus {
    Exported,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellTemplateIndex {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub index_id: String,
    pub manifest_id: String,
    pub project_id: String,
    pub project_revision: u64,
    pub templates: Vec<StudioShellTemplateIndexEntry>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellTemplateIndexEntry {
    pub template_id: String,
    pub artifact_id: String,
    pub graph_id: String,
    pub shell_id: String,
    pub target_kind: StudioShellTargetKind,
    pub template_path: String,
    pub descriptor_path: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellTemplateManifest {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub template_id: String,
    pub artifact_id: String,
    pub graph_id: String,
    pub shell_id: String,
    pub target_kind: StudioShellTargetKind,
    pub target_host_profile: String,
    pub host_profile_class: Option<String>,
    pub source_descriptor_path: String,
    pub descriptor_path: String,
    pub runtime_authority: StudioShellRuntimeAuthority,
    pub host_routes: StudioShellHostRoutes,
    pub package_ids: Vec<String>,
    pub module_ids: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellRuntimeAuthority {
    pub command_session_authority: String,
    pub install_launch_evidence_authority: String,
    pub studio_role: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostRoutes {
    pub app_id: Option<String>,
    pub install_route: Option<String>,
    pub launch_route: Option<String>,
    pub command_bridge: Option<String>,
    pub evidence_pull_route: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellTemplateIndexValidationReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub index_id: String,
    pub status: StudioValidationStatus,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellBundleReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub revision: u64,
    pub graph_id: String,
    pub status: StudioShellBundleStatus,
    pub issue_code: Option<String>,
    pub message: String,
    pub bundle_files: Vec<String>,
    pub descriptor_validation: Option<StudioShellDescriptorValidationReport>,
    pub artifact_validation: Option<StudioShellArtifactManifestValidationReport>,
    pub template_validation: Option<StudioShellTemplateIndexValidationReport>,
    pub descriptor: Option<StudioShellDescriptor>,
    pub artifact_manifest: Option<StudioShellArtifactManifest>,
    pub template_index: Option<StudioShellTemplateIndex>,
    pub template_manifest: Option<StudioShellTemplateManifest>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellBundleStatus {
    Exported,
    Rejected,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellBundleValidationReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub revision: u64,
    pub graph_id: String,
    pub status: StudioValidationStatus,
    pub expected_bundle_files: Vec<String>,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessHandoffPackageReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_index_schema: String,
    pub index_path: Option<String>,
    pub requested_candidate_id: Option<String>,
    pub default_candidate_id: Option<String>,
    pub selected_candidate_id: Option<String>,
    pub selection_status: StudioShellReleaseCandidateReviewSelectionStatus,
    pub selection_issue_code: Option<String>,
    pub candidate_manifest_schema: Option<String>,
    pub candidate_manifest_path: Option<String>,
    pub candidate_id: Option<String>,
    pub candidate_label: Option<String>,
    pub review_schema: Option<String>,
    pub review_path: Option<String>,
    pub handoff_manifest_schema: Option<String>,
    pub handoff_manifest_path: Option<String>,
    pub manifest_id: Option<String>,
    pub project_id: Option<String>,
    pub project_revision: Option<u64>,
    pub status: StudioShellHostessHandoffPackageStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub handoff_owner: String,
    pub review_owner: Option<String>,
    pub command_session_authority: Option<String>,
    pub install_launch_evidence_authority: Option<String>,
    pub studio_role: Option<String>,
    pub handoff_ready_count: usize,
    pub handoff_failed_count: usize,
    pub handoff_missing_bundle_count: usize,
    pub acceptance_baseline_id: Option<String>,
    pub acceptance_baseline_status: Option<StudioShellHandoffAcceptanceBaselineSelectionStatus>,
    pub acceptance_comparison_status: Option<StudioShellHandoffAcceptanceComparisonStatus>,
    pub export_package_baseline_id: Option<String>,
    pub export_package_baseline_status: Option<StudioShellExportPackageBaselineSelectionStatus>,
    pub export_package_comparison_status: Option<StudioShellExportPackageComparisonStatus>,
    pub required_owner_actions: Vec<StudioShellHostessHandoffPackageAction>,
    pub prohibited_actions: Vec<String>,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessHandoffPackageStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessHandoffPackageAction {
    pub action_id: String,
    pub owner: String,
    pub status: StudioShellHostessHandoffPackageActionStatus,
    pub source: String,
    pub next_required_action: String,
    pub prohibited_in_studio: bool,
    pub issue_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessHandoffPackageActionStatus {
    Ready,
    Blocked,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessOwnerIntakeReport {
    #[serde(rename = "$schema")]
    pub schema_id: String,
    pub source_package_schema: String,
    pub package_path: Option<String>,
    pub selected_candidate_id: Option<String>,
    pub candidate_manifest_path: Option<String>,
    pub review_path: Option<String>,
    pub handoff_manifest_path: Option<String>,
    pub manifest_id: Option<String>,
    pub project_id: Option<String>,
    pub project_revision: Option<u64>,
    pub status: StudioShellHostessOwnerIntakeStatus,
    pub issue_code: Option<String>,
    pub execution_policy: String,
    pub intake_owner: String,
    pub handoff_owner: String,
    pub review_owner: Option<String>,
    pub command_session_authority: Option<String>,
    pub install_launch_evidence_authority: Option<String>,
    pub studio_role: Option<String>,
    pub handoff_ready_count: usize,
    pub handoff_failed_count: usize,
    pub handoff_missing_bundle_count: usize,
    pub acceptance_baseline_id: Option<String>,
    pub acceptance_baseline_status: Option<StudioShellHandoffAcceptanceBaselineSelectionStatus>,
    pub acceptance_comparison_status: Option<StudioShellHandoffAcceptanceComparisonStatus>,
    pub export_package_baseline_id: Option<String>,
    pub export_package_baseline_status: Option<StudioShellExportPackageBaselineSelectionStatus>,
    pub export_package_comparison_status: Option<StudioShellExportPackageComparisonStatus>,
    pub source_owner_action_count: usize,
    pub ready_assignment_count: usize,
    pub blocked_assignment_count: usize,
    pub hostess_ready_action_count: usize,
    pub manifold_ready_action_count: usize,
    pub assignments: Vec<StudioShellHostessOwnerIntakeAssignment>,
    pub prohibited_actions: Vec<String>,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessOwnerIntakeStatus {
    Ready,
    Blocked,
    Rejected,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct StudioShellHostessOwnerIntakeAssignment {
    pub action_id: String,
    pub owner: String,
    pub status: StudioShellHostessOwnerIntakeAssignmentStatus,
    pub request_kind: String,
    pub source: String,
    pub next_required_action: String,
    pub prohibited_in_studio: bool,
    pub issue_code: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StudioShellHostessOwnerIntakeAssignmentStatus {
    Ready,
    Blocked,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioResolvedProject {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub revision: u64,
    pub graphs: Vec<StudioResolvedGraph>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioResolvedGraph {
    pub graph_id: String,
    pub target_host_profile: String,
    pub package_count: usize,
    pub module_count: usize,
    pub operator_shell_count: usize,
    pub node_count: usize,
    pub edge_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioExportPlan {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub revision: u64,
    pub bundles: Vec<StudioExportBundle>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioExportBundle {
    pub bundle_id: String,
    pub graph_id: String,
    pub target_host_profile: String,
    pub package_ids: Vec<String>,
    pub module_ids: Vec<String>,
    pub operator_shell_ids: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioViewModel {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub revision: u64,
    pub display_name: String,
    pub validation_status: StudioValidationStatus,
    pub validation_pass_count: usize,
    pub validation_fail_count: usize,
    pub validation_issues: Vec<StudioValidationIssueView>,
    pub focused_issue: Option<StudioIssueFocusView>,
    pub requested_issue_check_id: Option<String>,
    pub selected_issue_index: Option<usize>,
    pub selected_issue_check_id: Option<String>,
    pub issue_selection_code: Option<String>,
    pub graph_count: usize,
    pub requested_graph_id: Option<String>,
    pub selected_graph_index: Option<usize>,
    pub selected_graph_id: Option<String>,
    pub selection_issue_code: Option<String>,
    pub requested_node_id: Option<String>,
    pub selected_node_id: Option<String>,
    pub node_selection_code: Option<String>,
    pub selected_node: Option<StudioNodeInspectorView>,
    pub requested_edge_id: Option<String>,
    pub selected_edge_id: Option<String>,
    pub edge_selection_code: Option<String>,
    pub selected_edge: Option<StudioEdgeInspectorView>,
    pub shell_preview: Option<StudioShellPreviewView>,
    pub catalog_package_count: usize,
    pub catalog_module_count: usize,
    pub host_profile_count: usize,
    pub catalog_packages: Vec<StudioCatalogPackageView>,
    pub host_profiles: Vec<StudioHostProfileView>,
    pub graphs: Vec<StudioGraphView>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioValidationIssueView {
    pub check_id: String,
    pub issue_code: Option<String>,
    pub evidence: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub edge_ids: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reference_ids: Vec<String>,
    pub targets_selected_graph: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioIssueFocusView {
    pub issue_index: usize,
    pub check_id: String,
    pub issue_code: Option<String>,
    pub evidence: String,
    pub graph_id: String,
    pub node_id: Option<String>,
    pub edge_id: Option<String>,
    pub reference_id: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioNodeInspectorView {
    pub graph_id: String,
    pub node_id: String,
    pub kind: String,
    pub reference_id: String,
    pub label: String,
    pub validation_issue_count: usize,
    pub reference_status: String,
    pub package_manifest_path: Option<String>,
    pub package_module_ids: Vec<String>,
    pub module_package_ids: Vec<String>,
    pub host_profile: Option<StudioNodeHostProfileView>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioNodeHostProfileView {
    pub profile_id: String,
    pub host_profile: Option<String>,
    pub app_id: Option<String>,
    pub install_route: Option<String>,
    pub launch_route: Option<String>,
    pub command_bridge: Option<String>,
    pub evidence_pull_route: Option<String>,
    pub required_permissions: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioEdgeInspectorView {
    pub graph_id: String,
    pub edge_id: String,
    pub kind: String,
    pub source_node_id: String,
    pub source_label: Option<String>,
    pub source_kind: Option<String>,
    pub source_reference_id: Option<String>,
    pub target_node_id: String,
    pub target_label: Option<String>,
    pub target_kind: Option<String>,
    pub target_reference_id: Option<String>,
    pub validation_issue_count: usize,
    pub endpoint_status: String,
    pub binding_kind: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioShellPreviewView {
    pub graph_id: String,
    pub status: StudioShellDescriptorStatus,
    pub issue_code: Option<String>,
    pub message: String,
    pub descriptor_id: Option<String>,
    pub descriptor_path: Option<String>,
    pub shell_id: Option<String>,
    pub shell_label: Option<String>,
    pub target_host_profile: Option<String>,
    pub target_kind: Option<StudioShellTargetKind>,
    pub host_profile_class: Option<String>,
    pub app_id: Option<String>,
    pub install_route: Option<String>,
    pub launch_route: Option<String>,
    pub command_bridge: Option<String>,
    pub evidence_pull_route: Option<String>,
    pub package_count: usize,
    pub module_count: usize,
    pub validation_slot_count: usize,
    pub stream_binding_count: usize,
    pub command_binding_count: usize,
    pub descriptor_validation_status: Option<StudioValidationStatus>,
    pub template_id: Option<String>,
    pub template_path: Option<String>,
    pub template_descriptor_path: Option<String>,
    pub runtime_command_authority: Option<String>,
    pub runtime_host_authority: Option<String>,
    pub studio_role: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioCatalogPackageView {
    pub package_id: String,
    pub manifest_path: String,
    pub module_count: usize,
    pub module_ids: Vec<String>,
    pub in_selected_graph: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioHostProfileView {
    pub profile_id: String,
    pub host_profile: Option<String>,
    pub app_id: Option<String>,
    pub install_route: Option<String>,
    pub launch_route: Option<String>,
    pub command_bridge: Option<String>,
    pub evidence_pull_route: Option<String>,
    pub required_permissions: Vec<String>,
    pub targets_selected_graph: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioGraphView {
    pub graph_id: String,
    pub display_name: String,
    pub target_host_profile: String,
    pub validation_issue_count: usize,
    pub node_count: usize,
    pub edge_count: usize,
    pub package_count: usize,
    pub module_count: usize,
    pub operator_shell_count: usize,
    pub node_rows: Vec<StudioNodeView>,
    pub edge_rows: Vec<StudioEdgeView>,
    pub layout: Option<StudioGraphLayoutView>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioNodeView {
    pub node_id: String,
    pub kind: String,
    pub reference_id: String,
    pub label: String,
    pub validation_issue_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioEdgeView {
    pub edge_id: String,
    pub kind: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub validation_issue_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioGraphLayoutView {
    pub layout_id: String,
    pub coordinate_space: String,
    pub node_count: usize,
    pub edge_count: usize,
    pub nodes: Vec<StudioNodeLayoutView>,
    pub edges: Vec<StudioEdgeLayoutView>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioNodeLayoutView {
    pub node_id: String,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub validation_issue_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioEdgeLayoutView {
    pub edge_id: String,
    pub route: String,
    pub validation_issue_count: usize,
}
