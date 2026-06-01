use serde::{Deserialize, Serialize};

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

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioValidationReport {
    #[serde(rename = "$schema")]
    pub schema_id: &'static str,
    pub project_id: String,
    pub revision: u64,
    pub status: StudioValidationStatus,
    pub checks: Vec<StudioValidationCheck>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct StudioValidationCheck {
    pub check_id: String,
    pub status: StudioValidationStatus,
    pub evidence: String,
    pub issue_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graph_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub node_ids: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub edge_ids: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub reference_ids: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
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
    pub graph_count: usize,
    pub requested_graph_id: Option<String>,
    pub selected_graph_index: Option<usize>,
    pub selected_graph_id: Option<String>,
    pub selection_issue_code: Option<String>,
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
    pub check_id: String,
    pub issue_code: Option<String>,
    pub evidence: String,
    pub graph_id: String,
    pub node_id: Option<String>,
    pub edge_id: Option<String>,
    pub reference_id: Option<String>,
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
