use rusty_studio_model::{
    StudioBindingKind, StudioEdge, StudioEdgeKind, StudioEdgeRouteKind, StudioEditOperation,
    StudioEditReport, StudioEditStatus, StudioExportBundle, StudioExportPlan, StudioGraph,
    StudioHostProfileView, StudioIssueFocusView, StudioManifoldPackageValidationCheck,
    StudioManifoldPackageValidationReport, StudioNode, StudioNodeKind,
    StudioPackageEvidenceIntakeDecision, StudioPackageEvidenceIntakeEntry,
    StudioPackageEvidenceIntakeReport, StudioPackageEvidenceIntakeStatus, StudioProject,
    StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewReport,
    StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus,
    StudioProjectedMotionBreathAuthoringReviewReport,
    StudioProjectedMotionBreathAuthoringReviewStatus,
    StudioProjectedMotionBreathShellHandoffReviewReport,
    StudioProjectedMotionBreathShellHandoffReviewStatus,
    StudioProjectedMotionBreathSourceAdapterSelectionReviewReport,
    StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus, StudioResolvedGraph,
    StudioResolvedProject, StudioShellArtifact, StudioShellArtifactManifest,
    StudioShellArtifactManifestValidationReport, StudioShellArtifactRejection,
    StudioShellArtifactReport, StudioShellArtifactStatus, StudioShellBinding,
    StudioShellBundleReport, StudioShellBundleStatus, StudioShellBundleValidationReport,
    StudioShellDescriptor, StudioShellDescriptorReport, StudioShellDescriptorStatus,
    StudioShellDescriptorValidationReport, StudioShellExportPackageBaselineIndex,
    StudioShellExportPackageBaselineIndexEntry, StudioShellExportPackageBaselineManifest,
    StudioShellExportPackageBaselineSelectionEntry,
    StudioShellExportPackageBaselineSelectionReport,
    StudioShellExportPackageBaselineSelectionStatus, StudioShellExportPackageComparisonChange,
    StudioShellExportPackageComparisonEntry, StudioShellExportPackageComparisonReport,
    StudioShellExportPackageComparisonStatus, StudioShellExportPackageDescriptorRef,
    StudioShellExportPackageEntry, StudioShellExportPackageReport, StudioShellExportPackageStatus,
    StudioShellExportPackageTargetSummary, StudioShellExportPackageTemplateRef,
    StudioShellHandoffAcceptanceBaselineIndex, StudioShellHandoffAcceptanceBaselineIndexEntry,
    StudioShellHandoffAcceptanceBaselineManifest,
    StudioShellHandoffAcceptanceBaselineSelectionEntry,
    StudioShellHandoffAcceptanceBaselineSelectionReport,
    StudioShellHandoffAcceptanceBaselineSelectionStatus, StudioShellHandoffAcceptanceCheck,
    StudioShellHandoffAcceptanceChecklistEntry, StudioShellHandoffAcceptanceChecklistReport,
    StudioShellHandoffAcceptanceComparisonChange, StudioShellHandoffAcceptanceComparisonEntry,
    StudioShellHandoffAcceptanceComparisonReport, StudioShellHandoffAcceptanceComparisonStatus,
    StudioShellHandoffAcceptanceStatus, StudioShellHandoffAcceptanceSummaryReport,
    StudioShellHandoffAcceptanceTargetSummary, StudioShellHandoffIntakeDecision,
    StudioShellHandoffIntakeEntry, StudioShellHandoffIntakeReport, StudioShellHandoffIntakeStatus,
    StudioShellHandoffIntakeTargetSummary, StudioShellHandoffKind, StudioShellHandoffManifest,
    StudioShellHandoffManifestEntry, StudioShellHandoffManifestTarget,
    StudioShellHandoffManifestValidationReport, StudioShellHandoffReadinessEntry,
    StudioShellHandoffReadinessReport, StudioShellHandoffReadinessTargetSummary,
    StudioShellHandoffReport, StudioShellHostProfile, StudioShellHostRoutes,
    StudioShellHostessHandoffPackageAction, StudioShellHostessHandoffPackageActionStatus,
    StudioShellHostessHandoffPackageReport, StudioShellHostessHandoffPackageStatus,
    StudioShellHostessOwnerIntakeAssignment, StudioShellHostessOwnerIntakeAssignmentStatus,
    StudioShellHostessOwnerIntakeReport, StudioShellHostessOwnerIntakeStatus,
    StudioShellHostessStagingAcceptanceChecklistEntry,
    StudioShellHostessStagingAcceptanceChecklistReport,
    StudioShellHostessStagingAcceptanceComparisonChange,
    StudioShellHostessStagingAcceptanceComparisonEntry,
    StudioShellHostessStagingAcceptanceComparisonReport,
    StudioShellHostessStagingAcceptanceComparisonStatus, StudioShellHostessStagingAcceptanceIndex,
    StudioShellHostessStagingAcceptanceIndexEntry, StudioShellHostessStagingAcceptanceManifest,
    StudioShellHostessStagingAcceptanceSelectionEntry,
    StudioShellHostessStagingAcceptanceSelectionReport,
    StudioShellHostessStagingAcceptanceSelectionStatus, StudioShellHostessStagingAcceptanceStatus,
    StudioShellHostessStagingExecutionAck, StudioShellHostessStagingExecutionAckStatus,
    StudioShellHostessStagingExecutionAction, StudioShellHostessStagingExecutionActionStatus,
    StudioShellHostessStagingExecutionReject, StudioShellHostessStagingExecutionRejectStatus,
    StudioShellHostessStagingExecutionRequestReport,
    StudioShellHostessStagingExecutionRequestStatus, StudioShellHostessStagingFilePlan,
    StudioShellHostessStagingFilePlanStatus, StudioShellHostessStagingFileRequest,
    StudioShellHostessStagingFileRequestStatus, StudioShellHostessStagingHandoffEnvelope,
    StudioShellHostessStagingHandoffEnvelopeStatus, StudioShellHostessStagingHandoffInstruction,
    StudioShellHostessStagingHandoffInstructionStatus, StudioShellHostessStagingHandoffProvenance,
    StudioShellHostessStagingHandoffRequestSummary, StudioShellHostessStagingPlannedFile,
    StudioShellHostessStagingPreviewArtifact, StudioShellHostessStagingPreviewGroup,
    StudioShellHostessStagingPreviewGroupStatus, StudioShellHostessStagingPreviewManifest,
    StudioShellHostessStagingPreviewStatus, StudioShellReleaseCandidateReviewIndex,
    StudioShellReleaseCandidateReviewIndexEntry, StudioShellReleaseCandidateReviewManifest,
    StudioShellReleaseCandidateReviewReport, StudioShellReleaseCandidateReviewSelectionEntry,
    StudioShellReleaseCandidateReviewSelectionReport,
    StudioShellReleaseCandidateReviewSelectionStatus, StudioShellReleaseCandidateReviewStatus,
    StudioShellRunbookEntry, StudioShellRunbookReport, StudioShellRunbookStatus,
    StudioShellRunbookTargetSummary, StudioShellRuntimeAuthority, StudioShellTargetKind,
    StudioShellTemplateIndex, StudioShellTemplateIndexEntry,
    StudioShellTemplateIndexValidationReport, StudioShellTemplateManifest,
    StudioShellTemplateReport, StudioShellTemplateStatus, StudioValidationCheck,
    StudioValidationIssueView, StudioValidationReport, StudioValidationStatus, StudioViewModel,
    EDIT_REPORT_SCHEMA, EXPORT_PLAN_SCHEMA, MANIFOLD_PACKAGE_VALIDATION_REPORT_SCHEMA,
    MOTION_BREATH_PROFILE_SCHEMA, PACKAGE_EVIDENCE_INTAKE_REPORT_SCHEMA,
    PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_CASE_SCHEMA,
    PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_EVIDENCE_REVIEW_SCHEMA,
    PROJECTED_MOTION_BREATH_AUTHORING_REVIEW_SCHEMA,
    PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA,
    PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_DESCRIPTOR_SCHEMA,
    PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_SELECTION_REVIEW_SCHEMA,
    PROJECTED_MOTION_BREATH_SOURCE_BINDING_SCHEMA, PROJECT_SCHEMA, RESOLVED_PROJECT_SCHEMA,
    SHELL_ARTIFACT_MANIFEST_SCHEMA, SHELL_ARTIFACT_MANIFEST_VALIDATION_REPORT_SCHEMA,
    SHELL_ARTIFACT_REPORT_SCHEMA, SHELL_BUNDLE_REPORT_SCHEMA,
    SHELL_BUNDLE_VALIDATION_REPORT_SCHEMA, SHELL_DESCRIPTOR_REPORT_SCHEMA, SHELL_DESCRIPTOR_SCHEMA,
    SHELL_DESCRIPTOR_VALIDATION_REPORT_SCHEMA, SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA,
    SHELL_EXPORT_PACKAGE_BASELINE_MANIFEST_SCHEMA, SHELL_EXPORT_PACKAGE_BASELINE_SELECTION_SCHEMA,
    SHELL_EXPORT_PACKAGE_COMPARISON_SCHEMA, SHELL_EXPORT_PACKAGE_REPORT_SCHEMA,
    SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA,
    SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA,
    SHELL_HANDOFF_ACCEPTANCE_BASELINE_SELECTION_SCHEMA, SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA,
    SHELL_HANDOFF_ACCEPTANCE_COMPARISON_SCHEMA, SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA,
    SHELL_HANDOFF_INTAKE_REPORT_SCHEMA, SHELL_HANDOFF_MANIFEST_SCHEMA,
    SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA, SHELL_HANDOFF_READINESS_REPORT_SCHEMA,
    SHELL_HANDOFF_REPORT_SCHEMA, SHELL_HOSTESS_HANDOFF_PACKAGE_SCHEMA,
    SHELL_HOSTESS_OWNER_INTAKE_SCHEMA, SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA,
    SHELL_HOSTESS_STAGING_ACCEPTANCE_COMPARISON_SCHEMA,
    SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA,
    SHELL_HOSTESS_STAGING_ACCEPTANCE_MANIFEST_SCHEMA,
    SHELL_HOSTESS_STAGING_ACCEPTANCE_SELECTION_SCHEMA, SHELL_HOSTESS_STAGING_EXECUTION_ACK_SCHEMA,
    SHELL_HOSTESS_STAGING_EXECUTION_REJECT_SCHEMA, SHELL_HOSTESS_STAGING_EXECUTION_REQUEST_SCHEMA,
    SHELL_HOSTESS_STAGING_FILE_PLAN_SCHEMA, SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA,
    SHELL_HOSTESS_STAGING_PREVIEW_MANIFEST_SCHEMA, SHELL_RELEASE_CANDIDATE_REVIEW_INDEX_SCHEMA,
    SHELL_RELEASE_CANDIDATE_REVIEW_MANIFEST_SCHEMA, SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA,
    SHELL_RELEASE_CANDIDATE_REVIEW_SELECTION_SCHEMA, SHELL_RUNBOOK_REPORT_SCHEMA,
    SHELL_TEMPLATE_INDEX_SCHEMA, SHELL_TEMPLATE_INDEX_VALIDATION_REPORT_SCHEMA,
    SHELL_TEMPLATE_MANIFEST_SCHEMA, SHELL_TEMPLATE_REPORT_SCHEMA, VALIDATION_REPORT_SCHEMA,
    VIEW_MODEL_SCHEMA,
};
use rusty_studio_model::{
    StudioCatalogPackageView, StudioEdgeInspectorView, StudioEdgeLayoutView, StudioEdgeView,
    StudioGraphLayoutView, StudioGraphView, StudioNodeHostProfileView, StudioNodeInspectorView,
    StudioNodeLayoutView, StudioNodeView, StudioShellPreviewView,
};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

mod error;
mod io;

pub use error::StudioCoreError;
pub use io::*;

const NEXT_PALETTE_MODULE_REQUEST: &str = "module.palette.next_available";
const PROJECTED_MOTION_BREATH_PACKAGE_ID: &str = "package.projected_motion_breath";
const PROJECTED_MOTION_BREATH_MODULE_ID: &str = "module.breath.projected_motion";
const MANIFOLD_SHELL_HANDOFF_SCHEMA: &str = "rusty.manifold.shell.handoff.v1";
const DEFAULT_MANIFOLD_SHELL_HANDOFF_VALIDATION_SLOT_ID: &str = "host_run.slot.synthetic_smoke";
const PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_CHECK_SUFFIX: &str =
    "projected_motion_adapter_normalization";
const PROJECTED_MOTION_BREATH_REQUIRED_CHECK_SUFFIXES: [&str; 3] = [
    "projected_motion_contract",
    "projected_motion_profile_commands",
    "projected_motion_goldens",
];

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct StudioGeneratedManifoldShellHandoffManifest {
    #[serde(rename = "$schema")]
    schema_id: &'static str,
    handoff_id: String,
    handoff_revision: u64,
    target_host_profile: String,
    shell_app_id: String,
    validation_slot_id: String,
    stream_bindings: Vec<StudioGeneratedManifoldShellStreamBinding>,
    command_ids: Vec<String>,
    transport_offers: Vec<StudioGeneratedManifoldTransportOffer>,
    expected_scorecard_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct StudioGeneratedManifoldShellStreamBinding {
    stream_id: String,
    direction: StudioGeneratedManifoldShellStreamDirection,
    role: String,
    required: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum StudioGeneratedManifoldShellStreamDirection {
    Publish,
    Subscribe,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
struct StudioGeneratedManifoldTransportOffer {
    transport_id: String,
    transport: StudioGeneratedManifoldEndpointTransport,
    endpoint_id: Option<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
enum StudioGeneratedManifoldEndpointTransport {
    InProcess,
    Stdio,
    Http,
}

pub fn validate_project(project: &StudioProject) -> StudioValidationReport {
    validate_project_with_base(project, None)
}

pub fn validate_project_with_base(
    project: &StudioProject,
    base_dir: Option<&Path>,
) -> StudioValidationReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.schema",
        project.schema_id == PROJECT_SCHEMA,
        "project schema id is supported",
        "unsupported project schema id",
        "studio.issue.unsupported_schema",
    );
    push_check(
        &mut checks,
        "studio.check.project_id",
        is_dotted_id(&project.project_id),
        "project id uses dotted-id grammar",
        "project id is not a dotted id",
        "studio.issue.invalid_project_id",
    );
    push_check(
        &mut checks,
        "studio.check.revision",
        project.revision > 0,
        "project revision is positive",
        "project revision must be positive",
        "studio.issue.invalid_revision",
    );
    push_check(
        &mut checks,
        "studio.check.graphs_present",
        !project.graphs.is_empty(),
        "project contains at least one graph",
        "project must contain at least one graph",
        "studio.issue.no_graphs",
    );

    let reference_index = validate_project_references(project, base_dir, &mut checks);
    for graph in &project.graphs {
        validate_graph(graph, reference_index.as_ref(), &mut checks);
    }

    StudioValidationReport {
        schema_id: VALIDATION_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        status: if checks
            .iter()
            .any(|check| check.status == StudioValidationStatus::Fail)
        {
            StudioValidationStatus::Fail
        } else {
            StudioValidationStatus::Pass
        },
        checks,
    }
}

pub fn resolve_project(project: &StudioProject) -> StudioResolvedProject {
    StudioResolvedProject {
        schema_id: RESOLVED_PROJECT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        graphs: project.graphs.iter().map(resolve_graph).collect(),
    }
}

pub fn export_plan(project: &StudioProject) -> StudioExportPlan {
    StudioExportPlan {
        schema_id: EXPORT_PLAN_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        bundles: project
            .graphs
            .iter()
            .map(|graph| StudioExportBundle {
                bundle_id: format!("studio.export.{}", graph.graph_id),
                graph_id: graph.graph_id.clone(),
                target_host_profile: graph.target_host_profile.clone(),
                package_ids: graph
                    .nodes
                    .iter()
                    .filter(|node| node.kind == StudioNodeKind::Package)
                    .map(|node| node.reference_id.clone())
                    .collect(),
                module_ids: graph
                    .nodes
                    .iter()
                    .filter(|node| node.kind == StudioNodeKind::Module)
                    .map(|node| node.reference_id.clone())
                    .collect(),
                operator_shell_ids: graph
                    .nodes
                    .iter()
                    .filter(|node| node.kind == StudioNodeKind::OperatorShell)
                    .map(|node| node.reference_id.clone())
                    .collect(),
            })
            .collect(),
    }
}

pub fn view_model(project: &StudioProject, base_dir: Option<&Path>) -> StudioViewModel {
    view_model_for_graph(project, base_dir, None)
}

pub fn view_model_for_graph(
    project: &StudioProject,
    base_dir: Option<&Path>,
    requested_graph_id: Option<&str>,
) -> StudioViewModel {
    view_model_for_graph_and_issue(project, base_dir, requested_graph_id, None)
}

pub fn view_model_for_graph_and_issue(
    project: &StudioProject,
    base_dir: Option<&Path>,
    requested_graph_id: Option<&str>,
    requested_issue_check_id: Option<&str>,
) -> StudioViewModel {
    view_model_for_graph_issue_and_node(
        project,
        base_dir,
        requested_graph_id,
        requested_issue_check_id,
        None,
    )
}

pub fn view_model_for_graph_issue_and_node(
    project: &StudioProject,
    base_dir: Option<&Path>,
    requested_graph_id: Option<&str>,
    requested_issue_check_id: Option<&str>,
    requested_node_id: Option<&str>,
) -> StudioViewModel {
    view_model_for_graph_issue_node_and_edge(
        project,
        base_dir,
        requested_graph_id,
        requested_issue_check_id,
        requested_node_id,
        None,
    )
}

pub fn view_model_for_graph_issue_node_and_edge(
    project: &StudioProject,
    base_dir: Option<&Path>,
    requested_graph_id: Option<&str>,
    requested_issue_check_id: Option<&str>,
    requested_node_id: Option<&str>,
    requested_edge_id: Option<&str>,
) -> StudioViewModel {
    let validation = validate_project_with_base(project, base_dir);
    let validation_pass_count = validation
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Pass)
        .count();
    let validation_fail_count = validation.checks.len() - validation_pass_count;
    let issue_target_index = validation_issue_target_index(&validation);
    let graphs = project
        .graphs
        .iter()
        .map(|graph| graph_view(graph, &issue_target_index))
        .collect::<Vec<_>>();
    let selected_graph_index = selected_graph_index(&graphs, requested_graph_id);
    let selected_graph_id = selected_graph_index
        .and_then(|index| graphs.get(index))
        .map(|graph| graph.graph_id.clone());
    let validation_issues = validation_issue_views(&validation, selected_graph_id.as_deref());
    let issue_selection = focused_issue_selection(&validation_issues, requested_issue_check_id);
    let selected_graph = selected_graph_index.and_then(|index| project.graphs.get(index));
    let selected_graph_view = selected_graph_index.and_then(|index| graphs.get(index));
    let reference_index = reference_index_for_project(project, base_dir);
    let catalog_packages = catalog_package_views(reference_index.as_ref(), selected_graph);
    let host_profiles = host_profile_views(reference_index.as_ref(), selected_graph);
    let node_selection = selected_node_selection(
        selected_graph,
        selected_graph_view,
        reference_index.as_ref(),
        issue_selection.focused_issue.as_ref(),
        requested_node_id,
    );
    let edge_selection = selected_edge_selection(
        selected_graph,
        selected_graph_view,
        issue_selection.focused_issue.as_ref(),
        requested_edge_id,
    );
    let shell_preview =
        shell_preview_for_selected_graph(project, base_dir, selected_graph_id.as_deref());
    let catalog_module_count = catalog_packages
        .iter()
        .map(|package| package.module_count)
        .sum();
    let selection_issue_code = match (requested_graph_id, selected_graph_index) {
        (Some(_), None) => Some("studio.issue.graph_selection_missing".to_string()),
        _ => None,
    };
    StudioViewModel {
        schema_id: VIEW_MODEL_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        display_name: project.display_name.clone(),
        validation_status: validation.status,
        validation_pass_count,
        validation_fail_count,
        validation_issues,
        focused_issue: issue_selection.focused_issue,
        requested_issue_check_id: requested_issue_check_id.map(str::to_string),
        selected_issue_index: issue_selection.selected_issue_index,
        selected_issue_check_id: issue_selection.selected_issue_check_id,
        issue_selection_code: issue_selection.issue_selection_code,
        graph_count: project.graphs.len(),
        requested_graph_id: requested_graph_id.map(str::to_string),
        selected_graph_index,
        selected_graph_id,
        selection_issue_code,
        requested_node_id: requested_node_id.map(str::to_string),
        selected_node_id: node_selection.selected_node_id,
        node_selection_code: node_selection.node_selection_code,
        selected_node: node_selection.selected_node,
        requested_edge_id: requested_edge_id.map(str::to_string),
        selected_edge_id: edge_selection.selected_edge_id,
        edge_selection_code: edge_selection.edge_selection_code,
        selected_edge: edge_selection.selected_edge,
        shell_preview,
        catalog_package_count: catalog_packages.len(),
        catalog_module_count,
        host_profile_count: host_profiles.len(),
        catalog_packages,
        host_profiles,
        graphs,
    }
}

pub fn retarget_graph_host_profile(
    project: &mut StudioProject,
    graph_id: &str,
    host_profile_reference_id: &str,
    base_dir: Option<&Path>,
) -> StudioEditReport {
    let original_revision = project.revision;

    if !is_dotted_id(graph_id) {
        return retarget_host_edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_graph_id".to_string()),
            "Graph id is not a dotted id".to_string(),
            graph_id,
            host_profile_reference_id,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if !is_dotted_id(host_profile_reference_id) {
        return retarget_host_edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_reference_id".to_string()),
            "Host profile reference id is not a dotted id".to_string(),
            graph_id,
            host_profile_reference_id,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    let mut candidate = project.clone();
    let Some(graph) = candidate
        .graphs
        .iter_mut()
        .find(|graph| graph.graph_id == graph_id)
    else {
        return retarget_host_edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditStatus::Rejected,
            Some("studio.issue.graph_missing".to_string()),
            "Graph was not found in the project".to_string(),
            graph_id,
            host_profile_reference_id,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    };

    let mut changed_fields = Vec::new();
    if graph.target_host_profile != host_profile_reference_id {
        graph.target_host_profile = host_profile_reference_id.to_string();
        changed_fields.push(format!("graphs.{graph_id}.target_host_profile"));
    }

    if graph
        .nodes
        .iter()
        .all(|node| node.kind != StudioNodeKind::HostProfile)
    {
        return retarget_host_edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditStatus::Rejected,
            Some("studio.issue.no_host_profile_node".to_string()),
            "Graph does not contain a host_profile node".to_string(),
            graph_id,
            host_profile_reference_id,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if graph.nodes.iter().all(|node| {
        node.kind != StudioNodeKind::HostProfile || node.reference_id != host_profile_reference_id
    }) {
        let host_node = graph
            .nodes
            .iter_mut()
            .find(|node| node.kind == StudioNodeKind::HostProfile)
            .expect("host_profile node presence was checked");
        host_node.reference_id = host_profile_reference_id.to_string();
        changed_fields.push(format!(
            "graphs.{graph_id}.nodes.{}.reference_id",
            host_node.node_id
        ));
    }

    if !changed_fields.is_empty() {
        let Some(next_revision) = candidate.revision.checked_add(1) else {
            return retarget_host_edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditStatus::Rejected,
                Some("studio.issue.revision_overflow".to_string()),
                "Project revision cannot be incremented".to_string(),
                graph_id,
                host_profile_reference_id,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        };
        candidate.revision = next_revision;
    }

    let validation = validate_project_with_base(&candidate, base_dir);
    if validation.status == StudioValidationStatus::Pass {
        *project = candidate;
        let resulting_revision = project.revision;
        let message = if changed_fields.is_empty() {
            "Graph already targets the requested host profile"
        } else {
            "Graph host profile was retargeted"
        };
        return retarget_host_edit_report(
            project,
            original_revision,
            resulting_revision,
            StudioEditStatus::Applied,
            None,
            message.to_string(),
            graph_id,
            host_profile_reference_id,
            changed_fields,
            validation,
        );
    }

    let issue_code = first_failed_issue_code(&validation)
        .unwrap_or_else(|| "studio.issue.edit_rejected".to_string());
    retarget_host_edit_report(
        project,
        original_revision,
        original_revision,
        StudioEditStatus::Rejected,
        Some(issue_code),
        "Edited project candidate failed validation; source project was left unchanged".to_string(),
        graph_id,
        host_profile_reference_id,
        Vec::new(),
        validation,
    )
}

pub fn add_next_catalog_module_to_graph(
    project: &mut StudioProject,
    graph_id: &str,
    base_dir: Option<&Path>,
) -> StudioEditReport {
    add_next_catalog_module_to_graph_with_package(project, graph_id, None, base_dir)
}

pub fn add_next_catalog_module_from_package_to_graph(
    project: &mut StudioProject,
    graph_id: &str,
    package_reference_id: &str,
    base_dir: Option<&Path>,
) -> StudioEditReport {
    add_next_catalog_module_to_graph_with_package(
        project,
        graph_id,
        Some(package_reference_id),
        base_dir,
    )
}

fn add_next_catalog_module_to_graph_with_package(
    project: &mut StudioProject,
    graph_id: &str,
    package_reference_id: Option<&str>,
    base_dir: Option<&Path>,
) -> StudioEditReport {
    let original_revision = project.revision;
    let requested_host_profile = graph_target_host_profile(project, graph_id);
    let requested_reference_id = package_reference_id.unwrap_or(NEXT_PALETTE_MODULE_REQUEST);

    if !is_dotted_id(graph_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::AddModule,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_graph_id".to_string()),
            "Graph id is not a dotted id".to_string(),
            graph_id,
            requested_reference_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if let Some(package_reference_id) = package_reference_id {
        if !is_dotted_id(package_reference_id) {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::AddModule,
                StudioEditStatus::Rejected,
                Some("studio.issue.invalid_reference_id".to_string()),
                "Package reference id is not a dotted id".to_string(),
                graph_id,
                requested_reference_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        }
    }

    let Some(graph) = project
        .graphs
        .iter()
        .find(|graph| graph.graph_id == graph_id)
    else {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::AddModule,
            StudioEditStatus::Rejected,
            Some("studio.issue.graph_missing".to_string()),
            "Graph was not found in the project".to_string(),
            graph_id,
            requested_reference_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    };

    let Some(reference_index) = reference_index_for_project(project, base_dir) else {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::AddModule,
            StudioEditStatus::Rejected,
            Some("studio.issue.reference_index_missing".to_string()),
            "Package catalog references are unavailable for palette selection".to_string(),
            graph_id,
            requested_reference_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    };

    if let Some(package_reference_id) = package_reference_id {
        if !reference_index.package_ids.contains(package_reference_id) {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::AddModule,
                StudioEditStatus::Rejected,
                Some("studio.issue.package_reference_missing".to_string()),
                "Package reference is missing from the package catalog".to_string(),
                graph_id,
                requested_reference_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        }
    }

    let selection = if let Some(package_reference_id) = package_reference_id {
        next_available_catalog_module_for_package(graph, &reference_index, package_reference_id)
    } else {
        next_available_catalog_module(graph, &reference_index)
    };

    let Some(selection) = selection else {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::AddModule,
            StudioEditStatus::Rejected,
            Some("studio.issue.no_available_palette_module".to_string()),
            "No catalog module is available to add to the selected graph".to_string(),
            graph_id,
            requested_reference_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    };

    add_module_to_graph(
        project,
        graph_id,
        &selection.package_id,
        &selection.module_id,
        Some(&selection.label),
        base_dir,
    )
}

pub fn add_module_to_graph(
    project: &mut StudioProject,
    graph_id: &str,
    package_reference_id: &str,
    module_reference_id: &str,
    module_label: Option<&str>,
    base_dir: Option<&Path>,
) -> StudioEditReport {
    let original_revision = project.revision;
    let requested_host_profile = graph_target_host_profile(project, graph_id);

    if !is_dotted_id(graph_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::AddModule,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_graph_id".to_string()),
            "Graph id is not a dotted id".to_string(),
            graph_id,
            module_reference_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if !is_dotted_id(package_reference_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::AddModule,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_reference_id".to_string()),
            "Package reference id is not a dotted id".to_string(),
            graph_id,
            module_reference_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if !is_dotted_id(module_reference_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::AddModule,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_reference_id".to_string()),
            "Module reference id is not a dotted id".to_string(),
            graph_id,
            module_reference_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if let Some(reference_index) = reference_index_for_project(project, base_dir) {
        if !reference_index.package_ids.contains(package_reference_id) {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::AddModule,
                StudioEditStatus::Rejected,
                Some("studio.issue.package_reference_missing".to_string()),
                "Package reference is missing from the package catalog".to_string(),
                graph_id,
                module_reference_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        }
        if !reference_index
            .package_modules
            .get(package_reference_id)
            .is_some_and(|modules| modules.contains(module_reference_id))
        {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::AddModule,
                StudioEditStatus::Rejected,
                Some("studio.issue.package_module_reference_missing".to_string()),
                "Module reference is not exported by the requested package".to_string(),
                graph_id,
                module_reference_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        }
    }

    let mut candidate = project.clone();
    let mut changed_fields = Vec::new();
    {
        let Some(graph) = candidate
            .graphs
            .iter_mut()
            .find(|graph| graph.graph_id == graph_id)
        else {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::AddModule,
                StudioEditStatus::Rejected,
                Some("studio.issue.graph_missing".to_string()),
                "Graph was not found in the project".to_string(),
                graph_id,
                module_reference_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        };

        let generated_package_node_id = generated_node_id(package_reference_id);
        let package_node_id = if let Some(node) = graph.nodes.iter().find(|node| {
            node.kind == StudioNodeKind::Package && node.reference_id == package_reference_id
        }) {
            node.node_id.clone()
        } else {
            if graph
                .nodes
                .iter()
                .any(|node| node.node_id == generated_package_node_id)
            {
                return edit_report(
                    project,
                    original_revision,
                    original_revision,
                    StudioEditOperation::AddModule,
                    StudioEditStatus::Rejected,
                    Some("studio.issue.node_id_conflict".to_string()),
                    "Generated package node id conflicts with an existing node".to_string(),
                    graph_id,
                    module_reference_id,
                    &requested_host_profile,
                    Vec::new(),
                    validate_project_with_base(project, base_dir),
                );
            }
            graph.nodes.push(StudioNode {
                node_id: generated_package_node_id.clone(),
                kind: StudioNodeKind::Package,
                reference_id: package_reference_id.to_string(),
                label: label_for_reference(package_reference_id),
            });
            changed_fields.push(format!(
                "graphs.{graph_id}.nodes.{generated_package_node_id}"
            ));
            generated_package_node_id
        };

        let generated_module_node_id = generated_node_id(module_reference_id);
        let module_node_id = if let Some(node) = graph.nodes.iter().find(|node| {
            node.kind == StudioNodeKind::Module && node.reference_id == module_reference_id
        }) {
            node.node_id.clone()
        } else {
            if graph
                .nodes
                .iter()
                .any(|node| node.node_id == generated_module_node_id)
            {
                return edit_report(
                    project,
                    original_revision,
                    original_revision,
                    StudioEditOperation::AddModule,
                    StudioEditStatus::Rejected,
                    Some("studio.issue.node_id_conflict".to_string()),
                    "Generated module node id conflicts with an existing node".to_string(),
                    graph_id,
                    module_reference_id,
                    &requested_host_profile,
                    Vec::new(),
                    validate_project_with_base(project, base_dir),
                );
            }
            graph.nodes.push(StudioNode {
                node_id: generated_module_node_id.clone(),
                kind: StudioNodeKind::Module,
                reference_id: module_reference_id.to_string(),
                label: module_label
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                    .map(str::to_string)
                    .unwrap_or_else(|| label_for_reference(module_reference_id)),
            });
            changed_fields.push(format!(
                "graphs.{graph_id}.nodes.{generated_module_node_id}"
            ));
            generated_module_node_id
        };

        let package_module_edge_exists = graph.edges.iter().any(|edge| {
            edge.kind == StudioEdgeKind::PackageProvidesModule
                && edge.source_node_id == package_node_id
                && edge.target_node_id == module_node_id
        });
        if !package_module_edge_exists {
            let generated_edge_id =
                generated_package_module_edge_id(package_reference_id, module_reference_id);
            if graph
                .edges
                .iter()
                .any(|edge| edge.edge_id == generated_edge_id)
            {
                return edit_report(
                    project,
                    original_revision,
                    original_revision,
                    StudioEditOperation::AddModule,
                    StudioEditStatus::Rejected,
                    Some("studio.issue.edge_id_conflict".to_string()),
                    "Generated package/module edge id conflicts with an existing edge".to_string(),
                    graph_id,
                    module_reference_id,
                    &requested_host_profile,
                    Vec::new(),
                    validate_project_with_base(project, base_dir),
                );
            }
            graph.edges.push(StudioEdge {
                edge_id: generated_edge_id.clone(),
                kind: StudioEdgeKind::PackageProvidesModule,
                source_node_id: package_node_id,
                target_node_id: module_node_id,
            });
            changed_fields.push(format!("graphs.{graph_id}.edges.{generated_edge_id}"));
        }
    }

    if !changed_fields.is_empty() {
        let Some(next_revision) = candidate.revision.checked_add(1) else {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::AddModule,
                StudioEditStatus::Rejected,
                Some("studio.issue.revision_overflow".to_string()),
                "Project revision cannot be incremented".to_string(),
                graph_id,
                module_reference_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        };
        candidate.revision = next_revision;
    }

    let validation = validate_project_with_base(&candidate, base_dir);
    if validation.status == StudioValidationStatus::Pass {
        *project = candidate;
        let resulting_revision = project.revision;
        let message = if changed_fields.is_empty() {
            "Graph already contains the requested package/module link"
        } else {
            "Graph package/module link was added"
        };
        return edit_report(
            project,
            original_revision,
            resulting_revision,
            StudioEditOperation::AddModule,
            StudioEditStatus::Applied,
            None,
            message.to_string(),
            graph_id,
            module_reference_id,
            &requested_host_profile,
            changed_fields,
            validation,
        );
    }

    let issue_code = first_failed_issue_code(&validation)
        .unwrap_or_else(|| "studio.issue.edit_rejected".to_string());
    edit_report(
        project,
        original_revision,
        original_revision,
        StudioEditOperation::AddModule,
        StudioEditStatus::Rejected,
        Some(issue_code),
        "Edited project candidate failed validation; source project was left unchanged".to_string(),
        graph_id,
        module_reference_id,
        &requested_host_profile,
        Vec::new(),
        validation,
    )
}

pub fn remove_module_from_graph(
    project: &mut StudioProject,
    graph_id: &str,
    module_reference_id: &str,
    base_dir: Option<&Path>,
) -> StudioEditReport {
    let original_revision = project.revision;
    let requested_host_profile = graph_target_host_profile(project, graph_id);

    if !is_dotted_id(graph_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::RemoveModule,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_graph_id".to_string()),
            "Graph id is not a dotted id".to_string(),
            graph_id,
            module_reference_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if !is_dotted_id(module_reference_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::RemoveModule,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_reference_id".to_string()),
            "Module reference id is not a dotted id".to_string(),
            graph_id,
            module_reference_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    let mut candidate = project.clone();
    let mut changed_fields = Vec::new();
    {
        let Some(graph) = candidate
            .graphs
            .iter_mut()
            .find(|graph| graph.graph_id == graph_id)
        else {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::RemoveModule,
                StudioEditStatus::Rejected,
                Some("studio.issue.graph_missing".to_string()),
                "Graph was not found in the project".to_string(),
                graph_id,
                module_reference_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        };

        let module_node_ids = graph
            .nodes
            .iter()
            .filter(|node| {
                node.kind == StudioNodeKind::Module && node.reference_id == module_reference_id
            })
            .map(|node| node.node_id.clone())
            .collect::<BTreeSet<_>>();

        for node_id in &module_node_ids {
            changed_fields.push(format!("graphs.{graph_id}.nodes.{node_id}"));
        }
        graph
            .nodes
            .retain(|node| !module_node_ids.contains(&node.node_id));

        let incident_edge_ids = graph
            .edges
            .iter()
            .filter(|edge| {
                module_node_ids.contains(&edge.source_node_id)
                    || module_node_ids.contains(&edge.target_node_id)
            })
            .map(|edge| edge.edge_id.clone())
            .collect::<Vec<_>>();
        for edge_id in &incident_edge_ids {
            changed_fields.push(format!("graphs.{graph_id}.edges.{edge_id}"));
        }
        graph
            .edges
            .retain(|edge| !incident_edge_ids.contains(&edge.edge_id));

        if let Some(layout) = graph.layout.as_mut() {
            let removed_layout_node_ids = layout
                .nodes
                .iter()
                .filter(|node| module_node_ids.contains(&node.node_id))
                .map(|node| node.node_id.clone())
                .collect::<Vec<_>>();
            for node_id in &removed_layout_node_ids {
                changed_fields.push(format!("graphs.{graph_id}.layout.nodes.{node_id}"));
            }
            layout
                .nodes
                .retain(|node| !module_node_ids.contains(&node.node_id));

            let removed_layout_edge_ids = layout
                .edges
                .iter()
                .filter(|edge| incident_edge_ids.contains(&edge.edge_id))
                .map(|edge| edge.edge_id.clone())
                .collect::<Vec<_>>();
            for edge_id in &removed_layout_edge_ids {
                changed_fields.push(format!("graphs.{graph_id}.layout.edges.{edge_id}"));
            }
            layout
                .edges
                .retain(|edge| !incident_edge_ids.contains(&edge.edge_id));
        }
    }

    if !changed_fields.is_empty() {
        let Some(next_revision) = candidate.revision.checked_add(1) else {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::RemoveModule,
                StudioEditStatus::Rejected,
                Some("studio.issue.revision_overflow".to_string()),
                "Project revision cannot be incremented".to_string(),
                graph_id,
                module_reference_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        };
        candidate.revision = next_revision;
    }

    let validation = validate_project_with_base(&candidate, base_dir);
    if validation.status == StudioValidationStatus::Pass {
        *project = candidate;
        let resulting_revision = project.revision;
        let message = if changed_fields.is_empty() {
            "Graph already omits the requested module"
        } else {
            "Graph module and incident edges were removed"
        };
        return edit_report(
            project,
            original_revision,
            resulting_revision,
            StudioEditOperation::RemoveModule,
            StudioEditStatus::Applied,
            None,
            message.to_string(),
            graph_id,
            module_reference_id,
            &requested_host_profile,
            changed_fields,
            validation,
        );
    }

    let issue_code = first_failed_issue_code(&validation)
        .unwrap_or_else(|| "studio.issue.edit_rejected".to_string());
    edit_report(
        project,
        original_revision,
        original_revision,
        StudioEditOperation::RemoveModule,
        StudioEditStatus::Rejected,
        Some(issue_code),
        "Edited project candidate failed validation; source project was left unchanged".to_string(),
        graph_id,
        module_reference_id,
        &requested_host_profile,
        Vec::new(),
        validation,
    )
}

pub fn add_binding_to_graph(
    project: &mut StudioProject,
    graph_id: &str,
    binding_kind: StudioBindingKind,
    source_node_id: &str,
    target_node_id: &str,
    base_dir: Option<&Path>,
) -> StudioEditReport {
    let original_revision = project.revision;
    let requested_host_profile = graph_target_host_profile(project, graph_id);
    let requested_binding_id =
        generated_binding_edge_id(binding_kind, source_node_id, target_node_id);

    if !is_dotted_id(graph_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::AddBinding,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_graph_id".to_string()),
            "Graph id is not a dotted id".to_string(),
            graph_id,
            &requested_binding_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if !is_dotted_id(source_node_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::AddBinding,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_node_id".to_string()),
            "Binding source node id is not a dotted id".to_string(),
            graph_id,
            &requested_binding_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if !is_dotted_id(target_node_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::AddBinding,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_node_id".to_string()),
            "Binding target node id is not a dotted id".to_string(),
            graph_id,
            &requested_binding_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if source_node_id == target_node_id {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::AddBinding,
            StudioEditStatus::Rejected,
            Some("studio.issue.self_binding".to_string()),
            "Binding source and target must be different nodes".to_string(),
            graph_id,
            &requested_binding_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    let mut candidate = project.clone();
    let mut changed_fields = Vec::new();
    {
        let Some(graph) = candidate
            .graphs
            .iter_mut()
            .find(|graph| graph.graph_id == graph_id)
        else {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::AddBinding,
                StudioEditStatus::Rejected,
                Some("studio.issue.graph_missing".to_string()),
                "Graph was not found in the project".to_string(),
                graph_id,
                &requested_binding_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        };

        let Some(source_node) = graph
            .nodes
            .iter()
            .find(|node| node.node_id == source_node_id)
        else {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::AddBinding,
                StudioEditStatus::Rejected,
                Some("studio.issue.binding_source_missing".to_string()),
                "Binding source node was not found in the graph".to_string(),
                graph_id,
                &requested_binding_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        };
        let source_kind = source_node.kind;

        let Some(target_node) = graph
            .nodes
            .iter()
            .find(|node| node.node_id == target_node_id)
        else {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::AddBinding,
                StudioEditStatus::Rejected,
                Some("studio.issue.binding_target_missing".to_string()),
                "Binding target node was not found in the graph".to_string(),
                graph_id,
                &requested_binding_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        };
        let target_kind = target_node.kind;

        if !binding_endpoint_kinds_are_valid(binding_kind, source_kind, target_kind) {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::AddBinding,
                StudioEditStatus::Rejected,
                Some("studio.issue.binding_endpoint_kind_mismatch".to_string()),
                binding_endpoint_kind_message(binding_kind).to_string(),
                graph_id,
                &requested_binding_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        }

        let edge_kind = edge_kind_for_binding(binding_kind);
        let binding_exists = graph.edges.iter().any(|edge| {
            edge.kind == edge_kind
                && edge.source_node_id == source_node_id
                && edge.target_node_id == target_node_id
        });
        if !binding_exists {
            if graph
                .edges
                .iter()
                .any(|edge| edge.edge_id == requested_binding_id)
            {
                return edit_report(
                    project,
                    original_revision,
                    original_revision,
                    StudioEditOperation::AddBinding,
                    StudioEditStatus::Rejected,
                    Some("studio.issue.edge_id_conflict".to_string()),
                    "Generated binding edge id conflicts with an existing edge".to_string(),
                    graph_id,
                    &requested_binding_id,
                    &requested_host_profile,
                    Vec::new(),
                    validate_project_with_base(project, base_dir),
                );
            }
            graph.edges.push(StudioEdge {
                edge_id: requested_binding_id.clone(),
                kind: edge_kind,
                source_node_id: source_node_id.to_string(),
                target_node_id: target_node_id.to_string(),
            });
            changed_fields.push(format!("graphs.{graph_id}.edges.{requested_binding_id}"));
        }
    }

    if !changed_fields.is_empty() {
        let Some(next_revision) = candidate.revision.checked_add(1) else {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::AddBinding,
                StudioEditStatus::Rejected,
                Some("studio.issue.revision_overflow".to_string()),
                "Project revision cannot be incremented".to_string(),
                graph_id,
                &requested_binding_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        };
        candidate.revision = next_revision;
    }

    let validation = validate_project_with_base(&candidate, base_dir);
    if validation.status == StudioValidationStatus::Pass {
        *project = candidate;
        let resulting_revision = project.revision;
        let message = if changed_fields.is_empty() {
            "Graph already contains the requested binding"
        } else {
            "Graph binding was added"
        };
        return edit_report(
            project,
            original_revision,
            resulting_revision,
            StudioEditOperation::AddBinding,
            StudioEditStatus::Applied,
            None,
            message.to_string(),
            graph_id,
            &requested_binding_id,
            &requested_host_profile,
            changed_fields,
            validation,
        );
    }

    let issue_code = first_failed_issue_code(&validation)
        .unwrap_or_else(|| "studio.issue.edit_rejected".to_string());
    edit_report(
        project,
        original_revision,
        original_revision,
        StudioEditOperation::AddBinding,
        StudioEditStatus::Rejected,
        Some(issue_code),
        "Edited project candidate failed validation; source project was left unchanged".to_string(),
        graph_id,
        &requested_binding_id,
        &requested_host_profile,
        Vec::new(),
        validation,
    )
}

pub fn remove_binding_from_graph(
    project: &mut StudioProject,
    graph_id: &str,
    binding_kind: StudioBindingKind,
    source_node_id: &str,
    target_node_id: &str,
    base_dir: Option<&Path>,
) -> StudioEditReport {
    let original_revision = project.revision;
    let requested_host_profile = graph_target_host_profile(project, graph_id);
    let requested_binding_id =
        generated_binding_edge_id(binding_kind, source_node_id, target_node_id);

    if !is_dotted_id(graph_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::RemoveBinding,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_graph_id".to_string()),
            "Graph id is not a dotted id".to_string(),
            graph_id,
            &requested_binding_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if !is_dotted_id(source_node_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::RemoveBinding,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_node_id".to_string()),
            "Binding source node id is not a dotted id".to_string(),
            graph_id,
            &requested_binding_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    if !is_dotted_id(target_node_id) {
        return edit_report(
            project,
            original_revision,
            original_revision,
            StudioEditOperation::RemoveBinding,
            StudioEditStatus::Rejected,
            Some("studio.issue.invalid_node_id".to_string()),
            "Binding target node id is not a dotted id".to_string(),
            graph_id,
            &requested_binding_id,
            &requested_host_profile,
            Vec::new(),
            validate_project_with_base(project, base_dir),
        );
    }

    let mut candidate = project.clone();
    let mut changed_fields = Vec::new();
    {
        let Some(graph) = candidate
            .graphs
            .iter_mut()
            .find(|graph| graph.graph_id == graph_id)
        else {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::RemoveBinding,
                StudioEditStatus::Rejected,
                Some("studio.issue.graph_missing".to_string()),
                "Graph was not found in the project".to_string(),
                graph_id,
                &requested_binding_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        };

        let edge_kind = edge_kind_for_binding(binding_kind);
        let removed_edge_ids = graph
            .edges
            .iter()
            .filter(|edge| {
                edge.kind == edge_kind
                    && edge.source_node_id == source_node_id
                    && edge.target_node_id == target_node_id
            })
            .map(|edge| edge.edge_id.clone())
            .collect::<BTreeSet<_>>();
        for edge_id in &removed_edge_ids {
            changed_fields.push(format!("graphs.{graph_id}.edges.{edge_id}"));
        }
        graph
            .edges
            .retain(|edge| !removed_edge_ids.contains(&edge.edge_id));
        if let Some(layout) = graph.layout.as_mut() {
            let removed_layout_edge_ids = layout
                .edges
                .iter()
                .filter(|edge| removed_edge_ids.contains(&edge.edge_id))
                .map(|edge| edge.edge_id.clone())
                .collect::<Vec<_>>();
            for edge_id in &removed_layout_edge_ids {
                changed_fields.push(format!("graphs.{graph_id}.layout.edges.{edge_id}"));
            }
            layout
                .edges
                .retain(|edge| !removed_edge_ids.contains(&edge.edge_id));
        }
    }

    if !changed_fields.is_empty() {
        let Some(next_revision) = candidate.revision.checked_add(1) else {
            return edit_report(
                project,
                original_revision,
                original_revision,
                StudioEditOperation::RemoveBinding,
                StudioEditStatus::Rejected,
                Some("studio.issue.revision_overflow".to_string()),
                "Project revision cannot be incremented".to_string(),
                graph_id,
                &requested_binding_id,
                &requested_host_profile,
                Vec::new(),
                validate_project_with_base(project, base_dir),
            );
        };
        candidate.revision = next_revision;
    }

    let validation = validate_project_with_base(&candidate, base_dir);
    if validation.status == StudioValidationStatus::Pass {
        *project = candidate;
        let resulting_revision = project.revision;
        let message = if changed_fields.is_empty() {
            "Graph already omits the requested binding"
        } else {
            "Graph binding was removed"
        };
        return edit_report(
            project,
            original_revision,
            resulting_revision,
            StudioEditOperation::RemoveBinding,
            StudioEditStatus::Applied,
            None,
            message.to_string(),
            graph_id,
            &requested_binding_id,
            &requested_host_profile,
            changed_fields,
            validation,
        );
    }

    let issue_code = first_failed_issue_code(&validation)
        .unwrap_or_else(|| "studio.issue.edit_rejected".to_string());
    edit_report(
        project,
        original_revision,
        original_revision,
        StudioEditOperation::RemoveBinding,
        StudioEditStatus::Rejected,
        Some(issue_code),
        "Edited project candidate failed validation; source project was left unchanged".to_string(),
        graph_id,
        &requested_binding_id,
        &requested_host_profile,
        Vec::new(),
        validation,
    )
}

pub fn shell_descriptor_for_graph(
    project: &StudioProject,
    base_dir: Option<&Path>,
    graph_id: &str,
) -> StudioShellDescriptorReport {
    let validation = validate_project_with_base(project, base_dir);

    if !is_dotted_id(graph_id) {
        return shell_descriptor_report(
            project,
            graph_id,
            StudioShellDescriptorStatus::Rejected,
            Some("studio.issue.invalid_graph_id".to_string()),
            "Graph id is not a dotted id".to_string(),
            validation,
            None,
        );
    }

    if validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_issue_code(&validation)
            .unwrap_or_else(|| "studio.issue.validation_failed".to_string());
        return shell_descriptor_report(
            project,
            graph_id,
            StudioShellDescriptorStatus::Rejected,
            Some(issue_code),
            "Project validation failed; shell descriptor was not exported".to_string(),
            validation,
            None,
        );
    }

    let Some(graph) = project
        .graphs
        .iter()
        .find(|graph| graph.graph_id == graph_id)
    else {
        return shell_descriptor_report(
            project,
            graph_id,
            StudioShellDescriptorStatus::Rejected,
            Some("studio.issue.graph_missing".to_string()),
            "Graph was not found in the project".to_string(),
            validation,
            None,
        );
    };

    let operator_shell_nodes = graph
        .nodes
        .iter()
        .filter(|node| node.kind == StudioNodeKind::OperatorShell)
        .collect::<Vec<_>>();
    let Some(shell_node) = operator_shell_nodes.first().copied() else {
        return shell_descriptor_report(
            project,
            graph_id,
            StudioShellDescriptorStatus::Rejected,
            Some("studio.issue.no_operator_shell".to_string()),
            "Graph does not contain an operator_shell node".to_string(),
            validation,
            None,
        );
    };
    if operator_shell_nodes.len() > 1 {
        return shell_descriptor_report(
            project,
            graph_id,
            StudioShellDescriptorStatus::Rejected,
            Some("studio.issue.multiple_operator_shells".to_string()),
            "Graph contains multiple operator_shell nodes; export one shell at a time".to_string(),
            validation,
            None,
        );
    }

    let reference_index = reference_index_for_project(project, base_dir);
    let descriptor = StudioShellDescriptor {
        schema_id: SHELL_DESCRIPTOR_SCHEMA.to_string(),
        descriptor_id: format!("studio.shell_descriptor.{}", graph.graph_id),
        project_id: project.project_id.clone(),
        project_revision: project.revision,
        graph_id: graph.graph_id.clone(),
        display_name: graph.display_name.clone(),
        shell_id: shell_node.reference_id.clone(),
        shell_label: shell_node.label.clone(),
        target_host_profile: graph.target_host_profile.clone(),
        host_profile: shell_host_profile(&graph.target_host_profile, reference_index.as_ref()),
        package_ids: graph
            .nodes
            .iter()
            .filter(|node| node.kind == StudioNodeKind::Package)
            .map(|node| node.reference_id.clone())
            .collect(),
        module_ids: graph
            .nodes
            .iter()
            .filter(|node| node.kind == StudioNodeKind::Module)
            .map(|node| node.reference_id.clone())
            .collect(),
        validation_slot_ids: graph
            .nodes
            .iter()
            .filter(|node| node.kind == StudioNodeKind::ValidationSlot)
            .map(|node| node.reference_id.clone())
            .collect(),
        stream_bindings: graph
            .edges
            .iter()
            .filter(|edge| edge.kind == rusty_studio_model::StudioEdgeKind::StreamBinding)
            .map(shell_binding)
            .collect(),
        command_bindings: graph
            .edges
            .iter()
            .filter(|edge| edge.kind == rusty_studio_model::StudioEdgeKind::CommandBinding)
            .map(shell_binding)
            .collect(),
    };

    shell_descriptor_report(
        project,
        graph_id,
        StudioShellDescriptorStatus::Exported,
        None,
        "Shell descriptor exported".to_string(),
        validation,
        Some(descriptor),
    )
}

pub fn validate_shell_descriptor(
    descriptor: &StudioShellDescriptor,
) -> StudioShellDescriptorValidationReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.schema",
        descriptor.schema_id == SHELL_DESCRIPTOR_SCHEMA,
        "shell descriptor schema id is supported",
        "shell descriptor schema id is unsupported",
        "studio.issue.shell_descriptor_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.descriptor_id",
        is_dotted_id(&descriptor.descriptor_id),
        "descriptor id uses dotted-id grammar",
        "descriptor id is not a dotted id",
        "studio.issue.invalid_descriptor_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.project_id",
        is_dotted_id(&descriptor.project_id),
        "project id uses dotted-id grammar",
        "project id is not a dotted id",
        "studio.issue.invalid_project_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.project_revision",
        descriptor.project_revision > 0,
        "project revision is positive",
        "project revision must be positive",
        "studio.issue.invalid_revision",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.graph_id",
        is_dotted_id(&descriptor.graph_id),
        "graph id uses dotted-id grammar",
        "graph id is not a dotted id",
        "studio.issue.invalid_graph_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.shell_id",
        is_dotted_id(&descriptor.shell_id),
        "shell id uses dotted-id grammar",
        "shell id is not a dotted id",
        "studio.issue.invalid_shell_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.target_host_profile",
        is_dotted_id(&descriptor.target_host_profile),
        "target host profile uses dotted-id grammar",
        "target host profile is not a dotted id",
        "studio.issue.invalid_target_host_profile",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.host_profile_id",
        is_dotted_id(&descriptor.host_profile.profile_id),
        "host profile id uses dotted-id grammar",
        "host profile id is not a dotted id",
        "studio.issue.invalid_host_profile_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.host_profile_matches_target",
        descriptor.host_profile.profile_id == descriptor.target_host_profile,
        "host profile id matches target host profile",
        "host profile id does not match target host profile",
        "studio.issue.host_profile_target_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.host_app_id",
        optional_dotted_id(descriptor.host_profile.app_id.as_deref()),
        "host app id is absent or uses dotted-id grammar",
        "host app id is not a dotted id",
        "studio.issue.invalid_host_app_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.install_route",
        optional_dotted_id(descriptor.host_profile.install_route.as_deref()),
        "install route is absent or uses dotted-id grammar",
        "install route is not a dotted id",
        "studio.issue.invalid_install_route",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.launch_route",
        optional_dotted_id(descriptor.host_profile.launch_route.as_deref()),
        "launch route is absent or uses dotted-id grammar",
        "launch route is not a dotted id",
        "studio.issue.invalid_launch_route",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.command_bridge",
        optional_dotted_id(descriptor.host_profile.command_bridge.as_deref()),
        "command bridge is absent or uses dotted-id grammar",
        "command bridge is not a dotted id",
        "studio.issue.invalid_command_bridge",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.evidence_pull_route",
        optional_dotted_id(descriptor.host_profile.evidence_pull_route.as_deref()),
        "evidence pull route is absent or uses dotted-id grammar",
        "evidence pull route is not a dotted id",
        "studio.issue.invalid_evidence_pull_route",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.required_permissions",
        all_dotted_ids(&descriptor.host_profile.required_permissions),
        "required permissions use dotted-id grammar",
        "one or more required permissions are not dotted ids",
        "studio.issue.invalid_required_permission",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.packages_present",
        !descriptor.package_ids.is_empty(),
        "descriptor declares package ids",
        "descriptor must declare at least one package id",
        "studio.issue.no_descriptor_packages",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.package_ids",
        all_dotted_ids(&descriptor.package_ids),
        "package ids use dotted-id grammar",
        "one or more package ids are not dotted ids",
        "studio.issue.invalid_package_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.module_ids",
        all_dotted_ids(&descriptor.module_ids),
        "module ids use dotted-id grammar",
        "one or more module ids are not dotted ids",
        "studio.issue.invalid_module_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_descriptor.validation_slot_ids",
        all_dotted_ids(&descriptor.validation_slot_ids),
        "validation slot ids use dotted-id grammar",
        "one or more validation slot ids are not dotted ids",
        "studio.issue.invalid_validation_slot_id",
    );
    validate_shell_bindings("stream_bindings", &descriptor.stream_bindings, &mut checks);
    validate_shell_bindings(
        "command_bindings",
        &descriptor.command_bindings,
        &mut checks,
    );

    StudioShellDescriptorValidationReport {
        schema_id: SHELL_DESCRIPTOR_VALIDATION_REPORT_SCHEMA,
        descriptor_id: descriptor.descriptor_id.clone(),
        status: if checks
            .iter()
            .any(|check| check.status == StudioValidationStatus::Fail)
        {
            StudioValidationStatus::Fail
        } else {
            StudioValidationStatus::Pass
        },
        checks,
    }
}

pub fn shell_artifacts_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
) -> StudioShellArtifactReport {
    let validation = validate_project_with_base(project, base_dir);
    if validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_issue_code(&validation)
            .unwrap_or_else(|| "studio.issue.validation_failed".to_string());
        return shell_artifact_report(
            project,
            StudioShellArtifactStatus::Rejected,
            Some(issue_code),
            "Project validation failed; shell artifacts were not exported".to_string(),
            validation,
            None,
            Vec::new(),
            Vec::new(),
        );
    }

    let mut artifacts = Vec::new();
    let mut descriptors = Vec::new();
    let mut rejections = Vec::new();
    for graph in &project.graphs {
        let descriptor_report = shell_descriptor_for_graph(project, base_dir, &graph.graph_id);
        match (descriptor_report.status, descriptor_report.descriptor) {
            (StudioShellDescriptorStatus::Exported, Some(descriptor)) => {
                let descriptor_validation = validate_shell_descriptor(&descriptor);
                if descriptor_validation.status == StudioValidationStatus::Pass {
                    artifacts.push(shell_artifact_for_descriptor(&descriptor));
                    descriptors.push(descriptor);
                } else {
                    let issue_code = first_failed_check_issue_code(&descriptor_validation)
                        .unwrap_or_else(|| "studio.issue.shell_descriptor_invalid".to_string());
                    rejections.push(StudioShellArtifactRejection {
                        graph_id: graph.graph_id.clone(),
                        issue_code: Some(issue_code),
                        message: "Generated shell descriptor failed validation".to_string(),
                    });
                }
            }
            (_, _) => {
                rejections.push(StudioShellArtifactRejection {
                    graph_id: graph.graph_id.clone(),
                    issue_code: descriptor_report.issue_code,
                    message: descriptor_report.message,
                });
            }
        }
    }

    if !rejections.is_empty() {
        return shell_artifact_report(
            project,
            StudioShellArtifactStatus::Rejected,
            rejections
                .first()
                .and_then(|rejection| rejection.issue_code.clone()),
            "One or more graph shell descriptors could not be exported".to_string(),
            validation,
            None,
            Vec::new(),
            rejections,
        );
    }

    let manifest = StudioShellArtifactManifest {
        schema_id: SHELL_ARTIFACT_MANIFEST_SCHEMA.to_string(),
        manifest_id: format!("studio.shell_artifacts.{}", project.project_id),
        project_id: project.project_id.clone(),
        project_revision: project.revision,
        artifacts,
    };
    shell_artifact_report(
        project,
        StudioShellArtifactStatus::Exported,
        None,
        "Shell artifacts exported".to_string(),
        validation,
        Some(manifest),
        descriptors,
        Vec::new(),
    )
}

pub fn validate_shell_artifact_manifest(
    manifest: &StudioShellArtifactManifest,
    base_dir: Option<&Path>,
) -> StudioShellArtifactManifestValidationReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.schema",
        manifest.schema_id == SHELL_ARTIFACT_MANIFEST_SCHEMA,
        "shell artifact manifest schema id is supported",
        "shell artifact manifest schema id is unsupported",
        "studio.issue.shell_artifact_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.manifest_id",
        is_dotted_id(&manifest.manifest_id),
        "manifest id uses dotted-id grammar",
        "manifest id is not a dotted id",
        "studio.issue.invalid_manifest_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.project_id",
        is_dotted_id(&manifest.project_id),
        "project id uses dotted-id grammar",
        "project id is not a dotted id",
        "studio.issue.invalid_project_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.project_revision",
        manifest.project_revision > 0,
        "project revision is positive",
        "project revision must be positive",
        "studio.issue.invalid_revision",
    );
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.artifacts_present",
        !manifest.artifacts.is_empty(),
        "manifest declares shell artifacts",
        "manifest must declare at least one shell artifact",
        "studio.issue.no_shell_artifacts",
    );

    let duplicate_artifact_ids = duplicate_artifact_field(&manifest.artifacts, |artifact| {
        artifact.artifact_id.as_str()
    });
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.unique_artifact_ids",
        duplicate_artifact_ids.is_empty(),
        "artifact ids are unique",
        &format!(
            "duplicate artifact ids: {}",
            duplicate_artifact_ids.join(", ")
        ),
        "studio.issue.duplicate_artifact_id",
    );
    let duplicate_graph_ids =
        duplicate_artifact_field(&manifest.artifacts, |artifact| artifact.graph_id.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.unique_graph_ids",
        duplicate_graph_ids.is_empty(),
        "artifact graph ids are unique",
        &format!(
            "duplicate artifact graph ids: {}",
            duplicate_graph_ids.join(", ")
        ),
        "studio.issue.duplicate_artifact_graph_id",
    );
    let duplicate_descriptor_paths = duplicate_artifact_field(&manifest.artifacts, |artifact| {
        artifact.descriptor_path.as_str()
    });
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.unique_descriptor_paths",
        duplicate_descriptor_paths.is_empty(),
        "descriptor paths are unique",
        &format!(
            "duplicate descriptor paths: {}",
            duplicate_descriptor_paths.join(", ")
        ),
        "studio.issue.duplicate_descriptor_path",
    );

    for artifact in &manifest.artifacts {
        validate_shell_artifact_manifest_entry(artifact, base_dir, &mut checks);
    }

    StudioShellArtifactManifestValidationReport {
        schema_id: SHELL_ARTIFACT_MANIFEST_VALIDATION_REPORT_SCHEMA,
        manifest_id: manifest.manifest_id.clone(),
        status: if checks
            .iter()
            .any(|check| check.status == StudioValidationStatus::Fail)
        {
            StudioValidationStatus::Fail
        } else {
            StudioValidationStatus::Pass
        },
        checks,
    }
}

pub fn shell_templates_for_artifact_manifest(
    manifest: &StudioShellArtifactManifest,
    base_dir: Option<&Path>,
) -> StudioShellTemplateReport {
    let validation = validate_shell_artifact_manifest(manifest, base_dir);
    if validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_shell_artifact_manifest_issue_code(&validation)
            .unwrap_or_else(|| "studio.issue.shell_artifact_manifest_invalid".to_string());
        return shell_template_report(
            manifest,
            StudioShellTemplateStatus::Rejected,
            Some(issue_code),
            "Shell artifact manifest validation failed; shell templates were not exported"
                .to_string(),
            validation,
            None,
            Vec::new(),
        );
    }

    let templates: Vec<_> = manifest
        .artifacts
        .iter()
        .map(shell_template_for_artifact)
        .collect();
    let index = StudioShellTemplateIndex {
        schema_id: SHELL_TEMPLATE_INDEX_SCHEMA.to_string(),
        index_id: format!("studio.shell_templates.{}", manifest.project_id),
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        templates: manifest
            .artifacts
            .iter()
            .map(shell_template_index_entry)
            .collect(),
    };

    shell_template_report(
        manifest,
        StudioShellTemplateStatus::Exported,
        None,
        "Shell templates exported".to_string(),
        validation,
        Some(index),
        templates,
    )
}

pub fn selected_shell_bundle_for_graph(
    project: &StudioProject,
    base_dir: Option<&Path>,
    graph_id: &str,
) -> StudioShellBundleReport {
    let descriptor_report = shell_descriptor_for_graph(project, base_dir, graph_id);
    let (StudioShellDescriptorStatus::Exported, Some(descriptor)) =
        (descriptor_report.status, descriptor_report.descriptor)
    else {
        return shell_bundle_report(
            project,
            graph_id,
            StudioShellBundleStatus::Rejected,
            descriptor_report.issue_code,
            descriptor_report.message,
            Vec::new(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
    };

    let descriptor_validation = validate_shell_descriptor(&descriptor);
    if descriptor_validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_check_issue_code(&descriptor_validation)
            .unwrap_or_else(|| "studio.issue.shell_descriptor_invalid".to_string());
        return shell_bundle_report(
            project,
            graph_id,
            StudioShellBundleStatus::Rejected,
            Some(issue_code),
            "Generated shell descriptor failed validation".to_string(),
            Vec::new(),
            Some(descriptor_validation),
            None,
            None,
            Some(descriptor),
            None,
            None,
            None,
        );
    }

    let artifact = shell_artifact_for_descriptor(&descriptor);
    let artifact_manifest = StudioShellArtifactManifest {
        schema_id: SHELL_ARTIFACT_MANIFEST_SCHEMA.to_string(),
        manifest_id: selected_shell_bundle_manifest_id(&project.project_id, &descriptor.graph_id),
        project_id: project.project_id.clone(),
        project_revision: project.revision,
        artifacts: vec![artifact.clone()],
    };
    let artifact_validation = validate_shell_artifact_manifest(&artifact_manifest, None);
    if artifact_validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_shell_artifact_manifest_issue_code(&artifact_validation)
            .unwrap_or_else(|| "studio.issue.shell_artifact_manifest_invalid".to_string());
        return shell_bundle_report(
            project,
            graph_id,
            StudioShellBundleStatus::Rejected,
            Some(issue_code),
            "Generated selected shell artifact manifest failed validation".to_string(),
            Vec::new(),
            Some(descriptor_validation),
            Some(artifact_validation),
            None,
            Some(descriptor),
            Some(artifact_manifest),
            None,
            None,
        );
    }

    let template_manifest = shell_template_for_artifact(&artifact);
    let template_entry = shell_template_index_entry(&artifact);
    let template_index = StudioShellTemplateIndex {
        schema_id: SHELL_TEMPLATE_INDEX_SCHEMA.to_string(),
        index_id: selected_shell_bundle_template_index_id(
            &project.project_id,
            &descriptor.graph_id,
        ),
        manifest_id: artifact_manifest.manifest_id.clone(),
        project_id: project.project_id.clone(),
        project_revision: project.revision,
        templates: vec![template_entry.clone()],
    };
    let template_validation = validate_shell_template_index(&template_index, None);
    if template_validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_shell_template_index_issue_code(&template_validation)
            .unwrap_or_else(|| "studio.issue.shell_template_index_invalid".to_string());
        return shell_bundle_report(
            project,
            graph_id,
            StudioShellBundleStatus::Rejected,
            Some(issue_code),
            "Generated selected shell template index failed validation".to_string(),
            Vec::new(),
            Some(descriptor_validation),
            Some(artifact_validation),
            Some(template_validation),
            Some(descriptor),
            Some(artifact_manifest),
            Some(template_index),
            Some(template_manifest),
        );
    }

    let bundle_files = selected_shell_bundle_files(&artifact, &template_entry);
    shell_bundle_report(
        project,
        graph_id,
        StudioShellBundleStatus::Exported,
        None,
        "Selected shell bundle exported".to_string(),
        bundle_files,
        Some(descriptor_validation),
        Some(artifact_validation),
        Some(template_validation),
        Some(descriptor),
        Some(artifact_manifest),
        Some(template_index),
        Some(template_manifest),
    )
}

pub fn save_shell_bundle(
    output_dir: &Path,
    report: &StudioShellBundleReport,
) -> Result<Vec<String>, StudioCoreError> {
    if report.status != StudioShellBundleStatus::Exported {
        return Ok(Vec::new());
    }
    let Some(descriptor) = report.descriptor.as_ref() else {
        return Ok(Vec::new());
    };
    let Some(artifact_manifest) = report.artifact_manifest.as_ref() else {
        return Ok(Vec::new());
    };
    let Some(template_index) = report.template_index.as_ref() else {
        return Ok(Vec::new());
    };
    let Some(template_manifest) = report.template_manifest.as_ref() else {
        return Ok(Vec::new());
    };

    let mut written_files = BTreeSet::new();
    for relative_path in descriptor_bundle_paths(report) {
        save_json(
            &relative_output_path(output_dir, &relative_path),
            descriptor,
        )?;
        written_files.insert(relative_path);
    }
    let manifold_handoff_path = shell_manifold_handoff_artifact_path(&descriptor.graph_id);
    save_json(
        &relative_output_path(output_dir, &manifold_handoff_path),
        &manifold_shell_handoff_for_descriptor(descriptor),
    )?;
    written_files.insert(manifold_handoff_path);
    save_json(
        &relative_output_path(output_dir, "shell-artifacts.json"),
        artifact_manifest,
    )?;
    written_files.insert("shell-artifacts.json".to_string());
    save_json(
        &relative_output_path(output_dir, "shell-templates.json"),
        template_index,
    )?;
    written_files.insert("shell-templates.json".to_string());
    for entry in &template_index.templates {
        if entry.template_id == template_manifest.template_id {
            save_json(
                &relative_output_path(output_dir, &entry.template_path),
                template_manifest,
            )?;
            written_files.insert(entry.template_path.clone());
        }
    }
    Ok(written_files.into_iter().collect())
}

pub fn validate_selected_shell_bundle(
    project: &StudioProject,
    base_dir: Option<&Path>,
    graph_id: &str,
    bundle_dir: &Path,
) -> StudioShellBundleValidationReport {
    let expected = selected_shell_bundle_for_graph(project, base_dir, graph_id);
    let expected_bundle_files = expected.bundle_files.clone();
    let mut checks = Vec::new();
    let preview_issue = expected
        .issue_code
        .as_deref()
        .unwrap_or("studio.issue.shell_bundle_preview_rejected");
    push_bundle_check(
        &mut checks,
        graph_id,
        "studio.check.shell_bundle.current_preview",
        expected.status == StudioShellBundleStatus::Exported,
        "current selected graph exports a shell bundle",
        &expected.message,
        preview_issue,
    );

    if expected.status != StudioShellBundleStatus::Exported {
        return shell_bundle_validation_report(project, graph_id, expected_bundle_files, checks);
    }

    for (index, relative_path) in expected.bundle_files.iter().enumerate() {
        let file_path = relative_output_path(bundle_dir, relative_path);
        push_bundle_check(
            &mut checks,
            graph_id,
            &format!("studio.check.shell_bundle.file.{index}.exists"),
            file_path.is_file(),
            "expected bundle file exists",
            &format!("expected bundle file is missing: {relative_path}"),
            "studio.issue.shell_bundle_file_missing",
        );
    }

    let expected_descriptor = expected.descriptor.as_ref();
    if let Some(descriptor_relative_path) = descriptor_bundle_paths(&expected).first().cloned() {
        let descriptor_path = relative_output_path(bundle_dir, &descriptor_relative_path);
        match load_shell_descriptor(&descriptor_path) {
            Ok(descriptor) => {
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.descriptor.parse",
                    true,
                    "descriptor JSON parsed",
                    "descriptor JSON did not parse",
                    "studio.issue.descriptor_parse_failed",
                );
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.descriptor.current_match",
                    expected_descriptor == Some(&descriptor),
                    "descriptor matches the current selected graph preview",
                    "descriptor differs from the current selected graph preview",
                    "studio.issue.shell_bundle_descriptor_mismatch",
                );
            }
            Err(error) => {
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.descriptor.parse",
                    false,
                    "descriptor JSON parsed",
                    &error.to_string(),
                    "studio.issue.descriptor_parse_failed",
                );
            }
        }
    } else {
        push_bundle_check(
            &mut checks,
            graph_id,
            "studio.check.shell_bundle.descriptor.path",
            false,
            "current preview has a descriptor path",
            "current preview has no descriptor path",
            "studio.issue.descriptor_missing",
        );
    }

    let expected_artifact_manifest = expected.artifact_manifest.as_ref();
    let artifact_manifest_path = relative_output_path(bundle_dir, "shell-artifacts.json");
    match load_shell_artifact_manifest(&artifact_manifest_path) {
        Ok(manifest) => {
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.artifact_manifest.parse",
                true,
                "artifact manifest JSON parsed",
                "artifact manifest JSON did not parse",
                "studio.issue.shell_artifact_manifest_parse_failed",
            );
            let validation = validate_shell_artifact_manifest(&manifest, Some(bundle_dir));
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.artifact_manifest.validation",
                validation.status == StudioValidationStatus::Pass,
                "artifact manifest validates against written descriptor files",
                "artifact manifest validation failed against written descriptor files",
                "studio.issue.shell_artifact_manifest_invalid",
            );
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.artifact_manifest.current_match",
                expected_artifact_manifest == Some(&manifest),
                "artifact manifest matches the current selected graph preview",
                "artifact manifest differs from the current selected graph preview",
                "studio.issue.shell_bundle_artifact_manifest_mismatch",
            );
        }
        Err(error) => {
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.artifact_manifest.parse",
                false,
                "artifact manifest JSON parsed",
                &error.to_string(),
                "studio.issue.shell_artifact_manifest_parse_failed",
            );
        }
    }

    let expected_template_index = expected.template_index.as_ref();
    let template_index_path = relative_output_path(bundle_dir, "shell-templates.json");
    let mut template_path_from_index = expected_template_index
        .and_then(|index| index.templates.first())
        .map(|entry| entry.template_path.clone());
    match load_shell_template_index(&template_index_path) {
        Ok(index) => {
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.template_index.parse",
                true,
                "template index JSON parsed",
                "template index JSON did not parse",
                "studio.issue.shell_template_index_parse_failed",
            );
            let validation = validate_shell_template_index(&index, Some(bundle_dir));
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.template_index.validation",
                validation.status == StudioValidationStatus::Pass,
                "template index validates against written template and descriptor files",
                "template index validation failed against written template and descriptor files",
                "studio.issue.shell_template_index_invalid",
            );
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.template_index.current_match",
                expected_template_index == Some(&index),
                "template index matches the current selected graph preview",
                "template index differs from the current selected graph preview",
                "studio.issue.shell_bundle_template_index_mismatch",
            );
            if template_path_from_index.is_none() {
                template_path_from_index = index
                    .templates
                    .first()
                    .map(|entry| entry.template_path.clone());
            }
        }
        Err(error) => {
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.template_index.parse",
                false,
                "template index JSON parsed",
                &error.to_string(),
                "studio.issue.shell_template_index_parse_failed",
            );
        }
    }

    let expected_template_manifest = expected.template_manifest.as_ref();
    if let Some(template_relative_path) = template_path_from_index {
        let template_path = relative_output_path(bundle_dir, &template_relative_path);
        match load_shell_template_manifest(&template_path) {
            Ok(template) => {
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.template_manifest.parse",
                    true,
                    "template manifest JSON parsed",
                    "template manifest JSON did not parse",
                    "studio.issue.shell_template_manifest_parse_failed",
                );
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.template_manifest.current_match",
                    expected_template_manifest == Some(&template),
                    "template manifest matches the current selected graph preview",
                    "template manifest differs from the current selected graph preview",
                    "studio.issue.shell_bundle_template_manifest_mismatch",
                );
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.template_manifest.runtime_authority",
                    template.runtime_authority.command_session_authority == "rusty.manifold"
                        && template.runtime_authority.install_launch_evidence_authority
                            == "rusty.hostess"
                        && template.runtime_authority.studio_role == "authoring.export_planning",
                    "template manifest preserves Manifold, Hostess, and Studio authority boundaries",
                    "template manifest runtime authority changed",
                    "studio.issue.runtime_authority_mismatch",
                );
            }
            Err(error) => {
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.template_manifest.parse",
                    false,
                    "template manifest JSON parsed",
                    &error.to_string(),
                    "studio.issue.shell_template_manifest_parse_failed",
                );
            }
        }
    } else {
        push_bundle_check(
            &mut checks,
            graph_id,
            "studio.check.shell_bundle.template_manifest.path",
            false,
            "current preview has a template manifest path",
            "current preview has no template manifest path",
            "studio.issue.template_missing",
        );
    }

    shell_bundle_validation_report(project, graph_id, expected_bundle_files, checks)
}

pub fn shell_handoff_for_bundle(
    project: &StudioProject,
    base_dir: Option<&Path>,
    graph_id: &str,
    bundle_dir: &Path,
) -> StudioShellHandoffReport {
    let validation = validate_selected_shell_bundle(project, base_dir, graph_id, bundle_dir);
    let artifact_manifest_path = relative_output_path(bundle_dir, "shell-artifacts.json");
    let template_index_path = relative_output_path(bundle_dir, "shell-templates.json");
    if validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_shell_bundle_validation_issue_code(&validation)
            .unwrap_or_else(|| "studio.issue.shell_bundle_validation_failed".to_string());
        return shell_handoff_report(
            project,
            graph_id,
            StudioValidationStatus::Fail,
            Some(issue_code),
            "Selected shell bundle validation failed".to_string(),
            bundle_dir,
            String::new(),
            artifact_manifest_path.display().to_string(),
            template_index_path.display().to_string(),
            String::new(),
            Vec::new(),
            StudioShellTargetKind::Unknown,
            None,
            validation,
        );
    }

    let index = match load_shell_template_index(&template_index_path) {
        Ok(index) => index,
        Err(error) => {
            return shell_handoff_report(
                project,
                graph_id,
                StudioValidationStatus::Fail,
                Some("studio.issue.shell_template_index_parse_failed".to_string()),
                error.to_string(),
                bundle_dir,
                String::new(),
                artifact_manifest_path.display().to_string(),
                template_index_path.display().to_string(),
                String::new(),
                Vec::new(),
                StudioShellTargetKind::Unknown,
                None,
                validation,
            );
        }
    };
    let Some(entry) = index
        .templates
        .iter()
        .find(|entry| entry.graph_id == graph_id)
        .or_else(|| index.templates.first())
    else {
        return shell_handoff_report(
            project,
            graph_id,
            StudioValidationStatus::Fail,
            Some("studio.issue.shell_template_missing".to_string()),
            "Shell template index does not declare a loadable template".to_string(),
            bundle_dir,
            String::new(),
            artifact_manifest_path.display().to_string(),
            template_index_path.display().to_string(),
            String::new(),
            Vec::new(),
            StudioShellTargetKind::Unknown,
            None,
            validation,
        );
    };

    let descriptor_path = relative_output_path(bundle_dir, &entry.descriptor_path);
    let template_manifest_path = relative_output_path(bundle_dir, &entry.template_path);
    let template_manifest = match load_shell_template_manifest(&template_manifest_path) {
        Ok(template_manifest) => template_manifest,
        Err(error) => {
            return shell_handoff_report(
                project,
                graph_id,
                StudioValidationStatus::Fail,
                Some("studio.issue.shell_template_manifest_parse_failed".to_string()),
                error.to_string(),
                bundle_dir,
                descriptor_path.display().to_string(),
                artifact_manifest_path.display().to_string(),
                template_index_path.display().to_string(),
                template_manifest_path.display().to_string(),
                Vec::new(),
                entry.target_kind,
                None,
                validation,
            );
        }
    };

    shell_handoff_report(
        project,
        graph_id,
        StudioValidationStatus::Pass,
        None,
        format!(
            "{} shell handoff ready",
            shell_target_kind_label(entry.target_kind)
        ),
        bundle_dir,
        descriptor_path.display().to_string(),
        artifact_manifest_path.display().to_string(),
        template_index_path.display().to_string(),
        template_manifest_path.display().to_string(),
        vec![
            "--templates".to_string(),
            template_index_path.display().to_string(),
        ],
        entry.target_kind,
        Some(template_manifest.runtime_authority),
        validation,
    )
}

pub fn desktop_shell_handoff_for_bundle(
    project: &StudioProject,
    base_dir: Option<&Path>,
    graph_id: &str,
    bundle_dir: &Path,
) -> StudioShellHandoffReport {
    let report = shell_handoff_for_bundle(project, base_dir, graph_id, bundle_dir);
    if report.status == StudioValidationStatus::Pass
        && report.target_kind != StudioShellTargetKind::Desktop
    {
        return shell_handoff_report(
            project,
            graph_id,
            StudioValidationStatus::Fail,
            Some("studio.issue.shell_handoff_target_mismatch".to_string()),
            format!(
                "Selected shell bundle targets {}; desktop shell handoff requires desktop",
                shell_target_kind_label(report.target_kind)
            ),
            bundle_dir,
            report.descriptor_path,
            report.artifact_manifest_path,
            report.template_index_path,
            report.template_manifest_path,
            Vec::new(),
            report.target_kind,
            report.runtime_authority,
            report.validation,
        );
    }
    report
}

pub fn shell_handoff_readiness_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
    bundle_root: &Path,
) -> StudioShellHandoffReadinessReport {
    let plan = export_plan(project);
    let reference_index = reference_index_for_project(project, base_dir);
    let entries = project
        .graphs
        .iter()
        .zip(plan.bundles.iter())
        .map(|(graph, export_bundle)| {
            let bundle_dir = bundle_root.join(&graph.graph_id);
            let handoff = shell_handoff_for_bundle(project, base_dir, &graph.graph_id, &bundle_dir);
            let host_profile =
                shell_host_profile(&graph.target_host_profile, reference_index.as_ref());
            let intended_target_kind = shell_target_kind(host_profile.host_profile.as_deref());
            shell_handoff_readiness_entry(graph, export_bundle, handoff, intended_target_kind)
        })
        .collect::<Vec<_>>();
    let graph_count = entries.len();
    let ready_count = entries
        .iter()
        .filter(|entry| entry.status == StudioValidationStatus::Pass)
        .count();
    let failed_count = entries
        .iter()
        .filter(|entry| entry.status == StudioValidationStatus::Fail)
        .count();
    let missing_bundle_count = entries
        .iter()
        .filter(|entry| {
            entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
        })
        .count();
    let target_summaries = shell_handoff_readiness_target_summaries(&entries);
    let status = if entries.is_empty()
        || entries
            .iter()
            .any(|entry| entry.status == StudioValidationStatus::Fail)
    {
        StudioValidationStatus::Fail
    } else {
        StudioValidationStatus::Pass
    };
    StudioShellHandoffReadinessReport {
        schema_id: SHELL_HANDOFF_READINESS_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        bundle_root: bundle_root.display().to_string(),
        status,
        graph_count,
        ready_count,
        failed_count,
        missing_bundle_count,
        target_summaries,
        entries,
    }
}

pub fn shell_handoff_manifest_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
    bundle_root: &Path,
) -> StudioShellHandoffManifest {
    let readiness = shell_handoff_readiness_for_project(project, base_dir, bundle_root);
    shell_handoff_manifest_from_readiness(&readiness)
}

pub fn validate_shell_handoff_manifest(
    manifest: &StudioShellHandoffManifest,
) -> StudioShellHandoffManifestValidationReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.schema",
        manifest.schema_id == SHELL_HANDOFF_MANIFEST_SCHEMA,
        "shell handoff manifest schema id is supported",
        "shell handoff manifest schema id is unsupported",
        "studio.issue.shell_handoff_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.manifest_id",
        is_dotted_id(&manifest.manifest_id),
        "manifest id uses dotted-id grammar",
        "manifest id is not a dotted id",
        "studio.issue.invalid_manifest_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.project_id",
        is_dotted_id(&manifest.project_id),
        "project id uses dotted-id grammar",
        "project id is not a dotted id",
        "studio.issue.invalid_project_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.project_revision",
        manifest.project_revision > 0,
        "project revision is positive",
        "project revision must be positive",
        "studio.issue.invalid_revision",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.source_readiness_schema",
        manifest.source_readiness_schema == SHELL_HANDOFF_READINESS_REPORT_SCHEMA,
        "source readiness schema id is supported",
        "source readiness schema id is unsupported",
        "studio.issue.shell_handoff_readiness_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.bundle_root",
        !manifest.bundle_root.trim().is_empty(),
        "bundle root is present",
        "bundle root must be present",
        "studio.issue.missing_bundle_root",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.handoffs_present",
        !manifest.handoffs.is_empty(),
        "manifest declares shell handoffs",
        "manifest must declare at least one shell handoff",
        "studio.issue.no_shell_handoffs",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_manifest.targets_present",
        !manifest.targets.is_empty(),
        "manifest declares target summaries",
        "manifest must declare at least one target summary",
        "studio.issue.no_target_summaries",
    );
    validate_shell_handoff_manifest_counts(manifest, &mut checks);
    validate_shell_handoff_manifest_authority(
        "studio.check.shell_handoff_manifest.runtime_authority",
        &manifest.runtime_authority,
        &mut checks,
    );
    validate_shell_handoff_manifest_target_coverage(manifest, &mut checks);
    for target in &manifest.targets {
        validate_shell_handoff_manifest_target(target, &manifest.handoffs, &mut checks);
    }
    for handoff in &manifest.handoffs {
        validate_shell_handoff_manifest_entry(handoff, &mut checks);
    }

    StudioShellHandoffManifestValidationReport {
        schema_id: SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA.to_string(),
        manifest_id: manifest.manifest_id.clone(),
        status: if checks
            .iter()
            .any(|check| check.status == StudioValidationStatus::Fail)
        {
            StudioValidationStatus::Fail
        } else {
            StudioValidationStatus::Pass
        },
        checks,
    }
}

pub fn shell_handoff_intake_for_manifest(
    manifest: &StudioShellHandoffManifest,
) -> StudioShellHandoffIntakeReport {
    let validation = validate_shell_handoff_manifest(manifest);
    let authority = shell_runtime_authority();
    if validation.status == StudioValidationStatus::Fail {
        return StudioShellHandoffIntakeReport {
            schema_id: SHELL_HANDOFF_INTAKE_REPORT_SCHEMA.to_string(),
            manifest_id: manifest.manifest_id.clone(),
            project_id: manifest.project_id.clone(),
            project_revision: manifest.project_revision,
            status: StudioShellHandoffIntakeStatus::Rejected,
            issue_code: first_failed_validation_check_issue_code(&validation.checks),
            command_session_authority: authority.command_session_authority,
            install_launch_evidence_authority: authority.install_launch_evidence_authority,
            studio_role: authority.studio_role,
            accepted_count: 0,
            blocked_count: 0,
            target_summaries: Vec::new(),
            entries: Vec::new(),
            validation,
        };
    }

    let entries = manifest
        .handoffs
        .iter()
        .map(|handoff| shell_handoff_intake_entry(handoff, &authority))
        .collect::<Vec<_>>();
    let accepted_count = entries
        .iter()
        .filter(|entry| entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner)
        .count();
    let blocked_count = entries.len() - accepted_count;
    let target_summaries = shell_handoff_intake_target_summaries(&entries);

    StudioShellHandoffIntakeReport {
        schema_id: SHELL_HANDOFF_INTAKE_REPORT_SCHEMA.to_string(),
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        status: StudioShellHandoffIntakeStatus::Accepted,
        issue_code: None,
        command_session_authority: authority.command_session_authority,
        install_launch_evidence_authority: authority.install_launch_evidence_authority,
        studio_role: authority.studio_role,
        accepted_count,
        blocked_count,
        target_summaries,
        entries,
        validation,
    }
}

pub fn shell_runbook_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
    bundle_root: &Path,
) -> StudioShellRunbookReport {
    let manifest = shell_handoff_manifest_for_project(project, base_dir, bundle_root);
    shell_runbook_for_manifest(&manifest)
}

pub fn shell_runbook_for_manifest(
    manifest: &StudioShellHandoffManifest,
) -> StudioShellRunbookReport {
    let intake = shell_handoff_intake_for_manifest(manifest);
    let entries = intake
        .entries
        .iter()
        .map(shell_runbook_entry)
        .collect::<Vec<_>>();
    let ready_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellRunbookStatus::Ready)
        .count();
    let blocked_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellRunbookStatus::Blocked)
        .count();
    let rejected_count = if intake.status == StudioShellHandoffIntakeStatus::Rejected {
        1
    } else {
        entries
            .iter()
            .filter(|entry| entry.status == StudioShellRunbookStatus::Rejected)
            .count()
    };
    let status = if intake.status == StudioShellHandoffIntakeStatus::Rejected {
        StudioShellRunbookStatus::Rejected
    } else if blocked_count > 0 || entries.is_empty() {
        StudioShellRunbookStatus::Blocked
    } else {
        StudioShellRunbookStatus::Ready
    };
    let issue_code = match status {
        StudioShellRunbookStatus::Ready => None,
        StudioShellRunbookStatus::Blocked => entries
            .iter()
            .find(|entry| entry.status == StudioShellRunbookStatus::Blocked)
            .and_then(|entry| entry.issue_code.clone()),
        StudioShellRunbookStatus::Rejected => intake.issue_code.clone(),
    };

    StudioShellRunbookReport {
        schema_id: SHELL_RUNBOOK_REPORT_SCHEMA.to_string(),
        source_manifest_schema: manifest.schema_id.clone(),
        source_intake_schema: intake.schema_id.clone(),
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        bundle_root: manifest.bundle_root.clone(),
        status,
        issue_code,
        ready_count,
        blocked_count,
        rejected_count,
        target_summaries: shell_runbook_target_summaries(&entries),
        prohibited_actions: shell_handoff_acceptance_prohibited_actions(),
        entries,
    }
}

fn shell_runbook_entry(entry: &StudioShellHandoffIntakeEntry) -> StudioShellRunbookEntry {
    let (host_routes, route_status, route_issue_code) = shell_runbook_host_routes(entry);
    let status = if route_status == StudioValidationStatus::Pass
        && entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner
    {
        StudioShellRunbookStatus::Ready
    } else {
        StudioShellRunbookStatus::Blocked
    };
    let issue_code = match status {
        StudioShellRunbookStatus::Ready => None,
        StudioShellRunbookStatus::Blocked => route_issue_code
            .clone()
            .or_else(|| entry.issue_code.clone())
            .or_else(|| Some("studio.issue.shell_runbook_blocked".to_string())),
        StudioShellRunbookStatus::Rejected => entry.issue_code.clone(),
    };
    let responsible_owner = if status == StudioShellRunbookStatus::Ready {
        entry.install_launch_evidence_authority.clone()
    } else {
        "rusty.studio".to_string()
    };
    let cli_request =
        if status == StudioShellRunbookStatus::Ready && !entry.consumer_args.is_empty() {
            ["cargo", "run", "-p", entry.consumer_id.as_str(), "--"]
                .into_iter()
                .map(str::to_string)
                .chain(entry.consumer_args.iter().cloned())
                .collect()
        } else {
            Vec::new()
        };

    StudioShellRunbookEntry {
        export_bundle_id: entry.export_bundle_id.clone(),
        graph_id: entry.graph_id.clone(),
        display_name: entry.display_name.clone(),
        target_host_profile: entry.target_host_profile.clone(),
        target_kind: entry.target_kind,
        handoff_kind: entry.handoff_kind,
        status,
        issue_code,
        decision: entry.decision,
        responsible_owner,
        handoff_request_kind: entry.handoff_request_kind.clone(),
        runtime_route_kind: entry.runtime_route_kind.clone(),
        next_required_action: entry.next_required_action.clone(),
        execution_policy: "not_executed.request_only".to_string(),
        command_session_authority: entry.command_session_authority.clone(),
        install_launch_evidence_authority: entry.install_launch_evidence_authority.clone(),
        studio_role: entry.studio_role.clone(),
        consumer_id: entry.consumer_id.clone(),
        bundle_dir: entry.bundle_dir.clone(),
        template_index_path: entry.template_index_path.clone(),
        consumer_args: entry.consumer_args.clone(),
        cli_request,
        host_routes,
        route_status,
        route_issue_code,
        package_ids: entry.package_ids.clone(),
        module_ids: entry.module_ids.clone(),
        operator_shell_ids: entry.operator_shell_ids.clone(),
    }
}

fn shell_runbook_host_routes(
    entry: &StudioShellHandoffIntakeEntry,
) -> (
    StudioShellHostRoutes,
    StudioValidationStatus,
    Option<String>,
) {
    if entry.decision != StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner {
        return (
            empty_shell_host_routes(),
            StudioValidationStatus::Fail,
            entry.issue_code.clone(),
        );
    }

    let index = match load_shell_template_index(Path::new(&entry.template_index_path)) {
        Ok(index) => index,
        Err(_) => {
            return (
                empty_shell_host_routes(),
                StudioValidationStatus::Fail,
                Some("studio.issue.shell_runbook_template_index_load_failed".to_string()),
            );
        }
    };
    let Some(template_entry) = index
        .templates
        .iter()
        .find(|template| template.graph_id == entry.graph_id)
    else {
        return (
            empty_shell_host_routes(),
            StudioValidationStatus::Fail,
            Some("studio.issue.shell_runbook_template_missing".to_string()),
        );
    };
    let template_path =
        relative_output_path(Path::new(&entry.bundle_dir), &template_entry.template_path);
    match load_shell_template_manifest(&template_path) {
        Ok(template) => (template.host_routes, StudioValidationStatus::Pass, None),
        Err(_) => (
            empty_shell_host_routes(),
            StudioValidationStatus::Fail,
            Some("studio.issue.shell_runbook_template_manifest_load_failed".to_string()),
        ),
    }
}

fn empty_shell_host_routes() -> StudioShellHostRoutes {
    StudioShellHostRoutes {
        app_id: None,
        install_route: None,
        launch_route: None,
        command_bridge: None,
        evidence_pull_route: None,
    }
}

fn shell_runbook_target_summaries(
    entries: &[StudioShellRunbookEntry],
) -> Vec<StudioShellRunbookTargetSummary> {
    shell_target_kinds()
        .iter()
        .filter_map(|target_kind| shell_runbook_target_summary(entries, *target_kind))
        .collect()
}

fn shell_runbook_target_summary(
    entries: &[StudioShellRunbookEntry],
    target_kind: StudioShellTargetKind,
) -> Option<StudioShellRunbookTargetSummary> {
    let target_entries = entries
        .iter()
        .filter(|entry| entry.target_kind == target_kind)
        .collect::<Vec<_>>();
    if target_entries.is_empty() {
        return None;
    }

    Some(StudioShellRunbookTargetSummary {
        target_kind,
        ready_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellRunbookStatus::Ready)
            .count(),
        blocked_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellRunbookStatus::Blocked)
            .count(),
        rejected_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellRunbookStatus::Rejected)
            .count(),
        graph_ids: unique_strings(target_entries.iter().map(|entry| entry.graph_id.clone())),
        consumer_ids: unique_strings(target_entries.iter().map(|entry| entry.consumer_id.clone())),
        responsible_owners: unique_strings(
            target_entries
                .iter()
                .map(|entry| entry.responsible_owner.clone()),
        ),
        runtime_route_kinds: unique_strings(
            target_entries
                .iter()
                .map(|entry| entry.runtime_route_kind.clone()),
        ),
        issue_codes: unique_strings(
            target_entries
                .iter()
                .filter_map(|entry| entry.issue_code.clone()),
        ),
    })
}

pub fn shell_export_package_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
    bundle_root: &Path,
) -> StudioShellExportPackageReport {
    let manifest = shell_handoff_manifest_for_project(project, base_dir, bundle_root);
    shell_export_package_for_manifest(&manifest)
}

pub fn shell_export_package_for_manifest(
    manifest: &StudioShellHandoffManifest,
) -> StudioShellExportPackageReport {
    let runbook = shell_runbook_for_manifest(manifest);
    let entries = runbook
        .entries
        .iter()
        .map(shell_export_package_entry)
        .collect::<Vec<_>>();
    let ready_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellExportPackageStatus::Ready)
        .count();
    let blocked_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellExportPackageStatus::Blocked)
        .count();
    let rejected_count = if runbook.status == StudioShellRunbookStatus::Rejected {
        1
    } else {
        entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Rejected)
            .count()
    };
    let descriptor_count = entries
        .iter()
        .filter(|entry| entry.descriptor.is_some())
        .count();
    let template_manifest_count = entries
        .iter()
        .filter(|entry| entry.template_manifest.is_some())
        .count();
    let status = if runbook.status == StudioShellRunbookStatus::Rejected {
        StudioShellExportPackageStatus::Rejected
    } else if blocked_count > 0 || entries.is_empty() {
        StudioShellExportPackageStatus::Blocked
    } else {
        StudioShellExportPackageStatus::Ready
    };
    let issue_code = match status {
        StudioShellExportPackageStatus::Ready => None,
        StudioShellExportPackageStatus::Blocked => entries
            .iter()
            .find(|entry| entry.status == StudioShellExportPackageStatus::Blocked)
            .and_then(|entry| entry.issue_code.clone()),
        StudioShellExportPackageStatus::Rejected => runbook.issue_code.clone(),
    };

    StudioShellExportPackageReport {
        schema_id: SHELL_EXPORT_PACKAGE_REPORT_SCHEMA.to_string(),
        source_manifest_schema: manifest.schema_id.clone(),
        source_runbook_schema: runbook.schema_id.clone(),
        package_id: format!("studio.shell_export_package.{}", manifest.project_id),
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        bundle_root: manifest.bundle_root.clone(),
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        review_owner: "rusty.hostess".to_string(),
        command_session_authority: manifest.runtime_authority.command_session_authority.clone(),
        install_launch_evidence_authority: manifest
            .runtime_authority
            .install_launch_evidence_authority
            .clone(),
        studio_role: manifest.runtime_authority.studio_role.clone(),
        ready_count,
        blocked_count,
        rejected_count,
        descriptor_count,
        template_manifest_count,
        runbook_entry_count: runbook.entries.len(),
        target_summaries: shell_export_package_target_summaries(&entries),
        prohibited_actions: runbook.prohibited_actions,
        entries,
    }
}

fn shell_export_package_entry(entry: &StudioShellRunbookEntry) -> StudioShellExportPackageEntry {
    let (descriptor, template_manifest, package_issue_code) =
        shell_export_package_artifact_refs(entry);
    let source_status = match entry.status {
        StudioShellRunbookStatus::Ready => StudioShellExportPackageStatus::Ready,
        StudioShellRunbookStatus::Blocked => StudioShellExportPackageStatus::Blocked,
        StudioShellRunbookStatus::Rejected => StudioShellExportPackageStatus::Rejected,
    };
    let status = if source_status == StudioShellExportPackageStatus::Ready
        && descriptor.is_some()
        && template_manifest.is_some()
    {
        StudioShellExportPackageStatus::Ready
    } else if source_status == StudioShellExportPackageStatus::Rejected {
        StudioShellExportPackageStatus::Rejected
    } else {
        StudioShellExportPackageStatus::Blocked
    };
    let issue_code = match status {
        StudioShellExportPackageStatus::Ready => None,
        StudioShellExportPackageStatus::Blocked => package_issue_code
            .or_else(|| entry.issue_code.clone())
            .or_else(|| Some("studio.issue.shell_export_package_blocked".to_string())),
        StudioShellExportPackageStatus::Rejected => entry.issue_code.clone(),
    };
    let responsible_owner = if status == StudioShellExportPackageStatus::Ready {
        entry.responsible_owner.clone()
    } else {
        "rusty.studio".to_string()
    };

    StudioShellExportPackageEntry {
        export_bundle_id: entry.export_bundle_id.clone(),
        graph_id: entry.graph_id.clone(),
        display_name: entry.display_name.clone(),
        target_host_profile: entry.target_host_profile.clone(),
        target_kind: entry.target_kind,
        status,
        issue_code,
        responsible_owner,
        execution_policy: "not_executed.review_only".to_string(),
        consumer_id: entry.consumer_id.clone(),
        runtime_route_kind: entry.runtime_route_kind.clone(),
        next_required_action: "review_with_runtime_owner".to_string(),
        bundle_dir: entry.bundle_dir.clone(),
        descriptor,
        template_manifest,
        runbook_cli_request: if status == StudioShellExportPackageStatus::Ready {
            entry.cli_request.clone()
        } else {
            Vec::new()
        },
        host_routes: entry.host_routes.clone(),
        package_ids: entry.package_ids.clone(),
        module_ids: entry.module_ids.clone(),
        operator_shell_ids: entry.operator_shell_ids.clone(),
    }
}

fn shell_export_package_artifact_refs(
    entry: &StudioShellRunbookEntry,
) -> (
    Option<StudioShellExportPackageDescriptorRef>,
    Option<StudioShellExportPackageTemplateRef>,
    Option<String>,
) {
    if entry.status != StudioShellRunbookStatus::Ready
        && entry.decision != StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner
    {
        return (None, None, entry.issue_code.clone());
    }

    let index = match load_shell_template_index(Path::new(&entry.template_index_path)) {
        Ok(index) => index,
        Err(_) => {
            return (
                None,
                None,
                Some("studio.issue.shell_export_package_template_index_load_failed".to_string()),
            );
        }
    };
    let Some(template_entry) = index
        .templates
        .iter()
        .find(|template| template.graph_id == entry.graph_id)
    else {
        return (
            None,
            None,
            Some("studio.issue.shell_export_package_template_missing".to_string()),
        );
    };

    let descriptor_path = relative_output_path(
        Path::new(&entry.bundle_dir),
        &template_entry.descriptor_path,
    );
    let template_manifest_path =
        relative_output_path(Path::new(&entry.bundle_dir), &template_entry.template_path);

    let (descriptor_ref, descriptor_issue_code) = match load_shell_descriptor(&descriptor_path) {
        Ok(descriptor)
            if descriptor.graph_id == entry.graph_id
                && descriptor.shell_id == template_entry.shell_id =>
        {
            (
                Some(shell_export_package_descriptor_ref(
                    &descriptor,
                    &descriptor_path,
                )),
                None,
            )
        }
        Ok(_) => (
            None,
            Some("studio.issue.shell_export_package_descriptor_mismatch".to_string()),
        ),
        Err(_) => (
            None,
            Some("studio.issue.shell_export_package_descriptor_load_failed".to_string()),
        ),
    };

    let (template_ref, template_issue_code) =
        match load_shell_template_manifest(&template_manifest_path) {
            Ok(template)
                if template.graph_id == entry.graph_id
                    && template.template_id == template_entry.template_id
                    && template.artifact_id == template_entry.artifact_id =>
            {
                (
                    Some(shell_export_package_template_ref(
                        &template,
                        &entry.template_index_path,
                        &template_manifest_path,
                    )),
                    None,
                )
            }
            Ok(_) => (
                None,
                Some("studio.issue.shell_export_package_template_mismatch".to_string()),
            ),
            Err(_) => (
                None,
                Some("studio.issue.shell_export_package_template_load_failed".to_string()),
            ),
        };

    (
        descriptor_ref,
        template_ref,
        descriptor_issue_code.or(template_issue_code),
    )
}

fn shell_export_package_descriptor_ref(
    descriptor: &StudioShellDescriptor,
    descriptor_path: &Path,
) -> StudioShellExportPackageDescriptorRef {
    StudioShellExportPackageDescriptorRef {
        descriptor_path: descriptor_path.display().to_string(),
        descriptor_id: descriptor.descriptor_id.clone(),
        graph_id: descriptor.graph_id.clone(),
        shell_id: descriptor.shell_id.clone(),
        target_host_profile: descriptor.target_host_profile.clone(),
        package_count: descriptor.package_ids.len(),
        module_count: descriptor.module_ids.len(),
        command_binding_count: descriptor.command_bindings.len(),
        stream_binding_count: descriptor.stream_bindings.len(),
        validation_slot_count: descriptor.validation_slot_ids.len(),
    }
}

fn shell_export_package_template_ref(
    template: &StudioShellTemplateManifest,
    template_index_path: &str,
    template_manifest_path: &Path,
) -> StudioShellExportPackageTemplateRef {
    StudioShellExportPackageTemplateRef {
        template_index_path: template_index_path.to_string(),
        template_manifest_path: template_manifest_path.display().to_string(),
        template_id: template.template_id.clone(),
        artifact_id: template.artifact_id.clone(),
        graph_id: template.graph_id.clone(),
        shell_id: template.shell_id.clone(),
        target_host_profile: template.target_host_profile.clone(),
        host_routes: template.host_routes.clone(),
        runtime_authority: template.runtime_authority.clone(),
    }
}

fn shell_export_package_target_summaries(
    entries: &[StudioShellExportPackageEntry],
) -> Vec<StudioShellExportPackageTargetSummary> {
    shell_target_kinds()
        .iter()
        .filter_map(|target_kind| shell_export_package_target_summary(entries, *target_kind))
        .collect()
}

fn shell_export_package_target_summary(
    entries: &[StudioShellExportPackageEntry],
    target_kind: StudioShellTargetKind,
) -> Option<StudioShellExportPackageTargetSummary> {
    let target_entries = entries
        .iter()
        .filter(|entry| entry.target_kind == target_kind)
        .collect::<Vec<_>>();
    if target_entries.is_empty() {
        return None;
    }

    Some(StudioShellExportPackageTargetSummary {
        target_kind,
        ready_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Ready)
            .count(),
        blocked_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Blocked)
            .count(),
        rejected_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Rejected)
            .count(),
        descriptor_count: target_entries
            .iter()
            .filter(|entry| entry.descriptor.is_some())
            .count(),
        template_manifest_count: target_entries
            .iter()
            .filter(|entry| entry.template_manifest.is_some())
            .count(),
        graph_ids: unique_strings(target_entries.iter().map(|entry| entry.graph_id.clone())),
        consumer_ids: unique_strings(target_entries.iter().map(|entry| entry.consumer_id.clone())),
        responsible_owners: unique_strings(
            target_entries
                .iter()
                .map(|entry| entry.responsible_owner.clone()),
        ),
        issue_codes: unique_strings(
            target_entries
                .iter()
                .filter_map(|entry| entry.issue_code.clone()),
        ),
    })
}

pub fn shell_export_package_baseline_manifest_for_report(
    package: &StudioShellExportPackageReport,
    package_path: &Path,
    baseline_id: Option<&str>,
    label: Option<&str>,
) -> StudioShellExportPackageBaselineManifest {
    let baseline_id = baseline_id
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_export_package_baseline_id(package));
    let label = label
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_export_package_baseline_label(package));

    StudioShellExportPackageBaselineManifest {
        schema_id: SHELL_EXPORT_PACKAGE_BASELINE_MANIFEST_SCHEMA.to_string(),
        baseline_id,
        label,
        package_path: package_path.display().to_string(),
        package_schema: package.schema_id.clone(),
        package_id: package.package_id.clone(),
        manifest_id: package.manifest_id.clone(),
        project_id: package.project_id.clone(),
        project_revision: package.project_revision,
        status: package.status,
        issue_code: package.issue_code.clone(),
        execution_policy: package.execution_policy.clone(),
        review_owner: package.review_owner.clone(),
        command_session_authority: package.command_session_authority.clone(),
        install_launch_evidence_authority: package.install_launch_evidence_authority.clone(),
        studio_role: package.studio_role.clone(),
        ready_count: package.ready_count,
        blocked_count: package.blocked_count,
        rejected_count: package.rejected_count,
        descriptor_count: package.descriptor_count,
        template_manifest_count: package.template_manifest_count,
        runbook_entry_count: package.runbook_entry_count,
        target_count: package.target_summaries.len(),
        prohibited_actions: package.prohibited_actions.clone(),
    }
}

pub fn shell_export_package_baseline_index_for_manifests(
    baselines: Vec<(StudioShellExportPackageBaselineManifest, Option<PathBuf>)>,
    default_baseline_id: Option<&str>,
) -> StudioShellExportPackageBaselineIndex {
    let entries = baselines
        .into_iter()
        .map(|(baseline, baseline_manifest_path)| {
            shell_export_package_baseline_index_entry_for_manifest(baseline, baseline_manifest_path)
        })
        .collect::<Vec<_>>();

    shell_export_package_baseline_index_for_entries(entries, default_baseline_id)
}

pub fn append_shell_export_package_baseline_index_manifests(
    index: &StudioShellExportPackageBaselineIndex,
    baselines: Vec<(StudioShellExportPackageBaselineManifest, Option<PathBuf>)>,
    default_baseline_id: Option<&str>,
) -> StudioShellExportPackageBaselineIndex {
    let entries = index
        .entries
        .iter()
        .cloned()
        .chain(
            baselines
                .into_iter()
                .map(|(baseline, baseline_manifest_path)| {
                    shell_export_package_baseline_index_entry_for_manifest(
                        baseline,
                        baseline_manifest_path,
                    )
                }),
        )
        .collect::<Vec<_>>();
    let default_baseline_id = default_baseline_id.or(index.default_baseline_id.as_deref());

    shell_export_package_baseline_index_for_entries(entries, default_baseline_id)
}

pub fn promote_shell_export_package_baseline_index_default(
    index: &StudioShellExportPackageBaselineIndex,
    baseline_id: &str,
) -> Option<StudioShellExportPackageBaselineIndex> {
    index
        .entries
        .iter()
        .any(|entry| entry.baseline_id == baseline_id)
        .then(|| {
            shell_export_package_baseline_index_for_entries(
                index.entries.clone(),
                Some(baseline_id),
            )
        })
}

fn shell_export_package_baseline_index_entry_for_manifest(
    baseline: StudioShellExportPackageBaselineManifest,
    baseline_manifest_path: Option<PathBuf>,
) -> StudioShellExportPackageBaselineIndexEntry {
    StudioShellExportPackageBaselineIndexEntry {
        baseline_id: baseline.baseline_id,
        label: baseline.label,
        baseline_manifest_path: baseline_manifest_path.map(|path| path.display().to_string()),
        package_path: baseline.package_path,
        package_schema: baseline.package_schema,
        package_id: baseline.package_id,
        manifest_id: baseline.manifest_id,
        project_id: baseline.project_id,
        project_revision: baseline.project_revision,
        status: baseline.status,
        issue_code: baseline.issue_code,
        ready_count: baseline.ready_count,
        blocked_count: baseline.blocked_count,
        rejected_count: baseline.rejected_count,
        descriptor_count: baseline.descriptor_count,
        template_manifest_count: baseline.template_manifest_count,
        runbook_entry_count: baseline.runbook_entry_count,
        target_count: baseline.target_count,
    }
}

fn shell_export_package_baseline_index_for_entries(
    entries: Vec<StudioShellExportPackageBaselineIndexEntry>,
    default_baseline_id: Option<&str>,
) -> StudioShellExportPackageBaselineIndex {
    let mut by_id = BTreeMap::new();
    for entry in entries {
        by_id.insert(entry.baseline_id.clone(), entry);
    }

    let entries = by_id.into_values().collect::<Vec<_>>();
    let default_baseline_id = default_baseline_id
        .filter(|baseline_id| {
            entries
                .iter()
                .any(|entry| entry.baseline_id == *baseline_id)
        })
        .map(str::to_string)
        .or_else(|| entries.first().map(|entry| entry.baseline_id.clone()));

    StudioShellExportPackageBaselineIndex {
        schema_id: SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA.to_string(),
        project_ids: unique_strings(entries.iter().map(|entry| entry.project_id.clone())),
        package_ids: unique_strings(entries.iter().map(|entry| entry.package_id.clone())),
        manifest_ids: unique_strings(entries.iter().map(|entry| entry.manifest_id.clone())),
        default_baseline_id,
        baseline_count: entries.len(),
        ready_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Ready)
            .count(),
        blocked_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Blocked)
            .count(),
        rejected_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellExportPackageStatus::Rejected)
            .count(),
        entries,
    }
}

pub fn select_shell_export_package_baseline_index_entry<'a>(
    index: &'a StudioShellExportPackageBaselineIndex,
    baseline_id: Option<&str>,
) -> Option<&'a StudioShellExportPackageBaselineIndexEntry> {
    let selected_id = baseline_id.or(index.default_baseline_id.as_deref());
    selected_id
        .and_then(|selected_id| {
            index
                .entries
                .iter()
                .find(|entry| entry.baseline_id == selected_id)
        })
        .or_else(|| {
            baseline_id
                .is_none()
                .then(|| index.entries.first())
                .flatten()
        })
}

pub fn summarize_shell_export_package_baseline_index_selection(
    index: &StudioShellExportPackageBaselineIndex,
    index_path: Option<&Path>,
    requested_baseline_id: Option<&str>,
) -> StudioShellExportPackageBaselineSelectionReport {
    let selected_entry =
        select_shell_export_package_baseline_index_entry(index, requested_baseline_id);
    let selected_baseline_id = selected_entry.map(|entry| entry.baseline_id.clone());
    let status = if index.entries.is_empty() {
        StudioShellExportPackageBaselineSelectionStatus::Empty
    } else if selected_entry.is_some() {
        StudioShellExportPackageBaselineSelectionStatus::Selected
    } else {
        StudioShellExportPackageBaselineSelectionStatus::Missing
    };
    let issue_code = match status {
        StudioShellExportPackageBaselineSelectionStatus::Selected => None,
        StudioShellExportPackageBaselineSelectionStatus::Missing => {
            Some("studio.issue.shell_export_package_baseline_not_found".to_string())
        }
        StudioShellExportPackageBaselineSelectionStatus::Empty => {
            Some("studio.issue.shell_export_package_baseline_index_empty".to_string())
        }
    };

    StudioShellExportPackageBaselineSelectionReport {
        schema_id: SHELL_EXPORT_PACKAGE_BASELINE_SELECTION_SCHEMA.to_string(),
        source_index_schema: index.schema_id.clone(),
        index_path: index_path.map(|path| path.display().to_string()),
        requested_baseline_id: requested_baseline_id.map(str::to_string),
        default_baseline_id: index.default_baseline_id.clone(),
        selected_baseline_id: selected_baseline_id.clone(),
        status,
        issue_code,
        baseline_count: index.baseline_count,
        ready_baseline_count: index.ready_baseline_count,
        blocked_baseline_count: index.blocked_baseline_count,
        rejected_baseline_count: index.rejected_baseline_count,
        project_ids: index.project_ids.clone(),
        package_ids: index.package_ids.clone(),
        manifest_ids: index.manifest_ids.clone(),
        entries: index
            .entries
            .iter()
            .map(|entry| StudioShellExportPackageBaselineSelectionEntry {
                baseline_id: entry.baseline_id.clone(),
                label: entry.label.clone(),
                selected: selected_baseline_id.as_deref() == Some(entry.baseline_id.as_str()),
                default: index.default_baseline_id.as_deref() == Some(entry.baseline_id.as_str()),
                baseline_manifest_path: entry.baseline_manifest_path.clone(),
                package_path: entry.package_path.clone(),
                package_id: entry.package_id.clone(),
                manifest_id: entry.manifest_id.clone(),
                project_id: entry.project_id.clone(),
                project_revision: entry.project_revision,
                status: entry.status,
                issue_code: entry.issue_code.clone(),
                ready_count: entry.ready_count,
                blocked_count: entry.blocked_count,
                rejected_count: entry.rejected_count,
                descriptor_count: entry.descriptor_count,
                template_manifest_count: entry.template_manifest_count,
                runbook_entry_count: entry.runbook_entry_count,
                target_count: entry.target_count,
            })
            .collect(),
    }
}

fn default_shell_export_package_baseline_id(package: &StudioShellExportPackageReport) -> String {
    format!(
        "{}.rev{}.{}",
        package.project_id,
        package.project_revision,
        shell_export_package_status_key(package.status)
    )
}

fn default_shell_export_package_baseline_label(package: &StudioShellExportPackageReport) -> String {
    format!(
        "{} revision {} {} export package baseline",
        package.project_id,
        package.project_revision,
        shell_export_package_status_key(package.status)
    )
}

fn shell_export_package_status_key(status: StudioShellExportPackageStatus) -> &'static str {
    match status {
        StudioShellExportPackageStatus::Ready => "ready",
        StudioShellExportPackageStatus::Blocked => "blocked",
        StudioShellExportPackageStatus::Rejected => "rejected",
    }
}

pub fn compare_shell_export_packages(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> StudioShellExportPackageComparisonReport {
    compare_shell_export_packages_with_identity(baseline, candidate, None, None)
}

pub fn compare_shell_export_packages_against_baseline_manifest(
    baseline_identity: &StudioShellExportPackageBaselineManifest,
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> StudioShellExportPackageComparisonReport {
    compare_shell_export_packages_with_identity(baseline, candidate, Some(baseline_identity), None)
}

pub fn compare_shell_export_packages_against_baseline_index_entry(
    baseline_index: &StudioShellExportPackageBaselineIndex,
    baseline_index_path: Option<&Path>,
    baseline_index_entry: &StudioShellExportPackageBaselineIndexEntry,
    baseline_manifest_path: Option<&Path>,
    baseline_identity: &StudioShellExportPackageBaselineManifest,
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> StudioShellExportPackageComparisonReport {
    compare_shell_export_packages_with_identity(
        baseline,
        candidate,
        Some(baseline_identity),
        Some(ShellExportPackageBaselineIndexComparisonContext {
            index: baseline_index,
            index_path: baseline_index_path,
            entry: baseline_index_entry,
            baseline_manifest_path,
        }),
    )
}

struct ShellExportPackageBaselineIndexComparisonContext<'a> {
    index: &'a StudioShellExportPackageBaselineIndex,
    index_path: Option<&'a Path>,
    entry: &'a StudioShellExportPackageBaselineIndexEntry,
    baseline_manifest_path: Option<&'a Path>,
}

fn compare_shell_export_packages_with_identity(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
    baseline_identity: Option<&StudioShellExportPackageBaselineManifest>,
    baseline_index: Option<ShellExportPackageBaselineIndexComparisonContext<'_>>,
) -> StudioShellExportPackageComparisonReport {
    let mut checks = shell_export_package_comparison_checks(baseline, candidate);
    if let Some(baseline_identity) = baseline_identity {
        checks.extend(shell_export_package_baseline_identity_checks(
            baseline_identity,
            baseline,
        ));
        if let Some(baseline_index) = baseline_index.as_ref() {
            checks.extend(shell_export_package_baseline_index_entry_checks(
                baseline_index,
                baseline_identity,
            ));
        }
    }
    let comparable = checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass);

    let entries = if comparable {
        shell_export_package_comparison_entries(baseline, candidate)
    } else {
        Vec::new()
    };

    let ready_delta = count_delta(candidate.ready_count, baseline.ready_count);
    let blocked_delta = count_delta(candidate.blocked_count, baseline.blocked_count);
    let rejected_delta = count_delta(candidate.rejected_count, baseline.rejected_count);
    let descriptor_delta = count_delta(candidate.descriptor_count, baseline.descriptor_count);
    let template_manifest_delta = count_delta(
        candidate.template_manifest_count,
        baseline.template_manifest_count,
    );
    let runbook_entry_delta =
        count_delta(candidate.runbook_entry_count, baseline.runbook_entry_count);

    let status = if !comparable {
        StudioShellExportPackageComparisonStatus::Incomparable
    } else if export_package_status_score(candidate.status)
        < export_package_status_score(baseline.status)
        || ready_delta < 0
        || blocked_delta > 0
        || rejected_delta > 0
        || descriptor_delta < 0
        || template_manifest_delta < 0
        || runbook_entry_delta < 0
        || entries.iter().any(|entry| {
            matches!(
                entry.change,
                StudioShellExportPackageComparisonChange::Regressed
                    | StudioShellExportPackageComparisonChange::Removed
                    | StudioShellExportPackageComparisonChange::Changed
            )
        })
    {
        StudioShellExportPackageComparisonStatus::Regressed
    } else if export_package_status_score(candidate.status)
        > export_package_status_score(baseline.status)
        || ready_delta > 0
        || blocked_delta < 0
        || rejected_delta < 0
        || descriptor_delta > 0
        || template_manifest_delta > 0
        || runbook_entry_delta > 0
        || entries
            .iter()
            .any(|entry| entry.change == StudioShellExportPackageComparisonChange::Improved)
    {
        StudioShellExportPackageComparisonStatus::Improved
    } else {
        StudioShellExportPackageComparisonStatus::Unchanged
    };

    let issue_code = match status {
        StudioShellExportPackageComparisonStatus::Incomparable => {
            first_failed_validation_check_issue_code(&checks)
        }
        StudioShellExportPackageComparisonStatus::Regressed => entries
            .iter()
            .find(|entry| {
                matches!(
                    entry.change,
                    StudioShellExportPackageComparisonChange::Regressed
                        | StudioShellExportPackageComparisonChange::Removed
                        | StudioShellExportPackageComparisonChange::Changed
                )
            })
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| Some("studio.issue.shell_export_package_regressed".to_string())),
        StudioShellExportPackageComparisonStatus::Improved
        | StudioShellExportPackageComparisonStatus::Unchanged => None,
    };

    StudioShellExportPackageComparisonReport {
        schema_id: SHELL_EXPORT_PACKAGE_COMPARISON_SCHEMA.to_string(),
        baseline_identity_schema: baseline_identity.map(|identity| identity.schema_id.clone()),
        baseline_id: baseline_identity.map(|identity| identity.baseline_id.clone()),
        baseline_label: baseline_identity.map(|identity| identity.label.clone()),
        baseline_package_path: baseline_identity.map(|identity| identity.package_path.clone()),
        baseline_index_schema: baseline_index
            .as_ref()
            .map(|context| context.index.schema_id.clone()),
        baseline_index_path: baseline_index
            .as_ref()
            .and_then(|context| context.index_path.map(|path| path.display().to_string())),
        baseline_index_default_baseline_id: baseline_index
            .as_ref()
            .and_then(|context| context.index.default_baseline_id.clone()),
        baseline_index_selected_baseline_id: baseline_index
            .as_ref()
            .map(|context| context.entry.baseline_id.clone()),
        baseline_schema: baseline.schema_id.clone(),
        candidate_schema: candidate.schema_id.clone(),
        baseline_package_id: baseline.package_id.clone(),
        candidate_package_id: candidate.package_id.clone(),
        baseline_manifest_id: baseline.manifest_id.clone(),
        candidate_manifest_id: candidate.manifest_id.clone(),
        baseline_project_id: baseline.project_id.clone(),
        candidate_project_id: candidate.project_id.clone(),
        baseline_project_revision: baseline.project_revision,
        candidate_project_revision: candidate.project_revision,
        baseline_status: baseline.status,
        candidate_status: candidate.status,
        status,
        issue_code,
        baseline_ready_count: baseline.ready_count,
        candidate_ready_count: candidate.ready_count,
        ready_delta,
        baseline_blocked_count: baseline.blocked_count,
        candidate_blocked_count: candidate.blocked_count,
        blocked_delta,
        baseline_rejected_count: baseline.rejected_count,
        candidate_rejected_count: candidate.rejected_count,
        rejected_delta,
        baseline_descriptor_count: baseline.descriptor_count,
        candidate_descriptor_count: candidate.descriptor_count,
        descriptor_delta,
        baseline_template_manifest_count: baseline.template_manifest_count,
        candidate_template_manifest_count: candidate.template_manifest_count,
        template_manifest_delta,
        baseline_runbook_entry_count: baseline.runbook_entry_count,
        candidate_runbook_entry_count: candidate.runbook_entry_count,
        runbook_entry_delta,
        checks,
        entries,
    }
}

fn shell_export_package_comparison_checks(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_schema",
        baseline.schema_id == SHELL_EXPORT_PACKAGE_REPORT_SCHEMA,
        "baseline export-package schema id is supported",
        "baseline export-package schema id is unsupported",
        "studio.issue.shell_export_package_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.candidate_schema",
        candidate.schema_id == SHELL_EXPORT_PACKAGE_REPORT_SCHEMA,
        "candidate export-package schema id is supported",
        "candidate export-package schema id is unsupported",
        "studio.issue.shell_export_package_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_source_schemas",
        baseline.source_manifest_schema == SHELL_HANDOFF_MANIFEST_SCHEMA
            && baseline.source_runbook_schema == SHELL_RUNBOOK_REPORT_SCHEMA,
        "baseline source schemas are supported",
        "baseline source schemas are unsupported",
        "studio.issue.shell_export_package_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.candidate_source_schemas",
        candidate.source_manifest_schema == SHELL_HANDOFF_MANIFEST_SCHEMA
            && candidate.source_runbook_schema == SHELL_RUNBOOK_REPORT_SCHEMA,
        "candidate source schemas are supported",
        "candidate source schemas are unsupported",
        "studio.issue.shell_export_package_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.project_id",
        baseline.project_id == candidate.project_id,
        "baseline and candidate project ids match",
        "baseline and candidate project ids differ",
        "studio.issue.shell_export_package_project_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.execution_policy",
        baseline.execution_policy == candidate.execution_policy
            && baseline.execution_policy == "not_executed.review_only",
        "baseline and candidate use review-only execution policy",
        "baseline and candidate execution policies differ or are executable",
        "studio.issue.shell_export_package_execution_policy_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.authority",
        baseline.command_session_authority == candidate.command_session_authority
            && baseline.command_session_authority == "rusty.manifold"
            && baseline.install_launch_evidence_authority
                == candidate.install_launch_evidence_authority
            && baseline.install_launch_evidence_authority == "rusty.hostess"
            && baseline.studio_role == candidate.studio_role
            && baseline.studio_role == "authoring.export_planning",
        "baseline and candidate keep Manifold/Hostess/Studio authority",
        "baseline and candidate authority fields differ or drifted",
        "studio.issue.shell_export_package_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.prohibited_actions",
        string_set(&baseline.prohibited_actions) == string_set(&candidate.prohibited_actions),
        "baseline and candidate prohibited actions match",
        "baseline and candidate prohibited actions differ",
        "studio.issue.shell_export_package_prohibited_actions_mismatch",
    );
    checks
}

fn shell_export_package_baseline_identity_checks(
    baseline_identity: &StudioShellExportPackageBaselineManifest,
    baseline: &StudioShellExportPackageReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_schema",
        baseline_identity.schema_id == SHELL_EXPORT_PACKAGE_BASELINE_MANIFEST_SCHEMA,
        "baseline identity schema id is supported",
        "baseline identity schema id is unsupported",
        "studio.issue.shell_export_package_baseline_identity_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_package_schema",
        baseline_identity.package_schema == baseline.schema_id,
        "baseline identity names the loaded package schema",
        "baseline identity does not name the loaded package schema",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_package",
        baseline_identity.package_id == baseline.package_id
            && baseline_identity.manifest_id == baseline.manifest_id,
        "baseline identity package ids match the loaded package",
        "baseline identity package ids differ from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_project",
        baseline_identity.project_id == baseline.project_id
            && baseline_identity.project_revision == baseline.project_revision,
        "baseline identity project metadata matches the loaded package",
        "baseline identity project metadata differs from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_status_counts",
        baseline_identity.status == baseline.status
            && baseline_identity.ready_count == baseline.ready_count
            && baseline_identity.blocked_count == baseline.blocked_count
            && baseline_identity.rejected_count == baseline.rejected_count
            && baseline_identity.descriptor_count == baseline.descriptor_count
            && baseline_identity.template_manifest_count == baseline.template_manifest_count
            && baseline_identity.runbook_entry_count == baseline.runbook_entry_count
            && baseline_identity.target_count == baseline.target_summaries.len(),
        "baseline identity review counts match the loaded package",
        "baseline identity review counts differ from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_authority",
        baseline_identity.execution_policy == baseline.execution_policy
            && baseline_identity.review_owner == baseline.review_owner
            && baseline_identity.command_session_authority == baseline.command_session_authority
            && baseline_identity.install_launch_evidence_authority
                == baseline.install_launch_evidence_authority
            && baseline_identity.studio_role == baseline.studio_role
            && string_set(&baseline_identity.prohibited_actions)
                == string_set(&baseline.prohibited_actions),
        "baseline identity authority fields match the loaded package",
        "baseline identity authority fields differ from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    checks
}

fn shell_export_package_baseline_index_entry_checks(
    context: &ShellExportPackageBaselineIndexComparisonContext<'_>,
    baseline_identity: &StudioShellExportPackageBaselineManifest,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    let entry = context.entry;
    let expected_manifest_path = context
        .baseline_manifest_path
        .map(|path| path.display().to_string());
    let manifest_path_matches = match (
        expected_manifest_path.as_deref(),
        entry.baseline_manifest_path.as_deref(),
    ) {
        (Some(expected), Some(actual)) => actual == expected,
        (None, Some(actual)) => !actual.trim().is_empty(),
        _ => false,
    };

    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_schema",
        context.index.schema_id == SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA,
        "baseline index schema id is supported",
        "baseline index schema id is unsupported",
        "studio.issue.shell_export_package_baseline_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_selected_baseline",
        entry.baseline_id == baseline_identity.baseline_id,
        "baseline index selected entry matches the loaded baseline identity",
        "baseline index selected entry differs from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_manifest_path",
        manifest_path_matches,
        "baseline index entry records the selected baseline manifest path",
        "baseline index entry is missing or mismatches the selected manifest path",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_package_path",
        entry.package_path == baseline_identity.package_path,
        "baseline index entry package path matches the loaded baseline identity",
        "baseline index entry package path differs from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_package",
        entry.package_schema == baseline_identity.package_schema
            && entry.package_id == baseline_identity.package_id
            && entry.manifest_id == baseline_identity.manifest_id,
        "baseline index entry package ids match the loaded baseline identity",
        "baseline index entry package ids differ from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_project",
        entry.project_id == baseline_identity.project_id
            && entry.project_revision == baseline_identity.project_revision,
        "baseline index entry project metadata matches the loaded baseline identity",
        "baseline index entry project metadata differs from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_status_counts",
        entry.status == baseline_identity.status
            && entry.ready_count == baseline_identity.ready_count
            && entry.blocked_count == baseline_identity.blocked_count
            && entry.rejected_count == baseline_identity.rejected_count
            && entry.descriptor_count == baseline_identity.descriptor_count
            && entry.template_manifest_count == baseline_identity.template_manifest_count
            && entry.runbook_entry_count == baseline_identity.runbook_entry_count
            && entry.target_count == baseline_identity.target_count,
        "baseline index entry review counts match the loaded baseline identity",
        "baseline index entry review counts differ from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    checks
}

fn shell_export_package_comparison_entries(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> Vec<StudioShellExportPackageComparisonEntry> {
    let baseline_entries = baseline
        .entries
        .iter()
        .map(|entry| (entry.graph_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let candidate_entries = candidate
        .entries
        .iter()
        .map(|entry| (entry.graph_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let graph_ids = baseline_entries
        .keys()
        .chain(candidate_entries.keys())
        .map(|graph_id| (*graph_id).to_string())
        .collect::<BTreeSet<_>>();

    graph_ids
        .into_iter()
        .map(|graph_id| {
            shell_export_package_comparison_entry(
                &graph_id,
                baseline_entries.get(graph_id.as_str()).copied(),
                candidate_entries.get(graph_id.as_str()).copied(),
            )
        })
        .collect()
}

fn shell_export_package_comparison_entry(
    graph_id: &str,
    baseline: Option<&StudioShellExportPackageEntry>,
    candidate: Option<&StudioShellExportPackageEntry>,
) -> StudioShellExportPackageComparisonEntry {
    let baseline_score = baseline.map(|entry| export_package_status_score(entry.status));
    let candidate_score = candidate.map(|entry| export_package_status_score(entry.status));
    let score_delta = candidate_score.unwrap_or(0) - baseline_score.unwrap_or(0);
    let change = match (baseline, candidate) {
        (None, Some(_)) => StudioShellExportPackageComparisonChange::Added,
        (Some(_), None) => StudioShellExportPackageComparisonChange::Removed,
        (Some(_), Some(_)) if score_delta > 0 => StudioShellExportPackageComparisonChange::Improved,
        (Some(_), Some(_)) if score_delta < 0 => {
            StudioShellExportPackageComparisonChange::Regressed
        }
        (Some(baseline), Some(candidate))
            if baseline.consumer_id != candidate.consumer_id
                || baseline.issue_code != candidate.issue_code
                || baseline.descriptor.is_some() != candidate.descriptor.is_some()
                || baseline.template_manifest.is_some()
                    != candidate.template_manifest.is_some()
                || baseline.runbook_cli_request.is_empty()
                    != candidate.runbook_cli_request.is_empty() =>
        {
            StudioShellExportPackageComparisonChange::Changed
        }
        (Some(_), Some(_)) => StudioShellExportPackageComparisonChange::Unchanged,
        (None, None) => StudioShellExportPackageComparisonChange::Unchanged,
    };
    let issue_code = match change {
        StudioShellExportPackageComparisonChange::Regressed
        | StudioShellExportPackageComparisonChange::Removed => candidate
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| baseline.and_then(|entry| entry.issue_code.clone()))
            .or_else(|| Some("studio.issue.shell_export_package_regressed".to_string())),
        StudioShellExportPackageComparisonChange::Added
        | StudioShellExportPackageComparisonChange::Improved
        | StudioShellExportPackageComparisonChange::Unchanged
        | StudioShellExportPackageComparisonChange::Changed => None,
    };

    StudioShellExportPackageComparisonEntry {
        graph_id: graph_id.to_string(),
        target_kind: candidate
            .map(|entry| entry.target_kind)
            .or_else(|| baseline.map(|entry| entry.target_kind)),
        baseline_status: baseline.map(|entry| entry.status),
        candidate_status: candidate.map(|entry| entry.status),
        change,
        score_delta,
        baseline_consumer_id: baseline.map(|entry| entry.consumer_id.clone()),
        candidate_consumer_id: candidate.map(|entry| entry.consumer_id.clone()),
        baseline_descriptor_present: baseline
            .map(|entry| entry.descriptor.is_some())
            .unwrap_or(false),
        candidate_descriptor_present: candidate
            .map(|entry| entry.descriptor.is_some())
            .unwrap_or(false),
        baseline_template_manifest_present: baseline
            .map(|entry| entry.template_manifest.is_some())
            .unwrap_or(false),
        candidate_template_manifest_present: candidate
            .map(|entry| entry.template_manifest.is_some())
            .unwrap_or(false),
        baseline_runbook_cli_request_present: baseline
            .map(|entry| !entry.runbook_cli_request.is_empty())
            .unwrap_or(false),
        candidate_runbook_cli_request_present: candidate
            .map(|entry| !entry.runbook_cli_request.is_empty())
            .unwrap_or(false),
        baseline_issue_code: baseline.and_then(|entry| entry.issue_code.clone()),
        candidate_issue_code: candidate.and_then(|entry| entry.issue_code.clone()),
        issue_code,
    }
}

fn export_package_status_score(status: StudioShellExportPackageStatus) -> isize {
    match status {
        StudioShellExportPackageStatus::Rejected => 0,
        StudioShellExportPackageStatus::Blocked => 1,
        StudioShellExportPackageStatus::Ready => 2,
    }
}

pub fn shell_handoff_acceptance_checklist_for_intake(
    intake: &StudioShellHandoffIntakeReport,
) -> StudioShellHandoffAcceptanceChecklistReport {
    let intake_checks = shell_handoff_acceptance_intake_checks(intake);
    let intake_is_accepted = intake_checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass);

    let entries = if intake_is_accepted {
        intake
            .entries
            .iter()
            .map(shell_handoff_acceptance_checklist_entry)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let ready_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Ready)
        .count();
    let blocked_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Blocked)
        .count();
    let rejected_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Rejected)
        .count();
    let status = if !intake_is_accepted || rejected_count > 0 {
        StudioShellHandoffAcceptanceStatus::Rejected
    } else if blocked_count > 0 {
        StudioShellHandoffAcceptanceStatus::Blocked
    } else {
        StudioShellHandoffAcceptanceStatus::Ready
    };
    let issue_code = match status {
        StudioShellHandoffAcceptanceStatus::Ready => None,
        StudioShellHandoffAcceptanceStatus::Blocked => entries
            .iter()
            .find(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Blocked)
            .and_then(|entry| entry.issue_code.clone()),
        StudioShellHandoffAcceptanceStatus::Rejected => intake.issue_code.clone().or_else(|| {
            first_failed_validation_check_issue_code(&intake_checks).or_else(|| {
                entries
                    .iter()
                    .find(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Rejected)
                    .and_then(|entry| entry.issue_code.clone())
            })
        }),
    };

    StudioShellHandoffAcceptanceChecklistReport {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA.to_string(),
        source_intake_schema: intake.schema_id.clone(),
        manifest_id: intake.manifest_id.clone(),
        project_id: intake.project_id.clone(),
        project_revision: intake.project_revision,
        status,
        issue_code,
        prohibited_actions: shell_handoff_acceptance_prohibited_actions(),
        ready_count,
        blocked_count,
        rejected_count,
        intake_checks,
        entries,
    }
}

pub fn shell_handoff_acceptance_checklist_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
    bundle_root: &Path,
) -> StudioShellHandoffAcceptanceChecklistReport {
    let manifest = shell_handoff_manifest_for_project(project, base_dir, bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);
    shell_handoff_acceptance_checklist_for_intake(&intake)
}

pub fn summarize_shell_handoff_acceptance_checklist(
    checklist: &StudioShellHandoffAcceptanceChecklistReport,
    checklist_path: Option<&Path>,
) -> StudioShellHandoffAcceptanceSummaryReport {
    let failed_intake_check_count = checklist
        .intake_checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();

    StudioShellHandoffAcceptanceSummaryReport {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA.to_string(),
        checklist_schema: checklist.schema_id.clone(),
        checklist_path: checklist_path.map(|path| path.display().to_string()),
        manifest_id: checklist.manifest_id.clone(),
        project_id: checklist.project_id.clone(),
        project_revision: checklist.project_revision,
        status: checklist.status,
        issue_code: checklist.issue_code.clone(),
        ready_count: checklist.ready_count,
        blocked_count: checklist.blocked_count,
        rejected_count: checklist.rejected_count,
        entry_count: checklist.entries.len(),
        intake_check_count: checklist.intake_checks.len(),
        failed_intake_check_count,
        prohibited_actions: checklist.prohibited_actions.clone(),
        targets: shell_handoff_acceptance_target_summaries(&checklist.entries),
    }
}

pub fn shell_handoff_acceptance_baseline_manifest_for_checklist(
    checklist: &StudioShellHandoffAcceptanceChecklistReport,
    checklist_path: &Path,
    baseline_id: Option<&str>,
    label: Option<&str>,
) -> StudioShellHandoffAcceptanceBaselineManifest {
    let summary = summarize_shell_handoff_acceptance_checklist(checklist, Some(checklist_path));
    let baseline_id = baseline_id
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_handoff_acceptance_baseline_id(&summary));
    let label = label
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_handoff_acceptance_baseline_label(&summary));

    StudioShellHandoffAcceptanceBaselineManifest {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA.to_string(),
        baseline_id,
        label,
        checklist_path: checklist_path.display().to_string(),
        summary,
    }
}

pub fn shell_handoff_acceptance_baseline_index_for_manifests(
    baselines: Vec<(
        StudioShellHandoffAcceptanceBaselineManifest,
        Option<PathBuf>,
    )>,
    default_baseline_id: Option<&str>,
) -> StudioShellHandoffAcceptanceBaselineIndex {
    let entries = baselines
        .into_iter()
        .map(|(baseline, baseline_manifest_path)| {
            shell_handoff_acceptance_baseline_index_entry_for_manifest(
                baseline,
                baseline_manifest_path,
            )
        })
        .collect::<Vec<_>>();

    shell_handoff_acceptance_baseline_index_for_entries(entries, default_baseline_id)
}

pub fn append_shell_handoff_acceptance_baseline_index_manifests(
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baselines: Vec<(
        StudioShellHandoffAcceptanceBaselineManifest,
        Option<PathBuf>,
    )>,
    default_baseline_id: Option<&str>,
) -> StudioShellHandoffAcceptanceBaselineIndex {
    let entries = index
        .entries
        .iter()
        .cloned()
        .chain(
            baselines
                .into_iter()
                .map(|(baseline, baseline_manifest_path)| {
                    shell_handoff_acceptance_baseline_index_entry_for_manifest(
                        baseline,
                        baseline_manifest_path,
                    )
                }),
        )
        .collect::<Vec<_>>();
    let default_baseline_id = default_baseline_id.or(index.default_baseline_id.as_deref());

    shell_handoff_acceptance_baseline_index_for_entries(entries, default_baseline_id)
}

pub fn promote_shell_handoff_acceptance_baseline_index_default(
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_id: &str,
) -> Option<StudioShellHandoffAcceptanceBaselineIndex> {
    index
        .entries
        .iter()
        .any(|entry| entry.baseline_id == baseline_id)
        .then(|| {
            shell_handoff_acceptance_baseline_index_for_entries(
                index.entries.clone(),
                Some(baseline_id),
            )
        })
}

fn shell_handoff_acceptance_baseline_index_entry_for_manifest(
    baseline: StudioShellHandoffAcceptanceBaselineManifest,
    baseline_manifest_path: Option<PathBuf>,
) -> StudioShellHandoffAcceptanceBaselineIndexEntry {
    let StudioShellHandoffAcceptanceBaselineManifest {
        baseline_id,
        label,
        checklist_path,
        summary,
        ..
    } = baseline;

    StudioShellHandoffAcceptanceBaselineIndexEntry {
        baseline_id,
        label,
        baseline_manifest_path: baseline_manifest_path.map(|path| path.display().to_string()),
        checklist_path,
        summary_schema: summary.schema_id.clone(),
        checklist_schema: summary.checklist_schema.clone(),
        manifest_id: summary.manifest_id.clone(),
        project_id: summary.project_id.clone(),
        project_revision: summary.project_revision,
        status: summary.status,
        issue_code: summary.issue_code.clone(),
        ready_count: summary.ready_count,
        blocked_count: summary.blocked_count,
        rejected_count: summary.rejected_count,
        entry_count: summary.entry_count,
        target_count: summary.targets.len(),
    }
}

fn shell_handoff_acceptance_baseline_index_for_entries(
    entries: Vec<StudioShellHandoffAcceptanceBaselineIndexEntry>,
    default_baseline_id: Option<&str>,
) -> StudioShellHandoffAcceptanceBaselineIndex {
    let mut by_id = BTreeMap::new();
    for entry in entries {
        by_id.insert(entry.baseline_id.clone(), entry);
    }

    let entries = by_id.into_values().collect::<Vec<_>>();
    let default_baseline_id = default_baseline_id
        .filter(|baseline_id| {
            entries
                .iter()
                .any(|entry| entry.baseline_id == *baseline_id)
        })
        .map(str::to_string)
        .or_else(|| entries.first().map(|entry| entry.baseline_id.clone()));

    StudioShellHandoffAcceptanceBaselineIndex {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA.to_string(),
        project_ids: unique_strings(entries.iter().map(|entry| entry.project_id.clone())),
        manifest_ids: unique_strings(entries.iter().map(|entry| entry.manifest_id.clone())),
        default_baseline_id,
        baseline_count: entries.len(),
        ready_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Ready)
            .count(),
        blocked_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Blocked)
            .count(),
        rejected_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Rejected)
            .count(),
        entries,
    }
}

pub fn select_shell_handoff_acceptance_baseline_index_entry<'a>(
    index: &'a StudioShellHandoffAcceptanceBaselineIndex,
    baseline_id: Option<&str>,
) -> Option<&'a StudioShellHandoffAcceptanceBaselineIndexEntry> {
    let selected_id = baseline_id.or(index.default_baseline_id.as_deref());
    selected_id
        .and_then(|selected_id| {
            index
                .entries
                .iter()
                .find(|entry| entry.baseline_id == selected_id)
        })
        .or_else(|| {
            baseline_id
                .is_none()
                .then(|| index.entries.first())
                .flatten()
        })
}

pub fn summarize_shell_handoff_acceptance_baseline_index_selection(
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    index_path: Option<&Path>,
    requested_baseline_id: Option<&str>,
) -> StudioShellHandoffAcceptanceBaselineSelectionReport {
    let selected_entry =
        select_shell_handoff_acceptance_baseline_index_entry(index, requested_baseline_id);
    let selected_baseline_id = selected_entry.map(|entry| entry.baseline_id.clone());
    let status = if index.entries.is_empty() {
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Empty
    } else if selected_entry.is_some() {
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
    } else {
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Missing
    };
    let issue_code = match status {
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected => None,
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Missing => {
            Some("studio.issue.shell_handoff_acceptance_baseline_not_found".to_string())
        }
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Empty => {
            Some("studio.issue.shell_handoff_acceptance_baseline_index_empty".to_string())
        }
    };

    StudioShellHandoffAcceptanceBaselineSelectionReport {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_BASELINE_SELECTION_SCHEMA.to_string(),
        source_index_schema: index.schema_id.clone(),
        index_path: index_path.map(|path| path.display().to_string()),
        requested_baseline_id: requested_baseline_id.map(str::to_string),
        default_baseline_id: index.default_baseline_id.clone(),
        selected_baseline_id: selected_baseline_id.clone(),
        status,
        issue_code,
        baseline_count: index.baseline_count,
        ready_baseline_count: index.ready_baseline_count,
        blocked_baseline_count: index.blocked_baseline_count,
        rejected_baseline_count: index.rejected_baseline_count,
        project_ids: index.project_ids.clone(),
        manifest_ids: index.manifest_ids.clone(),
        entries: index
            .entries
            .iter()
            .map(|entry| StudioShellHandoffAcceptanceBaselineSelectionEntry {
                baseline_id: entry.baseline_id.clone(),
                label: entry.label.clone(),
                selected: selected_baseline_id.as_deref() == Some(entry.baseline_id.as_str()),
                default: index.default_baseline_id.as_deref() == Some(entry.baseline_id.as_str()),
                baseline_manifest_path: entry.baseline_manifest_path.clone(),
                checklist_path: entry.checklist_path.clone(),
                project_id: entry.project_id.clone(),
                project_revision: entry.project_revision,
                status: entry.status,
                issue_code: entry.issue_code.clone(),
                ready_count: entry.ready_count,
                blocked_count: entry.blocked_count,
                rejected_count: entry.rejected_count,
                entry_count: entry.entry_count,
                target_count: entry.target_count,
            })
            .collect(),
    }
}

pub fn compare_shell_handoff_acceptance_checklists(
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> StudioShellHandoffAcceptanceComparisonReport {
    compare_shell_handoff_acceptance_checklists_with_identity(baseline, candidate, None, None)
}

pub fn compare_shell_handoff_acceptance_against_baseline_manifest(
    baseline_identity: &StudioShellHandoffAcceptanceBaselineManifest,
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> StudioShellHandoffAcceptanceComparisonReport {
    compare_shell_handoff_acceptance_checklists_with_identity(
        baseline,
        candidate,
        Some(baseline_identity),
        None,
    )
}

pub fn compare_shell_handoff_acceptance_against_baseline_index_entry(
    baseline_index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_index_path: Option<&Path>,
    baseline_index_entry: &StudioShellHandoffAcceptanceBaselineIndexEntry,
    baseline_manifest_path: Option<&Path>,
    baseline_identity: &StudioShellHandoffAcceptanceBaselineManifest,
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> StudioShellHandoffAcceptanceComparisonReport {
    compare_shell_handoff_acceptance_checklists_with_identity(
        baseline,
        candidate,
        Some(baseline_identity),
        Some(ShellHandoffAcceptanceBaselineIndexComparisonContext {
            index: baseline_index,
            index_path: baseline_index_path,
            entry: baseline_index_entry,
            baseline_manifest_path,
        }),
    )
}

struct ShellHandoffAcceptanceBaselineIndexComparisonContext<'a> {
    index: &'a StudioShellHandoffAcceptanceBaselineIndex,
    index_path: Option<&'a Path>,
    entry: &'a StudioShellHandoffAcceptanceBaselineIndexEntry,
    baseline_manifest_path: Option<&'a Path>,
}

fn compare_shell_handoff_acceptance_checklists_with_identity(
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
    baseline_identity: Option<&StudioShellHandoffAcceptanceBaselineManifest>,
    baseline_index: Option<ShellHandoffAcceptanceBaselineIndexComparisonContext<'_>>,
) -> StudioShellHandoffAcceptanceComparisonReport {
    let mut checks = shell_handoff_acceptance_comparison_checks(baseline, candidate);
    if let Some(baseline_identity) = baseline_identity {
        checks.extend(shell_handoff_acceptance_baseline_identity_checks(
            baseline_identity,
            baseline,
        ));
        if let Some(baseline_index) = baseline_index.as_ref() {
            checks.extend(shell_handoff_acceptance_baseline_index_entry_checks(
                baseline_index,
                baseline_identity,
            ));
        }
    }
    let comparable = checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass);

    let entries = if comparable {
        shell_handoff_acceptance_comparison_entries(baseline, candidate)
    } else {
        Vec::new()
    };

    let ready_delta = count_delta(candidate.ready_count, baseline.ready_count);
    let blocked_delta = count_delta(candidate.blocked_count, baseline.blocked_count);
    let rejected_delta = count_delta(candidate.rejected_count, baseline.rejected_count);

    let status = if !comparable {
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable
    } else if acceptance_status_score(candidate.status) < acceptance_status_score(baseline.status)
        || ready_delta < 0
        || blocked_delta > 0
        || rejected_delta > 0
        || entries
            .iter()
            .any(|entry| entry.change == StudioShellHandoffAcceptanceComparisonChange::Regressed)
    {
        StudioShellHandoffAcceptanceComparisonStatus::Regressed
    } else if acceptance_status_score(candidate.status) > acceptance_status_score(baseline.status)
        || ready_delta > 0
        || blocked_delta < 0
        || rejected_delta < 0
        || entries
            .iter()
            .any(|entry| entry.change == StudioShellHandoffAcceptanceComparisonChange::Improved)
    {
        StudioShellHandoffAcceptanceComparisonStatus::Improved
    } else {
        StudioShellHandoffAcceptanceComparisonStatus::Unchanged
    };

    let issue_code = match status {
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable => {
            first_failed_validation_check_issue_code(&checks)
        }
        StudioShellHandoffAcceptanceComparisonStatus::Regressed => entries
            .iter()
            .find(|entry| entry.change == StudioShellHandoffAcceptanceComparisonChange::Regressed)
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| Some("studio.issue.shell_handoff_acceptance_regressed".to_string())),
        StudioShellHandoffAcceptanceComparisonStatus::Improved
        | StudioShellHandoffAcceptanceComparisonStatus::Unchanged => None,
    };

    StudioShellHandoffAcceptanceComparisonReport {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_COMPARISON_SCHEMA.to_string(),
        baseline_identity_schema: baseline_identity.map(|identity| identity.schema_id.clone()),
        baseline_id: baseline_identity.map(|identity| identity.baseline_id.clone()),
        baseline_label: baseline_identity.map(|identity| identity.label.clone()),
        baseline_checklist_path: baseline_identity.map(|identity| identity.checklist_path.clone()),
        baseline_index_schema: baseline_index
            .as_ref()
            .map(|context| context.index.schema_id.clone()),
        baseline_index_path: baseline_index
            .as_ref()
            .and_then(|context| context.index_path.map(|path| path.display().to_string())),
        baseline_index_default_baseline_id: baseline_index
            .as_ref()
            .and_then(|context| context.index.default_baseline_id.clone()),
        baseline_index_selected_baseline_id: baseline_index
            .as_ref()
            .map(|context| context.entry.baseline_id.clone()),
        baseline_schema: baseline.schema_id.clone(),
        candidate_schema: candidate.schema_id.clone(),
        baseline_manifest_id: baseline.manifest_id.clone(),
        candidate_manifest_id: candidate.manifest_id.clone(),
        baseline_project_id: baseline.project_id.clone(),
        candidate_project_id: candidate.project_id.clone(),
        baseline_project_revision: baseline.project_revision,
        candidate_project_revision: candidate.project_revision,
        baseline_status: baseline.status,
        candidate_status: candidate.status,
        status,
        issue_code,
        baseline_ready_count: baseline.ready_count,
        candidate_ready_count: candidate.ready_count,
        ready_delta,
        baseline_blocked_count: baseline.blocked_count,
        candidate_blocked_count: candidate.blocked_count,
        blocked_delta,
        baseline_rejected_count: baseline.rejected_count,
        candidate_rejected_count: candidate.rejected_count,
        rejected_delta,
        checks,
        entries,
    }
}

pub fn validate_shell_template_index(
    index: &StudioShellTemplateIndex,
    base_dir: Option<&Path>,
) -> StudioShellTemplateIndexValidationReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_template_index.schema",
        index.schema_id == SHELL_TEMPLATE_INDEX_SCHEMA,
        "shell template index schema id is supported",
        "shell template index schema id is unsupported",
        "studio.issue.shell_template_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_template_index.index_id",
        is_dotted_id(&index.index_id),
        "index id uses dotted-id grammar",
        "index id is not a dotted id",
        "studio.issue.invalid_index_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_template_index.manifest_id",
        is_dotted_id(&index.manifest_id),
        "manifest id uses dotted-id grammar",
        "manifest id is not a dotted id",
        "studio.issue.invalid_manifest_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_template_index.project_id",
        is_dotted_id(&index.project_id),
        "project id uses dotted-id grammar",
        "project id is not a dotted id",
        "studio.issue.invalid_project_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_template_index.project_revision",
        index.project_revision > 0,
        "project revision is positive",
        "project revision must be positive",
        "studio.issue.invalid_revision",
    );
    push_check(
        &mut checks,
        "studio.check.shell_template_index.templates_present",
        !index.templates.is_empty(),
        "index declares shell templates",
        "index must declare at least one shell template",
        "studio.issue.no_shell_templates",
    );

    let duplicate_template_ids =
        duplicate_template_field(&index.templates, |entry| entry.template_id.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_template_index.unique_template_ids",
        duplicate_template_ids.is_empty(),
        "template ids are unique",
        &format!(
            "duplicate template ids: {}",
            duplicate_template_ids.join(", ")
        ),
        "studio.issue.duplicate_template_id",
    );
    let duplicate_artifact_ids =
        duplicate_template_field(&index.templates, |entry| entry.artifact_id.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_template_index.unique_artifact_ids",
        duplicate_artifact_ids.is_empty(),
        "artifact ids are unique",
        &format!(
            "duplicate artifact ids: {}",
            duplicate_artifact_ids.join(", ")
        ),
        "studio.issue.duplicate_artifact_id",
    );
    let duplicate_graph_ids =
        duplicate_template_field(&index.templates, |entry| entry.graph_id.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_template_index.unique_graph_ids",
        duplicate_graph_ids.is_empty(),
        "graph ids are unique",
        &format!("duplicate graph ids: {}", duplicate_graph_ids.join(", ")),
        "studio.issue.duplicate_template_graph_id",
    );
    let duplicate_template_paths =
        duplicate_template_field(&index.templates, |entry| entry.template_path.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_template_index.unique_template_paths",
        duplicate_template_paths.is_empty(),
        "template paths are unique",
        &format!(
            "duplicate template paths: {}",
            duplicate_template_paths.join(", ")
        ),
        "studio.issue.duplicate_template_path",
    );
    let duplicate_descriptor_paths =
        duplicate_template_field(&index.templates, |entry| entry.descriptor_path.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_template_index.unique_descriptor_paths",
        duplicate_descriptor_paths.is_empty(),
        "descriptor paths are unique",
        &format!(
            "duplicate descriptor paths: {}",
            duplicate_descriptor_paths.join(", ")
        ),
        "studio.issue.duplicate_descriptor_path",
    );

    for entry in &index.templates {
        validate_shell_template_index_entry(entry, base_dir, &mut checks);
    }

    StudioShellTemplateIndexValidationReport {
        schema_id: SHELL_TEMPLATE_INDEX_VALIDATION_REPORT_SCHEMA,
        index_id: index.index_id.clone(),
        status: if checks
            .iter()
            .any(|check| check.status == StudioValidationStatus::Fail)
        {
            StudioValidationStatus::Fail
        } else {
            StudioValidationStatus::Pass
        },
        checks,
    }
}

#[derive(Default)]
struct ReferenceIndex {
    package_ids: BTreeSet<String>,
    module_ids: BTreeSet<String>,
    package_modules: BTreeMap<String, BTreeSet<String>>,
    package_manifest_paths: BTreeMap<String, String>,
    host_profiles: BTreeMap<String, HostProfileReference>,
}

#[derive(Clone, Debug, Default)]
struct HostProfileReference {
    profile_id: String,
    host_profile: Option<String>,
    app_id: Option<String>,
    install_route: Option<String>,
    launch_route: Option<String>,
    command_bridge: Option<String>,
    evidence_pull_route: Option<String>,
    required_permissions: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CatalogModuleSelection {
    package_id: String,
    module_id: String,
    label: String,
}

fn reference_index_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
) -> Option<ReferenceIndex> {
    let mut checks = Vec::new();
    validate_project_references(project, base_dir, &mut checks)
}

fn catalog_package_views(
    reference_index: Option<&ReferenceIndex>,
    selected_graph: Option<&StudioGraph>,
) -> Vec<StudioCatalogPackageView> {
    let Some(reference_index) = reference_index else {
        return Vec::new();
    };
    let selected_packages = selected_node_reference_ids(selected_graph, StudioNodeKind::Package);
    reference_index
        .package_ids
        .iter()
        .map(|package_id| {
            let module_ids = reference_index
                .package_modules
                .get(package_id)
                .map(|module_ids| module_ids.iter().cloned().collect::<Vec<_>>())
                .unwrap_or_default();
            StudioCatalogPackageView {
                package_id: package_id.clone(),
                manifest_path: reference_index
                    .package_manifest_paths
                    .get(package_id)
                    .cloned()
                    .unwrap_or_default(),
                module_count: module_ids.len(),
                module_ids,
                in_selected_graph: selected_packages.contains(package_id),
            }
        })
        .collect()
}

fn host_profile_views(
    reference_index: Option<&ReferenceIndex>,
    selected_graph: Option<&StudioGraph>,
) -> Vec<StudioHostProfileView> {
    let Some(reference_index) = reference_index else {
        return Vec::new();
    };
    reference_index
        .host_profiles
        .values()
        .map(|reference| StudioHostProfileView {
            profile_id: reference.profile_id.clone(),
            host_profile: reference.host_profile.clone(),
            app_id: reference.app_id.clone(),
            install_route: reference.install_route.clone(),
            launch_route: reference.launch_route.clone(),
            command_bridge: reference.command_bridge.clone(),
            evidence_pull_route: reference.evidence_pull_route.clone(),
            required_permissions: reference.required_permissions.clone(),
            targets_selected_graph: selected_graph.is_some_and(|graph| {
                graph.target_host_profile.as_str() == reference.profile_id.as_str()
            }),
        })
        .collect()
}

fn selected_node_reference_ids(
    selected_graph: Option<&StudioGraph>,
    kind: StudioNodeKind,
) -> BTreeSet<String> {
    selected_graph
        .into_iter()
        .flat_map(|graph| graph.nodes.iter())
        .filter(|node| node.kind == kind)
        .map(|node| node.reference_id.clone())
        .collect()
}

struct SelectedNodeSelection {
    selected_node: Option<StudioNodeInspectorView>,
    selected_node_id: Option<String>,
    node_selection_code: Option<String>,
}

fn selected_node_selection(
    selected_graph: Option<&StudioGraph>,
    selected_graph_view: Option<&StudioGraphView>,
    reference_index: Option<&ReferenceIndex>,
    focused_issue: Option<&StudioIssueFocusView>,
    requested_node_id: Option<&str>,
) -> SelectedNodeSelection {
    let Some(graph) = selected_graph else {
        return SelectedNodeSelection {
            selected_node: None,
            selected_node_id: None,
            node_selection_code: requested_node_id
                .map(|_| "studio.issue.node_selection_missing".to_string()),
        };
    };
    let fallback_node_id = focused_issue
        .filter(|focus| focus.graph_id == graph.graph_id)
        .and_then(|focus| focus.node_id.as_deref())
        .filter(|node_id| graph.nodes.iter().any(|node| node.node_id == *node_id))
        .or_else(|| graph.nodes.first().map(|node| node.node_id.as_str()));
    let (selected_node_id, node_selection_code) = if let Some(requested_node_id) = requested_node_id
    {
        if graph
            .nodes
            .iter()
            .any(|node| node.node_id == requested_node_id)
        {
            (Some(requested_node_id), None)
        } else {
            (
                fallback_node_id,
                Some("studio.issue.node_selection_missing".to_string()),
            )
        }
    } else {
        (fallback_node_id, None)
    };
    let selected_node = selected_node_id
        .and_then(|node_id| graph.nodes.iter().find(|node| node.node_id == node_id))
        .map(|node| node_inspector_view(graph, selected_graph_view, reference_index, node));
    SelectedNodeSelection {
        selected_node_id: selected_node.as_ref().map(|node| node.node_id.clone()),
        selected_node,
        node_selection_code,
    }
}

fn node_inspector_view(
    graph: &StudioGraph,
    graph_view: Option<&StudioGraphView>,
    reference_index: Option<&ReferenceIndex>,
    node: &StudioNode,
) -> StudioNodeInspectorView {
    let validation_issue_count = graph_view
        .and_then(|graph| {
            graph
                .node_rows
                .iter()
                .find(|row| row.node_id == node.node_id)
        })
        .map(|row| row.validation_issue_count)
        .unwrap_or(0);
    let package_module_ids = package_module_ids(reference_index, &node.reference_id);
    let module_package_ids = module_package_ids(reference_index, &node.reference_id);
    let package_manifest_path =
        package_manifest_path(reference_index, node.kind, &node.reference_id);
    let host_profile = host_profile_inspector(reference_index, node.kind, &node.reference_id);
    StudioNodeInspectorView {
        graph_id: graph.graph_id.clone(),
        node_id: node.node_id.clone(),
        kind: node_kind_label(node.kind).to_string(),
        reference_id: node.reference_id.clone(),
        label: node.label.clone(),
        validation_issue_count,
        reference_status: node_reference_status(reference_index, node.kind, &node.reference_id)
            .to_string(),
        package_manifest_path,
        package_module_ids,
        module_package_ids,
        host_profile,
    }
}

fn node_reference_status(
    reference_index: Option<&ReferenceIndex>,
    kind: StudioNodeKind,
    reference_id: &str,
) -> &'static str {
    let Some(reference_index) = reference_index else {
        return match kind {
            StudioNodeKind::OperatorShell | StudioNodeKind::ValidationSlot => "authored",
            _ => "reference_index_unavailable",
        };
    };
    match kind {
        StudioNodeKind::Package => {
            if reference_index.package_ids.contains(reference_id) {
                "resolved"
            } else {
                "missing"
            }
        }
        StudioNodeKind::Module => {
            if reference_index.module_ids.contains(reference_id) {
                "resolved"
            } else {
                "missing"
            }
        }
        StudioNodeKind::HostProfile => {
            if reference_index.host_profiles.contains_key(reference_id) {
                "resolved"
            } else {
                "missing"
            }
        }
        StudioNodeKind::OperatorShell | StudioNodeKind::ValidationSlot => "authored",
    }
}

fn package_manifest_path(
    reference_index: Option<&ReferenceIndex>,
    kind: StudioNodeKind,
    reference_id: &str,
) -> Option<String> {
    if kind != StudioNodeKind::Package {
        return None;
    }
    reference_index
        .and_then(|index| index.package_manifest_paths.get(reference_id))
        .cloned()
}

fn package_module_ids(reference_index: Option<&ReferenceIndex>, package_id: &str) -> Vec<String> {
    reference_index
        .and_then(|index| index.package_modules.get(package_id))
        .map(|modules| modules.iter().cloned().collect())
        .unwrap_or_default()
}

fn module_package_ids(reference_index: Option<&ReferenceIndex>, module_id: &str) -> Vec<String> {
    reference_index
        .map(|index| {
            index
                .package_modules
                .iter()
                .filter(|(_, module_ids)| module_ids.contains(module_id))
                .map(|(package_id, _)| package_id.clone())
                .collect()
        })
        .unwrap_or_default()
}

fn host_profile_inspector(
    reference_index: Option<&ReferenceIndex>,
    kind: StudioNodeKind,
    reference_id: &str,
) -> Option<StudioNodeHostProfileView> {
    if kind != StudioNodeKind::HostProfile {
        return None;
    }
    let reference = reference_index?.host_profiles.get(reference_id)?;
    Some(StudioNodeHostProfileView {
        profile_id: reference.profile_id.clone(),
        host_profile: reference.host_profile.clone(),
        app_id: reference.app_id.clone(),
        install_route: reference.install_route.clone(),
        launch_route: reference.launch_route.clone(),
        command_bridge: reference.command_bridge.clone(),
        evidence_pull_route: reference.evidence_pull_route.clone(),
        required_permissions: reference.required_permissions.clone(),
    })
}

struct SelectedEdgeSelection {
    selected_edge: Option<StudioEdgeInspectorView>,
    selected_edge_id: Option<String>,
    edge_selection_code: Option<String>,
}

fn selected_edge_selection(
    selected_graph: Option<&StudioGraph>,
    selected_graph_view: Option<&StudioGraphView>,
    focused_issue: Option<&StudioIssueFocusView>,
    requested_edge_id: Option<&str>,
) -> SelectedEdgeSelection {
    let Some(graph) = selected_graph else {
        return SelectedEdgeSelection {
            selected_edge: None,
            selected_edge_id: None,
            edge_selection_code: requested_edge_id
                .map(|_| "studio.issue.edge_selection_missing".to_string()),
        };
    };
    let fallback_edge_id = focused_issue
        .filter(|focus| focus.graph_id == graph.graph_id)
        .and_then(|focus| focus.edge_id.as_deref())
        .filter(|edge_id| graph.edges.iter().any(|edge| edge.edge_id == *edge_id))
        .or_else(|| graph.edges.first().map(|edge| edge.edge_id.as_str()));
    let (selected_edge_id, edge_selection_code) = if let Some(requested_edge_id) = requested_edge_id
    {
        if graph
            .edges
            .iter()
            .any(|edge| edge.edge_id == requested_edge_id)
        {
            (Some(requested_edge_id), None)
        } else {
            (
                fallback_edge_id,
                Some("studio.issue.edge_selection_missing".to_string()),
            )
        }
    } else {
        (fallback_edge_id, None)
    };
    let selected_edge = selected_edge_id
        .and_then(|edge_id| graph.edges.iter().find(|edge| edge.edge_id == edge_id))
        .map(|edge| edge_inspector_view(graph, selected_graph_view, edge));
    SelectedEdgeSelection {
        selected_edge_id: selected_edge.as_ref().map(|edge| edge.edge_id.clone()),
        selected_edge,
        edge_selection_code,
    }
}

fn edge_inspector_view(
    graph: &StudioGraph,
    graph_view: Option<&StudioGraphView>,
    edge: &StudioEdge,
) -> StudioEdgeInspectorView {
    let source = graph
        .nodes
        .iter()
        .find(|node| node.node_id == edge.source_node_id);
    let target = graph
        .nodes
        .iter()
        .find(|node| node.node_id == edge.target_node_id);
    let validation_issue_count = graph_view
        .and_then(|graph| {
            graph
                .edge_rows
                .iter()
                .find(|row| row.edge_id == edge.edge_id)
        })
        .map(|row| row.validation_issue_count)
        .unwrap_or(0);
    StudioEdgeInspectorView {
        graph_id: graph.graph_id.clone(),
        edge_id: edge.edge_id.clone(),
        kind: edge_kind_label(edge.kind).to_string(),
        source_node_id: edge.source_node_id.clone(),
        source_label: source.map(|node| node.label.clone()),
        source_kind: source.map(|node| node_kind_label(node.kind).to_string()),
        source_reference_id: source.map(|node| node.reference_id.clone()),
        target_node_id: edge.target_node_id.clone(),
        target_label: target.map(|node| node.label.clone()),
        target_kind: target.map(|node| node_kind_label(node.kind).to_string()),
        target_reference_id: target.map(|node| node.reference_id.clone()),
        validation_issue_count,
        endpoint_status: edge_endpoint_status(source, target).to_string(),
        binding_kind: binding_kind_for_edge(edge.kind).map(binding_kind_short_label),
    }
}

fn edge_endpoint_status(source: Option<&StudioNode>, target: Option<&StudioNode>) -> &'static str {
    match (source.is_some(), target.is_some()) {
        (true, true) => "endpoints_resolved",
        (false, true) => "missing_source",
        (true, false) => "missing_target",
        (false, false) => "missing_endpoints",
    }
}

fn binding_kind_short_label(kind: StudioBindingKind) -> String {
    match kind {
        StudioBindingKind::Stream => "stream".to_string(),
        StudioBindingKind::Command => "command".to_string(),
    }
}

fn shell_preview_for_selected_graph(
    project: &StudioProject,
    base_dir: Option<&Path>,
    selected_graph_id: Option<&str>,
) -> Option<StudioShellPreviewView> {
    let graph_id = selected_graph_id?;
    let report = shell_descriptor_for_graph(project, base_dir, graph_id);
    let status = report.status;
    let issue_code = report.issue_code.clone();
    let message = report.message.clone();
    let Some(descriptor) = report.descriptor.as_ref() else {
        return Some(StudioShellPreviewView {
            graph_id: report.graph_id,
            status,
            issue_code,
            message,
            descriptor_id: None,
            descriptor_path: None,
            shell_id: None,
            shell_label: None,
            target_host_profile: None,
            target_kind: None,
            host_profile_class: None,
            app_id: None,
            install_route: None,
            launch_route: None,
            command_bridge: None,
            evidence_pull_route: None,
            package_count: 0,
            module_count: 0,
            validation_slot_count: 0,
            stream_binding_count: 0,
            command_binding_count: 0,
            descriptor_validation_status: None,
            template_id: None,
            template_path: None,
            template_descriptor_path: None,
            runtime_command_authority: None,
            runtime_host_authority: None,
            studio_role: None,
        });
    };

    let descriptor_validation = validate_shell_descriptor(descriptor);
    let artifact = shell_artifact_for_descriptor(descriptor);
    let template = shell_template_for_artifact(&artifact);
    let template_entry = shell_template_index_entry(&artifact);
    Some(StudioShellPreviewView {
        graph_id: descriptor.graph_id.clone(),
        status,
        issue_code,
        message,
        descriptor_id: Some(descriptor.descriptor_id.clone()),
        descriptor_path: Some(artifact.descriptor_path.clone()),
        shell_id: Some(descriptor.shell_id.clone()),
        shell_label: Some(descriptor.shell_label.clone()),
        target_host_profile: Some(descriptor.target_host_profile.clone()),
        target_kind: Some(artifact.target_kind),
        host_profile_class: artifact.host_profile_class.clone(),
        app_id: artifact.app_id.clone(),
        install_route: artifact.install_route.clone(),
        launch_route: artifact.launch_route.clone(),
        command_bridge: artifact.command_bridge.clone(),
        evidence_pull_route: artifact.evidence_pull_route.clone(),
        package_count: descriptor.package_ids.len(),
        module_count: descriptor.module_ids.len(),
        validation_slot_count: descriptor.validation_slot_ids.len(),
        stream_binding_count: descriptor.stream_bindings.len(),
        command_binding_count: descriptor.command_bindings.len(),
        descriptor_validation_status: Some(descriptor_validation.status),
        template_id: Some(template.template_id),
        template_path: Some(template_entry.template_path),
        template_descriptor_path: Some(template_entry.descriptor_path),
        runtime_command_authority: Some(template.runtime_authority.command_session_authority),
        runtime_host_authority: Some(template.runtime_authority.install_launch_evidence_authority),
        studio_role: Some(template.runtime_authority.studio_role),
    })
}

fn next_available_catalog_module(
    graph: &StudioGraph,
    reference_index: &ReferenceIndex,
) -> Option<CatalogModuleSelection> {
    for package_id in &reference_index.package_ids {
        if let Some(selection) =
            next_available_catalog_module_for_package(graph, reference_index, package_id)
        {
            return Some(selection);
        }
    }
    None
}

fn next_available_catalog_module_for_package(
    graph: &StudioGraph,
    reference_index: &ReferenceIndex,
    package_id: &str,
) -> Option<CatalogModuleSelection> {
    let module_ids = reference_index.package_modules.get(package_id)?;
    let selected_modules = selected_node_reference_ids(Some(graph), StudioNodeKind::Module);
    let mut candidates = module_ids
        .iter()
        .filter(|module_id| !selected_modules.contains(*module_id))
        .cloned()
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        palette_module_rank(left)
            .cmp(&palette_module_rank(right))
            .then_with(|| left.cmp(right))
    });
    candidates
        .into_iter()
        .next()
        .map(|module_id| CatalogModuleSelection {
            package_id: package_id.to_string(),
            label: label_for_reference(&module_id),
            module_id,
        })
}

fn palette_module_rank(module_id: &str) -> u8 {
    if module_id.ends_with(".provider") || module_id.ends_with("_provider") {
        0
    } else {
        1
    }
}

fn shell_host_profile(
    profile_id: &str,
    reference_index: Option<&ReferenceIndex>,
) -> StudioShellHostProfile {
    let reference = reference_index.and_then(|index| index.host_profiles.get(profile_id));
    StudioShellHostProfile {
        profile_id: reference
            .map(|reference| reference.profile_id.clone())
            .unwrap_or_else(|| profile_id.to_string()),
        host_profile: reference.and_then(|reference| reference.host_profile.clone()),
        app_id: reference.and_then(|reference| reference.app_id.clone()),
        install_route: reference.and_then(|reference| reference.install_route.clone()),
        launch_route: reference.and_then(|reference| reference.launch_route.clone()),
        command_bridge: reference.and_then(|reference| reference.command_bridge.clone()),
        evidence_pull_route: reference.and_then(|reference| reference.evidence_pull_route.clone()),
        required_permissions: reference
            .map(|reference| reference.required_permissions.clone())
            .unwrap_or_default(),
    }
}

fn shell_binding(edge: &StudioEdge) -> StudioShellBinding {
    StudioShellBinding {
        binding_id: edge.edge_id.clone(),
        source_node_id: edge.source_node_id.clone(),
        target_node_id: edge.target_node_id.clone(),
    }
}

fn shell_descriptor_report(
    project: &StudioProject,
    graph_id: &str,
    status: StudioShellDescriptorStatus,
    issue_code: Option<String>,
    message: String,
    validation: StudioValidationReport,
    descriptor: Option<StudioShellDescriptor>,
) -> StudioShellDescriptorReport {
    StudioShellDescriptorReport {
        schema_id: SHELL_DESCRIPTOR_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        status,
        issue_code,
        message,
        graph_id: graph_id.to_string(),
        validation,
        descriptor,
    }
}

fn validate_shell_bindings(
    field: &str,
    bindings: &[StudioShellBinding],
    checks: &mut Vec<StudioValidationCheck>,
) {
    let duplicate_ids = duplicate_binding_ids(bindings);
    push_check(
        checks,
        &format!("studio.check.shell_descriptor.{field}.unique_ids"),
        duplicate_ids.is_empty(),
        "binding ids are unique",
        &format!("duplicate binding ids: {}", duplicate_ids.join(", ")),
        "studio.issue.duplicate_binding_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_descriptor.{field}.ids"),
        bindings
            .iter()
            .all(|binding| is_dotted_id(&binding.binding_id)),
        "binding ids use dotted-id grammar",
        "one or more binding ids are not dotted ids",
        "studio.issue.invalid_binding_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_descriptor.{field}.source_nodes"),
        bindings
            .iter()
            .all(|binding| is_dotted_id(&binding.source_node_id)),
        "binding source node ids use dotted-id grammar",
        "one or more binding source node ids are not dotted ids",
        "studio.issue.invalid_binding_source",
    );
    push_check(
        checks,
        &format!("studio.check.shell_descriptor.{field}.target_nodes"),
        bindings
            .iter()
            .all(|binding| is_dotted_id(&binding.target_node_id)),
        "binding target node ids use dotted-id grammar",
        "one or more binding target node ids are not dotted ids",
        "studio.issue.invalid_binding_target",
    );
}

fn duplicate_binding_ids(bindings: &[StudioShellBinding]) -> Vec<String> {
    let mut counts = BTreeMap::new();
    for binding in bindings {
        *counts.entry(binding.binding_id.clone()).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .filter_map(|(id, count)| (count > 1).then_some(id))
        .collect()
}

fn shell_artifact_for_descriptor(descriptor: &StudioShellDescriptor) -> StudioShellArtifact {
    StudioShellArtifact {
        artifact_id: format!("studio.shell_artifact.{}", descriptor.graph_id),
        graph_id: descriptor.graph_id.clone(),
        shell_id: descriptor.shell_id.clone(),
        target_kind: shell_target_kind(descriptor.host_profile.host_profile.as_deref()),
        target_host_profile: descriptor.target_host_profile.clone(),
        host_profile_class: descriptor.host_profile.host_profile.clone(),
        descriptor_path: shell_descriptor_artifact_path(&descriptor.graph_id),
        app_id: descriptor.host_profile.app_id.clone(),
        install_route: descriptor.host_profile.install_route.clone(),
        launch_route: descriptor.host_profile.launch_route.clone(),
        command_bridge: descriptor.host_profile.command_bridge.clone(),
        evidence_pull_route: descriptor.host_profile.evidence_pull_route.clone(),
        package_ids: descriptor.package_ids.clone(),
        module_ids: descriptor.module_ids.clone(),
    }
}

pub fn shell_descriptor_artifact_path(graph_id: &str) -> String {
    format!("descriptors/{graph_id}.shell-descriptor.json")
}

fn shell_manifold_handoff_artifact_path(graph_id: &str) -> String {
    format!("descriptors/{graph_id}.manifold-shell-handoff.json")
}

fn manifold_shell_handoff_for_descriptor(
    descriptor: &StudioShellDescriptor,
) -> StudioGeneratedManifoldShellHandoffManifest {
    StudioGeneratedManifoldShellHandoffManifest {
        schema_id: MANIFOLD_SHELL_HANDOFF_SCHEMA,
        handoff_id: format!("shell_handoff.{}", descriptor.graph_id),
        handoff_revision: descriptor.project_revision,
        target_host_profile: descriptor
            .host_profile
            .host_profile
            .clone()
            .unwrap_or_else(|| descriptor.target_host_profile.clone()),
        shell_app_id: descriptor
            .host_profile
            .app_id
            .clone()
            .unwrap_or_else(|| descriptor.shell_id.clone()),
        validation_slot_id: descriptor
            .validation_slot_ids
            .first()
            .cloned()
            .unwrap_or_else(|| DEFAULT_MANIFOLD_SHELL_HANDOFF_VALIDATION_SLOT_ID.to_string()),
        stream_bindings: descriptor
            .stream_bindings
            .iter()
            .filter(|binding| binding.binding_id.starts_with("stream."))
            .map(manifold_shell_stream_binding)
            .collect(),
        command_ids: descriptor
            .command_bindings
            .iter()
            .filter_map(|binding| {
                binding
                    .binding_id
                    .starts_with("command.")
                    .then(|| binding.binding_id.clone())
            })
            .collect(),
        transport_offers: vec![StudioGeneratedManifoldTransportOffer {
            transport_id: format!("transport.shell_handoff.{}", descriptor.graph_id),
            transport: manifold_transport_for_command_bridge(
                descriptor.host_profile.command_bridge.as_deref(),
            ),
            endpoint_id: None,
        }],
        expected_scorecard_id: format!("scorecard.shell_handoff.{}", descriptor.graph_id),
    }
}

fn manifold_shell_stream_binding(
    binding: &StudioShellBinding,
) -> StudioGeneratedManifoldShellStreamBinding {
    let shell_is_source = binding.source_node_id.contains(".shell.");
    let direction = if shell_is_source {
        StudioGeneratedManifoldShellStreamDirection::Publish
    } else {
        StudioGeneratedManifoldShellStreamDirection::Subscribe
    };
    let role = match direction {
        StudioGeneratedManifoldShellStreamDirection::Publish => "role.shell.publish",
        StudioGeneratedManifoldShellStreamDirection::Subscribe => "role.shell.subscribe",
    };
    StudioGeneratedManifoldShellStreamBinding {
        stream_id: binding.binding_id.clone(),
        direction,
        role: role.to_string(),
        required: true,
    }
}

fn manifold_transport_for_command_bridge(
    command_bridge: Option<&str>,
) -> StudioGeneratedManifoldEndpointTransport {
    match command_bridge {
        Some(bridge) if bridge.contains("http") => StudioGeneratedManifoldEndpointTransport::Http,
        Some(bridge) if bridge.contains("stdio") || bridge.contains("cli") => {
            StudioGeneratedManifoldEndpointTransport::Stdio
        }
        _ => StudioGeneratedManifoldEndpointTransport::InProcess,
    }
}

pub fn shell_template_manifest_path(artifact: &StudioShellArtifact) -> String {
    format!(
        "shells/{}/{}.shell-template.json",
        shell_target_kind_path(artifact.target_kind),
        artifact.graph_id
    )
}

pub fn shell_template_descriptor_path(graph_id: &str) -> String {
    format!("descriptors/{graph_id}.shell-descriptor.json")
}

fn shell_target_kind(host_profile_class: Option<&str>) -> StudioShellTargetKind {
    match host_profile_class {
        Some("host.desktop") => StudioShellTargetKind::Desktop,
        Some("host.mobile") => StudioShellTargetKind::Phone,
        Some("host.headset") | Some("host.quest") => StudioShellTargetKind::Quest,
        _ => StudioShellTargetKind::Unknown,
    }
}

fn shell_target_kind_path(target_kind: StudioShellTargetKind) -> &'static str {
    match target_kind {
        StudioShellTargetKind::Desktop => "desktop",
        StudioShellTargetKind::Phone => "phone",
        StudioShellTargetKind::Quest => "quest",
        StudioShellTargetKind::Unknown => "unknown",
    }
}

fn shell_artifact_report(
    project: &StudioProject,
    status: StudioShellArtifactStatus,
    issue_code: Option<String>,
    message: String,
    validation: StudioValidationReport,
    manifest: Option<StudioShellArtifactManifest>,
    descriptors: Vec<StudioShellDescriptor>,
    rejections: Vec<StudioShellArtifactRejection>,
) -> StudioShellArtifactReport {
    StudioShellArtifactReport {
        schema_id: SHELL_ARTIFACT_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        status,
        issue_code,
        message,
        validation,
        manifest,
        descriptors,
        rejections,
    }
}

fn validate_shell_artifact_manifest_entry(
    artifact: &StudioShellArtifact,
    base_dir: Option<&Path>,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = artifact.artifact_id.clone();
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.id"),
        is_dotted_id(&artifact.artifact_id),
        "artifact id uses dotted-id grammar",
        "artifact id is not a dotted id",
        "studio.issue.invalid_artifact_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.graph_id"),
        is_dotted_id(&artifact.graph_id),
        "artifact graph id uses dotted-id grammar",
        "artifact graph id is not a dotted id",
        "studio.issue.invalid_graph_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.shell_id"),
        is_dotted_id(&artifact.shell_id),
        "artifact shell id uses dotted-id grammar",
        "artifact shell id is not a dotted id",
        "studio.issue.invalid_shell_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.target_host_profile"),
        is_dotted_id(&artifact.target_host_profile),
        "target host profile uses dotted-id grammar",
        "target host profile is not a dotted id",
        "studio.issue.invalid_target_host_profile",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.host_profile_class"),
        optional_dotted_id(artifact.host_profile_class.as_deref()),
        "host profile class is absent or uses dotted-id grammar",
        "host profile class is not a dotted id",
        "studio.issue.invalid_host_profile_class",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.app_id"),
        optional_dotted_id(artifact.app_id.as_deref()),
        "app id is absent or uses dotted-id grammar",
        "app id is not a dotted id",
        "studio.issue.invalid_app_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.install_route"),
        optional_dotted_id(artifact.install_route.as_deref()),
        "install route is absent or uses dotted-id grammar",
        "install route is not a dotted id",
        "studio.issue.invalid_install_route",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.launch_route"),
        optional_dotted_id(artifact.launch_route.as_deref()),
        "launch route is absent or uses dotted-id grammar",
        "launch route is not a dotted id",
        "studio.issue.invalid_launch_route",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.command_bridge"),
        optional_dotted_id(artifact.command_bridge.as_deref()),
        "command bridge is absent or uses dotted-id grammar",
        "command bridge is not a dotted id",
        "studio.issue.invalid_command_bridge",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.evidence_pull_route"),
        optional_dotted_id(artifact.evidence_pull_route.as_deref()),
        "evidence pull route is absent or uses dotted-id grammar",
        "evidence pull route is not a dotted id",
        "studio.issue.invalid_evidence_pull_route",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.package_ids"),
        all_dotted_ids(&artifact.package_ids),
        "package ids use dotted-id grammar",
        "one or more package ids are not dotted ids",
        "studio.issue.invalid_package_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.module_ids"),
        all_dotted_ids(&artifact.module_ids),
        "module ids use dotted-id grammar",
        "one or more module ids are not dotted ids",
        "studio.issue.invalid_module_id",
    );

    let descriptor_path_is_safe = is_safe_relative_manifest_path(&artifact.descriptor_path);
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_path"),
        descriptor_path_is_safe,
        "descriptor path is a safe relative path",
        "descriptor path must be a portable relative path without traversal",
        "studio.issue.invalid_descriptor_path",
    );

    if let Some(base_dir) = base_dir.filter(|_| descriptor_path_is_safe) {
        validate_shell_artifact_descriptor_reference(artifact, base_dir, checks);
    }
}

fn validate_shell_artifact_descriptor_reference(
    artifact: &StudioShellArtifact,
    base_dir: &Path,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = artifact.artifact_id.clone();
    let descriptor_path = resolve_manifest_relative_path(base_dir, &artifact.descriptor_path);
    let descriptor_exists = descriptor_path.is_file();
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_exists"),
        descriptor_exists,
        "descriptor path resolves to a file",
        "descriptor path does not resolve to a file",
        "studio.issue.descriptor_missing",
    );
    if !descriptor_exists {
        return;
    }

    let descriptor = match load_shell_descriptor(&descriptor_path) {
        Ok(descriptor) => {
            push_check(
                checks,
                &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_parse"),
                true,
                "descriptor JSON parsed",
                "descriptor JSON did not parse",
                "studio.issue.descriptor_parse_failed",
            );
            descriptor
        }
        Err(error) => {
            checks.push(StudioValidationCheck {
                check_id: format!(
                    "studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_parse"
                ),
                status: StudioValidationStatus::Fail,
                evidence: error.to_string(),
                issue_code: Some("studio.issue.descriptor_parse_failed".to_string()),
                graph_id: Some(artifact.graph_id.clone()),
                node_ids: Vec::new(),
                edge_ids: Vec::new(),
                reference_ids: Vec::new(),
            });
            return;
        }
    };

    let descriptor_validation = validate_shell_descriptor(&descriptor);
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_validation"),
        descriptor_validation.status == StudioValidationStatus::Pass,
        "descriptor validation passed",
        "descriptor validation failed",
        "studio.issue.descriptor_validation_failed",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_graph"),
        descriptor.graph_id == artifact.graph_id,
        "descriptor graph id matches artifact graph id",
        "descriptor graph id does not match artifact graph id",
        "studio.issue.descriptor_graph_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_shell"),
        descriptor.shell_id == artifact.shell_id,
        "descriptor shell id matches artifact shell id",
        "descriptor shell id does not match artifact shell id",
        "studio.issue.descriptor_shell_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_target"),
        descriptor.target_host_profile == artifact.target_host_profile,
        "descriptor target host profile matches artifact target host profile",
        "descriptor target host profile does not match artifact target host profile",
        "studio.issue.descriptor_target_mismatch",
    );
    push_check(
        checks,
        &format!(
            "studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_host_profile_class"
        ),
        descriptor.host_profile.host_profile == artifact.host_profile_class,
        "descriptor host profile class matches artifact host profile class",
        "descriptor host profile class does not match artifact host profile class",
        "studio.issue.descriptor_host_profile_class_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_target_kind"),
        shell_target_kind(descriptor.host_profile.host_profile.as_deref()) == artifact.target_kind,
        "descriptor target kind matches artifact target kind",
        "descriptor target kind does not match artifact target kind",
        "studio.issue.descriptor_target_kind_mismatch",
    );
}

fn validate_shell_template_index_entry(
    entry: &StudioShellTemplateIndexEntry,
    base_dir: Option<&Path>,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = entry.template_id.clone();
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.template_id"),
        is_dotted_id(&entry.template_id),
        "template id uses dotted-id grammar",
        "template id is not a dotted id",
        "studio.issue.invalid_template_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.artifact_id"),
        is_dotted_id(&entry.artifact_id),
        "artifact id uses dotted-id grammar",
        "artifact id is not a dotted id",
        "studio.issue.invalid_artifact_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.graph_id"),
        is_dotted_id(&entry.graph_id),
        "graph id uses dotted-id grammar",
        "graph id is not a dotted id",
        "studio.issue.invalid_graph_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.shell_id"),
        is_dotted_id(&entry.shell_id),
        "shell id uses dotted-id grammar",
        "shell id is not a dotted id",
        "studio.issue.invalid_shell_id",
    );
    let template_path_is_safe = is_safe_relative_manifest_path(&entry.template_path);
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.template_path"),
        template_path_is_safe,
        "template path is a safe relative path",
        "template path must be a portable relative path without traversal",
        "studio.issue.invalid_template_path",
    );
    let descriptor_path_is_safe = is_safe_relative_manifest_path(&entry.descriptor_path);
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_path"),
        descriptor_path_is_safe,
        "descriptor path is a safe relative path",
        "descriptor path must be a portable relative path without traversal",
        "studio.issue.invalid_descriptor_path",
    );

    if let Some(base_dir) = base_dir.filter(|_| template_path_is_safe && descriptor_path_is_safe) {
        validate_shell_template_files(entry, base_dir, checks);
    }
}

fn validate_shell_template_files(
    entry: &StudioShellTemplateIndexEntry,
    base_dir: &Path,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = entry.template_id.clone();
    let template_path = resolve_manifest_relative_path(base_dir, &entry.template_path);
    let template_exists = template_path.is_file();
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.template_exists"),
        template_exists,
        "template path resolves to a file",
        "template path does not resolve to a file",
        "studio.issue.template_missing",
    );
    if !template_exists {
        return;
    }

    let template = match load_shell_template_manifest(&template_path) {
        Ok(template) => {
            push_check(
                checks,
                &format!("studio.check.shell_template_index.template.{prefix}.template_parse"),
                true,
                "template JSON parsed",
                "template JSON did not parse",
                "studio.issue.template_parse_failed",
            );
            template
        }
        Err(error) => {
            checks.push(StudioValidationCheck {
                check_id: format!(
                    "studio.check.shell_template_index.template.{prefix}.template_parse"
                ),
                status: StudioValidationStatus::Fail,
                evidence: error.to_string(),
                issue_code: Some("studio.issue.template_parse_failed".to_string()),
                graph_id: Some(entry.graph_id.clone()),
                node_ids: Vec::new(),
                edge_ids: Vec::new(),
                reference_ids: Vec::new(),
            });
            return;
        }
    };

    validate_shell_template_manifest_reference(entry, &template, checks);

    let descriptor_path = resolve_manifest_relative_path(base_dir, &entry.descriptor_path);
    let descriptor_exists = descriptor_path.is_file();
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_exists"),
        descriptor_exists,
        "descriptor path resolves to a file",
        "descriptor path does not resolve to a file",
        "studio.issue.descriptor_missing",
    );
    if !descriptor_exists {
        return;
    }
    let descriptor = match load_shell_descriptor(&descriptor_path) {
        Ok(descriptor) => {
            push_check(
                checks,
                &format!("studio.check.shell_template_index.template.{prefix}.descriptor_parse"),
                true,
                "descriptor JSON parsed",
                "descriptor JSON did not parse",
                "studio.issue.descriptor_parse_failed",
            );
            descriptor
        }
        Err(error) => {
            checks.push(StudioValidationCheck {
                check_id: format!(
                    "studio.check.shell_template_index.template.{prefix}.descriptor_parse"
                ),
                status: StudioValidationStatus::Fail,
                evidence: error.to_string(),
                issue_code: Some("studio.issue.descriptor_parse_failed".to_string()),
                graph_id: Some(entry.graph_id.clone()),
                node_ids: Vec::new(),
                edge_ids: Vec::new(),
                reference_ids: Vec::new(),
            });
            return;
        }
    };

    let descriptor_validation = validate_shell_descriptor(&descriptor);
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_validation"),
        descriptor_validation.status == StudioValidationStatus::Pass,
        "descriptor validation passed",
        "descriptor validation failed",
        "studio.issue.descriptor_validation_failed",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_graph"),
        descriptor.graph_id == entry.graph_id && descriptor.graph_id == template.graph_id,
        "descriptor graph id matches template index and manifest",
        "descriptor graph id does not match template index and manifest",
        "studio.issue.descriptor_graph_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_shell"),
        descriptor.shell_id == entry.shell_id && descriptor.shell_id == template.shell_id,
        "descriptor shell id matches template index and manifest",
        "descriptor shell id does not match template index and manifest",
        "studio.issue.descriptor_shell_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_target"),
        descriptor.target_host_profile == template.target_host_profile,
        "descriptor target host profile matches template manifest",
        "descriptor target host profile does not match template manifest",
        "studio.issue.descriptor_target_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_target_kind"),
        shell_target_kind(descriptor.host_profile.host_profile.as_deref()) == entry.target_kind
            && entry.target_kind == template.target_kind,
        "descriptor target kind matches template index and manifest",
        "descriptor target kind does not match template index and manifest",
        "studio.issue.descriptor_target_kind_mismatch",
    );
}

fn validate_shell_template_manifest_reference(
    entry: &StudioShellTemplateIndexEntry,
    template: &StudioShellTemplateManifest,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = entry.template_id.clone();
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.template_schema"),
        template.schema_id == SHELL_TEMPLATE_MANIFEST_SCHEMA,
        "template manifest schema id is supported",
        "template manifest schema id is unsupported",
        "studio.issue.shell_template_manifest_schema",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.template_id_matches"),
        template.template_id == entry.template_id,
        "template id matches index entry",
        "template id does not match index entry",
        "studio.issue.template_id_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.artifact_id_matches"),
        template.artifact_id == entry.artifact_id,
        "artifact id matches index entry",
        "artifact id does not match index entry",
        "studio.issue.artifact_id_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.graph_id_matches"),
        template.graph_id == entry.graph_id,
        "graph id matches index entry",
        "graph id does not match index entry",
        "studio.issue.template_graph_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.shell_id_matches"),
        template.shell_id == entry.shell_id,
        "shell id matches index entry",
        "shell id does not match index entry",
        "studio.issue.template_shell_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.target_kind_matches"),
        template.target_kind == entry.target_kind,
        "target kind matches index entry",
        "target kind does not match index entry",
        "studio.issue.template_target_kind_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_path_matches"),
        template.descriptor_path == entry.descriptor_path,
        "descriptor path matches index entry",
        "descriptor path does not match index entry",
        "studio.issue.template_descriptor_path_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.target_host_profile"),
        is_dotted_id(&template.target_host_profile),
        "target host profile uses dotted-id grammar",
        "target host profile is not a dotted id",
        "studio.issue.invalid_target_host_profile",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.host_profile_class"),
        optional_dotted_id(template.host_profile_class.as_deref()),
        "host profile class is absent or uses dotted-id grammar",
        "host profile class is not a dotted id",
        "studio.issue.invalid_host_profile_class",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.source_descriptor_path"),
        is_safe_relative_manifest_path(&template.source_descriptor_path),
        "source descriptor path is a safe relative path",
        "source descriptor path must be a portable relative path without traversal",
        "studio.issue.invalid_source_descriptor_path",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.runtime_command_authority"),
        template.runtime_authority.command_session_authority == "rusty.manifold",
        "Manifold owns command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.runtime_host_authority"),
        template.runtime_authority.install_launch_evidence_authority == "rusty.hostess",
        "Hostess owns install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.studio_role"),
        template.runtime_authority.studio_role == "authoring.export_planning",
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.host_routes"),
        optional_dotted_id(template.host_routes.app_id.as_deref())
            && optional_dotted_id(template.host_routes.install_route.as_deref())
            && optional_dotted_id(template.host_routes.launch_route.as_deref())
            && optional_dotted_id(template.host_routes.command_bridge.as_deref())
            && optional_dotted_id(template.host_routes.evidence_pull_route.as_deref()),
        "host routes are absent or use dotted-id grammar",
        "one or more host routes are not dotted ids",
        "studio.issue.invalid_host_route",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.package_ids"),
        all_dotted_ids(&template.package_ids),
        "package ids use dotted-id grammar",
        "one or more package ids are not dotted ids",
        "studio.issue.invalid_package_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.module_ids"),
        all_dotted_ids(&template.module_ids),
        "module ids use dotted-id grammar",
        "one or more module ids are not dotted ids",
        "studio.issue.invalid_module_id",
    );
}

fn shell_template_for_artifact(artifact: &StudioShellArtifact) -> StudioShellTemplateManifest {
    StudioShellTemplateManifest {
        schema_id: SHELL_TEMPLATE_MANIFEST_SCHEMA.to_string(),
        template_id: shell_template_id(&artifact.graph_id),
        artifact_id: artifact.artifact_id.clone(),
        graph_id: artifact.graph_id.clone(),
        shell_id: artifact.shell_id.clone(),
        target_kind: artifact.target_kind,
        target_host_profile: artifact.target_host_profile.clone(),
        host_profile_class: artifact.host_profile_class.clone(),
        source_descriptor_path: artifact.descriptor_path.clone(),
        descriptor_path: shell_template_descriptor_path(&artifact.graph_id),
        runtime_authority: shell_runtime_authority(),
        host_routes: StudioShellHostRoutes {
            app_id: artifact.app_id.clone(),
            install_route: artifact.install_route.clone(),
            launch_route: artifact.launch_route.clone(),
            command_bridge: artifact.command_bridge.clone(),
            evidence_pull_route: artifact.evidence_pull_route.clone(),
        },
        package_ids: artifact.package_ids.clone(),
        module_ids: artifact.module_ids.clone(),
    }
}

fn shell_template_index_entry(artifact: &StudioShellArtifact) -> StudioShellTemplateIndexEntry {
    StudioShellTemplateIndexEntry {
        template_id: shell_template_id(&artifact.graph_id),
        artifact_id: artifact.artifact_id.clone(),
        graph_id: artifact.graph_id.clone(),
        shell_id: artifact.shell_id.clone(),
        target_kind: artifact.target_kind,
        template_path: shell_template_manifest_path(artifact),
        descriptor_path: shell_template_descriptor_path(&artifact.graph_id),
    }
}

fn shell_template_id(graph_id: &str) -> String {
    format!("studio.shell_template.{graph_id}")
}

fn shell_handoff_manifest_id(project_id: &str) -> String {
    format!("studio.shell_handoffs.{project_id}")
}

fn shell_runtime_authority() -> StudioShellRuntimeAuthority {
    StudioShellRuntimeAuthority {
        command_session_authority: "rusty.manifold".to_string(),
        install_launch_evidence_authority: "rusty.hostess".to_string(),
        studio_role: "authoring.export_planning".to_string(),
    }
}

fn shell_template_report(
    manifest: &StudioShellArtifactManifest,
    status: StudioShellTemplateStatus,
    issue_code: Option<String>,
    message: String,
    validation: StudioShellArtifactManifestValidationReport,
    index: Option<StudioShellTemplateIndex>,
    templates: Vec<StudioShellTemplateManifest>,
) -> StudioShellTemplateReport {
    StudioShellTemplateReport {
        schema_id: SHELL_TEMPLATE_REPORT_SCHEMA,
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        status,
        issue_code,
        message,
        validation,
        index,
        templates,
    }
}

fn shell_bundle_report(
    project: &StudioProject,
    graph_id: &str,
    status: StudioShellBundleStatus,
    issue_code: Option<String>,
    message: String,
    bundle_files: Vec<String>,
    descriptor_validation: Option<StudioShellDescriptorValidationReport>,
    artifact_validation: Option<StudioShellArtifactManifestValidationReport>,
    template_validation: Option<StudioShellTemplateIndexValidationReport>,
    descriptor: Option<StudioShellDescriptor>,
    artifact_manifest: Option<StudioShellArtifactManifest>,
    template_index: Option<StudioShellTemplateIndex>,
    template_manifest: Option<StudioShellTemplateManifest>,
) -> StudioShellBundleReport {
    StudioShellBundleReport {
        schema_id: SHELL_BUNDLE_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        graph_id: graph_id.to_string(),
        status,
        issue_code,
        message,
        bundle_files,
        descriptor_validation,
        artifact_validation,
        template_validation,
        descriptor,
        artifact_manifest,
        template_index,
        template_manifest,
    }
}

fn shell_bundle_validation_report(
    project: &StudioProject,
    graph_id: &str,
    expected_bundle_files: Vec<String>,
    checks: Vec<StudioValidationCheck>,
) -> StudioShellBundleValidationReport {
    StudioShellBundleValidationReport {
        schema_id: SHELL_BUNDLE_VALIDATION_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        graph_id: graph_id.to_string(),
        status: if checks
            .iter()
            .any(|check| check.status == StudioValidationStatus::Fail)
        {
            StudioValidationStatus::Fail
        } else {
            StudioValidationStatus::Pass
        },
        expected_bundle_files,
        checks,
    }
}

fn shell_handoff_report(
    project: &StudioProject,
    graph_id: &str,
    status: StudioValidationStatus,
    issue_code: Option<String>,
    message: String,
    bundle_dir: &Path,
    descriptor_path: String,
    artifact_manifest_path: String,
    template_index_path: String,
    template_manifest_path: String,
    consumer_args: Vec<String>,
    target_kind: StudioShellTargetKind,
    runtime_authority: Option<StudioShellRuntimeAuthority>,
    validation: StudioShellBundleValidationReport,
) -> StudioShellHandoffReport {
    StudioShellHandoffReport {
        schema_id: SHELL_HANDOFF_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        graph_id: graph_id.to_string(),
        status,
        issue_code,
        message,
        handoff_kind: shell_handoff_kind_for_target(target_kind),
        consumer_id: shell_handoff_consumer_id(target_kind).to_string(),
        target_kind,
        bundle_dir: bundle_dir.display().to_string(),
        descriptor_path,
        artifact_manifest_path,
        template_index_path,
        template_manifest_path,
        consumer_args,
        runtime_authority,
        validation,
    }
}

fn shell_handoff_readiness_entry(
    graph: &StudioGraph,
    export_bundle: &StudioExportBundle,
    handoff: StudioShellHandoffReport,
    intended_target_kind: StudioShellTargetKind,
) -> StudioShellHandoffReadinessEntry {
    let failed_check_count = handoff
        .validation
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let package_count = export_bundle.package_ids.len();
    let module_count = export_bundle.module_ids.len();
    let operator_shell_count = export_bundle.operator_shell_ids.len();
    let uses_intended_target = handoff.target_kind == StudioShellTargetKind::Unknown
        && handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing");
    let target_kind = if uses_intended_target {
        intended_target_kind
    } else {
        handoff.target_kind
    };
    let handoff_kind = if uses_intended_target {
        shell_handoff_kind_for_target(target_kind)
    } else {
        handoff.handoff_kind
    };
    let consumer_id = if uses_intended_target {
        shell_handoff_consumer_id(target_kind).to_string()
    } else {
        handoff.consumer_id
    };
    StudioShellHandoffReadinessEntry {
        export_bundle_id: export_bundle.bundle_id.clone(),
        graph_id: graph.graph_id.clone(),
        display_name: graph.display_name.clone(),
        target_host_profile: export_bundle.target_host_profile.clone(),
        target_kind,
        package_ids: export_bundle.package_ids.clone(),
        module_ids: export_bundle.module_ids.clone(),
        operator_shell_ids: export_bundle.operator_shell_ids.clone(),
        package_count,
        module_count,
        operator_shell_count,
        status: handoff.status,
        issue_code: handoff.issue_code,
        message: handoff.message,
        handoff_kind,
        consumer_id,
        bundle_dir: handoff.bundle_dir,
        template_index_path: handoff.template_index_path,
        consumer_args: handoff.consumer_args,
        runtime_authority: handoff.runtime_authority,
        validation_status: handoff.validation.status,
        failed_check_count,
    }
}

fn shell_handoff_readiness_target_summaries(
    entries: &[StudioShellHandoffReadinessEntry],
) -> Vec<StudioShellHandoffReadinessTargetSummary> {
    [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
        StudioShellTargetKind::Unknown,
    ]
    .iter()
    .filter_map(|target_kind| shell_handoff_readiness_target_summary(entries, *target_kind))
    .collect()
}

fn shell_handoff_readiness_target_summary(
    entries: &[StudioShellHandoffReadinessEntry],
    target_kind: StudioShellTargetKind,
) -> Option<StudioShellHandoffReadinessTargetSummary> {
    let mut graph_count = 0;
    let mut ready_count = 0;
    let mut failed_count = 0;
    let mut missing_bundle_count = 0;
    let mut package_count = 0;
    let mut module_count = 0;
    let mut operator_shell_count = 0;
    let mut graph_ids = Vec::new();
    let mut consumer_ids = Vec::new();
    let mut issue_codes = Vec::new();
    let mut bundle_dirs = Vec::new();
    let mut ready_bundle_dirs = Vec::new();
    let mut failed_bundle_dirs = Vec::new();
    let mut missing_bundle_dirs = Vec::new();
    let mut template_index_paths = Vec::new();

    for entry in entries
        .iter()
        .filter(|entry| entry.target_kind == target_kind)
    {
        graph_count += 1;
        if entry.status == StudioValidationStatus::Pass {
            ready_count += 1;
        }
        if entry.status == StudioValidationStatus::Fail {
            failed_count += 1;
        }
        if entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing") {
            missing_bundle_count += 1;
        }
        package_count += entry.package_count;
        module_count += entry.module_count;
        operator_shell_count += entry.operator_shell_count;
        graph_ids.push(entry.graph_id.clone());
        if !bundle_dirs.contains(&entry.bundle_dir) {
            bundle_dirs.push(entry.bundle_dir.clone());
        }
        if !template_index_paths.contains(&entry.template_index_path) {
            template_index_paths.push(entry.template_index_path.clone());
        }
        if entry.status == StudioValidationStatus::Pass
            && !ready_bundle_dirs.contains(&entry.bundle_dir)
        {
            ready_bundle_dirs.push(entry.bundle_dir.clone());
        }
        if entry.status == StudioValidationStatus::Fail
            && !failed_bundle_dirs.contains(&entry.bundle_dir)
        {
            failed_bundle_dirs.push(entry.bundle_dir.clone());
        }
        if entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            && !missing_bundle_dirs.contains(&entry.bundle_dir)
        {
            missing_bundle_dirs.push(entry.bundle_dir.clone());
        }
        if !consumer_ids.contains(&entry.consumer_id) {
            consumer_ids.push(entry.consumer_id.clone());
        }
        if let Some(issue_code) = entry.issue_code.as_ref() {
            if !issue_codes.contains(issue_code) {
                issue_codes.push(issue_code.clone());
            }
        }
    }

    (graph_count > 0).then(|| StudioShellHandoffReadinessTargetSummary {
        target_kind,
        graph_count,
        ready_count,
        failed_count,
        missing_bundle_count,
        package_count,
        module_count,
        operator_shell_count,
        graph_ids,
        consumer_ids,
        issue_codes,
        bundle_dirs,
        ready_bundle_dirs,
        failed_bundle_dirs,
        missing_bundle_dirs,
        template_index_paths,
    })
}

fn shell_handoff_manifest_from_readiness(
    readiness: &StudioShellHandoffReadinessReport,
) -> StudioShellHandoffManifest {
    StudioShellHandoffManifest {
        schema_id: SHELL_HANDOFF_MANIFEST_SCHEMA.to_string(),
        manifest_id: shell_handoff_manifest_id(&readiness.project_id),
        project_id: readiness.project_id.clone(),
        project_revision: readiness.revision,
        source_readiness_schema: readiness.schema_id.to_string(),
        bundle_root: readiness.bundle_root.clone(),
        status: readiness.status,
        graph_count: readiness.graph_count,
        ready_count: readiness.ready_count,
        failed_count: readiness.failed_count,
        missing_bundle_count: readiness.missing_bundle_count,
        runtime_authority: shell_runtime_authority(),
        targets: readiness
            .target_summaries
            .iter()
            .map(shell_handoff_manifest_target)
            .collect(),
        handoffs: readiness
            .entries
            .iter()
            .map(shell_handoff_manifest_entry)
            .collect(),
    }
}

fn shell_handoff_manifest_target(
    summary: &StudioShellHandoffReadinessTargetSummary,
) -> StudioShellHandoffManifestTarget {
    StudioShellHandoffManifestTarget {
        target_kind: summary.target_kind,
        graph_count: summary.graph_count,
        ready_count: summary.ready_count,
        failed_count: summary.failed_count,
        missing_bundle_count: summary.missing_bundle_count,
        package_count: summary.package_count,
        module_count: summary.module_count,
        operator_shell_count: summary.operator_shell_count,
        graph_ids: summary.graph_ids.clone(),
        consumer_ids: summary.consumer_ids.clone(),
        issue_codes: summary.issue_codes.clone(),
        bundle_dirs: summary.bundle_dirs.clone(),
        ready_bundle_dirs: summary.ready_bundle_dirs.clone(),
        failed_bundle_dirs: summary.failed_bundle_dirs.clone(),
        missing_bundle_dirs: summary.missing_bundle_dirs.clone(),
        template_index_paths: summary.template_index_paths.clone(),
    }
}

fn shell_handoff_manifest_entry(
    entry: &StudioShellHandoffReadinessEntry,
) -> StudioShellHandoffManifestEntry {
    StudioShellHandoffManifestEntry {
        export_bundle_id: entry.export_bundle_id.clone(),
        graph_id: entry.graph_id.clone(),
        display_name: entry.display_name.clone(),
        target_host_profile: entry.target_host_profile.clone(),
        target_kind: entry.target_kind,
        status: entry.status,
        issue_code: entry.issue_code.clone(),
        message: entry.message.clone(),
        handoff_kind: entry.handoff_kind,
        consumer_id: entry.consumer_id.clone(),
        bundle_dir: entry.bundle_dir.clone(),
        template_index_path: entry.template_index_path.clone(),
        consumer_args: entry.consumer_args.clone(),
        runtime_authority: entry.runtime_authority.clone(),
        package_ids: entry.package_ids.clone(),
        module_ids: entry.module_ids.clone(),
        operator_shell_ids: entry.operator_shell_ids.clone(),
        validation_status: entry.validation_status,
        failed_check_count: entry.failed_check_count,
    }
}

fn shell_handoff_intake_entry(
    handoff: &StudioShellHandoffManifestEntry,
    authority: &StudioShellRuntimeAuthority,
) -> StudioShellHandoffIntakeEntry {
    let decision = if handoff.status == StudioValidationStatus::Pass {
        StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner
    } else {
        StudioShellHandoffIntakeDecision::BlockedByHandoffIssue
    };
    StudioShellHandoffIntakeEntry {
        export_bundle_id: handoff.export_bundle_id.clone(),
        graph_id: handoff.graph_id.clone(),
        display_name: handoff.display_name.clone(),
        target_host_profile: handoff.target_host_profile.clone(),
        target_kind: handoff.target_kind,
        handoff_kind: handoff.handoff_kind,
        consumer_id: handoff.consumer_id.clone(),
        handoff_status: handoff.status,
        issue_code: handoff.issue_code.clone(),
        decision,
        handoff_request_kind: "operator_shell_handoff".to_string(),
        runtime_route_kind: format!(
            "{}_operator_shell",
            shell_target_kind_label(handoff.target_kind)
        ),
        next_required_action: shell_handoff_intake_next_action(decision).to_string(),
        bundle_dir: handoff.bundle_dir.clone(),
        template_index_path: handoff.template_index_path.clone(),
        consumer_args: handoff.consumer_args.clone(),
        command_session_authority: authority.command_session_authority.clone(),
        install_launch_evidence_authority: authority.install_launch_evidence_authority.clone(),
        studio_role: authority.studio_role.clone(),
        package_ids: handoff.package_ids.clone(),
        module_ids: handoff.module_ids.clone(),
        operator_shell_ids: handoff.operator_shell_ids.clone(),
    }
}

fn shell_handoff_intake_next_action(decision: StudioShellHandoffIntakeDecision) -> &'static str {
    match decision {
        StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner => "stage_with_runtime_owner",
        StudioShellHandoffIntakeDecision::BlockedByManifestIssue => "repair_handoff_manifest",
        StudioShellHandoffIntakeDecision::BlockedByHandoffIssue => "repair_export_bundle",
    }
}

fn shell_handoff_intake_target_summaries(
    entries: &[StudioShellHandoffIntakeEntry],
) -> Vec<StudioShellHandoffIntakeTargetSummary> {
    shell_target_kinds()
        .iter()
        .filter_map(|target_kind| shell_handoff_intake_target_summary(entries, *target_kind))
        .collect()
}

fn shell_handoff_intake_target_summary(
    entries: &[StudioShellHandoffIntakeEntry],
    target_kind: StudioShellTargetKind,
) -> Option<StudioShellHandoffIntakeTargetSummary> {
    let target_entries = entries
        .iter()
        .filter(|entry| entry.target_kind == target_kind)
        .collect::<Vec<_>>();
    if target_entries.is_empty() {
        return None;
    }

    let accepted_count = target_entries
        .iter()
        .filter(|entry| entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner)
        .count();
    let blocked_count = target_entries.len() - accepted_count;
    Some(StudioShellHandoffIntakeTargetSummary {
        target_kind,
        accepted_count,
        blocked_count,
        graph_ids: unique_strings(target_entries.iter().map(|entry| entry.graph_id.clone())),
        consumer_ids: unique_strings(target_entries.iter().map(|entry| entry.consumer_id.clone())),
        bundle_dirs: unique_strings(target_entries.iter().map(|entry| entry.bundle_dir.clone())),
        template_index_paths: unique_strings(
            target_entries
                .iter()
                .map(|entry| entry.template_index_path.clone()),
        ),
    })
}

fn shell_handoff_acceptance_target_summaries(
    entries: &[StudioShellHandoffAcceptanceChecklistEntry],
) -> Vec<StudioShellHandoffAcceptanceTargetSummary> {
    shell_target_kinds()
        .iter()
        .filter_map(|target_kind| shell_handoff_acceptance_target_summary(entries, *target_kind))
        .collect()
}

fn default_shell_handoff_acceptance_baseline_id(
    summary: &StudioShellHandoffAcceptanceSummaryReport,
) -> String {
    format!(
        "{}.rev{}.{}",
        summary.project_id,
        summary.project_revision,
        shell_handoff_acceptance_status_key(summary.status)
    )
}

fn default_shell_handoff_acceptance_baseline_label(
    summary: &StudioShellHandoffAcceptanceSummaryReport,
) -> String {
    format!(
        "{} revision {} {} acceptance baseline",
        summary.project_id,
        summary.project_revision,
        shell_handoff_acceptance_status_key(summary.status)
    )
}

fn shell_handoff_acceptance_status_key(status: StudioShellHandoffAcceptanceStatus) -> &'static str {
    match status {
        StudioShellHandoffAcceptanceStatus::Ready => "ready",
        StudioShellHandoffAcceptanceStatus::Blocked => "blocked",
        StudioShellHandoffAcceptanceStatus::Rejected => "rejected",
    }
}

fn shell_handoff_acceptance_target_summary(
    entries: &[StudioShellHandoffAcceptanceChecklistEntry],
    target_kind: StudioShellTargetKind,
) -> Option<StudioShellHandoffAcceptanceTargetSummary> {
    let target_entries = entries
        .iter()
        .filter(|entry| entry.target_kind == target_kind)
        .collect::<Vec<_>>();
    if target_entries.is_empty() {
        return None;
    }

    Some(StudioShellHandoffAcceptanceTargetSummary {
        target_kind,
        graph_count: target_entries.len(),
        ready_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Ready)
            .count(),
        blocked_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Blocked)
            .count(),
        rejected_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Rejected)
            .count(),
        graph_ids: unique_strings(target_entries.iter().map(|entry| entry.graph_id.clone())),
        consumer_ids: unique_strings(target_entries.iter().map(|entry| entry.consumer_id.clone())),
        route_kinds: unique_strings(
            target_entries
                .iter()
                .map(|entry| entry.runtime_route_kind.clone()),
        ),
        issue_codes: unique_strings(
            target_entries
                .iter()
                .filter_map(|entry| entry.issue_code.clone()),
        ),
    })
}

fn shell_handoff_acceptance_intake_checks(
    intake: &StudioShellHandoffIntakeReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance.intake_schema",
        intake.schema_id == SHELL_HANDOFF_INTAKE_REPORT_SCHEMA,
        "source intake schema id is supported",
        "source intake schema id is unsupported",
        "studio.issue.shell_handoff_intake_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance.intake_status",
        intake.status == StudioShellHandoffIntakeStatus::Accepted,
        "source intake was accepted",
        "source intake was rejected",
        "studio.issue.shell_handoff_intake_rejected",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance.intake_validation",
        intake.validation.status == StudioValidationStatus::Pass,
        "source intake validation passed",
        "source intake validation failed",
        "studio.issue.shell_handoff_intake_validation_failed",
    );
    let authority = StudioShellRuntimeAuthority {
        command_session_authority: intake.command_session_authority.clone(),
        install_launch_evidence_authority: intake.install_launch_evidence_authority.clone(),
        studio_role: intake.studio_role.clone(),
    };
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance.authority",
        runtime_authority_matches(&authority),
        "source intake preserves Manifold/Hostess/Studio authority boundaries",
        "source intake authority does not preserve Manifold/Hostess/Studio boundaries",
        "studio.issue.runtime_authority_mismatch",
    );
    let accepted_count = intake
        .entries
        .iter()
        .filter(|entry| entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner)
        .count();
    let blocked_count = intake.entries.len().saturating_sub(accepted_count);
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance.counts",
        intake.accepted_count == accepted_count
            && intake.blocked_count == blocked_count
            && intake.entries.len() == intake.accepted_count + intake.blocked_count,
        "source intake counts match entry decisions",
        "source intake counts do not match entry decisions",
        "studio.issue.shell_handoff_intake_count_mismatch",
    );
    checks
}

fn shell_handoff_acceptance_checklist_entry(
    entry: &StudioShellHandoffIntakeEntry,
) -> StudioShellHandoffAcceptanceChecklistEntry {
    let checks = shell_handoff_acceptance_entry_checks(entry);
    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let status = if entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner
        && !has_failed_check
    {
        StudioShellHandoffAcceptanceStatus::Ready
    } else if entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner {
        StudioShellHandoffAcceptanceStatus::Rejected
    } else {
        StudioShellHandoffAcceptanceStatus::Blocked
    };
    let issue_code = match status {
        StudioShellHandoffAcceptanceStatus::Ready => None,
        StudioShellHandoffAcceptanceStatus::Blocked => entry
            .issue_code
            .clone()
            .or_else(|| first_failed_acceptance_check_issue_code(&checks)),
        StudioShellHandoffAcceptanceStatus::Rejected => {
            first_failed_acceptance_check_issue_code(&checks)
        }
    };

    StudioShellHandoffAcceptanceChecklistEntry {
        graph_id: entry.graph_id.clone(),
        target_kind: entry.target_kind,
        consumer_id: entry.consumer_id.clone(),
        runtime_route_kind: entry.runtime_route_kind.clone(),
        source_decision: entry.decision,
        status,
        issue_code,
        next_required_action: entry.next_required_action.clone(),
        command_session_authority: entry.command_session_authority.clone(),
        install_launch_evidence_authority: entry.install_launch_evidence_authority.clone(),
        studio_role: entry.studio_role.clone(),
        checks,
    }
}

fn shell_handoff_acceptance_entry_checks(
    entry: &StudioShellHandoffIntakeEntry,
) -> Vec<StudioShellHandoffAcceptanceCheck> {
    let mut checks = Vec::new();
    let prefix = if entry.graph_id.is_empty() {
        "unknown".to_string()
    } else {
        entry.graph_id.clone()
    };
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.decision"),
        "rusty.studio",
        entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner,
        "intake entry is ready for runtime owner staging",
        "intake entry is blocked before runtime owner staging",
        "studio.issue.shell_handoff_acceptance_blocked",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.command_session_authority"),
        "rusty.manifold",
        entry.command_session_authority == "rusty.manifold",
        "Manifold remains command/session authority",
        "command/session authority is not Manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_acceptance_check(
        &mut checks,
        &format!(
            "studio.check.shell_handoff_acceptance.entry.{prefix}.install_launch_evidence_authority"
        ),
        "rusty.hostess",
        entry.install_launch_evidence_authority == "rusty.hostess",
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority is not Hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.studio_role"),
        "rusty.studio",
        entry.studio_role == "authoring.export_planning",
        "Studio role remains authoring/export planning",
        "Studio role exceeds authoring/export planning",
        "studio.issue.runtime_authority_mismatch",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.route_kind"),
        "rusty.manifold",
        entry.runtime_route_kind
            == format!(
                "{}_operator_shell",
                shell_target_kind_label(entry.target_kind)
            ),
        "runtime route kind matches target kind",
        "runtime route kind does not match target kind",
        "studio.issue.shell_handoff_route_mismatch",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.bundle_dir"),
        "rusty.hostess",
        !entry.bundle_dir.trim().is_empty(),
        "bundle dir is available for downstream staging",
        "bundle dir is missing",
        "studio.issue.handoff_path_missing",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.template_index_path"),
        "rusty.hostess",
        path_ends_with_shell_templates(&entry.template_index_path),
        "template index path points to shell-templates.json",
        "template index path does not point to shell-templates.json",
        "studio.issue.handoff_template_index_path_mismatch",
    );
    let consumer_args_ready = entry.consumer_args.iter().any(|arg| arg == "--templates")
        && entry
            .consumer_args
            .iter()
            .any(|arg| arg == &entry.template_index_path);
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.consumer_args"),
        "rusty.manifold",
        consumer_args_ready,
        "consumer args identify the template index",
        "consumer args do not identify the template index",
        "studio.issue.handoff_consumer_args_mismatch",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.operator_shell"),
        "rusty.studio",
        !entry.operator_shell_ids.is_empty(),
        "operator shell ids are present",
        "operator shell ids are missing",
        "studio.issue.no_operator_shell",
    );
    checks
}

fn push_acceptance_check(
    checks: &mut Vec<StudioShellHandoffAcceptanceCheck>,
    check_id: &str,
    owner: &str,
    passed: bool,
    pass_evidence: &str,
    fail_evidence: &str,
    issue_code: &str,
) {
    checks.push(StudioShellHandoffAcceptanceCheck {
        check_id: check_id.to_string(),
        owner: owner.to_string(),
        status: if passed {
            StudioValidationStatus::Pass
        } else {
            StudioValidationStatus::Fail
        },
        evidence: if passed { pass_evidence } else { fail_evidence }.to_string(),
        issue_code: (!passed).then(|| issue_code.to_string()),
    });
}

fn shell_handoff_acceptance_prohibited_actions() -> Vec<String> {
    [
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
    ]
    .iter()
    .map(|action| action.to_string())
    .collect()
}

fn first_failed_acceptance_check_issue_code(
    checks: &[StudioShellHandoffAcceptanceCheck],
) -> Option<String> {
    checks
        .iter()
        .find(|check| check.status == StudioValidationStatus::Fail)
        .and_then(|check| check.issue_code.clone())
}

fn first_failed_validation_check_issue_code(checks: &[StudioValidationCheck]) -> Option<String> {
    checks
        .iter()
        .find(|check| check.status == StudioValidationStatus::Fail)
        .and_then(|check| check.issue_code.clone())
}

fn shell_handoff_acceptance_comparison_checks(
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_schema",
        baseline.schema_id == SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA,
        "baseline checklist schema id is supported",
        "baseline checklist schema id is unsupported",
        "studio.issue.shell_handoff_acceptance_checklist_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.candidate_schema",
        candidate.schema_id == SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA,
        "candidate checklist schema id is supported",
        "candidate checklist schema id is unsupported",
        "studio.issue.shell_handoff_acceptance_checklist_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_source_schema",
        baseline.source_intake_schema == SHELL_HANDOFF_INTAKE_REPORT_SCHEMA,
        "baseline source intake schema id is supported",
        "baseline source intake schema id is unsupported",
        "studio.issue.shell_handoff_intake_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.candidate_source_schema",
        candidate.source_intake_schema == SHELL_HANDOFF_INTAKE_REPORT_SCHEMA,
        "candidate source intake schema id is supported",
        "candidate source intake schema id is unsupported",
        "studio.issue.shell_handoff_intake_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.project_id",
        baseline.project_id == candidate.project_id,
        "baseline and candidate project ids match",
        "baseline and candidate project ids differ",
        "studio.issue.shell_handoff_acceptance_project_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.prohibited_actions",
        string_set(&baseline.prohibited_actions) == string_set(&candidate.prohibited_actions),
        "baseline and candidate prohibited actions match",
        "baseline and candidate prohibited actions differ",
        "studio.issue.shell_handoff_acceptance_prohibited_actions_mismatch",
    );
    checks
}

fn shell_handoff_acceptance_baseline_identity_checks(
    baseline_identity: &StudioShellHandoffAcceptanceBaselineManifest,
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_schema",
        baseline_identity.schema_id == SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA,
        "baseline identity schema id is supported",
        "baseline identity schema id is unsupported",
        "studio.issue.shell_handoff_acceptance_baseline_identity_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_summary_schema",
        baseline_identity.summary.schema_id == SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA,
        "baseline identity summary schema id is supported",
        "baseline identity summary schema id is unsupported",
        "studio.issue.shell_handoff_acceptance_summary_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_checklist_schema",
        baseline_identity.summary.checklist_schema == baseline.schema_id,
        "baseline identity summary names the loaded checklist schema",
        "baseline identity summary does not name the loaded checklist schema",
        "studio.issue.shell_handoff_acceptance_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_manifest",
        baseline_identity.summary.manifest_id == baseline.manifest_id,
        "baseline identity manifest id matches the loaded checklist",
        "baseline identity manifest id differs from the loaded checklist",
        "studio.issue.shell_handoff_acceptance_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_project",
        baseline_identity.summary.project_id == baseline.project_id
            && baseline_identity.summary.project_revision == baseline.project_revision,
        "baseline identity project metadata matches the loaded checklist",
        "baseline identity project metadata differs from the loaded checklist",
        "studio.issue.shell_handoff_acceptance_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_status_counts",
        baseline_identity.summary.status == baseline.status
            && baseline_identity.summary.ready_count == baseline.ready_count
            && baseline_identity.summary.blocked_count == baseline.blocked_count
            && baseline_identity.summary.rejected_count == baseline.rejected_count
            && baseline_identity.summary.entry_count == baseline.entries.len(),
        "baseline identity readiness counts match the loaded checklist",
        "baseline identity readiness counts differ from the loaded checklist",
        "studio.issue.shell_handoff_acceptance_baseline_identity_mismatch",
    );
    checks
}

fn shell_handoff_acceptance_baseline_index_entry_checks(
    context: &ShellHandoffAcceptanceBaselineIndexComparisonContext<'_>,
    baseline_identity: &StudioShellHandoffAcceptanceBaselineManifest,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    let entry = context.entry;
    let summary = &baseline_identity.summary;
    let expected_manifest_path = context
        .baseline_manifest_path
        .map(|path| path.display().to_string());
    let manifest_path_matches = match (
        expected_manifest_path.as_deref(),
        entry.baseline_manifest_path.as_deref(),
    ) {
        (Some(expected), Some(actual)) => actual == expected,
        (None, Some(actual)) => !actual.trim().is_empty(),
        _ => false,
    };

    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_schema",
        context.index.schema_id == SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA,
        "baseline index schema id is supported",
        "baseline index schema id is unsupported",
        "studio.issue.shell_handoff_acceptance_baseline_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_selected_baseline",
        entry.baseline_id == baseline_identity.baseline_id,
        "baseline index selected entry matches the loaded baseline identity",
        "baseline index selected entry differs from the loaded baseline identity",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_manifest_path",
        manifest_path_matches,
        "baseline index entry manifest path names the loaded baseline identity",
        "baseline index entry manifest path is missing or stale",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_checklist_path",
        entry.checklist_path == baseline_identity.checklist_path,
        "baseline index entry checklist path matches the loaded baseline identity",
        "baseline index entry checklist path differs from the loaded baseline identity",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_summary_schema",
        entry.summary_schema == summary.schema_id
            && entry.checklist_schema == summary.checklist_schema,
        "baseline index entry schema references match the loaded baseline identity",
        "baseline index entry schema references differ from the loaded baseline identity",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_source_metadata",
        entry.manifest_id == summary.manifest_id
            && entry.project_id == summary.project_id
            && entry.project_revision == summary.project_revision,
        "baseline index entry source metadata matches the loaded baseline identity",
        "baseline index entry source metadata differs from the loaded baseline identity",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_status_counts",
        entry.status == summary.status
            && entry.issue_code == summary.issue_code
            && entry.ready_count == summary.ready_count
            && entry.blocked_count == summary.blocked_count
            && entry.rejected_count == summary.rejected_count
            && entry.entry_count == summary.entry_count
            && entry.target_count == summary.targets.len(),
        "baseline index entry readiness counts match the loaded baseline identity",
        "baseline index entry readiness counts differ from the loaded baseline identity",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    checks
}

fn shell_handoff_acceptance_comparison_entries(
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> Vec<StudioShellHandoffAcceptanceComparisonEntry> {
    let baseline_entries = baseline
        .entries
        .iter()
        .map(|entry| (entry.graph_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let candidate_entries = candidate
        .entries
        .iter()
        .map(|entry| (entry.graph_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let graph_ids = baseline_entries
        .keys()
        .chain(candidate_entries.keys())
        .map(|graph_id| (*graph_id).to_string())
        .collect::<BTreeSet<_>>();

    graph_ids
        .into_iter()
        .map(|graph_id| {
            shell_handoff_acceptance_comparison_entry(
                &graph_id,
                baseline_entries.get(graph_id.as_str()).copied(),
                candidate_entries.get(graph_id.as_str()).copied(),
            )
        })
        .collect()
}

fn shell_handoff_acceptance_comparison_entry(
    graph_id: &str,
    baseline: Option<&StudioShellHandoffAcceptanceChecklistEntry>,
    candidate: Option<&StudioShellHandoffAcceptanceChecklistEntry>,
) -> StudioShellHandoffAcceptanceComparisonEntry {
    let baseline_score = baseline.map(|entry| acceptance_status_score(entry.status));
    let candidate_score = candidate.map(|entry| acceptance_status_score(entry.status));
    let score_delta = candidate_score.unwrap_or(0) - baseline_score.unwrap_or(0);
    let change = match (baseline, candidate) {
        (None, Some(_)) => StudioShellHandoffAcceptanceComparisonChange::Added,
        (Some(_), None) => StudioShellHandoffAcceptanceComparisonChange::Removed,
        (Some(_), Some(_)) if score_delta > 0 => {
            StudioShellHandoffAcceptanceComparisonChange::Improved
        }
        (Some(_), Some(_)) if score_delta < 0 => {
            StudioShellHandoffAcceptanceComparisonChange::Regressed
        }
        (Some(baseline), Some(candidate))
            if baseline.consumer_id != candidate.consumer_id
                || baseline.runtime_route_kind != candidate.runtime_route_kind
                || baseline.issue_code != candidate.issue_code =>
        {
            StudioShellHandoffAcceptanceComparisonChange::Changed
        }
        (Some(_), Some(_)) => StudioShellHandoffAcceptanceComparisonChange::Unchanged,
        (None, None) => StudioShellHandoffAcceptanceComparisonChange::Unchanged,
    };
    let issue_code = match change {
        StudioShellHandoffAcceptanceComparisonChange::Regressed
        | StudioShellHandoffAcceptanceComparisonChange::Removed => candidate
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| baseline.and_then(|entry| entry.issue_code.clone()))
            .or_else(|| Some("studio.issue.shell_handoff_acceptance_regressed".to_string())),
        StudioShellHandoffAcceptanceComparisonChange::Added
        | StudioShellHandoffAcceptanceComparisonChange::Improved
        | StudioShellHandoffAcceptanceComparisonChange::Unchanged
        | StudioShellHandoffAcceptanceComparisonChange::Changed => None,
    };

    StudioShellHandoffAcceptanceComparisonEntry {
        graph_id: graph_id.to_string(),
        target_kind: candidate
            .map(|entry| entry.target_kind)
            .or_else(|| baseline.map(|entry| entry.target_kind)),
        baseline_status: baseline.map(|entry| entry.status),
        candidate_status: candidate.map(|entry| entry.status),
        change,
        score_delta,
        baseline_consumer_id: baseline.map(|entry| entry.consumer_id.clone()),
        candidate_consumer_id: candidate.map(|entry| entry.consumer_id.clone()),
        baseline_route_kind: baseline.map(|entry| entry.runtime_route_kind.clone()),
        candidate_route_kind: candidate.map(|entry| entry.runtime_route_kind.clone()),
        baseline_issue_code: baseline.and_then(|entry| entry.issue_code.clone()),
        candidate_issue_code: candidate.and_then(|entry| entry.issue_code.clone()),
        issue_code,
    }
}

fn acceptance_status_score(status: StudioShellHandoffAcceptanceStatus) -> isize {
    match status {
        StudioShellHandoffAcceptanceStatus::Rejected => 0,
        StudioShellHandoffAcceptanceStatus::Blocked => 1,
        StudioShellHandoffAcceptanceStatus::Ready => 2,
    }
}

pub fn shell_release_candidate_review_for_manifest(
    manifest: &StudioShellHandoffManifest,
    manifest_path: Option<&Path>,
    acceptance_baseline_index: &StudioShellHandoffAcceptanceBaselineIndex,
    acceptance_baseline_index_path: Option<&Path>,
    acceptance_baseline_id: Option<&str>,
    export_package_baseline_index: &StudioShellExportPackageBaselineIndex,
    export_package_baseline_index_path: Option<&Path>,
    export_package_baseline_id: Option<&str>,
) -> StudioShellReleaseCandidateReviewReport {
    let manifest_validation = validate_shell_handoff_manifest(manifest);
    let intake = shell_handoff_intake_for_manifest(manifest);
    let candidate_acceptance = shell_handoff_acceptance_checklist_for_intake(&intake);
    let candidate_export_package = shell_export_package_for_manifest(manifest);
    let acceptance_selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        acceptance_baseline_index,
        acceptance_baseline_index_path,
        acceptance_baseline_id,
    );
    let export_package_selection = summarize_shell_export_package_baseline_index_selection(
        export_package_baseline_index,
        export_package_baseline_index_path,
        export_package_baseline_id,
    );

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.source_manifest_schema",
        manifest.schema_id == SHELL_HANDOFF_MANIFEST_SCHEMA,
        "source handoff manifest schema is supported",
        "source handoff manifest schema is unsupported",
        "studio.issue.shell_release_candidate_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.handoff_manifest_validation",
        manifest_validation.status == StudioValidationStatus::Pass,
        "handoff manifest validation passed",
        "handoff manifest validation failed",
        "studio.issue.shell_release_candidate_manifest_validation_failed",
    );
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.handoff_ready",
        manifest.status == StudioValidationStatus::Pass,
        "handoff manifest is ready for downstream review",
        "handoff manifest still has failed or missing generated shell bundles",
        "studio.issue.shell_release_candidate_handoff_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.runtime_command_authority",
        manifest.runtime_authority.command_session_authority == "rusty.manifold",
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.runtime_host_authority",
        manifest.runtime_authority.install_launch_evidence_authority == "rusty.hostess",
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.studio_role",
        manifest.runtime_authority.studio_role == "authoring.export_planning",
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );

    let (acceptance_comparison, acceptance_checks) = shell_release_candidate_acceptance_comparison(
        acceptance_baseline_index,
        acceptance_baseline_index_path,
        acceptance_baseline_id,
        &acceptance_selection,
        &candidate_acceptance,
    );
    checks.extend(acceptance_checks);
    let acceptance_comparison_ok = acceptance_comparison.as_ref().is_some_and(|comparison| {
        matches!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Improved
                | StudioShellHandoffAcceptanceComparisonStatus::Unchanged
        )
    });
    push_release_candidate_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.acceptance_comparison_not_regressed",
        acceptance_comparison_ok,
        "acceptance comparison is unchanged or improved",
        "acceptance comparison is missing, regressed, or incomparable",
        acceptance_comparison
            .as_ref()
            .and_then(|comparison| comparison.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_release_candidate_acceptance_comparison_blocked"),
    );

    let (export_package_comparison, export_package_checks) =
        shell_release_candidate_export_package_comparison(
            export_package_baseline_index,
            export_package_baseline_index_path,
            export_package_baseline_id,
            &export_package_selection,
            &candidate_export_package,
        );
    checks.extend(export_package_checks);
    let export_package_comparison_ok =
        export_package_comparison
            .as_ref()
            .is_some_and(|comparison| {
                matches!(
                    comparison.status,
                    StudioShellExportPackageComparisonStatus::Improved
                        | StudioShellExportPackageComparisonStatus::Unchanged
                )
            });
    push_release_candidate_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.export_package_comparison_not_regressed",
        export_package_comparison_ok,
        "export-package comparison is unchanged or improved",
        "export-package comparison is missing, regressed, or incomparable",
        export_package_comparison
            .as_ref()
            .and_then(|comparison| comparison.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_release_candidate_export_package_comparison_blocked"),
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let status = if manifest_validation.status == StudioValidationStatus::Fail
        || manifest.schema_id != SHELL_HANDOFF_MANIFEST_SCHEMA
    {
        StudioShellReleaseCandidateReviewStatus::Rejected
    } else if has_failed_check {
        StudioShellReleaseCandidateReviewStatus::Blocked
    } else {
        StudioShellReleaseCandidateReviewStatus::Ready
    };
    let issue_code = match status {
        StudioShellReleaseCandidateReviewStatus::Ready => None,
        StudioShellReleaseCandidateReviewStatus::Blocked
        | StudioShellReleaseCandidateReviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioShellReleaseCandidateReviewReport {
        schema_id: SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA.to_string(),
        source_manifest_schema: manifest.schema_id.clone(),
        manifest_path: manifest_path.map(|path| path.display().to_string()),
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        review_owner: "rusty.hostess".to_string(),
        command_session_authority: manifest.runtime_authority.command_session_authority.clone(),
        install_launch_evidence_authority: manifest
            .runtime_authority
            .install_launch_evidence_authority
            .clone(),
        studio_role: manifest.runtime_authority.studio_role.clone(),
        handoff_status: manifest.status,
        handoff_ready_count: manifest.ready_count,
        handoff_failed_count: manifest.failed_count,
        handoff_missing_bundle_count: manifest.missing_bundle_count,
        acceptance_baseline_selection: acceptance_selection,
        acceptance_comparison,
        export_package_baseline_selection: export_package_selection,
        export_package_comparison,
        checks,
        prohibited_actions: unique_strings(
            candidate_acceptance
                .prohibited_actions
                .iter()
                .cloned()
                .chain(candidate_export_package.prohibited_actions.iter().cloned()),
        ),
    }
}

fn shell_release_candidate_acceptance_comparison(
    baseline_index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_index_path: Option<&Path>,
    baseline_id: Option<&str>,
    selection: &StudioShellHandoffAcceptanceBaselineSelectionReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> (
    Option<StudioShellHandoffAcceptanceComparisonReport>,
    Vec<StudioValidationCheck>,
) {
    let mut checks = Vec::new();
    let selected =
        selection.status == StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected;
    push_release_candidate_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.acceptance_baseline_selected",
        selected,
        "acceptance baseline index selected a baseline",
        "acceptance baseline index did not select a baseline",
        selection
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_release_candidate_acceptance_baseline_not_selected"),
    );
    if !selected {
        return (None, checks);
    }

    let Some(entry) =
        select_shell_handoff_acceptance_baseline_index_entry(baseline_index, baseline_id)
    else {
        return (None, checks);
    };
    let Some(baseline_manifest_path) = entry.baseline_manifest_path.as_ref().map(PathBuf::from)
    else {
        push_release_candidate_check(
            &mut checks,
            "studio.check.shell_release_candidate_review.acceptance_baseline_manifest_path",
            false,
            "acceptance baseline index entry has a manifest path",
            "acceptance baseline index entry does not include a manifest path",
            "studio.issue.shell_release_candidate_acceptance_baseline_manifest_missing",
        );
        return (None, checks);
    };

    let baseline_identity =
        match load_shell_handoff_acceptance_baseline_manifest(&baseline_manifest_path) {
            Ok(baseline_identity) => baseline_identity,
            Err(error) => {
                checks.push(failed_release_candidate_check(
                    "studio.check.shell_release_candidate_review.acceptance_baseline_manifest_load",
                    error.to_string(),
                    "studio.issue.shell_release_candidate_acceptance_baseline_load_failed",
                ));
                return (None, checks);
            }
        };
    let baseline_path = PathBuf::from(&baseline_identity.checklist_path);
    let baseline = match load_shell_handoff_acceptance_checklist(&baseline_path) {
        Ok(baseline) => baseline,
        Err(error) => {
            checks.push(failed_release_candidate_check(
                "studio.check.shell_release_candidate_review.acceptance_baseline_checklist_load",
                error.to_string(),
                "studio.issue.shell_release_candidate_acceptance_checklist_load_failed",
            ));
            return (None, checks);
        }
    };
    let comparison = compare_shell_handoff_acceptance_against_baseline_index_entry(
        baseline_index,
        baseline_index_path,
        entry,
        Some(&baseline_manifest_path),
        &baseline_identity,
        &baseline,
        candidate,
    );
    (Some(comparison), checks)
}

fn shell_release_candidate_export_package_comparison(
    baseline_index: &StudioShellExportPackageBaselineIndex,
    baseline_index_path: Option<&Path>,
    baseline_id: Option<&str>,
    selection: &StudioShellExportPackageBaselineSelectionReport,
    candidate: &StudioShellExportPackageReport,
) -> (
    Option<StudioShellExportPackageComparisonReport>,
    Vec<StudioValidationCheck>,
) {
    let mut checks = Vec::new();
    let selected = selection.status == StudioShellExportPackageBaselineSelectionStatus::Selected;
    push_release_candidate_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.export_package_baseline_selected",
        selected,
        "export-package baseline index selected a baseline",
        "export-package baseline index did not select a baseline",
        selection
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_release_candidate_export_package_baseline_not_selected"),
    );
    if !selected {
        return (None, checks);
    }

    let Some(entry) = select_shell_export_package_baseline_index_entry(baseline_index, baseline_id)
    else {
        return (None, checks);
    };
    let Some(baseline_manifest_path) = entry.baseline_manifest_path.as_ref().map(PathBuf::from)
    else {
        push_release_candidate_check(
            &mut checks,
            "studio.check.shell_release_candidate_review.export_package_baseline_manifest_path",
            false,
            "export-package baseline index entry has a manifest path",
            "export-package baseline index entry does not include a manifest path",
            "studio.issue.shell_release_candidate_export_package_baseline_manifest_missing",
        );
        return (None, checks);
    };

    let baseline_identity =
        match load_shell_export_package_baseline_manifest(&baseline_manifest_path) {
            Ok(baseline_identity) => baseline_identity,
            Err(error) => {
                checks.push(failed_release_candidate_check(
                "studio.check.shell_release_candidate_review.export_package_baseline_manifest_load",
                error.to_string(),
                "studio.issue.shell_release_candidate_export_package_baseline_load_failed",
            ));
                return (None, checks);
            }
        };
    let baseline_path = PathBuf::from(&baseline_identity.package_path);
    let baseline = match load_shell_export_package_report(&baseline_path) {
        Ok(baseline) => baseline,
        Err(error) => {
            checks.push(failed_release_candidate_check(
                "studio.check.shell_release_candidate_review.export_package_baseline_report_load",
                error.to_string(),
                "studio.issue.shell_release_candidate_export_package_report_load_failed",
            ));
            return (None, checks);
        }
    };
    let comparison = compare_shell_export_packages_against_baseline_index_entry(
        baseline_index,
        baseline_index_path,
        entry,
        Some(&baseline_manifest_path),
        &baseline_identity,
        &baseline,
        candidate,
    );
    (Some(comparison), checks)
}

fn push_release_candidate_check(
    checks: &mut Vec<StudioValidationCheck>,
    check_id: &str,
    valid: bool,
    pass_evidence: &str,
    fail_evidence: &str,
    issue_code: &str,
) {
    push_check(
        checks,
        check_id,
        valid,
        pass_evidence,
        fail_evidence,
        issue_code,
    );
}

fn failed_release_candidate_check(
    check_id: &str,
    evidence: String,
    issue_code: &str,
) -> StudioValidationCheck {
    StudioValidationCheck {
        check_id: check_id.to_string(),
        status: StudioValidationStatus::Fail,
        evidence,
        issue_code: Some(issue_code.to_string()),
        graph_id: None,
        node_ids: Vec::new(),
        edge_ids: Vec::new(),
        reference_ids: Vec::new(),
    }
}

pub fn shell_release_candidate_review_manifest_for_report(
    review: &StudioShellReleaseCandidateReviewReport,
    review_path: &Path,
    candidate_id: Option<&str>,
    label: Option<&str>,
) -> StudioShellReleaseCandidateReviewManifest {
    let candidate_id = candidate_id
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_release_candidate_review_id(review));
    let label = label
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_release_candidate_review_label(review));

    StudioShellReleaseCandidateReviewManifest {
        schema_id: SHELL_RELEASE_CANDIDATE_REVIEW_MANIFEST_SCHEMA.to_string(),
        candidate_id,
        label,
        review_path: review_path.display().to_string(),
        review_schema: review.schema_id.clone(),
        manifest_id: review.manifest_id.clone(),
        project_id: review.project_id.clone(),
        project_revision: review.project_revision,
        status: review.status,
        issue_code: review.issue_code.clone(),
        execution_policy: review.execution_policy.clone(),
        review_owner: review.review_owner.clone(),
        command_session_authority: review.command_session_authority.clone(),
        install_launch_evidence_authority: review.install_launch_evidence_authority.clone(),
        studio_role: review.studio_role.clone(),
        handoff_ready_count: review.handoff_ready_count,
        handoff_failed_count: review.handoff_failed_count,
        handoff_missing_bundle_count: review.handoff_missing_bundle_count,
        acceptance_baseline_status: review.acceptance_baseline_selection.status,
        acceptance_baseline_id: review
            .acceptance_baseline_selection
            .selected_baseline_id
            .clone(),
        acceptance_comparison_status: review
            .acceptance_comparison
            .as_ref()
            .map(|comparison| comparison.status),
        export_package_baseline_status: review.export_package_baseline_selection.status,
        export_package_baseline_id: review
            .export_package_baseline_selection
            .selected_baseline_id
            .clone(),
        export_package_comparison_status: review
            .export_package_comparison
            .as_ref()
            .map(|comparison| comparison.status),
        check_count: review.checks.len(),
        failed_check_count: review
            .checks
            .iter()
            .filter(|check| check.status == StudioValidationStatus::Fail)
            .count(),
        prohibited_actions: review.prohibited_actions.clone(),
    }
}

pub fn shell_release_candidate_review_index_for_manifests(
    candidates: Vec<(StudioShellReleaseCandidateReviewManifest, Option<PathBuf>)>,
    default_candidate_id: Option<&str>,
) -> StudioShellReleaseCandidateReviewIndex {
    let entries = candidates
        .into_iter()
        .map(|(candidate, candidate_manifest_path)| {
            shell_release_candidate_review_index_entry_for_manifest(
                candidate,
                candidate_manifest_path,
            )
        })
        .collect::<Vec<_>>();

    shell_release_candidate_review_index_for_entries(entries, default_candidate_id)
}

pub fn append_shell_release_candidate_review_index_manifests(
    index: &StudioShellReleaseCandidateReviewIndex,
    candidates: Vec<(StudioShellReleaseCandidateReviewManifest, Option<PathBuf>)>,
    default_candidate_id: Option<&str>,
) -> StudioShellReleaseCandidateReviewIndex {
    let entries = index
        .entries
        .iter()
        .cloned()
        .chain(
            candidates
                .into_iter()
                .map(|(candidate, candidate_manifest_path)| {
                    shell_release_candidate_review_index_entry_for_manifest(
                        candidate,
                        candidate_manifest_path,
                    )
                }),
        )
        .collect::<Vec<_>>();
    let default_candidate_id = default_candidate_id.or(index.default_candidate_id.as_deref());

    shell_release_candidate_review_index_for_entries(entries, default_candidate_id)
}

pub fn promote_shell_release_candidate_review_index_default(
    index: &StudioShellReleaseCandidateReviewIndex,
    candidate_id: &str,
) -> Option<StudioShellReleaseCandidateReviewIndex> {
    index
        .entries
        .iter()
        .any(|entry| entry.candidate_id == candidate_id)
        .then(|| {
            shell_release_candidate_review_index_for_entries(
                index.entries.clone(),
                Some(candidate_id),
            )
        })
}

fn shell_release_candidate_review_index_entry_for_manifest(
    candidate: StudioShellReleaseCandidateReviewManifest,
    candidate_manifest_path: Option<PathBuf>,
) -> StudioShellReleaseCandidateReviewIndexEntry {
    StudioShellReleaseCandidateReviewIndexEntry {
        candidate_id: candidate.candidate_id,
        label: candidate.label,
        candidate_manifest_path: candidate_manifest_path.map(|path| path.display().to_string()),
        review_path: candidate.review_path,
        review_schema: candidate.review_schema,
        manifest_id: candidate.manifest_id,
        project_id: candidate.project_id,
        project_revision: candidate.project_revision,
        status: candidate.status,
        issue_code: candidate.issue_code,
        execution_policy: candidate.execution_policy,
        review_owner: candidate.review_owner,
        command_session_authority: candidate.command_session_authority,
        install_launch_evidence_authority: candidate.install_launch_evidence_authority,
        studio_role: candidate.studio_role,
        handoff_ready_count: candidate.handoff_ready_count,
        handoff_failed_count: candidate.handoff_failed_count,
        handoff_missing_bundle_count: candidate.handoff_missing_bundle_count,
        acceptance_baseline_status: candidate.acceptance_baseline_status,
        acceptance_baseline_id: candidate.acceptance_baseline_id,
        acceptance_comparison_status: candidate.acceptance_comparison_status,
        export_package_baseline_status: candidate.export_package_baseline_status,
        export_package_baseline_id: candidate.export_package_baseline_id,
        export_package_comparison_status: candidate.export_package_comparison_status,
        check_count: candidate.check_count,
        failed_check_count: candidate.failed_check_count,
    }
}

fn shell_release_candidate_review_index_for_entries(
    entries: Vec<StudioShellReleaseCandidateReviewIndexEntry>,
    default_candidate_id: Option<&str>,
) -> StudioShellReleaseCandidateReviewIndex {
    let mut by_id = BTreeMap::new();
    for entry in entries {
        by_id.insert(entry.candidate_id.clone(), entry);
    }

    let entries = by_id.into_values().collect::<Vec<_>>();
    let default_candidate_id = default_candidate_id
        .filter(|candidate_id| {
            entries
                .iter()
                .any(|entry| entry.candidate_id == *candidate_id)
        })
        .map(str::to_string)
        .or_else(|| entries.first().map(|entry| entry.candidate_id.clone()));

    StudioShellReleaseCandidateReviewIndex {
        schema_id: SHELL_RELEASE_CANDIDATE_REVIEW_INDEX_SCHEMA.to_string(),
        project_ids: unique_strings(entries.iter().map(|entry| entry.project_id.clone())),
        manifest_ids: unique_strings(entries.iter().map(|entry| entry.manifest_id.clone())),
        default_candidate_id,
        candidate_count: entries.len(),
        ready_candidate_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellReleaseCandidateReviewStatus::Ready)
            .count(),
        blocked_candidate_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellReleaseCandidateReviewStatus::Blocked)
            .count(),
        rejected_candidate_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellReleaseCandidateReviewStatus::Rejected)
            .count(),
        entries,
    }
}

pub fn select_shell_release_candidate_review_index_entry<'a>(
    index: &'a StudioShellReleaseCandidateReviewIndex,
    candidate_id: Option<&str>,
) -> Option<&'a StudioShellReleaseCandidateReviewIndexEntry> {
    let selected_id = candidate_id.or(index.default_candidate_id.as_deref());
    selected_id
        .and_then(|selected_id| {
            index
                .entries
                .iter()
                .find(|entry| entry.candidate_id == selected_id)
        })
        .or_else(|| {
            candidate_id
                .is_none()
                .then(|| index.entries.first())
                .flatten()
        })
}

pub fn summarize_shell_release_candidate_review_index_selection(
    index: &StudioShellReleaseCandidateReviewIndex,
    index_path: Option<&Path>,
    requested_candidate_id: Option<&str>,
) -> StudioShellReleaseCandidateReviewSelectionReport {
    let selected_entry =
        select_shell_release_candidate_review_index_entry(index, requested_candidate_id);
    let selected_candidate_id = selected_entry.map(|entry| entry.candidate_id.clone());
    let status = if index.entries.is_empty() {
        StudioShellReleaseCandidateReviewSelectionStatus::Empty
    } else if selected_entry.is_some() {
        StudioShellReleaseCandidateReviewSelectionStatus::Selected
    } else {
        StudioShellReleaseCandidateReviewSelectionStatus::Missing
    };
    let issue_code = match status {
        StudioShellReleaseCandidateReviewSelectionStatus::Selected => None,
        StudioShellReleaseCandidateReviewSelectionStatus::Missing => {
            Some("studio.issue.shell_release_candidate_review_not_found".to_string())
        }
        StudioShellReleaseCandidateReviewSelectionStatus::Empty => {
            Some("studio.issue.shell_release_candidate_review_index_empty".to_string())
        }
    };

    StudioShellReleaseCandidateReviewSelectionReport {
        schema_id: SHELL_RELEASE_CANDIDATE_REVIEW_SELECTION_SCHEMA.to_string(),
        source_index_schema: index.schema_id.clone(),
        index_path: index_path.map(|path| path.display().to_string()),
        requested_candidate_id: requested_candidate_id.map(str::to_string),
        default_candidate_id: index.default_candidate_id.clone(),
        selected_candidate_id: selected_candidate_id.clone(),
        status,
        issue_code,
        candidate_count: index.candidate_count,
        ready_candidate_count: index.ready_candidate_count,
        blocked_candidate_count: index.blocked_candidate_count,
        rejected_candidate_count: index.rejected_candidate_count,
        project_ids: index.project_ids.clone(),
        manifest_ids: index.manifest_ids.clone(),
        entries: index
            .entries
            .iter()
            .map(|entry| StudioShellReleaseCandidateReviewSelectionEntry {
                candidate_id: entry.candidate_id.clone(),
                label: entry.label.clone(),
                selected: selected_candidate_id.as_deref() == Some(entry.candidate_id.as_str()),
                default: index.default_candidate_id.as_deref() == Some(entry.candidate_id.as_str()),
                candidate_manifest_path: entry.candidate_manifest_path.clone(),
                review_path: entry.review_path.clone(),
                manifest_id: entry.manifest_id.clone(),
                project_id: entry.project_id.clone(),
                project_revision: entry.project_revision,
                status: entry.status,
                issue_code: entry.issue_code.clone(),
                acceptance_baseline_id: entry.acceptance_baseline_id.clone(),
                acceptance_comparison_status: entry.acceptance_comparison_status,
                export_package_baseline_id: entry.export_package_baseline_id.clone(),
                export_package_comparison_status: entry.export_package_comparison_status,
                check_count: entry.check_count,
                failed_check_count: entry.failed_check_count,
            })
            .collect(),
    }
}

pub fn shell_hostess_handoff_package_for_release_candidate_index(
    index: &StudioShellReleaseCandidateReviewIndex,
    index_path: Option<&Path>,
    requested_candidate_id: Option<&str>,
) -> StudioShellHostessHandoffPackageReport {
    let selection = summarize_shell_release_candidate_review_index_selection(
        index,
        index_path,
        requested_candidate_id,
    );
    let selected_entry =
        select_shell_release_candidate_review_index_entry(index, requested_candidate_id);
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.source_index_schema",
        index.schema_id == SHELL_RELEASE_CANDIDATE_REVIEW_INDEX_SCHEMA,
        "source release-candidate review index schema is supported",
        "source release-candidate review index schema is unsupported",
        "studio.issue.shell_release_candidate_review_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.candidate_selected",
        selection.status == StudioShellReleaseCandidateReviewSelectionStatus::Selected,
        "release-candidate review index selected a candidate",
        "release-candidate review index did not select a candidate",
        selection
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_release_candidate_review_not_selected"),
    );

    let candidate_manifest_path = selected_entry
        .and_then(|entry| entry.candidate_manifest_path.as_ref())
        .map(PathBuf::from);
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.candidate_manifest_path",
        candidate_manifest_path.is_some(),
        "selected candidate has an identity manifest path",
        "selected candidate does not include an identity manifest path",
        "studio.issue.shell_hostess_handoff_candidate_manifest_missing",
    );

    let candidate_manifest = candidate_manifest_path.as_ref().and_then(|path| {
        match load_shell_release_candidate_review_manifest(path) {
            Ok(candidate) => Some(candidate),
            Err(error) => {
                checks.push(failed_hostess_handoff_package_check(
                    "studio.check.shell_hostess_handoff_package.candidate_manifest_load",
                    error.to_string(),
                    "studio.issue.shell_hostess_handoff_candidate_manifest_load_failed",
                ));
                None
            }
        }
    });

    let candidate_manifest_schema = candidate_manifest
        .as_ref()
        .map(|candidate| candidate.schema_id.clone());
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.candidate_manifest_schema",
        candidate_manifest.as_ref().is_some_and(|candidate| {
            candidate.schema_id == SHELL_RELEASE_CANDIDATE_REVIEW_MANIFEST_SCHEMA
        }),
        "selected candidate identity manifest schema is supported",
        "selected candidate identity manifest schema is unsupported or unavailable",
        "studio.issue.shell_release_candidate_review_manifest_schema",
    );
    let candidate_id_matches_index = selected_entry
        .zip(candidate_manifest.as_ref())
        .is_some_and(|(entry, candidate)| entry.candidate_id == candidate.candidate_id);
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.candidate_identity_matches_index",
        candidate_id_matches_index,
        "selected candidate identity matches the index entry",
        "selected candidate identity does not match the index entry",
        "studio.issue.shell_hostess_handoff_candidate_identity_mismatch",
    );

    let review_path = candidate_manifest
        .as_ref()
        .map(|candidate| PathBuf::from(&candidate.review_path))
        .or_else(|| selected_entry.map(|entry| PathBuf::from(&entry.review_path)));
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.review_path",
        review_path.is_some(),
        "selected candidate names a release-candidate review artifact",
        "selected candidate does not name a release-candidate review artifact",
        "studio.issue.shell_hostess_handoff_review_missing",
    );

    let review = review_path.as_ref().and_then(|path| {
        match load_shell_release_candidate_review_report(path) {
            Ok(review) => Some(review),
            Err(error) => {
                checks.push(failed_hostess_handoff_package_check(
                    "studio.check.shell_hostess_handoff_package.review_load",
                    error.to_string(),
                    "studio.issue.shell_hostess_handoff_review_load_failed",
                ));
                None
            }
        }
    });
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.review_schema",
        review
            .as_ref()
            .is_some_and(|review| review.schema_id == SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA),
        "selected review artifact schema is supported",
        "selected review artifact schema is unsupported or unavailable",
        "studio.issue.shell_release_candidate_review_schema",
    );
    let review_matches_candidate = candidate_manifest
        .as_ref()
        .zip(review_path.as_ref())
        .is_some_and(|(candidate, review_path)| {
            candidate.review_path == review_path.display().to_string()
        });
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.review_matches_candidate",
        review_matches_candidate,
        "selected review artifact path matches the candidate identity",
        "selected review artifact path does not match the candidate identity",
        "studio.issue.shell_hostess_handoff_review_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.review_ready",
        review
            .as_ref()
            .is_some_and(|review| review.status == StudioShellReleaseCandidateReviewStatus::Ready),
        "selected release candidate is ready for Hostess handoff",
        "selected release candidate is not ready for Hostess handoff",
        review
            .as_ref()
            .and_then(|review| review.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_hostess_handoff_release_candidate_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.handoff_manifest_path",
        review
            .as_ref()
            .and_then(|review| review.manifest_path.as_ref())
            .is_some(),
        "selected review names a saved handoff manifest",
        "selected review does not name a saved handoff manifest",
        "studio.issue.shell_hostess_handoff_manifest_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.handoff_ready",
        review.as_ref().is_some_and(|review| {
            review.handoff_status == StudioValidationStatus::Pass
                && review.handoff_failed_count == 0
                && review.handoff_missing_bundle_count == 0
        }),
        "handoff manifest is ready with no failed or missing bundles",
        "handoff manifest has failed or missing bundles",
        "studio.issue.shell_release_candidate_handoff_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.runtime_command_authority",
        review
            .as_ref()
            .is_some_and(|review| review.command_session_authority == "rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.runtime_host_authority",
        review
            .as_ref()
            .is_some_and(|review| review.install_launch_evidence_authority == "rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.studio_role",
        review
            .as_ref()
            .is_some_and(|review| review.studio_role == "authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.acceptance_baseline_selected",
        review.as_ref().is_some_and(|review| {
            review.acceptance_baseline_selection.status
                == StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
        }),
        "acceptance baseline selection is present",
        "acceptance baseline selection is missing",
        "studio.issue.shell_release_candidate_acceptance_baseline_not_selected",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.acceptance_comparison_ready",
        review.as_ref().is_some_and(|review| {
            review
                .acceptance_comparison
                .as_ref()
                .is_some_and(|comparison| {
                    matches!(
                        comparison.status,
                        StudioShellHandoffAcceptanceComparisonStatus::Improved
                            | StudioShellHandoffAcceptanceComparisonStatus::Unchanged
                    )
                })
        }),
        "acceptance comparison is unchanged or improved",
        "acceptance comparison is missing, regressed, or incomparable",
        review
            .as_ref()
            .and_then(|review| review.acceptance_comparison.as_ref())
            .and_then(|comparison| comparison.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_release_candidate_acceptance_comparison_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.export_package_baseline_selected",
        review.as_ref().is_some_and(|review| {
            review.export_package_baseline_selection.status
                == StudioShellExportPackageBaselineSelectionStatus::Selected
        }),
        "export-package baseline selection is present",
        "export-package baseline selection is missing",
        "studio.issue.shell_release_candidate_export_package_baseline_not_selected",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.export_package_comparison_ready",
        review.as_ref().is_some_and(|review| {
            review
                .export_package_comparison
                .as_ref()
                .is_some_and(|comparison| {
                    matches!(
                        comparison.status,
                        StudioShellExportPackageComparisonStatus::Improved
                            | StudioShellExportPackageComparisonStatus::Unchanged
                    )
                })
        }),
        "export-package comparison is unchanged or improved",
        "export-package comparison is missing, regressed, or incomparable",
        review
            .as_ref()
            .and_then(|review| review.export_package_comparison.as_ref())
            .and_then(|comparison| comparison.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_release_candidate_export_package_comparison_blocked"),
    );

    let prohibited_actions = shell_hostess_handoff_package_prohibited_actions(review.as_ref());
    for action in [
        "stage_generated_shells",
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
        "collect_install_launch_evidence",
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_handoff_package.prohibits_{action}"),
            prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "handoff package explicitly prohibits this Studio action",
            "handoff package does not explicitly prohibit this Studio action",
            "studio.issue.shell_hostess_handoff_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_release_candidate_review_index_schema")
                    | Some("studio.issue.shell_release_candidate_review_manifest_schema")
                    | Some("studio.issue.shell_release_candidate_review_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessHandoffPackageStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessHandoffPackageStatus::Blocked
    } else {
        StudioShellHostessHandoffPackageStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessHandoffPackageStatus::Ready => None,
        StudioShellHostessHandoffPackageStatus::Blocked
        | StudioShellHostessHandoffPackageStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let required_owner_actions =
        shell_hostess_handoff_package_owner_actions(status, issue_code.as_deref());

    StudioShellHostessHandoffPackageReport {
        schema_id: SHELL_HOSTESS_HANDOFF_PACKAGE_SCHEMA.to_string(),
        source_index_schema: index.schema_id.clone(),
        index_path: index_path.map(|path| path.display().to_string()),
        requested_candidate_id: requested_candidate_id.map(str::to_string),
        default_candidate_id: index.default_candidate_id.clone(),
        selected_candidate_id: selection.selected_candidate_id.clone(),
        selection_status: selection.status,
        selection_issue_code: selection.issue_code,
        candidate_manifest_schema,
        candidate_manifest_path: candidate_manifest_path.map(|path| path.display().to_string()),
        candidate_id: candidate_manifest
            .as_ref()
            .map(|candidate| candidate.candidate_id.clone())
            .or_else(|| selected_entry.map(|entry| entry.candidate_id.clone())),
        candidate_label: candidate_manifest
            .as_ref()
            .map(|candidate| candidate.label.clone())
            .or_else(|| selected_entry.map(|entry| entry.label.clone())),
        review_schema: review
            .as_ref()
            .map(|review| review.schema_id.clone())
            .or_else(|| selected_entry.map(|entry| entry.review_schema.clone())),
        review_path: review_path.map(|path| path.display().to_string()),
        handoff_manifest_schema: review
            .as_ref()
            .map(|review| review.source_manifest_schema.clone()),
        handoff_manifest_path: review
            .as_ref()
            .and_then(|review| review.manifest_path.clone()),
        manifest_id: review
            .as_ref()
            .map(|review| review.manifest_id.clone())
            .or_else(|| selected_entry.map(|entry| entry.manifest_id.clone())),
        project_id: review
            .as_ref()
            .map(|review| review.project_id.clone())
            .or_else(|| selected_entry.map(|entry| entry.project_id.clone())),
        project_revision: review
            .as_ref()
            .map(|review| review.project_revision)
            .or_else(|| selected_entry.map(|entry| entry.project_revision)),
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        handoff_owner: "rusty.hostess".to_string(),
        review_owner: review.as_ref().map(|review| review.review_owner.clone()),
        command_session_authority: review
            .as_ref()
            .map(|review| review.command_session_authority.clone()),
        install_launch_evidence_authority: review
            .as_ref()
            .map(|review| review.install_launch_evidence_authority.clone()),
        studio_role: review.as_ref().map(|review| review.studio_role.clone()),
        handoff_ready_count: review
            .as_ref()
            .map(|review| review.handoff_ready_count)
            .unwrap_or(0),
        handoff_failed_count: review
            .as_ref()
            .map(|review| review.handoff_failed_count)
            .unwrap_or(0),
        handoff_missing_bundle_count: review
            .as_ref()
            .map(|review| review.handoff_missing_bundle_count)
            .unwrap_or(0),
        acceptance_baseline_id: review.as_ref().and_then(|review| {
            review
                .acceptance_baseline_selection
                .selected_baseline_id
                .clone()
        }),
        acceptance_baseline_status: review
            .as_ref()
            .map(|review| review.acceptance_baseline_selection.status),
        acceptance_comparison_status: review
            .as_ref()
            .and_then(|review| review.acceptance_comparison.as_ref())
            .map(|comparison| comparison.status),
        export_package_baseline_id: review.as_ref().and_then(|review| {
            review
                .export_package_baseline_selection
                .selected_baseline_id
                .clone()
        }),
        export_package_baseline_status: review
            .as_ref()
            .map(|review| review.export_package_baseline_selection.status),
        export_package_comparison_status: review
            .as_ref()
            .and_then(|review| review.export_package_comparison.as_ref())
            .map(|comparison| comparison.status),
        required_owner_actions,
        prohibited_actions,
        checks,
    }
}

fn shell_hostess_handoff_package_prohibited_actions(
    review: Option<&StudioShellReleaseCandidateReviewReport>,
) -> Vec<String> {
    unique_strings(
        shell_handoff_acceptance_prohibited_actions()
            .into_iter()
            .chain(
                ["stage_generated_shells", "collect_install_launch_evidence"]
                    .into_iter()
                    .map(str::to_string),
            )
            .chain(
                review
                    .into_iter()
                    .flat_map(|review| review.prohibited_actions.iter().cloned()),
            ),
    )
}

fn shell_hostess_handoff_package_owner_actions(
    status: StudioShellHostessHandoffPackageStatus,
    issue_code: Option<&str>,
) -> Vec<StudioShellHostessHandoffPackageAction> {
    [
        (
            "hostess.review_release_candidate",
            "rusty.hostess",
            "release_candidate_review",
            "review_selected_release_candidate",
        ),
        (
            "hostess.stage_generated_shells",
            "rusty.hostess",
            "shell_handoff_manifest",
            "stage_generated_shells_outside_studio",
        ),
        (
            "manifold.review_command_session_contract",
            "rusty.manifold",
            "release_candidate_review",
            "review_command_session_contract_outside_studio",
        ),
        (
            "hostess.collect_install_launch_evidence",
            "rusty.hostess",
            "hostess_handoff_package",
            "collect_install_launch_evidence_outside_studio",
        ),
    ]
    .into_iter()
    .map(|(action_id, owner, source, next_required_action)| {
        StudioShellHostessHandoffPackageAction {
            action_id: action_id.to_string(),
            owner: owner.to_string(),
            status: if status == StudioShellHostessHandoffPackageStatus::Ready {
                StudioShellHostessHandoffPackageActionStatus::Ready
            } else {
                StudioShellHostessHandoffPackageActionStatus::Blocked
            },
            source: source.to_string(),
            next_required_action: next_required_action.to_string(),
            prohibited_in_studio: true,
            issue_code: (status != StudioShellHostessHandoffPackageStatus::Ready).then(|| {
                issue_code
                    .unwrap_or("studio.issue.shell_hostess_handoff_package_blocked")
                    .to_string()
            }),
        }
    })
    .collect()
}

fn failed_hostess_handoff_package_check(
    check_id: &str,
    evidence: String,
    issue_code: &str,
) -> StudioValidationCheck {
    StudioValidationCheck {
        check_id: check_id.to_string(),
        status: StudioValidationStatus::Fail,
        evidence,
        issue_code: Some(issue_code.to_string()),
        graph_id: None,
        node_ids: Vec::new(),
        edge_ids: Vec::new(),
        reference_ids: Vec::new(),
    }
}

pub fn package_evidence_intake_for_validation_report(
    report: &StudioManifoldPackageValidationReport,
    report_path: Option<&Path>,
    target_package_id: &str,
) -> StudioPackageEvidenceIntakeReport {
    let package_prefix = format!("validation.package.{target_package_id}.");
    let required_check_ids = projected_motion_breath_required_check_ids(target_package_id);
    let required_check_id_set = required_check_ids
        .iter()
        .cloned()
        .collect::<BTreeSet<String>>();
    let target_package_checks = report
        .checks
        .iter()
        .filter(|check| check.check_id.starts_with(&package_prefix))
        .collect::<Vec<_>>();
    let target_package_check_count = target_package_checks.len();
    let target_package_supported = target_package_id == PROJECTED_MOTION_BREATH_PACKAGE_ID;

    let mut entries = Vec::new();
    for required_check_id in &required_check_ids {
        if let Some(check) = target_package_checks
            .iter()
            .find(|check| check.check_id == *required_check_id)
        {
            entries.push(package_evidence_intake_entry(check, true));
        } else {
            entries.push(missing_package_evidence_intake_entry(required_check_id));
        }
    }
    for check in target_package_checks {
        if !required_check_id_set.contains(&check.check_id) {
            entries.push(package_evidence_intake_entry(check, false));
        }
    }

    let ready_required_check_count = entries
        .iter()
        .filter(|entry| {
            entry.required_for_studio
                && entry.decision == StudioPackageEvidenceIntakeDecision::Ready
        })
        .count();
    let blocked_required_check_count = required_check_ids.len() - ready_required_check_count;
    let observed_check_count = entries
        .iter()
        .filter(|entry| !entry.required_for_studio)
        .count();
    let failed_target_check_ids = entries
        .iter()
        .filter(|entry| entry.source_status == StudioValidationStatus::Fail)
        .map(|entry| entry.check_id.clone())
        .collect::<Vec<_>>();
    let missing_required_check_ids = entries
        .iter()
        .filter(|entry| {
            entry.required_for_studio
                && entry.decision == StudioPackageEvidenceIntakeDecision::BlockedByMissingCheck
        })
        .map(|entry| entry.check_id.clone())
        .collect::<Vec<_>>();

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.source_report_schema",
        report.schema_id == MANIFOLD_PACKAGE_VALIDATION_REPORT_SCHEMA,
        "source Manifold package validation report schema is supported",
        "source Manifold package validation report schema is unsupported",
        "studio.issue.package_evidence_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.source_report_path",
        report_path.is_some(),
        "source Manifold package validation report has a durable path",
        "source Manifold package validation report path is missing",
        "studio.issue.package_evidence_source_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.source_report_status",
        report.status == StudioValidationStatus::Pass,
        "source Manifold package validation report passed",
        "source Manifold package validation report failed",
        "studio.issue.package_evidence_source_report_failed",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.target_package_id",
        is_dotted_id(target_package_id),
        "target package id uses dotted-id grammar",
        "target package id is not a dotted id",
        "studio.issue.package_evidence_target_package_id",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.target_package_supported",
        target_package_supported,
        "target package is supported by this Studio intake",
        "target package is not supported by this Studio intake",
        "studio.issue.package_evidence_target_package_unsupported",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.target_package_present",
        target_package_check_count > 0,
        "source report contains target package checks",
        "source report does not contain target package checks",
        "studio.issue.package_evidence_target_package_missing",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.required_checks_present",
        missing_required_check_ids.is_empty(),
        "source report contains all required projected-motion breath checks",
        &format!(
            "source report is missing required checks: {}",
            missing_required_check_ids.join(", ")
        ),
        "studio.issue.package_evidence_required_check_missing",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.required_checks_pass",
        blocked_required_check_count == 0,
        "all required projected-motion breath checks pass",
        "one or more required projected-motion breath checks are blocked",
        "studio.issue.package_evidence_required_check_failed",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.target_package_checks_pass",
        failed_target_check_ids.is_empty(),
        "all target package checks visible to Studio pass",
        &format!(
            "target package checks failed: {}",
            failed_target_check_ids.join(", ")
        ),
        "studio.issue.package_evidence_target_package_check_failed",
    );
    push_check(
        &mut checks,
        "studio.check.package_evidence_intake.execution_policy",
        true,
        "Studio package evidence intake is review-only and not executed",
        "Studio package evidence intake attempted execution",
        "studio.issue.package_evidence_execution_policy",
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.package_evidence_source_schema")
                    | Some("studio.issue.package_evidence_target_package_id")
                    | Some("studio.issue.package_evidence_target_package_unsupported")
            )
    });
    let status = if has_rejected_check {
        StudioPackageEvidenceIntakeStatus::Rejected
    } else if has_failed_check {
        StudioPackageEvidenceIntakeStatus::Blocked
    } else {
        StudioPackageEvidenceIntakeStatus::Ready
    };
    let issue_code = match status {
        StudioPackageEvidenceIntakeStatus::Ready => None,
        StudioPackageEvidenceIntakeStatus::Blocked
        | StudioPackageEvidenceIntakeStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioPackageEvidenceIntakeReport {
        schema_id: PACKAGE_EVIDENCE_INTAKE_REPORT_SCHEMA.to_string(),
        source_report_schema: report.schema_id.clone(),
        source_report_path: report_path.map(|path| path.display().to_string()),
        target_package_id: target_package_id.to_string(),
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        runtime_authority: "rusty.manifold".to_string(),
        authoring_authority: "rusty.studio".to_string(),
        platform_validation_authority: "rusty.hostess".to_string(),
        runtime_execution_performed: false,
        platform_execution_performed: false,
        source_report_status: report.status,
        source_check_count: report.checks.len(),
        target_package_check_count,
        required_check_count: required_check_ids.len(),
        ready_required_check_count,
        blocked_required_check_count,
        observed_check_count,
        entries,
        prohibited_actions: package_evidence_intake_prohibited_actions(),
        checks,
    }
}

fn projected_motion_breath_required_check_ids(target_package_id: &str) -> Vec<String> {
    PROJECTED_MOTION_BREATH_REQUIRED_CHECK_SUFFIXES
        .iter()
        .map(|suffix| format!("validation.package.{target_package_id}.{suffix}"))
        .collect()
}

fn package_evidence_intake_entry(
    check: &StudioManifoldPackageValidationCheck,
    required_for_studio: bool,
) -> StudioPackageEvidenceIntakeEntry {
    let decision = match (check.status, required_for_studio) {
        (StudioValidationStatus::Pass, true) => StudioPackageEvidenceIntakeDecision::Ready,
        (StudioValidationStatus::Pass, false) => StudioPackageEvidenceIntakeDecision::Observed,
        (StudioValidationStatus::Fail, _) => {
            StudioPackageEvidenceIntakeDecision::BlockedByFailedCheck
        }
    };
    StudioPackageEvidenceIntakeEntry {
        check_id: check.check_id.clone(),
        source_status: check.status,
        evidence: check.evidence.clone(),
        required_for_studio,
        decision,
        next_required_action: package_evidence_next_action(decision).to_string(),
        issue_code: (check.status == StudioValidationStatus::Fail)
            .then(|| "studio.issue.package_evidence_source_check_failed".to_string()),
    }
}

fn missing_package_evidence_intake_entry(check_id: &str) -> StudioPackageEvidenceIntakeEntry {
    StudioPackageEvidenceIntakeEntry {
        check_id: check_id.to_string(),
        source_status: StudioValidationStatus::Fail,
        evidence: "required source check missing".to_string(),
        required_for_studio: true,
        decision: StudioPackageEvidenceIntakeDecision::BlockedByMissingCheck,
        next_required_action: package_evidence_next_action(
            StudioPackageEvidenceIntakeDecision::BlockedByMissingCheck,
        )
        .to_string(),
        issue_code: Some("studio.issue.package_evidence_required_check_missing".to_string()),
    }
}

fn package_evidence_next_action(decision: StudioPackageEvidenceIntakeDecision) -> &'static str {
    match decision {
        StudioPackageEvidenceIntakeDecision::Ready => "review_package_in_studio",
        StudioPackageEvidenceIntakeDecision::Observed => "observe_nonblocking_package_evidence",
        StudioPackageEvidenceIntakeDecision::BlockedByMissingCheck => {
            "rerun_manifold_package_validation"
        }
        StudioPackageEvidenceIntakeDecision::BlockedByFailedCheck => {
            "repair_manifold_package_evidence"
        }
    }
}

fn package_evidence_intake_prohibited_actions() -> Vec<String> {
    [
        "build",
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
        "start_runtime_package",
    ]
    .iter()
    .map(|action| action.to_string())
    .collect()
}

pub fn projected_motion_breath_shell_handoff_review_for_evidence(
    evidence: &Value,
    evidence_path: Option<&Path>,
) -> StudioProjectedMotionBreathShellHandoffReviewReport {
    let source_evidence_schema = json_string(evidence, "$schema");
    let target_package_id = nested_json_string(evidence, "package", "package_id");
    let handoff_id = nested_json_string(evidence, "shell_handoff", "handoff_id");
    let target_host_profile = nested_json_string(evidence, "shell_handoff", "target_host_profile");
    let shell_app_id = nested_json_string(evidence, "shell_handoff", "shell_app_id");
    let command_ids = nested_json_string_array(evidence, "shell_handoff", "command_ids");
    let exported_stream_ids =
        nested_json_string_array(evidence, "package_contract", "exported_stream_ids");
    let feedback_sink_streams = nested_json_string_array(
        evidence,
        "package_contract",
        "feedback_sink_provides_streams",
    );
    let binding_pairs = projected_motion_breath_shell_binding_pairs(evidence);
    let required_bindings = projected_motion_breath_shell_required_bindings();
    let ready_required_binding_count = required_bindings
        .iter()
        .filter(|binding| binding_pairs.contains(*binding))
        .count();
    let transport_ids = projected_motion_breath_shell_transport_ids(evidence);
    let runtime_execution_performed =
        nested_json_bool(evidence, "execution", "runtime_execution_performed").unwrap_or(true);
    let platform_execution_performed =
        nested_json_bool(evidence, "execution", "platform_execution_performed").unwrap_or(true);
    let broker_transport_used =
        nested_json_bool(evidence, "execution", "broker_transport_used").unwrap_or(true);
    let downstream_shell_runtime_used =
        nested_json_bool(evidence, "execution", "downstream_shell_runtime_used").unwrap_or(true);
    let legacy_app_dependency_used =
        nested_json_bool(evidence, "execution", "legacy_app_dependency_used").unwrap_or(true);
    let legacy_rusty_xr_repo_used =
        nested_json_bool(evidence, "execution", "legacy_rusty_xr_repo_used").unwrap_or(true);
    let feedback_receipt_exported = exported_stream_ids
        .iter()
        .any(|stream_id| stream_id == "stream.breath.feedback_receipt");
    let feedback_sink_provides_receipt = feedback_sink_streams
        .iter()
        .any(|stream_id| stream_id == "stream.breath.feedback_receipt");
    let clean_execution_boundary = !runtime_execution_performed
        && !platform_execution_performed
        && !broker_transport_used
        && !downstream_shell_runtime_used
        && !legacy_app_dependency_used
        && !legacy_rusty_xr_repo_used;

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.source_schema",
        source_evidence_schema.as_deref()
            == Some("rusty.hostess.projected_motion_breath.shell_handoff_validation_evidence.v1"),
        "source Hostess shell handoff evidence schema is supported",
        "source Hostess shell handoff evidence schema is unsupported",
        "studio.issue.projected_motion_breath_shell_handoff_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.source_path",
        evidence_path.is_some(),
        "source Hostess shell handoff evidence has a durable path",
        "source Hostess shell handoff evidence path is missing",
        "studio.issue.projected_motion_breath_shell_handoff_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.source_status",
        evidence.get("status").and_then(Value::as_str) == Some("pass")
            && evidence
                .get("scorecard")
                .and_then(|scorecard| scorecard.get("status"))
                .and_then(Value::as_str)
                == Some("pass"),
        "source Hostess shell handoff evidence and scorecard passed",
        "source Hostess shell handoff evidence or scorecard failed",
        "studio.issue.projected_motion_breath_shell_handoff_source_failed",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.target_package",
        target_package_id.as_deref() == Some(PROJECTED_MOTION_BREATH_PACKAGE_ID),
        "source evidence targets projected-motion breath",
        "source evidence targets a different package",
        "studio.issue.projected_motion_breath_shell_handoff_package_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.handoff_id",
        handoff_id.as_deref().is_some_and(is_dotted_id),
        "source evidence declares a dotted shell handoff id",
        "source evidence is missing a dotted shell handoff id",
        "studio.issue.projected_motion_breath_shell_handoff_id",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.required_bindings",
        ready_required_binding_count == required_bindings.len(),
        "source evidence includes controller pose publish, feedback subscribe, and receipt publish bindings",
        "source evidence is missing one or more required PMB shell bindings",
        "studio.issue.projected_motion_breath_shell_handoff_required_bindings",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.feedback_receipt_export",
        feedback_receipt_exported && feedback_sink_provides_receipt,
        "source evidence proves feedback receipt export and feedback sink provisioning",
        "source evidence does not prove feedback receipt export and feedback sink provisioning",
        "studio.issue.projected_motion_breath_shell_handoff_feedback_receipt",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.command_status",
        command_ids
            .iter()
            .any(|command_id| command_id == "command.breath.status"),
        "source evidence exposes command.breath.status for read-only handoff checks",
        "source evidence does not expose command.breath.status",
        "studio.issue.projected_motion_breath_shell_handoff_command_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.transport_offer",
        !transport_ids.is_empty(),
        "source evidence includes a named transport offer for downstream shell wiring",
        "source evidence does not include a named transport offer",
        "studio.issue.projected_motion_breath_shell_handoff_transport_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_shell_handoff.authority_boundary",
        clean_execution_boundary,
        "Studio review preserves Hostess runtime evidence ownership and avoids shell execution",
        "source evidence indicates runtime, transport, downstream shell, or legacy repo execution",
        "studio.issue.projected_motion_breath_shell_handoff_authority_mismatch",
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.projected_motion_breath_shell_handoff_source_schema")
                    | Some("studio.issue.projected_motion_breath_shell_handoff_package_mismatch")
            )
    });
    let status = if has_rejected_check {
        StudioProjectedMotionBreathShellHandoffReviewStatus::Rejected
    } else if has_failed_check {
        StudioProjectedMotionBreathShellHandoffReviewStatus::Blocked
    } else {
        StudioProjectedMotionBreathShellHandoffReviewStatus::Ready
    };
    let issue_code = match status {
        StudioProjectedMotionBreathShellHandoffReviewStatus::Ready => None,
        StudioProjectedMotionBreathShellHandoffReviewStatus::Blocked
        | StudioProjectedMotionBreathShellHandoffReviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioProjectedMotionBreathShellHandoffReviewReport {
        schema_id: PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA.to_string(),
        source_evidence_schema,
        source_evidence_path: evidence_path.map(|path| path.display().to_string()),
        target_package_id,
        handoff_id,
        target_host_profile,
        shell_app_id,
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        runtime_authority: "rusty.manifold".to_string(),
        authoring_authority: "rusty.studio".to_string(),
        platform_validation_authority: "rusty.hostess".to_string(),
        runtime_execution_performed,
        platform_execution_performed,
        broker_transport_used,
        downstream_shell_runtime_used,
        legacy_app_dependency_used,
        required_binding_count: required_bindings.len(),
        ready_required_binding_count,
        stream_bindings: binding_pairs
            .iter()
            .map(|(stream_id, direction)| format!("{stream_id}:{direction}"))
            .collect(),
        command_ids,
        transport_ids,
        feedback_receipt_exported,
        feedback_sink_provides_receipt,
        proposal_kind: "review_shell_handoff_for_hostess_owner_execution".to_string(),
        prohibited_actions: projected_motion_breath_shell_handoff_review_prohibited_actions(),
        checks,
    }
}

fn projected_motion_breath_shell_required_bindings() -> BTreeSet<(String, String)> {
    [
        ("stream.motion.object_pose", "publish"),
        ("stream.breath.feedback_state", "subscribe"),
        ("stream.breath.feedback_receipt", "publish"),
    ]
    .iter()
    .map(|(stream_id, direction)| (stream_id.to_string(), direction.to_string()))
    .collect()
}

fn projected_motion_breath_shell_binding_pairs(evidence: &Value) -> BTreeSet<(String, String)> {
    let mut bindings = BTreeSet::new();
    if let Some(shell_handoff) = evidence.get("shell_handoff") {
        for field in ["binding_pairs", "stream_bindings"] {
            if let Some(values) = shell_handoff.get(field).and_then(Value::as_array) {
                for value in values {
                    if let (Some(stream_id), Some(direction)) = (
                        value.get("stream_id").and_then(Value::as_str),
                        value.get("direction").and_then(Value::as_str),
                    ) {
                        bindings.insert((stream_id.to_string(), direction.to_string()));
                    }
                }
            }
        }
    }
    bindings
}

fn projected_motion_breath_shell_transport_ids(evidence: &Value) -> Vec<String> {
    evidence
        .get("shell_handoff")
        .and_then(|shell_handoff| shell_handoff.get("transport_offers"))
        .and_then(Value::as_array)
        .map(|offers| {
            offers
                .iter()
                .filter_map(|offer| offer.get("transport_id").and_then(Value::as_str))
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn projected_motion_breath_shell_handoff_review_prohibited_actions() -> Vec<String> {
    [
        "build",
        "install",
        "launch",
        "stage_shell_files",
        "launch_downstream_shell",
        "open_command_session",
        "collect_device_evidence",
        "start_runtime_package",
    ]
    .iter()
    .map(|action| action.to_string())
    .collect()
}

pub fn projected_motion_breath_authoring_review_for_intake(
    intake: &StudioPackageEvidenceIntakeReport,
    intake_path: Option<&Path>,
    profile: &Value,
    profile_path: Option<&Path>,
) -> StudioProjectedMotionBreathAuthoringReviewReport {
    let required_package_checks =
        projected_motion_breath_required_check_ids(PROJECTED_MOTION_BREATH_PACKAGE_ID);
    let source_profile_schema = json_string(profile, "$schema");
    let profile_id = json_string(profile, "profile_id");
    let target_module_id = json_string(profile, "target_module_id");
    let input_kinds = json_string_array(profile, "input_kinds");
    let projection_mode = nested_json_string(profile, "projection", "mode");
    let fallback_projection_mode = nested_json_string(profile, "projection", "fallback_mode");
    let required_package_checks_ready = required_package_checks.iter().all(|required_check_id| {
        intake.entries.iter().any(|entry| {
            entry.check_id == *required_check_id
                && entry.required_for_studio
                && entry.decision == StudioPackageEvidenceIntakeDecision::Ready
        })
    });
    let authority_preserved = intake.runtime_authority == "rusty.manifold"
        && intake.authoring_authority == "rusty.studio"
        && intake.platform_validation_authority == "rusty.hostess"
        && !intake.runtime_execution_performed
        && !intake.platform_execution_performed;

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.source_intake_schema",
        intake.schema_id == PACKAGE_EVIDENCE_INTAKE_REPORT_SCHEMA,
        "source package evidence intake schema is supported",
        "source package evidence intake schema is unsupported",
        "studio.issue.package_evidence_intake_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.source_intake_path",
        intake_path.is_some(),
        "source package evidence intake has a durable path",
        "source package evidence intake path is missing",
        "studio.issue.projected_motion_breath_intake_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.package_evidence_ready",
        intake.status == StudioPackageEvidenceIntakeStatus::Ready,
        "source package evidence intake is ready",
        "source package evidence intake is blocked or rejected",
        intake
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.package_evidence_intake_not_ready"),
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.target_package",
        intake.target_package_id == PROJECTED_MOTION_BREATH_PACKAGE_ID,
        "source package evidence targets projected-motion breath",
        "source package evidence targets a different package",
        "studio.issue.projected_motion_breath_package_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.required_package_checks_ready",
        required_package_checks_ready,
        "all projected-motion breath package evidence checks are ready",
        "one or more projected-motion breath package evidence checks are not ready",
        "studio.issue.projected_motion_breath_package_evidence_not_ready",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.authority_boundary",
        authority_preserved,
        "Studio review preserves Manifold, Studio, and Hostess authorities",
        "Studio review authority boundary is not preserved",
        "studio.issue.projected_motion_breath_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_path",
        profile_path.is_some(),
        "source motion-breath profile has a durable path",
        "source motion-breath profile path is missing",
        "studio.issue.projected_motion_breath_profile_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_schema",
        source_profile_schema.as_deref() == Some(MOTION_BREATH_PROFILE_SCHEMA),
        "source profile schema is supported",
        "source profile schema is unsupported",
        "studio.issue.motion_breath_profile_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_id",
        profile_id.as_deref().is_some_and(is_dotted_id),
        "profile id uses dotted-id grammar",
        "profile id is missing or not a dotted id",
        "studio.issue.motion_breath_profile_id",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_target_module",
        target_module_id.as_deref() == Some("module.breath.projected_motion"),
        "profile targets the projected-motion breath module",
        "profile does not target the projected-motion breath module",
        "studio.issue.motion_breath_profile_target_module",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_input_kinds",
        ["pose", "vector3"]
            .iter()
            .all(|required| input_kinds.iter().any(|kind| kind == required)),
        "profile declares pose and vector3 input kinds",
        "profile does not declare both pose and vector3 input kinds",
        "studio.issue.motion_breath_profile_input_kinds",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.profile_projection",
        projection_mode.is_some(),
        "profile declares a projection mode for review",
        "profile does not declare a projection mode",
        "studio.issue.motion_breath_profile_projection",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_authoring.execution_policy",
        true,
        "Studio projected-motion breath authoring review is proposal-only and not executed",
        "Studio projected-motion breath authoring review attempted execution",
        "studio.issue.projected_motion_breath_authoring_execution_policy",
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.package_evidence_intake_schema")
                    | Some("studio.issue.motion_breath_profile_schema")
                    | Some("studio.issue.projected_motion_breath_package_mismatch")
            )
    });
    let status = if has_rejected_check {
        StudioProjectedMotionBreathAuthoringReviewStatus::Rejected
    } else if has_failed_check {
        StudioProjectedMotionBreathAuthoringReviewStatus::Blocked
    } else {
        StudioProjectedMotionBreathAuthoringReviewStatus::Ready
    };
    let issue_code = match status {
        StudioProjectedMotionBreathAuthoringReviewStatus::Ready => None,
        StudioProjectedMotionBreathAuthoringReviewStatus::Blocked
        | StudioProjectedMotionBreathAuthoringReviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioProjectedMotionBreathAuthoringReviewReport {
        schema_id: PROJECTED_MOTION_BREATH_AUTHORING_REVIEW_SCHEMA.to_string(),
        source_intake_schema: intake.schema_id.clone(),
        source_intake_path: intake_path.map(|path| path.display().to_string()),
        source_profile_schema,
        source_profile_path: profile_path.map(|path| path.display().to_string()),
        target_package_id: PROJECTED_MOTION_BREATH_PACKAGE_ID.to_string(),
        target_module_id,
        profile_id,
        status,
        issue_code,
        execution_policy: "not_executed.proposal_only".to_string(),
        runtime_authority: "rusty.manifold".to_string(),
        authoring_authority: "rusty.studio".to_string(),
        platform_validation_authority: "rusty.hostess".to_string(),
        runtime_execution_performed: false,
        platform_execution_performed: false,
        package_evidence_status: intake.status,
        package_required_check_count: intake.required_check_count,
        package_ready_required_check_count: intake.ready_required_check_count,
        package_blocked_required_check_count: intake.blocked_required_check_count,
        input_kinds,
        projection_mode,
        fallback_projection_mode,
        proposed_command_id: "command.breath.set_profile".to_string(),
        proposed_profile_operation: "propose_profile_for_runtime_owner_review".to_string(),
        required_package_checks,
        prohibited_actions: package_evidence_intake_prohibited_actions(),
        checks,
    }
}

pub fn projected_motion_breath_source_adapter_selection_review_for_authoring(
    authoring_review: &StudioProjectedMotionBreathAuthoringReviewReport,
    authoring_review_path: Option<&Path>,
    source_descriptors: &Value,
    source_descriptors_path: Option<&Path>,
    selected_adapter_id: &str,
) -> StudioProjectedMotionBreathSourceAdapterSelectionReviewReport {
    let descriptors = source_adapter_descriptor_array(source_descriptors);
    let selected_descriptor =
        find_source_adapter_descriptor(source_descriptors, selected_adapter_id);
    let selected_source_kind =
        selected_descriptor.and_then(|value| json_string(value, "source_kind"));
    let selected_input_kind =
        selected_descriptor.and_then(|value| json_string(value, "input_kind"));
    let selected_output_stream_id =
        selected_descriptor.and_then(|value| json_string(value, "output_stream_id"));
    let descriptor_schema = json_string(source_descriptors, "$schema");
    let descriptor_target_module = json_string(source_descriptors, "target_module_id");
    let selected_input_supported = selected_input_kind.as_ref().is_some_and(|kind| {
        authoring_review
            .input_kinds
            .iter()
            .any(|input| input == kind)
    });
    let selected_stream_supported = matches!(
        (
            selected_input_kind.as_deref(),
            selected_output_stream_id.as_deref()
        ),
        (Some("pose"), Some("stream.motion.object_pose"))
            | (Some("vector3"), Some("stream.motion.vector3"))
    );
    let descriptor_source_clean = source_descriptors
        .get("runtime_execution_performed")
        .and_then(Value::as_bool)
        == Some(false)
        && source_descriptors
            .get("platform_execution_performed")
            .and_then(Value::as_bool)
            == Some(false)
        && source_descriptors
            .get("device_required")
            .and_then(Value::as_bool)
            == Some(false);
    let selected_descriptor_clean = selected_descriptor.is_some_and(|descriptor| {
        descriptor.get("transport_kind").and_then(Value::as_str) == Some("descriptor_only")
            && descriptor
                .get("requires_platform_sdk")
                .and_then(Value::as_bool)
                == Some(false)
            && descriptor
                .get("requires_device_api")
                .and_then(Value::as_bool)
                == Some(false)
            && descriptor
                .get("runtime_adapter_included")
                .and_then(Value::as_bool)
                == Some(false)
    });
    let authority_preserved = authoring_review.runtime_authority == "rusty.manifold"
        && authoring_review.authoring_authority == "rusty.studio"
        && authoring_review.platform_validation_authority == "rusty.hostess"
        && !authoring_review.runtime_execution_performed
        && !authoring_review.platform_execution_performed;

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.authoring_schema",
        authoring_review.schema_id == PROJECTED_MOTION_BREATH_AUTHORING_REVIEW_SCHEMA,
        "source authoring review schema is supported",
        "source authoring review schema is unsupported",
        "studio.issue.projected_motion_breath_authoring_review_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.authoring_ready",
        authoring_review.status == StudioProjectedMotionBreathAuthoringReviewStatus::Ready,
        "source authoring review is ready",
        "source authoring review is blocked or rejected",
        authoring_review
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.projected_motion_breath_authoring_review_not_ready"),
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.descriptor_schema",
        descriptor_schema.as_deref()
            == Some(PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_DESCRIPTOR_SCHEMA),
        "source adapter descriptor schema is supported",
        "source adapter descriptor schema is unsupported",
        "studio.issue.projected_motion_breath_source_adapter_descriptor_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.target_contract",
        source_descriptors.get("package_id").and_then(Value::as_str)
            == Some(PROJECTED_MOTION_BREATH_PACKAGE_ID)
            && descriptor_target_module.as_deref() == Some("module.breath.projected_motion")
            && authoring_review.target_package_id == PROJECTED_MOTION_BREATH_PACKAGE_ID
            && authoring_review.target_module_id.as_deref()
                == Some("module.breath.projected_motion"),
        "source adapter descriptors target projected-motion breath",
        "source adapter descriptor target package or module drifted",
        "studio.issue.projected_motion_breath_source_adapter_target",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.selected_adapter_id",
        is_dotted_id(selected_adapter_id),
        "selected source adapter id uses dotted-id grammar",
        "selected source adapter id is not a dotted id",
        "studio.issue.projected_motion_breath_source_adapter_id",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.selected_adapter_present",
        selected_descriptor.is_some(),
        "selected source adapter descriptor is present",
        "selected source adapter descriptor is missing",
        "studio.issue.projected_motion_breath_source_adapter_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.input_kind",
        selected_input_supported,
        "selected source adapter input kind is supported by the profile intent",
        "selected source adapter input kind is not supported by the profile intent",
        "studio.issue.projected_motion_breath_source_adapter_input_kind",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.stream_binding",
        selected_stream_supported,
        "selected source adapter maps to a supported pose/vector stream",
        "selected source adapter stream does not match its input kind",
        "studio.issue.projected_motion_breath_source_adapter_stream",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_source_adapter_selection.non_executing",
        descriptor_source_clean && selected_descriptor_clean && authority_preserved,
        "source adapter selection is descriptor-only and preserves authority boundaries",
        "source adapter selection attempted runtime, platform, device, or authority drift",
        "studio.issue.projected_motion_breath_source_adapter_execution_policy",
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.projected_motion_breath_source_adapter_descriptor_schema")
                    | Some("studio.issue.projected_motion_breath_source_adapter_target")
            )
    });
    let status = if has_rejected_check {
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Rejected
    } else if has_failed_check {
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Blocked
    } else {
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Ready
    };
    let issue_code = match status {
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Ready => None,
        StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Blocked
        | StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioProjectedMotionBreathSourceAdapterSelectionReviewReport {
        schema_id: PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_SELECTION_REVIEW_SCHEMA.to_string(),
        source_authoring_review_schema: authoring_review.schema_id.clone(),
        source_authoring_review_path: authoring_review_path.map(|path| path.display().to_string()),
        source_descriptor_schema: descriptor_schema,
        source_descriptor_path: source_descriptors_path.map(|path| path.display().to_string()),
        target_package_id: PROJECTED_MOTION_BREATH_PACKAGE_ID.to_string(),
        target_module_id: authoring_review.target_module_id.clone(),
        profile_id: authoring_review.profile_id.clone(),
        selected_adapter_id: selected_adapter_id.to_string(),
        selected_source_kind,
        selected_input_kind,
        selected_output_stream_id,
        status,
        issue_code,
        execution_policy: "not_executed.proposal_only".to_string(),
        runtime_authority: "rusty.manifold".to_string(),
        authoring_authority: "rusty.studio".to_string(),
        platform_validation_authority: "rusty.hostess".to_string(),
        runtime_execution_performed: false,
        platform_execution_performed: false,
        source_authoring_review_status: authoring_review.status,
        source_descriptor_count: descriptors.len(),
        matching_descriptor_count: if selected_descriptor.is_some() { 1 } else { 0 },
        proposal_kind: "propose_source_adapter_for_runtime_owner_review".to_string(),
        prohibited_actions: package_evidence_intake_prohibited_actions(),
        checks,
    }
}

pub fn projected_motion_breath_adapter_normalization_evidence_review_for_selection(
    selection_review: &StudioProjectedMotionBreathSourceAdapterSelectionReviewReport,
    selection_review_path: Option<&Path>,
    package_report: &StudioManifoldPackageValidationReport,
    package_report_path: Option<&Path>,
    source_binding: &Value,
    source_binding_path: Option<&Path>,
    normalization_case: &Value,
    normalization_case_path: Option<&Path>,
) -> StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewReport {
    let adapter_normalization_check_id = projected_motion_breath_adapter_normalization_check_id();
    let adapter_normalization_check = package_report
        .checks
        .iter()
        .find(|check| check.check_id == adapter_normalization_check_id);
    let adapter_normalization_check_status = adapter_normalization_check.map(|check| check.status);
    let source_binding_schema = json_string(source_binding, "$schema");
    let source_normalization_case_schema = json_string(normalization_case, "$schema");
    let binding_id = json_string(source_binding, "binding_id");
    let normalization_case_id = json_string(normalization_case, "case_id");
    let source_payload_kind = json_string(normalization_case, "source_payload_kind");
    let expected_sample_kind = json_string(normalization_case, "expected_sample_kind");
    let binding_selected_adapter_id = json_string(source_binding, "selected_adapter_id");
    let binding_selected_input_kind = json_string(source_binding, "selected_input_kind");
    let binding_selected_output_stream_id =
        json_string(source_binding, "selected_output_stream_id");
    let normalization_binding_path = json_string(normalization_case, "binding_path");

    let source_binding_selected_adapter_match = binding_selected_adapter_id.as_deref()
        == Some(selection_review.selected_adapter_id.as_str());
    let source_binding_stream_match = binding_selected_input_kind
        == selection_review.selected_input_kind
        && binding_selected_output_stream_id == selection_review.selected_output_stream_id;
    let source_binding_target_match = source_binding.get("package_id").and_then(Value::as_str)
        == Some(PROJECTED_MOTION_BREATH_PACKAGE_ID)
        && source_binding
            .get("target_module_id")
            .and_then(Value::as_str)
            == Some(PROJECTED_MOTION_BREATH_MODULE_ID)
        && source_binding.get("profile_id").and_then(Value::as_str)
            == selection_review.profile_id.as_deref();
    let source_binding_path_match = source_binding_path.is_some_and(|path| {
        normalization_binding_path
            .as_deref()
            .is_some_and(|binding_path| path_matches_reference_suffix(path, binding_path))
    });
    let normalization_payload_matches = adapter_normalization_payload_matches(
        selection_review.selected_source_kind.as_deref(),
        source_payload_kind.as_deref(),
        expected_sample_kind.as_deref(),
    );
    let deterministic_normalization_evidence = source_binding_path_match
        && normalization_payload_matches
        && source_binding_selected_adapter_match
        && source_binding_stream_match;
    let selection_authority_preserved = selection_review.runtime_authority == "rusty.manifold"
        && selection_review.authoring_authority == "rusty.studio"
        && selection_review.platform_validation_authority == "rusty.hostess"
        && !selection_review.runtime_execution_performed
        && !selection_review.platform_execution_performed;
    let source_binding_clean = source_binding
        .get("execution_policy")
        .and_then(Value::as_str)
        == Some("not_executed.schema_binding_only")
        && source_binding
            .get("runtime_execution_performed")
            .and_then(Value::as_bool)
            == Some(false)
        && source_binding
            .get("platform_execution_performed")
            .and_then(Value::as_bool)
            == Some(false)
        && source_binding
            .get("device_required")
            .and_then(Value::as_bool)
            == Some(false);
    let normalization_case_clean = normalization_case
        .get("execution_policy")
        .and_then(Value::as_str)
        == Some("not_executed.fixture_normalization_only")
        && normalization_case
            .get("runtime_execution_performed")
            .and_then(Value::as_bool)
            == Some(false)
        && normalization_case
            .get("platform_execution_performed")
            .and_then(Value::as_bool)
            == Some(false)
        && normalization_case
            .get("device_required")
            .and_then(Value::as_bool)
            == Some(false);

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.selection_schema",
        selection_review.schema_id
            == PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_SELECTION_REVIEW_SCHEMA,
        "source adapter selection review schema is supported",
        "source adapter selection review schema is unsupported",
        "studio.issue.projected_motion_breath_source_adapter_selection_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.selection_ready",
        selection_review.status
            == StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Ready,
        "source adapter selection review is ready",
        "source adapter selection review is blocked or rejected",
        selection_review
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.projected_motion_breath_source_adapter_selection_not_ready"),
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.package_report_schema",
        package_report.schema_id == MANIFOLD_PACKAGE_VALIDATION_REPORT_SCHEMA,
        "source Manifold package validation report schema is supported",
        "source Manifold package validation report schema is unsupported",
        "studio.issue.package_evidence_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.package_check_present",
        adapter_normalization_check.is_some(),
        "source package report includes adapter-normalization evidence",
        "source package report is missing adapter-normalization evidence",
        "studio.issue.projected_motion_breath_adapter_normalization_check_missing",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.package_check_pass",
        adapter_normalization_check_status == Some(StudioValidationStatus::Pass),
        "source package adapter-normalization evidence passed",
        "source package adapter-normalization evidence did not pass",
        "studio.issue.projected_motion_breath_adapter_normalization_check_failed",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.source_binding_schema",
        source_binding_schema.as_deref() == Some(PROJECTED_MOTION_BREATH_SOURCE_BINDING_SCHEMA),
        "source binding schema is supported",
        "source binding schema is unsupported",
        "studio.issue.projected_motion_breath_source_binding_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.source_binding_target",
        source_binding_target_match
            && selection_review.target_package_id == PROJECTED_MOTION_BREATH_PACKAGE_ID
            && selection_review.target_module_id.as_deref()
                == Some(PROJECTED_MOTION_BREATH_MODULE_ID),
        "source binding targets the selected projected-motion breath contract",
        "source binding target package, module, or profile drifted",
        "studio.issue.projected_motion_breath_source_binding_target",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.source_binding_adapter",
        source_binding_selected_adapter_match,
        "source binding selected adapter matches Studio selection",
        "source binding selected adapter differs from Studio selection",
        "studio.issue.projected_motion_breath_adapter_normalization_binding_adapter",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.source_binding_stream",
        source_binding_stream_match,
        "source binding stream matches selected input kind and output stream",
        "source binding stream differs from selected input kind or output stream",
        "studio.issue.projected_motion_breath_adapter_normalization_binding_stream",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.case_schema",
        source_normalization_case_schema.as_deref()
            == Some(PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_CASE_SCHEMA),
        "adapter-normalization case schema is supported",
        "adapter-normalization case schema is unsupported",
        "studio.issue.projected_motion_breath_adapter_normalization_case_schema",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.case_binding_path",
        source_binding_path_match,
        "adapter-normalization case points at the selected source binding",
        "adapter-normalization case does not point at the selected source binding",
        "studio.issue.projected_motion_breath_adapter_normalization_binding_path",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.payload_kind",
        normalization_payload_matches,
        "adapter-normalization case payload kind matches the selected source kind",
        "adapter-normalization case payload kind does not match the selected source kind",
        "studio.issue.projected_motion_breath_adapter_normalization_payload_kind",
    );
    push_check(
        &mut checks,
        "studio.check.projected_motion_breath_adapter_normalization.non_executing",
        selection_authority_preserved && source_binding_clean && normalization_case_clean,
        "adapter-normalization evidence is schema-only and preserves authority boundaries",
        "adapter-normalization evidence attempted runtime, platform, device, or authority drift",
        "studio.issue.projected_motion_breath_adapter_normalization_execution_policy",
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check =
        checks.iter().any(|check| {
            check.status == StudioValidationStatus::Fail
                && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.projected_motion_breath_source_adapter_selection_schema")
                    | Some("studio.issue.package_evidence_source_schema")
                    | Some("studio.issue.projected_motion_breath_source_binding_schema")
                    | Some("studio.issue.projected_motion_breath_source_binding_target")
                    | Some("studio.issue.projected_motion_breath_adapter_normalization_case_schema")
            )
        });
    let status = if has_rejected_check {
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Rejected
    } else if has_failed_check {
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Blocked
    } else {
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Ready
    };
    let issue_code = match status {
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Ready => None,
        StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Blocked
        | StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewReport {
        schema_id: PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_EVIDENCE_REVIEW_SCHEMA.to_string(),
        source_selection_review_schema: selection_review.schema_id.clone(),
        source_selection_review_path: selection_review_path.map(|path| path.display().to_string()),
        source_package_report_schema: package_report.schema_id.clone(),
        source_package_report_path: package_report_path.map(|path| path.display().to_string()),
        source_binding_schema,
        source_binding_path: source_binding_path.map(|path| path.display().to_string()),
        source_normalization_case_schema,
        source_normalization_case_path: normalization_case_path
            .map(|path| path.display().to_string()),
        target_package_id: PROJECTED_MOTION_BREATH_PACKAGE_ID.to_string(),
        target_module_id: selection_review.target_module_id.clone(),
        profile_id: selection_review.profile_id.clone(),
        selected_adapter_id: selection_review.selected_adapter_id.clone(),
        selected_source_kind: selection_review.selected_source_kind.clone(),
        selected_input_kind: selection_review.selected_input_kind.clone(),
        selected_output_stream_id: selection_review.selected_output_stream_id.clone(),
        binding_id,
        normalization_case_id,
        source_payload_kind,
        expected_sample_kind,
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        runtime_authority: "rusty.manifold".to_string(),
        authoring_authority: "rusty.studio".to_string(),
        platform_validation_authority: "rusty.hostess".to_string(),
        runtime_execution_performed: false,
        platform_execution_performed: false,
        source_selection_status: selection_review.status,
        adapter_normalization_check_id,
        adapter_normalization_check_status,
        source_binding_selected_adapter_match,
        deterministic_normalization_evidence,
        proposal_kind: "review_adapter_normalization_for_runtime_owner".to_string(),
        prohibited_actions: package_evidence_intake_prohibited_actions(),
        checks,
    }
}

fn projected_motion_breath_adapter_normalization_check_id() -> String {
    format!(
        "validation.package.{PROJECTED_MOTION_BREATH_PACKAGE_ID}.{PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_CHECK_SUFFIX}"
    )
}

fn adapter_normalization_payload_matches(
    selected_source_kind: Option<&str>,
    source_payload_kind: Option<&str>,
    expected_sample_kind: Option<&str>,
) -> bool {
    matches!(
        (
            selected_source_kind,
            source_payload_kind,
            expected_sample_kind
        ),
        (
            Some("object_pose"),
            Some("object_pose"),
            Some("rigid_motion")
        ) | (
            Some("xr_controller_pose"),
            Some("object_pose"),
            Some("rigid_motion")
        ) | (
            Some("vector_motion"),
            Some("vector_motion"),
            Some("vector_motion")
        ) | (
            Some("wearable_acceleration"),
            Some("vector_motion"),
            Some("vector_motion")
        ) | (
            Some("external_patch_stream_bridge"),
            Some("external_patch_channels"),
            Some("vector_motion")
        )
    )
}

fn path_matches_reference_suffix(actual_path: &Path, reference_suffix: &str) -> bool {
    let actual = actual_path.display().to_string().replace('\\', "/");
    let expected = reference_suffix.replace('\\', "/");
    actual.ends_with(&expected)
        || actual_path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|file_name| expected.ends_with(file_name))
}

fn source_adapter_descriptor_array(document: &Value) -> Vec<&Value> {
    document
        .get("source_adapters")
        .and_then(Value::as_array)
        .map(|values| values.iter().collect())
        .unwrap_or_default()
}

fn find_source_adapter_descriptor<'a>(
    document: &'a Value,
    selected_adapter_id: &str,
) -> Option<&'a Value> {
    source_adapter_descriptor_array(document)
        .into_iter()
        .find(|descriptor| {
            descriptor.get("adapter_id").and_then(Value::as_str) == Some(selected_adapter_id)
        })
}

fn json_string(document: &Value, field: &str) -> Option<String> {
    document
        .get(field)
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn nested_json_string(document: &Value, object_field: &str, field: &str) -> Option<String> {
    document
        .get(object_field)
        .and_then(|value| value.get(field))
        .and_then(Value::as_str)
        .map(str::to_string)
}

fn nested_json_bool(document: &Value, object_field: &str, field: &str) -> Option<bool> {
    document
        .get(object_field)
        .and_then(|value| value.get(field))
        .and_then(Value::as_bool)
}

fn json_string_array(document: &Value, field: &str) -> Vec<String> {
    document
        .get(field)
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn nested_json_string_array(document: &Value, object_field: &str, field: &str) -> Vec<String> {
    document
        .get(object_field)
        .and_then(|value| value.get(field))
        .and_then(Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

pub fn shell_hostess_owner_intake_for_handoff_package(
    package: &StudioShellHostessHandoffPackageReport,
    package_path: Option<&Path>,
) -> StudioShellHostessOwnerIntakeReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.source_package_schema",
        package.schema_id == SHELL_HOSTESS_HANDOFF_PACKAGE_SCHEMA,
        "source Hostess handoff package schema is supported",
        "source Hostess handoff package schema is unsupported",
        "studio.issue.shell_hostess_handoff_package_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.package_path",
        package_path.is_some(),
        "source Hostess handoff package has a durable path",
        "source Hostess handoff package path is missing",
        "studio.issue.shell_hostess_owner_intake_package_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.package_ready",
        package.status == StudioShellHostessHandoffPackageStatus::Ready,
        "source Hostess handoff package is ready",
        "source Hostess handoff package is not ready",
        package
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_handoff_package_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.package_execution_policy",
        package.execution_policy == "not_executed.review_only",
        "source package is a review-only Studio artifact",
        "source package execution policy is not review-only",
        "studio.issue.shell_hostess_handoff_package_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.candidate_selected",
        package.selected_candidate_id.is_some() && package.candidate_id.is_some(),
        "source package names a selected release candidate",
        "source package does not name a selected release candidate",
        "studio.issue.shell_hostess_owner_intake_candidate_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.candidate_manifest_path",
        package.candidate_manifest_path.is_some(),
        "source package names a candidate identity manifest",
        "source package does not name a candidate identity manifest",
        "studio.issue.shell_hostess_owner_intake_candidate_manifest_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.review_path",
        package.review_path.is_some(),
        "source package names a release-candidate review artifact",
        "source package does not name a release-candidate review artifact",
        "studio.issue.shell_hostess_owner_intake_review_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.handoff_manifest_path",
        package.handoff_manifest_path.is_some(),
        "source package names a shell handoff manifest",
        "source package does not name a shell handoff manifest",
        "studio.issue.shell_hostess_owner_intake_handoff_manifest_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.handoff_manifest_schema",
        package.handoff_manifest_schema.as_deref() == Some(SHELL_HANDOFF_MANIFEST_SCHEMA),
        "source handoff manifest schema is supported",
        "source handoff manifest schema is unsupported or unavailable",
        "studio.issue.shell_handoff_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.runtime_command_authority",
        package.command_session_authority.as_deref() == Some("rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.runtime_host_authority",
        package.install_launch_evidence_authority.as_deref() == Some("rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.studio_role",
        package.studio_role.as_deref() == Some("authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.source_package_checks_pass",
        package
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "source Hostess handoff package checks all pass",
        "source Hostess handoff package contains failed checks",
        "studio.issue.shell_hostess_handoff_package_failed_check",
    );

    for action_id in [
        "hostess.review_release_candidate",
        "hostess.stage_generated_shells",
        "manifold.review_command_session_contract",
        "hostess.collect_install_launch_evidence",
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_owner_intake.has_{action_id}"),
            package
                .required_owner_actions
                .iter()
                .any(|action| action.action_id == action_id),
            "source package includes this required owner action",
            "source package is missing this required owner action",
            "studio.issue.shell_hostess_owner_intake_action_missing",
        );
    }
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.owner_actions_ready",
        !package.required_owner_actions.is_empty()
            && package
                .required_owner_actions
                .iter()
                .all(|action| action.status == StudioShellHostessHandoffPackageActionStatus::Ready),
        "all source package owner actions are ready",
        "one or more source package owner actions are blocked",
        "studio.issue.shell_hostess_owner_intake_action_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.owner_actions_prohibited_in_studio",
        !package.required_owner_actions.is_empty()
            && package
                .required_owner_actions
                .iter()
                .all(|action| action.prohibited_in_studio),
        "all downstream owner actions are explicitly prohibited in Studio",
        "one or more downstream owner actions are not prohibited in Studio",
        "studio.issue.shell_hostess_owner_intake_action_not_prohibited",
    );

    for action in [
        "stage_generated_shells",
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
        "collect_install_launch_evidence",
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_owner_intake.prohibits_{action}"),
            package
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "owner intake explicitly preserves this Studio prohibition",
            "owner intake is missing this Studio prohibition",
            "studio.issue.shell_hostess_owner_intake_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_handoff_package_schema")
                    | Some("studio.issue.shell_handoff_manifest_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessOwnerIntakeStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessOwnerIntakeStatus::Blocked
    } else {
        StudioShellHostessOwnerIntakeStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessOwnerIntakeStatus::Ready => None,
        StudioShellHostessOwnerIntakeStatus::Blocked
        | StudioShellHostessOwnerIntakeStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let assignments =
        shell_hostess_owner_intake_assignments(package, status, issue_code.as_deref());
    let ready_assignment_count = assignments
        .iter()
        .filter(|assignment| {
            assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
        })
        .count();
    let blocked_assignment_count = assignments
        .iter()
        .filter(|assignment| {
            assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Blocked
        })
        .count();
    let hostess_ready_action_count = assignments
        .iter()
        .filter(|assignment| {
            assignment.owner == "rusty.hostess"
                && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
        })
        .count();
    let manifold_ready_action_count = assignments
        .iter()
        .filter(|assignment| {
            assignment.owner == "rusty.manifold"
                && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
        })
        .count();

    StudioShellHostessOwnerIntakeReport {
        schema_id: SHELL_HOSTESS_OWNER_INTAKE_SCHEMA.to_string(),
        source_package_schema: package.schema_id.clone(),
        package_path: package_path.map(|path| path.display().to_string()),
        selected_candidate_id: package.selected_candidate_id.clone(),
        candidate_manifest_path: package.candidate_manifest_path.clone(),
        review_path: package.review_path.clone(),
        handoff_manifest_path: package.handoff_manifest_path.clone(),
        manifest_id: package.manifest_id.clone(),
        project_id: package.project_id.clone(),
        project_revision: package.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.request_only".to_string(),
        intake_owner: "rusty.hostess".to_string(),
        handoff_owner: package.handoff_owner.clone(),
        review_owner: package.review_owner.clone(),
        command_session_authority: package.command_session_authority.clone(),
        install_launch_evidence_authority: package.install_launch_evidence_authority.clone(),
        studio_role: package.studio_role.clone(),
        handoff_ready_count: package.handoff_ready_count,
        handoff_failed_count: package.handoff_failed_count,
        handoff_missing_bundle_count: package.handoff_missing_bundle_count,
        acceptance_baseline_id: package.acceptance_baseline_id.clone(),
        acceptance_baseline_status: package.acceptance_baseline_status,
        acceptance_comparison_status: package.acceptance_comparison_status,
        export_package_baseline_id: package.export_package_baseline_id.clone(),
        export_package_baseline_status: package.export_package_baseline_status,
        export_package_comparison_status: package.export_package_comparison_status,
        source_owner_action_count: package.required_owner_actions.len(),
        ready_assignment_count,
        blocked_assignment_count,
        hostess_ready_action_count,
        manifold_ready_action_count,
        assignments,
        prohibited_actions: package.prohibited_actions.clone(),
        checks,
    }
}

fn shell_hostess_owner_intake_assignments(
    package: &StudioShellHostessHandoffPackageReport,
    status: StudioShellHostessOwnerIntakeStatus,
    issue_code: Option<&str>,
) -> Vec<StudioShellHostessOwnerIntakeAssignment> {
    package
        .required_owner_actions
        .iter()
        .map(|action| {
            let assignment_status = if status == StudioShellHostessOwnerIntakeStatus::Ready
                && action.status == StudioShellHostessHandoffPackageActionStatus::Ready
            {
                StudioShellHostessOwnerIntakeAssignmentStatus::Ready
            } else {
                StudioShellHostessOwnerIntakeAssignmentStatus::Blocked
            };
            StudioShellHostessOwnerIntakeAssignment {
                action_id: action.action_id.clone(),
                owner: action.owner.clone(),
                status: assignment_status,
                request_kind: shell_hostess_owner_intake_request_kind(&action.owner).to_string(),
                source: action.source.clone(),
                next_required_action: action.next_required_action.clone(),
                prohibited_in_studio: action.prohibited_in_studio,
                issue_code: (assignment_status
                    == StudioShellHostessOwnerIntakeAssignmentStatus::Blocked)
                    .then(|| {
                        action
                            .issue_code
                            .as_deref()
                            .or(issue_code)
                            .unwrap_or("studio.issue.shell_hostess_owner_intake_blocked")
                            .to_string()
                    }),
            }
        })
        .collect()
}

fn shell_hostess_owner_intake_request_kind(owner: &str) -> &'static str {
    match owner {
        "rusty.hostess" => "hostess_owner_action_request",
        "rusty.manifold" => "manifold_owner_review_request",
        _ => "owner_action_request",
    }
}

pub fn shell_hostess_staging_preview_for_owner_intake(
    intake: &StudioShellHostessOwnerIntakeReport,
    intake_path: Option<&Path>,
) -> StudioShellHostessStagingPreviewManifest {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.source_intake_schema",
        intake.schema_id == SHELL_HOSTESS_OWNER_INTAKE_SCHEMA,
        "source Hostess owner intake schema is supported",
        "source Hostess owner intake schema is unsupported",
        "studio.issue.shell_hostess_owner_intake_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.intake_path",
        intake_path.is_some(),
        "source Hostess owner intake has a durable path",
        "source Hostess owner intake path is missing",
        "studio.issue.shell_hostess_staging_preview_intake_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.intake_ready",
        intake.status == StudioShellHostessOwnerIntakeStatus::Ready,
        "source Hostess owner intake is ready",
        "source Hostess owner intake is not ready",
        intake
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_owner_intake_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.intake_execution_policy",
        intake.execution_policy == "not_executed.request_only",
        "source intake is request-only and not executed",
        "source intake execution policy is not request-only",
        "studio.issue.shell_hostess_owner_intake_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.package_path",
        intake.package_path.is_some(),
        "source intake names a Hostess handoff package",
        "source intake does not name a Hostess handoff package",
        "studio.issue.shell_hostess_staging_preview_package_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.handoff_manifest_path",
        intake.handoff_manifest_path.is_some(),
        "source intake names a shell handoff manifest",
        "source intake does not name a shell handoff manifest",
        "studio.issue.shell_hostess_staging_preview_handoff_manifest_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.runtime_command_authority",
        intake.command_session_authority.as_deref() == Some("rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.runtime_host_authority",
        intake.install_launch_evidence_authority.as_deref() == Some("rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.studio_role",
        intake.studio_role.as_deref() == Some("authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.source_intake_checks_pass",
        intake
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "source Hostess owner intake checks all pass",
        "source Hostess owner intake contains failed checks",
        "studio.issue.shell_hostess_owner_intake_failed_check",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.assignments_ready",
        !intake.assignments.is_empty()
            && intake.assignments.iter().all(|assignment| {
                assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
            }),
        "all owner-intake assignments are ready",
        "one or more owner-intake assignments are blocked",
        "studio.issue.shell_hostess_staging_preview_assignment_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.assignments_prohibited_in_studio",
        !intake.assignments.is_empty()
            && intake
                .assignments
                .iter()
                .all(|assignment| assignment.prohibited_in_studio),
        "all staging preview assignments remain prohibited in Studio",
        "one or more staging preview assignments are not prohibited in Studio",
        "studio.issue.shell_hostess_staging_preview_assignment_not_prohibited",
    );

    for action_id in [
        "hostess.review_release_candidate",
        "hostess.stage_generated_shells",
        "manifold.review_command_session_contract",
        "hostess.collect_install_launch_evidence",
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_staging_preview.has_{action_id}"),
            intake
                .assignments
                .iter()
                .any(|assignment| assignment.action_id == action_id),
            "source intake includes this downstream assignment",
            "source intake is missing this downstream assignment",
            "studio.issue.shell_hostess_staging_preview_assignment_missing",
        );
    }

    let handoff_manifest = intake.handoff_manifest_path.as_ref().and_then(|path| {
        match load_shell_handoff_manifest(Path::new(path)) {
            Ok(manifest) => Some(manifest),
            Err(error) => {
                checks.push(failed_hostess_staging_preview_check(
                    "studio.check.shell_hostess_staging_preview.handoff_manifest_load",
                    error.to_string(),
                    "studio.issue.shell_hostess_staging_preview_handoff_manifest_load_failed",
                ));
                None
            }
        }
    });
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.handoff_manifest_schema",
        handoff_manifest
            .as_ref()
            .is_some_and(|manifest| manifest.schema_id == SHELL_HANDOFF_MANIFEST_SCHEMA),
        "source handoff manifest schema is supported",
        "source handoff manifest schema is unsupported or unavailable",
        "studio.issue.shell_handoff_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.handoff_manifest_identity",
        handoff_manifest.as_ref().is_some_and(|manifest| {
            intake.manifest_id.as_deref() == Some(manifest.manifest_id.as_str())
                && intake.project_id.as_deref() == Some(manifest.project_id.as_str())
                && intake.project_revision == Some(manifest.project_revision)
        }),
        "source handoff manifest identity matches the owner intake",
        "source handoff manifest identity does not match the owner intake",
        "studio.issue.shell_hostess_staging_preview_handoff_manifest_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.handoff_manifest_ready",
        handoff_manifest.as_ref().is_some_and(|manifest| {
            manifest.status == StudioValidationStatus::Pass
                && manifest.failed_count == 0
                && manifest.missing_bundle_count == 0
        }),
        "source handoff manifest is ready with no failed or missing bundles",
        "source handoff manifest has failed or missing bundles",
        "studio.issue.shell_hostess_staging_preview_handoff_manifest_blocked",
    );

    let export_package = handoff_manifest
        .as_ref()
        .map(shell_export_package_for_manifest);
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.export_package_ready",
        export_package
            .as_ref()
            .is_some_and(|package| package.status == StudioShellExportPackageStatus::Ready),
        "derived export package has descriptor and template paths for every target",
        "derived export package is not ready for staging preview",
        export_package
            .as_ref()
            .and_then(|package| package.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_hostess_staging_preview_export_package_blocked"),
    );

    for action in [
        "stage_generated_shells",
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
        "collect_install_launch_evidence",
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_staging_preview.prohibits_{action}"),
            intake
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "staging preview explicitly preserves this Studio prohibition",
            "staging preview is missing this Studio prohibition",
            "studio.issue.shell_hostess_staging_preview_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_owner_intake_schema")
                    | Some("studio.issue.shell_handoff_manifest_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessStagingPreviewStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessStagingPreviewStatus::Blocked
    } else {
        StudioShellHostessStagingPreviewStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessStagingPreviewStatus::Ready => None,
        StudioShellHostessStagingPreviewStatus::Blocked
        | StudioShellHostessStagingPreviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let intake_artifact_path = intake_path.map(|path| path.display().to_string());
    let groups = shell_hostess_staging_preview_groups(
        intake,
        intake_artifact_path.as_deref(),
        export_package.as_ref(),
        status,
        issue_code.as_deref(),
    );
    let ready_group_count = groups
        .iter()
        .filter(|group| group.status == StudioShellHostessStagingPreviewGroupStatus::Ready)
        .count();
    let blocked_group_count = groups
        .iter()
        .filter(|group| group.status == StudioShellHostessStagingPreviewGroupStatus::Blocked)
        .count();
    let expected_artifact_count = groups
        .iter()
        .map(|group| group.expected_artifact_count)
        .sum();

    StudioShellHostessStagingPreviewManifest {
        schema_id: SHELL_HOSTESS_STAGING_PREVIEW_MANIFEST_SCHEMA.to_string(),
        source_intake_schema: intake.schema_id.clone(),
        source_handoff_manifest_schema: handoff_manifest
            .as_ref()
            .map(|manifest| manifest.schema_id.clone()),
        intake_path: intake_artifact_path,
        package_path: intake.package_path.clone(),
        handoff_manifest_path: intake.handoff_manifest_path.clone(),
        selected_candidate_id: intake.selected_candidate_id.clone(),
        manifest_id: intake.manifest_id.clone(),
        project_id: intake.project_id.clone(),
        project_revision: intake.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.preview_only".to_string(),
        staging_owner: "rusty.hostess".to_string(),
        command_session_authority: intake.command_session_authority.clone(),
        install_launch_evidence_authority: intake.install_launch_evidence_authority.clone(),
        studio_role: intake.studio_role.clone(),
        assignment_count: intake.assignments.len(),
        ready_assignment_count: intake.ready_assignment_count,
        blocked_assignment_count: intake.blocked_assignment_count,
        ready_group_count,
        blocked_group_count,
        expected_artifact_count,
        groups,
        prohibited_actions: intake.prohibited_actions.clone(),
        checks,
    }
}

fn shell_hostess_staging_preview_groups(
    intake: &StudioShellHostessOwnerIntakeReport,
    intake_artifact_path: Option<&str>,
    export_package: Option<&StudioShellExportPackageReport>,
    preview_status: StudioShellHostessStagingPreviewStatus,
    preview_issue_code: Option<&str>,
) -> Vec<StudioShellHostessStagingPreviewGroup> {
    intake
        .assignments
        .iter()
        .map(|assignment| {
            let expected_artifacts = shell_hostess_staging_preview_artifacts_for_assignment(
                assignment,
                intake,
                intake_artifact_path,
                export_package,
            );
            let status = if preview_status == StudioShellHostessStagingPreviewStatus::Ready
                && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
                && !expected_artifacts.is_empty()
            {
                StudioShellHostessStagingPreviewGroupStatus::Ready
            } else {
                StudioShellHostessStagingPreviewGroupStatus::Blocked
            };
            let target_kinds = unique_strings(expected_artifacts.iter().filter_map(|artifact| {
                artifact
                    .target_kind
                    .map(shell_target_kind_label)
                    .map(str::to_string)
            }));
            let graph_ids = unique_strings(
                expected_artifacts
                    .iter()
                    .filter_map(|artifact| artifact.graph_id.clone()),
            );
            StudioShellHostessStagingPreviewGroup {
                action_id: assignment.action_id.clone(),
                owner: assignment.owner.clone(),
                request_kind: assignment.request_kind.clone(),
                route_kind: shell_hostess_staging_preview_route_kind(&assignment.action_id)
                    .to_string(),
                status,
                issue_code: (status == StudioShellHostessStagingPreviewGroupStatus::Blocked).then(
                    || {
                        assignment
                            .issue_code
                            .as_deref()
                            .or(preview_issue_code)
                            .unwrap_or("studio.issue.shell_hostess_staging_preview_blocked")
                            .to_string()
                    },
                ),
                source: assignment.source.clone(),
                next_required_action: assignment.next_required_action.clone(),
                prohibited_in_studio: assignment.prohibited_in_studio,
                expected_artifact_count: expected_artifacts.len(),
                target_kinds,
                graph_ids,
                expected_artifacts,
            }
        })
        .collect()
}

fn shell_hostess_staging_preview_artifacts_for_assignment(
    assignment: &StudioShellHostessOwnerIntakeAssignment,
    intake: &StudioShellHostessOwnerIntakeReport,
    intake_artifact_path: Option<&str>,
    export_package: Option<&StudioShellExportPackageReport>,
) -> Vec<StudioShellHostessStagingPreviewArtifact> {
    let mut artifacts = Vec::new();
    match assignment.action_id.as_str() {
        "hostess.review_release_candidate" => {
            push_optional_staging_artifact(
                &mut artifacts,
                "candidate_manifest",
                intake.candidate_manifest_path.as_deref(),
                None,
                None,
                None,
                None,
            );
            push_optional_staging_artifact(
                &mut artifacts,
                "release_candidate_review",
                intake.review_path.as_deref(),
                None,
                None,
                None,
                None,
            );
            push_optional_staging_artifact(
                &mut artifacts,
                "hostess_handoff_package",
                intake.package_path.as_deref(),
                None,
                None,
                None,
                None,
            );
            push_optional_staging_artifact(
                &mut artifacts,
                "hostess_owner_intake",
                intake_artifact_path,
                None,
                None,
                None,
                None,
            );
        }
        "hostess.stage_generated_shells" => {
            shell_hostess_staging_preview_common_artifacts(
                &mut artifacts,
                intake,
                intake_artifact_path,
            );
            for entry in export_package
                .into_iter()
                .flat_map(|package| package.entries.iter())
            {
                push_staging_artifact(
                    &mut artifacts,
                    "shell_bundle_dir",
                    &entry.bundle_dir,
                    Some(entry.target_kind),
                    Some(entry.graph_id.as_str()),
                    Some(entry.consumer_id.as_str()),
                    None,
                );
                if let Some(descriptor) = entry.descriptor.as_ref() {
                    push_staging_artifact(
                        &mut artifacts,
                        "shell_descriptor",
                        &descriptor.descriptor_path,
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        None,
                    );
                    let manifold_handoff_path = relative_output_path(
                        Path::new(&entry.bundle_dir),
                        &shell_manifold_handoff_artifact_path(&entry.graph_id),
                    );
                    push_staging_artifact(
                        &mut artifacts,
                        "manifold_shell_handoff",
                        &manifold_handoff_path.display().to_string(),
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        None,
                    );
                }
                if let Some(template) = entry.template_manifest.as_ref() {
                    push_staging_artifact(
                        &mut artifacts,
                        "shell_template_manifest",
                        &template.template_manifest_path,
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        entry.host_routes.install_route.as_deref(),
                    );
                }
            }
        }
        "manifold.review_command_session_contract" => {
            push_optional_staging_artifact(
                &mut artifacts,
                "shell_handoff_manifest",
                intake.handoff_manifest_path.as_deref(),
                None,
                None,
                None,
                Some("manifold.command_session_contract"),
            );
            for entry in export_package
                .into_iter()
                .flat_map(|package| package.entries.iter())
            {
                if let Some(descriptor) = entry.descriptor.as_ref() {
                    push_staging_artifact(
                        &mut artifacts,
                        "shell_descriptor",
                        &descriptor.descriptor_path,
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        Some("manifold.command_session_contract"),
                    );
                    let manifold_handoff_path = relative_output_path(
                        Path::new(&entry.bundle_dir),
                        &shell_manifold_handoff_artifact_path(&entry.graph_id),
                    );
                    push_staging_artifact(
                        &mut artifacts,
                        "manifold_shell_handoff",
                        &manifold_handoff_path.display().to_string(),
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        Some("manifold.shell_handoff_review"),
                    );
                }
                if let Some(template) = entry.template_manifest.as_ref() {
                    let route_hint = entry
                        .host_routes
                        .command_bridge
                        .as_deref()
                        .unwrap_or(entry.runtime_route_kind.as_str());
                    push_staging_artifact(
                        &mut artifacts,
                        "shell_template_manifest",
                        &template.template_manifest_path,
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        Some(route_hint),
                    );
                }
            }
        }
        "hostess.collect_install_launch_evidence" => {
            push_optional_staging_artifact(
                &mut artifacts,
                "shell_handoff_manifest",
                intake.handoff_manifest_path.as_deref(),
                None,
                None,
                None,
                Some("hostess.install_launch_evidence"),
            );
            for entry in export_package
                .into_iter()
                .flat_map(|package| package.entries.iter())
            {
                push_staging_artifact(
                    &mut artifacts,
                    "shell_bundle_dir",
                    &entry.bundle_dir,
                    Some(entry.target_kind),
                    Some(entry.graph_id.as_str()),
                    Some(entry.consumer_id.as_str()),
                    entry.host_routes.evidence_pull_route.as_deref(),
                );
            }
        }
        _ => {
            shell_hostess_staging_preview_common_artifacts(
                &mut artifacts,
                intake,
                intake_artifact_path,
            );
        }
    }
    artifacts
}

fn shell_hostess_staging_preview_common_artifacts(
    artifacts: &mut Vec<StudioShellHostessStagingPreviewArtifact>,
    intake: &StudioShellHostessOwnerIntakeReport,
    intake_artifact_path: Option<&str>,
) {
    push_optional_staging_artifact(
        artifacts,
        "hostess_handoff_package",
        intake.package_path.as_deref(),
        None,
        None,
        None,
        None,
    );
    push_optional_staging_artifact(
        artifacts,
        "hostess_owner_intake",
        intake_artifact_path,
        None,
        None,
        None,
        None,
    );
    push_optional_staging_artifact(
        artifacts,
        "shell_handoff_manifest",
        intake.handoff_manifest_path.as_deref(),
        None,
        None,
        None,
        None,
    );
}

fn push_optional_staging_artifact(
    artifacts: &mut Vec<StudioShellHostessStagingPreviewArtifact>,
    artifact_kind: &str,
    path: Option<&str>,
    target_kind: Option<StudioShellTargetKind>,
    graph_id: Option<&str>,
    consumer_id: Option<&str>,
    route_hint: Option<&str>,
) {
    if let Some(path) = path {
        push_staging_artifact(
            artifacts,
            artifact_kind,
            path,
            target_kind,
            graph_id,
            consumer_id,
            route_hint,
        );
    }
}

fn push_staging_artifact(
    artifacts: &mut Vec<StudioShellHostessStagingPreviewArtifact>,
    artifact_kind: &str,
    path: &str,
    target_kind: Option<StudioShellTargetKind>,
    graph_id: Option<&str>,
    consumer_id: Option<&str>,
    route_hint: Option<&str>,
) {
    artifacts.push(StudioShellHostessStagingPreviewArtifact {
        artifact_kind: artifact_kind.to_string(),
        path: path.to_string(),
        target_kind,
        graph_id: graph_id.map(str::to_string),
        consumer_id: consumer_id.map(str::to_string),
        route_hint: route_hint.map(str::to_string),
    });
}

fn shell_hostess_staging_preview_route_kind(action_id: &str) -> &'static str {
    match action_id {
        "hostess.review_release_candidate" => "hostess.review.release_candidate",
        "hostess.stage_generated_shells" => "hostess.stage.generated_shells",
        "manifold.review_command_session_contract" => "manifold.review.command_session_contract",
        "hostess.collect_install_launch_evidence" => "hostess.collect.install_launch_evidence",
        _ => "owner.review.assignment",
    }
}

fn failed_hostess_staging_preview_check(
    check_id: &str,
    evidence: String,
    issue_code: &str,
) -> StudioValidationCheck {
    StudioValidationCheck {
        check_id: check_id.to_string(),
        status: StudioValidationStatus::Fail,
        evidence,
        issue_code: Some(issue_code.to_string()),
        graph_id: None,
        node_ids: Vec::new(),
        edge_ids: Vec::new(),
        reference_ids: Vec::new(),
    }
}

pub fn shell_hostess_staging_file_plan_for_preview(
    preview: &StudioShellHostessStagingPreviewManifest,
    preview_path: Option<&Path>,
) -> StudioShellHostessStagingFilePlan {
    let planned_files = shell_hostess_staging_planned_files(preview);
    let source_artifact_count: usize = preview
        .groups
        .iter()
        .map(|group| group.expected_artifact_count)
        .sum();
    let duplicate_artifact_count = source_artifact_count.saturating_sub(planned_files.len());
    let ready_preview_group_count = preview
        .groups
        .iter()
        .filter(|group| group.status == StudioShellHostessStagingPreviewGroupStatus::Ready)
        .count();
    let blocked_preview_group_count = preview
        .groups
        .iter()
        .filter(|group| group.status == StudioShellHostessStagingPreviewGroupStatus::Blocked)
        .count();

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.source_preview_schema",
        preview.schema_id == SHELL_HOSTESS_STAGING_PREVIEW_MANIFEST_SCHEMA,
        "source Hostess staging preview schema is supported",
        "source Hostess staging preview schema is unsupported",
        "studio.issue.shell_hostess_staging_preview_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.preview_path",
        preview_path.is_some(),
        "source Hostess staging preview has a durable path",
        "source Hostess staging preview path is missing",
        "studio.issue.shell_hostess_staging_file_plan_preview_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.preview_ready",
        preview.status == StudioShellHostessStagingPreviewStatus::Ready,
        "source Hostess staging preview is ready",
        "source Hostess staging preview is not ready",
        preview
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_staging_preview_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.preview_execution_policy",
        preview.execution_policy == "not_executed.preview_only",
        "source preview is preview-only and not executed",
        "source preview execution policy is not preview-only",
        "studio.issue.shell_hostess_staging_preview_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.staging_owner",
        preview.staging_owner == "rusty.hostess",
        "Hostess remains staging owner",
        "staging owner must remain rusty.hostess",
        "studio.issue.staging_owner_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.runtime_command_authority",
        preview.command_session_authority.as_deref() == Some("rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.runtime_host_authority",
        preview.install_launch_evidence_authority.as_deref() == Some("rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.studio_role",
        preview.studio_role.as_deref() == Some("authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.source_preview_checks_pass",
        preview
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "source Hostess staging preview checks all pass",
        "source Hostess staging preview contains failed checks",
        "studio.issue.shell_hostess_staging_preview_failed_check",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.preview_groups_ready",
        !preview.groups.is_empty()
            && preview
                .groups
                .iter()
                .all(|group| group.status == StudioShellHostessStagingPreviewGroupStatus::Ready),
        "all source preview groups are ready",
        "one or more source preview groups are blocked",
        "studio.issue.shell_hostess_staging_file_plan_group_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.preview_groups_prohibited_in_studio",
        !preview.groups.is_empty()
            && preview
                .groups
                .iter()
                .all(|group| group.prohibited_in_studio),
        "all staging file-plan groups remain prohibited in Studio",
        "one or more staging file-plan groups are not prohibited in Studio",
        "studio.issue.shell_hostess_staging_file_plan_group_not_prohibited",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.artifacts_present",
        source_artifact_count > 0 && !planned_files.is_empty(),
        "source preview exposes artifacts to plan",
        "source preview does not expose artifacts to plan",
        "studio.issue.shell_hostess_staging_file_plan_artifacts_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.destination_paths_unique",
        shell_hostess_staging_destination_paths_are_unique(&planned_files),
        "planned destination paths are unique after deduplication",
        "planned destination paths collide after deduplication",
        "studio.issue.shell_hostess_staging_file_plan_destination_collision",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.destination_paths_safe",
        planned_files
            .iter()
            .all(|file| is_safe_relative_manifest_path(&file.destination_path)),
        "planned destination paths are portable relative paths",
        "one or more planned destination paths are unsafe",
        "studio.issue.shell_hostess_staging_file_plan_destination_path_unsafe",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.has_shared_request",
        planned_files.iter().any(|file| file.target_kind.is_none()),
        "file plan includes a shared Hostess staging request",
        "file plan is missing shared Hostess staging artifacts",
        "studio.issue.shell_hostess_staging_file_plan_shared_request_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.has_target_requests",
        planned_files.iter().any(|file| file.target_kind.is_some()),
        "file plan includes per-target Hostess staging requests",
        "file plan is missing per-target Hostess staging artifacts",
        "studio.issue.shell_hostess_staging_file_plan_target_request_missing",
    );

    for action in [
        "stage_generated_shells",
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
        "collect_install_launch_evidence",
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_staging_file_plan.prohibits_{action}"),
            preview
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "staging file plan explicitly preserves this Studio prohibition",
            "staging file plan is missing this Studio prohibition",
            "studio.issue.shell_hostess_staging_file_plan_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_staging_preview_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessStagingFilePlanStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessStagingFilePlanStatus::Blocked
    } else {
        StudioShellHostessStagingFilePlanStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessStagingFilePlanStatus::Ready => None,
        StudioShellHostessStagingFilePlanStatus::Blocked
        | StudioShellHostessStagingFilePlanStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let requests =
        shell_hostess_staging_file_requests(planned_files, status, issue_code.as_deref());
    let ready_request_count = requests
        .iter()
        .filter(|request| request.status == StudioShellHostessStagingFileRequestStatus::Ready)
        .count();
    let blocked_request_count = requests
        .iter()
        .filter(|request| request.status == StudioShellHostessStagingFileRequestStatus::Blocked)
        .count();
    let target_request_count = requests
        .iter()
        .filter(|request| request.target_kind.is_some())
        .count();
    let shared_request_count = requests.len().saturating_sub(target_request_count);
    let planned_file_count = requests
        .iter()
        .map(|request| request.planned_file_count)
        .sum();

    StudioShellHostessStagingFilePlan {
        schema_id: SHELL_HOSTESS_STAGING_FILE_PLAN_SCHEMA.to_string(),
        source_preview_schema: preview.schema_id.clone(),
        preview_path: preview_path.map(|path| path.display().to_string()),
        intake_path: preview.intake_path.clone(),
        package_path: preview.package_path.clone(),
        handoff_manifest_path: preview.handoff_manifest_path.clone(),
        selected_candidate_id: preview.selected_candidate_id.clone(),
        manifest_id: preview.manifest_id.clone(),
        project_id: preview.project_id.clone(),
        project_revision: preview.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.dry_run_only".to_string(),
        staging_owner: "rusty.hostess".to_string(),
        command_session_authority: preview.command_session_authority.clone(),
        install_launch_evidence_authority: preview.install_launch_evidence_authority.clone(),
        studio_role: preview.studio_role.clone(),
        preview_group_count: preview.groups.len(),
        ready_preview_group_count,
        blocked_preview_group_count,
        source_artifact_count,
        planned_file_count,
        duplicate_artifact_count,
        request_count: requests.len(),
        ready_request_count,
        blocked_request_count,
        target_request_count,
        shared_request_count,
        requests,
        prohibited_actions: preview.prohibited_actions.clone(),
        checks,
    }
}

#[derive(Clone, Debug)]
struct StagingPlannedFileBuilder {
    artifact_kind: String,
    source_path: String,
    destination_path: String,
    target_kind: Option<StudioShellTargetKind>,
    graph_id: Option<String>,
    consumer_id: Option<String>,
    route_hints: BTreeSet<String>,
    source_action_ids: BTreeSet<String>,
    source_route_kinds: BTreeSet<String>,
}

fn shell_hostess_staging_planned_files(
    preview: &StudioShellHostessStagingPreviewManifest,
) -> Vec<StudioShellHostessStagingPlannedFile> {
    let mut files: BTreeMap<String, StagingPlannedFileBuilder> = BTreeMap::new();
    for group in &preview.groups {
        for artifact in &group.expected_artifacts {
            let key = shell_hostess_staging_artifact_key(artifact);
            let destination_path = shell_hostess_staging_destination_path(artifact);
            let entry = files
                .entry(key)
                .or_insert_with(|| StagingPlannedFileBuilder {
                    artifact_kind: artifact.artifact_kind.clone(),
                    source_path: artifact.path.clone(),
                    destination_path,
                    target_kind: artifact.target_kind,
                    graph_id: artifact.graph_id.clone(),
                    consumer_id: artifact.consumer_id.clone(),
                    route_hints: BTreeSet::new(),
                    source_action_ids: BTreeSet::new(),
                    source_route_kinds: BTreeSet::new(),
                });
            if let Some(route_hint) = artifact.route_hint.as_ref() {
                entry.route_hints.insert(route_hint.clone());
            }
            entry.source_action_ids.insert(group.action_id.clone());
            entry.source_route_kinds.insert(group.route_kind.clone());
        }
    }
    files
        .into_values()
        .map(|file| StudioShellHostessStagingPlannedFile {
            artifact_kind: file.artifact_kind,
            source_path: file.source_path,
            destination_path: file.destination_path,
            target_kind: file.target_kind,
            graph_id: file.graph_id,
            consumer_id: file.consumer_id,
            route_hints: file.route_hints.into_iter().collect(),
            source_action_ids: file.source_action_ids.into_iter().collect(),
            source_route_kinds: file.source_route_kinds.into_iter().collect(),
        })
        .collect()
}

fn shell_hostess_staging_file_requests(
    planned_files: Vec<StudioShellHostessStagingPlannedFile>,
    plan_status: StudioShellHostessStagingFilePlanStatus,
    plan_issue_code: Option<&str>,
) -> Vec<StudioShellHostessStagingFileRequest> {
    let mut grouped: BTreeMap<String, Vec<StudioShellHostessStagingPlannedFile>> = BTreeMap::new();
    for file in planned_files {
        grouped
            .entry(shell_hostess_staging_target_key(&file))
            .or_default()
            .push(file);
    }
    grouped
        .into_iter()
        .map(|(target_key, mut planned_files)| {
            planned_files.sort_by(|left, right| left.destination_path.cmp(&right.destination_path));
            let target_kind = planned_files.iter().find_map(|file| file.target_kind);
            let graph_id = planned_files.iter().find_map(|file| file.graph_id.clone());
            let consumer_id = planned_files
                .iter()
                .find_map(|file| file.consumer_id.clone());
            let action_ids = unique_strings(
                planned_files
                    .iter()
                    .flat_map(|file| file.source_action_ids.iter().cloned()),
            );
            let route_kinds = unique_strings(
                planned_files
                    .iter()
                    .flat_map(|file| file.source_route_kinds.iter().cloned()),
            );
            let status = if plan_status == StudioShellHostessStagingFilePlanStatus::Ready
                && !planned_files.is_empty()
            {
                StudioShellHostessStagingFileRequestStatus::Ready
            } else {
                StudioShellHostessStagingFileRequestStatus::Blocked
            };
            let destination_root =
                shell_hostess_staging_destination_root(target_kind, graph_id.as_deref());
            StudioShellHostessStagingFileRequest {
                request_id: format!(
                    "hostess.staging_file_plan.{}",
                    shell_hostess_staging_request_id_segment(&target_key)
                ),
                request_kind: if target_kind.is_some() {
                    "hostess_target_staging_file_plan".to_string()
                } else {
                    "hostess_shared_staging_file_plan".to_string()
                },
                owner: "rusty.hostess".to_string(),
                status,
                issue_code: (status == StudioShellHostessStagingFileRequestStatus::Blocked).then(
                    || {
                        plan_issue_code
                            .unwrap_or("studio.issue.shell_hostess_staging_file_plan_blocked")
                            .to_string()
                    },
                ),
                target_key,
                target_kind,
                graph_id,
                consumer_id,
                destination_root,
                action_ids,
                route_kinds,
                planned_file_count: planned_files.len(),
                planned_files,
            }
        })
        .collect()
}

fn shell_hostess_staging_destination_paths_are_unique(
    planned_files: &[StudioShellHostessStagingPlannedFile],
) -> bool {
    let mut seen = BTreeSet::new();
    planned_files
        .iter()
        .all(|file| seen.insert(file.destination_path.clone()))
}

fn shell_hostess_staging_artifact_key(
    artifact: &StudioShellHostessStagingPreviewArtifact,
) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        artifact.artifact_kind,
        artifact.path,
        artifact
            .target_kind
            .map(shell_target_kind_label)
            .unwrap_or("shared"),
        artifact.graph_id.as_deref().unwrap_or("shared"),
        artifact.consumer_id.as_deref().unwrap_or("shared")
    )
}

fn shell_hostess_staging_target_key(file: &StudioShellHostessStagingPlannedFile) -> String {
    match file.target_kind {
        Some(target_kind) => format!(
            "{}/{}",
            shell_target_kind_label(target_kind),
            file.graph_id
                .as_deref()
                .map(shell_hostess_staging_safe_segment)
                .unwrap_or_else(|| "unknown_graph".to_string())
        ),
        None => "shared".to_string(),
    }
}

fn shell_hostess_staging_destination_root(
    target_kind: Option<StudioShellTargetKind>,
    graph_id: Option<&str>,
) -> String {
    match target_kind {
        Some(target_kind) => format!(
            "hostess-staging/targets/{}/{}",
            shell_target_kind_label(target_kind),
            graph_id
                .map(shell_hostess_staging_safe_segment)
                .unwrap_or_else(|| "unknown_graph".to_string())
        ),
        None => "hostess-staging/shared".to_string(),
    }
}

fn shell_hostess_staging_destination_path(
    artifact: &StudioShellHostessStagingPreviewArtifact,
) -> String {
    let root =
        shell_hostess_staging_destination_root(artifact.target_kind, artifact.graph_id.as_deref());
    match artifact.target_kind {
        Some(_) => match artifact.artifact_kind.as_str() {
            "shell_bundle_dir" => format!("{root}/bundle"),
            "shell_descriptor" => {
                format!(
                    "{root}/descriptor/{}",
                    source_path_file_name(&artifact.path)
                )
            }
            "manifold_shell_handoff" => {
                format!("{root}/manifold/{}", source_path_file_name(&artifact.path))
            }
            "shell_template_manifest" => {
                format!("{root}/template/{}", source_path_file_name(&artifact.path))
            }
            other => format!(
                "{root}/{}/{}",
                shell_hostess_staging_safe_segment(other),
                source_path_file_name(&artifact.path)
            ),
        },
        None => shell_hostess_staging_shared_destination_path(&root, artifact),
    }
}

fn shell_hostess_staging_shared_destination_path(
    root: &str,
    artifact: &StudioShellHostessStagingPreviewArtifact,
) -> String {
    match artifact.artifact_kind.as_str() {
        "candidate_manifest" => format!("{root}/release-candidate/candidate-manifest.json"),
        "release_candidate_review" => {
            format!("{root}/release-candidate/release-candidate-review.json")
        }
        "hostess_handoff_package" => format!("{root}/hostess/hostess-handoff-package.json"),
        "hostess_owner_intake" => format!("{root}/hostess/hostess-owner-intake.json"),
        "shell_handoff_manifest" => format!("{root}/handoffs/shell-handoffs.json"),
        other => format!(
            "{root}/{}/{}",
            shell_hostess_staging_safe_segment(other),
            source_path_file_name(&artifact.path)
        ),
    }
}

fn source_path_file_name(path: &str) -> String {
    path.replace('\\', "/")
        .split('/')
        .filter(|segment| !segment.is_empty())
        .next_back()
        .map(shell_hostess_staging_safe_segment)
        .unwrap_or_else(|| "artifact.json".to_string())
}

fn shell_hostess_staging_request_id_segment(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => character,
            '/' | '.' => '.',
            _ => '_',
        })
        .collect()
}

fn shell_hostess_staging_safe_segment(value: &str) -> String {
    let segment = value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '-' => character,
            _ => '_',
        })
        .collect::<String>();
    if segment.is_empty() {
        "unknown".to_string()
    } else {
        segment
    }
}

pub fn shell_hostess_staging_handoff_envelope_for_file_plan(
    file_plan: &StudioShellHostessStagingFilePlan,
    file_plan_path: Option<&Path>,
) -> StudioShellHostessStagingHandoffEnvelope {
    let provenance = shell_hostess_staging_handoff_provenance(file_plan);
    let instruction_specs = shell_hostess_staging_handoff_instruction_specs(file_plan_path);
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.source_file_plan_schema",
        file_plan.schema_id == SHELL_HOSTESS_STAGING_FILE_PLAN_SCHEMA,
        "source Hostess staging file-plan schema is supported",
        "source Hostess staging file-plan schema is unsupported",
        "studio.issue.shell_hostess_staging_file_plan_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.file_plan_path",
        file_plan_path.is_some(),
        "source Hostess staging file plan has a durable path",
        "source Hostess staging file plan path is missing",
        "studio.issue.shell_hostess_staging_handoff_file_plan_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.file_plan_ready",
        file_plan.status == StudioShellHostessStagingFilePlanStatus::Ready,
        "source Hostess staging file plan is ready",
        "source Hostess staging file plan is not ready",
        file_plan
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_staging_file_plan_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.file_plan_execution_policy",
        file_plan.execution_policy == "not_executed.dry_run_only",
        "source file plan is dry-run only and not executed",
        "source file plan execution policy is not dry-run only",
        "studio.issue.shell_hostess_staging_file_plan_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.staging_owner",
        file_plan.staging_owner == "rusty.hostess",
        "Hostess remains staging owner",
        "staging owner must remain rusty.hostess",
        "studio.issue.staging_owner_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.runtime_command_authority",
        file_plan.command_session_authority.as_deref() == Some("rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.runtime_host_authority",
        file_plan.install_launch_evidence_authority.as_deref() == Some("rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.studio_role",
        file_plan.studio_role.as_deref() == Some("authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.source_file_plan_checks_pass",
        file_plan
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "source Hostess staging file-plan checks all pass",
        "source Hostess staging file-plan contains failed checks",
        "studio.issue.shell_hostess_staging_file_plan_failed_check",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.requests_ready",
        !file_plan.requests.is_empty()
            && file_plan
                .requests
                .iter()
                .all(|request| request.status == StudioShellHostessStagingFileRequestStatus::Ready),
        "all source file-plan requests are ready",
        "one or more source file-plan requests are blocked",
        "studio.issue.shell_hostess_staging_handoff_request_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.request_counts",
        file_plan.request_count == file_plan.requests.len()
            && file_plan.ready_request_count == file_plan.requests.len()
            && file_plan.blocked_request_count == 0,
        "source file-plan request counts match request rows",
        "source file-plan request counts do not match request rows",
        "studio.issue.shell_hostess_staging_handoff_request_count_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.planned_file_counts",
        file_plan.planned_file_count
            == file_plan
                .requests
                .iter()
                .map(|request| request.planned_file_count)
                .sum::<usize>(),
        "source file-plan planned-file count matches request rows",
        "source file-plan planned-file count does not match request rows",
        "studio.issue.shell_hostess_staging_handoff_file_count_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.provenance_present",
        !provenance.plan_checksum.is_empty()
            && !provenance.source_artifact_kinds.is_empty()
            && !provenance.source_action_ids.is_empty()
            && !provenance.source_route_kinds.is_empty()
            && !provenance.target_keys.is_empty(),
        "handoff envelope has checksum and source provenance summary",
        "handoff envelope is missing checksum or source provenance summary",
        "studio.issue.shell_hostess_staging_handoff_provenance_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.instructions_present",
        instruction_specs.len() >= 4,
        "handoff envelope includes external-owner instructions",
        "handoff envelope is missing external-owner instructions",
        "studio.issue.shell_hostess_staging_handoff_instruction_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.instructions_prohibited_in_studio",
        instruction_specs
            .iter()
            .all(|spec| spec.prohibited_in_studio),
        "all handoff instructions remain prohibited in Studio",
        "one or more handoff instructions are not prohibited in Studio",
        "studio.issue.shell_hostess_staging_handoff_instruction_not_prohibited",
    );

    for action in [
        "stage_generated_shells",
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
        "collect_install_launch_evidence",
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_staging_handoff.prohibits_{action}"),
            file_plan
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "staging handoff explicitly preserves this Studio prohibition",
            "staging handoff is missing this Studio prohibition",
            "studio.issue.shell_hostess_staging_handoff_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_staging_file_plan_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessStagingHandoffEnvelopeStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessStagingHandoffEnvelopeStatus::Blocked
    } else {
        StudioShellHostessStagingHandoffEnvelopeStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessStagingHandoffEnvelopeStatus::Ready => None,
        StudioShellHostessStagingHandoffEnvelopeStatus::Blocked
        | StudioShellHostessStagingHandoffEnvelopeStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let owner_instructions = shell_hostess_staging_handoff_instructions(
        instruction_specs,
        status,
        issue_code.as_deref(),
    );
    let ready_instruction_count = owner_instructions
        .iter()
        .filter(|instruction| {
            instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Ready
        })
        .count();
    let blocked_instruction_count = owner_instructions
        .iter()
        .filter(|instruction| {
            instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Blocked
        })
        .count();

    StudioShellHostessStagingHandoffEnvelope {
        schema_id: SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA.to_string(),
        source_file_plan_schema: file_plan.schema_id.clone(),
        file_plan_path: file_plan_path.map(|path| path.display().to_string()),
        preview_path: file_plan.preview_path.clone(),
        intake_path: file_plan.intake_path.clone(),
        package_path: file_plan.package_path.clone(),
        handoff_manifest_path: file_plan.handoff_manifest_path.clone(),
        selected_candidate_id: file_plan.selected_candidate_id.clone(),
        envelope_id: default_shell_hostess_staging_handoff_envelope_id(file_plan),
        manifest_id: file_plan.manifest_id.clone(),
        project_id: file_plan.project_id.clone(),
        project_revision: file_plan.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.handoff_only".to_string(),
        handoff_owner: "rusty.hostess".to_string(),
        staging_owner: file_plan.staging_owner.clone(),
        command_session_authority: file_plan.command_session_authority.clone(),
        install_launch_evidence_authority: file_plan.install_launch_evidence_authority.clone(),
        studio_role: file_plan.studio_role.clone(),
        planned_file_count: file_plan.planned_file_count,
        request_count: file_plan.request_count,
        ready_request_count: file_plan.ready_request_count,
        blocked_request_count: file_plan.blocked_request_count,
        target_request_count: file_plan.target_request_count,
        shared_request_count: file_plan.shared_request_count,
        instruction_count: owner_instructions.len(),
        ready_instruction_count,
        blocked_instruction_count,
        provenance,
        request_summaries: shell_hostess_staging_handoff_request_summaries(file_plan),
        owner_instructions,
        prohibited_actions: file_plan.prohibited_actions.clone(),
        checks,
    }
}

#[derive(Clone, Debug)]
struct StagingHandoffInstructionSpec {
    instruction_id: &'static str,
    owner: &'static str,
    instruction_kind: &'static str,
    route_kind: &'static str,
    source: &'static str,
    next_required_action: &'static str,
    prohibited_in_studio: bool,
    expected_input_path: Option<String>,
}

fn shell_hostess_staging_handoff_instruction_specs(
    file_plan_path: Option<&Path>,
) -> Vec<StagingHandoffInstructionSpec> {
    let file_plan_path = file_plan_path.map(|path| path.display().to_string());
    vec![
        StagingHandoffInstructionSpec {
            instruction_id: "hostess.review_staging_handoff",
            owner: "rusty.hostess",
            instruction_kind: "hostess_handoff_review",
            route_kind: "hostess.review.staging_handoff",
            source: "hostess_staging_handoff_envelope",
            next_required_action: "review_staging_handoff_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingHandoffInstructionSpec {
            instruction_id: "hostess.copy_staging_files",
            owner: "rusty.hostess",
            instruction_kind: "hostess_file_copy_request",
            route_kind: "hostess.stage.files_from_plan",
            source: "hostess_staging_file_plan",
            next_required_action: "copy_stage_files_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingHandoffInstructionSpec {
            instruction_id: "manifold.review_command_session_contract",
            owner: "rusty.manifold",
            instruction_kind: "manifold_contract_review",
            route_kind: "manifold.review.command_session_contract",
            source: "hostess_staging_file_plan",
            next_required_action: "review_command_session_contract_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingHandoffInstructionSpec {
            instruction_id: "hostess.collect_install_launch_evidence",
            owner: "rusty.hostess",
            instruction_kind: "hostess_evidence_collection_request",
            route_kind: "hostess.collect.install_launch_evidence",
            source: "hostess_staging_file_plan",
            next_required_action: "collect_install_launch_evidence_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path,
        },
    ]
}

fn shell_hostess_staging_handoff_instructions(
    specs: Vec<StagingHandoffInstructionSpec>,
    envelope_status: StudioShellHostessStagingHandoffEnvelopeStatus,
    envelope_issue_code: Option<&str>,
) -> Vec<StudioShellHostessStagingHandoffInstruction> {
    specs
        .into_iter()
        .map(|spec| {
            let status = if envelope_status == StudioShellHostessStagingHandoffEnvelopeStatus::Ready
            {
                StudioShellHostessStagingHandoffInstructionStatus::Ready
            } else {
                StudioShellHostessStagingHandoffInstructionStatus::Blocked
            };
            StudioShellHostessStagingHandoffInstruction {
                instruction_id: spec.instruction_id.to_string(),
                owner: spec.owner.to_string(),
                status,
                issue_code: (status == StudioShellHostessStagingHandoffInstructionStatus::Blocked)
                    .then(|| {
                        envelope_issue_code
                            .unwrap_or("studio.issue.shell_hostess_staging_handoff_blocked")
                            .to_string()
                    }),
                instruction_kind: spec.instruction_kind.to_string(),
                route_kind: spec.route_kind.to_string(),
                source: spec.source.to_string(),
                next_required_action: spec.next_required_action.to_string(),
                prohibited_in_studio: spec.prohibited_in_studio,
                expected_input_path: spec.expected_input_path,
            }
        })
        .collect()
}

fn shell_hostess_staging_handoff_request_summaries(
    file_plan: &StudioShellHostessStagingFilePlan,
) -> Vec<StudioShellHostessStagingHandoffRequestSummary> {
    file_plan
        .requests
        .iter()
        .map(|request| StudioShellHostessStagingHandoffRequestSummary {
            request_id: request.request_id.clone(),
            request_kind: request.request_kind.clone(),
            owner: request.owner.clone(),
            status: request.status,
            target_key: request.target_key.clone(),
            target_kind: request.target_kind,
            graph_id: request.graph_id.clone(),
            consumer_id: request.consumer_id.clone(),
            destination_root: request.destination_root.clone(),
            planned_file_count: request.planned_file_count,
            route_kinds: request.route_kinds.clone(),
            action_ids: request.action_ids.clone(),
        })
        .collect()
}

fn shell_hostess_staging_handoff_provenance(
    file_plan: &StudioShellHostessStagingFilePlan,
) -> StudioShellHostessStagingHandoffProvenance {
    let planned_files = file_plan
        .requests
        .iter()
        .flat_map(|request| request.planned_files.iter());
    let source_artifact_kinds =
        unique_strings(planned_files.clone().map(|file| file.artifact_kind.clone()));
    let source_action_ids = unique_strings(
        file_plan
            .requests
            .iter()
            .flat_map(|request| request.action_ids.iter().cloned()),
    );
    let source_route_kinds = unique_strings(
        file_plan
            .requests
            .iter()
            .flat_map(|request| request.route_kinds.iter().cloned()),
    );
    let target_keys = unique_strings(
        file_plan
            .requests
            .iter()
            .map(|request| request.target_key.clone()),
    );
    let destination_roots = unique_strings(
        file_plan
            .requests
            .iter()
            .map(|request| request.destination_root.clone()),
    );
    StudioShellHostessStagingHandoffProvenance {
        checksum_algorithm: "fnv1a64.studio_staging_file_plan.v1".to_string(),
        plan_checksum: shell_hostess_staging_file_plan_checksum(file_plan),
        source_artifact_kinds,
        source_action_ids,
        source_route_kinds,
        target_keys,
        destination_roots,
    }
}

fn shell_hostess_staging_file_plan_checksum(
    file_plan: &StudioShellHostessStagingFilePlan,
) -> String {
    let mut hasher = Fnv1a64::new();
    hasher.update(&file_plan.schema_id);
    hasher.update(file_plan.project_id.as_deref().unwrap_or(""));
    hasher.update(
        &file_plan
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_default(),
    );
    for request in &file_plan.requests {
        hasher.update(&request.request_id);
        hasher.update(&request.target_key);
        hasher.update(&request.destination_root);
        for file in &request.planned_files {
            hasher.update(&file.artifact_kind);
            hasher.update(&file.source_path);
            hasher.update(&file.destination_path);
            hasher.update(file.graph_id.as_deref().unwrap_or(""));
            hasher.update(file.consumer_id.as_deref().unwrap_or(""));
            for route_hint in &file.route_hints {
                hasher.update(route_hint);
            }
            for action_id in &file.source_action_ids {
                hasher.update(action_id);
            }
            for route_kind in &file.source_route_kinds {
                hasher.update(route_kind);
            }
        }
    }
    format!("{:016x}", hasher.finish())
}

struct Fnv1a64 {
    value: u64,
}

impl Fnv1a64 {
    fn new() -> Self {
        Self {
            value: 0xcbf29ce484222325,
        }
    }

    fn update(&mut self, value: &str) {
        for byte in value.as_bytes().iter().copied().chain([0xff]) {
            self.value ^= u64::from(byte);
            self.value = self.value.wrapping_mul(0x100000001b3);
        }
    }

    fn finish(self) -> u64 {
        self.value
    }
}

fn default_shell_hostess_staging_handoff_envelope_id(
    file_plan: &StudioShellHostessStagingFilePlan,
) -> String {
    format!(
        "studio.hostess_staging_handoff.{}.rev{}",
        file_plan.project_id.as_deref().unwrap_or("unknown_project"),
        file_plan
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    )
}

pub fn shell_hostess_staging_acceptance_checklist_for_handoff(
    handoff: &StudioShellHostessStagingHandoffEnvelope,
    handoff_path: Option<&Path>,
) -> StudioShellHostessStagingAcceptanceChecklistReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.source_handoff_schema",
        handoff.schema_id == SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA,
        "source Hostess staging handoff schema is supported",
        "source Hostess staging handoff schema is unsupported",
        "studio.issue.shell_hostess_staging_handoff_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.handoff_path",
        handoff_path.is_some(),
        "source Hostess staging handoff has a durable path",
        "source Hostess staging handoff path is missing",
        "studio.issue.shell_hostess_staging_acceptance_handoff_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.handoff_ready",
        handoff.status == StudioShellHostessStagingHandoffEnvelopeStatus::Ready,
        "source Hostess staging handoff is ready",
        "source Hostess staging handoff is not ready",
        handoff
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_staging_handoff_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.handoff_execution_policy",
        handoff.execution_policy == "not_executed.handoff_only",
        "source handoff is handoff-only and not executed",
        "source handoff execution policy is not handoff-only",
        "studio.issue.shell_hostess_staging_handoff_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.handoff_owner",
        handoff.handoff_owner == "rusty.hostess" && handoff.staging_owner == "rusty.hostess",
        "Hostess remains handoff and staging owner",
        "handoff and staging owners must remain rusty.hostess",
        "studio.issue.shell_hostess_staging_handoff_owner_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.runtime_command_authority",
        handoff.command_session_authority.as_deref() == Some("rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.runtime_host_authority",
        handoff.install_launch_evidence_authority.as_deref() == Some("rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.studio_role",
        handoff.studio_role.as_deref() == Some("authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.source_handoff_checks_pass",
        handoff
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "source Hostess staging handoff checks all pass",
        "source Hostess staging handoff contains failed checks",
        "studio.issue.shell_hostess_staging_handoff_failed_check",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.requests_ready",
        !handoff.request_summaries.is_empty()
            && handoff
                .request_summaries
                .iter()
                .all(|request| request.status == StudioShellHostessStagingFileRequestStatus::Ready),
        "all handoff request summaries are ready",
        "one or more handoff request summaries are blocked",
        "studio.issue.shell_hostess_staging_acceptance_request_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.request_counts",
        handoff.request_count == handoff.request_summaries.len()
            && handoff.ready_request_count == handoff.request_summaries.len()
            && handoff.blocked_request_count == 0,
        "handoff request counts match request summaries",
        "handoff request counts do not match request summaries",
        "studio.issue.shell_hostess_staging_acceptance_request_count_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.instructions_ready",
        !handoff.owner_instructions.is_empty()
            && handoff.owner_instructions.iter().all(|instruction| {
                instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Ready
            }),
        "all handoff owner instructions are ready",
        "one or more handoff owner instructions are blocked",
        "studio.issue.shell_hostess_staging_acceptance_instruction_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.instruction_counts",
        handoff.instruction_count == handoff.owner_instructions.len()
            && handoff.ready_instruction_count == handoff.owner_instructions.len()
            && handoff.blocked_instruction_count == 0,
        "handoff instruction counts match instruction rows",
        "handoff instruction counts do not match instruction rows",
        "studio.issue.shell_hostess_staging_acceptance_instruction_count_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.instructions_prohibited_in_studio",
        !handoff.owner_instructions.is_empty()
            && handoff
                .owner_instructions
                .iter()
                .all(|instruction| instruction.prohibited_in_studio),
        "all handoff instructions remain prohibited in Studio",
        "one or more handoff instructions are not prohibited in Studio",
        "studio.issue.shell_hostess_staging_acceptance_instruction_not_prohibited",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.provenance_checksum",
        handoff.provenance.checksum_algorithm == "fnv1a64.studio_staging_file_plan.v1"
            && handoff.provenance.plan_checksum.len() == 16,
        "handoff checksum uses the expected staging file-plan algorithm",
        "handoff checksum is missing or uses an unexpected algorithm",
        "studio.issue.shell_hostess_staging_acceptance_checksum_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.provenance_sources",
        !handoff.provenance.source_artifact_kinds.is_empty()
            && !handoff.provenance.source_action_ids.is_empty()
            && !handoff.provenance.source_route_kinds.is_empty()
            && !handoff.provenance.target_keys.is_empty()
            && !handoff.provenance.destination_roots.is_empty(),
        "handoff provenance names artifacts, actions, routes, targets, and roots",
        "handoff provenance is missing artifacts, actions, routes, targets, or roots",
        "studio.issue.shell_hostess_staging_acceptance_provenance_missing",
    );

    for (instruction_id, owner, route_kind) in [
        (
            "hostess.review_staging_handoff",
            "rusty.hostess",
            "hostess.review.staging_handoff",
        ),
        (
            "hostess.copy_staging_files",
            "rusty.hostess",
            "hostess.stage.files_from_plan",
        ),
        (
            "manifold.review_command_session_contract",
            "rusty.manifold",
            "manifold.review.command_session_contract",
        ),
        (
            "hostess.collect_install_launch_evidence",
            "rusty.hostess",
            "hostess.collect.install_launch_evidence",
        ),
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_staging_acceptance.has_{instruction_id}"),
            shell_hostess_staging_handoff_has_ready_instruction(
                handoff,
                instruction_id,
                owner,
                route_kind,
            ),
            "handoff includes this ready external-owner instruction",
            "handoff is missing this ready external-owner instruction",
            "studio.issue.shell_hostess_staging_acceptance_instruction_missing",
        );
    }

    for action in [
        "stage_generated_shells",
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
        "collect_install_launch_evidence",
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_staging_acceptance.prohibits_{action}"),
            handoff
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "staging acceptance explicitly preserves this Studio prohibition",
            "staging acceptance is missing this Studio prohibition",
            "studio.issue.shell_hostess_staging_acceptance_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_staging_handoff_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessStagingAcceptanceStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessStagingAcceptanceStatus::Blocked
    } else {
        StudioShellHostessStagingAcceptanceStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessStagingAcceptanceStatus::Ready => None,
        StudioShellHostessStagingAcceptanceStatus::Blocked
        | StudioShellHostessStagingAcceptanceStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let entries = if status == StudioShellHostessStagingAcceptanceStatus::Rejected {
        Vec::new()
    } else {
        shell_hostess_staging_acceptance_entries(
            shell_hostess_staging_acceptance_item_specs(handoff, handoff_path),
            status,
            issue_code.as_deref(),
        )
    };
    let ready_item_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Ready)
        .count();
    let blocked_item_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Blocked)
        .count();
    let rejected_item_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Rejected)
        .count();

    StudioShellHostessStagingAcceptanceChecklistReport {
        schema_id: SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA.to_string(),
        source_handoff_schema: handoff.schema_id.clone(),
        handoff_path: handoff_path.map(|path| path.display().to_string()),
        file_plan_path: handoff.file_plan_path.clone(),
        preview_path: handoff.preview_path.clone(),
        intake_path: handoff.intake_path.clone(),
        package_path: handoff.package_path.clone(),
        handoff_manifest_path: handoff.handoff_manifest_path.clone(),
        selected_candidate_id: handoff.selected_candidate_id.clone(),
        envelope_id: handoff.envelope_id.clone(),
        manifest_id: handoff.manifest_id.clone(),
        project_id: handoff.project_id.clone(),
        project_revision: handoff.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.acceptance_check_only".to_string(),
        checklist_owner: "rusty.hostess".to_string(),
        handoff_owner: handoff.handoff_owner.clone(),
        staging_owner: handoff.staging_owner.clone(),
        command_session_authority: handoff.command_session_authority.clone(),
        install_launch_evidence_authority: handoff.install_launch_evidence_authority.clone(),
        studio_role: handoff.studio_role.clone(),
        request_count: handoff.request_count,
        ready_request_count: handoff.ready_request_count,
        blocked_request_count: handoff.blocked_request_count,
        instruction_count: handoff.instruction_count,
        ready_instruction_count: handoff.ready_instruction_count,
        blocked_instruction_count: handoff.blocked_instruction_count,
        checksum_algorithm: handoff.provenance.checksum_algorithm.clone(),
        plan_checksum: handoff.provenance.plan_checksum.clone(),
        ready_item_count,
        blocked_item_count,
        rejected_item_count,
        prohibited_actions: handoff.prohibited_actions.clone(),
        handoff_checks: checks,
        entries,
    }
}

fn shell_hostess_staging_handoff_has_ready_instruction(
    handoff: &StudioShellHostessStagingHandoffEnvelope,
    instruction_id: &str,
    owner: &str,
    route_kind: &str,
) -> bool {
    handoff.owner_instructions.iter().any(|instruction| {
        instruction.instruction_id == instruction_id
            && instruction.owner == owner
            && instruction.route_kind == route_kind
            && instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Ready
            && instruction.prohibited_in_studio
    })
}

#[derive(Clone, Debug)]
struct StagingAcceptanceItemSpec {
    item_id: &'static str,
    owner: &'static str,
    item_kind: &'static str,
    route_kind: &'static str,
    source: &'static str,
    evidence: String,
    next_required_action: &'static str,
    prohibited_in_studio: bool,
    expected_input_path: Option<String>,
}

fn shell_hostess_staging_acceptance_item_specs(
    handoff: &StudioShellHostessStagingHandoffEnvelope,
    handoff_path: Option<&Path>,
) -> Vec<StagingAcceptanceItemSpec> {
    let handoff_path = handoff_path.map(|path| path.display().to_string());
    let file_plan_path = handoff.file_plan_path.clone();
    vec![
        StagingAcceptanceItemSpec {
            item_id: "hostess.accept_staging_handoff",
            owner: "rusty.hostess",
            item_kind: "hostess_acceptance_gate",
            route_kind: "hostess.accept.staging_handoff",
            source: "hostess_staging_handoff_envelope",
            evidence: format!(
                "handoff envelope {} is ready for Hostess acceptance",
                handoff.envelope_id
            ),
            next_required_action: "accept_or_reject_handoff_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: handoff_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.verify_staging_file_plan_checksum",
            owner: "rusty.hostess",
            item_kind: "hostess_checksum_gate",
            route_kind: "hostess.verify.staging_file_plan_checksum",
            source: "hostess_staging_handoff_envelope",
            evidence: format!(
                "{} checksum {}",
                handoff.provenance.checksum_algorithm, handoff.provenance.plan_checksum
            ),
            next_required_action: "verify_file_plan_checksum_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.review_staging_file_requests",
            owner: "rusty.hostess",
            item_kind: "hostess_file_plan_review_gate",
            route_kind: "hostess.review.staging_file_requests",
            source: "hostess_staging_handoff_envelope",
            evidence: format!(
                "{} ready requests over {} planned files",
                handoff.ready_request_count, handoff.planned_file_count
            ),
            next_required_action: "review_shared_and_target_requests_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.copy_staging_files",
            owner: "rusty.hostess",
            item_kind: "hostess_file_copy_request",
            route_kind: "hostess.stage.files_from_plan",
            source: "hostess_staging_file_plan",
            evidence: "file copy remains an external Hostess action".to_string(),
            next_required_action: "copy_stage_files_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "manifold.review_command_session_contract",
            owner: "rusty.manifold",
            item_kind: "manifold_contract_review",
            route_kind: "manifold.review.command_session_contract",
            source: "hostess_staging_handoff_envelope",
            evidence: "Manifold remains command/session authority".to_string(),
            next_required_action: "review_command_session_contract_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: handoff_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.collect_install_launch_evidence",
            owner: "rusty.hostess",
            item_kind: "hostess_evidence_collection_request",
            route_kind: "hostess.collect.install_launch_evidence",
            source: "hostess_staging_handoff_envelope",
            evidence: "install/launch evidence remains an external Hostess action".to_string(),
            next_required_action: "collect_install_launch_evidence_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: handoff_path,
        },
    ]
}

fn shell_hostess_staging_acceptance_entries(
    specs: Vec<StagingAcceptanceItemSpec>,
    checklist_status: StudioShellHostessStagingAcceptanceStatus,
    checklist_issue_code: Option<&str>,
) -> Vec<StudioShellHostessStagingAcceptanceChecklistEntry> {
    specs
        .into_iter()
        .map(|spec| {
            let status = if checklist_status == StudioShellHostessStagingAcceptanceStatus::Ready {
                StudioShellHostessStagingAcceptanceStatus::Ready
            } else {
                StudioShellHostessStagingAcceptanceStatus::Blocked
            };
            StudioShellHostessStagingAcceptanceChecklistEntry {
                item_id: spec.item_id.to_string(),
                owner: spec.owner.to_string(),
                status,
                issue_code: (status != StudioShellHostessStagingAcceptanceStatus::Ready).then(
                    || {
                        checklist_issue_code
                            .unwrap_or("studio.issue.shell_hostess_staging_acceptance_blocked")
                            .to_string()
                    },
                ),
                item_kind: spec.item_kind.to_string(),
                route_kind: spec.route_kind.to_string(),
                source: spec.source.to_string(),
                evidence: spec.evidence,
                next_required_action: spec.next_required_action.to_string(),
                prohibited_in_studio: spec.prohibited_in_studio,
                expected_input_path: spec.expected_input_path,
            }
        })
        .collect()
}

pub fn shell_hostess_staging_acceptance_manifest_for_checklist(
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
    checklist_path: &Path,
    acceptance_id: Option<&str>,
    label: Option<&str>,
) -> StudioShellHostessStagingAcceptanceManifest {
    let acceptance_id = acceptance_id
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_hostess_staging_acceptance_id(checklist));
    let label = label
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_hostess_staging_acceptance_label(checklist));

    StudioShellHostessStagingAcceptanceManifest {
        schema_id: SHELL_HOSTESS_STAGING_ACCEPTANCE_MANIFEST_SCHEMA.to_string(),
        acceptance_id,
        label,
        checklist_path: checklist_path.display().to_string(),
        checklist_schema: checklist.schema_id.clone(),
        envelope_id: checklist.envelope_id.clone(),
        manifest_id: checklist.manifest_id.clone(),
        project_id: checklist.project_id.clone(),
        project_revision: checklist.project_revision,
        status: checklist.status,
        issue_code: checklist.issue_code.clone(),
        execution_policy: checklist.execution_policy.clone(),
        checklist_owner: checklist.checklist_owner.clone(),
        handoff_owner: checklist.handoff_owner.clone(),
        staging_owner: checklist.staging_owner.clone(),
        command_session_authority: checklist.command_session_authority.clone(),
        install_launch_evidence_authority: checklist.install_launch_evidence_authority.clone(),
        studio_role: checklist.studio_role.clone(),
        request_count: checklist.request_count,
        ready_request_count: checklist.ready_request_count,
        blocked_request_count: checklist.blocked_request_count,
        instruction_count: checklist.instruction_count,
        ready_instruction_count: checklist.ready_instruction_count,
        blocked_instruction_count: checklist.blocked_instruction_count,
        checksum_algorithm: checklist.checksum_algorithm.clone(),
        plan_checksum: checklist.plan_checksum.clone(),
        ready_item_count: checklist.ready_item_count,
        blocked_item_count: checklist.blocked_item_count,
        rejected_item_count: checklist.rejected_item_count,
        prohibited_actions: checklist.prohibited_actions.clone(),
    }
}

pub fn shell_hostess_staging_acceptance_index_for_manifests(
    acceptances: Vec<(StudioShellHostessStagingAcceptanceManifest, Option<PathBuf>)>,
    default_acceptance_id: Option<&str>,
) -> StudioShellHostessStagingAcceptanceIndex {
    let entries = acceptances
        .into_iter()
        .map(|(acceptance, acceptance_manifest_path)| {
            shell_hostess_staging_acceptance_index_entry_for_manifest(
                acceptance,
                acceptance_manifest_path,
            )
        })
        .collect::<Vec<_>>();

    shell_hostess_staging_acceptance_index_for_entries(entries, default_acceptance_id)
}

pub fn append_shell_hostess_staging_acceptance_index_manifests(
    index: &StudioShellHostessStagingAcceptanceIndex,
    acceptances: Vec<(StudioShellHostessStagingAcceptanceManifest, Option<PathBuf>)>,
    default_acceptance_id: Option<&str>,
) -> StudioShellHostessStagingAcceptanceIndex {
    let entries = index
        .entries
        .iter()
        .cloned()
        .chain(
            acceptances
                .into_iter()
                .map(|(acceptance, acceptance_manifest_path)| {
                    shell_hostess_staging_acceptance_index_entry_for_manifest(
                        acceptance,
                        acceptance_manifest_path,
                    )
                }),
        )
        .collect::<Vec<_>>();
    let default_acceptance_id = default_acceptance_id.or(index.default_acceptance_id.as_deref());

    shell_hostess_staging_acceptance_index_for_entries(entries, default_acceptance_id)
}

pub fn promote_shell_hostess_staging_acceptance_index_default(
    index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_id: &str,
) -> Option<StudioShellHostessStagingAcceptanceIndex> {
    index
        .entries
        .iter()
        .any(|entry| entry.acceptance_id == acceptance_id)
        .then(|| {
            shell_hostess_staging_acceptance_index_for_entries(
                index.entries.clone(),
                Some(acceptance_id),
            )
        })
}

fn shell_hostess_staging_acceptance_index_entry_for_manifest(
    acceptance: StudioShellHostessStagingAcceptanceManifest,
    acceptance_manifest_path: Option<PathBuf>,
) -> StudioShellHostessStagingAcceptanceIndexEntry {
    StudioShellHostessStagingAcceptanceIndexEntry {
        acceptance_id: acceptance.acceptance_id,
        label: acceptance.label,
        acceptance_manifest_path: acceptance_manifest_path.map(|path| path.display().to_string()),
        checklist_path: acceptance.checklist_path,
        checklist_schema: acceptance.checklist_schema,
        envelope_id: acceptance.envelope_id,
        manifest_id: acceptance.manifest_id,
        project_id: acceptance.project_id,
        project_revision: acceptance.project_revision,
        status: acceptance.status,
        issue_code: acceptance.issue_code,
        execution_policy: acceptance.execution_policy,
        checklist_owner: acceptance.checklist_owner,
        handoff_owner: acceptance.handoff_owner,
        staging_owner: acceptance.staging_owner,
        command_session_authority: acceptance.command_session_authority,
        install_launch_evidence_authority: acceptance.install_launch_evidence_authority,
        studio_role: acceptance.studio_role,
        request_count: acceptance.request_count,
        ready_request_count: acceptance.ready_request_count,
        blocked_request_count: acceptance.blocked_request_count,
        instruction_count: acceptance.instruction_count,
        ready_instruction_count: acceptance.ready_instruction_count,
        blocked_instruction_count: acceptance.blocked_instruction_count,
        checksum_algorithm: acceptance.checksum_algorithm,
        plan_checksum: acceptance.plan_checksum,
        ready_item_count: acceptance.ready_item_count,
        blocked_item_count: acceptance.blocked_item_count,
        rejected_item_count: acceptance.rejected_item_count,
    }
}

fn shell_hostess_staging_acceptance_index_for_entries(
    entries: Vec<StudioShellHostessStagingAcceptanceIndexEntry>,
    default_acceptance_id: Option<&str>,
) -> StudioShellHostessStagingAcceptanceIndex {
    let mut by_id = BTreeMap::new();
    for entry in entries {
        by_id.insert(entry.acceptance_id.clone(), entry);
    }

    let entries = by_id.into_values().collect::<Vec<_>>();
    let default_acceptance_id = default_acceptance_id
        .filter(|acceptance_id| {
            entries
                .iter()
                .any(|entry| entry.acceptance_id == *acceptance_id)
        })
        .map(str::to_string)
        .or_else(|| entries.first().map(|entry| entry.acceptance_id.clone()));

    StudioShellHostessStagingAcceptanceIndex {
        schema_id: SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA.to_string(),
        project_ids: unique_strings(entries.iter().filter_map(|entry| entry.project_id.clone())),
        envelope_ids: unique_strings(entries.iter().map(|entry| entry.envelope_id.clone())),
        manifest_ids: unique_strings(entries.iter().filter_map(|entry| entry.manifest_id.clone())),
        default_acceptance_id,
        acceptance_count: entries.len(),
        ready_acceptance_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Ready)
            .count(),
        blocked_acceptance_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Blocked)
            .count(),
        rejected_acceptance_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Rejected)
            .count(),
        entries,
    }
}

pub fn select_shell_hostess_staging_acceptance_index_entry<'a>(
    index: &'a StudioShellHostessStagingAcceptanceIndex,
    acceptance_id: Option<&str>,
) -> Option<&'a StudioShellHostessStagingAcceptanceIndexEntry> {
    let selected_id = acceptance_id.or(index.default_acceptance_id.as_deref());
    selected_id
        .and_then(|selected_id| {
            index
                .entries
                .iter()
                .find(|entry| entry.acceptance_id == selected_id)
        })
        .or_else(|| {
            acceptance_id
                .is_none()
                .then(|| index.entries.first())
                .flatten()
        })
}

pub fn summarize_shell_hostess_staging_acceptance_index_selection(
    index: &StudioShellHostessStagingAcceptanceIndex,
    index_path: Option<&Path>,
    requested_acceptance_id: Option<&str>,
) -> StudioShellHostessStagingAcceptanceSelectionReport {
    let selected_entry =
        select_shell_hostess_staging_acceptance_index_entry(index, requested_acceptance_id);
    let selected_acceptance_id = selected_entry.map(|entry| entry.acceptance_id.clone());
    let status = if index.entries.is_empty() {
        StudioShellHostessStagingAcceptanceSelectionStatus::Empty
    } else if selected_entry.is_some() {
        StudioShellHostessStagingAcceptanceSelectionStatus::Selected
    } else {
        StudioShellHostessStagingAcceptanceSelectionStatus::Missing
    };
    let issue_code = match status {
        StudioShellHostessStagingAcceptanceSelectionStatus::Selected => None,
        StudioShellHostessStagingAcceptanceSelectionStatus::Missing => {
            Some("studio.issue.shell_hostess_staging_acceptance_not_found".to_string())
        }
        StudioShellHostessStagingAcceptanceSelectionStatus::Empty => {
            Some("studio.issue.shell_hostess_staging_acceptance_index_empty".to_string())
        }
    };

    StudioShellHostessStagingAcceptanceSelectionReport {
        schema_id: SHELL_HOSTESS_STAGING_ACCEPTANCE_SELECTION_SCHEMA.to_string(),
        source_index_schema: index.schema_id.clone(),
        index_path: index_path.map(|path| path.display().to_string()),
        requested_acceptance_id: requested_acceptance_id.map(str::to_string),
        default_acceptance_id: index.default_acceptance_id.clone(),
        selected_acceptance_id: selected_acceptance_id.clone(),
        status,
        issue_code,
        acceptance_count: index.acceptance_count,
        ready_acceptance_count: index.ready_acceptance_count,
        blocked_acceptance_count: index.blocked_acceptance_count,
        rejected_acceptance_count: index.rejected_acceptance_count,
        project_ids: index.project_ids.clone(),
        envelope_ids: index.envelope_ids.clone(),
        manifest_ids: index.manifest_ids.clone(),
        entries: index
            .entries
            .iter()
            .map(|entry| StudioShellHostessStagingAcceptanceSelectionEntry {
                acceptance_id: entry.acceptance_id.clone(),
                label: entry.label.clone(),
                selected: selected_acceptance_id.as_deref() == Some(entry.acceptance_id.as_str()),
                default: index.default_acceptance_id.as_deref()
                    == Some(entry.acceptance_id.as_str()),
                acceptance_manifest_path: entry.acceptance_manifest_path.clone(),
                checklist_path: entry.checklist_path.clone(),
                envelope_id: entry.envelope_id.clone(),
                project_id: entry.project_id.clone(),
                project_revision: entry.project_revision,
                status: entry.status,
                issue_code: entry.issue_code.clone(),
                ready_item_count: entry.ready_item_count,
                blocked_item_count: entry.blocked_item_count,
                rejected_item_count: entry.rejected_item_count,
                request_count: entry.request_count,
                instruction_count: entry.instruction_count,
            })
            .collect(),
    }
}

pub fn shell_hostess_staging_execution_request_for_acceptance_index_entry(
    acceptance_index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_index_path: Option<&Path>,
    acceptance_index_entry: &StudioShellHostessStagingAcceptanceIndexEntry,
    acceptance_manifest_path: Option<&Path>,
    acceptance: &StudioShellHostessStagingAcceptanceManifest,
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
    handoff_path: Option<&Path>,
    handoff: &StudioShellHostessStagingHandoffEnvelope,
) -> StudioShellHostessStagingExecutionRequestReport {
    shell_hostess_staging_execution_request_for_acceptance_index_entry_with_pmb_review(
        acceptance_index,
        acceptance_index_path,
        acceptance_index_entry,
        acceptance_manifest_path,
        acceptance,
        checklist,
        handoff_path,
        handoff,
        None,
        None,
        false,
    )
}

pub fn shell_hostess_staging_execution_request_for_acceptance_index_entry_with_pmb_review(
    acceptance_index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_index_path: Option<&Path>,
    acceptance_index_entry: &StudioShellHostessStagingAcceptanceIndexEntry,
    acceptance_manifest_path: Option<&Path>,
    acceptance: &StudioShellHostessStagingAcceptanceManifest,
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
    handoff_path: Option<&Path>,
    handoff: &StudioShellHostessStagingHandoffEnvelope,
    pmb_shell_handoff_review_path: Option<&Path>,
    pmb_shell_handoff_review: Option<&StudioProjectedMotionBreathShellHandoffReviewReport>,
    require_pmb_shell_handoff_review: bool,
) -> StudioShellHostessStagingExecutionRequestReport {
    let mut checks = Vec::new();
    let expected_manifest_path = acceptance_manifest_path.map(|path| path.display().to_string());
    let manifest_path_matches = match (
        expected_manifest_path.as_deref(),
        acceptance_index_entry.acceptance_manifest_path.as_deref(),
    ) {
        (Some(expected), Some(actual)) => actual == expected,
        (None, Some(actual)) => !actual.trim().is_empty(),
        _ => false,
    };
    let expected_handoff_path = handoff_path
        .map(|path| path.display().to_string())
        .or_else(|| checklist.handoff_path.clone());
    let expected_acceptance_specs =
        shell_hostess_staging_acceptance_item_specs(handoff, handoff_path);
    let expected_acceptance_entries_match = expected_acceptance_specs.len()
        == checklist.entries.len()
        && expected_acceptance_specs.iter().all(|spec| {
            checklist.entries.iter().any(|entry| {
                entry.item_id == spec.item_id
                    && entry.owner == spec.owner
                    && entry.status == StudioShellHostessStagingAcceptanceStatus::Ready
                    && entry.issue_code.is_none()
                    && entry.item_kind == spec.item_kind
                    && entry.route_kind == spec.route_kind
                    && entry.source == spec.source
                    && entry.prohibited_in_studio == spec.prohibited_in_studio
                    && entry.expected_input_path == spec.expected_input_path
            })
        });
    let pmb_shell_handoff_review_required =
        require_pmb_shell_handoff_review || pmb_shell_handoff_review_path.is_some();
    let pmb_shell_handoff_review_path_string =
        pmb_shell_handoff_review_path.map(|path| path.display().to_string());
    let pmb_shell_handoff_review_ready =
        pmb_shell_handoff_review_is_ready(pmb_shell_handoff_review);
    let pmb_shell_handoff_review_issue_code =
        pmb_shell_handoff_review_issue_code(pmb_shell_handoff_review);
    let hostess_operator_start_preflight_cli_args = hostess_operator_start_preflight_pmb_cli_args(
        pmb_shell_handoff_review_required,
        pmb_shell_handoff_review_path_string.as_deref(),
    );

    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_index_schema",
        acceptance_index.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA,
        "source Hostess acceptance index schema is supported",
        "source Hostess acceptance index schema is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.selected_acceptance",
        acceptance_index_entry.acceptance_id == acceptance.acceptance_id,
        "selected acceptance index entry matches the loaded acceptance manifest",
        "selected acceptance index entry differs from the loaded acceptance manifest",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_manifest_path",
        manifest_path_matches,
        "selected acceptance index entry names the loaded acceptance manifest",
        "selected acceptance index entry is missing or stale",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_manifest_schema",
        acceptance.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_MANIFEST_SCHEMA,
        "acceptance manifest schema is supported",
        "acceptance manifest schema is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_checklist_schema",
        acceptance.checklist_schema == checklist.schema_id
            && checklist.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA,
        "acceptance manifest names the loaded checklist schema",
        "acceptance manifest checklist schema differs from the loaded checklist",
        "studio.issue.shell_hostess_staging_acceptance_checklist_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_checklist_path",
        !acceptance.checklist_path.trim().is_empty()
            && acceptance_index_entry.checklist_path == acceptance.checklist_path,
        "acceptance manifest and index agree on the checklist path",
        "acceptance manifest and index checklist paths differ",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_source_metadata",
        acceptance.envelope_id == checklist.envelope_id
            && acceptance.manifest_id == checklist.manifest_id
            && acceptance.project_id == checklist.project_id
            && acceptance.project_revision == checklist.project_revision
            && acceptance_index_entry.envelope_id == acceptance.envelope_id
            && acceptance_index_entry.manifest_id == acceptance.manifest_id
            && acceptance_index_entry.project_id == acceptance.project_id
            && acceptance_index_entry.project_revision == acceptance.project_revision,
        "acceptance manifest, index, and checklist source metadata match",
        "acceptance manifest, index, and checklist source metadata differ",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_status_counts",
        acceptance.status == checklist.status
            && acceptance.issue_code == checklist.issue_code
            && acceptance.ready_item_count == checklist.ready_item_count
            && acceptance.blocked_item_count == checklist.blocked_item_count
            && acceptance.rejected_item_count == checklist.rejected_item_count
            && acceptance.request_count == checklist.request_count
            && acceptance.instruction_count == checklist.instruction_count
            && acceptance_index_entry.status == acceptance.status
            && acceptance_index_entry.issue_code == acceptance.issue_code
            && acceptance_index_entry.ready_item_count == acceptance.ready_item_count
            && acceptance_index_entry.blocked_item_count == acceptance.blocked_item_count
            && acceptance_index_entry.rejected_item_count == acceptance.rejected_item_count,
        "acceptance manifest, index, and checklist readiness counts match",
        "acceptance manifest, index, and checklist readiness counts differ",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_ready",
        checklist.status == StudioShellHostessStagingAcceptanceStatus::Ready,
        "selected Hostess staging acceptance is ready",
        "selected Hostess staging acceptance is not ready",
        checklist
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_staging_acceptance_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_execution_policy",
        acceptance.execution_policy == "not_executed.acceptance_check_only"
            && checklist.execution_policy == "not_executed.acceptance_check_only"
            && acceptance_index_entry.execution_policy == acceptance.execution_policy,
        "acceptance artifacts remain acceptance-check-only",
        "acceptance artifacts changed execution policy",
        "studio.issue.shell_hostess_staging_acceptance_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.handoff_schema",
        handoff.schema_id == SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA,
        "source Hostess staging handoff schema is supported",
        "source Hostess staging handoff schema is unsupported",
        "studio.issue.shell_hostess_staging_handoff_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.handoff_path",
        expected_handoff_path
            .as_deref()
            .is_some_and(|path| checklist.handoff_path.as_deref() == Some(path)),
        "acceptance checklist names the loaded handoff envelope",
        "acceptance checklist handoff path is missing or stale",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.handoff_ready",
        handoff.status == StudioShellHostessStagingHandoffEnvelopeStatus::Ready,
        "source Hostess staging handoff is ready",
        "source Hostess staging handoff is not ready",
        handoff
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_staging_handoff_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.handoff_execution_policy",
        handoff.execution_policy == "not_executed.handoff_only",
        "source handoff remains handoff-only",
        "source handoff execution policy changed",
        "studio.issue.shell_hostess_staging_handoff_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.handoff_identity",
        handoff.envelope_id == checklist.envelope_id
            && handoff.manifest_id == checklist.manifest_id
            && handoff.project_id == checklist.project_id
            && handoff.project_revision == checklist.project_revision
            && handoff.selected_candidate_id == checklist.selected_candidate_id,
        "handoff identity matches the selected acceptance checklist",
        "handoff identity differs from the selected acceptance checklist",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.handoff_provenance",
        handoff.file_plan_path == checklist.file_plan_path
            && handoff.preview_path == checklist.preview_path
            && handoff.intake_path == checklist.intake_path
            && handoff.package_path == checklist.package_path
            && handoff.handoff_manifest_path == checklist.handoff_manifest_path
            && handoff.provenance.checksum_algorithm == checklist.checksum_algorithm
            && handoff.provenance.plan_checksum == checklist.plan_checksum,
        "handoff provenance matches the selected acceptance checklist",
        "handoff provenance differs from the selected acceptance checklist",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.authority",
        acceptance.checklist_owner == "rusty.hostess"
            && acceptance.handoff_owner == "rusty.hostess"
            && acceptance.staging_owner == "rusty.hostess"
            && checklist.checklist_owner == "rusty.hostess"
            && checklist.handoff_owner == "rusty.hostess"
            && checklist.staging_owner == "rusty.hostess"
            && handoff.handoff_owner == "rusty.hostess"
            && handoff.staging_owner == "rusty.hostess"
            && checklist.command_session_authority.as_deref() == Some("rusty.manifold")
            && checklist.install_launch_evidence_authority.as_deref() == Some("rusty.hostess")
            && checklist.studio_role.as_deref() == Some("authoring.export_planning"),
        "Hostess, Manifold, and Studio authority fields remain separated",
        "Hostess, Manifold, or Studio authority fields drifted",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_entry_contracts",
        expected_acceptance_entries_match,
        "acceptance entries match the expected Hostess/Manifold adapter contracts",
        "acceptance entries drifted from expected Hostess/Manifold adapter contracts",
        "studio.issue.shell_hostess_staging_acceptance_entry_drift",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.no_failed_handoff_checks",
        checklist
            .handoff_checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "acceptance checklist carries no failed handoff checks",
        "acceptance checklist carries failed handoff checks",
        "studio.issue.shell_hostess_staging_handoff_failed_check",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.prohibited_studio_actions",
        [
            "stage_generated_shells",
            "install",
            "launch",
            "open_command_session",
            "collect_device_evidence",
            "collect_install_launch_evidence",
        ]
        .iter()
        .all(|action| {
            acceptance
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action)
                && checklist
                    .prohibited_actions
                    .iter()
                    .any(|candidate| candidate == action)
        }),
        "execution request preserves all Studio prohibitions",
        "execution request is missing one or more Studio prohibitions",
        "studio.issue.shell_hostess_staging_acceptance_prohibited_action_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.pmb_shell_handoff_review",
        !pmb_shell_handoff_review_required
            || (pmb_shell_handoff_review_path_string.is_some()
                && pmb_shell_handoff_review_ready
                && pmb_shell_handoff_review_issue_code.is_none()),
        "PMB shell handoff review is ready for Hostess operator-start preflight",
        "PMB shell handoff review is missing, blocked, or invalid",
        pmb_shell_handoff_review_issue_code
            .as_deref()
            .unwrap_or("studio.issue.projected_motion_breath_shell_handoff_review_missing"),
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_staging_acceptance_index_schema")
                    | Some("studio.issue.shell_hostess_staging_acceptance_manifest_schema")
                    | Some("studio.issue.shell_hostess_staging_acceptance_checklist_schema")
                    | Some("studio.issue.shell_hostess_staging_handoff_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessStagingExecutionRequestStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessStagingExecutionRequestStatus::Blocked
    } else {
        StudioShellHostessStagingExecutionRequestStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessStagingExecutionRequestStatus::Ready => None,
        StudioShellHostessStagingExecutionRequestStatus::Blocked
        | StudioShellHostessStagingExecutionRequestStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let request_id =
        default_shell_hostess_staging_execution_request_id(&acceptance.acceptance_id, checklist);
    let actions = shell_hostess_staging_execution_actions(checklist, status, issue_code.as_deref());
    let ready_adapter_action_count = actions
        .iter()
        .filter(|action| action.status == StudioShellHostessStagingExecutionActionStatus::Ready)
        .count();
    let blocked_adapter_action_count = actions
        .iter()
        .filter(|action| action.status == StudioShellHostessStagingExecutionActionStatus::Blocked)
        .count();
    let required_action_ids = actions
        .iter()
        .map(|action| action.action_id.clone())
        .collect::<Vec<_>>();
    let ack_template = shell_hostess_staging_execution_ack_template(
        &request_id,
        required_action_ids.clone(),
        checklist.command_session_authority.clone(),
        checklist.install_launch_evidence_authority.clone(),
    );
    let reject_template =
        shell_hostess_staging_execution_reject_template(&request_id, required_action_ids);

    StudioShellHostessStagingExecutionRequestReport {
        schema_id: SHELL_HOSTESS_STAGING_EXECUTION_REQUEST_SCHEMA.to_string(),
        request_id,
        source_acceptance_index_schema: Some(acceptance_index.schema_id.clone()),
        acceptance_index_path: acceptance_index_path.map(|path| path.display().to_string()),
        selected_acceptance_id: acceptance.acceptance_id.clone(),
        acceptance_manifest_path: acceptance_manifest_path.map(|path| path.display().to_string()),
        acceptance_schema: acceptance.schema_id.clone(),
        acceptance_checklist_path: acceptance.checklist_path.clone(),
        acceptance_checklist_schema: checklist.schema_id.clone(),
        source_acceptance_status: checklist.status,
        source_handoff_schema: handoff.schema_id.clone(),
        handoff_path: expected_handoff_path,
        envelope_id: checklist.envelope_id.clone(),
        manifest_id: checklist.manifest_id.clone(),
        project_id: checklist.project_id.clone(),
        project_revision: checklist.project_revision,
        selected_candidate_id: checklist.selected_candidate_id.clone(),
        file_plan_path: checklist.file_plan_path.clone(),
        preview_path: checklist.preview_path.clone(),
        intake_path: checklist.intake_path.clone(),
        package_path: checklist.package_path.clone(),
        handoff_manifest_path: checklist.handoff_manifest_path.clone(),
        pmb_shell_handoff_review_required,
        pmb_shell_handoff_review_path: pmb_shell_handoff_review_path_string,
        source_pmb_shell_handoff_review_schema: pmb_shell_handoff_review
            .map(|review| review.schema_id.clone()),
        source_pmb_shell_handoff_review_status: pmb_shell_handoff_review
            .map(|review| review.status),
        source_pmb_shell_handoff_review_issue_code: pmb_shell_handoff_review
            .and_then(|review| review.issue_code.clone()),
        source_pmb_shell_handoff_id: pmb_shell_handoff_review
            .and_then(|review| review.handoff_id.clone()),
        source_pmb_shell_app_id: pmb_shell_handoff_review
            .and_then(|review| review.shell_app_id.clone()),
        pmb_shell_handoff_review_ready,
        hostess_operator_start_preflight_cli_args,
        status,
        issue_code,
        execution_policy: "not_executed.hostess_request_only".to_string(),
        adapter_owner: "rusty.hostess".to_string(),
        requester_role: "rusty.studio".to_string(),
        command_session_authority: checklist.command_session_authority.clone(),
        install_launch_evidence_authority: checklist.install_launch_evidence_authority.clone(),
        studio_role: checklist.studio_role.clone(),
        request_count: checklist.request_count,
        ready_request_count: checklist.ready_request_count,
        blocked_request_count: checklist.blocked_request_count,
        instruction_count: checklist.instruction_count,
        ready_instruction_count: checklist.ready_instruction_count,
        blocked_instruction_count: checklist.blocked_instruction_count,
        checksum_algorithm: checklist.checksum_algorithm.clone(),
        plan_checksum: checklist.plan_checksum.clone(),
        prohibited_studio_actions: checklist.prohibited_actions.clone(),
        adapter_action_count: actions.len(),
        ready_adapter_action_count,
        blocked_adapter_action_count,
        actions,
        checks,
        ack_template,
        reject_template,
    }
}

fn pmb_shell_handoff_review_is_ready(
    review: Option<&StudioProjectedMotionBreathShellHandoffReviewReport>,
) -> bool {
    let Some(review) = review else {
        return false;
    };
    review.schema_id == PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA
        && review.status == StudioProjectedMotionBreathShellHandoffReviewStatus::Ready
        && review.issue_code.is_none()
        && review.execution_policy == "not_executed.review_only"
        && review.runtime_authority == "rusty.manifold"
        && review.authoring_authority == "rusty.studio"
        && review.platform_validation_authority == "rusty.hostess"
        && !review.runtime_execution_performed
        && !review.platform_execution_performed
        && !review.broker_transport_used
        && !review.downstream_shell_runtime_used
        && !review.legacy_app_dependency_used
        && review.required_binding_count > 0
        && review.ready_required_binding_count == review.required_binding_count
        && review.feedback_receipt_exported
        && review.feedback_sink_provides_receipt
        && review
            .command_ids
            .iter()
            .any(|command_id| command_id == "command.breath.status")
        && !review.transport_ids.is_empty()
}

fn pmb_shell_handoff_review_issue_code(
    review: Option<&StudioProjectedMotionBreathShellHandoffReviewReport>,
) -> Option<String> {
    let Some(review) = review else {
        return Some(
            "studio.issue.projected_motion_breath_shell_handoff_review_missing".to_string(),
        );
    };
    if review.schema_id != PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA {
        return Some(
            "studio.issue.projected_motion_breath_shell_handoff_review_schema".to_string(),
        );
    }
    if review.status != StudioProjectedMotionBreathShellHandoffReviewStatus::Ready {
        return Some(review.issue_code.clone().unwrap_or_else(|| {
            "studio.issue.projected_motion_breath_shell_handoff_review_not_ready".to_string()
        }));
    }
    if !pmb_shell_handoff_review_is_ready(Some(review)) {
        return Some(review.issue_code.clone().unwrap_or_else(|| {
            "studio.issue.projected_motion_breath_shell_handoff_review_boundary".to_string()
        }));
    }
    None
}

fn hostess_operator_start_preflight_pmb_cli_args(
    pmb_shell_handoff_review_required: bool,
    pmb_shell_handoff_review_path: Option<&str>,
) -> Vec<String> {
    if !pmb_shell_handoff_review_required {
        return Vec::new();
    }
    let mut args = Vec::new();
    if let Some(path) = pmb_shell_handoff_review_path {
        args.push("--pmb-shell-handoff-review-in".to_string());
        args.push(path.to_string());
    }
    args.push("--require-pmb-shell-handoff-review".to_string());
    args
}

fn shell_hostess_staging_execution_actions(
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
    request_status: StudioShellHostessStagingExecutionRequestStatus,
    request_issue_code: Option<&str>,
) -> Vec<StudioShellHostessStagingExecutionAction> {
    checklist
        .entries
        .iter()
        .map(|entry| {
            let status = if request_status == StudioShellHostessStagingExecutionRequestStatus::Ready
                && entry.status == StudioShellHostessStagingAcceptanceStatus::Ready
                && entry.prohibited_in_studio
            {
                StudioShellHostessStagingExecutionActionStatus::Ready
            } else {
                StudioShellHostessStagingExecutionActionStatus::Blocked
            };
            StudioShellHostessStagingExecutionAction {
                action_id: format!("adapter.{}", entry.item_id),
                owner: entry.owner.clone(),
                status,
                issue_code: (status == StudioShellHostessStagingExecutionActionStatus::Blocked)
                    .then(|| {
                        entry
                            .issue_code
                            .as_deref()
                            .or(request_issue_code)
                            .unwrap_or(
                                "studio.issue.shell_hostess_staging_execution_request_blocked",
                            )
                            .to_string()
                    }),
                action_kind: entry.item_kind.clone(),
                route_kind: entry.route_kind.clone(),
                source_item_id: entry.item_id.clone(),
                responsible_authority: entry.owner.clone(),
                expected_input_path: entry.expected_input_path.clone(),
                next_required_action: entry.next_required_action.clone(),
                ack_required: true,
                execution_in_studio: false,
            }
        })
        .collect()
}

fn shell_hostess_staging_execution_ack_template(
    request_id: &str,
    required_action_ids: Vec<String>,
    command_session_authority: Option<String>,
    install_launch_evidence_authority: Option<String>,
) -> StudioShellHostessStagingExecutionAck {
    StudioShellHostessStagingExecutionAck {
        schema_id: SHELL_HOSTESS_STAGING_EXECUTION_ACK_SCHEMA.to_string(),
        request_id: request_id.to_string(),
        accepted_by: "rusty.hostess".to_string(),
        ack_status: StudioShellHostessStagingExecutionAckStatus::Pending,
        execution_in_studio: false,
        command_session_authority,
        install_launch_evidence_authority,
        required_action_ids,
        accepted_action_ids: Vec::new(),
        required_evidence_kinds: vec![
            "hostess_staging_request_ack".to_string(),
            "hostess_file_copy_stage_receipt".to_string(),
            "hostess_install_launch_evidence_receipt".to_string(),
            "manifold_command_session_contract_review".to_string(),
        ],
        issue_code: None,
    }
}

fn shell_hostess_staging_execution_reject_template(
    request_id: &str,
    request_action_ids: Vec<String>,
) -> StudioShellHostessStagingExecutionReject {
    StudioShellHostessStagingExecutionReject {
        schema_id: SHELL_HOSTESS_STAGING_EXECUTION_REJECT_SCHEMA.to_string(),
        request_id: request_id.to_string(),
        rejected_by: "rusty.hostess".to_string(),
        reject_status: StudioShellHostessStagingExecutionRejectStatus::Pending,
        execution_in_studio: false,
        request_action_ids,
        rejected_action_ids: Vec::new(),
        reason_code: None,
        next_required_action: "hostess_ack_or_reject_request_outside_studio".to_string(),
        issue_code: None,
    }
}

pub fn compare_shell_hostess_staging_acceptance_checklists(
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> StudioShellHostessStagingAcceptanceComparisonReport {
    compare_shell_hostess_staging_acceptance_checklists_with_identity(
        baseline, candidate, None, None,
    )
}

pub fn compare_shell_hostess_staging_acceptance_against_manifest(
    baseline_identity: &StudioShellHostessStagingAcceptanceManifest,
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> StudioShellHostessStagingAcceptanceComparisonReport {
    compare_shell_hostess_staging_acceptance_checklists_with_identity(
        baseline,
        candidate,
        Some(baseline_identity),
        None,
    )
}

pub fn compare_shell_hostess_staging_acceptance_against_index_entry(
    acceptance_index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_index_path: Option<&Path>,
    acceptance_index_entry: &StudioShellHostessStagingAcceptanceIndexEntry,
    acceptance_manifest_path: Option<&Path>,
    baseline_identity: &StudioShellHostessStagingAcceptanceManifest,
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> StudioShellHostessStagingAcceptanceComparisonReport {
    compare_shell_hostess_staging_acceptance_checklists_with_identity(
        baseline,
        candidate,
        Some(baseline_identity),
        Some(ShellHostessStagingAcceptanceIndexComparisonContext {
            index: acceptance_index,
            index_path: acceptance_index_path,
            entry: acceptance_index_entry,
            acceptance_manifest_path,
        }),
    )
}

struct ShellHostessStagingAcceptanceIndexComparisonContext<'a> {
    index: &'a StudioShellHostessStagingAcceptanceIndex,
    index_path: Option<&'a Path>,
    entry: &'a StudioShellHostessStagingAcceptanceIndexEntry,
    acceptance_manifest_path: Option<&'a Path>,
}

fn compare_shell_hostess_staging_acceptance_checklists_with_identity(
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
    baseline_identity: Option<&StudioShellHostessStagingAcceptanceManifest>,
    acceptance_index: Option<ShellHostessStagingAcceptanceIndexComparisonContext<'_>>,
) -> StudioShellHostessStagingAcceptanceComparisonReport {
    let mut checks = shell_hostess_staging_acceptance_comparison_checks(baseline, candidate);
    if let Some(baseline_identity) = baseline_identity {
        checks.extend(shell_hostess_staging_acceptance_baseline_identity_checks(
            baseline_identity,
            baseline,
        ));
        if let Some(acceptance_index) = acceptance_index.as_ref() {
            checks.extend(shell_hostess_staging_acceptance_index_entry_checks(
                acceptance_index,
                baseline_identity,
            ));
        }
    }
    let comparable = checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass);

    let entries = if comparable {
        shell_hostess_staging_acceptance_comparison_entries(baseline, candidate)
    } else {
        Vec::new()
    };
    let has_entry_contract_drift = entries
        .iter()
        .any(|entry| entry.change == StudioShellHostessStagingAcceptanceComparisonChange::Changed);
    if comparable {
        push_check(
            &mut checks,
            "studio.check.shell_hostess_staging_acceptance_comparison.entry_contracts",
            !has_entry_contract_drift,
            "baseline and candidate acceptance entry contracts match",
            "baseline and candidate acceptance entry contracts drifted",
            "studio.issue.shell_hostess_staging_acceptance_entry_drift",
        );
    }
    let comparable = checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass);

    let ready_item_delta = count_delta(candidate.ready_item_count, baseline.ready_item_count);
    let blocked_item_delta = count_delta(candidate.blocked_item_count, baseline.blocked_item_count);
    let rejected_item_delta =
        count_delta(candidate.rejected_item_count, baseline.rejected_item_count);

    let status = if has_entry_contract_drift || !comparable {
        StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable
    } else if shell_hostess_staging_acceptance_status_score(candidate.status)
        < shell_hostess_staging_acceptance_status_score(baseline.status)
        || ready_item_delta < 0
        || blocked_item_delta > 0
        || rejected_item_delta > 0
        || entries.iter().any(|entry| {
            matches!(
                entry.change,
                StudioShellHostessStagingAcceptanceComparisonChange::Regressed
                    | StudioShellHostessStagingAcceptanceComparisonChange::Removed
            )
        })
    {
        StudioShellHostessStagingAcceptanceComparisonStatus::Regressed
    } else if shell_hostess_staging_acceptance_status_score(candidate.status)
        > shell_hostess_staging_acceptance_status_score(baseline.status)
        || ready_item_delta > 0
        || blocked_item_delta < 0
        || rejected_item_delta < 0
        || entries.iter().any(|entry| {
            entry.change == StudioShellHostessStagingAcceptanceComparisonChange::Improved
        })
    {
        StudioShellHostessStagingAcceptanceComparisonStatus::Improved
    } else {
        StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged
    };

    let issue_code = match status {
        StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable => {
            first_failed_validation_check_issue_code(&checks)
        }
        StudioShellHostessStagingAcceptanceComparisonStatus::Regressed => entries
            .iter()
            .find(|entry| {
                matches!(
                    entry.change,
                    StudioShellHostessStagingAcceptanceComparisonChange::Regressed
                        | StudioShellHostessStagingAcceptanceComparisonChange::Removed
                )
            })
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| {
                candidate.issue_code.clone().or_else(|| {
                    Some("studio.issue.shell_hostess_staging_acceptance_regressed".to_string())
                })
            }),
        StudioShellHostessStagingAcceptanceComparisonStatus::Improved
        | StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged => None,
    };

    StudioShellHostessStagingAcceptanceComparisonReport {
        schema_id: SHELL_HOSTESS_STAGING_ACCEPTANCE_COMPARISON_SCHEMA.to_string(),
        baseline_identity_schema: baseline_identity.map(|identity| identity.schema_id.clone()),
        baseline_acceptance_id: baseline_identity.map(|identity| identity.acceptance_id.clone()),
        baseline_label: baseline_identity.map(|identity| identity.label.clone()),
        baseline_checklist_path: baseline_identity.map(|identity| identity.checklist_path.clone()),
        baseline_index_schema: acceptance_index
            .as_ref()
            .map(|context| context.index.schema_id.clone()),
        baseline_index_path: acceptance_index
            .as_ref()
            .and_then(|context| context.index_path.map(|path| path.display().to_string())),
        baseline_index_default_acceptance_id: acceptance_index
            .as_ref()
            .and_then(|context| context.index.default_acceptance_id.clone()),
        baseline_index_selected_acceptance_id: acceptance_index
            .as_ref()
            .map(|context| context.entry.acceptance_id.clone()),
        baseline_schema: baseline.schema_id.clone(),
        candidate_schema: candidate.schema_id.clone(),
        baseline_envelope_id: baseline.envelope_id.clone(),
        candidate_envelope_id: candidate.envelope_id.clone(),
        baseline_manifest_id: baseline.manifest_id.clone(),
        candidate_manifest_id: candidate.manifest_id.clone(),
        baseline_project_id: baseline.project_id.clone(),
        candidate_project_id: candidate.project_id.clone(),
        baseline_project_revision: baseline.project_revision,
        candidate_project_revision: candidate.project_revision,
        baseline_status: baseline.status,
        candidate_status: candidate.status,
        status,
        issue_code,
        baseline_ready_item_count: baseline.ready_item_count,
        candidate_ready_item_count: candidate.ready_item_count,
        ready_item_delta,
        baseline_blocked_item_count: baseline.blocked_item_count,
        candidate_blocked_item_count: candidate.blocked_item_count,
        blocked_item_delta,
        baseline_rejected_item_count: baseline.rejected_item_count,
        candidate_rejected_item_count: candidate.rejected_item_count,
        rejected_item_delta,
        checks,
        entries,
    }
}

fn shell_hostess_staging_acceptance_comparison_checks(
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_schema",
        baseline.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA,
        "baseline Hostess staging acceptance schema id is supported",
        "baseline Hostess staging acceptance schema id is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_checklist_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.candidate_schema",
        candidate.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA,
        "candidate Hostess staging acceptance schema id is supported",
        "candidate Hostess staging acceptance schema id is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_checklist_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.source_schema",
        baseline.source_handoff_schema == SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA
            && candidate.source_handoff_schema == SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA,
        "baseline and candidate source handoff schemas are supported",
        "baseline or candidate source handoff schema is unsupported",
        "studio.issue.shell_hostess_staging_handoff_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.project_metadata",
        baseline.project_id == candidate.project_id
            && baseline.project_revision == candidate.project_revision
            && baseline.manifest_id == candidate.manifest_id
            && baseline.selected_candidate_id == candidate.selected_candidate_id,
        "baseline and candidate project metadata matches",
        "baseline and candidate project metadata differs",
        "studio.issue.shell_hostess_staging_acceptance_source_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.envelope",
        baseline.envelope_id == candidate.envelope_id,
        "baseline and candidate Hostess staging envelopes match",
        "baseline and candidate Hostess staging envelopes differ",
        "studio.issue.shell_hostess_staging_acceptance_envelope_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.plan_checksum",
        baseline.checksum_algorithm == candidate.checksum_algorithm
            && baseline.plan_checksum == candidate.plan_checksum,
        "baseline and candidate staging file-plan checksums match",
        "baseline and candidate staging file-plan checksums differ",
        "studio.issue.shell_hostess_staging_acceptance_checksum_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.prohibited_actions",
        string_set(&baseline.prohibited_actions) == string_set(&candidate.prohibited_actions),
        "baseline and candidate Studio-prohibited actions match",
        "baseline and candidate Studio-prohibited actions differ",
        "studio.issue.shell_hostess_staging_acceptance_prohibited_actions_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.execution_policy",
        baseline.execution_policy == "not_executed.acceptance_check_only"
            && candidate.execution_policy == "not_executed.acceptance_check_only",
        "baseline and candidate remain acceptance-check-only",
        "baseline or candidate is no longer acceptance-check-only",
        "studio.issue.shell_hostess_staging_acceptance_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.owner_authority",
        baseline.checklist_owner == "rusty.hostess"
            && candidate.checklist_owner == "rusty.hostess"
            && baseline.handoff_owner == "rusty.hostess"
            && candidate.handoff_owner == "rusty.hostess"
            && baseline.staging_owner == "rusty.hostess"
            && candidate.staging_owner == "rusty.hostess",
        "baseline and candidate preserve Hostess ownership",
        "baseline or candidate changed Hostess ownership",
        "studio.issue.shell_hostess_staging_acceptance_owner_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.runtime_authority",
        baseline.command_session_authority.as_deref() == Some("rusty.manifold")
            && candidate.command_session_authority.as_deref() == Some("rusty.manifold")
            && baseline.install_launch_evidence_authority.as_deref() == Some("rusty.hostess")
            && candidate.install_launch_evidence_authority.as_deref() == Some("rusty.hostess")
            && baseline.studio_role.as_deref() == Some("authoring.export_planning")
            && candidate.studio_role.as_deref() == Some("authoring.export_planning"),
        "baseline and candidate preserve Manifold, Hostess, and Studio authority",
        "baseline or candidate changed Manifold, Hostess, or Studio authority",
        "studio.issue.runtime_authority_mismatch",
    );
    checks
}

fn shell_hostess_staging_acceptance_baseline_identity_checks(
    baseline_identity: &StudioShellHostessStagingAcceptanceManifest,
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_schema",
        baseline_identity.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_MANIFEST_SCHEMA,
        "baseline identity schema id is supported",
        "baseline identity schema id is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_checklist_schema",
        baseline_identity.checklist_schema == baseline.schema_id,
        "baseline identity names the loaded checklist schema",
        "baseline identity does not name the loaded checklist schema",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_checklist_path",
        !baseline_identity.checklist_path.trim().is_empty(),
        "baseline identity has a durable checklist path",
        "baseline identity checklist path is missing",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_source_metadata",
        baseline_identity.envelope_id == baseline.envelope_id
            && baseline_identity.manifest_id == baseline.manifest_id
            && baseline_identity.project_id == baseline.project_id
            && baseline_identity.project_revision == baseline.project_revision,
        "baseline identity source metadata matches the loaded checklist",
        "baseline identity source metadata differs from the loaded checklist",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_status_counts",
        baseline_identity.status == baseline.status
            && baseline_identity.issue_code == baseline.issue_code
            && baseline_identity.ready_item_count == baseline.ready_item_count
            && baseline_identity.blocked_item_count == baseline.blocked_item_count
            && baseline_identity.rejected_item_count == baseline.rejected_item_count
            && baseline_identity.request_count == baseline.request_count
            && baseline_identity.instruction_count == baseline.instruction_count,
        "baseline identity readiness counts match the loaded checklist",
        "baseline identity readiness counts differ from the loaded checklist",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_authority",
        baseline_identity.execution_policy == baseline.execution_policy
            && baseline_identity.checklist_owner == baseline.checklist_owner
            && baseline_identity.handoff_owner == baseline.handoff_owner
            && baseline_identity.staging_owner == baseline.staging_owner
            && baseline_identity.command_session_authority == baseline.command_session_authority
            && baseline_identity.install_launch_evidence_authority
                == baseline.install_launch_evidence_authority
            && baseline_identity.studio_role == baseline.studio_role,
        "baseline identity authority fields match the loaded checklist",
        "baseline identity authority fields differ from the loaded checklist",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_provenance",
        baseline_identity.checksum_algorithm == baseline.checksum_algorithm
            && baseline_identity.plan_checksum == baseline.plan_checksum
            && string_set(&baseline_identity.prohibited_actions)
                == string_set(&baseline.prohibited_actions),
        "baseline identity provenance matches the loaded checklist",
        "baseline identity provenance differs from the loaded checklist",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    checks
}

fn shell_hostess_staging_acceptance_index_entry_checks(
    context: &ShellHostessStagingAcceptanceIndexComparisonContext<'_>,
    baseline_identity: &StudioShellHostessStagingAcceptanceManifest,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    let entry = context.entry;
    let expected_manifest_path = context
        .acceptance_manifest_path
        .map(|path| path.display().to_string());
    let manifest_path_matches = match (
        expected_manifest_path.as_deref(),
        entry.acceptance_manifest_path.as_deref(),
    ) {
        (Some(expected), Some(actual)) => actual == expected,
        (None, Some(actual)) => !actual.trim().is_empty(),
        _ => false,
    };

    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_schema",
        context.index.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA,
        "baseline acceptance index schema id is supported",
        "baseline acceptance index schema id is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_selected_acceptance",
        entry.acceptance_id == baseline_identity.acceptance_id,
        "baseline acceptance index selected entry matches the loaded identity",
        "baseline acceptance index selected entry differs from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_manifest_path",
        manifest_path_matches,
        "baseline acceptance index entry manifest path names the loaded identity",
        "baseline acceptance index entry manifest path is missing or stale",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_checklist_path",
        entry.checklist_path == baseline_identity.checklist_path
            && entry.checklist_schema == baseline_identity.checklist_schema,
        "baseline acceptance index checklist references match the loaded identity",
        "baseline acceptance index checklist references differ from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_source_metadata",
        entry.envelope_id == baseline_identity.envelope_id
            && entry.manifest_id == baseline_identity.manifest_id
            && entry.project_id == baseline_identity.project_id
            && entry.project_revision == baseline_identity.project_revision,
        "baseline acceptance index source metadata matches the loaded identity",
        "baseline acceptance index source metadata differs from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_status_counts",
        entry.status == baseline_identity.status
            && entry.issue_code == baseline_identity.issue_code
            && entry.ready_item_count == baseline_identity.ready_item_count
            && entry.blocked_item_count == baseline_identity.blocked_item_count
            && entry.rejected_item_count == baseline_identity.rejected_item_count
            && entry.request_count == baseline_identity.request_count
            && entry.instruction_count == baseline_identity.instruction_count,
        "baseline acceptance index readiness counts match the loaded identity",
        "baseline acceptance index readiness counts differ from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_authority",
        entry.execution_policy == baseline_identity.execution_policy
            && entry.checklist_owner == baseline_identity.checklist_owner
            && entry.handoff_owner == baseline_identity.handoff_owner
            && entry.staging_owner == baseline_identity.staging_owner
            && entry.command_session_authority == baseline_identity.command_session_authority
            && entry.install_launch_evidence_authority
                == baseline_identity.install_launch_evidence_authority
            && entry.studio_role == baseline_identity.studio_role,
        "baseline acceptance index authority fields match the loaded identity",
        "baseline acceptance index authority fields differ from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_provenance",
        entry.checksum_algorithm == baseline_identity.checksum_algorithm
            && entry.plan_checksum == baseline_identity.plan_checksum,
        "baseline acceptance index checksum references match the loaded identity",
        "baseline acceptance index checksum references differ from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    checks
}

fn shell_hostess_staging_acceptance_comparison_entries(
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> Vec<StudioShellHostessStagingAcceptanceComparisonEntry> {
    let baseline_entries = baseline
        .entries
        .iter()
        .map(|entry| (entry.item_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let candidate_entries = candidate
        .entries
        .iter()
        .map(|entry| (entry.item_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let item_ids = baseline_entries
        .keys()
        .chain(candidate_entries.keys())
        .map(|item_id| (*item_id).to_string())
        .collect::<BTreeSet<_>>();

    item_ids
        .into_iter()
        .map(|item_id| {
            shell_hostess_staging_acceptance_comparison_entry(
                &item_id,
                baseline_entries.get(item_id.as_str()).copied(),
                candidate_entries.get(item_id.as_str()).copied(),
            )
        })
        .collect()
}

fn shell_hostess_staging_acceptance_comparison_entry(
    item_id: &str,
    baseline: Option<&StudioShellHostessStagingAcceptanceChecklistEntry>,
    candidate: Option<&StudioShellHostessStagingAcceptanceChecklistEntry>,
) -> StudioShellHostessStagingAcceptanceComparisonEntry {
    let baseline_score =
        baseline.map(|entry| shell_hostess_staging_acceptance_status_score(entry.status));
    let candidate_score =
        candidate.map(|entry| shell_hostess_staging_acceptance_status_score(entry.status));
    let score_delta = candidate_score.unwrap_or(0) - baseline_score.unwrap_or(0);
    let change = match (baseline, candidate) {
        (None, Some(_)) => StudioShellHostessStagingAcceptanceComparisonChange::Added,
        (Some(_), None) => StudioShellHostessStagingAcceptanceComparisonChange::Removed,
        (Some(_), Some(_)) if score_delta > 0 => {
            StudioShellHostessStagingAcceptanceComparisonChange::Improved
        }
        (Some(_), Some(_)) if score_delta < 0 => {
            StudioShellHostessStagingAcceptanceComparisonChange::Regressed
        }
        (Some(baseline), Some(candidate))
            if baseline.owner != candidate.owner
                || baseline.route_kind != candidate.route_kind
                || baseline.issue_code != candidate.issue_code
                || baseline.prohibited_in_studio != candidate.prohibited_in_studio
                || baseline.expected_input_path != candidate.expected_input_path =>
        {
            StudioShellHostessStagingAcceptanceComparisonChange::Changed
        }
        (Some(_), Some(_)) => StudioShellHostessStagingAcceptanceComparisonChange::Unchanged,
        (None, None) => StudioShellHostessStagingAcceptanceComparisonChange::Unchanged,
    };
    let issue_code = match change {
        StudioShellHostessStagingAcceptanceComparisonChange::Regressed
        | StudioShellHostessStagingAcceptanceComparisonChange::Removed => candidate
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| baseline.and_then(|entry| entry.issue_code.clone()))
            .or_else(|| {
                Some("studio.issue.shell_hostess_staging_acceptance_regressed".to_string())
            }),
        StudioShellHostessStagingAcceptanceComparisonChange::Added
        | StudioShellHostessStagingAcceptanceComparisonChange::Improved
        | StudioShellHostessStagingAcceptanceComparisonChange::Unchanged => None,
        StudioShellHostessStagingAcceptanceComparisonChange::Changed => {
            Some("studio.issue.shell_hostess_staging_acceptance_entry_drift".to_string())
        }
    };

    StudioShellHostessStagingAcceptanceComparisonEntry {
        item_id: item_id.to_string(),
        owner: candidate
            .map(|entry| entry.owner.clone())
            .or_else(|| baseline.map(|entry| entry.owner.clone()))
            .unwrap_or_else(|| "unknown".to_string()),
        baseline_status: baseline.map(|entry| entry.status),
        candidate_status: candidate.map(|entry| entry.status),
        change,
        score_delta,
        baseline_route_kind: baseline.map(|entry| entry.route_kind.clone()),
        candidate_route_kind: candidate.map(|entry| entry.route_kind.clone()),
        baseline_issue_code: baseline.and_then(|entry| entry.issue_code.clone()),
        candidate_issue_code: candidate.and_then(|entry| entry.issue_code.clone()),
        issue_code,
    }
}

fn default_shell_hostess_staging_acceptance_id(
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> String {
    format!(
        "studio.hostess_staging_acceptance.{}.rev{}.{}",
        checklist.project_id.as_deref().unwrap_or("unknown_project"),
        checklist
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        shell_hostess_staging_acceptance_status_key(checklist.status)
    )
}

fn default_shell_hostess_staging_execution_request_id(
    acceptance_id: &str,
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> String {
    format!(
        "studio.hostess_staging_execution_request.{}.rev{}.{}",
        checklist.project_id.as_deref().unwrap_or("unknown_project"),
        checklist
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        acceptance_id
    )
}

fn default_shell_hostess_staging_acceptance_label(
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> String {
    format!(
        "{} revision {} {} Hostess staging acceptance",
        checklist.project_id.as_deref().unwrap_or("unknown project"),
        checklist
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        shell_hostess_staging_acceptance_status_key(checklist.status)
    )
}

fn shell_hostess_staging_acceptance_status_key(
    status: StudioShellHostessStagingAcceptanceStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingAcceptanceStatus::Ready => "ready",
        StudioShellHostessStagingAcceptanceStatus::Blocked => "blocked",
        StudioShellHostessStagingAcceptanceStatus::Rejected => "rejected",
    }
}

fn shell_hostess_staging_acceptance_status_score(
    status: StudioShellHostessStagingAcceptanceStatus,
) -> isize {
    match status {
        StudioShellHostessStagingAcceptanceStatus::Rejected => 0,
        StudioShellHostessStagingAcceptanceStatus::Blocked => 1,
        StudioShellHostessStagingAcceptanceStatus::Ready => 2,
    }
}

fn default_shell_release_candidate_review_id(
    review: &StudioShellReleaseCandidateReviewReport,
) -> String {
    format!(
        "{}.rev{}.{}",
        review.project_id,
        review.project_revision,
        shell_release_candidate_review_status_key(review.status)
    )
}

fn default_shell_release_candidate_review_label(
    review: &StudioShellReleaseCandidateReviewReport,
) -> String {
    format!(
        "{} revision {} {} release candidate",
        review.project_id,
        review.project_revision,
        shell_release_candidate_review_status_key(review.status)
    )
}

fn shell_release_candidate_review_status_key(
    status: StudioShellReleaseCandidateReviewStatus,
) -> &'static str {
    match status {
        StudioShellReleaseCandidateReviewStatus::Ready => "ready",
        StudioShellReleaseCandidateReviewStatus::Blocked => "blocked",
        StudioShellReleaseCandidateReviewStatus::Rejected => "rejected",
    }
}

fn count_delta(candidate: usize, baseline: usize) -> isize {
    candidate as isize - baseline as isize
}

fn string_set(values: &[String]) -> BTreeSet<String> {
    values.iter().cloned().collect()
}

fn validate_shell_handoff_manifest_counts(
    manifest: &StudioShellHandoffManifest,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let ready_count = manifest
        .handoffs
        .iter()
        .filter(|handoff| handoff.status == StudioValidationStatus::Pass)
        .count();
    let failed_count = manifest
        .handoffs
        .iter()
        .filter(|handoff| handoff.status == StudioValidationStatus::Fail)
        .count();
    let missing_bundle_count = manifest
        .handoffs
        .iter()
        .filter(|handoff| {
            handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
        })
        .count();
    push_check(
        checks,
        "studio.check.shell_handoff_manifest.graph_count",
        manifest.graph_count == manifest.handoffs.len(),
        "graph count matches declared handoffs",
        &format!(
            "graph count {} does not match {} handoffs",
            manifest.graph_count,
            manifest.handoffs.len()
        ),
        "studio.issue.shell_handoff_manifest_count_mismatch",
    );
    push_check(
        checks,
        "studio.check.shell_handoff_manifest.ready_count",
        manifest.ready_count == ready_count,
        "ready count matches handoff statuses",
        &format!(
            "ready count {} does not match {ready_count} ready handoffs",
            manifest.ready_count
        ),
        "studio.issue.shell_handoff_manifest_count_mismatch",
    );
    push_check(
        checks,
        "studio.check.shell_handoff_manifest.failed_count",
        manifest.failed_count == failed_count,
        "failed count matches handoff statuses",
        &format!(
            "failed count {} does not match {failed_count} failed handoffs",
            manifest.failed_count
        ),
        "studio.issue.shell_handoff_manifest_count_mismatch",
    );
    push_check(
        checks,
        "studio.check.shell_handoff_manifest.missing_bundle_count",
        manifest.missing_bundle_count == missing_bundle_count,
        "missing-bundle count matches handoff issue codes",
        &format!(
            "missing-bundle count {} does not match {missing_bundle_count} missing handoffs",
            manifest.missing_bundle_count
        ),
        "studio.issue.shell_handoff_manifest_count_mismatch",
    );
    let expected_status = shell_handoff_manifest_expected_status(&manifest.handoffs);
    push_check(
        checks,
        "studio.check.shell_handoff_manifest.status",
        manifest.status == expected_status,
        "manifest status matches handoff statuses",
        "manifest status does not match handoff statuses",
        "studio.issue.shell_handoff_manifest_status_mismatch",
    );
}

fn validate_shell_handoff_manifest_target_coverage(
    manifest: &StudioShellHandoffManifest,
    checks: &mut Vec<StudioValidationCheck>,
) {
    for target_kind in shell_target_kinds() {
        let handoff_count = manifest
            .handoffs
            .iter()
            .filter(|handoff| handoff.target_kind == target_kind)
            .count();
        let target_count = manifest
            .targets
            .iter()
            .filter(|target| target.target_kind == target_kind)
            .count();
        let label = shell_target_kind_label(target_kind);
        push_check(
            checks,
            &format!("studio.check.shell_handoff_manifest.target.{label}.unique_summary"),
            target_count <= 1,
            "target summary is unique",
            "target summary appears more than once",
            "studio.issue.duplicate_target_summary",
        );
        push_check(
            checks,
            &format!("studio.check.shell_handoff_manifest.target.{label}.summary_present"),
            handoff_count == 0 || target_count == 1,
            "target summary is present when handoffs target it",
            "target summary is missing for one or more handoffs",
            "studio.issue.missing_target_summary",
        );
    }
}

fn validate_shell_handoff_manifest_target(
    target: &StudioShellHandoffManifestTarget,
    handoffs: &[StudioShellHandoffManifestEntry],
    checks: &mut Vec<StudioValidationCheck>,
) {
    let target_handoffs = handoffs
        .iter()
        .filter(|handoff| handoff.target_kind == target.target_kind)
        .collect::<Vec<_>>();
    let label = shell_target_kind_label(target.target_kind);
    let ready_count = target_handoffs
        .iter()
        .filter(|handoff| handoff.status == StudioValidationStatus::Pass)
        .count();
    let failed_count = target_handoffs
        .iter()
        .filter(|handoff| handoff.status == StudioValidationStatus::Fail)
        .count();
    let missing_bundle_count = target_handoffs
        .iter()
        .filter(|handoff| {
            handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
        })
        .count();
    let package_count = target_handoffs
        .iter()
        .map(|handoff| handoff.package_ids.len())
        .sum::<usize>();
    let module_count = target_handoffs
        .iter()
        .map(|handoff| handoff.module_ids.len())
        .sum::<usize>();
    let operator_shell_count = target_handoffs
        .iter()
        .map(|handoff| handoff.operator_shell_ids.len())
        .sum::<usize>();
    let graph_ids = unique_strings(
        target_handoffs
            .iter()
            .map(|handoff| handoff.graph_id.clone()),
    );
    let consumer_ids = unique_strings(
        target_handoffs
            .iter()
            .map(|handoff| handoff.consumer_id.clone()),
    );
    let issue_codes = unique_strings(
        target_handoffs
            .iter()
            .filter_map(|handoff| handoff.issue_code.clone()),
    );
    let bundle_dirs = unique_strings(
        target_handoffs
            .iter()
            .map(|handoff| handoff.bundle_dir.clone()),
    );
    let ready_bundle_dirs = unique_strings(
        target_handoffs
            .iter()
            .filter(|handoff| handoff.status == StudioValidationStatus::Pass)
            .map(|handoff| handoff.bundle_dir.clone()),
    );
    let failed_bundle_dirs = unique_strings(
        target_handoffs
            .iter()
            .filter(|handoff| handoff.status == StudioValidationStatus::Fail)
            .map(|handoff| handoff.bundle_dir.clone()),
    );
    let missing_bundle_dirs = unique_strings(
        target_handoffs
            .iter()
            .filter(|handoff| {
                handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            })
            .map(|handoff| handoff.bundle_dir.clone()),
    );
    let template_index_paths = unique_strings(
        target_handoffs
            .iter()
            .map(|handoff| handoff.template_index_path.clone()),
    );

    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.graph_count"),
        target.graph_count == target_handoffs.len(),
        "target graph count matches handoffs",
        "target graph count does not match handoffs",
        "studio.issue.target_summary_count_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.ready_count"),
        target.ready_count == ready_count,
        "target ready count matches handoffs",
        "target ready count does not match handoffs",
        "studio.issue.target_summary_count_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.failed_count"),
        target.failed_count == failed_count,
        "target failed count matches handoffs",
        "target failed count does not match handoffs",
        "studio.issue.target_summary_count_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.missing_bundle_count"),
        target.missing_bundle_count == missing_bundle_count,
        "target missing-bundle count matches handoffs",
        "target missing-bundle count does not match handoffs",
        "studio.issue.target_summary_count_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.coverage_counts"),
        target.package_count == package_count
            && target.module_count == module_count
            && target.operator_shell_count == operator_shell_count,
        "target package/module/operator-shell counts match handoffs",
        "target package/module/operator-shell counts do not match handoffs",
        "studio.issue.target_summary_count_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.graph_ids"),
        same_unique_strings(&target.graph_ids, &graph_ids) && all_dotted_ids(&target.graph_ids),
        "target graph ids match handoffs and use dotted-id grammar",
        "target graph ids do not match handoffs or contain an invalid id",
        "studio.issue.target_summary_graph_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.consumer_ids"),
        same_unique_strings(&target.consumer_ids, &consumer_ids)
            && all_dotted_ids(&target.consumer_ids),
        "target consumer ids match handoffs and use dotted-id grammar",
        "target consumer ids do not match handoffs or contain an invalid id",
        "studio.issue.target_summary_consumer_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.issue_codes"),
        same_unique_strings(&target.issue_codes, &issue_codes)
            && all_dotted_ids(&target.issue_codes),
        "target issue codes match handoffs and use dotted-id grammar",
        "target issue codes do not match handoffs or contain an invalid id",
        "studio.issue.target_summary_issue_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.bundle_dirs"),
        same_unique_strings(&target.bundle_dirs, &bundle_dirs)
            && target
                .bundle_dirs
                .iter()
                .all(|path| !path.trim().is_empty()),
        "target bundle dirs match handoffs",
        "target bundle dirs do not match handoffs",
        "studio.issue.target_summary_path_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.ready_bundle_dirs"),
        same_unique_strings(&target.ready_bundle_dirs, &ready_bundle_dirs),
        "target ready bundle dirs match handoffs",
        "target ready bundle dirs do not match handoffs",
        "studio.issue.target_summary_path_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.failed_bundle_dirs"),
        same_unique_strings(&target.failed_bundle_dirs, &failed_bundle_dirs),
        "target failed bundle dirs match handoffs",
        "target failed bundle dirs do not match handoffs",
        "studio.issue.target_summary_path_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.missing_bundle_dirs"),
        same_unique_strings(&target.missing_bundle_dirs, &missing_bundle_dirs),
        "target missing bundle dirs match handoffs",
        "target missing bundle dirs do not match handoffs",
        "studio.issue.target_summary_path_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.target.{label}.template_index_paths"),
        same_unique_strings(&target.template_index_paths, &template_index_paths)
            && target
                .template_index_paths
                .iter()
                .all(|path| path_ends_with_shell_templates(path)),
        "target template-index paths match handoffs",
        "target template-index paths do not match handoffs",
        "studio.issue.target_summary_path_mismatch",
    );
}

fn validate_shell_handoff_manifest_entry(
    handoff: &StudioShellHandoffManifestEntry,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = if handoff.graph_id.is_empty() {
        "unknown".to_string()
    } else {
        handoff.graph_id.clone()
    };
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.export_bundle_id"),
        is_dotted_id(&handoff.export_bundle_id)
            && handoff.export_bundle_id == format!("studio.export.{}", handoff.graph_id),
        "handoff export bundle id matches graph id",
        "handoff export bundle id is invalid or does not match graph id",
        "studio.issue.handoff_export_bundle_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.graph_id"),
        is_dotted_id(&handoff.graph_id),
        "handoff graph id uses dotted-id grammar",
        "handoff graph id is not a dotted id",
        "studio.issue.invalid_graph_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.target_host_profile"),
        is_dotted_id(&handoff.target_host_profile),
        "handoff target host profile uses dotted-id grammar",
        "handoff target host profile is not a dotted id",
        "studio.issue.invalid_target_host_profile",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.consumer"),
        handoff.consumer_id == shell_handoff_consumer_id(handoff.target_kind)
            && is_dotted_id(&handoff.consumer_id),
        "handoff consumer matches target kind",
        "handoff consumer does not match target kind",
        "studio.issue.handoff_consumer_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.kind"),
        handoff.handoff_kind == shell_handoff_kind_for_target(handoff.target_kind),
        "handoff kind matches target kind",
        "handoff kind does not match target kind",
        "studio.issue.handoff_kind_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.ids"),
        all_dotted_ids(&handoff.package_ids)
            && all_dotted_ids(&handoff.module_ids)
            && all_dotted_ids(&handoff.operator_shell_ids),
        "handoff package, module, and operator-shell ids use dotted-id grammar",
        "one or more handoff package, module, or operator-shell ids are invalid",
        "studio.issue.handoff_id_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.bundle_dir"),
        !handoff.bundle_dir.trim().is_empty(),
        "handoff bundle dir is present",
        "handoff bundle dir must be present",
        "studio.issue.handoff_path_missing",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.template_index_path"),
        path_ends_with_shell_templates(&handoff.template_index_path),
        "handoff template-index path points to shell-templates.json",
        "handoff template-index path must point to shell-templates.json",
        "studio.issue.handoff_template_index_path_mismatch",
    );
    let pass_status_consistent = handoff.status != StudioValidationStatus::Pass
        || (handoff.issue_code.is_none()
            && handoff.validation_status == StudioValidationStatus::Pass
            && handoff.failed_check_count == 0);
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.pass_status"),
        pass_status_consistent,
        "passing handoff has no issue and no failed checks",
        "passing handoff carries an issue, failed validation, or failed checks",
        "studio.issue.handoff_status_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.fail_status"),
        handoff.status != StudioValidationStatus::Fail || handoff.issue_code.is_some(),
        "failing handoff carries an issue code",
        "failing handoff must carry an issue code",
        "studio.issue.handoff_status_mismatch",
    );
    let pass_args_consistent = handoff.status != StudioValidationStatus::Pass
        || (handoff.consumer_args.iter().any(|arg| arg == "--templates")
            && handoff
                .consumer_args
                .iter()
                .any(|arg| arg == &handoff.template_index_path));
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.consumer_args"),
        pass_args_consistent,
        "passing handoff carries template consumer args",
        "passing handoff must carry --templates and the template-index path",
        "studio.issue.handoff_consumer_args_mismatch",
    );
    let runtime_authority_consistent = match handoff.runtime_authority.as_ref() {
        Some(authority) => runtime_authority_matches(authority),
        None => handoff.status == StudioValidationStatus::Fail,
    };
    push_check(
        checks,
        &format!("studio.check.shell_handoff_manifest.handoff.{prefix}.runtime_authority"),
        runtime_authority_consistent,
        "handoff runtime authority preserves Manifold/Hostess/Studio boundaries",
        "handoff runtime authority is missing or does not preserve boundaries",
        "studio.issue.runtime_authority_mismatch",
    );
}

fn validate_shell_handoff_manifest_authority(
    check_id: &str,
    authority: &StudioShellRuntimeAuthority,
    checks: &mut Vec<StudioValidationCheck>,
) {
    push_check(
        checks,
        check_id,
        runtime_authority_matches(authority),
        "runtime authority preserves Manifold/Hostess/Studio boundaries",
        "runtime authority does not preserve Manifold/Hostess/Studio boundaries",
        "studio.issue.runtime_authority_mismatch",
    );
}

fn shell_handoff_manifest_expected_status(
    handoffs: &[StudioShellHandoffManifestEntry],
) -> StudioValidationStatus {
    if handoffs.is_empty()
        || handoffs
            .iter()
            .any(|handoff| handoff.status == StudioValidationStatus::Fail)
    {
        StudioValidationStatus::Fail
    } else {
        StudioValidationStatus::Pass
    }
}

fn runtime_authority_matches(authority: &StudioShellRuntimeAuthority) -> bool {
    authority.command_session_authority == "rusty.manifold"
        && authority.install_launch_evidence_authority == "rusty.hostess"
        && authority.studio_role == "authoring.export_planning"
}

fn path_ends_with_shell_templates(path: &str) -> bool {
    !path.trim().is_empty() && path.replace('\\', "/").ends_with("/shell-templates.json")
}

fn same_unique_strings(actual: &[String], expected: &[String]) -> bool {
    actual.len() == expected.len()
        && actual.iter().collect::<BTreeSet<_>>() == expected.iter().collect::<BTreeSet<_>>()
}

fn unique_strings<I>(values: I) -> Vec<String>
where
    I: IntoIterator<Item = String>,
{
    let mut seen = BTreeSet::new();
    let mut unique = Vec::new();
    for value in values {
        if seen.insert(value.clone()) {
            unique.push(value);
        }
    }
    unique
}

fn shell_target_kinds() -> [StudioShellTargetKind; 4] {
    [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
        StudioShellTargetKind::Unknown,
    ]
}

fn shell_handoff_kind_for_target(target_kind: StudioShellTargetKind) -> StudioShellHandoffKind {
    match target_kind {
        StudioShellTargetKind::Desktop => StudioShellHandoffKind::DesktopShell,
        StudioShellTargetKind::Phone => StudioShellHandoffKind::PhoneShell,
        StudioShellTargetKind::Quest => StudioShellHandoffKind::QuestShell,
        StudioShellTargetKind::Unknown => StudioShellHandoffKind::UnknownShell,
    }
}

fn shell_handoff_consumer_id(target_kind: StudioShellTargetKind) -> &'static str {
    match target_kind {
        StudioShellTargetKind::Desktop => "rusty-studio-desktop-shell",
        StudioShellTargetKind::Phone => "rusty-studio-phone-shell",
        StudioShellTargetKind::Quest => "rusty-studio-quest-shell",
        StudioShellTargetKind::Unknown => "rusty-studio-operator-shell",
    }
}

fn shell_target_kind_label(target_kind: StudioShellTargetKind) -> &'static str {
    match target_kind {
        StudioShellTargetKind::Desktop => "desktop",
        StudioShellTargetKind::Phone => "phone",
        StudioShellTargetKind::Quest => "quest",
        StudioShellTargetKind::Unknown => "unknown",
    }
}

fn push_bundle_check(
    checks: &mut Vec<StudioValidationCheck>,
    graph_id: &str,
    check_id: &str,
    passed: bool,
    pass_evidence: &str,
    fail_evidence: &str,
    issue_code: &str,
) {
    push_contextual_check(
        checks,
        check_id,
        passed,
        pass_evidence,
        fail_evidence,
        issue_code,
        Some(graph_id),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
}

fn selected_shell_bundle_manifest_id(project_id: &str, graph_id: &str) -> String {
    format!("studio.shell_artifacts.{project_id}.{graph_id}")
}

fn selected_shell_bundle_template_index_id(project_id: &str, graph_id: &str) -> String {
    format!("studio.shell_templates.{project_id}.{graph_id}")
}

fn selected_shell_bundle_files(
    artifact: &StudioShellArtifact,
    template_entry: &StudioShellTemplateIndexEntry,
) -> Vec<String> {
    let mut files = BTreeSet::new();
    files.insert(artifact.descriptor_path.clone());
    files.insert(shell_manifold_handoff_artifact_path(&artifact.graph_id));
    files.insert(template_entry.descriptor_path.clone());
    files.insert(template_entry.template_path.clone());
    files.insert("shell-artifacts.json".to_string());
    files.insert("shell-templates.json".to_string());
    files.into_iter().collect()
}

fn descriptor_bundle_paths(report: &StudioShellBundleReport) -> Vec<String> {
    let mut paths = BTreeSet::new();
    if let Some(manifest) = report.artifact_manifest.as_ref() {
        for artifact in &manifest.artifacts {
            paths.insert(artifact.descriptor_path.clone());
        }
    }
    if let Some(index) = report.template_index.as_ref() {
        for entry in &index.templates {
            paths.insert(entry.descriptor_path.clone());
        }
    }
    paths.into_iter().collect()
}

fn relative_output_path(output_dir: &Path, relative_path: &str) -> PathBuf {
    relative_path
        .split('/')
        .fold(output_dir.to_path_buf(), |path, segment| path.join(segment))
}

fn duplicate_artifact_field<F>(artifacts: &[StudioShellArtifact], field: F) -> Vec<String>
where
    F: Fn(&StudioShellArtifact) -> &str,
{
    let mut counts = BTreeMap::new();
    for artifact in artifacts {
        *counts.entry(field(artifact).to_string()).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .filter_map(|(id, count)| (count > 1).then_some(id))
        .collect()
}

fn duplicate_template_field<F>(entries: &[StudioShellTemplateIndexEntry], field: F) -> Vec<String>
where
    F: Fn(&StudioShellTemplateIndexEntry) -> &str,
{
    let mut counts = BTreeMap::new();
    for entry in entries {
        *counts.entry(field(entry).to_string()).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .filter_map(|(id, count)| (count > 1).then_some(id))
        .collect()
}

fn is_safe_relative_manifest_path(value: &str) -> bool {
    if value.is_empty() || value.contains('\\') {
        return false;
    }
    let path = Path::new(value);
    if path.is_absolute() {
        return false;
    }
    path.components()
        .all(|component| matches!(component, std::path::Component::Normal(_)))
}

fn resolve_manifest_relative_path(base_dir: &Path, relative_path: &str) -> PathBuf {
    relative_path
        .split('/')
        .fold(base_dir.to_path_buf(), |path, segment| path.join(segment))
}

fn validate_graph(
    graph: &StudioGraph,
    reference_index: Option<&ReferenceIndex>,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = graph.graph_id.clone();
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.id"),
        is_dotted_id(&graph.graph_id),
        "graph id uses dotted-id grammar",
        "graph id is not a dotted id",
        "studio.issue.invalid_graph_id",
        Some(&prefix),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.nodes_present"),
        !graph.nodes.is_empty(),
        "graph contains nodes",
        "graph must contain nodes",
        "studio.issue.no_nodes",
        Some(&prefix),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );

    let mut node_ids = BTreeSet::new();
    let mut duplicate_nodes = Vec::new();
    let mut host_profile_refs = BTreeSet::new();
    for node in &graph.nodes {
        if !node_ids.insert(node.node_id.clone()) {
            duplicate_nodes.push(node.node_id.clone());
        }
        if node.kind == StudioNodeKind::HostProfile {
            host_profile_refs.insert(node.reference_id.clone());
        }
        push_contextual_check(
            checks,
            &format!("studio.check.graph.{prefix}.node.{}.id", node.node_id),
            is_dotted_id(&node.node_id),
            "node id uses dotted-id grammar",
            "node id is not a dotted id",
            "studio.issue.invalid_node_id",
            Some(&prefix),
            vec![node.node_id.clone()],
            Vec::new(),
            Vec::new(),
        );
        push_contextual_check(
            checks,
            &format!(
                "studio.check.graph.{prefix}.node.{}.reference",
                node.node_id
            ),
            is_dotted_id(&node.reference_id),
            "node reference id uses dotted-id grammar",
            "node reference id is not a dotted id",
            "studio.issue.invalid_reference_id",
            Some(&prefix),
            vec![node.node_id.clone()],
            Vec::new(),
            vec![node.reference_id.clone()],
        );
    }
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.unique_nodes"),
        duplicate_nodes.is_empty(),
        "node ids are unique",
        &format!("duplicate node ids: {}", duplicate_nodes.join(", ")),
        "studio.issue.duplicate_node_id",
        Some(&prefix),
        duplicate_nodes.clone(),
        Vec::new(),
        Vec::new(),
    );
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.target_host"),
        host_profile_refs.contains(&graph.target_host_profile),
        "target host profile resolves to a host_profile node",
        "target host profile does not resolve to a host_profile node",
        "studio.issue.missing_target_host_profile",
        Some(&prefix),
        Vec::new(),
        Vec::new(),
        vec![graph.target_host_profile.clone()],
    );

    let edge_by_id = edge_duplicates(&graph.edges);
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.unique_edges"),
        edge_by_id.is_empty(),
        "edge ids are unique",
        &format!(
            "duplicate edge ids: {}",
            edge_by_id.keys().cloned().collect::<Vec<_>>().join(", ")
        ),
        "studio.issue.duplicate_edge_id",
        Some(&prefix),
        Vec::new(),
        edge_by_id.keys().cloned().collect::<Vec<_>>(),
        Vec::new(),
    );
    let edge_ids = graph
        .edges
        .iter()
        .map(|edge| edge.edge_id.clone())
        .collect::<BTreeSet<_>>();
    for edge in &graph.edges {
        validate_edge(graph, edge, &node_ids, checks);
    }
    validate_graph_layout(graph, &node_ids, &edge_ids, checks);
    if let Some(reference_index) = reference_index {
        validate_graph_references(graph, reference_index, checks);
    }
}

fn validate_graph_layout(
    graph: &StudioGraph,
    node_ids: &BTreeSet<String>,
    edge_ids: &BTreeSet<String>,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let Some(layout) = graph.layout.as_ref() else {
        return;
    };
    let prefix = &graph.graph_id;
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.layout.id"),
        is_dotted_id(&layout.layout_id),
        "graph layout id uses dotted-id grammar",
        "graph layout id is not a dotted id",
        "studio.issue.invalid_layout_id",
        Some(prefix),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.layout.coordinate_space"),
        is_dotted_id(&layout.coordinate_space),
        "graph layout coordinate space uses dotted-id grammar",
        "graph layout coordinate space is not a dotted id",
        "studio.issue.invalid_layout_coordinate_space",
        Some(prefix),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );

    let mut layout_node_ids = BTreeSet::new();
    let mut duplicate_layout_nodes = Vec::new();
    for node in &layout.nodes {
        if !layout_node_ids.insert(node.node_id.clone()) {
            duplicate_layout_nodes.push(node.node_id.clone());
        }
        push_contextual_check(
            checks,
            &format!(
                "studio.check.graph.{prefix}.layout.node.{}.exists",
                node.node_id
            ),
            node_ids.contains(&node.node_id),
            "layout node references a graph node",
            "layout node references a missing graph node",
            "studio.issue.layout_node_missing",
            Some(prefix),
            vec![node.node_id.clone()],
            Vec::new(),
            Vec::new(),
        );
        push_contextual_check(
            checks,
            &format!(
                "studio.check.graph.{prefix}.layout.node.{}.box",
                node.node_id
            ),
            node.width > 0 && node.height > 0,
            "layout node box has positive dimensions",
            "layout node box must have positive dimensions",
            "studio.issue.invalid_layout_node_box",
            Some(prefix),
            vec![node.node_id.clone()],
            Vec::new(),
            Vec::new(),
        );
    }
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.layout.unique_nodes"),
        duplicate_layout_nodes.is_empty(),
        "layout node ids are unique",
        &format!(
            "duplicate layout node ids: {}",
            duplicate_layout_nodes.join(", ")
        ),
        "studio.issue.duplicate_layout_node_id",
        Some(prefix),
        duplicate_layout_nodes,
        Vec::new(),
        Vec::new(),
    );

    let mut layout_edge_ids = BTreeSet::new();
    let mut duplicate_layout_edges = Vec::new();
    for edge in &layout.edges {
        if !layout_edge_ids.insert(edge.edge_id.clone()) {
            duplicate_layout_edges.push(edge.edge_id.clone());
        }
        push_contextual_check(
            checks,
            &format!(
                "studio.check.graph.{prefix}.layout.edge.{}.exists",
                edge.edge_id
            ),
            edge_ids.contains(&edge.edge_id),
            "layout edge references a graph edge",
            "layout edge references a missing graph edge",
            "studio.issue.layout_edge_missing",
            Some(prefix),
            Vec::new(),
            vec![edge.edge_id.clone()],
            Vec::new(),
        );
    }
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.layout.unique_edges"),
        duplicate_layout_edges.is_empty(),
        "layout edge ids are unique",
        &format!(
            "duplicate layout edge ids: {}",
            duplicate_layout_edges.join(", ")
        ),
        "studio.issue.duplicate_layout_edge_id",
        Some(prefix),
        Vec::new(),
        duplicate_layout_edges,
        Vec::new(),
    );
}

fn validate_project_references(
    project: &StudioProject,
    base_dir: Option<&Path>,
    checks: &mut Vec<StudioValidationCheck>,
) -> Option<ReferenceIndex> {
    let Some(base_dir) = base_dir else {
        return None;
    };
    let mut references = ReferenceIndex::default();
    let catalog_path = resolve_project_path(base_dir, &project.package_catalog_path);
    push_check(
        checks,
        "studio.check.package_catalog_path",
        catalog_path.is_file(),
        "package catalog path resolves to a file",
        "package catalog path does not resolve to a file",
        "studio.issue.package_catalog_missing",
    );
    if let Some(catalog) = read_json_value(
        &catalog_path,
        checks,
        "studio.check.package_catalog.parse",
        "studio.issue.package_catalog_parse",
    ) {
        collect_catalog_references(&catalog_path, &catalog, &mut references, checks);
    }
    push_check(
        checks,
        "studio.check.host_run_profile_paths_present",
        !project.host_run_profile_paths.is_empty(),
        "project declares host-run profile paths",
        "project must declare at least one host-run profile path",
        "studio.issue.no_host_run_profiles",
    );
    for (index, path) in project.host_run_profile_paths.iter().enumerate() {
        let profile_path = resolve_project_path(base_dir, path);
        push_check(
            checks,
            &format!("studio.check.host_run_profile_path.{index}"),
            profile_path.is_file(),
            "host-run profile path resolves to a file",
            "host-run profile path does not resolve to a file",
            "studio.issue.host_run_profile_missing",
        );
        if let Some(profile) = read_json_value(
            &profile_path,
            checks,
            &format!("studio.check.host_run_profile_path.{index}.parse"),
            "studio.issue.host_run_profile_parse",
        ) {
            collect_host_profile_reference(index, &profile, &mut references, checks);
        }
    }
    Some(references)
}

fn collect_catalog_references(
    catalog_path: &Path,
    catalog: &Value,
    references: &mut ReferenceIndex,
    checks: &mut Vec<StudioValidationCheck>,
) {
    push_check(
        checks,
        "studio.check.package_catalog.schema",
        string_field(catalog, "$schema") == Some("rusty.manifold.package.catalog.v1"),
        "package catalog schema id is supported",
        "package catalog schema id is unsupported",
        "studio.issue.package_catalog_schema",
    );
    let packages = catalog.get("packages").and_then(Value::as_array);
    push_check(
        checks,
        "studio.check.package_catalog.packages_present",
        packages.is_some_and(|items| !items.is_empty()),
        "package catalog declares packages",
        "package catalog must declare at least one package",
        "studio.issue.package_catalog_empty",
    );
    let Some(packages) = packages else {
        return;
    };
    for (index, package) in packages.iter().enumerate() {
        let package_id = string_field(package, "package_id").unwrap_or_default();
        let manifest_path = string_field(package, "manifest_path").unwrap_or_default();
        let package_key = if package_id.is_empty() {
            format!("index.{index}")
        } else {
            package_id.to_string()
        };
        push_check(
            checks,
            &format!("studio.check.package_catalog.package.{package_key}.id"),
            is_dotted_id(package_id),
            "catalog package id uses dotted-id grammar",
            "catalog package id is missing or invalid",
            "studio.issue.package_catalog_package_id",
        );
        if is_dotted_id(package_id) {
            references.package_ids.insert(package_id.to_string());
            references
                .package_manifest_paths
                .insert(package_id.to_string(), manifest_path.to_string());
        }
        let manifest = resolve_catalog_manifest_path(catalog_path, manifest_path);
        push_check(
            checks,
            &format!("studio.check.package_catalog.package.{package_key}.manifest_path"),
            manifest.is_file(),
            "catalog package manifest path resolves to a file",
            "catalog package manifest path does not resolve to a file",
            "studio.issue.package_manifest_missing",
        );
        if let Some(manifest_json) = read_json_value(
            &manifest,
            checks,
            &format!("studio.check.package_manifest.{package_key}.parse"),
            "studio.issue.package_manifest_parse",
        ) {
            collect_package_manifest_references(package_id, &manifest_json, references, checks);
        }
    }
}

fn collect_package_manifest_references(
    expected_package_id: &str,
    manifest: &Value,
    references: &mut ReferenceIndex,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let package_key = if expected_package_id.is_empty() {
        "unknown"
    } else {
        expected_package_id
    };
    push_check(
        checks,
        &format!("studio.check.package_manifest.{package_key}.schema"),
        string_field(manifest, "$schema") == Some("rusty.manifold.package.manifest.v1"),
        "package manifest schema id is supported",
        "package manifest schema id is unsupported",
        "studio.issue.package_manifest_schema",
    );
    push_check(
        checks,
        &format!("studio.check.package_manifest.{package_key}.id_matches_catalog"),
        string_field(manifest, "package_id") == Some(expected_package_id),
        "package manifest id matches catalog package id",
        "package manifest id does not match catalog package id",
        "studio.issue.package_manifest_id_mismatch",
    );
    let modules = manifest
        .get("exports")
        .and_then(|exports| exports.get("modules"))
        .and_then(Value::as_array);
    push_check(
        checks,
        &format!("studio.check.package_manifest.{package_key}.module_exports"),
        modules.is_some_and(|items| !items.is_empty()),
        "package manifest exports modules",
        "package manifest must export at least one module",
        "studio.issue.package_manifest_no_modules",
    );
    let mut exported_modules = BTreeSet::new();
    if let Some(modules) = modules {
        for module in modules {
            if let Some(module_id) = module.as_str().filter(|value| is_dotted_id(value)) {
                references.module_ids.insert(module_id.to_string());
                exported_modules.insert(module_id.to_string());
            }
        }
    }
    if is_dotted_id(expected_package_id) {
        references
            .package_modules
            .insert(expected_package_id.to_string(), exported_modules);
    }
}

fn collect_host_profile_reference(
    index: usize,
    profile: &Value,
    references: &mut ReferenceIndex,
    checks: &mut Vec<StudioValidationCheck>,
) {
    push_check(
        checks,
        &format!("studio.check.host_run_profile_path.{index}.schema"),
        string_field(profile, "$schema")
            == Some("rusty.manifold.host_run.install_launch_profile.v1"),
        "host-run profile schema id is supported",
        "host-run profile schema id is unsupported",
        "studio.issue.host_run_profile_schema",
    );
    let profile_id = string_field(profile, "profile_id").unwrap_or_default();
    push_check(
        checks,
        &format!("studio.check.host_run_profile_path.{index}.profile_id"),
        is_dotted_id(profile_id),
        "host-run profile id uses dotted-id grammar",
        "host-run profile id is missing or invalid",
        "studio.issue.host_run_profile_id",
    );
    if is_dotted_id(profile_id) {
        references.host_profiles.insert(
            profile_id.to_string(),
            HostProfileReference {
                profile_id: profile_id.to_string(),
                host_profile: string_field(profile, "host_profile").map(str::to_string),
                app_id: string_field(profile, "app_id").map(str::to_string),
                install_route: string_field(profile, "install_route").map(str::to_string),
                launch_route: string_field(profile, "launch_route").map(str::to_string),
                command_bridge: string_field(profile, "command_bridge").map(str::to_string),
                evidence_pull_route: string_field(profile, "evidence_pull_route")
                    .map(str::to_string),
                required_permissions: string_array_field(profile, "required_permissions"),
            },
        );
    }
}

fn validate_graph_references(
    graph: &StudioGraph,
    reference_index: &ReferenceIndex,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let missing_package_targets = graph
        .nodes
        .iter()
        .filter(|node| node.kind == StudioNodeKind::Package)
        .filter(|node| !reference_index.package_ids.contains(&node.reference_id))
        .map(|node| (node.node_id.clone(), node.reference_id.clone()))
        .collect::<Vec<_>>();
    let missing_package_nodes = missing_package_targets
        .iter()
        .map(|(node_id, _)| node_id.clone())
        .collect::<Vec<_>>();
    let missing_packages = missing_package_targets
        .iter()
        .map(|(_, reference_id)| reference_id.clone())
        .collect::<Vec<_>>();
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{}.package_refs", graph.graph_id),
        missing_packages.is_empty(),
        "graph package nodes resolve through package catalog",
        &format!(
            "package references missing from catalog: {}",
            missing_packages.join(", ")
        ),
        "studio.issue.package_reference_missing",
        Some(&graph.graph_id),
        missing_package_nodes,
        Vec::new(),
        missing_packages.clone(),
    );

    let missing_module_targets = graph
        .nodes
        .iter()
        .filter(|node| node.kind == StudioNodeKind::Module)
        .filter(|node| !reference_index.module_ids.contains(&node.reference_id))
        .map(|node| (node.node_id.clone(), node.reference_id.clone()))
        .collect::<Vec<_>>();
    let missing_module_nodes = missing_module_targets
        .iter()
        .map(|(node_id, _)| node_id.clone())
        .collect::<Vec<_>>();
    let missing_modules = missing_module_targets
        .iter()
        .map(|(_, reference_id)| reference_id.clone())
        .collect::<Vec<_>>();
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{}.module_refs", graph.graph_id),
        missing_modules.is_empty(),
        "graph module nodes resolve through selected package manifests",
        &format!(
            "module references missing from package manifests: {}",
            missing_modules.join(", ")
        ),
        "studio.issue.module_reference_missing",
        Some(&graph.graph_id),
        missing_module_nodes,
        Vec::new(),
        missing_modules.clone(),
    );

    let missing_host_profile_targets = graph
        .nodes
        .iter()
        .filter(|node| node.kind == StudioNodeKind::HostProfile)
        .filter(|node| {
            !reference_index
                .host_profiles
                .contains_key(&node.reference_id)
        })
        .map(|node| (node.node_id.clone(), node.reference_id.clone()))
        .collect::<Vec<_>>();
    let missing_host_profile_nodes = missing_host_profile_targets
        .iter()
        .map(|(node_id, _)| node_id.clone())
        .collect::<Vec<_>>();
    let missing_host_profiles = missing_host_profile_targets
        .iter()
        .map(|(_, reference_id)| reference_id.clone())
        .collect::<Vec<_>>();
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{}.host_profile_refs", graph.graph_id),
        missing_host_profiles.is_empty(),
        "graph host-profile nodes resolve through declared host-run profiles",
        &format!(
            "host profile references missing from declared profiles: {}",
            missing_host_profiles.join(", ")
        ),
        "studio.issue.host_profile_reference_missing",
        Some(&graph.graph_id),
        missing_host_profile_nodes,
        Vec::new(),
        missing_host_profiles.clone(),
    );
    push_contextual_check(
        checks,
        &format!(
            "studio.check.graph.{}.target_host_profile_ref",
            graph.graph_id
        ),
        reference_index
            .host_profiles
            .contains_key(&graph.target_host_profile),
        "target host profile resolves through declared host-run profiles",
        "target host profile is missing from declared host-run profiles",
        "studio.issue.target_host_profile_reference_missing",
        Some(&graph.graph_id),
        Vec::new(),
        Vec::new(),
        vec![graph.target_host_profile.clone()],
    );
}

fn read_json_value(
    path: &Path,
    checks: &mut Vec<StudioValidationCheck>,
    check_id: &str,
    issue_code: &str,
) -> Option<Value> {
    match std::fs::read_to_string(path) {
        Ok(text) => match serde_json::from_str::<Value>(&text) {
            Ok(value) => {
                push_check(
                    checks,
                    check_id,
                    true,
                    "JSON document parsed",
                    "JSON document could not parse",
                    issue_code,
                );
                Some(value)
            }
            Err(error) => {
                push_check(
                    checks,
                    check_id,
                    false,
                    "JSON document parsed",
                    &format!("JSON parse error: {error}"),
                    issue_code,
                );
                None
            }
        },
        Err(error) => {
            push_check(
                checks,
                check_id,
                false,
                "JSON document parsed",
                &format!("JSON read error: {error}"),
                issue_code,
            );
            None
        }
    }
}

fn string_field<'a>(value: &'a Value, field: &str) -> Option<&'a str> {
    value.get(field).and_then(Value::as_str)
}

fn string_array_field(value: &Value, field: &str) -> Vec<String> {
    value
        .get(field)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn resolve_project_path(base_dir: &Path, path: &str) -> PathBuf {
    let path = Path::new(path);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_dir.join(path)
    }
}

fn resolve_catalog_manifest_path(catalog_path: &Path, manifest_path: &str) -> PathBuf {
    let path = Path::new(manifest_path);
    if path.is_absolute() {
        return path.to_path_buf();
    }
    let Some(catalog_dir) = catalog_path.parent() else {
        return path.to_path_buf();
    };
    let catalog_relative = catalog_dir.join(path);
    if catalog_relative.is_file() {
        return catalog_relative;
    }
    catalog_dir
        .parent()
        .map(|repo_root| repo_root.join(path))
        .unwrap_or(catalog_relative)
}

fn validate_edge(
    graph: &StudioGraph,
    edge: &StudioEdge,
    node_ids: &BTreeSet<String>,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = &graph.graph_id;
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.edge.{}.id", edge.edge_id),
        is_dotted_id(&edge.edge_id),
        "edge id uses dotted-id grammar",
        "edge id is not a dotted id",
        "studio.issue.invalid_edge_id",
        Some(prefix),
        Vec::new(),
        vec![edge.edge_id.clone()],
        Vec::new(),
    );
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.edge.{}.source", edge.edge_id),
        node_ids.contains(&edge.source_node_id),
        "edge source node exists",
        "edge source node is missing",
        "studio.issue.missing_edge_source",
        Some(prefix),
        vec![edge.source_node_id.clone()],
        vec![edge.edge_id.clone()],
        Vec::new(),
    );
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{prefix}.edge.{}.target", edge.edge_id),
        node_ids.contains(&edge.target_node_id),
        "edge target node exists",
        "edge target node is missing",
        "studio.issue.missing_edge_target",
        Some(prefix),
        vec![edge.target_node_id.clone()],
        vec![edge.edge_id.clone()],
        Vec::new(),
    );
    if let Some(binding_kind) = binding_kind_for_edge(edge.kind) {
        push_contextual_check(
            checks,
            &format!(
                "studio.check.graph.{prefix}.edge.{}.self_binding",
                edge.edge_id
            ),
            edge.source_node_id != edge.target_node_id,
            "binding edge connects distinct nodes",
            "binding edge source and target are the same node",
            "studio.issue.self_binding",
            Some(prefix),
            vec![edge.source_node_id.clone(), edge.target_node_id.clone()],
            vec![edge.edge_id.clone()],
            Vec::new(),
        );
        let source_kind = graph
            .nodes
            .iter()
            .find(|node| node.node_id == edge.source_node_id)
            .map(|node| node.kind);
        let target_kind = graph
            .nodes
            .iter()
            .find(|node| node.node_id == edge.target_node_id)
            .map(|node| node.kind);
        if let (Some(source_kind), Some(target_kind)) = (source_kind, target_kind) {
            push_contextual_check(
                checks,
                &format!(
                    "studio.check.graph.{prefix}.edge.{}.binding_endpoint_kinds",
                    edge.edge_id
                ),
                binding_endpoint_kinds_are_valid(binding_kind, source_kind, target_kind),
                "binding endpoint node kinds match the binding type",
                binding_endpoint_kind_message(binding_kind),
                "studio.issue.binding_endpoint_kind_mismatch",
                Some(prefix),
                vec![edge.source_node_id.clone(), edge.target_node_id.clone()],
                vec![edge.edge_id.clone()],
                Vec::new(),
            );
        }
    }
}

fn resolve_graph(graph: &StudioGraph) -> StudioResolvedGraph {
    StudioResolvedGraph {
        graph_id: graph.graph_id.clone(),
        target_host_profile: graph.target_host_profile.clone(),
        package_count: graph
            .nodes
            .iter()
            .filter(|node| node.kind == StudioNodeKind::Package)
            .count(),
        module_count: graph
            .nodes
            .iter()
            .filter(|node| node.kind == StudioNodeKind::Module)
            .count(),
        operator_shell_count: graph
            .nodes
            .iter()
            .filter(|node| node.kind == StudioNodeKind::OperatorShell)
            .count(),
        node_count: graph.nodes.len(),
        edge_count: graph.edges.len(),
    }
}

#[derive(Default)]
struct ValidationIssueTargetIndex {
    graph_counts: BTreeMap<String, usize>,
    node_counts: BTreeMap<(String, String), usize>,
    edge_counts: BTreeMap<(String, String), usize>,
}

impl ValidationIssueTargetIndex {
    fn graph_issue_count(&self, graph_id: &str) -> usize {
        self.graph_counts.get(graph_id).copied().unwrap_or(0)
    }

    fn node_issue_count(&self, graph_id: &str, node_id: &str) -> usize {
        self.node_counts
            .get(&(graph_id.to_string(), node_id.to_string()))
            .copied()
            .unwrap_or(0)
    }

    fn edge_issue_count(&self, graph_id: &str, edge_id: &str) -> usize {
        self.edge_counts
            .get(&(graph_id.to_string(), edge_id.to_string()))
            .copied()
            .unwrap_or(0)
    }
}

fn validation_issue_target_index(report: &StudioValidationReport) -> ValidationIssueTargetIndex {
    let mut index = ValidationIssueTargetIndex::default();
    for check in report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
    {
        let Some(graph_id) = check.graph_id.as_deref() else {
            continue;
        };
        *index.graph_counts.entry(graph_id.to_string()).or_insert(0) += 1;
        for node_id in &check.node_ids {
            *index
                .node_counts
                .entry((graph_id.to_string(), node_id.clone()))
                .or_insert(0) += 1;
        }
        for edge_id in &check.edge_ids {
            *index
                .edge_counts
                .entry((graph_id.to_string(), edge_id.clone()))
                .or_insert(0) += 1;
        }
    }
    index
}

fn graph_view(
    graph: &StudioGraph,
    issue_target_index: &ValidationIssueTargetIndex,
) -> StudioGraphView {
    let resolved = resolve_graph(graph);
    StudioGraphView {
        graph_id: graph.graph_id.clone(),
        display_name: graph.display_name.clone(),
        target_host_profile: graph.target_host_profile.clone(),
        validation_issue_count: issue_target_index.graph_issue_count(&graph.graph_id),
        node_count: resolved.node_count,
        edge_count: resolved.edge_count,
        package_count: resolved.package_count,
        module_count: resolved.module_count,
        operator_shell_count: resolved.operator_shell_count,
        node_rows: graph
            .nodes
            .iter()
            .map(|node| StudioNodeView {
                node_id: node.node_id.clone(),
                kind: node_kind_label(node.kind).to_string(),
                reference_id: node.reference_id.clone(),
                label: node.label.clone(),
                validation_issue_count: issue_target_index
                    .node_issue_count(&graph.graph_id, &node.node_id),
            })
            .collect(),
        edge_rows: graph
            .edges
            .iter()
            .map(|edge| StudioEdgeView {
                edge_id: edge.edge_id.clone(),
                kind: edge_kind_label(edge.kind).to_string(),
                source_node_id: edge.source_node_id.clone(),
                target_node_id: edge.target_node_id.clone(),
                validation_issue_count: issue_target_index
                    .edge_issue_count(&graph.graph_id, &edge.edge_id),
            })
            .collect(),
        layout: graph
            .layout
            .as_ref()
            .map(|layout| graph_layout_view(&graph.graph_id, layout, issue_target_index)),
    }
}

fn graph_layout_view(
    graph_id: &str,
    layout: &rusty_studio_model::StudioGraphLayout,
    issue_target_index: &ValidationIssueTargetIndex,
) -> StudioGraphLayoutView {
    StudioGraphLayoutView {
        layout_id: layout.layout_id.clone(),
        coordinate_space: layout.coordinate_space.clone(),
        node_count: layout.nodes.len(),
        edge_count: layout.edges.len(),
        nodes: layout
            .nodes
            .iter()
            .map(|node| StudioNodeLayoutView {
                node_id: node.node_id.clone(),
                x: node.x,
                y: node.y,
                width: node.width,
                height: node.height,
                validation_issue_count: issue_target_index
                    .node_issue_count(graph_id, &node.node_id),
            })
            .collect(),
        edges: layout
            .edges
            .iter()
            .map(|edge| StudioEdgeLayoutView {
                edge_id: edge.edge_id.clone(),
                route: edge_route_label(edge.route).to_string(),
                validation_issue_count: issue_target_index
                    .edge_issue_count(graph_id, &edge.edge_id),
            })
            .collect(),
    }
}

fn validation_issue_views(
    report: &StudioValidationReport,
    selected_graph_id: Option<&str>,
) -> Vec<StudioValidationIssueView> {
    report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .map(|check| StudioValidationIssueView {
            check_id: check.check_id.clone(),
            issue_code: check.issue_code.clone(),
            evidence: check.evidence.clone(),
            graph_id: check.graph_id.clone(),
            node_ids: check.node_ids.clone(),
            edge_ids: check.edge_ids.clone(),
            reference_ids: check.reference_ids.clone(),
            targets_selected_graph: check
                .graph_id
                .as_deref()
                .is_some_and(|graph_id| selected_graph_id == Some(graph_id)),
        })
        .collect()
}

struct FocusedIssueSelection {
    focused_issue: Option<StudioIssueFocusView>,
    selected_issue_index: Option<usize>,
    selected_issue_check_id: Option<String>,
    issue_selection_code: Option<String>,
}

fn focused_issue_selection(
    issues: &[StudioValidationIssueView],
    requested_issue_check_id: Option<&str>,
) -> FocusedIssueSelection {
    let (selected_issue_index, issue_selection_code) =
        if let Some(requested_issue_check_id) = requested_issue_check_id {
            match issues
                .iter()
                .position(|issue| issue.check_id == requested_issue_check_id)
            {
                Some(index) => (Some(index), None),
                None => (
                    fallback_issue_index(issues),
                    Some("studio.issue.validation_issue_selection_missing".to_string()),
                ),
            }
        } else {
            (fallback_issue_index(issues), None)
        };
    let selected_issue_check_id = selected_issue_index.map(|index| issues[index].check_id.clone());
    let focused_issue =
        selected_issue_index.and_then(|index| focused_issue_view(index, &issues[index]));
    FocusedIssueSelection {
        focused_issue,
        selected_issue_index,
        selected_issue_check_id,
        issue_selection_code,
    }
}

fn fallback_issue_index(issues: &[StudioValidationIssueView]) -> Option<usize> {
    issues
        .iter()
        .position(|issue| issue.targets_selected_graph)
        .or_else(|| issues.iter().position(|issue| issue.graph_id.is_some()))
        .or_else(|| (!issues.is_empty()).then_some(0))
}

fn focused_issue_view(
    issue_index: usize,
    issue: &StudioValidationIssueView,
) -> Option<StudioIssueFocusView> {
    let graph_id = issue.graph_id.clone()?;
    Some(StudioIssueFocusView {
        issue_index,
        check_id: issue.check_id.clone(),
        issue_code: issue.issue_code.clone(),
        evidence: issue.evidence.clone(),
        graph_id,
        node_id: issue.node_ids.first().cloned(),
        edge_id: issue.edge_ids.first().cloned(),
        reference_id: issue.reference_ids.first().cloned(),
    })
}

fn selected_graph_index(
    graphs: &[StudioGraphView],
    requested_graph_id: Option<&str>,
) -> Option<usize> {
    if graphs.is_empty() {
        return None;
    }
    if let Some(requested_graph_id) = requested_graph_id {
        return graphs
            .iter()
            .position(|graph| graph.graph_id == requested_graph_id);
    }
    Some(0)
}

fn node_kind_label(kind: StudioNodeKind) -> &'static str {
    match kind {
        StudioNodeKind::Package => "package",
        StudioNodeKind::Module => "module",
        StudioNodeKind::HostProfile => "host_profile",
        StudioNodeKind::ValidationSlot => "validation_slot",
        StudioNodeKind::OperatorShell => "operator_shell",
    }
}

fn edge_kind_label(kind: rusty_studio_model::StudioEdgeKind) -> &'static str {
    match kind {
        rusty_studio_model::StudioEdgeKind::PackageProvidesModule => "package_provides_module",
        rusty_studio_model::StudioEdgeKind::StreamBinding => "stream_binding",
        rusty_studio_model::StudioEdgeKind::CommandBinding => "command_binding",
        rusty_studio_model::StudioEdgeKind::ValidationSlotUsesPackage => {
            "validation_slot_uses_package"
        }
        rusty_studio_model::StudioEdgeKind::ShellTargetsHostProfile => "shell_targets_host_profile",
    }
}

fn edge_route_label(route: StudioEdgeRouteKind) -> &'static str {
    match route {
        StudioEdgeRouteKind::Direct => "direct",
        StudioEdgeRouteKind::Orthogonal => "orthogonal",
    }
}

fn edge_duplicates(edges: &[StudioEdge]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for edge in edges {
        *counts.entry(edge.edge_id.clone()).or_insert(0) += 1;
    }
    counts.retain(|_, count| *count > 1);
    counts
}

fn push_check(
    checks: &mut Vec<StudioValidationCheck>,
    check_id: &str,
    passed: bool,
    pass_evidence: &str,
    fail_evidence: &str,
    issue_code: &str,
) {
    push_contextual_check(
        checks,
        check_id,
        passed,
        pass_evidence,
        fail_evidence,
        issue_code,
        None,
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
}

fn push_contextual_check(
    checks: &mut Vec<StudioValidationCheck>,
    check_id: &str,
    passed: bool,
    pass_evidence: &str,
    fail_evidence: &str,
    issue_code: &str,
    graph_id: Option<&str>,
    node_ids: Vec<String>,
    edge_ids: Vec<String>,
    reference_ids: Vec<String>,
) {
    checks.push(StudioValidationCheck {
        check_id: check_id.to_string(),
        status: if passed {
            StudioValidationStatus::Pass
        } else {
            StudioValidationStatus::Fail
        },
        evidence: if passed { pass_evidence } else { fail_evidence }.to_string(),
        issue_code: (!passed).then(|| issue_code.to_string()),
        graph_id: graph_id.map(str::to_string),
        node_ids,
        edge_ids,
        reference_ids,
    });
}

fn retarget_host_edit_report(
    project: &StudioProject,
    original_revision: u64,
    resulting_revision: u64,
    status: StudioEditStatus,
    issue_code: Option<String>,
    message: String,
    graph_id: &str,
    requested_host_profile: &str,
    changed_fields: Vec<String>,
    validation: StudioValidationReport,
) -> StudioEditReport {
    edit_report(
        project,
        original_revision,
        resulting_revision,
        StudioEditOperation::RetargetHost,
        status,
        issue_code,
        message,
        graph_id,
        requested_host_profile,
        requested_host_profile,
        changed_fields,
        validation,
    )
}

fn edit_report(
    project: &StudioProject,
    original_revision: u64,
    resulting_revision: u64,
    operation: StudioEditOperation,
    status: StudioEditStatus,
    issue_code: Option<String>,
    message: String,
    graph_id: &str,
    requested_reference_id: &str,
    requested_host_profile: &str,
    changed_fields: Vec<String>,
    validation: StudioValidationReport,
) -> StudioEditReport {
    StudioEditReport {
        schema_id: EDIT_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        original_revision,
        resulting_revision,
        operation,
        status,
        issue_code,
        message,
        graph_id: graph_id.to_string(),
        requested_reference_id: requested_reference_id.to_string(),
        requested_host_profile: requested_host_profile.to_string(),
        changed_fields,
        validation,
    }
}

fn graph_target_host_profile(project: &StudioProject, graph_id: &str) -> String {
    project
        .graphs
        .iter()
        .find(|graph| graph.graph_id == graph_id)
        .map(|graph| graph.target_host_profile.clone())
        .unwrap_or_default()
}

fn generated_node_id(reference_id: &str) -> String {
    format!("node.{reference_id}")
}

fn generated_package_module_edge_id(
    package_reference_id: &str,
    module_reference_id: &str,
) -> String {
    format!("edge.{package_reference_id}.{module_reference_id}")
}

fn generated_binding_edge_id(
    binding_kind: StudioBindingKind,
    source_node_id: &str,
    target_node_id: &str,
) -> String {
    format!(
        "edge.{}.{}.{}",
        binding_kind_label(binding_kind),
        source_node_id,
        target_node_id
    )
}

fn edge_kind_for_binding(binding_kind: StudioBindingKind) -> StudioEdgeKind {
    match binding_kind {
        StudioBindingKind::Stream => StudioEdgeKind::StreamBinding,
        StudioBindingKind::Command => StudioEdgeKind::CommandBinding,
    }
}

fn binding_kind_for_edge(edge_kind: StudioEdgeKind) -> Option<StudioBindingKind> {
    match edge_kind {
        StudioEdgeKind::StreamBinding => Some(StudioBindingKind::Stream),
        StudioEdgeKind::CommandBinding => Some(StudioBindingKind::Command),
        _ => None,
    }
}

fn binding_kind_label(binding_kind: StudioBindingKind) -> &'static str {
    match binding_kind {
        StudioBindingKind::Stream => "stream_binding",
        StudioBindingKind::Command => "command_binding",
    }
}

fn binding_endpoint_kinds_are_valid(
    binding_kind: StudioBindingKind,
    source_kind: StudioNodeKind,
    target_kind: StudioNodeKind,
) -> bool {
    match binding_kind {
        StudioBindingKind::Stream => {
            source_kind == StudioNodeKind::Module && target_kind == StudioNodeKind::Module
        }
        StudioBindingKind::Command => {
            source_kind == StudioNodeKind::OperatorShell && target_kind == StudioNodeKind::Module
        }
    }
}

fn binding_endpoint_kind_message(binding_kind: StudioBindingKind) -> &'static str {
    match binding_kind {
        StudioBindingKind::Stream => "Stream bindings must connect module nodes",
        StudioBindingKind::Command => {
            "Command bindings must connect an operator_shell node to a module node"
        }
    }
}

fn label_for_reference(reference_id: &str) -> String {
    let leaf = reference_id.rsplit('.').next().unwrap_or(reference_id);
    leaf.split(['_', '-'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                Some(first) => {
                    let mut label = first.to_ascii_uppercase().to_string();
                    label.push_str(chars.as_str());
                    label
                }
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn first_failed_issue_code(report: &StudioValidationReport) -> Option<String> {
    report.checks.iter().find_map(|check| {
        (check.status == StudioValidationStatus::Fail)
            .then(|| check.issue_code.clone())
            .flatten()
    })
}

fn first_failed_check_issue_code(report: &StudioShellDescriptorValidationReport) -> Option<String> {
    report.checks.iter().find_map(|check| {
        (check.status == StudioValidationStatus::Fail)
            .then(|| check.issue_code.clone())
            .flatten()
    })
}

fn first_failed_shell_artifact_manifest_issue_code(
    report: &StudioShellArtifactManifestValidationReport,
) -> Option<String> {
    report.checks.iter().find_map(|check| {
        (check.status == StudioValidationStatus::Fail)
            .then(|| check.issue_code.clone())
            .flatten()
    })
}

fn first_failed_shell_template_index_issue_code(
    report: &StudioShellTemplateIndexValidationReport,
) -> Option<String> {
    report.checks.iter().find_map(|check| {
        (check.status == StudioValidationStatus::Fail)
            .then(|| check.issue_code.clone())
            .flatten()
    })
}

fn first_failed_shell_bundle_validation_issue_code(
    report: &StudioShellBundleValidationReport,
) -> Option<String> {
    report.checks.iter().find_map(|check| {
        (check.status == StudioValidationStatus::Fail)
            .then(|| check.issue_code.clone())
            .flatten()
    })
}

fn optional_dotted_id(value: Option<&str>) -> bool {
    match value {
        Some(value) => is_dotted_id(value),
        None => true,
    }
}

fn all_dotted_ids(values: &[String]) -> bool {
    values.iter().all(|value| is_dotted_id(value))
}

pub fn is_dotted_id(value: &str) -> bool {
    let mut chars = value.chars().peekable();
    let mut previous_dot = true;
    let mut saw_segment_char = false;
    while let Some(ch) = chars.next() {
        let is_segment_char =
            ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '_' || ch == '-';
        if ch == '.' {
            if previous_dot || !saw_segment_char || chars.peek().is_none() {
                return false;
            }
            previous_dot = true;
            saw_segment_char = false;
            continue;
        }
        if !is_segment_char {
            return false;
        }
        previous_dot = false;
        saw_segment_char = true;
    }
    !previous_dot && saw_segment_char
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_studio_model::{
        StudioBindingKind, StudioEdgeKind, StudioEdgeLayout, StudioEdgeRouteKind,
        StudioEditOperation, StudioEditStatus, StudioGraphLayout, StudioNode, StudioNodeKind,
        StudioNodeLayout, StudioShellArtifactStatus, StudioShellBundleStatus,
        StudioShellDescriptorStatus, StudioShellHandoffAcceptanceComparisonChange,
        StudioShellHandoffAcceptanceComparisonStatus, StudioShellHandoffAcceptanceStatus,
        StudioShellHandoffIntakeDecision, StudioShellHandoffIntakeStatus, StudioShellHandoffKind,
        StudioShellReleaseCandidateReviewStatus, StudioShellTargetKind, StudioShellTemplateStatus,
        SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA,
        SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA, SHELL_HANDOFF_ACCEPTANCE_COMPARISON_SCHEMA,
        SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA, SHELL_HANDOFF_INTAKE_REPORT_SCHEMA,
        SHELL_HANDOFF_MANIFEST_SCHEMA, SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA,
        SHELL_HANDOFF_READINESS_REPORT_SCHEMA, SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA,
        SHELL_TEMPLATE_INDEX_VALIDATION_REPORT_SCHEMA,
    };

    fn valid_project() -> StudioProject {
        StudioProject {
            schema_id: PROJECT_SCHEMA.to_string(),
            project_id: "studio.project.test".to_string(),
            revision: 1,
            display_name: "Test".to_string(),
            package_catalog_path: "packages/catalog.manifold.json".to_string(),
            host_run_profile_paths: vec![
                "fixtures/host-run/install-profile-desktop.json".to_string()
            ],
            graphs: vec![StudioGraph {
                graph_id: "studio.graph.test".to_string(),
                display_name: "Graph".to_string(),
                target_host_profile: "host_run.profile.desktop".to_string(),
                nodes: vec![
                    StudioNode {
                        node_id: "node.package.synthetic".to_string(),
                        kind: StudioNodeKind::Package,
                        reference_id: "package.synthetic".to_string(),
                        label: "Package".to_string(),
                    },
                    StudioNode {
                        node_id: "node.host.desktop".to_string(),
                        kind: StudioNodeKind::HostProfile,
                        reference_id: "host_run.profile.desktop".to_string(),
                        label: "Desktop".to_string(),
                    },
                ],
                edges: vec![StudioEdge {
                    edge_id: "edge.package_host".to_string(),
                    kind: StudioEdgeKind::ShellTargetsHostProfile,
                    source_node_id: "node.package.synthetic".to_string(),
                    target_node_id: "node.host.desktop".to_string(),
                }],
                layout: Some(StudioGraphLayout {
                    layout_id: "studio.layout.test".to_string(),
                    coordinate_space: "studio.canvas.logical_2d".to_string(),
                    nodes: vec![
                        StudioNodeLayout {
                            node_id: "node.package.synthetic".to_string(),
                            x: 40,
                            y: 40,
                            width: 180,
                            height: 72,
                        },
                        StudioNodeLayout {
                            node_id: "node.host.desktop".to_string(),
                            x: 340,
                            y: 40,
                            width: 180,
                            height: 72,
                        },
                    ],
                    edges: vec![StudioEdgeLayout {
                        edge_id: "edge.package_host".to_string(),
                        route: StudioEdgeRouteKind::Direct,
                    }],
                }),
            }],
        }
    }

    fn temp_root(name: &str) -> std::path::PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("rusty-studio-{name}-{unique}"));
        std::fs::create_dir_all(&root).expect("create temp root");
        root
    }

    fn write_fixture(path: &Path, text: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create fixture parent");
        }
        std::fs::write(path, text).expect("write fixture");
    }

    fn projected_motion_package_validation_report() -> StudioManifoldPackageValidationReport {
        let mut checks = [
            "dotted_ids",
            "exports",
            "module_links",
            "stream_links",
            "graph_links",
            "deployment_links",
            "runtime_state_links",
            "timestamp_policy",
            "provider_processor_split",
            "command_rejections",
            "scorecards",
            "projected_motion_contract",
            "projected_motion_profile_commands",
            "projected_motion_goldens",
            "projected_motion_source_adapters",
            "projected_motion_source_bindings",
            "projected_motion_adapter_normalization",
        ]
        .iter()
        .map(|suffix| StudioManifoldPackageValidationCheck {
            check_id: format!("validation.package.{PROJECTED_MOTION_BREATH_PACKAGE_ID}.{suffix}"),
            status: StudioValidationStatus::Pass,
            evidence: format!("{suffix} passed"),
        })
        .collect::<Vec<_>>();
        checks.push(StudioManifoldPackageValidationCheck {
            check_id: "validation.package.package.other.dotted_ids".to_string(),
            status: StudioValidationStatus::Pass,
            evidence: "other package passed".to_string(),
        });
        StudioManifoldPackageValidationReport {
            schema_id: MANIFOLD_PACKAGE_VALIDATION_REPORT_SCHEMA.to_string(),
            status: StudioValidationStatus::Pass,
            checks,
        }
    }

    fn ready_projected_motion_package_evidence_intake() -> StudioPackageEvidenceIntakeReport {
        package_evidence_intake_for_validation_report(
            &projected_motion_package_validation_report(),
            Some(Path::new("target/manifold-package-validation.json")),
            PROJECTED_MOTION_BREATH_PACKAGE_ID,
        )
    }

    fn projected_motion_profile_document() -> Value {
        serde_json::json!({
            "$schema": MOTION_BREATH_PROFILE_SCHEMA,
            "profile_id": "profile.projected_motion_breath.synthetic_default",
            "target_module_id": "module.breath.projected_motion",
            "input_kinds": ["pose", "vector3"],
            "projection": {
                "mode": "principal_motion_axis",
                "fallback_mode": "fixed_axis"
            }
        })
    }

    fn ready_projected_motion_authoring_review() -> StudioProjectedMotionBreathAuthoringReviewReport
    {
        projected_motion_breath_authoring_review_for_intake(
            &ready_projected_motion_package_evidence_intake(),
            Some(Path::new("target/package-evidence-intake.json")),
            &projected_motion_profile_document(),
            Some(Path::new("target/profile-synthetic.json")),
        )
    }

    fn projected_motion_source_adapter_descriptors() -> Value {
        serde_json::json!({
            "$schema": PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_DESCRIPTOR_SCHEMA,
            "descriptor_set_id": "descriptor_set.projected_motion_breath.source_adapters.synthetic",
            "package_id": PROJECTED_MOTION_BREATH_PACKAGE_ID,
            "target_module_id": "module.breath.projected_motion",
            "execution_policy": "not_executed.schema_descriptors_only",
            "runtime_execution_performed": false,
            "platform_execution_performed": false,
            "device_required": false,
            "source_adapters": [
                {
                    "adapter_id": "adapter.projected_motion_breath.object_pose_generic",
                    "source_kind": "object_pose",
                    "input_kind": "pose",
                    "output_stream_id": "stream.motion.object_pose",
                    "transport_kind": "descriptor_only",
                    "requires_platform_sdk": false,
                    "requires_device_api": false,
                    "runtime_adapter_included": false
                },
                {
                    "adapter_id": "adapter.projected_motion_breath.external_patch_stream_bridge_shape",
                    "source_kind": "external_patch_stream_bridge",
                    "input_kind": "vector3",
                    "output_stream_id": "stream.motion.vector3",
                    "transport_kind": "descriptor_only",
                    "requires_platform_sdk": false,
                    "requires_device_api": false,
                    "runtime_adapter_included": false
                }
            ]
        })
    }

    fn ready_projected_motion_source_adapter_selection(
    ) -> StudioProjectedMotionBreathSourceAdapterSelectionReviewReport {
        projected_motion_breath_source_adapter_selection_review_for_authoring(
            &ready_projected_motion_authoring_review(),
            Some(Path::new(
                "target/projected-motion-breath-authoring-review.json",
            )),
            &projected_motion_source_adapter_descriptors(),
            Some(Path::new("target/source-adapter-descriptors.json")),
            "adapter.projected_motion_breath.external_patch_stream_bridge_shape",
        )
    }

    fn projected_motion_source_binding_document() -> Value {
        serde_json::json!({
            "$schema": PROJECTED_MOTION_BREATH_SOURCE_BINDING_SCHEMA,
            "binding_id": "binding.projected_motion_breath.synthetic.external_patch_stream",
            "package_id": PROJECTED_MOTION_BREATH_PACKAGE_ID,
            "target_module_id": "module.breath.projected_motion",
            "profile_id": "profile.projected_motion_breath.synthetic_default",
            "profile_path": "fixtures/valid/profile-synthetic.json",
            "descriptor_set_path": "fixtures/valid/source-adapter-descriptors.json",
            "selected_adapter_id": "adapter.projected_motion_breath.external_patch_stream_bridge_shape",
            "selected_source_kind": "external_patch_stream_bridge",
            "selected_input_kind": "vector3",
            "selected_output_stream_id": "stream.motion.vector3",
            "source_stream_id": "stream.motion.vector3",
            "binding_policy": "descriptor_only.owner_review_required",
            "execution_policy": "not_executed.schema_binding_only",
            "runtime_execution_performed": false,
            "platform_execution_performed": false,
            "device_required": false
        })
    }

    fn projected_motion_adapter_normalization_case_document() -> Value {
        serde_json::json!({
            "$schema": PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_CASE_SCHEMA,
            "case_id": "case.projected_motion_breath.normalize.external_patch_vector",
            "package_id": PROJECTED_MOTION_BREATH_PACKAGE_ID,
            "binding_path": "fixtures/valid/source-binding-synthetic.json",
            "source_payload_kind": "external_patch_channels",
            "input": {
                "source_id": "source.synthetic.patch_stream",
                "sample_time_s": 12.0,
                "host_time_s": 12.011,
                "frame_id": "frame.synthetic.patch",
                "channel_values": {
                    "breath_x": 0.11,
                    "breath_y": 0.42,
                    "breath_z": -0.03
                },
                "channel_map": {
                    "x": "breath_x",
                    "y": "breath_y",
                    "z": "breath_z"
                },
                "units": "normalized_patch_units",
                "quality01": 0.91
            },
            "expected_sample_kind": "vector_motion",
            "expected": {
                "source_id": "source.synthetic.patch_stream",
                "sample_time_s": 12.0,
                "host_time_s": 12.011,
                "frame_id": "frame.synthetic.patch",
                "vector3": [0.11, 0.42, -0.03],
                "units": "normalized_patch_units",
                "quality01": 0.91
            },
            "execution_policy": "not_executed.fixture_normalization_only",
            "runtime_execution_performed": false,
            "platform_execution_performed": false,
            "device_required": false
        })
    }

    fn projected_motion_shell_handoff_evidence() -> Value {
        serde_json::json!({
            "$schema": "rusty.hostess.projected_motion_breath.shell_handoff_validation_evidence.v1",
            "status": "pass",
            "package": {
                "package_id": PROJECTED_MOTION_BREATH_PACKAGE_ID
            },
            "execution": {
                "runtime_execution_performed": false,
                "platform_execution_performed": false,
                "broker_transport_used": false,
                "downstream_shell_runtime_used": false,
                "legacy_app_dependency_used": false,
                "legacy_rusty_xr_repo_used": false
            },
            "shell_handoff": {
                "handoff_id": "shell_handoff.projected_motion_breath.loopback",
                "target_host_profile": "host.headset",
                "shell_app_id": "app.downstream_shell",
                "binding_pairs": [
                    {"stream_id": "stream.motion.object_pose", "direction": "publish"},
                    {"stream_id": "stream.breath.feedback_state", "direction": "subscribe"},
                    {"stream_id": "stream.breath.feedback_receipt", "direction": "publish"}
                ],
                "command_ids": ["command.breath.status"],
                "transport_offers": [
                    {"transport_id": "transport.shell_loopback", "transport": "http"}
                ]
            },
            "package_contract": {
                "exported_stream_ids": [
                    "stream.motion.object_pose",
                    "stream.breath.feedback_state",
                    "stream.breath.feedback_receipt"
                ],
                "feedback_sink_provides_streams": [
                    "stream.breath.feedback_state",
                    "stream.breath.feedback_receipt"
                ]
            },
            "scorecard": {
                "status": "pass"
            }
        })
    }

    #[test]
    fn package_evidence_intake_accepts_projected_motion_report() {
        let root = temp_root("package-evidence-intake");
        let report_path = root.join("manifold-package-validation.json");
        let report = projected_motion_package_validation_report();
        save_json(&report_path, &report).expect("save package validation report");

        let loaded =
            load_manifold_package_validation_report(&report_path).expect("load package report");
        let intake = package_evidence_intake_for_validation_report(
            &loaded,
            Some(&report_path),
            PROJECTED_MOTION_BREATH_PACKAGE_ID,
        );

        assert_eq!(intake.schema_id, PACKAGE_EVIDENCE_INTAKE_REPORT_SCHEMA);
        assert_eq!(
            intake.source_report_schema,
            MANIFOLD_PACKAGE_VALIDATION_REPORT_SCHEMA
        );
        assert_eq!(intake.status, StudioPackageEvidenceIntakeStatus::Ready);
        assert_eq!(intake.issue_code, None);
        assert_eq!(intake.execution_policy, "not_executed.review_only");
        assert_eq!(intake.runtime_authority, "rusty.manifold");
        assert_eq!(intake.authoring_authority, "rusty.studio");
        assert_eq!(intake.platform_validation_authority, "rusty.hostess");
        assert!(!intake.runtime_execution_performed);
        assert!(!intake.platform_execution_performed);
        assert_eq!(intake.required_check_count, 3);
        assert_eq!(intake.ready_required_check_count, 3);
        assert_eq!(intake.blocked_required_check_count, 0);
        assert_eq!(intake.target_package_check_count, 17);
        assert_eq!(intake.observed_check_count, 14);
        assert!(intake
            .prohibited_actions
            .iter()
            .any(|action| action == "start_runtime_package"));
        assert!(intake.entries.iter().any(|entry| {
            entry.check_id
                == "validation.package.package.projected_motion_breath.projected_motion_goldens"
                && entry.required_for_studio
                && entry.decision == StudioPackageEvidenceIntakeDecision::Ready
        }));
        assert!(intake
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
    }

    #[test]
    fn package_evidence_intake_blocks_missing_required_projected_motion_check() {
        let mut report = projected_motion_package_validation_report();
        report.checks.retain(|check| {
            check.check_id
                != "validation.package.package.projected_motion_breath.projected_motion_goldens"
        });
        let intake = package_evidence_intake_for_validation_report(
            &report,
            Some(Path::new("target/manifold-package-validation.json")),
            PROJECTED_MOTION_BREATH_PACKAGE_ID,
        );

        assert_eq!(intake.status, StudioPackageEvidenceIntakeStatus::Blocked);
        assert_eq!(
            intake.issue_code.as_deref(),
            Some("studio.issue.package_evidence_required_check_missing")
        );
        assert_eq!(intake.ready_required_check_count, 2);
        assert_eq!(intake.blocked_required_check_count, 1);
        assert!(intake.entries.iter().any(|entry| {
            entry.check_id
                == "validation.package.package.projected_motion_breath.projected_motion_goldens"
                && entry.required_for_studio
                && entry.decision == StudioPackageEvidenceIntakeDecision::BlockedByMissingCheck
        }));
    }

    #[test]
    fn package_evidence_intake_rejects_unsupported_source_schema() {
        let mut report = projected_motion_package_validation_report();
        report.schema_id = "rusty.manifold.package.validation_report.v0".to_string();
        let intake = package_evidence_intake_for_validation_report(
            &report,
            Some(Path::new("target/manifold-package-validation.json")),
            PROJECTED_MOTION_BREATH_PACKAGE_ID,
        );

        assert_eq!(intake.status, StudioPackageEvidenceIntakeStatus::Rejected);
        assert_eq!(
            intake.issue_code.as_deref(),
            Some("studio.issue.package_evidence_source_schema")
        );
    }

    #[test]
    fn projected_motion_breath_shell_handoff_review_accepts_hostess_evidence() {
        let root = temp_root("projected-motion-shell-handoff-review");
        let evidence_path = root.join("pmb-shell-handoff.json");
        let evidence = projected_motion_shell_handoff_evidence();
        save_json(&evidence_path, &evidence).expect("save shell handoff evidence");
        let loaded = load_projected_motion_breath_shell_handoff_evidence(&evidence_path)
            .expect("load shell handoff evidence");

        let review = projected_motion_breath_shell_handoff_review_for_evidence(
            &loaded,
            Some(&evidence_path),
        );

        assert_eq!(
            review.schema_id,
            PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA
        );
        assert_eq!(
            review.status,
            StudioProjectedMotionBreathShellHandoffReviewStatus::Ready
        );
        assert_eq!(review.issue_code, None);
        assert_eq!(
            review.target_package_id.as_deref(),
            Some(PROJECTED_MOTION_BREATH_PACKAGE_ID)
        );
        assert_eq!(
            review.handoff_id.as_deref(),
            Some("shell_handoff.projected_motion_breath.loopback")
        );
        assert_eq!(review.required_binding_count, 3);
        assert_eq!(review.ready_required_binding_count, 3);
        assert!(review
            .stream_bindings
            .contains(&"stream.breath.feedback_receipt:publish".to_string()));
        assert!(review
            .command_ids
            .contains(&"command.breath.status".to_string()));
        assert!(review
            .transport_ids
            .contains(&"transport.shell_loopback".to_string()));
        assert!(review.feedback_receipt_exported);
        assert!(review.feedback_sink_provides_receipt);
        assert_eq!(review.runtime_authority, "rusty.manifold");
        assert_eq!(review.authoring_authority, "rusty.studio");
        assert_eq!(review.platform_validation_authority, "rusty.hostess");
        assert!(!review.runtime_execution_performed);
        assert!(!review.platform_execution_performed);
        assert!(!review.broker_transport_used);
        assert!(!review.downstream_shell_runtime_used);
        assert!(!review.legacy_app_dependency_used);
        assert!(review
            .prohibited_actions
            .iter()
            .any(|action| action == "launch_downstream_shell"));
        assert!(review
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
    }

    #[test]
    fn projected_motion_breath_shell_handoff_review_blocks_missing_receipt_binding() {
        let mut evidence = projected_motion_shell_handoff_evidence();
        let bindings = evidence["shell_handoff"]["binding_pairs"]
            .as_array_mut()
            .expect("binding pairs");
        bindings.retain(|binding| {
            binding.get("stream_id").and_then(Value::as_str)
                != Some("stream.breath.feedback_receipt")
        });

        let review = projected_motion_breath_shell_handoff_review_for_evidence(
            &evidence,
            Some(Path::new("target/pmb-shell-handoff.json")),
        );

        assert_eq!(
            review.status,
            StudioProjectedMotionBreathShellHandoffReviewStatus::Blocked
        );
        assert_eq!(review.ready_required_binding_count, 2);
        assert_eq!(
            review.issue_code.as_deref(),
            Some("studio.issue.projected_motion_breath_shell_handoff_required_bindings")
        );
        assert!(review.checks.iter().any(|check| {
            check.check_id == "studio.check.projected_motion_breath_shell_handoff.required_bindings"
                && check.status == StudioValidationStatus::Fail
        }));
    }

    #[test]
    fn projected_motion_breath_authoring_review_accepts_ready_intake_and_profile() {
        let root = temp_root("projected-motion-authoring-review");
        let intake_path = root.join("package-evidence-intake.json");
        let profile_path = root.join("profile-synthetic.json");
        let intake = ready_projected_motion_package_evidence_intake();
        let profile = projected_motion_profile_document();
        save_json(&intake_path, &intake).expect("save intake");
        save_json(&profile_path, &profile).expect("save profile");
        let loaded_intake =
            load_package_evidence_intake_report(&intake_path).expect("load package intake");
        let loaded_profile =
            load_motion_breath_profile_document(&profile_path).expect("load motion profile");

        let review = projected_motion_breath_authoring_review_for_intake(
            &loaded_intake,
            Some(&intake_path),
            &loaded_profile,
            Some(&profile_path),
        );

        assert_eq!(
            review.schema_id,
            PROJECTED_MOTION_BREATH_AUTHORING_REVIEW_SCHEMA
        );
        assert_eq!(
            review.status,
            StudioProjectedMotionBreathAuthoringReviewStatus::Ready
        );
        assert_eq!(review.issue_code, None);
        assert_eq!(
            review.profile_id.as_deref(),
            Some("profile.projected_motion_breath.synthetic_default")
        );
        assert_eq!(
            review.target_module_id.as_deref(),
            Some("module.breath.projected_motion")
        );
        assert_eq!(
            review.projection_mode.as_deref(),
            Some("principal_motion_axis")
        );
        assert_eq!(review.proposed_command_id, "command.breath.set_profile");
        assert_eq!(
            review.proposed_profile_operation,
            "propose_profile_for_runtime_owner_review"
        );
        assert_eq!(review.runtime_authority, "rusty.manifold");
        assert_eq!(review.authoring_authority, "rusty.studio");
        assert_eq!(review.platform_validation_authority, "rusty.hostess");
        assert!(!review.runtime_execution_performed);
        assert!(!review.platform_execution_performed);
        assert_eq!(review.package_ready_required_check_count, 3);
        assert_eq!(review.package_blocked_required_check_count, 0);
        assert_eq!(review.required_package_checks.len(), 3);
        assert!(review.input_kinds.contains(&"pose".to_string()));
        assert!(review.input_kinds.contains(&"vector3".to_string()));
        assert!(review
            .prohibited_actions
            .iter()
            .any(|action| action == "start_runtime_package"));
        assert!(review
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
    }

    #[test]
    fn projected_motion_breath_authoring_review_blocks_unready_package_evidence() {
        let mut report = projected_motion_package_validation_report();
        report.checks.retain(|check| {
            check.check_id
                != "validation.package.package.projected_motion_breath.projected_motion_goldens"
        });
        let intake = package_evidence_intake_for_validation_report(
            &report,
            Some(Path::new("target/manifold-package-validation.json")),
            PROJECTED_MOTION_BREATH_PACKAGE_ID,
        );
        let review = projected_motion_breath_authoring_review_for_intake(
            &intake,
            Some(Path::new("target/package-evidence-intake.json")),
            &projected_motion_profile_document(),
            Some(Path::new("target/profile-synthetic.json")),
        );

        assert_eq!(
            review.status,
            StudioProjectedMotionBreathAuthoringReviewStatus::Blocked
        );
        assert_eq!(
            review.issue_code.as_deref(),
            Some("studio.issue.package_evidence_required_check_missing")
        );
        assert_eq!(review.package_ready_required_check_count, 2);
        assert_eq!(review.package_blocked_required_check_count, 1);
    }

    #[test]
    fn projected_motion_breath_authoring_review_rejects_unsupported_profile_schema() {
        let intake = ready_projected_motion_package_evidence_intake();
        let mut profile = projected_motion_profile_document();
        profile["$schema"] = serde_json::json!("rusty.motion_breath_profile.v0");
        let review = projected_motion_breath_authoring_review_for_intake(
            &intake,
            Some(Path::new("target/package-evidence-intake.json")),
            &profile,
            Some(Path::new("target/profile-synthetic.json")),
        );

        assert_eq!(
            review.status,
            StudioProjectedMotionBreathAuthoringReviewStatus::Rejected
        );
        assert_eq!(
            review.issue_code.as_deref(),
            Some("studio.issue.motion_breath_profile_schema")
        );
    }

    #[test]
    fn projected_motion_breath_source_adapter_selection_accepts_descriptor() {
        let review = ready_projected_motion_authoring_review();
        let descriptors = projected_motion_source_adapter_descriptors();

        let selection = projected_motion_breath_source_adapter_selection_review_for_authoring(
            &review,
            Some(Path::new(
                "target/projected-motion-breath-authoring-review.json",
            )),
            &descriptors,
            Some(Path::new("target/source-adapter-descriptors.json")),
            "adapter.projected_motion_breath.external_patch_stream_bridge_shape",
        );

        assert_eq!(
            selection.schema_id,
            PROJECTED_MOTION_BREATH_SOURCE_ADAPTER_SELECTION_REVIEW_SCHEMA
        );
        assert_eq!(
            selection.status,
            StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Ready
        );
        assert_eq!(selection.issue_code, None);
        assert_eq!(
            selection.selected_source_kind.as_deref(),
            Some("external_patch_stream_bridge")
        );
        assert_eq!(selection.selected_input_kind.as_deref(), Some("vector3"));
        assert_eq!(
            selection.selected_output_stream_id.as_deref(),
            Some("stream.motion.vector3")
        );
        assert_eq!(selection.source_descriptor_count, 2);
        assert_eq!(selection.matching_descriptor_count, 1);
        assert_eq!(selection.execution_policy, "not_executed.proposal_only");
        assert_eq!(selection.runtime_authority, "rusty.manifold");
        assert_eq!(selection.authoring_authority, "rusty.studio");
        assert_eq!(selection.platform_validation_authority, "rusty.hostess");
        assert!(!selection.runtime_execution_performed);
        assert!(!selection.platform_execution_performed);
        assert_eq!(
            selection.proposal_kind,
            "propose_source_adapter_for_runtime_owner_review"
        );
        assert!(selection
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
    }

    #[test]
    fn projected_motion_breath_source_adapter_selection_blocks_missing_adapter() {
        let review = ready_projected_motion_authoring_review();
        let descriptors = projected_motion_source_adapter_descriptors();

        let selection = projected_motion_breath_source_adapter_selection_review_for_authoring(
            &review,
            Some(Path::new(
                "target/projected-motion-breath-authoring-review.json",
            )),
            &descriptors,
            Some(Path::new("target/source-adapter-descriptors.json")),
            "adapter.projected_motion_breath.missing",
        );

        assert_eq!(
            selection.status,
            StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Blocked
        );
        assert_eq!(
            selection.issue_code.as_deref(),
            Some("studio.issue.projected_motion_breath_source_adapter_missing")
        );
        assert_eq!(selection.matching_descriptor_count, 0);
    }

    #[test]
    fn projected_motion_breath_source_adapter_selection_rejects_bad_descriptor_schema() {
        let review = ready_projected_motion_authoring_review();
        let mut descriptors = projected_motion_source_adapter_descriptors();
        descriptors["$schema"] = serde_json::json!("rusty.manifold.source_adapter.v0");

        let selection = projected_motion_breath_source_adapter_selection_review_for_authoring(
            &review,
            Some(Path::new(
                "target/projected-motion-breath-authoring-review.json",
            )),
            &descriptors,
            Some(Path::new("target/source-adapter-descriptors.json")),
            "adapter.projected_motion_breath.external_patch_stream_bridge_shape",
        );

        assert_eq!(
            selection.status,
            StudioProjectedMotionBreathSourceAdapterSelectionReviewStatus::Rejected
        );
        assert_eq!(
            selection.issue_code.as_deref(),
            Some("studio.issue.projected_motion_breath_source_adapter_descriptor_schema")
        );
    }

    #[test]
    fn projected_motion_breath_adapter_normalization_review_accepts_selected_evidence() {
        let root = temp_root("projected-motion-adapter-normalization-review");
        let selection_path = root.join("source-adapter-selection-review.json");
        let package_report_path = root.join("manifold-package-validation.json");
        let source_binding_path = root
            .join("fixtures")
            .join("valid")
            .join("source-binding-synthetic.json");
        let normalization_case_path = root.join("adapter-normalization-external-patch-vector.json");
        let selection = ready_projected_motion_source_adapter_selection();
        let package_report = projected_motion_package_validation_report();
        let source_binding = projected_motion_source_binding_document();
        let normalization_case = projected_motion_adapter_normalization_case_document();
        save_json(&selection_path, &selection).expect("save selection review");
        save_json(&package_report_path, &package_report).expect("save package report");
        save_json(&source_binding_path, &source_binding).expect("save source binding");
        save_json(&normalization_case_path, &normalization_case).expect("save normalization case");

        let loaded_selection =
            load_projected_motion_breath_source_adapter_selection_review_report(&selection_path)
                .expect("load selection review");
        let loaded_package =
            load_manifold_package_validation_report(&package_report_path).expect("load package");
        let loaded_binding =
            load_projected_motion_breath_source_binding_document(&source_binding_path)
                .expect("load source binding");
        let loaded_case = load_projected_motion_breath_adapter_normalization_case_document(
            &normalization_case_path,
        )
        .expect("load normalization case");

        let review = projected_motion_breath_adapter_normalization_evidence_review_for_selection(
            &loaded_selection,
            Some(&selection_path),
            &loaded_package,
            Some(&package_report_path),
            &loaded_binding,
            Some(&source_binding_path),
            &loaded_case,
            Some(&normalization_case_path),
        );

        assert_eq!(
            review.schema_id,
            PROJECTED_MOTION_BREATH_ADAPTER_NORMALIZATION_EVIDENCE_REVIEW_SCHEMA
        );
        assert_eq!(
            review.status,
            StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Ready
        );
        assert_eq!(review.issue_code, None);
        assert_eq!(
            review.selected_adapter_id,
            "adapter.projected_motion_breath.external_patch_stream_bridge_shape"
        );
        assert_eq!(
            review.binding_id.as_deref(),
            Some("binding.projected_motion_breath.synthetic.external_patch_stream")
        );
        assert_eq!(
            review.normalization_case_id.as_deref(),
            Some("case.projected_motion_breath.normalize.external_patch_vector")
        );
        assert_eq!(
            review.source_payload_kind.as_deref(),
            Some("external_patch_channels")
        );
        assert_eq!(
            review.expected_sample_kind.as_deref(),
            Some("vector_motion")
        );
        assert_eq!(
            review.adapter_normalization_check_status,
            Some(StudioValidationStatus::Pass)
        );
        assert!(review.source_binding_selected_adapter_match);
        assert!(review.deterministic_normalization_evidence);
        assert_eq!(review.execution_policy, "not_executed.review_only");
        assert_eq!(review.runtime_authority, "rusty.manifold");
        assert_eq!(review.authoring_authority, "rusty.studio");
        assert_eq!(review.platform_validation_authority, "rusty.hostess");
        assert!(!review.runtime_execution_performed);
        assert!(!review.platform_execution_performed);
        assert!(review
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
    }

    #[test]
    fn projected_motion_breath_adapter_normalization_review_blocks_missing_package_check() {
        let mut package_report = projected_motion_package_validation_report();
        package_report.checks.retain(|check| {
            check.check_id
                != "validation.package.package.projected_motion_breath.projected_motion_adapter_normalization"
        });
        let review = projected_motion_breath_adapter_normalization_evidence_review_for_selection(
            &ready_projected_motion_source_adapter_selection(),
            Some(Path::new("target/source-adapter-selection-review.json")),
            &package_report,
            Some(Path::new("target/manifold-package-validation.json")),
            &projected_motion_source_binding_document(),
            Some(Path::new("fixtures/valid/source-binding-synthetic.json")),
            &projected_motion_adapter_normalization_case_document(),
            Some(Path::new(
                "fixtures/valid/adapter-normalization-external-patch-vector.json",
            )),
        );

        assert_eq!(
            review.status,
            StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Blocked
        );
        assert_eq!(
            review.issue_code.as_deref(),
            Some("studio.issue.projected_motion_breath_adapter_normalization_check_missing")
        );
        assert_eq!(review.adapter_normalization_check_status, None);
    }

    #[test]
    fn projected_motion_breath_adapter_normalization_review_blocks_binding_adapter_drift() {
        let mut source_binding = projected_motion_source_binding_document();
        source_binding["selected_adapter_id"] =
            serde_json::json!("adapter.projected_motion_breath.vector_motion_generic");

        let review = projected_motion_breath_adapter_normalization_evidence_review_for_selection(
            &ready_projected_motion_source_adapter_selection(),
            Some(Path::new("target/source-adapter-selection-review.json")),
            &projected_motion_package_validation_report(),
            Some(Path::new("target/manifold-package-validation.json")),
            &source_binding,
            Some(Path::new("fixtures/valid/source-binding-synthetic.json")),
            &projected_motion_adapter_normalization_case_document(),
            Some(Path::new(
                "fixtures/valid/adapter-normalization-external-patch-vector.json",
            )),
        );

        assert_eq!(
            review.status,
            StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Blocked
        );
        assert_eq!(
            review.issue_code.as_deref(),
            Some("studio.issue.projected_motion_breath_adapter_normalization_binding_adapter")
        );
        assert!(!review.source_binding_selected_adapter_match);
        assert!(!review.deterministic_normalization_evidence);
    }

    #[test]
    fn projected_motion_breath_adapter_normalization_review_rejects_bad_case_schema() {
        let mut normalization_case = projected_motion_adapter_normalization_case_document();
        normalization_case["$schema"] = serde_json::json!(
            "rusty.manifold.projected_motion_breath.adapter_normalization_case.v0"
        );

        let review = projected_motion_breath_adapter_normalization_evidence_review_for_selection(
            &ready_projected_motion_source_adapter_selection(),
            Some(Path::new("target/source-adapter-selection-review.json")),
            &projected_motion_package_validation_report(),
            Some(Path::new("target/manifold-package-validation.json")),
            &projected_motion_source_binding_document(),
            Some(Path::new("fixtures/valid/source-binding-synthetic.json")),
            &normalization_case,
            Some(Path::new(
                "fixtures/valid/adapter-normalization-external-patch-vector.json",
            )),
        );

        assert_eq!(
            review.status,
            StudioProjectedMotionBreathAdapterNormalizationEvidenceReviewStatus::Rejected
        );
        assert_eq!(
            review.issue_code.as_deref(),
            Some("studio.issue.projected_motion_breath_adapter_normalization_case_schema")
        );
    }

    fn write_reference_fixture_tree(root: &Path) {
        write_fixture(
            &root.join("packages/catalog.manifold.json"),
            r#"{
  "$schema": "rusty.manifold.package.catalog.v1",
  "catalog_id": "catalog.test",
  "packages": [
    {
      "package_id": "package.synthetic",
      "manifest_path": "packages/synthetic/manifests/package.manifold.json"
    }
  ]
}"#,
        );
        write_fixture(
            &root.join("packages/synthetic/manifests/package.manifold.json"),
            r#"{
  "$schema": "rusty.manifold.package.manifest.v1",
  "package_id": "package.synthetic",
  "version": "0.1.0",
  "exports": {
    "modules": [
      "module.synthetic_provider"
    ],
    "streams": [],
    "commands": []
  }
}"#,
        );
        write_fixture(
            &root.join("profiles/desktop.json"),
            r#"{
  "$schema": "rusty.manifold.host_run.install_launch_profile.v1",
  "profile_id": "host_run.profile.desktop",
  "host_profile": "host.desktop",
  "app_id": "app.host_shell.desktop",
  "install_route": "install.local_process",
  "launch_route": "launch.local_process",
  "command_bridge": "bridge.local_cli",
  "required_permissions": [],
  "evidence_pull_route": "evidence.filesystem"
}"#,
        );
        write_fixture(
            &root.join("profiles/headset.json"),
            r#"{
  "$schema": "rusty.manifold.host_run.install_launch_profile.v1",
  "profile_id": "host_run.profile.headset",
  "host_profile": "host.headset",
  "app_id": "app.host_shell.headset",
  "install_route": "install.android_package",
  "launch_route": "launch.android_intent",
  "command_bridge": "bridge.adb_intent_file",
  "required_permissions": [
    "permission.bluetooth.scan",
    "permission.bluetooth.connect",
    "permission.location.fine"
  ],
  "evidence_pull_route": "evidence.adb_pull"
}"#,
        );
        write_fixture(
            &root.join("profiles/mobile.json"),
            r#"{
  "$schema": "rusty.manifold.host_run.install_launch_profile.v1",
  "profile_id": "host_run.profile.mobile",
  "host_profile": "host.mobile",
  "app_id": "app.host_shell.mobile",
  "install_route": "install.android_package",
  "launch_route": "launch.android_intent",
  "command_bridge": "bridge.adb_intent_file",
  "required_permissions": [],
  "evidence_pull_route": "evidence.adb_pull"
}"#,
        );
    }

    fn write_multi_package_reference_fixture_tree(root: &Path) {
        write_reference_fixture_tree(root);
        write_fixture(
            &root.join("packages/catalog.manifold.json"),
            r#"{
  "$schema": "rusty.manifold.package.catalog.v1",
  "catalog_id": "catalog.test",
  "packages": [
    {
      "package_id": "package.synthetic",
      "manifest_path": "packages/synthetic/manifests/package.manifold.json"
    },
    {
      "package_id": "package.biosignal",
      "manifest_path": "packages/biosignal/manifests/package.manifold.json"
    }
  ]
}"#,
        );
        write_fixture(
            &root.join("packages/biosignal/manifests/package.manifold.json"),
            r#"{
  "$schema": "rusty.manifold.package.manifest.v1",
  "package_id": "package.biosignal",
  "version": "0.1.0",
  "exports": {
    "modules": [
      "module.biosignal.processor",
      "module.biosignal.provider"
    ],
    "streams": [],
    "commands": []
  }
}"#,
        );
    }

    fn valid_project_with_relative_references() -> StudioProject {
        let mut project = valid_project();
        project.package_catalog_path = "packages/catalog.manifold.json".to_string();
        project.host_run_profile_paths = vec![
            "profiles/desktop.json".to_string(),
            "profiles/headset.json".to_string(),
        ];
        project
    }

    fn valid_shell_project_with_relative_references() -> StudioProject {
        let mut project = valid_project_with_relative_references();
        project.graphs[0].nodes.push(StudioNode {
            node_id: "node.module.synthetic_provider".to_string(),
            kind: StudioNodeKind::Module,
            reference_id: "module.synthetic_provider".to_string(),
            label: "Synthetic Provider".to_string(),
        });
        project.graphs[0].nodes.push(StudioNode {
            node_id: "node.shell.operator".to_string(),
            kind: StudioNodeKind::OperatorShell,
            reference_id: "shell.synthetic.operator".to_string(),
            label: "Operator Shell".to_string(),
        });
        project.graphs[0].edges.push(StudioEdge {
            edge_id: "edge.package_module".to_string(),
            kind: StudioEdgeKind::PackageProvidesModule,
            source_node_id: "node.package.synthetic".to_string(),
            target_node_id: "node.module.synthetic_provider".to_string(),
        });
        project.graphs[0].edges.push(StudioEdge {
            edge_id: "edge.shell_command".to_string(),
            kind: StudioEdgeKind::CommandBinding,
            source_node_id: "node.shell.operator".to_string(),
            target_node_id: "node.module.synthetic_provider".to_string(),
        });
        project
    }

    fn valid_multi_shell_project_with_relative_references() -> StudioProject {
        let mut project = valid_shell_project_with_relative_references();
        project.host_run_profile_paths = vec![
            "profiles/desktop.json".to_string(),
            "profiles/mobile.json".to_string(),
            "profiles/headset.json".to_string(),
        ];

        let mut phone_graph = project.graphs[0].clone();
        phone_graph.graph_id = "studio.graph.phone".to_string();
        phone_graph.display_name = "Phone Graph".to_string();
        phone_graph.target_host_profile = "host_run.profile.mobile".to_string();
        for node in &mut phone_graph.nodes {
            if node.kind == StudioNodeKind::HostProfile {
                node.node_id = "node.host.mobile".to_string();
                node.reference_id = "host_run.profile.mobile".to_string();
                node.label = "Phone".to_string();
            }
            if node.kind == StudioNodeKind::OperatorShell {
                node.reference_id = "shell.synthetic.phone_operator".to_string();
                node.label = "Phone Shell".to_string();
            }
        }
        for edge in &mut phone_graph.edges {
            if edge.kind == StudioEdgeKind::ShellTargetsHostProfile {
                edge.target_node_id = "node.host.mobile".to_string();
            }
        }
        if let Some(layout) = phone_graph.layout.as_mut() {
            layout.layout_id = "studio.layout.phone".to_string();
            for node in &mut layout.nodes {
                if node.node_id == "node.host.desktop" {
                    node.node_id = "node.host.mobile".to_string();
                }
            }
        }

        let mut quest_graph = project.graphs[0].clone();
        quest_graph.graph_id = "studio.graph.quest".to_string();
        quest_graph.display_name = "Quest Graph".to_string();
        quest_graph.target_host_profile = "host_run.profile.headset".to_string();
        for node in &mut quest_graph.nodes {
            if node.kind == StudioNodeKind::HostProfile {
                node.node_id = "node.host.headset".to_string();
                node.reference_id = "host_run.profile.headset".to_string();
                node.label = "Quest".to_string();
            }
            if node.kind == StudioNodeKind::OperatorShell {
                node.reference_id = "shell.synthetic.quest_operator".to_string();
                node.label = "Quest Shell".to_string();
            }
        }
        for edge in &mut quest_graph.edges {
            if edge.kind == StudioEdgeKind::ShellTargetsHostProfile {
                edge.target_node_id = "node.host.headset".to_string();
            }
        }
        if let Some(layout) = quest_graph.layout.as_mut() {
            layout.layout_id = "studio.layout.quest".to_string();
            for node in &mut layout.nodes {
                if node.node_id == "node.host.desktop" {
                    node.node_id = "node.host.headset".to_string();
                }
            }
        }

        project.graphs.push(phone_graph);
        project.graphs.push(quest_graph);
        project
    }

    fn save_selected_shell_bundles(project: &StudioProject, base_dir: &Path, bundle_root: &Path) {
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(project, Some(base_dir), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
    }

    #[test]
    fn valid_project_passes() {
        let report = validate_project(&valid_project());
        assert_eq!(report.status, StudioValidationStatus::Pass);
    }

    #[test]
    fn save_project_roundtrips_authored_project_json() {
        let root = temp_root("save-project");
        let path = root.join("nested/project.json");
        let project = valid_project();

        save_project(&path, &project).expect("save project");
        let loaded = load_project(&path).expect("load saved project");

        assert_eq!(loaded, project);
    }

    #[test]
    fn duplicate_node_fails() {
        let mut project = valid_project();
        let duplicate = project.graphs[0].nodes[0].clone();
        project.graphs[0].nodes.push(duplicate);
        let report = validate_project(&project);
        assert_eq!(report.status, StudioValidationStatus::Fail);
        let issue = report
            .checks
            .iter()
            .find(|check| check.issue_code.as_deref() == Some("studio.issue.duplicate_node_id"))
            .expect("duplicate node issue");
        assert_eq!(issue.graph_id.as_deref(), Some("studio.graph.test"));
        assert_eq!(issue.node_ids, vec!["node.package.synthetic".to_string()]);
    }

    #[test]
    fn missing_edge_target_fails() {
        let mut project = valid_project();
        project.graphs[0].edges[0].target_node_id = "node.missing".to_string();
        let report = validate_project(&project);
        assert_eq!(report.status, StudioValidationStatus::Fail);
        let issue = report
            .checks
            .iter()
            .find(|check| check.issue_code.as_deref() == Some("studio.issue.missing_edge_target"))
            .expect("missing edge target issue");
        assert_eq!(issue.graph_id.as_deref(), Some("studio.graph.test"));
        assert_eq!(issue.node_ids, vec!["node.missing".to_string()]);
        assert_eq!(issue.edge_ids, vec!["edge.package_host".to_string()]);
    }

    #[test]
    fn invalid_layout_references_fail() {
        let mut project = valid_project();
        let layout = project.graphs[0].layout.as_mut().expect("layout");
        layout.nodes[0].node_id = "node.missing".to_string();
        layout.edges[0].edge_id = "edge.missing".to_string();
        layout.nodes[1].width = 0;

        let report = validate_project(&project);

        assert_eq!(report.status, StudioValidationStatus::Fail);
        assert!(report.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.layout_node_missing")
                && check.node_ids == vec!["node.missing".to_string()]
        }));
        assert!(report.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.layout_edge_missing")
                && check.edge_ids == vec!["edge.missing".to_string()]
        }));
        assert!(report.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.invalid_layout_node_box")
                && check.node_ids == vec!["node.host.desktop".to_string()]
        }));
    }

    #[test]
    fn missing_target_host_profile_fails() {
        let mut project = valid_project();
        project.graphs[0].target_host_profile = "host_run.profile.headset".to_string();
        let report = validate_project(&project);
        assert_eq!(report.status, StudioValidationStatus::Fail);
        let issue = report
            .checks
            .iter()
            .find(|check| {
                check.issue_code.as_deref() == Some("studio.issue.missing_target_host_profile")
            })
            .expect("missing target host issue");
        assert_eq!(issue.graph_id.as_deref(), Some("studio.graph.test"));
        assert_eq!(
            issue.reference_ids,
            vec!["host_run.profile.headset".to_string()]
        );
    }

    #[test]
    fn missing_reference_paths_fail_when_base_dir_is_supplied() {
        let report = validate_project_with_base(&valid_project(), Some(Path::new("missing-base")));
        assert_eq!(report.status, StudioValidationStatus::Fail);
        assert!(report.checks.iter().any(
            |check| check.issue_code.as_deref() == Some("studio.issue.package_catalog_missing")
        ));
        assert!(report
            .checks
            .iter()
            .any(|check| check.issue_code.as_deref()
                == Some("studio.issue.host_run_profile_missing")));
    }

    #[test]
    fn content_reference_resolution_accepts_catalog_manifest_and_profile() {
        let root = temp_root("content-pass");
        write_reference_fixture_tree(&root);
        let report =
            validate_project_with_base(&valid_project_with_relative_references(), Some(&root));
        assert_eq!(report.status, StudioValidationStatus::Pass);
    }

    #[test]
    fn missing_package_catalog_reference_fails() {
        let root = temp_root("missing-package");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();
        project.graphs[0].nodes[0].reference_id = "package.missing".to_string();
        let report = validate_project_with_base(&project, Some(&root));
        assert_eq!(report.status, StudioValidationStatus::Fail);
        let issue = report
            .checks
            .iter()
            .find(|check| {
                check.issue_code.as_deref() == Some("studio.issue.package_reference_missing")
            })
            .expect("package reference issue");
        assert_eq!(issue.graph_id.as_deref(), Some("studio.graph.test"));
        assert_eq!(issue.node_ids, vec!["node.package.synthetic".to_string()]);
        assert_eq!(issue.reference_ids, vec!["package.missing".to_string()]);
    }

    #[test]
    fn missing_module_export_reference_fails() {
        let root = temp_root("missing-module");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();
        project.graphs[0].nodes.push(StudioNode {
            node_id: "node.module.missing".to_string(),
            kind: StudioNodeKind::Module,
            reference_id: "module.missing".to_string(),
            label: "Missing Module".to_string(),
        });
        let report = validate_project_with_base(&project, Some(&root));
        assert_eq!(report.status, StudioValidationStatus::Fail);
        assert!(report.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.module_reference_missing")
        }));
    }

    #[test]
    fn resolve_counts_graph_parts() {
        let resolved = resolve_project(&valid_project());
        let graph = &resolved.graphs[0];
        assert_eq!(graph.package_count, 1);
        assert_eq!(graph.module_count, 0);
        assert_eq!(graph.edge_count, 1);
    }

    #[test]
    fn export_plan_collects_bundle_refs() {
        let plan = export_plan(&valid_project());
        assert_eq!(plan.bundles[0].package_ids, vec!["package.synthetic"]);
        assert_eq!(
            plan.bundles[0].target_host_profile,
            "host_run.profile.desktop"
        );
    }

    #[test]
    fn shell_descriptor_exports_valid_graph() {
        let root = temp_root("shell-descriptor");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();

        let report = shell_descriptor_for_graph(&project, Some(&root), "studio.graph.test");
        let descriptor = report.descriptor.expect("descriptor exported");

        assert_eq!(report.status, StudioShellDescriptorStatus::Exported);
        assert_eq!(report.issue_code, None);
        assert_eq!(descriptor.schema_id, SHELL_DESCRIPTOR_SCHEMA);
        assert_eq!(descriptor.project_id, "studio.project.test");
        assert_eq!(descriptor.shell_id, "shell.synthetic.operator");
        assert_eq!(descriptor.target_host_profile, "host_run.profile.desktop");
        assert_eq!(
            descriptor.host_profile.host_profile.as_deref(),
            Some("host.desktop")
        );
        assert_eq!(descriptor.package_ids, vec!["package.synthetic"]);
        assert_eq!(descriptor.module_ids, vec!["module.synthetic_provider"]);
        assert_eq!(descriptor.command_bindings.len(), 1);
    }

    #[test]
    fn shell_descriptor_roundtrips_and_validates() {
        let root = temp_root("shell-descriptor-roundtrip");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();
        let report = shell_descriptor_for_graph(&project, Some(&root), "studio.graph.test");
        let descriptor = report.descriptor.expect("descriptor exported");
        let path = root.join("target/descriptor.json");

        save_json(&path, &descriptor).expect("save descriptor");
        let loaded = load_shell_descriptor(&path).expect("load descriptor");
        let validation = validate_shell_descriptor(&loaded);

        assert_eq!(loaded, descriptor);
        assert_eq!(
            validation.schema_id,
            SHELL_DESCRIPTOR_VALIDATION_REPORT_SCHEMA
        );
        assert_eq!(validation.status, StudioValidationStatus::Pass);
    }

    #[test]
    fn shell_descriptor_validation_rejects_target_mismatch() {
        let root = temp_root("shell-descriptor-target-mismatch");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();
        let report = shell_descriptor_for_graph(&project, Some(&root), "studio.graph.test");
        let mut descriptor = report.descriptor.expect("descriptor exported");
        descriptor.host_profile.profile_id = "host_run.profile.headset".to_string();

        let validation = validate_shell_descriptor(&descriptor);

        assert_eq!(validation.status, StudioValidationStatus::Fail);
        assert!(validation.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.host_profile_target_mismatch")
        }));
    }

    #[test]
    fn shell_artifacts_export_desktop_phone_and_quest_descriptors() {
        let root = temp_root("shell-artifacts");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();

        let report = shell_artifacts_for_project(&project, Some(&root));
        let manifest = report.manifest.expect("shell artifact manifest");

        assert_eq!(report.status, StudioShellArtifactStatus::Exported);
        assert_eq!(report.issue_code, None);
        assert_eq!(manifest.schema_id, SHELL_ARTIFACT_MANIFEST_SCHEMA);
        assert_eq!(manifest.artifacts.len(), 3);
        assert_eq!(report.descriptors.len(), 3);
        assert!(report.rejections.is_empty());
        assert!(manifest
            .artifacts
            .iter()
            .any(|artifact| artifact.target_kind == StudioShellTargetKind::Desktop));
        assert!(manifest
            .artifacts
            .iter()
            .any(|artifact| artifact.target_kind == StudioShellTargetKind::Phone));
        assert!(manifest
            .artifacts
            .iter()
            .any(|artifact| artifact.target_kind == StudioShellTargetKind::Quest));
        assert!(manifest.artifacts.iter().all(|artifact| {
            artifact
                .descriptor_path
                .starts_with("descriptors/studio.graph.")
                && artifact.descriptor_path.ends_with(".shell-descriptor.json")
        }));
        assert!(report
            .descriptors
            .iter()
            .all(|descriptor| validate_shell_descriptor(descriptor).status
                == StudioValidationStatus::Pass));
    }

    #[test]
    fn shell_artifact_manifest_roundtrips_and_validates_descriptors() {
        let root = temp_root("shell-artifact-manifest-roundtrip");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let report = shell_artifacts_for_project(&project, Some(&root));
        let manifest = report.manifest.expect("shell artifact manifest");
        let manifest_path = root.join("shell-artifacts.json");

        for descriptor in &report.descriptors {
            let descriptor_path = resolve_manifest_relative_path(
                &root,
                &shell_descriptor_artifact_path(&descriptor.graph_id),
            );
            save_json(&descriptor_path, descriptor).expect("save descriptor");
        }
        save_json(&manifest_path, &manifest).expect("save manifest");

        let loaded_manifest =
            load_shell_artifact_manifest(&manifest_path).expect("load shell artifact manifest");
        let validation = validate_shell_artifact_manifest(&loaded_manifest, Some(&root));

        assert_eq!(loaded_manifest, manifest);
        assert_eq!(
            validation.schema_id,
            SHELL_ARTIFACT_MANIFEST_VALIDATION_REPORT_SCHEMA
        );
        assert_eq!(validation.status, StudioValidationStatus::Pass);
    }

    #[test]
    fn shell_artifact_manifest_validation_rejects_descriptor_mismatch() {
        let root = temp_root("shell-artifact-manifest-descriptor-mismatch");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let report = shell_artifacts_for_project(&project, Some(&root));
        let mut manifest = report.manifest.expect("shell artifact manifest");

        for descriptor in &report.descriptors {
            let descriptor_path = resolve_manifest_relative_path(
                &root,
                &shell_descriptor_artifact_path(&descriptor.graph_id),
            );
            save_json(&descriptor_path, descriptor).expect("save descriptor");
        }
        manifest.artifacts[1].shell_id = "shell.synthetic.changed".to_string();

        let validation = validate_shell_artifact_manifest(&manifest, Some(&root));

        assert_eq!(validation.status, StudioValidationStatus::Fail);
        assert!(validation.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.descriptor_shell_mismatch")
        }));
    }

    #[test]
    fn shell_artifact_manifest_validation_rejects_path_traversal() {
        let root = temp_root("shell-artifact-manifest-path-traversal");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let report = shell_artifacts_for_project(&project, Some(&root));
        let mut manifest = report.manifest.expect("shell artifact manifest");
        manifest.artifacts[0].descriptor_path = "../outside.json".to_string();

        let validation = validate_shell_artifact_manifest(&manifest, Some(&root));

        assert_eq!(validation.status, StudioValidationStatus::Fail);
        assert!(validation.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.invalid_descriptor_path")
        }));
    }

    #[test]
    fn shell_templates_export_manifest_driven_index_and_templates() {
        let root = temp_root("shell-templates");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let report = shell_artifacts_for_project(&project, Some(&root));
        let manifest = report.manifest.expect("shell artifact manifest");

        for descriptor in &report.descriptors {
            let descriptor_path = resolve_manifest_relative_path(
                &root,
                &shell_descriptor_artifact_path(&descriptor.graph_id),
            );
            save_json(&descriptor_path, descriptor).expect("save descriptor");
        }

        let template_report = shell_templates_for_artifact_manifest(&manifest, Some(&root));
        let index = template_report.index.expect("shell template index");

        assert_eq!(template_report.status, StudioShellTemplateStatus::Exported);
        assert_eq!(template_report.issue_code, None);
        assert_eq!(index.schema_id, SHELL_TEMPLATE_INDEX_SCHEMA);
        assert_eq!(index.templates.len(), 3);
        assert_eq!(template_report.templates.len(), 3);
        assert!(index
            .templates
            .iter()
            .any(|entry| entry.template_path.starts_with("shells/desktop/")));
        assert!(index
            .templates
            .iter()
            .any(|entry| entry.template_path.starts_with("shells/phone/")));
        assert!(index
            .templates
            .iter()
            .any(|entry| entry.template_path.starts_with("shells/quest/")));
        assert!(template_report.templates.iter().all(|template| {
            template.schema_id == SHELL_TEMPLATE_MANIFEST_SCHEMA
                && template.descriptor_path.starts_with("descriptors/")
                && template.runtime_authority.command_session_authority == "rusty.manifold"
                && template.runtime_authority.install_launch_evidence_authority == "rusty.hostess"
                && template.runtime_authority.studio_role == "authoring.export_planning"
        }));
    }

    #[test]
    fn selected_shell_bundle_exports_single_graph_contract() {
        let root = temp_root("selected-shell-bundle");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();

        let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");

        assert_eq!(report.status, StudioShellBundleStatus::Exported);
        assert_eq!(report.issue_code, None);
        assert_eq!(report.graph_id, "studio.graph.test");
        assert_eq!(
            report.bundle_files,
            vec![
                "descriptors/studio.graph.test.manifold-shell-handoff.json".to_string(),
                "descriptors/studio.graph.test.shell-descriptor.json".to_string(),
                "shell-artifacts.json".to_string(),
                "shell-templates.json".to_string(),
                "shells/desktop/studio.graph.test.shell-template.json".to_string(),
            ]
        );
        assert_eq!(
            report
                .descriptor_validation
                .as_ref()
                .expect("descriptor validation")
                .status,
            StudioValidationStatus::Pass
        );
        assert_eq!(
            report
                .artifact_validation
                .as_ref()
                .expect("artifact validation")
                .status,
            StudioValidationStatus::Pass
        );
        assert_eq!(
            report
                .template_validation
                .as_ref()
                .expect("template validation")
                .status,
            StudioValidationStatus::Pass
        );
        let manifest = report
            .artifact_manifest
            .as_ref()
            .expect("artifact manifest");
        assert_eq!(manifest.artifacts.len(), 1);
        assert_eq!(
            manifest.manifest_id,
            "studio.shell_artifacts.studio.project.test.studio.graph.test"
        );
        let index = report.template_index.as_ref().expect("template index");
        assert_eq!(index.templates.len(), 1);
        assert_eq!(
            index.index_id,
            "studio.shell_templates.studio.project.test.studio.graph.test"
        );
        assert_eq!(
            report
                .template_manifest
                .as_ref()
                .expect("template manifest")
                .runtime_authority
                .install_launch_evidence_authority,
            "rusty.hostess"
        );
    }

    #[test]
    fn selected_shell_bundle_writes_valid_files() {
        let root = temp_root("selected-shell-bundle-write");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();
        let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");
        let output_dir = root.join("selected-shell");

        let written_files = save_shell_bundle(&output_dir, &report).expect("save shell bundle");

        assert_eq!(written_files, report.bundle_files);
        assert!(output_dir
            .join("descriptors/studio.graph.test.shell-descriptor.json")
            .is_file());
        let manifold_handoff_path =
            output_dir.join("descriptors/studio.graph.test.manifold-shell-handoff.json");
        assert!(manifold_handoff_path.is_file());
        assert!(output_dir.join("shell-artifacts.json").is_file());
        assert!(output_dir.join("shell-templates.json").is_file());
        assert!(output_dir
            .join("shells/desktop/studio.graph.test.shell-template.json")
            .is_file());
        let manifold_handoff: Value = serde_json::from_str(
            &std::fs::read_to_string(&manifold_handoff_path).expect("read Manifold handoff"),
        )
        .expect("parse Manifold handoff");
        assert_eq!(
            manifold_handoff.get("$schema").and_then(Value::as_str),
            Some(MANIFOLD_SHELL_HANDOFF_SCHEMA)
        );
        assert_eq!(
            manifold_handoff.get("handoff_id").and_then(Value::as_str),
            Some("shell_handoff.studio.graph.test")
        );
        assert_eq!(
            manifold_handoff
                .get("validation_slot_id")
                .and_then(Value::as_str),
            Some(DEFAULT_MANIFOLD_SHELL_HANDOFF_VALIDATION_SLOT_ID)
        );

        let manifest =
            load_shell_artifact_manifest(&output_dir.join("shell-artifacts.json")).unwrap();
        let artifact_validation = validate_shell_artifact_manifest(&manifest, Some(&output_dir));
        assert_eq!(artifact_validation.status, StudioValidationStatus::Pass);
        let index = load_shell_template_index(&output_dir.join("shell-templates.json")).unwrap();
        let template_validation = validate_shell_template_index(&index, Some(&output_dir));
        assert_eq!(template_validation.status, StudioValidationStatus::Pass);
    }

    #[test]
    fn selected_shell_bundle_validation_passes_written_bundle() {
        let root = temp_root("selected-shell-bundle-validate");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();
        let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");
        let output_dir = root.join("selected-shell");
        save_shell_bundle(&output_dir, &report).expect("save shell bundle");

        let validation =
            validate_selected_shell_bundle(&project, Some(&root), "studio.graph.test", &output_dir);

        assert_eq!(validation.status, StudioValidationStatus::Pass);
        assert_eq!(validation.expected_bundle_files, report.bundle_files);
        assert!(validation
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
    }

    #[test]
    fn selected_shell_bundle_validation_rejects_stale_descriptor() {
        let root = temp_root("selected-shell-bundle-stale");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();
        let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");
        let output_dir = root.join("selected-shell");
        save_shell_bundle(&output_dir, &report).expect("save shell bundle");
        let descriptor_path =
            output_dir.join("descriptors/studio.graph.test.shell-descriptor.json");
        let mut descriptor = load_shell_descriptor(&descriptor_path).expect("load descriptor");
        descriptor.shell_id = "shell.synthetic.stale".to_string();
        save_json(&descriptor_path, &descriptor).expect("save stale descriptor");

        let validation =
            validate_selected_shell_bundle(&project, Some(&root), "studio.graph.test", &output_dir);

        assert_eq!(validation.status, StudioValidationStatus::Fail);
        assert!(validation.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.shell_bundle_descriptor_mismatch")
        }));
        assert!(validation.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.shell_artifact_manifest_invalid")
        }));
    }

    #[test]
    fn desktop_shell_handoff_reports_validated_schema_file_entrypoint() {
        let root = temp_root("desktop-shell-handoff");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();
        let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");
        let output_dir = root.join("selected-shell");
        save_shell_bundle(&output_dir, &report).expect("save shell bundle");

        let handoff = desktop_shell_handoff_for_bundle(
            &project,
            Some(&root),
            "studio.graph.test",
            &output_dir,
        );

        assert_eq!(handoff.status, StudioValidationStatus::Pass);
        assert_eq!(handoff.handoff_kind, StudioShellHandoffKind::DesktopShell);
        assert_eq!(handoff.consumer_id, "rusty-studio-desktop-shell");
        assert_eq!(handoff.target_kind, StudioShellTargetKind::Desktop);
        assert_eq!(handoff.validation.status, StudioValidationStatus::Pass);
        assert!(
            handoff
                .descriptor_path
                .ends_with("descriptors\\studio.graph.test.shell-descriptor.json")
                || handoff
                    .descriptor_path
                    .ends_with("descriptors/studio.graph.test.shell-descriptor.json")
        );
        assert!(handoff
            .template_index_path
            .ends_with("shell-templates.json"));
        assert!(
            handoff
                .template_manifest_path
                .ends_with("shells\\desktop\\studio.graph.test.shell-template.json")
                || handoff
                    .template_manifest_path
                    .ends_with("shells/desktop/studio.graph.test.shell-template.json")
        );
        assert_eq!(
            handoff.consumer_args,
            vec![
                "--templates".to_string(),
                output_dir
                    .join("shell-templates.json")
                    .display()
                    .to_string(),
            ]
        );
        let authority = handoff
            .runtime_authority
            .expect("handoff carries runtime authority summary");
        assert_eq!(authority.command_session_authority, "rusty.manifold");
        assert_eq!(authority.install_launch_evidence_authority, "rusty.hostess");
        assert_eq!(authority.studio_role, "authoring.export_planning");
    }

    #[test]
    fn shell_handoff_reports_phone_and_quest_targets() {
        let root = temp_root("multi-target-shell-handoff");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        for (graph_id, expected_kind, expected_handoff, expected_consumer) in [
            (
                "studio.graph.phone",
                StudioShellTargetKind::Phone,
                StudioShellHandoffKind::PhoneShell,
                "rusty-studio-phone-shell",
            ),
            (
                "studio.graph.quest",
                StudioShellTargetKind::Quest,
                StudioShellHandoffKind::QuestShell,
                "rusty-studio-quest-shell",
            ),
        ] {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), graph_id);
            let output_dir = root.join(graph_id);
            save_shell_bundle(&output_dir, &report).expect("save shell bundle");

            let handoff = shell_handoff_for_bundle(&project, Some(&root), graph_id, &output_dir);

            assert_eq!(handoff.status, StudioValidationStatus::Pass);
            assert_eq!(handoff.handoff_kind, expected_handoff);
            assert_eq!(handoff.consumer_id, expected_consumer);
            assert_eq!(handoff.target_kind, expected_kind);
            assert_eq!(handoff.validation.status, StudioValidationStatus::Pass);
            assert_eq!(
                handoff.consumer_args,
                vec![
                    "--templates".to_string(),
                    output_dir
                        .join("shell-templates.json")
                        .display()
                        .to_string(),
                ]
            );
            assert_eq!(
                handoff
                    .runtime_authority
                    .as_ref()
                    .expect("runtime authority")
                    .install_launch_evidence_authority,
                "rusty.hostess"
            );
        }
    }

    #[test]
    fn desktop_shell_handoff_rejects_non_desktop_bundle() {
        let root = temp_root("desktop-shell-handoff-target-mismatch");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.phone");
        let output_dir = root.join("phone-selected-shell");
        save_shell_bundle(&output_dir, &report).expect("save phone shell bundle");

        let handoff = desktop_shell_handoff_for_bundle(
            &project,
            Some(&root),
            "studio.graph.phone",
            &output_dir,
        );

        assert_eq!(handoff.status, StudioValidationStatus::Fail);
        assert_eq!(
            handoff.issue_code.as_deref(),
            Some("studio.issue.shell_handoff_target_mismatch")
        );
        assert_eq!(handoff.handoff_kind, StudioShellHandoffKind::PhoneShell);
        assert_eq!(handoff.target_kind, StudioShellTargetKind::Phone);
        assert!(handoff.consumer_args.is_empty());
        assert_eq!(handoff.validation.status, StudioValidationStatus::Pass);
    }

    #[test]
    fn shell_handoff_readiness_reports_all_target_graphs() {
        let root = temp_root("shell-handoff-readiness");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }

        let readiness = shell_handoff_readiness_for_project(&project, Some(&root), &bundle_root);

        assert_eq!(readiness.schema_id, SHELL_HANDOFF_READINESS_REPORT_SCHEMA);
        assert_eq!(readiness.status, StudioValidationStatus::Pass);
        assert_eq!(readiness.graph_count, 3);
        assert_eq!(readiness.ready_count, 3);
        assert_eq!(readiness.failed_count, 0);
        assert_eq!(readiness.missing_bundle_count, 0);
        assert_eq!(readiness.entries.len(), 3);
        assert_eq!(readiness.target_summaries.len(), 3);
        for target_kind in [
            StudioShellTargetKind::Desktop,
            StudioShellTargetKind::Phone,
            StudioShellTargetKind::Quest,
        ] {
            let summary = readiness
                .target_summaries
                .iter()
                .find(|summary| summary.target_kind == target_kind)
                .expect("target summary");
            assert_eq!(summary.graph_count, 1);
            assert_eq!(summary.ready_count, 1);
            assert_eq!(summary.failed_count, 0);
            assert_eq!(summary.missing_bundle_count, 0);
            assert_eq!(summary.package_count, 1);
            assert_eq!(summary.module_count, 1);
            assert_eq!(summary.operator_shell_count, 1);
            assert!(summary.issue_codes.is_empty());
            assert_eq!(summary.graph_ids.len(), 1);
            assert_eq!(summary.consumer_ids.len(), 1);
            assert_eq!(summary.bundle_dirs.len(), 1);
            assert_eq!(summary.ready_bundle_dirs.len(), 1);
            assert!(summary.failed_bundle_dirs.is_empty());
            assert!(summary.missing_bundle_dirs.is_empty());
            assert_eq!(summary.template_index_paths.len(), 1);
            assert!(summary.template_index_paths[0].ends_with("shell-templates.json"));
        }
        assert!(readiness.entries.iter().all(|entry| {
            entry.status == StudioValidationStatus::Pass
                && entry.validation_status == StudioValidationStatus::Pass
                && entry.failed_check_count == 0
                && entry.consumer_args.iter().any(|arg| arg == "--templates")
                && entry.export_bundle_id == format!("studio.export.{}", entry.graph_id)
                && entry.package_ids == vec!["package.synthetic".to_string()]
                && entry.module_ids == vec!["module.synthetic_provider".to_string()]
                && entry.package_count == entry.package_ids.len()
                && entry.module_count == entry.module_ids.len()
                && entry.operator_shell_count == entry.operator_shell_ids.len()
        }));
        assert!(readiness.entries.iter().any(|entry| {
            entry.graph_id == "studio.graph.phone"
                && entry.target_host_profile == "host_run.profile.mobile"
                && entry.handoff_kind == StudioShellHandoffKind::PhoneShell
                && entry.consumer_id == "rusty-studio-phone-shell"
                && entry.target_kind == StudioShellTargetKind::Phone
                && entry.operator_shell_ids == vec!["shell.synthetic.phone_operator".to_string()]
        }));
        assert!(readiness.entries.iter().any(|entry| {
            entry.graph_id == "studio.graph.quest"
                && entry.target_host_profile == "host_run.profile.headset"
                && entry.handoff_kind == StudioShellHandoffKind::QuestShell
                && entry.consumer_id == "rusty-studio-quest-shell"
                && entry.target_kind == StudioShellTargetKind::Quest
                && entry.operator_shell_ids == vec!["shell.synthetic.quest_operator".to_string()]
        }));
    }

    #[test]
    fn shell_handoff_readiness_reports_missing_bundles() {
        let root = temp_root("shell-handoff-readiness-missing");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("missing-selected-shells");

        let readiness = shell_handoff_readiness_for_project(&project, Some(&root), &bundle_root);

        assert_eq!(readiness.status, StudioValidationStatus::Fail);
        assert_eq!(readiness.graph_count, 3);
        assert_eq!(readiness.ready_count, 0);
        assert_eq!(readiness.failed_count, 3);
        assert_eq!(readiness.missing_bundle_count, 3);
        assert_eq!(readiness.entries.len(), 3);
        assert_eq!(readiness.target_summaries.len(), 3);
        assert!(readiness.target_summaries.iter().all(|summary| {
            summary.graph_count == 1
                && summary.ready_count == 0
                && summary.failed_count == 1
                && summary.missing_bundle_count == 1
                && summary
                    .issue_codes
                    .iter()
                    .any(|issue| issue == "studio.issue.shell_bundle_file_missing")
                && summary.bundle_dirs.len() == 1
                && summary.ready_bundle_dirs.is_empty()
                && summary.failed_bundle_dirs.len() == 1
                && summary.missing_bundle_dirs.len() == 1
                && summary.template_index_paths.len() == 1
                && summary.template_index_paths[0].ends_with("shell-templates.json")
        }));
        assert!(readiness.entries.iter().all(|entry| {
            entry.status == StudioValidationStatus::Fail
                && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
                && entry.export_bundle_id == format!("studio.export.{}", entry.graph_id)
                && entry.package_count == 1
                && entry.module_count == 1
                && entry.operator_shell_count == 1
                && entry.failed_check_count > 0
        }));
    }

    #[test]
    fn shell_handoff_manifest_archives_readiness_paths() {
        let root = temp_root("shell-handoff-manifest");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }

        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

        assert_eq!(manifest.schema_id, SHELL_HANDOFF_MANIFEST_SCHEMA);
        assert_eq!(
            manifest.manifest_id,
            "studio.shell_handoffs.studio.project.test"
        );
        assert_eq!(
            manifest.source_readiness_schema,
            SHELL_HANDOFF_READINESS_REPORT_SCHEMA
        );
        assert_eq!(manifest.status, StudioValidationStatus::Pass);
        assert_eq!(manifest.graph_count, 3);
        assert_eq!(manifest.ready_count, 3);
        assert_eq!(manifest.failed_count, 0);
        assert_eq!(manifest.missing_bundle_count, 0);
        assert_eq!(manifest.targets.len(), 3);
        assert_eq!(manifest.handoffs.len(), 3);
        assert_eq!(
            manifest.runtime_authority.command_session_authority,
            "rusty.manifold"
        );
        assert_eq!(
            manifest.runtime_authority.install_launch_evidence_authority,
            "rusty.hostess"
        );
        assert_eq!(
            manifest.runtime_authority.studio_role,
            "authoring.export_planning"
        );
        for target_kind in [
            StudioShellTargetKind::Desktop,
            StudioShellTargetKind::Phone,
            StudioShellTargetKind::Quest,
        ] {
            let target = manifest
                .targets
                .iter()
                .find(|target| target.target_kind == target_kind)
                .expect("target manifest row");
            assert_eq!(target.graph_count, 1);
            assert_eq!(target.ready_count, 1);
            assert_eq!(target.failed_count, 0);
            assert_eq!(target.missing_bundle_count, 0);
            assert_eq!(target.bundle_dirs.len(), 1);
            assert_eq!(target.ready_bundle_dirs.len(), 1);
            assert!(target.failed_bundle_dirs.is_empty());
            assert!(target.missing_bundle_dirs.is_empty());
            assert_eq!(target.template_index_paths.len(), 1);
            assert!(target.template_index_paths[0].ends_with("shell-templates.json"));
        }
        assert!(manifest.handoffs.iter().all(|handoff| {
            handoff.status == StudioValidationStatus::Pass
                && handoff.validation_status == StudioValidationStatus::Pass
                && handoff.failed_check_count == 0
                && handoff.consumer_args.iter().any(|arg| arg == "--templates")
                && handoff
                    .template_index_path
                    .ends_with("shell-templates.json")
                && handoff.runtime_authority.is_some()
        }));
    }

    #[test]
    fn shell_handoff_manifest_archives_missing_bundle_paths() {
        let root = temp_root("shell-handoff-manifest-missing");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("missing-selected-shells");

        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

        assert_eq!(manifest.schema_id, SHELL_HANDOFF_MANIFEST_SCHEMA);
        assert_eq!(manifest.status, StudioValidationStatus::Fail);
        assert_eq!(manifest.graph_count, 3);
        assert_eq!(manifest.ready_count, 0);
        assert_eq!(manifest.failed_count, 3);
        assert_eq!(manifest.missing_bundle_count, 3);
        assert_eq!(manifest.targets.len(), 3);
        assert_eq!(manifest.handoffs.len(), 3);
        assert!(manifest.targets.iter().all(|target| {
            target.ready_bundle_dirs.is_empty()
                && target.failed_bundle_dirs.len() == 1
                && target.missing_bundle_dirs.len() == 1
                && target
                    .issue_codes
                    .iter()
                    .any(|issue| issue == "studio.issue.shell_bundle_file_missing")
                && target.template_index_paths[0].ends_with("shell-templates.json")
        }));
        assert!(manifest.handoffs.iter().all(|handoff| {
            handoff.status == StudioValidationStatus::Fail
                && handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
                && handoff.runtime_authority.is_none()
                && handoff.failed_check_count > 0
        }));
    }

    #[test]
    fn shell_handoff_manifest_validation_accepts_archived_manifest() {
        let root = temp_root("shell-handoff-manifest-validation");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let manifest_path = root.join("shell-handoffs.json");
        save_json(&manifest_path, &manifest).expect("save shell handoff manifest");

        let loaded = load_shell_handoff_manifest(&manifest_path).expect("load handoff manifest");
        let validation = validate_shell_handoff_manifest(&loaded);

        assert_eq!(loaded, manifest);
        assert_eq!(
            validation.schema_id,
            SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA
        );
        assert_eq!(
            validation.manifest_id,
            "studio.shell_handoffs.studio.project.test"
        );
        assert_eq!(validation.status, StudioValidationStatus::Pass);
        assert!(validation
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
    }

    #[test]
    fn shell_handoff_manifest_validation_rejects_authority_mismatch() {
        let root = temp_root("shell-handoff-manifest-authority-mismatch");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        manifest.runtime_authority.command_session_authority = "rusty.studio".to_string();

        let validation = validate_shell_handoff_manifest(&manifest);

        assert_eq!(validation.status, StudioValidationStatus::Fail);
        assert!(validation.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.runtime_authority_mismatch")
        }));
    }

    #[test]
    fn shell_handoff_manifest_validation_rejects_target_count_mismatch() {
        let root = temp_root("shell-handoff-manifest-target-mismatch");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        manifest.targets[0].ready_count = 0;

        let validation = validate_shell_handoff_manifest(&manifest);

        assert_eq!(validation.status, StudioValidationStatus::Fail);
        assert!(validation.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.target_summary_count_mismatch")
        }));
    }

    #[test]
    fn shell_handoff_manifest_validation_rejects_missing_consumer_args() {
        let root = temp_root("shell-handoff-manifest-consumer-args");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        manifest.handoffs[0].consumer_args.clear();

        let validation = validate_shell_handoff_manifest(&manifest);

        assert_eq!(validation.status, StudioValidationStatus::Fail);
        assert!(validation.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.handoff_consumer_args_mismatch")
        }));
    }

    #[test]
    fn shell_handoff_intake_accepts_valid_manifest() {
        let root = temp_root("shell-handoff-intake");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

        let intake = shell_handoff_intake_for_manifest(&manifest);

        assert_eq!(intake.schema_id, SHELL_HANDOFF_INTAKE_REPORT_SCHEMA);
        assert_eq!(intake.status, StudioShellHandoffIntakeStatus::Accepted);
        assert_eq!(intake.issue_code, None);
        assert_eq!(intake.validation.status, StudioValidationStatus::Pass);
        assert_eq!(intake.accepted_count, 3);
        assert_eq!(intake.blocked_count, 0);
        assert_eq!(intake.target_summaries.len(), 3);
        assert_eq!(intake.entries.len(), 3);
        assert_eq!(intake.command_session_authority, "rusty.manifold");
        assert_eq!(intake.install_launch_evidence_authority, "rusty.hostess");
        assert_eq!(intake.studio_role, "authoring.export_planning");
        for target_kind in [
            StudioShellTargetKind::Desktop,
            StudioShellTargetKind::Phone,
            StudioShellTargetKind::Quest,
        ] {
            let summary = intake
                .target_summaries
                .iter()
                .find(|summary| summary.target_kind == target_kind)
                .expect("intake target summary");
            assert_eq!(summary.accepted_count, 1);
            assert_eq!(summary.blocked_count, 0);
            assert_eq!(summary.graph_ids.len(), 1);
            assert_eq!(summary.consumer_ids.len(), 1);
            assert_eq!(summary.bundle_dirs.len(), 1);
            assert_eq!(summary.template_index_paths.len(), 1);
        }
        assert!(intake.entries.iter().all(|entry| {
            entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner
                && entry.handoff_status == StudioValidationStatus::Pass
                && entry.issue_code.is_none()
                && entry.handoff_request_kind == "operator_shell_handoff"
                && entry.next_required_action == "stage_with_runtime_owner"
                && entry.command_session_authority == "rusty.manifold"
                && entry.install_launch_evidence_authority == "rusty.hostess"
                && entry.studio_role == "authoring.export_planning"
                && entry.consumer_args.iter().any(|arg| arg == "--templates")
        }));
    }

    #[test]
    fn shell_runbook_reports_non_executed_owner_routes() {
        let root = temp_root("shell-runbook");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }

        let runbook = shell_runbook_for_project(&project, Some(&root), &bundle_root);

        assert_eq!(runbook.schema_id, SHELL_RUNBOOK_REPORT_SCHEMA);
        assert_eq!(
            runbook.source_manifest_schema,
            SHELL_HANDOFF_MANIFEST_SCHEMA
        );
        assert_eq!(
            runbook.source_intake_schema,
            SHELL_HANDOFF_INTAKE_REPORT_SCHEMA
        );
        assert_eq!(runbook.status, StudioShellRunbookStatus::Ready);
        assert_eq!(runbook.issue_code, None);
        assert_eq!(runbook.ready_count, 3);
        assert_eq!(runbook.blocked_count, 0);
        assert_eq!(runbook.rejected_count, 0);
        assert_eq!(runbook.target_summaries.len(), 3);
        assert_eq!(
            runbook.prohibited_actions,
            vec![
                "install".to_string(),
                "launch".to_string(),
                "open_command_session".to_string(),
                "collect_device_evidence".to_string(),
            ]
        );
        for target_kind in [
            StudioShellTargetKind::Desktop,
            StudioShellTargetKind::Phone,
            StudioShellTargetKind::Quest,
        ] {
            let summary = runbook
                .target_summaries
                .iter()
                .find(|summary| summary.target_kind == target_kind)
                .expect("runbook target summary");
            assert_eq!(summary.ready_count, 1);
            assert_eq!(summary.blocked_count, 0);
            assert_eq!(summary.rejected_count, 0);
            assert_eq!(summary.responsible_owners, vec!["rusty.hostess"]);
            assert!(summary.runtime_route_kinds[0].ends_with("_operator_shell"));
            assert!(summary.issue_codes.is_empty());
        }

        let desktop = runbook
            .entries
            .iter()
            .find(|entry| entry.target_kind == StudioShellTargetKind::Desktop)
            .expect("desktop runbook row");
        assert_eq!(desktop.status, StudioShellRunbookStatus::Ready);
        assert_eq!(desktop.responsible_owner, "rusty.hostess");
        assert_eq!(desktop.command_session_authority, "rusty.manifold");
        assert_eq!(desktop.install_launch_evidence_authority, "rusty.hostess");
        assert_eq!(desktop.execution_policy, "not_executed.request_only");
        assert_eq!(desktop.runtime_route_kind, "desktop_operator_shell");
        assert_eq!(desktop.next_required_action, "stage_with_runtime_owner");
        assert_eq!(
            desktop.host_routes.install_route.as_deref(),
            Some("install.local_process")
        );
        assert_eq!(
            desktop.host_routes.launch_route.as_deref(),
            Some("launch.local_process")
        );
        assert_eq!(
            desktop.host_routes.command_bridge.as_deref(),
            Some("bridge.local_cli")
        );
        assert_eq!(
            desktop.host_routes.evidence_pull_route.as_deref(),
            Some("evidence.filesystem")
        );
        assert_eq!(
            desktop.cli_request[..5],
            ["cargo", "run", "-p", "rusty-studio-desktop-shell", "--"]
        );
        assert!(desktop
            .cli_request
            .iter()
            .any(|arg| arg.ends_with("shell-templates.json")));

        let phone = runbook
            .entries
            .iter()
            .find(|entry| entry.target_kind == StudioShellTargetKind::Phone)
            .expect("phone runbook row");
        assert_eq!(phone.consumer_id, "rusty-studio-phone-shell");
        assert_eq!(
            phone.host_routes.install_route.as_deref(),
            Some("install.android_package")
        );
        assert_eq!(
            phone.host_routes.evidence_pull_route.as_deref(),
            Some("evidence.adb_pull")
        );

        let quest = runbook
            .entries
            .iter()
            .find(|entry| entry.target_kind == StudioShellTargetKind::Quest)
            .expect("quest runbook row");
        assert_eq!(quest.consumer_id, "rusty-studio-quest-shell");
        assert_eq!(
            quest.host_routes.install_route.as_deref(),
            Some("install.android_package")
        );
        assert_eq!(
            quest.host_routes.launch_route.as_deref(),
            Some("launch.android_intent")
        );
    }

    #[test]
    fn shell_runbook_blocks_missing_bundle_requests_without_execution_args() {
        let root = temp_root("shell-runbook-missing");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("missing-selected-shells");

        let runbook = shell_runbook_for_project(&project, Some(&root), &bundle_root);

        assert_eq!(runbook.status, StudioShellRunbookStatus::Blocked);
        assert_eq!(
            runbook.issue_code.as_deref(),
            Some("studio.issue.shell_bundle_file_missing")
        );
        assert_eq!(runbook.ready_count, 0);
        assert_eq!(runbook.blocked_count, 3);
        assert_eq!(runbook.rejected_count, 0);
        assert!(runbook.entries.iter().all(|entry| {
            entry.status == StudioShellRunbookStatus::Blocked
                && entry.responsible_owner == "rusty.studio"
                && entry.cli_request.is_empty()
                && entry.route_status == StudioValidationStatus::Fail
                && entry.execution_policy == "not_executed.request_only"
        }));
    }

    #[test]
    fn shell_runbook_blocks_template_index_graph_mismatch() {
        let root = temp_root("shell-runbook-template-mismatch");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let index_path = bundle_root
            .join("studio.graph.test")
            .join("shell-templates.json");
        let mut index = load_shell_template_index(&index_path).expect("load template index");
        index.templates[0].graph_id = "studio.graph.unrelated".to_string();
        save_json(&index_path, &index).expect("save mismatched template index");

        let runbook = shell_runbook_for_manifest(&manifest);

        assert_eq!(runbook.status, StudioShellRunbookStatus::Blocked);
        assert_eq!(runbook.ready_count, 2);
        assert_eq!(runbook.blocked_count, 1);
        let entry = runbook
            .entries
            .iter()
            .find(|entry| entry.graph_id == "studio.graph.test")
            .expect("mismatched runbook entry");
        assert_eq!(entry.status, StudioShellRunbookStatus::Blocked);
        assert_eq!(entry.responsible_owner, "rusty.studio");
        assert!(entry.cli_request.is_empty());
        assert_eq!(entry.route_status, StudioValidationStatus::Fail);
        assert_eq!(
            entry.route_issue_code.as_deref(),
            Some("studio.issue.shell_runbook_template_missing")
        );
        assert_eq!(
            entry.issue_code.as_deref(),
            Some("studio.issue.shell_runbook_template_missing")
        );
    }

    #[test]
    fn shell_export_package_groups_descriptors_templates_and_runbook_rows() {
        let root = temp_root("shell-export-package");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }

        let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);

        assert_eq!(package.schema_id, SHELL_EXPORT_PACKAGE_REPORT_SCHEMA);
        assert_eq!(
            package.source_manifest_schema,
            SHELL_HANDOFF_MANIFEST_SCHEMA
        );
        assert_eq!(package.source_runbook_schema, SHELL_RUNBOOK_REPORT_SCHEMA);
        assert_eq!(
            package.package_id,
            "studio.shell_export_package.studio.project.test"
        );
        assert_eq!(package.status, StudioShellExportPackageStatus::Ready);
        assert_eq!(package.issue_code, None);
        assert_eq!(package.execution_policy, "not_executed.review_only");
        assert_eq!(package.review_owner, "rusty.hostess");
        assert_eq!(package.command_session_authority, "rusty.manifold");
        assert_eq!(package.install_launch_evidence_authority, "rusty.hostess");
        assert_eq!(package.studio_role, "authoring.export_planning");
        assert_eq!(package.ready_count, 3);
        assert_eq!(package.blocked_count, 0);
        assert_eq!(package.rejected_count, 0);
        assert_eq!(package.descriptor_count, 3);
        assert_eq!(package.template_manifest_count, 3);
        assert_eq!(package.runbook_entry_count, 3);
        assert_eq!(package.target_summaries.len(), 3);
        assert_eq!(
            package.prohibited_actions,
            vec![
                "install".to_string(),
                "launch".to_string(),
                "open_command_session".to_string(),
                "collect_device_evidence".to_string(),
            ]
        );
        for target_kind in [
            StudioShellTargetKind::Desktop,
            StudioShellTargetKind::Phone,
            StudioShellTargetKind::Quest,
        ] {
            let summary = package
                .target_summaries
                .iter()
                .find(|summary| summary.target_kind == target_kind)
                .expect("export package target summary");
            assert_eq!(summary.ready_count, 1);
            assert_eq!(summary.blocked_count, 0);
            assert_eq!(summary.rejected_count, 0);
            assert_eq!(summary.descriptor_count, 1);
            assert_eq!(summary.template_manifest_count, 1);
            assert_eq!(summary.responsible_owners, vec!["rusty.hostess"]);
            assert!(summary.issue_codes.is_empty());
        }

        let desktop = package
            .entries
            .iter()
            .find(|entry| entry.target_kind == StudioShellTargetKind::Desktop)
            .expect("desktop export package row");
        assert_eq!(desktop.status, StudioShellExportPackageStatus::Ready);
        assert_eq!(desktop.responsible_owner, "rusty.hostess");
        assert_eq!(desktop.execution_policy, "not_executed.review_only");
        assert_eq!(desktop.next_required_action, "review_with_runtime_owner");
        assert_eq!(desktop.runtime_route_kind, "desktop_operator_shell");
        assert_eq!(
            desktop.runbook_cli_request[..5],
            ["cargo", "run", "-p", "rusty-studio-desktop-shell", "--"]
        );
        let descriptor = desktop.descriptor.as_ref().expect("desktop descriptor ref");
        assert_eq!(
            descriptor.descriptor_id,
            "studio.shell_descriptor.studio.graph.test"
        );
        assert_eq!(descriptor.graph_id, desktop.graph_id);
        assert_eq!(descriptor.package_count, 1);
        assert_eq!(descriptor.module_count, 1);
        assert_eq!(descriptor.command_binding_count, 1);
        let template = desktop
            .template_manifest
            .as_ref()
            .expect("desktop template ref");
        assert_eq!(
            template.template_id,
            "studio.shell_template.studio.graph.test"
        );
        assert_eq!(template.graph_id, desktop.graph_id);
        assert_eq!(
            template.host_routes.install_route.as_deref(),
            Some("install.local_process")
        );
        assert_eq!(
            template.runtime_authority.command_session_authority,
            "rusty.manifold"
        );

        let quest = package
            .entries
            .iter()
            .find(|entry| entry.target_kind == StudioShellTargetKind::Quest)
            .expect("quest export package row");
        let quest_template = quest
            .template_manifest
            .as_ref()
            .expect("quest template ref");
        assert_eq!(quest.consumer_id, "rusty-studio-quest-shell");
        assert_eq!(
            quest_template.host_routes.command_bridge.as_deref(),
            Some("bridge.adb_intent_file")
        );
        assert_eq!(
            quest.host_routes.evidence_pull_route.as_deref(),
            Some("evidence.adb_pull")
        );
    }

    #[test]
    fn shell_export_package_blocks_missing_bundle_without_descriptors() {
        let root = temp_root("shell-export-package-missing");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("missing-selected-shells");

        let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);

        assert_eq!(package.status, StudioShellExportPackageStatus::Blocked);
        assert_eq!(
            package.issue_code.as_deref(),
            Some("studio.issue.shell_bundle_file_missing")
        );
        assert_eq!(package.ready_count, 0);
        assert_eq!(package.blocked_count, 3);
        assert_eq!(package.rejected_count, 0);
        assert_eq!(package.descriptor_count, 0);
        assert_eq!(package.template_manifest_count, 0);
        assert_eq!(package.runbook_entry_count, 3);
        assert!(package.entries.iter().all(|entry| {
            entry.status == StudioShellExportPackageStatus::Blocked
                && entry.responsible_owner == "rusty.studio"
                && entry.descriptor.is_none()
                && entry.template_manifest.is_none()
                && entry.runbook_cli_request.is_empty()
                && entry.execution_policy == "not_executed.review_only"
        }));
    }

    #[test]
    fn shell_export_package_blocks_one_damaged_descriptor_from_valid_manifest() {
        let root = temp_root("shell-export-package-damaged-descriptor");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let runbook = shell_runbook_for_manifest(&manifest);
        assert_eq!(runbook.status, StudioShellRunbookStatus::Ready);

        std::fs::remove_file(
            bundle_root
                .join("studio.graph.phone")
                .join("descriptors/studio.graph.phone.shell-descriptor.json"),
        )
        .expect("remove phone descriptor");

        let package = shell_export_package_for_manifest(&manifest);

        assert_eq!(package.status, StudioShellExportPackageStatus::Blocked);
        assert_eq!(
            package.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_descriptor_load_failed")
        );
        assert_eq!(package.ready_count, 2);
        assert_eq!(package.blocked_count, 1);
        assert_eq!(package.rejected_count, 0);
        assert_eq!(package.descriptor_count, 2);
        assert_eq!(package.template_manifest_count, 3);
        assert_eq!(package.runbook_entry_count, 3);

        let phone = package
            .entries
            .iter()
            .find(|entry| entry.graph_id == "studio.graph.phone")
            .expect("phone export package row");
        assert_eq!(phone.status, StudioShellExportPackageStatus::Blocked);
        assert_eq!(phone.responsible_owner, "rusty.studio");
        assert_eq!(
            phone.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_descriptor_load_failed")
        );
        assert!(phone.descriptor.is_none());
        assert!(phone.template_manifest.is_some());
        assert!(phone.runbook_cli_request.is_empty());
        assert_eq!(
            phone.host_routes.command_bridge.as_deref(),
            Some("bridge.adb_intent_file")
        );

        for target_kind in [StudioShellTargetKind::Desktop, StudioShellTargetKind::Quest] {
            let entry = package
                .entries
                .iter()
                .find(|entry| entry.target_kind == target_kind)
                .expect("undamaged export package row");
            assert_eq!(entry.status, StudioShellExportPackageStatus::Ready);
            assert_eq!(entry.responsible_owner, "rusty.hostess");
            assert!(entry.descriptor.is_some());
            assert!(entry.template_manifest.is_some());
            assert!(!entry.runbook_cli_request.is_empty());
        }

        let phone_summary = package
            .target_summaries
            .iter()
            .find(|summary| summary.target_kind == StudioShellTargetKind::Phone)
            .expect("phone summary");
        assert_eq!(phone_summary.ready_count, 0);
        assert_eq!(phone_summary.blocked_count, 1);
        assert_eq!(phone_summary.descriptor_count, 0);
        assert_eq!(phone_summary.template_manifest_count, 1);
        assert_eq!(
            phone_summary.issue_codes,
            vec!["studio.issue.shell_export_package_descriptor_load_failed"]
        );
    }

    #[test]
    fn shell_export_package_blocks_one_damaged_template_from_valid_manifest() {
        let root = temp_root("shell-export-package-damaged-template");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let runbook = shell_runbook_for_manifest(&manifest);
        assert_eq!(runbook.status, StudioShellRunbookStatus::Ready);

        std::fs::remove_file(
            bundle_root
                .join("studio.graph.phone")
                .join("shells/phone/studio.graph.phone.shell-template.json"),
        )
        .expect("remove phone template manifest");

        let package = shell_export_package_for_manifest(&manifest);

        assert_eq!(package.status, StudioShellExportPackageStatus::Blocked);
        assert_eq!(
            package.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_template_load_failed")
        );
        assert_eq!(package.ready_count, 2);
        assert_eq!(package.blocked_count, 1);
        assert_eq!(package.rejected_count, 0);
        assert_eq!(package.descriptor_count, 3);
        assert_eq!(package.template_manifest_count, 2);
        assert_eq!(package.runbook_entry_count, 3);

        let phone = package
            .entries
            .iter()
            .find(|entry| entry.graph_id == "studio.graph.phone")
            .expect("phone export package row");
        assert_eq!(phone.status, StudioShellExportPackageStatus::Blocked);
        assert_eq!(phone.responsible_owner, "rusty.studio");
        assert_eq!(
            phone.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_template_load_failed")
        );
        assert!(phone.descriptor.is_some());
        assert!(phone.template_manifest.is_none());
        assert!(phone.runbook_cli_request.is_empty());
        assert_eq!(phone.host_routes, empty_shell_host_routes());

        for target_kind in [StudioShellTargetKind::Desktop, StudioShellTargetKind::Quest] {
            let entry = package
                .entries
                .iter()
                .find(|entry| entry.target_kind == target_kind)
                .expect("undamaged export package row");
            assert_eq!(entry.status, StudioShellExportPackageStatus::Ready);
            assert_eq!(entry.responsible_owner, "rusty.hostess");
            assert!(entry.descriptor.is_some());
            assert!(entry.template_manifest.is_some());
            assert!(!entry.runbook_cli_request.is_empty());
        }

        let phone_summary = package
            .target_summaries
            .iter()
            .find(|summary| summary.target_kind == StudioShellTargetKind::Phone)
            .expect("phone summary");
        assert_eq!(phone_summary.ready_count, 0);
        assert_eq!(phone_summary.blocked_count, 1);
        assert_eq!(phone_summary.descriptor_count, 1);
        assert_eq!(phone_summary.template_manifest_count, 0);
        assert_eq!(
            phone_summary.issue_codes,
            vec!["studio.issue.shell_export_package_template_load_failed"]
        );
    }

    #[test]
    fn shell_export_package_comparison_reports_unchanged_ready_packages() {
        let root = temp_root("shell-export-package-comparison");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);

        let comparison = compare_shell_export_packages(&package, &package);

        assert_eq!(comparison.schema_id, SHELL_EXPORT_PACKAGE_COMPARISON_SCHEMA);
        assert_eq!(
            comparison.baseline_schema,
            SHELL_EXPORT_PACKAGE_REPORT_SCHEMA
        );
        assert_eq!(
            comparison.status,
            StudioShellExportPackageComparisonStatus::Unchanged
        );
        assert_eq!(comparison.issue_code, None);
        assert_eq!(comparison.ready_delta, 0);
        assert_eq!(comparison.blocked_delta, 0);
        assert_eq!(comparison.rejected_delta, 0);
        assert_eq!(comparison.descriptor_delta, 0);
        assert_eq!(comparison.template_manifest_delta, 0);
        assert_eq!(comparison.runbook_entry_delta, 0);
        assert_eq!(comparison.entries.len(), 3);
        assert!(comparison.entries.iter().all(|entry| {
            entry.change == StudioShellExportPackageComparisonChange::Unchanged
                && entry.baseline_descriptor_present
                && entry.candidate_descriptor_present
                && entry.baseline_template_manifest_present
                && entry.candidate_template_manifest_present
                && entry.baseline_runbook_cli_request_present
                && entry.candidate_runbook_cli_request_present
        }));
        assert!(comparison
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
    }

    #[test]
    fn shell_export_package_comparison_reports_regressed_damaged_template() {
        let root = temp_root("shell-export-package-comparison-damaged-template");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let baseline = shell_export_package_for_manifest(&manifest);
        assert_eq!(baseline.status, StudioShellExportPackageStatus::Ready);

        std::fs::remove_file(
            bundle_root
                .join("studio.graph.phone")
                .join("shells/phone/studio.graph.phone.shell-template.json"),
        )
        .expect("remove phone template manifest");
        let candidate = shell_export_package_for_manifest(&manifest);

        let comparison = compare_shell_export_packages(&baseline, &candidate);

        assert_eq!(
            comparison.status,
            StudioShellExportPackageComparisonStatus::Regressed
        );
        assert_eq!(
            comparison.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_template_load_failed")
        );
        assert_eq!(comparison.ready_delta, -1);
        assert_eq!(comparison.blocked_delta, 1);
        assert_eq!(comparison.rejected_delta, 0);
        assert_eq!(comparison.descriptor_delta, 0);
        assert_eq!(comparison.template_manifest_delta, -1);
        assert_eq!(comparison.runbook_entry_delta, 0);
        let phone = comparison
            .entries
            .iter()
            .find(|entry| entry.graph_id == "studio.graph.phone")
            .expect("phone comparison entry");
        assert_eq!(
            phone.change,
            StudioShellExportPackageComparisonChange::Regressed
        );
        assert_eq!(phone.score_delta, -1);
        assert!(phone.baseline_descriptor_present);
        assert!(phone.candidate_descriptor_present);
        assert!(phone.baseline_template_manifest_present);
        assert!(!phone.candidate_template_manifest_present);
        assert!(phone.baseline_runbook_cli_request_present);
        assert!(!phone.candidate_runbook_cli_request_present);
        assert_eq!(
            phone.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_template_load_failed")
        );
    }

    #[test]
    fn shell_export_package_comparison_rejects_mismatched_projects() {
        let root = temp_root("shell-export-package-comparison-mismatch");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let baseline = shell_export_package_for_project(&project, Some(&root), &bundle_root);
        let mut candidate = baseline.clone();
        candidate.project_id = "studio.project.other".to_string();

        let comparison = compare_shell_export_packages(&baseline, &candidate);

        assert_eq!(
            comparison.status,
            StudioShellExportPackageComparisonStatus::Incomparable
        );
        assert_eq!(
            comparison.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_project_mismatch")
        );
        assert!(comparison.entries.is_empty());
        assert!(comparison.checks.iter().any(|check| {
            check.check_id == "studio.check.shell_export_package_comparison.project_id"
                && check.status == StudioValidationStatus::Fail
        }));
    }

    #[test]
    fn shell_export_package_baseline_index_lists_named_review_slots() {
        let root = temp_root("shell-export-package-baseline-index");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let ready_bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let manifest =
            shell_handoff_manifest_for_project(&project, Some(&root), &ready_bundle_root);
        let ready_package = shell_export_package_for_manifest(&manifest);
        let mut blocked_package = ready_package.clone();
        blocked_package.status = StudioShellExportPackageStatus::Blocked;
        blocked_package.issue_code =
            Some("studio.issue.shell_export_package_template_load_failed".to_string());
        blocked_package.ready_count = 2;
        blocked_package.blocked_count = 1;
        blocked_package.template_manifest_count = 2;

        let ready_package_path = root.join("shell-export-package-ready.json");
        let blocked_package_path = root.join("shell-export-package-blocked.json");
        let ready_baseline_path = root.join("shell-export-package-baseline-ready.json");
        let blocked_baseline_path = root.join("shell-export-package-baseline-blocked.json");
        let ready_baseline = shell_export_package_baseline_manifest_for_report(
            &ready_package,
            &ready_package_path,
            Some("synthetic-ready-package"),
            Some("Synthetic ready export package baseline"),
        );
        let blocked_baseline = shell_export_package_baseline_manifest_for_report(
            &blocked_package,
            &blocked_package_path,
            Some("synthetic-blocked-package"),
            Some("Synthetic blocked export package baseline"),
        );

        let index = shell_export_package_baseline_index_for_manifests(
            vec![
                (ready_baseline, Some(ready_baseline_path.clone())),
                (blocked_baseline, Some(blocked_baseline_path)),
            ],
            Some("synthetic-ready-package"),
        );
        let selection = summarize_shell_export_package_baseline_index_selection(
            &index,
            Some(&root.join("shell-export-package-baselines.json")),
            None,
        );

        assert_eq!(index.schema_id, SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA);
        assert_eq!(
            index.default_baseline_id.as_deref(),
            Some("synthetic-ready-package")
        );
        assert_eq!(index.baseline_count, 2);
        assert_eq!(index.ready_baseline_count, 1);
        assert_eq!(index.blocked_baseline_count, 1);
        assert_eq!(index.rejected_baseline_count, 0);
        assert_eq!(index.project_ids, vec!["studio.project.test"]);
        assert_eq!(
            index.package_ids,
            vec!["studio.shell_export_package.studio.project.test"]
        );
        let selected = select_shell_export_package_baseline_index_entry(&index, None)
            .expect("selected export package baseline");
        assert_eq!(selected.baseline_id, "synthetic-ready-package");
        assert_eq!(
            selected.baseline_manifest_path.as_deref(),
            Some(ready_baseline_path.display().to_string().as_str())
        );
        assert_eq!(
            selected.package_path,
            ready_package_path.display().to_string()
        );
        assert_eq!(selected.status, StudioShellExportPackageStatus::Ready);
        assert_eq!(selected.ready_count, 3);
        assert_eq!(selected.target_count, 3);
        assert_eq!(
            selection.schema_id,
            SHELL_EXPORT_PACKAGE_BASELINE_SELECTION_SCHEMA
        );
        assert_eq!(
            selection.status,
            StudioShellExportPackageBaselineSelectionStatus::Selected
        );
        assert_eq!(
            selection.selected_baseline_id.as_deref(),
            Some("synthetic-ready-package")
        );
        assert!(
            selection
                .entries
                .iter()
                .find(|entry| entry.baseline_id == "synthetic-ready-package")
                .expect("ready selection entry")
                .selected
        );
    }

    #[test]
    fn shell_export_package_comparison_carries_baseline_index_selection() {
        let root = temp_root("shell-export-package-comparison-baseline-index");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);
        let package_path = root.join("shell-export-package.json");
        let baseline_path = root.join("shell-export-package-baseline.json");
        let index_path = root.join("shell-export-package-baselines.json");
        let baseline_identity = shell_export_package_baseline_manifest_for_report(
            &package,
            &package_path,
            Some("synthetic-ready-package"),
            Some("Synthetic ready export package baseline"),
        );
        let index = shell_export_package_baseline_index_for_manifests(
            vec![(baseline_identity.clone(), Some(baseline_path.clone()))],
            Some("synthetic-ready-package"),
        );
        let selected_entry = select_shell_export_package_baseline_index_entry(&index, None)
            .expect("selected export package baseline index entry");

        let comparison = compare_shell_export_packages_against_baseline_index_entry(
            &index,
            Some(&index_path),
            selected_entry,
            Some(&baseline_path),
            &baseline_identity,
            &package,
            &package,
        );

        assert_eq!(
            comparison.baseline_identity_schema.as_deref(),
            Some(SHELL_EXPORT_PACKAGE_BASELINE_MANIFEST_SCHEMA)
        );
        assert_eq!(
            comparison.baseline_id.as_deref(),
            Some("synthetic-ready-package")
        );
        assert_eq!(
            comparison.baseline_package_path.as_deref(),
            Some(package_path.display().to_string().as_str())
        );
        assert_eq!(
            comparison.baseline_index_schema.as_deref(),
            Some(SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA)
        );
        assert_eq!(
            comparison.baseline_index_path.as_deref(),
            Some(index_path.display().to_string().as_str())
        );
        assert_eq!(
            comparison.baseline_index_selected_baseline_id.as_deref(),
            Some("synthetic-ready-package")
        );
        assert_eq!(
            comparison.status,
            StudioShellExportPackageComparisonStatus::Unchanged
        );
        assert!(comparison
            .checks
            .iter()
            .filter(|check| check.check_id.contains("baseline_identity"))
            .all(|check| check.status == StudioValidationStatus::Pass));
        assert!(comparison
            .checks
            .iter()
            .filter(|check| check.check_id.contains("baseline_index"))
            .all(|check| check.status == StudioValidationStatus::Pass));
        assert_eq!(comparison.entries.len(), 3);
    }

    #[test]
    fn shell_export_package_comparison_rejects_stale_baseline_identity() {
        let root = temp_root("shell-export-package-comparison-stale-baseline");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);
        let package_path = root.join("shell-export-package.json");
        let mut baseline_identity =
            shell_export_package_baseline_manifest_for_report(&package, &package_path, None, None);
        baseline_identity.project_revision += 1;

        let comparison = compare_shell_export_packages_against_baseline_manifest(
            &baseline_identity,
            &package,
            &package,
        );

        assert_eq!(
            comparison.status,
            StudioShellExportPackageComparisonStatus::Incomparable
        );
        assert_eq!(
            comparison.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_baseline_identity_mismatch")
        );
        assert!(comparison.entries.is_empty());
        assert!(comparison.checks.iter().any(|check| {
            check.check_id
                == "studio.check.shell_export_package_comparison.baseline_identity_project"
                && check.status == StudioValidationStatus::Fail
        }));
    }

    #[test]
    fn shell_handoff_intake_rejects_invalid_manifest() {
        let root = temp_root("shell-handoff-intake-invalid");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        manifest.runtime_authority.command_session_authority = "rusty.studio".to_string();

        let intake = shell_handoff_intake_for_manifest(&manifest);

        assert_eq!(intake.schema_id, SHELL_HANDOFF_INTAKE_REPORT_SCHEMA);
        assert_eq!(intake.status, StudioShellHandoffIntakeStatus::Rejected);
        assert_eq!(
            intake.issue_code.as_deref(),
            Some("studio.issue.runtime_authority_mismatch")
        );
        assert_eq!(intake.validation.status, StudioValidationStatus::Fail);
        assert_eq!(intake.accepted_count, 0);
        assert_eq!(intake.blocked_count, 0);
        assert!(intake.target_summaries.is_empty());
        assert!(intake.entries.is_empty());
        assert_eq!(intake.command_session_authority, "rusty.manifold");
        assert_eq!(intake.install_launch_evidence_authority, "rusty.hostess");
    }

    #[test]
    fn shell_handoff_intake_blocks_missing_bundles_after_valid_manifest_shape() {
        let root = temp_root("shell-handoff-intake-missing");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("missing-selected-shells");
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

        let intake = shell_handoff_intake_for_manifest(&manifest);

        assert_eq!(intake.status, StudioShellHandoffIntakeStatus::Accepted);
        assert_eq!(intake.validation.status, StudioValidationStatus::Pass);
        assert_eq!(intake.accepted_count, 0);
        assert_eq!(intake.blocked_count, 3);
        assert_eq!(intake.target_summaries.len(), 3);
        assert_eq!(intake.entries.len(), 3);
        assert!(intake.target_summaries.iter().all(|summary| {
            summary.accepted_count == 0
                && summary.blocked_count == 1
                && summary.graph_ids.len() == 1
                && summary.consumer_ids.len() == 1
        }));
        assert!(intake.entries.iter().all(|entry| {
            entry.decision == StudioShellHandoffIntakeDecision::BlockedByHandoffIssue
                && entry.handoff_status == StudioValidationStatus::Fail
                && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
                && entry.next_required_action == "repair_export_bundle"
                && entry.consumer_args.is_empty()
        }));
    }

    #[test]
    fn shell_handoff_acceptance_checklist_reports_ready_entries() {
        let root = temp_root("shell-handoff-acceptance");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let intake = shell_handoff_intake_for_manifest(&manifest);

        let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

        assert_eq!(
            checklist.schema_id,
            SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA
        );
        assert_eq!(
            checklist.source_intake_schema,
            SHELL_HANDOFF_INTAKE_REPORT_SCHEMA
        );
        assert_eq!(checklist.status, StudioShellHandoffAcceptanceStatus::Ready);
        assert_eq!(checklist.issue_code, None);
        assert_eq!(checklist.ready_count, 3);
        assert_eq!(checklist.blocked_count, 0);
        assert_eq!(checklist.rejected_count, 0);
        assert_eq!(checklist.entries.len(), 3);
        assert_eq!(
            checklist.prohibited_actions,
            vec![
                "install".to_string(),
                "launch".to_string(),
                "open_command_session".to_string(),
                "collect_device_evidence".to_string()
            ]
        );
        assert!(checklist
            .intake_checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
        assert!(checklist.entries.iter().all(|entry| {
            entry.status == StudioShellHandoffAcceptanceStatus::Ready
                && entry.issue_code.is_none()
                && entry.next_required_action == "stage_with_runtime_owner"
                && entry.command_session_authority == "rusty.manifold"
                && entry.install_launch_evidence_authority == "rusty.hostess"
                && entry.studio_role == "authoring.export_planning"
                && entry
                    .checks
                    .iter()
                    .all(|check| check.status == StudioValidationStatus::Pass)
        }));
    }

    #[test]
    fn shell_handoff_acceptance_snapshot_reports_ready_project() {
        let root = temp_root("shell-handoff-acceptance-snapshot");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let intake = shell_handoff_intake_for_manifest(&manifest);
        let expected = shell_handoff_acceptance_checklist_for_intake(&intake);

        let checklist =
            shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);

        assert_eq!(checklist, expected);
        assert_eq!(checklist.status, StudioShellHandoffAcceptanceStatus::Ready);
        assert_eq!(checklist.ready_count, 3);
        assert_eq!(checklist.blocked_count, 0);
        assert_eq!(checklist.rejected_count, 0);
        assert_eq!(checklist.entries.len(), 3);
    }

    #[test]
    fn shell_handoff_acceptance_snapshot_blocks_missing_bundles() {
        let root = temp_root("shell-handoff-acceptance-snapshot-missing");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("missing-selected-shells");

        let checklist =
            shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);

        assert_eq!(
            checklist.status,
            StudioShellHandoffAcceptanceStatus::Blocked
        );
        assert_eq!(
            checklist.issue_code.as_deref(),
            Some("studio.issue.shell_bundle_file_missing")
        );
        assert_eq!(checklist.ready_count, 0);
        assert_eq!(checklist.blocked_count, 3);
        assert_eq!(checklist.rejected_count, 0);
        assert_eq!(checklist.entries.len(), 3);
        assert!(checklist.entries.iter().all(|entry| {
            entry.status == StudioShellHandoffAcceptanceStatus::Blocked
                && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
        }));
    }

    #[test]
    fn shell_handoff_acceptance_summary_reports_baseline_metadata() {
        let root = temp_root("shell-handoff-acceptance-summary");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let checklist =
            shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
        let checklist_path = root.join("shell-handoff-acceptance-checklist.json");

        let summary =
            summarize_shell_handoff_acceptance_checklist(&checklist, Some(&checklist_path));

        assert_eq!(summary.schema_id, SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA);
        assert_eq!(
            summary.checklist_schema,
            SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA
        );
        let checklist_path_text = checklist_path.display().to_string();
        assert_eq!(
            summary.checklist_path.as_deref(),
            Some(checklist_path_text.as_str())
        );
        assert_eq!(summary.manifest_id, checklist.manifest_id);
        assert_eq!(summary.project_id, "studio.project.test");
        assert_eq!(summary.project_revision, 1);
        assert_eq!(summary.status, StudioShellHandoffAcceptanceStatus::Ready);
        assert_eq!(summary.issue_code, None);
        assert_eq!(summary.ready_count, 3);
        assert_eq!(summary.blocked_count, 0);
        assert_eq!(summary.rejected_count, 0);
        assert_eq!(summary.entry_count, 3);
        assert_eq!(summary.failed_intake_check_count, 0);
        assert_eq!(summary.targets.len(), 3);
        assert!(summary.targets.iter().all(|target| {
            target.graph_count == 1
                && target.ready_count == 1
                && target.blocked_count == 0
                && target.rejected_count == 0
                && target.graph_ids.len() == 1
                && target.consumer_ids.len() == 1
                && target.route_kinds.len() == 1
                && target.issue_codes.is_empty()
        }));
    }

    #[test]
    fn shell_handoff_acceptance_summary_reports_blocked_target_metadata() {
        let root = temp_root("shell-handoff-acceptance-summary-missing");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let checklist = shell_handoff_acceptance_checklist_for_project(
            &project,
            Some(&root),
            &root.join("missing-selected-shells"),
        );

        let summary = summarize_shell_handoff_acceptance_checklist(&checklist, None);

        assert_eq!(summary.checklist_path, None);
        assert_eq!(summary.status, StudioShellHandoffAcceptanceStatus::Blocked);
        assert_eq!(
            summary.issue_code.as_deref(),
            Some("studio.issue.shell_bundle_file_missing")
        );
        assert_eq!(summary.ready_count, 0);
        assert_eq!(summary.blocked_count, 3);
        assert_eq!(summary.rejected_count, 0);
        assert_eq!(summary.entry_count, 3);
        assert_eq!(summary.targets.len(), 3);
        assert!(summary.targets.iter().all(|target| {
            target.ready_count == 0
                && target.blocked_count == 1
                && target.rejected_count == 0
                && target.issue_codes == vec!["studio.issue.shell_bundle_file_missing".to_string()]
        }));
    }

    #[test]
    fn shell_handoff_acceptance_baseline_manifest_names_saved_checklist() {
        let root = temp_root("shell-handoff-acceptance-baseline-manifest");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let checklist =
            shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
        let checklist_path = root.join("shell-handoff-acceptance-checklist.json");

        let baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &checklist,
            &checklist_path,
            None,
            None,
        );

        assert_eq!(
            baseline.schema_id,
            SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA
        );
        assert_eq!(baseline.baseline_id, "studio.project.test.rev1.ready");
        assert_eq!(
            baseline.label,
            "studio.project.test revision 1 ready acceptance baseline"
        );
        let checklist_path_text = checklist_path.display().to_string();
        assert_eq!(baseline.checklist_path, checklist_path_text);
        assert_eq!(
            baseline.summary.schema_id,
            SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA
        );
        assert_eq!(
            baseline.summary.checklist_schema,
            SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA
        );
        assert_eq!(baseline.summary.project_id, "studio.project.test");
        assert_eq!(baseline.summary.project_revision, 1);
        assert_eq!(
            baseline.summary.status,
            StudioShellHandoffAcceptanceStatus::Ready
        );
        assert_eq!(baseline.summary.ready_count, 3);
        assert_eq!(baseline.summary.blocked_count, 0);
        assert_eq!(baseline.summary.rejected_count, 0);
        assert_eq!(baseline.summary.entry_count, 3);
        assert_eq!(baseline.summary.targets.len(), 3);
    }

    #[test]
    fn shell_handoff_acceptance_baseline_manifest_accepts_custom_identity() {
        let root = temp_root("shell-handoff-acceptance-baseline-manifest-custom");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let checklist = shell_handoff_acceptance_checklist_for_project(
            &project,
            Some(&root),
            &root.join("missing-selected-shells"),
        );
        let checklist_path = root.join("blocked-checklist.json");

        let baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &checklist,
            &checklist_path,
            Some("preflight-blocked"),
            Some("Preflight blocked baseline"),
        );

        assert_eq!(baseline.baseline_id, "preflight-blocked");
        assert_eq!(baseline.label, "Preflight blocked baseline");
        assert_eq!(
            baseline.summary.status,
            StudioShellHandoffAcceptanceStatus::Blocked
        );
        assert_eq!(
            baseline.summary.issue_code.as_deref(),
            Some("studio.issue.shell_bundle_file_missing")
        );
    }

    #[test]
    fn shell_handoff_acceptance_baseline_index_lists_named_baselines() {
        let root = temp_root("shell-handoff-acceptance-baseline-index");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let ready_bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let ready_checklist = shell_handoff_acceptance_checklist_for_project(
            &project,
            Some(&root),
            &ready_bundle_root,
        );
        let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
            &project,
            Some(&root),
            &root.join("missing-selected-shells"),
        );
        let ready_checklist_path = root.join("ready-checklist.json");
        let blocked_checklist_path = root.join("blocked-checklist.json");
        let ready_manifest_path = root.join("ready-baseline.json");
        let blocked_manifest_path = root.join("blocked-baseline.json");
        let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &ready_checklist,
            &ready_checklist_path,
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );
        let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &blocked_checklist,
            &blocked_checklist_path,
            Some("synthetic-blocked"),
            Some("Synthetic blocked acceptance baseline"),
        );

        let index = shell_handoff_acceptance_baseline_index_for_manifests(
            vec![
                (ready_baseline, Some(ready_manifest_path.clone())),
                (blocked_baseline, Some(blocked_manifest_path.clone())),
            ],
            Some("synthetic-ready"),
        );

        assert_eq!(
            index.schema_id,
            SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA
        );
        assert_eq!(index.project_ids, vec!["studio.project.test"]);
        assert_eq!(
            index.manifest_ids,
            vec!["studio.shell_handoffs.studio.project.test"]
        );
        assert_eq!(
            index.default_baseline_id.as_deref(),
            Some("synthetic-ready")
        );
        assert_eq!(index.baseline_count, 2);
        assert_eq!(index.ready_baseline_count, 1);
        assert_eq!(index.blocked_baseline_count, 1);
        assert_eq!(index.rejected_baseline_count, 0);
        assert_eq!(index.entries.len(), 2);
        assert_eq!(index.entries[0].baseline_id, "synthetic-blocked");
        assert_eq!(
            index.entries[0].baseline_manifest_path.as_deref(),
            Some(blocked_manifest_path.display().to_string().as_str())
        );
        assert_eq!(
            index.entries[0].checklist_path,
            blocked_checklist_path.display().to_string()
        );
        assert_eq!(
            index.entries[0].status,
            StudioShellHandoffAcceptanceStatus::Blocked
        );
        assert_eq!(index.entries[0].ready_count, 0);
        assert_eq!(index.entries[0].blocked_count, 3);
        assert_eq!(index.entries[0].target_count, 3);
        assert_eq!(index.entries[1].baseline_id, "synthetic-ready");
        assert_eq!(
            index.entries[1].baseline_manifest_path.as_deref(),
            Some(ready_manifest_path.display().to_string().as_str())
        );
        assert_eq!(index.entries[1].ready_count, 3);
        assert_eq!(index.entries[1].blocked_count, 0);
        assert_eq!(
            select_shell_handoff_acceptance_baseline_index_entry(&index, None)
                .map(|entry| entry.baseline_id.as_str()),
            Some("synthetic-ready")
        );
        assert_eq!(
            select_shell_handoff_acceptance_baseline_index_entry(&index, Some("synthetic-blocked"))
                .map(|entry| entry.baseline_id.as_str()),
            Some("synthetic-blocked")
        );
        assert!(
            select_shell_handoff_acceptance_baseline_index_entry(&index, Some("missing")).is_none()
        );
    }

    #[test]
    fn shell_handoff_acceptance_baseline_index_appends_named_baseline() {
        let root = temp_root("shell-handoff-acceptance-baseline-index-append");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let ready_bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let ready_checklist = shell_handoff_acceptance_checklist_for_project(
            &project,
            Some(&root),
            &ready_bundle_root,
        );
        let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
            &project,
            Some(&root),
            &root.join("missing-selected-shells"),
        );
        let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &ready_checklist,
            &root.join("ready-checklist.json"),
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );
        let blocked_manifest_path = root.join("blocked-baseline.json");
        let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &blocked_checklist,
            &root.join("blocked-checklist.json"),
            Some("synthetic-blocked"),
            Some("Synthetic blocked acceptance baseline"),
        );
        let index = shell_handoff_acceptance_baseline_index_for_manifests(
            vec![(ready_baseline, Some(root.join("ready-baseline.json")))],
            Some("synthetic-ready"),
        );

        let appended = append_shell_handoff_acceptance_baseline_index_manifests(
            &index,
            vec![(blocked_baseline, Some(blocked_manifest_path.clone()))],
            Some("synthetic-blocked"),
        );

        assert_eq!(
            appended.default_baseline_id.as_deref(),
            Some("synthetic-blocked")
        );
        assert_eq!(appended.baseline_count, 2);
        assert_eq!(appended.ready_baseline_count, 1);
        assert_eq!(appended.blocked_baseline_count, 1);
        assert_eq!(appended.entries[0].baseline_id, "synthetic-blocked");
        assert_eq!(
            appended.entries[0].baseline_manifest_path.as_deref(),
            Some(blocked_manifest_path.display().to_string().as_str())
        );
        assert_eq!(
            appended.entries[0].status,
            StudioShellHandoffAcceptanceStatus::Blocked
        );
        assert_eq!(appended.entries[1].baseline_id, "synthetic-ready");
        assert_eq!(
            select_shell_handoff_acceptance_baseline_index_entry(&appended, None)
                .map(|entry| entry.baseline_id.as_str()),
            Some("synthetic-blocked")
        );
    }

    #[test]
    fn shell_handoff_acceptance_baseline_index_promotes_existing_default() {
        let root = temp_root("shell-handoff-acceptance-baseline-index-promote");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let ready_bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let ready_checklist = shell_handoff_acceptance_checklist_for_project(
            &project,
            Some(&root),
            &ready_bundle_root,
        );
        let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
            &project,
            Some(&root),
            &root.join("missing-selected-shells"),
        );
        let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &ready_checklist,
            &root.join("ready-checklist.json"),
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );
        let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &blocked_checklist,
            &root.join("blocked-checklist.json"),
            Some("synthetic-blocked"),
            Some("Synthetic blocked acceptance baseline"),
        );
        let index = shell_handoff_acceptance_baseline_index_for_manifests(
            vec![
                (ready_baseline, Some(root.join("ready-baseline.json"))),
                (blocked_baseline, Some(root.join("blocked-baseline.json"))),
            ],
            Some("synthetic-blocked"),
        );

        let promoted =
            promote_shell_handoff_acceptance_baseline_index_default(&index, "synthetic-ready")
                .expect("promote ready baseline");

        assert_eq!(
            promoted.default_baseline_id.as_deref(),
            Some("synthetic-ready")
        );
        assert_eq!(promoted.baseline_count, 2);
        assert_eq!(promoted.ready_baseline_count, 1);
        assert_eq!(promoted.blocked_baseline_count, 1);
        assert_eq!(
            select_shell_handoff_acceptance_baseline_index_entry(&promoted, None)
                .map(|entry| entry.baseline_id.as_str()),
            Some("synthetic-ready")
        );
        assert!(
            promote_shell_handoff_acceptance_baseline_index_default(&index, "missing").is_none()
        );
    }

    #[test]
    fn shell_handoff_acceptance_baseline_selection_reports_default_entry() {
        let root = temp_root("shell-handoff-acceptance-baseline-selection");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let ready_bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let ready_checklist = shell_handoff_acceptance_checklist_for_project(
            &project,
            Some(&root),
            &ready_bundle_root,
        );
        let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
            &project,
            Some(&root),
            &root.join("missing-selected-shells"),
        );
        let ready_checklist_path = root.join("ready-checklist.json");
        let blocked_checklist_path = root.join("blocked-checklist.json");
        let ready_manifest_path = root.join("ready-baseline.json");
        let blocked_manifest_path = root.join("blocked-baseline.json");
        let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &ready_checklist,
            &ready_checklist_path,
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );
        let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &blocked_checklist,
            &blocked_checklist_path,
            Some("synthetic-blocked"),
            Some("Synthetic blocked acceptance baseline"),
        );
        let index = shell_handoff_acceptance_baseline_index_for_manifests(
            vec![
                (ready_baseline, Some(ready_manifest_path.clone())),
                (blocked_baseline, Some(blocked_manifest_path)),
            ],
            Some("synthetic-ready"),
        );
        let index_path = root.join("shell-handoff-acceptance-baselines.json");

        let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
            &index,
            Some(&index_path),
            None,
        );

        assert_eq!(
            selection.schema_id,
            SHELL_HANDOFF_ACCEPTANCE_BASELINE_SELECTION_SCHEMA
        );
        assert_eq!(
            selection.source_index_schema,
            SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA
        );
        assert_eq!(
            selection.index_path.as_deref(),
            Some(index_path.display().to_string().as_str())
        );
        assert_eq!(selection.requested_baseline_id, None);
        assert_eq!(
            selection.default_baseline_id.as_deref(),
            Some("synthetic-ready")
        );
        assert_eq!(
            selection.selected_baseline_id.as_deref(),
            Some("synthetic-ready")
        );
        assert_eq!(
            selection.status,
            StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
        );
        assert_eq!(selection.issue_code, None);
        assert_eq!(selection.baseline_count, 2);
        assert_eq!(selection.ready_baseline_count, 1);
        assert_eq!(selection.blocked_baseline_count, 1);
        assert_eq!(selection.project_ids, vec!["studio.project.test"]);
        assert_eq!(selection.entries.len(), 2);
        let selected = selection
            .entries
            .iter()
            .find(|entry| entry.baseline_id == "synthetic-ready")
            .expect("selected entry");
        assert!(selected.selected);
        assert!(selected.default);
        assert_eq!(
            selected.baseline_manifest_path.as_deref(),
            Some(ready_manifest_path.display().to_string().as_str())
        );
        assert_eq!(selected.status, StudioShellHandoffAcceptanceStatus::Ready);
        assert_eq!(selected.ready_count, 3);
        assert_eq!(selected.entry_count, 3);
        assert_eq!(selected.target_count, 3);
        let blocked = selection
            .entries
            .iter()
            .find(|entry| entry.baseline_id == "synthetic-blocked")
            .expect("blocked entry");
        assert!(!blocked.selected);
        assert!(!blocked.default);
        assert_eq!(blocked.status, StudioShellHandoffAcceptanceStatus::Blocked);
    }

    #[test]
    fn shell_handoff_acceptance_baseline_selection_reports_missing_request() {
        let root = temp_root("shell-handoff-acceptance-baseline-selection-missing");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let checklist =
            shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
        let checklist_path = root.join("ready-checklist.json");
        let baseline_path = root.join("ready-baseline.json");
        let baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &checklist,
            &checklist_path,
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );
        let index = shell_handoff_acceptance_baseline_index_for_manifests(
            vec![(baseline, Some(baseline_path))],
            Some("synthetic-ready"),
        );

        let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
            &index,
            None,
            Some("synthetic-missing"),
        );

        assert_eq!(
            selection.status,
            StudioShellHandoffAcceptanceBaselineSelectionStatus::Missing
        );
        assert_eq!(
            selection.issue_code.as_deref(),
            Some("studio.issue.shell_handoff_acceptance_baseline_not_found")
        );
        assert_eq!(
            selection.requested_baseline_id.as_deref(),
            Some("synthetic-missing")
        );
        assert_eq!(selection.selected_baseline_id, None);
        assert_eq!(selection.baseline_count, 1);
        assert!(selection.entries.iter().all(|entry| !entry.selected));
    }

    #[test]
    fn shell_handoff_acceptance_baseline_selection_reports_empty_index() {
        let index = shell_handoff_acceptance_baseline_index_for_manifests(Vec::new(), None);

        let selection =
            summarize_shell_handoff_acceptance_baseline_index_selection(&index, None, None);

        assert_eq!(
            selection.status,
            StudioShellHandoffAcceptanceBaselineSelectionStatus::Empty
        );
        assert_eq!(
            selection.issue_code.as_deref(),
            Some("studio.issue.shell_handoff_acceptance_baseline_index_empty")
        );
        assert_eq!(selection.baseline_count, 0);
        assert_eq!(selection.selected_baseline_id, None);
        assert!(selection.entries.is_empty());
    }

    #[test]
    fn shell_handoff_acceptance_checklist_rejects_invalid_intake() {
        let root = temp_root("shell-handoff-acceptance-invalid");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        manifest.runtime_authority.command_session_authority = "rusty.studio".to_string();
        let intake = shell_handoff_intake_for_manifest(&manifest);

        let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

        assert_eq!(
            checklist.status,
            StudioShellHandoffAcceptanceStatus::Rejected
        );
        assert_eq!(
            checklist.issue_code.as_deref(),
            Some("studio.issue.runtime_authority_mismatch")
        );
        assert_eq!(checklist.ready_count, 0);
        assert_eq!(checklist.blocked_count, 0);
        assert_eq!(checklist.rejected_count, 0);
        assert!(checklist.entries.is_empty());
        assert!(checklist.intake_checks.iter().any(|check| {
            check.status == StudioValidationStatus::Fail
                && check.issue_code.as_deref() == Some("studio.issue.shell_handoff_intake_rejected")
        }));
    }

    #[test]
    fn shell_handoff_acceptance_checklist_blocks_missing_bundles() {
        let root = temp_root("shell-handoff-acceptance-missing");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("missing-selected-shells");
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let intake = shell_handoff_intake_for_manifest(&manifest);

        let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

        assert_eq!(
            checklist.status,
            StudioShellHandoffAcceptanceStatus::Blocked
        );
        assert_eq!(
            checklist.issue_code.as_deref(),
            Some("studio.issue.shell_bundle_file_missing")
        );
        assert_eq!(checklist.ready_count, 0);
        assert_eq!(checklist.blocked_count, 3);
        assert_eq!(checklist.rejected_count, 0);
        assert_eq!(checklist.entries.len(), 3);
        assert!(checklist
            .intake_checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
        assert!(checklist.entries.iter().all(|entry| {
            entry.status == StudioShellHandoffAcceptanceStatus::Blocked
                && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
                && entry.next_required_action == "repair_export_bundle"
                && entry.checks.iter().any(|check| {
                    check.status == StudioValidationStatus::Fail
                        && check.issue_code.as_deref()
                            == Some("studio.issue.shell_handoff_acceptance_blocked")
                })
        }));
    }

    #[test]
    fn shell_handoff_acceptance_comparison_reports_unchanged_ready_checklists() {
        let root = temp_root("shell-handoff-acceptance-compare-unchanged");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let intake = shell_handoff_intake_for_manifest(&manifest);
        let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

        let comparison = compare_shell_handoff_acceptance_checklists(&checklist, &checklist);

        assert_eq!(
            comparison.schema_id,
            SHELL_HANDOFF_ACCEPTANCE_COMPARISON_SCHEMA
        );
        assert_eq!(comparison.baseline_identity_schema, None);
        assert_eq!(comparison.baseline_id, None);
        assert_eq!(comparison.baseline_label, None);
        assert_eq!(comparison.baseline_checklist_path, None);
        assert_eq!(comparison.baseline_index_schema, None);
        assert_eq!(comparison.baseline_index_path, None);
        assert_eq!(comparison.baseline_index_default_baseline_id, None);
        assert_eq!(comparison.baseline_index_selected_baseline_id, None);
        assert_eq!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Unchanged
        );
        assert_eq!(comparison.issue_code, None);
        assert_eq!(comparison.ready_delta, 0);
        assert_eq!(comparison.blocked_delta, 0);
        assert_eq!(comparison.rejected_delta, 0);
        assert_eq!(comparison.entries.len(), 3);
        assert!(comparison.entries.iter().all(|entry| entry.change
            == StudioShellHandoffAcceptanceComparisonChange::Unchanged
            && entry.score_delta == 0));
        assert!(comparison
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
    }

    #[test]
    fn shell_handoff_acceptance_comparison_carries_baseline_identity() {
        let root = temp_root("shell-handoff-acceptance-compare-baseline-identity");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let checklist =
            shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
        let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
        let baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &checklist,
            &checklist_path,
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );

        let comparison = compare_shell_handoff_acceptance_against_baseline_manifest(
            &baseline_identity,
            &checklist,
            &checklist,
        );

        assert_eq!(
            comparison.baseline_identity_schema.as_deref(),
            Some(SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA)
        );
        assert_eq!(comparison.baseline_id.as_deref(), Some("synthetic-ready"));
        assert_eq!(
            comparison.baseline_label.as_deref(),
            Some("Synthetic ready acceptance baseline")
        );
        let checklist_path_text = checklist_path.display().to_string();
        assert_eq!(
            comparison.baseline_checklist_path.as_deref(),
            Some(checklist_path_text.as_str())
        );
        assert_eq!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Unchanged
        );
        assert!(comparison
            .checks
            .iter()
            .filter(|check| check.check_id.contains("baseline_identity"))
            .all(|check| check.status == StudioValidationStatus::Pass));
        assert_eq!(comparison.entries.len(), 3);
    }

    #[test]
    fn shell_handoff_acceptance_comparison_carries_baseline_index_selection() {
        let root = temp_root("shell-handoff-acceptance-compare-baseline-index");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let checklist =
            shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
        let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
        let baseline_path = root.join("shell-handoff-acceptance-baseline.json");
        let index_path = root.join("shell-handoff-acceptance-baselines.json");
        let baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &checklist,
            &checklist_path,
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );
        let index = shell_handoff_acceptance_baseline_index_for_manifests(
            vec![(baseline_identity.clone(), Some(baseline_path.clone()))],
            Some("synthetic-ready"),
        );
        let selected_entry = select_shell_handoff_acceptance_baseline_index_entry(&index, None)
            .expect("selected baseline index entry");

        let comparison = compare_shell_handoff_acceptance_against_baseline_index_entry(
            &index,
            Some(&index_path),
            selected_entry,
            Some(&baseline_path),
            &baseline_identity,
            &checklist,
            &checklist,
        );

        assert_eq!(
            comparison.baseline_index_schema.as_deref(),
            Some(SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA)
        );
        assert_eq!(
            comparison.baseline_index_path.as_deref(),
            Some(index_path.display().to_string().as_str())
        );
        assert_eq!(
            comparison.baseline_index_default_baseline_id.as_deref(),
            Some("synthetic-ready")
        );
        assert_eq!(
            comparison.baseline_index_selected_baseline_id.as_deref(),
            Some("synthetic-ready")
        );
        assert_eq!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Unchanged
        );
        assert!(comparison
            .checks
            .iter()
            .filter(|check| check.check_id.contains("baseline_index"))
            .all(|check| check.status == StudioValidationStatus::Pass));
        assert_eq!(comparison.entries.len(), 3);
    }

    #[test]
    fn shell_handoff_acceptance_comparison_rejects_stale_baseline_index_selection() {
        let root = temp_root("shell-handoff-acceptance-compare-stale-baseline-index");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let checklist =
            shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
        let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
        let baseline_path = root.join("shell-handoff-acceptance-baseline.json");
        let index_path = root.join("shell-handoff-acceptance-baselines.json");
        let baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &checklist,
            &checklist_path,
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );
        let mut index = shell_handoff_acceptance_baseline_index_for_manifests(
            vec![(baseline_identity.clone(), Some(baseline_path.clone()))],
            Some("synthetic-ready"),
        );
        index.entries[0].ready_count += 1;
        let selected_entry = select_shell_handoff_acceptance_baseline_index_entry(&index, None)
            .expect("selected baseline index entry");

        let comparison = compare_shell_handoff_acceptance_against_baseline_index_entry(
            &index,
            Some(&index_path),
            selected_entry,
            Some(&baseline_path),
            &baseline_identity,
            &checklist,
            &checklist,
        );

        assert_eq!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Incomparable
        );
        assert_eq!(
            comparison.issue_code.as_deref(),
            Some("studio.issue.shell_handoff_acceptance_baseline_index_mismatch")
        );
        assert!(comparison.entries.is_empty());
        assert!(comparison.checks.iter().any(|check| {
            check.check_id
                == "studio.check.shell_handoff_acceptance_comparison.baseline_index_status_counts"
                && check.status == StudioValidationStatus::Fail
        }));
    }

    #[test]
    fn shell_handoff_acceptance_comparison_rejects_stale_baseline_identity() {
        let root = temp_root("shell-handoff-acceptance-compare-stale-baseline-identity");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let checklist =
            shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
        let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
        let mut baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &checklist,
            &checklist_path,
            None,
            None,
        );
        baseline_identity.summary.project_revision += 1;

        let comparison = compare_shell_handoff_acceptance_against_baseline_manifest(
            &baseline_identity,
            &checklist,
            &checklist,
        );

        assert_eq!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Incomparable
        );
        assert_eq!(
            comparison.issue_code.as_deref(),
            Some("studio.issue.shell_handoff_acceptance_baseline_identity_mismatch")
        );
        assert!(comparison.entries.is_empty());
        assert!(comparison.checks.iter().any(|check| {
            check.check_id
                == "studio.check.shell_handoff_acceptance_comparison.baseline_identity_project"
                && check.status == StudioValidationStatus::Fail
        }));
    }

    #[test]
    fn shell_handoff_acceptance_comparison_reports_regression_to_missing_bundles() {
        let root = temp_root("shell-handoff-acceptance-compare-regressed");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let ready_bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let ready_manifest =
            shell_handoff_manifest_for_project(&project, Some(&root), &ready_bundle_root);
        let ready_intake = shell_handoff_intake_for_manifest(&ready_manifest);
        let ready_checklist = shell_handoff_acceptance_checklist_for_intake(&ready_intake);
        let missing_manifest =
            shell_handoff_manifest_for_project(&project, Some(&root), &root.join("missing"));
        let missing_intake = shell_handoff_intake_for_manifest(&missing_manifest);
        let missing_checklist = shell_handoff_acceptance_checklist_for_intake(&missing_intake);

        let comparison =
            compare_shell_handoff_acceptance_checklists(&ready_checklist, &missing_checklist);

        assert_eq!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Regressed
        );
        assert_eq!(
            comparison.issue_code.as_deref(),
            Some("studio.issue.shell_bundle_file_missing")
        );
        assert_eq!(comparison.ready_delta, -3);
        assert_eq!(comparison.blocked_delta, 3);
        assert_eq!(comparison.rejected_delta, 0);
        assert_eq!(comparison.entries.len(), 3);
        assert!(comparison.entries.iter().all(|entry| {
            entry.change == StudioShellHandoffAcceptanceComparisonChange::Regressed
                && entry.score_delta == -1
                && entry.candidate_issue_code.as_deref()
                    == Some("studio.issue.shell_bundle_file_missing")
        }));
    }

    #[test]
    fn shell_handoff_acceptance_comparison_reports_improvement_from_missing_bundles() {
        let root = temp_root("shell-handoff-acceptance-compare-improved");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let ready_bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let ready_manifest =
            shell_handoff_manifest_for_project(&project, Some(&root), &ready_bundle_root);
        let ready_intake = shell_handoff_intake_for_manifest(&ready_manifest);
        let ready_checklist = shell_handoff_acceptance_checklist_for_intake(&ready_intake);
        let missing_manifest =
            shell_handoff_manifest_for_project(&project, Some(&root), &root.join("missing"));
        let missing_intake = shell_handoff_intake_for_manifest(&missing_manifest);
        let missing_checklist = shell_handoff_acceptance_checklist_for_intake(&missing_intake);

        let comparison =
            compare_shell_handoff_acceptance_checklists(&missing_checklist, &ready_checklist);

        assert_eq!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Improved
        );
        assert_eq!(comparison.issue_code, None);
        assert_eq!(comparison.ready_delta, 3);
        assert_eq!(comparison.blocked_delta, -3);
        assert_eq!(comparison.rejected_delta, 0);
        assert!(comparison.entries.iter().all(|entry| {
            entry.change == StudioShellHandoffAcceptanceComparisonChange::Improved
                && entry.score_delta == 1
        }));
    }

    #[test]
    fn shell_handoff_acceptance_comparison_rejects_mismatched_projects() {
        let root = temp_root("shell-handoff-acceptance-compare-mismatch");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        for graph in &project.graphs {
            let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
            save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
                .expect("save selected shell bundle");
        }
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let intake = shell_handoff_intake_for_manifest(&manifest);
        let baseline = shell_handoff_acceptance_checklist_for_intake(&intake);
        let mut candidate = baseline.clone();
        candidate.project_id = "studio.project.other".to_string();

        let comparison = compare_shell_handoff_acceptance_checklists(&baseline, &candidate);

        assert_eq!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Incomparable
        );
        assert_eq!(
            comparison.issue_code.as_deref(),
            Some("studio.issue.shell_handoff_acceptance_project_mismatch")
        );
        assert!(comparison.entries.is_empty());
        assert!(comparison.checks.iter().any(|check| {
            check.status == StudioValidationStatus::Fail
                && check.issue_code.as_deref()
                    == Some("studio.issue.shell_handoff_acceptance_project_mismatch")
        }));
    }

    #[test]
    fn shell_release_candidate_review_reports_ready_from_indexes() {
        let root = temp_root("shell-release-candidate-ready");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        save_selected_shell_bundles(&project, &root, &bundle_root);
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let manifest_path = root.join("shell-handoffs.json");
        save_json(&manifest_path, &manifest).expect("save shell handoff manifest");

        let acceptance_checklist = shell_handoff_acceptance_checklist_for_intake(
            &shell_handoff_intake_for_manifest(&manifest),
        );
        let acceptance_checklist_path = root.join("shell-handoff-acceptance-checklist.json");
        save_json(&acceptance_checklist_path, &acceptance_checklist)
            .expect("save acceptance checklist");
        let acceptance_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &acceptance_checklist,
            &acceptance_checklist_path,
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );
        let acceptance_baseline_path = root.join("shell-handoff-acceptance-baseline.json");
        save_json(&acceptance_baseline_path, &acceptance_baseline)
            .expect("save acceptance baseline");
        let acceptance_index = shell_handoff_acceptance_baseline_index_for_manifests(
            vec![(acceptance_baseline, Some(acceptance_baseline_path))],
            Some("synthetic-ready"),
        );
        let acceptance_index_path = root.join("shell-handoff-acceptance-baselines.json");
        save_json(&acceptance_index_path, &acceptance_index).expect("save acceptance index");

        let export_package = shell_export_package_for_manifest(&manifest);
        let export_package_path = root.join("shell-export-package.json");
        save_json(&export_package_path, &export_package).expect("save export package");
        let export_package_baseline = shell_export_package_baseline_manifest_for_report(
            &export_package,
            &export_package_path,
            Some("synthetic-ready-package"),
            Some("Synthetic ready export package baseline"),
        );
        let export_package_baseline_path = root.join("shell-export-package-baseline.json");
        save_json(&export_package_baseline_path, &export_package_baseline)
            .expect("save export package baseline");
        let export_package_index = shell_export_package_baseline_index_for_manifests(
            vec![(export_package_baseline, Some(export_package_baseline_path))],
            Some("synthetic-ready-package"),
        );
        let export_package_index_path = root.join("shell-export-package-baselines.json");
        save_json(&export_package_index_path, &export_package_index)
            .expect("save export package index");

        let review = shell_release_candidate_review_for_manifest(
            &manifest,
            Some(&manifest_path),
            &acceptance_index,
            Some(&acceptance_index_path),
            Some("synthetic-ready"),
            &export_package_index,
            Some(&export_package_index_path),
            Some("synthetic-ready-package"),
        );

        assert_eq!(review.schema_id, SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA);
        assert_eq!(review.source_manifest_schema, SHELL_HANDOFF_MANIFEST_SCHEMA);
        assert_eq!(
            review.manifest_path.as_deref(),
            Some(manifest_path.display().to_string().as_str())
        );
        assert_eq!(
            review.status,
            StudioShellReleaseCandidateReviewStatus::Ready
        );
        assert_eq!(review.issue_code, None);
        assert_eq!(review.execution_policy, "not_executed.review_only");
        assert_eq!(review.review_owner, "rusty.hostess");
        assert_eq!(review.command_session_authority, "rusty.manifold");
        assert_eq!(review.install_launch_evidence_authority, "rusty.hostess");
        assert_eq!(review.studio_role, "authoring.export_planning");
        assert_eq!(review.handoff_status, StudioValidationStatus::Pass);
        assert_eq!(review.handoff_ready_count, 3);
        assert_eq!(review.handoff_failed_count, 0);
        assert_eq!(review.handoff_missing_bundle_count, 0);
        assert_eq!(
            review.acceptance_baseline_selection.status,
            StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
        );
        assert_eq!(
            review
                .acceptance_baseline_selection
                .selected_baseline_id
                .as_deref(),
            Some("synthetic-ready")
        );
        assert_eq!(
            review
                .acceptance_comparison
                .as_ref()
                .map(|comparison| comparison.status),
            Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
        );
        assert_eq!(
            review.export_package_baseline_selection.status,
            StudioShellExportPackageBaselineSelectionStatus::Selected
        );
        assert_eq!(
            review
                .export_package_baseline_selection
                .selected_baseline_id
                .as_deref(),
            Some("synthetic-ready-package")
        );
        assert_eq!(
            review
                .export_package_comparison
                .as_ref()
                .map(|comparison| comparison.status),
            Some(StudioShellExportPackageComparisonStatus::Unchanged)
        );
        assert!(review
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
        assert!(review.prohibited_actions.contains(&"install".to_string()));
        assert!(review
            .prohibited_actions
            .contains(&"open_command_session".to_string()));
    }

    #[test]
    fn shell_release_candidate_review_blocks_regressed_export_package() {
        let root = temp_root("shell-release-candidate-regressed-package");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        save_selected_shell_bundles(&project, &root, &bundle_root);
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

        let acceptance_checklist = shell_handoff_acceptance_checklist_for_intake(
            &shell_handoff_intake_for_manifest(&manifest),
        );
        let acceptance_checklist_path = root.join("shell-handoff-acceptance-checklist.json");
        save_json(&acceptance_checklist_path, &acceptance_checklist)
            .expect("save acceptance checklist");
        let acceptance_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &acceptance_checklist,
            &acceptance_checklist_path,
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );
        let acceptance_baseline_path = root.join("shell-handoff-acceptance-baseline.json");
        save_json(&acceptance_baseline_path, &acceptance_baseline)
            .expect("save acceptance baseline");
        let acceptance_index = shell_handoff_acceptance_baseline_index_for_manifests(
            vec![(acceptance_baseline, Some(acceptance_baseline_path))],
            Some("synthetic-ready"),
        );
        let acceptance_index_path = root.join("shell-handoff-acceptance-baselines.json");

        let export_package = shell_export_package_for_manifest(&manifest);
        let export_package_path = root.join("shell-export-package.json");
        save_json(&export_package_path, &export_package).expect("save export package");
        let export_package_baseline = shell_export_package_baseline_manifest_for_report(
            &export_package,
            &export_package_path,
            Some("synthetic-ready-package"),
            Some("Synthetic ready export package baseline"),
        );
        let export_package_baseline_path = root.join("shell-export-package-baseline.json");
        save_json(&export_package_baseline_path, &export_package_baseline)
            .expect("save export package baseline");
        let export_package_index = shell_export_package_baseline_index_for_manifests(
            vec![(export_package_baseline, Some(export_package_baseline_path))],
            Some("synthetic-ready-package"),
        );
        let export_package_index_path = root.join("shell-export-package-baselines.json");

        std::fs::remove_file(
            bundle_root
                .join("studio.graph.phone")
                .join("shells/phone/studio.graph.phone.shell-template.json"),
        )
        .expect("remove phone template manifest");

        let review = shell_release_candidate_review_for_manifest(
            &manifest,
            None,
            &acceptance_index,
            Some(&acceptance_index_path),
            Some("synthetic-ready"),
            &export_package_index,
            Some(&export_package_index_path),
            Some("synthetic-ready-package"),
        );

        assert_eq!(
            review.status,
            StudioShellReleaseCandidateReviewStatus::Blocked
        );
        assert_eq!(
            review.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_template_load_failed")
        );
        assert_eq!(
            review
                .acceptance_comparison
                .as_ref()
                .map(|comparison| comparison.status),
            Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
        );
        let export_package_comparison = review
            .export_package_comparison
            .as_ref()
            .expect("export package comparison");
        assert_eq!(
            export_package_comparison.status,
            StudioShellExportPackageComparisonStatus::Regressed
        );
        assert_eq!(export_package_comparison.ready_delta, -1);
        assert_eq!(export_package_comparison.blocked_delta, 1);
        assert!(review.checks.iter().any(|check| {
            check.check_id
                == "studio.check.shell_release_candidate_review.export_package_comparison_not_regressed"
                && check.status == StudioValidationStatus::Fail
                && check.issue_code.as_deref()
                    == Some("studio.issue.shell_export_package_template_load_failed")
        }));
    }

    #[test]
    fn shell_release_candidate_review_index_lists_and_selects_candidates() {
        let root = temp_root("shell-release-candidate-index");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        save_selected_shell_bundles(&project, &root, &bundle_root);
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let manifest_path = root.join("shell-handoffs.json");
        save_json(&manifest_path, &manifest).expect("save shell handoff manifest");

        let acceptance_checklist = shell_handoff_acceptance_checklist_for_intake(
            &shell_handoff_intake_for_manifest(&manifest),
        );
        let acceptance_checklist_path = root.join("shell-handoff-acceptance-checklist.json");
        save_json(&acceptance_checklist_path, &acceptance_checklist)
            .expect("save acceptance checklist");
        let acceptance_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &acceptance_checklist,
            &acceptance_checklist_path,
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );
        let acceptance_baseline_path = root.join("shell-handoff-acceptance-baseline.json");
        save_json(&acceptance_baseline_path, &acceptance_baseline)
            .expect("save acceptance baseline");
        let acceptance_index = shell_handoff_acceptance_baseline_index_for_manifests(
            vec![(acceptance_baseline, Some(acceptance_baseline_path))],
            Some("synthetic-ready"),
        );
        let acceptance_index_path = root.join("shell-handoff-acceptance-baselines.json");
        save_json(&acceptance_index_path, &acceptance_index).expect("save acceptance index");

        let export_package = shell_export_package_for_manifest(&manifest);
        let export_package_path = root.join("shell-export-package.json");
        save_json(&export_package_path, &export_package).expect("save export package");
        let export_package_baseline = shell_export_package_baseline_manifest_for_report(
            &export_package,
            &export_package_path,
            Some("synthetic-ready-package"),
            Some("Synthetic ready export package baseline"),
        );
        let export_package_baseline_path = root.join("shell-export-package-baseline.json");
        save_json(&export_package_baseline_path, &export_package_baseline)
            .expect("save export package baseline");
        let export_package_index = shell_export_package_baseline_index_for_manifests(
            vec![(export_package_baseline, Some(export_package_baseline_path))],
            Some("synthetic-ready-package"),
        );
        let export_package_index_path = root.join("shell-export-package-baselines.json");
        save_json(&export_package_index_path, &export_package_index)
            .expect("save export package index");

        let ready_review = shell_release_candidate_review_for_manifest(
            &manifest,
            Some(&manifest_path),
            &acceptance_index,
            Some(&acceptance_index_path),
            Some("synthetic-ready"),
            &export_package_index,
            Some(&export_package_index_path),
            Some("synthetic-ready-package"),
        );
        let ready_review_path = root.join("shell-release-candidate-review.json");
        save_json(&ready_review_path, &ready_review).expect("save ready review");
        let ready_candidate = shell_release_candidate_review_manifest_for_report(
            &ready_review,
            &ready_review_path,
            None,
            None,
        );
        let ready_candidate_path = root.join("shell-release-candidate-review-manifest.json");
        save_json(&ready_candidate_path, &ready_candidate).expect("save ready candidate");

        assert_eq!(
            ready_candidate.schema_id,
            SHELL_RELEASE_CANDIDATE_REVIEW_MANIFEST_SCHEMA
        );
        assert_eq!(
            ready_candidate.candidate_id,
            "studio.project.test.rev1.ready"
        );
        assert_eq!(
            ready_candidate.review_path,
            ready_review_path.display().to_string()
        );
        assert_eq!(
            ready_candidate.status,
            StudioShellReleaseCandidateReviewStatus::Ready
        );
        assert_eq!(
            ready_candidate.acceptance_comparison_status,
            Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
        );
        assert_eq!(
            ready_candidate.export_package_comparison_status,
            Some(StudioShellExportPackageComparisonStatus::Unchanged)
        );
        assert_eq!(ready_candidate.failed_check_count, 0);

        let index = shell_release_candidate_review_index_for_manifests(
            vec![(ready_candidate.clone(), Some(ready_candidate_path.clone()))],
            None,
        );
        let index_path = root.join("shell-release-candidate-reviews.json");
        save_json(&index_path, &index).expect("save release candidate index");
        assert_eq!(index.schema_id, SHELL_RELEASE_CANDIDATE_REVIEW_INDEX_SCHEMA);
        assert_eq!(
            index.default_candidate_id.as_deref(),
            Some("studio.project.test.rev1.ready")
        );
        assert_eq!(index.candidate_count, 1);
        assert_eq!(index.ready_candidate_count, 1);
        assert_eq!(index.blocked_candidate_count, 0);
        assert_eq!(index.rejected_candidate_count, 0);
        assert_eq!(index.entries[0].candidate_id, ready_candidate.candidate_id);
        assert_eq!(
            index.entries[0].candidate_manifest_path.as_deref(),
            Some(ready_candidate_path.display().to_string().as_str())
        );

        let selection = summarize_shell_release_candidate_review_index_selection(
            &index,
            Some(&index_path),
            None,
        );
        assert_eq!(
            selection.schema_id,
            SHELL_RELEASE_CANDIDATE_REVIEW_SELECTION_SCHEMA
        );
        assert_eq!(
            selection.status,
            StudioShellReleaseCandidateReviewSelectionStatus::Selected
        );
        assert_eq!(selection.issue_code, None);
        assert_eq!(
            selection.selected_candidate_id.as_deref(),
            Some("studio.project.test.rev1.ready")
        );
        assert_eq!(selection.ready_candidate_count, 1);
        assert_eq!(selection.entries.len(), 1);
        assert!(selection.entries[0].selected);
        assert!(selection.entries[0].default);

        std::fs::remove_file(
            bundle_root
                .join("studio.graph.phone")
                .join("shells/phone/studio.graph.phone.shell-template.json"),
        )
        .expect("remove phone template manifest");
        let blocked_review = shell_release_candidate_review_for_manifest(
            &manifest,
            Some(&manifest_path),
            &acceptance_index,
            Some(&acceptance_index_path),
            Some("synthetic-ready"),
            &export_package_index,
            Some(&export_package_index_path),
            Some("synthetic-ready-package"),
        );
        let blocked_review_path = root.join("shell-release-candidate-review-blocked.json");
        save_json(&blocked_review_path, &blocked_review).expect("save blocked review");
        let blocked_candidate = shell_release_candidate_review_manifest_for_report(
            &blocked_review,
            &blocked_review_path,
            Some("synthetic-blocked"),
            Some("Synthetic blocked release candidate"),
        );
        let blocked_candidate_path = root.join("shell-release-candidate-blocked-manifest.json");
        save_json(&blocked_candidate_path, &blocked_candidate).expect("save blocked candidate");

        let appended = append_shell_release_candidate_review_index_manifests(
            &index,
            vec![(
                blocked_candidate.clone(),
                Some(blocked_candidate_path.clone()),
            )],
            Some("synthetic-blocked"),
        );
        assert_eq!(appended.candidate_count, 2);
        assert_eq!(appended.ready_candidate_count, 1);
        assert_eq!(appended.blocked_candidate_count, 1);
        assert_eq!(
            appended.default_candidate_id.as_deref(),
            Some("synthetic-blocked")
        );
        assert_eq!(
            select_shell_release_candidate_review_index_entry(&appended, Some("synthetic-blocked"))
                .map(|entry| entry.status),
            Some(StudioShellReleaseCandidateReviewStatus::Blocked)
        );

        let promoted = promote_shell_release_candidate_review_index_default(
            &appended,
            "studio.project.test.rev1.ready",
        )
        .expect("promote ready candidate");
        assert_eq!(
            promoted.default_candidate_id.as_deref(),
            Some("studio.project.test.rev1.ready")
        );
        let missing = summarize_shell_release_candidate_review_index_selection(
            &promoted,
            None,
            Some("missing"),
        );
        assert_eq!(
            missing.status,
            StudioShellReleaseCandidateReviewSelectionStatus::Missing
        );
        assert_eq!(
            missing.issue_code.as_deref(),
            Some("studio.issue.shell_release_candidate_review_not_found")
        );
        assert_eq!(missing.selected_candidate_id, None);
    }

    #[test]
    fn shell_hostess_handoff_package_summarizes_selected_candidate() {
        let root = temp_root("shell-hostess-handoff-package");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let bundle_root = root.join("selected-shells");
        save_selected_shell_bundles(&project, &root, &bundle_root);
        let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
        let manifest_path = root.join("shell-handoffs.json");
        save_json(&manifest_path, &manifest).expect("save shell handoff manifest");

        let acceptance_checklist = shell_handoff_acceptance_checklist_for_intake(
            &shell_handoff_intake_for_manifest(&manifest),
        );
        let acceptance_checklist_path = root.join("shell-handoff-acceptance-checklist.json");
        save_json(&acceptance_checklist_path, &acceptance_checklist)
            .expect("save acceptance checklist");
        let acceptance_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &acceptance_checklist,
            &acceptance_checklist_path,
            Some("synthetic-ready"),
            Some("Synthetic ready acceptance baseline"),
        );
        let acceptance_baseline_path = root.join("shell-handoff-acceptance-baseline.json");
        save_json(&acceptance_baseline_path, &acceptance_baseline)
            .expect("save acceptance baseline");
        let acceptance_index = shell_handoff_acceptance_baseline_index_for_manifests(
            vec![(acceptance_baseline, Some(acceptance_baseline_path))],
            Some("synthetic-ready"),
        );
        let acceptance_index_path = root.join("shell-handoff-acceptance-baselines.json");
        save_json(&acceptance_index_path, &acceptance_index).expect("save acceptance index");

        let export_package = shell_export_package_for_manifest(&manifest);
        let export_package_path = root.join("shell-export-package.json");
        save_json(&export_package_path, &export_package).expect("save export package");
        let export_package_baseline = shell_export_package_baseline_manifest_for_report(
            &export_package,
            &export_package_path,
            Some("synthetic-ready-package"),
            Some("Synthetic ready export package baseline"),
        );
        let export_package_baseline_path = root.join("shell-export-package-baseline.json");
        save_json(&export_package_baseline_path, &export_package_baseline)
            .expect("save export package baseline");
        let export_package_index = shell_export_package_baseline_index_for_manifests(
            vec![(export_package_baseline, Some(export_package_baseline_path))],
            Some("synthetic-ready-package"),
        );
        let export_package_index_path = root.join("shell-export-package-baselines.json");
        save_json(&export_package_index_path, &export_package_index)
            .expect("save export package index");

        let ready_review = shell_release_candidate_review_for_manifest(
            &manifest,
            Some(&manifest_path),
            &acceptance_index,
            Some(&acceptance_index_path),
            Some("synthetic-ready"),
            &export_package_index,
            Some(&export_package_index_path),
            Some("synthetic-ready-package"),
        );
        let ready_review_path = root.join("shell-release-candidate-review.json");
        save_json(&ready_review_path, &ready_review).expect("save ready review");
        let ready_candidate = shell_release_candidate_review_manifest_for_report(
            &ready_review,
            &ready_review_path,
            Some("synthetic-ready-candidate"),
            Some("Synthetic ready release candidate"),
        );
        let ready_candidate_path = root.join("shell-release-candidate-review-manifest.json");
        save_json(&ready_candidate_path, &ready_candidate).expect("save ready candidate");
        let index = shell_release_candidate_review_index_for_manifests(
            vec![(ready_candidate, Some(ready_candidate_path.clone()))],
            Some("synthetic-ready-candidate"),
        );
        let index_path = root.join("shell-release-candidate-reviews.json");
        save_json(&index_path, &index).expect("save release candidate index");

        let package = shell_hostess_handoff_package_for_release_candidate_index(
            &index,
            Some(&index_path),
            None,
        );

        assert_eq!(package.schema_id, SHELL_HOSTESS_HANDOFF_PACKAGE_SCHEMA);
        assert_eq!(
            package.source_index_schema,
            SHELL_RELEASE_CANDIDATE_REVIEW_INDEX_SCHEMA
        );
        assert_eq!(
            package.status,
            StudioShellHostessHandoffPackageStatus::Ready
        );
        assert_eq!(package.issue_code, None);
        assert_eq!(
            package.selected_candidate_id.as_deref(),
            Some("synthetic-ready-candidate")
        );
        assert_eq!(
            package.candidate_manifest_schema.as_deref(),
            Some(SHELL_RELEASE_CANDIDATE_REVIEW_MANIFEST_SCHEMA)
        );
        assert_eq!(
            package.candidate_manifest_path.as_deref(),
            Some(ready_candidate_path.display().to_string().as_str())
        );
        assert_eq!(
            package.review_schema.as_deref(),
            Some(SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA)
        );
        assert_eq!(
            package.handoff_manifest_path.as_deref(),
            Some(manifest_path.display().to_string().as_str())
        );
        assert_eq!(
            package.acceptance_baseline_id.as_deref(),
            Some("synthetic-ready")
        );
        assert_eq!(
            package.acceptance_comparison_status,
            Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
        );
        assert_eq!(
            package.export_package_baseline_id.as_deref(),
            Some("synthetic-ready-package")
        );
        assert_eq!(
            package.export_package_comparison_status,
            Some(StudioShellExportPackageComparisonStatus::Unchanged)
        );
        assert_eq!(
            package.command_session_authority.as_deref(),
            Some("rusty.manifold")
        );
        assert_eq!(
            package.install_launch_evidence_authority.as_deref(),
            Some("rusty.hostess")
        );
        assert_eq!(
            package.studio_role.as_deref(),
            Some("authoring.export_planning")
        );
        assert!(package
            .required_owner_actions
            .iter()
            .any(
                |action| action.action_id == "hostess.stage_generated_shells"
                    && action.owner == "rusty.hostess"
                    && action.status == StudioShellHostessHandoffPackageActionStatus::Ready
                    && action.prohibited_in_studio
            ));
        assert!(package.required_owner_actions.iter().any(|action| {
            action.action_id == "manifold.review_command_session_contract"
                && action.owner == "rusty.manifold"
                && action.status == StudioShellHostessHandoffPackageActionStatus::Ready
        }));
        for prohibited in [
            "stage_generated_shells",
            "install",
            "launch",
            "open_command_session",
            "collect_device_evidence",
            "collect_install_launch_evidence",
        ] {
            assert!(package.prohibited_actions.contains(&prohibited.to_string()));
        }
        assert!(package
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));

        let package_path = root.join("shell-hostess-handoff-package.json");
        save_json(&package_path, &package).expect("save Hostess handoff package");
        let intake = shell_hostess_owner_intake_for_handoff_package(&package, Some(&package_path));

        assert_eq!(intake.schema_id, SHELL_HOSTESS_OWNER_INTAKE_SCHEMA);
        assert_eq!(
            intake.source_package_schema,
            SHELL_HOSTESS_HANDOFF_PACKAGE_SCHEMA
        );
        assert_eq!(
            intake.package_path.as_deref(),
            Some(package_path.display().to_string().as_str())
        );
        assert_eq!(intake.status, StudioShellHostessOwnerIntakeStatus::Ready);
        assert_eq!(intake.issue_code, None);
        assert_eq!(intake.execution_policy, "not_executed.request_only");
        assert_eq!(intake.intake_owner, "rusty.hostess");
        assert_eq!(intake.handoff_owner, "rusty.hostess");
        assert_eq!(
            intake.command_session_authority.as_deref(),
            Some("rusty.manifold")
        );
        assert_eq!(
            intake.install_launch_evidence_authority.as_deref(),
            Some("rusty.hostess")
        );
        assert_eq!(
            intake.studio_role.as_deref(),
            Some("authoring.export_planning")
        );
        assert_eq!(intake.source_owner_action_count, 4);
        assert_eq!(intake.ready_assignment_count, 4);
        assert_eq!(intake.blocked_assignment_count, 0);
        assert_eq!(intake.hostess_ready_action_count, 3);
        assert_eq!(intake.manifold_ready_action_count, 1);
        assert!(intake.assignments.iter().any(|assignment| {
            assignment.action_id == "hostess.stage_generated_shells"
                && assignment.owner == "rusty.hostess"
                && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
                && assignment.request_kind == "hostess_owner_action_request"
                && assignment.prohibited_in_studio
        }));
        assert!(intake.assignments.iter().any(|assignment| {
            assignment.action_id == "manifold.review_command_session_contract"
                && assignment.owner == "rusty.manifold"
                && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
                && assignment.request_kind == "manifold_owner_review_request"
        }));
        assert!(intake
            .prohibited_actions
            .contains(&"collect_install_launch_evidence".to_string()));
        assert!(intake
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));

        let intake_path = root.join("shell-hostess-owner-intake.json");
        save_json(&intake_path, &intake).expect("save Hostess owner intake");
        let staging = shell_hostess_staging_preview_for_owner_intake(&intake, Some(&intake_path));

        assert_eq!(
            staging.schema_id,
            SHELL_HOSTESS_STAGING_PREVIEW_MANIFEST_SCHEMA
        );
        assert_eq!(
            staging.source_intake_schema,
            SHELL_HOSTESS_OWNER_INTAKE_SCHEMA
        );
        assert_eq!(
            staging.source_handoff_manifest_schema.as_deref(),
            Some(SHELL_HANDOFF_MANIFEST_SCHEMA)
        );
        assert_eq!(
            staging.intake_path.as_deref(),
            Some(intake_path.display().to_string().as_str())
        );
        assert_eq!(
            staging.status,
            StudioShellHostessStagingPreviewStatus::Ready
        );
        assert_eq!(staging.issue_code, None);
        assert_eq!(staging.execution_policy, "not_executed.preview_only");
        assert_eq!(staging.staging_owner, "rusty.hostess");
        assert_eq!(
            staging.command_session_authority.as_deref(),
            Some("rusty.manifold")
        );
        assert_eq!(
            staging.install_launch_evidence_authority.as_deref(),
            Some("rusty.hostess")
        );
        assert_eq!(
            staging.studio_role.as_deref(),
            Some("authoring.export_planning")
        );
        assert_eq!(staging.assignment_count, 4);
        assert_eq!(staging.ready_assignment_count, 4);
        assert_eq!(staging.blocked_assignment_count, 0);
        assert_eq!(staging.ready_group_count, 4);
        assert_eq!(staging.blocked_group_count, 0);
        assert!(staging.expected_artifact_count >= 18);
        let stage_group = staging
            .groups
            .iter()
            .find(|group| group.action_id == "hostess.stage_generated_shells")
            .expect("stage generated shells group");
        assert_eq!(stage_group.route_kind, "hostess.stage.generated_shells");
        assert_eq!(
            stage_group.status,
            StudioShellHostessStagingPreviewGroupStatus::Ready
        );
        assert!(stage_group
            .expected_artifacts
            .iter()
            .any(|artifact| artifact.artifact_kind == "shell_descriptor"));
        assert!(stage_group
            .expected_artifacts
            .iter()
            .any(|artifact| artifact.artifact_kind == "manifold_shell_handoff"));
        assert!(stage_group
            .expected_artifacts
            .iter()
            .any(|artifact| artifact.artifact_kind == "shell_template_manifest"));
        assert!(stage_group
            .expected_artifacts
            .iter()
            .any(|artifact| artifact.artifact_kind == "hostess_owner_intake"));
        let manifold_group = staging
            .groups
            .iter()
            .find(|group| group.action_id == "manifold.review_command_session_contract")
            .expect("Manifold review group");
        assert_eq!(
            manifold_group.route_kind,
            "manifold.review.command_session_contract"
        );
        assert!(manifold_group
            .expected_artifacts
            .iter()
            .any(|artifact| artifact.route_hint.is_some()));
        assert!(manifold_group.expected_artifacts.iter().any(|artifact| {
            artifact.artifact_kind == "manifold_shell_handoff"
                && artifact.route_hint.as_deref() == Some("manifold.shell_handoff_review")
        }));
        assert!(staging
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));

        let staging_path = root.join("shell-hostess-staging-preview.json");
        save_json(&staging_path, &staging).expect("save Hostess staging preview");
        let file_plan = shell_hostess_staging_file_plan_for_preview(&staging, Some(&staging_path));

        assert_eq!(file_plan.schema_id, SHELL_HOSTESS_STAGING_FILE_PLAN_SCHEMA);
        assert_eq!(
            file_plan.source_preview_schema,
            SHELL_HOSTESS_STAGING_PREVIEW_MANIFEST_SCHEMA
        );
        assert_eq!(
            file_plan.preview_path.as_deref(),
            Some(staging_path.display().to_string().as_str())
        );
        assert_eq!(
            file_plan.status,
            StudioShellHostessStagingFilePlanStatus::Ready
        );
        assert_eq!(file_plan.issue_code, None);
        assert_eq!(file_plan.execution_policy, "not_executed.dry_run_only");
        assert_eq!(file_plan.staging_owner, "rusty.hostess");
        assert_eq!(file_plan.preview_group_count, 4);
        assert_eq!(file_plan.ready_preview_group_count, 4);
        assert_eq!(file_plan.blocked_preview_group_count, 0);
        assert_eq!(
            file_plan.source_artifact_count,
            staging.expected_artifact_count
        );
        assert_eq!(file_plan.planned_file_count, 17);
        assert!(file_plan.duplicate_artifact_count > 0);
        assert_eq!(file_plan.request_count, 4);
        assert_eq!(file_plan.ready_request_count, 4);
        assert_eq!(file_plan.blocked_request_count, 0);
        assert_eq!(file_plan.target_request_count, 3);
        assert_eq!(file_plan.shared_request_count, 1);
        let shared_request = file_plan
            .requests
            .iter()
            .find(|request| request.target_key == "shared")
            .expect("shared staging request");
        assert_eq!(
            shared_request.status,
            StudioShellHostessStagingFileRequestStatus::Ready
        );
        assert!(shared_request
            .planned_files
            .iter()
            .any(|file| file.artifact_kind == "hostess_owner_intake"
                && file.destination_path
                    == "hostess-staging/shared/hostess/hostess-owner-intake.json"));
        assert!(shared_request
            .planned_files
            .iter()
            .any(|file| file.artifact_kind == "shell_handoff_manifest"
                && file.source_route_kinds.len() > 1));
        let desktop_request = file_plan
            .requests
            .iter()
            .find(|request| request.target_kind == Some(StudioShellTargetKind::Desktop))
            .expect("desktop staging request");
        assert_eq!(
            desktop_request.status,
            StudioShellHostessStagingFileRequestStatus::Ready
        );
        for artifact_kind in [
            "shell_bundle_dir",
            "shell_descriptor",
            "manifold_shell_handoff",
            "shell_template_manifest",
        ] {
            assert!(desktop_request
                .planned_files
                .iter()
                .any(|file| file.artifact_kind == artifact_kind));
        }
        assert!(desktop_request.planned_files.iter().any(|file| {
            file.artifact_kind == "manifold_shell_handoff"
                && file.destination_path
                    == "hostess-staging/targets/desktop/studio.graph.test/manifold/studio.graph.test.manifold-shell-handoff.json"
                && file
                    .route_hints
                    .contains(&"manifold.shell_handoff_review".to_string())
                && file.source_route_kinds.len() > 1
        }));
        assert!(file_plan.requests.iter().all(|request| {
            request.owner == "rusty.hostess"
                && request.planned_file_count == request.planned_files.len()
                && request.planned_files.iter().all(|file| {
                    !file.source_action_ids.is_empty()
                        && !file.source_route_kinds.is_empty()
                        && is_safe_relative_manifest_path(&file.destination_path)
                })
        }));
        assert!(file_plan
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));

        let file_plan_path = root.join("shell-hostess-staging-file-plan.json");
        save_json(&file_plan_path, &file_plan).expect("save Hostess staging file plan");
        let envelope =
            shell_hostess_staging_handoff_envelope_for_file_plan(&file_plan, Some(&file_plan_path));

        assert_eq!(
            envelope.schema_id,
            SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA
        );
        assert_eq!(
            envelope.source_file_plan_schema,
            SHELL_HOSTESS_STAGING_FILE_PLAN_SCHEMA
        );
        assert_eq!(
            envelope.file_plan_path.as_deref(),
            Some(file_plan_path.display().to_string().as_str())
        );
        assert_eq!(
            envelope.status,
            StudioShellHostessStagingHandoffEnvelopeStatus::Ready
        );
        assert_eq!(envelope.issue_code, None);
        assert_eq!(envelope.execution_policy, "not_executed.handoff_only");
        assert_eq!(envelope.handoff_owner, "rusty.hostess");
        assert_eq!(envelope.staging_owner, "rusty.hostess");
        assert_eq!(envelope.planned_file_count, file_plan.planned_file_count);
        assert_eq!(envelope.request_count, file_plan.request_count);
        assert_eq!(envelope.ready_request_count, file_plan.ready_request_count);
        assert_eq!(envelope.blocked_request_count, 0);
        assert_eq!(envelope.target_request_count, 3);
        assert_eq!(envelope.shared_request_count, 1);
        assert_eq!(envelope.instruction_count, 4);
        assert_eq!(envelope.ready_instruction_count, 4);
        assert_eq!(envelope.blocked_instruction_count, 0);
        assert_eq!(
            envelope.envelope_id,
            "studio.hostess_staging_handoff.studio.project.test.rev1"
        );
        assert_eq!(
            envelope.provenance.checksum_algorithm,
            "fnv1a64.studio_staging_file_plan.v1"
        );
        assert_eq!(envelope.provenance.plan_checksum.len(), 16);
        assert!(envelope
            .provenance
            .source_artifact_kinds
            .contains(&"shell_template_manifest".to_string()));
        assert!(envelope
            .provenance
            .source_route_kinds
            .contains(&"hostess.stage.generated_shells".to_string()));
        assert!(envelope
            .provenance
            .source_action_ids
            .contains(&"hostess.stage_generated_shells".to_string()));
        assert!(envelope
            .provenance
            .target_keys
            .contains(&"shared".to_string()));
        assert!(envelope
            .request_summaries
            .iter()
            .any(
                |summary| summary.target_kind == Some(StudioShellTargetKind::Desktop)
                    && summary.planned_file_count == 4
            ));
        assert!(envelope.owner_instructions.iter().any(|instruction| {
            instruction.instruction_id == "hostess.copy_staging_files"
                && instruction.owner == "rusty.hostess"
                && instruction.route_kind == "hostess.stage.files_from_plan"
                && instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Ready
                && instruction.prohibited_in_studio
        }));
        assert!(envelope.owner_instructions.iter().any(|instruction| {
            instruction.instruction_id == "manifold.review_command_session_contract"
                && instruction.owner == "rusty.manifold"
                && instruction.route_kind == "manifold.review.command_session_contract"
                && instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Ready
        }));
        assert!(envelope
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));

        let handoff_path = root.join("shell-hostess-staging-handoff.json");
        save_json(&handoff_path, &envelope).expect("save Hostess staging handoff");
        let staging_acceptance =
            shell_hostess_staging_acceptance_checklist_for_handoff(&envelope, Some(&handoff_path));

        assert_eq!(
            staging_acceptance.schema_id,
            SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA
        );
        assert_eq!(
            staging_acceptance.source_handoff_schema,
            SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA
        );
        assert_eq!(
            staging_acceptance.handoff_path.as_deref(),
            Some(handoff_path.display().to_string().as_str())
        );
        assert_eq!(
            staging_acceptance.status,
            StudioShellHostessStagingAcceptanceStatus::Ready
        );
        assert_eq!(staging_acceptance.issue_code, None);
        assert_eq!(
            staging_acceptance.execution_policy,
            "not_executed.acceptance_check_only"
        );
        assert_eq!(staging_acceptance.checklist_owner, "rusty.hostess");
        assert_eq!(staging_acceptance.handoff_owner, "rusty.hostess");
        assert_eq!(staging_acceptance.staging_owner, "rusty.hostess");
        assert_eq!(
            staging_acceptance.envelope_id,
            "studio.hostess_staging_handoff.studio.project.test.rev1"
        );
        assert_eq!(
            staging_acceptance.plan_checksum,
            envelope.provenance.plan_checksum
        );
        assert_eq!(staging_acceptance.ready_item_count, 6);
        assert_eq!(staging_acceptance.blocked_item_count, 0);
        assert_eq!(staging_acceptance.rejected_item_count, 0);
        assert_eq!(staging_acceptance.request_count, envelope.request_count);
        assert_eq!(
            staging_acceptance.instruction_count,
            envelope.instruction_count
        );
        assert!(staging_acceptance.entries.iter().any(|entry| {
            entry.item_id == "hostess.copy_staging_files"
                && entry.owner == "rusty.hostess"
                && entry.route_kind == "hostess.stage.files_from_plan"
                && entry.status == StudioShellHostessStagingAcceptanceStatus::Ready
                && entry.prohibited_in_studio
        }));
        assert!(staging_acceptance.entries.iter().any(|entry| {
            entry.item_id == "manifold.review_command_session_contract"
                && entry.owner == "rusty.manifold"
                && entry.route_kind == "manifold.review.command_session_contract"
        }));
        assert!(staging_acceptance
            .handoff_checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));

        std::fs::remove_file(
            bundle_root
                .join("studio.graph.phone")
                .join("shells/phone/studio.graph.phone.shell-template.json"),
        )
        .expect("remove phone template manifest");
        let blocked_review = shell_release_candidate_review_for_manifest(
            &manifest,
            Some(&manifest_path),
            &acceptance_index,
            Some(&acceptance_index_path),
            Some("synthetic-ready"),
            &export_package_index,
            Some(&export_package_index_path),
            Some("synthetic-ready-package"),
        );
        let blocked_review_path = root.join("shell-release-candidate-review-blocked.json");
        save_json(&blocked_review_path, &blocked_review).expect("save blocked review");
        let blocked_candidate = shell_release_candidate_review_manifest_for_report(
            &blocked_review,
            &blocked_review_path,
            Some("synthetic-blocked-candidate"),
            Some("Synthetic blocked release candidate"),
        );
        let blocked_candidate_path = root.join("shell-release-candidate-blocked-manifest.json");
        save_json(&blocked_candidate_path, &blocked_candidate).expect("save blocked candidate");
        let blocked_index = append_shell_release_candidate_review_index_manifests(
            &index,
            vec![(blocked_candidate, Some(blocked_candidate_path))],
            Some("synthetic-blocked-candidate"),
        );

        let blocked_package = shell_hostess_handoff_package_for_release_candidate_index(
            &blocked_index,
            Some(&index_path),
            None,
        );

        assert_eq!(
            blocked_package.status,
            StudioShellHostessHandoffPackageStatus::Blocked
        );
        assert_eq!(
            blocked_package.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_template_load_failed")
        );
        assert!(blocked_package.required_owner_actions.iter().all(|action| {
            action.status == StudioShellHostessHandoffPackageActionStatus::Blocked
                && action.issue_code.as_deref()
                    == Some("studio.issue.shell_export_package_template_load_failed")
        }));

        let blocked_intake =
            shell_hostess_owner_intake_for_handoff_package(&blocked_package, Some(&package_path));
        assert_eq!(
            blocked_intake.status,
            StudioShellHostessOwnerIntakeStatus::Blocked
        );
        assert_eq!(
            blocked_intake.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_template_load_failed")
        );
        assert_eq!(blocked_intake.ready_assignment_count, 0);
        assert_eq!(blocked_intake.blocked_assignment_count, 4);
        assert!(blocked_intake.assignments.iter().all(|assignment| {
            assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Blocked
                && assignment.issue_code.as_deref()
                    == Some("studio.issue.shell_export_package_template_load_failed")
        }));

        let blocked_staging =
            shell_hostess_staging_preview_for_owner_intake(&blocked_intake, Some(&intake_path));
        assert_eq!(
            blocked_staging.status,
            StudioShellHostessStagingPreviewStatus::Blocked
        );
        assert_eq!(
            blocked_staging.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_template_load_failed")
        );
        assert_eq!(blocked_staging.ready_group_count, 0);
        assert_eq!(blocked_staging.blocked_group_count, 4);
        assert!(blocked_staging.groups.iter().all(|group| {
            group.status == StudioShellHostessStagingPreviewGroupStatus::Blocked
                && group.issue_code.as_deref()
                    == Some("studio.issue.shell_export_package_template_load_failed")
        }));

        let blocked_file_plan =
            shell_hostess_staging_file_plan_for_preview(&blocked_staging, Some(&staging_path));
        assert_eq!(
            blocked_file_plan.status,
            StudioShellHostessStagingFilePlanStatus::Blocked
        );
        assert_eq!(
            blocked_file_plan.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_template_load_failed")
        );
        assert_eq!(blocked_file_plan.ready_request_count, 0);
        assert_eq!(
            blocked_file_plan.blocked_request_count,
            blocked_file_plan.request_count
        );
        assert!(blocked_file_plan.requests.iter().all(|request| {
            request.status == StudioShellHostessStagingFileRequestStatus::Blocked
                && request.issue_code.as_deref()
                    == Some("studio.issue.shell_export_package_template_load_failed")
        }));

        let blocked_envelope = shell_hostess_staging_handoff_envelope_for_file_plan(
            &blocked_file_plan,
            Some(&file_plan_path),
        );
        assert_eq!(
            blocked_envelope.status,
            StudioShellHostessStagingHandoffEnvelopeStatus::Blocked
        );
        assert_eq!(
            blocked_envelope.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_template_load_failed")
        );
        assert_eq!(blocked_envelope.ready_instruction_count, 0);
        assert_eq!(blocked_envelope.blocked_instruction_count, 4);
        assert!(blocked_envelope
            .owner_instructions
            .iter()
            .all(|instruction| {
                instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Blocked
                    && instruction.issue_code.as_deref()
                        == Some("studio.issue.shell_export_package_template_load_failed")
            }));

        let blocked_acceptance = shell_hostess_staging_acceptance_checklist_for_handoff(
            &blocked_envelope,
            Some(&handoff_path),
        );
        assert_eq!(
            blocked_acceptance.status,
            StudioShellHostessStagingAcceptanceStatus::Blocked
        );
        assert_eq!(
            blocked_acceptance.issue_code.as_deref(),
            Some("studio.issue.shell_export_package_template_load_failed")
        );
        assert_eq!(blocked_acceptance.ready_item_count, 0);
        assert_eq!(blocked_acceptance.blocked_item_count, 6);
        assert_eq!(blocked_acceptance.rejected_item_count, 0);
        assert!(blocked_acceptance.entries.iter().all(|entry| {
            entry.status == StudioShellHostessStagingAcceptanceStatus::Blocked
                && entry.issue_code.as_deref()
                    == Some("studio.issue.shell_export_package_template_load_failed")
        }));

        let ready_acceptance_path = root.join("shell-hostess-staging-acceptance-ready.json");
        let blocked_acceptance_path = root.join("shell-hostess-staging-acceptance-blocked.json");
        let ready_manifest_path = root.join("shell-hostess-staging-acceptance-ready-manifest.json");
        let blocked_manifest_path =
            root.join("shell-hostess-staging-acceptance-blocked-manifest.json");
        let ready_acceptance = shell_hostess_staging_acceptance_manifest_for_checklist(
            &staging_acceptance,
            &ready_acceptance_path,
            None,
            None,
        );
        let blocked_acceptance_manifest = shell_hostess_staging_acceptance_manifest_for_checklist(
            &blocked_acceptance,
            &blocked_acceptance_path,
            Some("synthetic-blocked-hostess-acceptance"),
            Some("Synthetic blocked Hostess staging acceptance"),
        );

        assert_eq!(
            ready_acceptance.schema_id,
            SHELL_HOSTESS_STAGING_ACCEPTANCE_MANIFEST_SCHEMA
        );
        assert_eq!(
            ready_acceptance.acceptance_id,
            "studio.hostess_staging_acceptance.studio.project.test.rev1.ready"
        );
        assert_eq!(
            ready_acceptance.label,
            "studio.project.test revision 1 ready Hostess staging acceptance"
        );
        assert_eq!(
            ready_acceptance.checklist_path,
            ready_acceptance_path.display().to_string()
        );
        assert_eq!(
            ready_acceptance.checklist_schema,
            SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA
        );
        assert_eq!(
            ready_acceptance.envelope_id,
            "studio.hostess_staging_handoff.studio.project.test.rev1"
        );
        assert_eq!(
            ready_acceptance.status,
            StudioShellHostessStagingAcceptanceStatus::Ready
        );
        assert_eq!(ready_acceptance.ready_item_count, 6);
        assert_eq!(ready_acceptance.blocked_item_count, 0);
        assert_eq!(
            ready_acceptance.request_count,
            staging_acceptance.request_count
        );
        assert_eq!(
            ready_acceptance.execution_policy,
            "not_executed.acceptance_check_only"
        );
        assert_eq!(
            ready_acceptance.command_session_authority.as_deref(),
            Some("rusty.manifold")
        );
        assert_eq!(
            ready_acceptance
                .install_launch_evidence_authority
                .as_deref(),
            Some("rusty.hostess")
        );
        assert_eq!(
            ready_acceptance.plan_checksum,
            staging_acceptance.plan_checksum
        );
        assert_eq!(
            blocked_acceptance_manifest.acceptance_id,
            "synthetic-blocked-hostess-acceptance"
        );
        assert_eq!(
            blocked_acceptance_manifest.status,
            StudioShellHostessStagingAcceptanceStatus::Blocked
        );
        assert_eq!(blocked_acceptance_manifest.ready_item_count, 0);
        assert_eq!(blocked_acceptance_manifest.blocked_item_count, 6);

        let index = shell_hostess_staging_acceptance_index_for_manifests(
            vec![
                (ready_acceptance.clone(), Some(ready_manifest_path.clone())),
                (
                    blocked_acceptance_manifest.clone(),
                    Some(blocked_manifest_path.clone()),
                ),
            ],
            Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready"),
        );

        assert_eq!(
            index.schema_id,
            SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA
        );
        assert_eq!(index.project_ids, vec!["studio.project.test"]);
        assert_eq!(
            index.envelope_ids,
            vec!["studio.hostess_staging_handoff.studio.project.test.rev1"]
        );
        assert_eq!(
            index.default_acceptance_id.as_deref(),
            Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
        );
        assert_eq!(index.acceptance_count, 2);
        assert_eq!(index.ready_acceptance_count, 1);
        assert_eq!(index.blocked_acceptance_count, 1);
        assert_eq!(index.rejected_acceptance_count, 0);
        assert_eq!(index.entries.len(), 2);
        assert_eq!(
            index.entries[0].acceptance_id,
            "studio.hostess_staging_acceptance.studio.project.test.rev1.ready"
        );
        assert_eq!(index.entries[0].ready_item_count, 6);
        assert_eq!(
            index.entries[0].acceptance_manifest_path.as_deref(),
            Some(ready_manifest_path.display().to_string().as_str())
        );
        assert_eq!(
            index.entries[1].acceptance_id,
            "synthetic-blocked-hostess-acceptance"
        );
        assert_eq!(index.entries[1].blocked_item_count, 6);
        assert_eq!(
            select_shell_hostess_staging_acceptance_index_entry(&index, None)
                .map(|entry| entry.acceptance_id.as_str()),
            Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
        );
        assert_eq!(
            select_shell_hostess_staging_acceptance_index_entry(
                &index,
                Some("synthetic-blocked-hostess-acceptance")
            )
            .map(|entry| entry.status),
            Some(StudioShellHostessStagingAcceptanceStatus::Blocked)
        );
        assert!(
            select_shell_hostess_staging_acceptance_index_entry(&index, Some("missing")).is_none()
        );

        let selection = summarize_shell_hostess_staging_acceptance_index_selection(
            &index,
            Some(&root.join("shell-hostess-staging-acceptances.json")),
            None,
        );
        assert_eq!(
            selection.schema_id,
            SHELL_HOSTESS_STAGING_ACCEPTANCE_SELECTION_SCHEMA
        );
        assert_eq!(
            selection.source_index_schema,
            SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA
        );
        assert_eq!(
            selection.status,
            StudioShellHostessStagingAcceptanceSelectionStatus::Selected
        );
        assert_eq!(
            selection.selected_acceptance_id.as_deref(),
            Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
        );
        assert_eq!(selection.acceptance_count, 2);
        assert!(selection.entries.iter().any(|entry| entry.acceptance_id
            == "studio.hostess_staging_acceptance.studio.project.test.rev1.ready"
            && entry.selected
            && entry.default
            && entry.ready_item_count == 6));
        let missing_selection = summarize_shell_hostess_staging_acceptance_index_selection(
            &index,
            None,
            Some("missing"),
        );
        assert_eq!(
            missing_selection.status,
            StudioShellHostessStagingAcceptanceSelectionStatus::Missing
        );
        assert_eq!(
            missing_selection.issue_code.as_deref(),
            Some("studio.issue.shell_hostess_staging_acceptance_not_found")
        );
        let empty_index = shell_hostess_staging_acceptance_index_for_manifests(Vec::new(), None);
        let empty_selection =
            summarize_shell_hostess_staging_acceptance_index_selection(&empty_index, None, None);
        assert_eq!(
            empty_selection.status,
            StudioShellHostessStagingAcceptanceSelectionStatus::Empty
        );

        let direct_comparison = compare_shell_hostess_staging_acceptance_against_manifest(
            &ready_acceptance,
            &staging_acceptance,
            &staging_acceptance,
        );
        assert_eq!(
            direct_comparison.schema_id,
            SHELL_HOSTESS_STAGING_ACCEPTANCE_COMPARISON_SCHEMA
        );
        assert_eq!(
            direct_comparison.status,
            StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged
        );
        assert_eq!(
            direct_comparison.baseline_acceptance_id.as_deref(),
            Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
        );
        assert_eq!(direct_comparison.ready_item_delta, 0);
        assert_eq!(direct_comparison.blocked_item_delta, 0);
        assert_eq!(direct_comparison.rejected_item_delta, 0);
        assert_eq!(direct_comparison.entries.len(), 6);
        assert!(direct_comparison
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
        assert!(direct_comparison.entries.iter().all(|entry| {
            entry.change == StudioShellHostessStagingAcceptanceComparisonChange::Unchanged
        }));

        let mut changed_contract_candidate = staging_acceptance.clone();
        changed_contract_candidate
            .entries
            .iter_mut()
            .find(|entry| entry.item_id == "hostess.accept_staging_handoff")
            .expect("acceptance row")
            .owner = "rusty.studio".to_string();
        changed_contract_candidate
            .entries
            .iter_mut()
            .find(|entry| entry.item_id == "hostess.copy_staging_files")
            .expect("copy row")
            .route_kind = "hostess.stage.files_from_drifted_plan".to_string();
        let review_entry = changed_contract_candidate
            .entries
            .iter_mut()
            .find(|entry| entry.item_id == "hostess.review_staging_file_requests")
            .expect("review row");
        review_entry.prohibited_in_studio = false;
        review_entry.expected_input_path = Some("target/drifted-input.json".to_string());
        let changed_contract_comparison = compare_shell_hostess_staging_acceptance_against_manifest(
            &ready_acceptance,
            &staging_acceptance,
            &changed_contract_candidate,
        );
        assert_eq!(
            changed_contract_comparison.status,
            StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable
        );
        assert_eq!(
            changed_contract_comparison.issue_code.as_deref(),
            Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
        );
        assert_eq!(changed_contract_comparison.ready_item_delta, 0);
        assert_eq!(changed_contract_comparison.blocked_item_delta, 0);
        assert_eq!(changed_contract_comparison.rejected_item_delta, 0);
        assert_eq!(
            changed_contract_comparison
                .entries
                .iter()
                .filter(|entry| entry.change
                    == StudioShellHostessStagingAcceptanceComparisonChange::Changed)
                .count(),
            3
        );
        assert!(changed_contract_comparison.entries.iter().all(|entry| {
            entry.change != StudioShellHostessStagingAcceptanceComparisonChange::Changed
                || entry.issue_code.as_deref()
                    == Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
        }));
        assert!(changed_contract_comparison.checks.iter().any(|check| {
            check.check_id
                == "studio.check.shell_hostess_staging_acceptance_comparison.entry_contracts"
                && check.status == StudioValidationStatus::Fail
        }));

        let assert_single_entry_contract_drift =
            |candidate: StudioShellHostessStagingAcceptanceChecklistReport,
             expected_item_id: &str| {
                let comparison = compare_shell_hostess_staging_acceptance_against_manifest(
                    &ready_acceptance,
                    &staging_acceptance,
                    &candidate,
                );
                assert_eq!(
                    comparison.status,
                    StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable
                );
                assert_ne!(
                    comparison.status,
                    StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged
                );
                assert_eq!(
                    comparison.issue_code.as_deref(),
                    Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
                );
                let changed_entries = comparison
                    .entries
                    .iter()
                    .filter(|entry| {
                        entry.change == StudioShellHostessStagingAcceptanceComparisonChange::Changed
                    })
                    .collect::<Vec<_>>();
                assert_eq!(changed_entries.len(), 1);
                assert_eq!(changed_entries[0].item_id, expected_item_id);
                assert_eq!(
                    changed_entries[0].issue_code.as_deref(),
                    Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
                );
                assert!(comparison.checks.iter().any(|check| {
                    check.check_id
                        == "studio.check.shell_hostess_staging_acceptance_comparison.entry_contracts"
                        && check.status == StudioValidationStatus::Fail
                        && check.issue_code.as_deref()
                            == Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
                }));
            };

        let mut owner_drift_candidate = staging_acceptance.clone();
        owner_drift_candidate
            .entries
            .iter_mut()
            .find(|entry| entry.item_id == "hostess.accept_staging_handoff")
            .expect("acceptance row")
            .owner = "rusty.studio".to_string();
        assert_single_entry_contract_drift(owner_drift_candidate, "hostess.accept_staging_handoff");

        let mut route_drift_candidate = staging_acceptance.clone();
        route_drift_candidate
            .entries
            .iter_mut()
            .find(|entry| entry.item_id == "hostess.copy_staging_files")
            .expect("copy row")
            .route_kind = "hostess.stage.files_from_drifted_plan".to_string();
        assert_single_entry_contract_drift(route_drift_candidate, "hostess.copy_staging_files");

        let mut prohibited_drift_candidate = staging_acceptance.clone();
        prohibited_drift_candidate
            .entries
            .iter_mut()
            .find(|entry| entry.item_id == "hostess.review_staging_file_requests")
            .expect("review row")
            .prohibited_in_studio = false;
        assert_single_entry_contract_drift(
            prohibited_drift_candidate,
            "hostess.review_staging_file_requests",
        );

        let mut expected_input_drift_candidate = staging_acceptance.clone();
        expected_input_drift_candidate
            .entries
            .iter_mut()
            .find(|entry| entry.item_id == "hostess.review_staging_file_requests")
            .expect("review row")
            .expected_input_path = Some("target/drifted-input.json".to_string());
        assert_single_entry_contract_drift(
            expected_input_drift_candidate,
            "hostess.review_staging_file_requests",
        );

        let ready_index_entry = select_shell_hostess_staging_acceptance_index_entry(&index, None)
            .expect("select ready Hostess staging acceptance");
        let index_path = root.join("shell-hostess-staging-acceptances.json");
        let index_comparison = compare_shell_hostess_staging_acceptance_against_index_entry(
            &index,
            Some(&index_path),
            ready_index_entry,
            Some(&ready_manifest_path),
            &ready_acceptance,
            &staging_acceptance,
            &staging_acceptance,
        );
        assert_eq!(
            index_comparison.status,
            StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged
        );
        assert_eq!(
            index_comparison
                .baseline_index_selected_acceptance_id
                .as_deref(),
            Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
        );
        assert_eq!(
            index_comparison
                .baseline_index_default_acceptance_id
                .as_deref(),
            Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
        );

        let execution_request = shell_hostess_staging_execution_request_for_acceptance_index_entry(
            &index,
            Some(&index_path),
            ready_index_entry,
            Some(&ready_manifest_path),
            &ready_acceptance,
            &staging_acceptance,
            Some(&handoff_path),
            &envelope,
        );
        assert_eq!(
            execution_request.schema_id,
            SHELL_HOSTESS_STAGING_EXECUTION_REQUEST_SCHEMA
        );
        assert_eq!(
            execution_request.status,
            StudioShellHostessStagingExecutionRequestStatus::Ready
        );
        assert_eq!(execution_request.issue_code, None);
        assert_eq!(
            execution_request.execution_policy,
            "not_executed.hostess_request_only"
        );
        assert_eq!(execution_request.adapter_owner, "rusty.hostess");
        assert_eq!(execution_request.requester_role, "rusty.studio");
        assert_eq!(
            execution_request.command_session_authority.as_deref(),
            Some("rusty.manifold")
        );
        assert_eq!(
            execution_request
                .install_launch_evidence_authority
                .as_deref(),
            Some("rusty.hostess")
        );
        assert_eq!(
            execution_request.selected_acceptance_id,
            "studio.hostess_staging_acceptance.studio.project.test.rev1.ready"
        );
        assert_eq!(
            execution_request.acceptance_manifest_path.as_deref(),
            Some(ready_manifest_path.display().to_string().as_str())
        );
        assert_eq!(
            execution_request.handoff_path.as_deref(),
            Some(handoff_path.display().to_string().as_str())
        );
        assert_eq!(execution_request.adapter_action_count, 6);
        assert_eq!(execution_request.ready_adapter_action_count, 6);
        assert_eq!(execution_request.blocked_adapter_action_count, 0);
        assert!(!execution_request.pmb_shell_handoff_review_required);
        assert_eq!(execution_request.pmb_shell_handoff_review_path, None);
        assert!(!execution_request.pmb_shell_handoff_review_ready);
        assert!(execution_request
            .hostess_operator_start_preflight_cli_args
            .is_empty());
        assert!(execution_request.actions.iter().all(|action| {
            action.status == StudioShellHostessStagingExecutionActionStatus::Ready
                && action.ack_required
                && !action.execution_in_studio
        }));
        assert!(execution_request.actions.iter().any(|action| {
            action.source_item_id == "hostess.copy_staging_files"
                && action.owner == "rusty.hostess"
                && action.route_kind == "hostess.stage.files_from_plan"
        }));
        assert!(execution_request.actions.iter().any(|action| {
            action.source_item_id == "manifold.review_command_session_contract"
                && action.owner == "rusty.manifold"
                && action.route_kind == "manifold.review.command_session_contract"
        }));
        assert_eq!(
            execution_request.ack_template.schema_id,
            SHELL_HOSTESS_STAGING_EXECUTION_ACK_SCHEMA
        );
        assert_eq!(
            execution_request.ack_template.ack_status,
            StudioShellHostessStagingExecutionAckStatus::Pending
        );
        assert!(!execution_request.ack_template.execution_in_studio);
        assert_eq!(
            execution_request.ack_template.required_action_ids.len(),
            execution_request.adapter_action_count
        );
        assert_eq!(
            execution_request.reject_template.schema_id,
            SHELL_HOSTESS_STAGING_EXECUTION_REJECT_SCHEMA
        );
        assert_eq!(
            execution_request.reject_template.reject_status,
            StudioShellHostessStagingExecutionRejectStatus::Pending
        );
        assert!(!execution_request.reject_template.execution_in_studio);

        let pmb_review_path = root.join("target/pmb-shell-handoff.studio-review.json");
        let pmb_review = projected_motion_breath_shell_handoff_review_for_evidence(
            &projected_motion_shell_handoff_evidence(),
            Some(&pmb_review_path),
        );
        let gated_execution_request =
            shell_hostess_staging_execution_request_for_acceptance_index_entry_with_pmb_review(
                &index,
                Some(&index_path),
                ready_index_entry,
                Some(&ready_manifest_path),
                &ready_acceptance,
                &staging_acceptance,
                Some(&handoff_path),
                &envelope,
                Some(&pmb_review_path),
                Some(&pmb_review),
                true,
            );
        assert_eq!(
            gated_execution_request.status,
            StudioShellHostessStagingExecutionRequestStatus::Ready
        );
        assert!(gated_execution_request.pmb_shell_handoff_review_required);
        assert!(gated_execution_request.pmb_shell_handoff_review_ready);
        assert_eq!(
            gated_execution_request
                .pmb_shell_handoff_review_path
                .as_deref(),
            Some(pmb_review_path.display().to_string().as_str())
        );
        assert_eq!(
            gated_execution_request
                .source_pmb_shell_handoff_review_schema
                .as_deref(),
            Some(PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA)
        );
        assert_eq!(
            gated_execution_request.source_pmb_shell_handoff_review_status,
            Some(StudioProjectedMotionBreathShellHandoffReviewStatus::Ready)
        );
        assert_eq!(
            gated_execution_request
                .source_pmb_shell_handoff_id
                .as_deref(),
            Some("shell_handoff.projected_motion_breath.loopback")
        );
        assert_eq!(
            gated_execution_request.hostess_operator_start_preflight_cli_args,
            vec![
                "--pmb-shell-handoff-review-in".to_string(),
                pmb_review_path.display().to_string(),
                "--require-pmb-shell-handoff-review".to_string(),
            ]
        );
        assert!(gated_execution_request.checks.iter().any(|check| {
            check.check_id
                == "studio.check.shell_hostess_staging_execution_request.pmb_shell_handoff_review"
                && check.status == StudioValidationStatus::Pass
        }));

        let missing_pmb_execution_request =
            shell_hostess_staging_execution_request_for_acceptance_index_entry_with_pmb_review(
                &index,
                Some(&index_path),
                ready_index_entry,
                Some(&ready_manifest_path),
                &ready_acceptance,
                &staging_acceptance,
                Some(&handoff_path),
                &envelope,
                None,
                None,
                true,
            );
        assert_eq!(
            missing_pmb_execution_request.status,
            StudioShellHostessStagingExecutionRequestStatus::Blocked
        );
        assert_eq!(
            missing_pmb_execution_request.issue_code.as_deref(),
            Some("studio.issue.projected_motion_breath_shell_handoff_review_missing")
        );
        assert!(missing_pmb_execution_request.pmb_shell_handoff_review_required);
        assert!(!missing_pmb_execution_request.pmb_shell_handoff_review_ready);
        assert_eq!(
            missing_pmb_execution_request.hostess_operator_start_preflight_cli_args,
            vec!["--require-pmb-shell-handoff-review".to_string()]
        );
        assert_eq!(missing_pmb_execution_request.ready_adapter_action_count, 0);
        assert!(missing_pmb_execution_request.checks.iter().any(|check| {
            check.check_id
                == "studio.check.shell_hostess_staging_execution_request.pmb_shell_handoff_review"
                && check.status == StudioValidationStatus::Fail
        }));

        let changed_execution_request =
            shell_hostess_staging_execution_request_for_acceptance_index_entry(
                &index,
                Some(&index_path),
                ready_index_entry,
                Some(&ready_manifest_path),
                &ready_acceptance,
                &changed_contract_candidate,
                Some(&handoff_path),
                &envelope,
            );
        assert_eq!(
            changed_execution_request.status,
            StudioShellHostessStagingExecutionRequestStatus::Blocked
        );
        assert_eq!(
            changed_execution_request.issue_code.as_deref(),
            Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
        );
        assert_eq!(changed_execution_request.ready_adapter_action_count, 0);
        assert_eq!(changed_execution_request.blocked_adapter_action_count, 6);
        assert!(changed_execution_request.checks.iter().any(|check| {
            check.check_id
                == "studio.check.shell_hostess_staging_execution_request.acceptance_entry_contracts"
                && check.status == StudioValidationStatus::Fail
        }));
        assert!(changed_execution_request
            .actions
            .iter()
            .all(|action| !action.execution_in_studio));

        let mut regressed_candidate = staging_acceptance.clone();
        regressed_candidate.status = StudioShellHostessStagingAcceptanceStatus::Blocked;
        regressed_candidate.issue_code =
            Some("studio.issue.shell_hostess_staging_acceptance_blocked".to_string());
        regressed_candidate.ready_item_count = 0;
        regressed_candidate.blocked_item_count = regressed_candidate.entries.len();
        for entry in &mut regressed_candidate.entries {
            entry.status = StudioShellHostessStagingAcceptanceStatus::Blocked;
            entry.issue_code =
                Some("studio.issue.shell_hostess_staging_acceptance_blocked".to_string());
        }
        let regressed_comparison = compare_shell_hostess_staging_acceptance_against_manifest(
            &ready_acceptance,
            &staging_acceptance,
            &regressed_candidate,
        );
        assert_eq!(
            regressed_comparison.status,
            StudioShellHostessStagingAcceptanceComparisonStatus::Regressed
        );
        assert_eq!(
            regressed_comparison.issue_code.as_deref(),
            Some("studio.issue.shell_hostess_staging_acceptance_blocked")
        );
        assert_eq!(regressed_comparison.ready_item_delta, -6);
        assert_eq!(regressed_comparison.blocked_item_delta, 6);
        assert_eq!(
            regressed_comparison
                .entries
                .iter()
                .filter(|entry| entry.change
                    == StudioShellHostessStagingAcceptanceComparisonChange::Regressed)
                .count(),
            6
        );

        let mut stale_identity = ready_acceptance.clone();
        stale_identity.project_id = Some("studio.project.stale".to_string());
        let stale_identity_comparison = compare_shell_hostess_staging_acceptance_against_manifest(
            &stale_identity,
            &staging_acceptance,
            &staging_acceptance,
        );
        assert_eq!(
            stale_identity_comparison.status,
            StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable
        );
        assert_eq!(
            stale_identity_comparison.issue_code.as_deref(),
            Some("studio.issue.shell_hostess_staging_acceptance_identity_mismatch")
        );

        let mut stale_index = index.clone();
        stale_index.entries[0].ready_item_count = 5;
        let stale_index_entry =
            select_shell_hostess_staging_acceptance_index_entry(&stale_index, None)
                .expect("select stale Hostess staging acceptance");
        let stale_index_comparison = compare_shell_hostess_staging_acceptance_against_index_entry(
            &stale_index,
            Some(&index_path),
            stale_index_entry,
            Some(&ready_manifest_path),
            &ready_acceptance,
            &staging_acceptance,
            &staging_acceptance,
        );
        assert_eq!(
            stale_index_comparison.status,
            StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable
        );
        assert_eq!(
            stale_index_comparison.issue_code.as_deref(),
            Some("studio.issue.shell_hostess_staging_acceptance_index_mismatch")
        );
        assert!(stale_index_comparison.checks.iter().any(|check| {
            check.check_id
                == "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_status_counts"
                && check.status == StudioValidationStatus::Fail
        }));

        let appended = append_shell_hostess_staging_acceptance_index_manifests(
            &shell_hostess_staging_acceptance_index_for_manifests(
                vec![(ready_acceptance, Some(ready_manifest_path.clone()))],
                None,
            ),
            vec![(
                blocked_acceptance_manifest,
                Some(blocked_manifest_path.clone()),
            )],
            Some("synthetic-blocked-hostess-acceptance"),
        );
        assert_eq!(
            appended.default_acceptance_id.as_deref(),
            Some("synthetic-blocked-hostess-acceptance")
        );
        let promoted = promote_shell_hostess_staging_acceptance_index_default(
            &appended,
            "studio.hostess_staging_acceptance.studio.project.test.rev1.ready",
        )
        .expect("promote ready Hostess staging acceptance");
        assert_eq!(
            promoted.default_acceptance_id.as_deref(),
            Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
        );
        assert!(
            promote_shell_hostess_staging_acceptance_index_default(&appended, "missing").is_none()
        );
    }

    #[test]
    fn desktop_shell_handoff_rejects_unvalidated_bundle() {
        let root = temp_root("desktop-shell-handoff-reject");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();
        let output_dir = root.join("missing-selected-shell");

        let handoff = desktop_shell_handoff_for_bundle(
            &project,
            Some(&root),
            "studio.graph.test",
            &output_dir,
        );

        assert_eq!(handoff.status, StudioValidationStatus::Fail);
        assert_eq!(
            handoff.issue_code.as_deref(),
            Some("studio.issue.shell_bundle_file_missing")
        );
        assert!(handoff.consumer_args.is_empty());
        assert_eq!(handoff.validation.status, StudioValidationStatus::Fail);
    }

    #[test]
    fn selected_shell_bundle_reports_descriptor_rejection() {
        let root = temp_root("selected-shell-bundle-reject");
        write_reference_fixture_tree(&root);
        let project = valid_project_with_relative_references();

        let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");

        assert_eq!(report.status, StudioShellBundleStatus::Rejected);
        assert_eq!(
            report.issue_code.as_deref(),
            Some("studio.issue.no_operator_shell")
        );
        assert!(report.bundle_files.is_empty());
        assert_eq!(report.descriptor, None);
        assert_eq!(report.template_index, None);
    }

    #[test]
    fn shell_templates_reject_invalid_artifact_manifest() {
        let root = temp_root("shell-template-invalid-manifest");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let report = shell_artifacts_for_project(&project, Some(&root));
        let mut manifest = report.manifest.expect("shell artifact manifest");
        manifest.artifacts[0].descriptor_path = "../outside.json".to_string();

        let template_report = shell_templates_for_artifact_manifest(&manifest, Some(&root));

        assert_eq!(template_report.status, StudioShellTemplateStatus::Rejected);
        assert_eq!(template_report.index, None);
        assert!(template_report.templates.is_empty());
        assert_eq!(
            template_report.issue_code.as_deref(),
            Some("studio.issue.invalid_descriptor_path")
        );
    }

    #[test]
    fn shell_template_index_roundtrips_and_validates_files() {
        let root = temp_root("shell-template-index-roundtrip");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let artifact_report = shell_artifacts_for_project(&project, Some(&root));
        let manifest = artifact_report
            .manifest
            .as_ref()
            .expect("shell artifact manifest");
        for descriptor in &artifact_report.descriptors {
            let descriptor_path = resolve_manifest_relative_path(
                &root,
                &shell_descriptor_artifact_path(&descriptor.graph_id),
            );
            save_json(&descriptor_path, descriptor).expect("save descriptor");
        }
        let template_report = shell_templates_for_artifact_manifest(manifest, Some(&root));
        let index = template_report.index.as_ref().expect("template index");
        for (entry, template) in index.templates.iter().zip(template_report.templates.iter()) {
            save_json(
                &resolve_manifest_relative_path(&root, &entry.template_path),
                template,
            )
            .expect("save template");
        }
        let index_path = root.join("shell-templates.json");
        save_json(&index_path, index).expect("save index");

        let loaded_index = load_shell_template_index(&index_path).expect("load template index");
        let validation = validate_shell_template_index(&loaded_index, Some(&root));

        assert_eq!(loaded_index, *index);
        assert_eq!(
            validation.schema_id,
            SHELL_TEMPLATE_INDEX_VALIDATION_REPORT_SCHEMA
        );
        assert_eq!(validation.status, StudioValidationStatus::Pass);
    }

    #[test]
    fn shell_template_index_validation_rejects_template_mismatch() {
        let root = temp_root("shell-template-index-mismatch");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let artifact_report = shell_artifacts_for_project(&project, Some(&root));
        let manifest = artifact_report
            .manifest
            .as_ref()
            .expect("shell artifact manifest");
        for descriptor in &artifact_report.descriptors {
            let descriptor_path = resolve_manifest_relative_path(
                &root,
                &shell_descriptor_artifact_path(&descriptor.graph_id),
            );
            save_json(&descriptor_path, descriptor).expect("save descriptor");
        }
        let template_report = shell_templates_for_artifact_manifest(manifest, Some(&root));
        let mut index = template_report.index.expect("template index");
        for (entry, template) in index.templates.iter().zip(template_report.templates.iter()) {
            save_json(
                &resolve_manifest_relative_path(&root, &entry.template_path),
                template,
            )
            .expect("save template");
        }
        index.templates[0].shell_id = "shell.synthetic.changed".to_string();

        let validation = validate_shell_template_index(&index, Some(&root));

        assert_eq!(validation.status, StudioValidationStatus::Fail);
        assert!(validation.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.template_shell_mismatch")
        }));
    }

    #[test]
    fn shell_template_index_validation_rejects_path_traversal() {
        let root = temp_root("shell-template-index-path-traversal");
        write_reference_fixture_tree(&root);
        let project = valid_multi_shell_project_with_relative_references();
        let artifact_report = shell_artifacts_for_project(&project, Some(&root));
        let manifest = artifact_report
            .manifest
            .as_ref()
            .expect("shell artifact manifest");
        for descriptor in &artifact_report.descriptors {
            let descriptor_path = resolve_manifest_relative_path(
                &root,
                &shell_descriptor_artifact_path(&descriptor.graph_id),
            );
            save_json(&descriptor_path, descriptor).expect("save descriptor");
        }
        let template_report = shell_templates_for_artifact_manifest(manifest, Some(&root));
        let mut index = template_report.index.expect("template index");
        index.templates[0].template_path = "../outside.json".to_string();

        let validation = validate_shell_template_index(&index, Some(&root));

        assert_eq!(validation.status, StudioValidationStatus::Fail);
        assert!(validation.checks.iter().any(|check| {
            check.issue_code.as_deref() == Some("studio.issue.invalid_template_path")
        }));
    }

    #[test]
    fn shell_artifacts_reject_invalid_graph_descriptor() {
        let root = temp_root("shell-artifacts-reject");
        write_reference_fixture_tree(&root);
        let mut project = valid_multi_shell_project_with_relative_references();
        for node in &mut project.graphs[1].nodes {
            if node.kind == StudioNodeKind::OperatorShell {
                node.kind = StudioNodeKind::ValidationSlot;
            }
        }
        project.graphs[1]
            .edges
            .retain(|edge| edge.kind != StudioEdgeKind::CommandBinding);

        let report = shell_artifacts_for_project(&project, Some(&root));

        assert_eq!(report.status, StudioShellArtifactStatus::Rejected);
        assert_eq!(report.manifest, None);
        assert!(report.descriptors.is_empty());
        assert!(report.rejections.iter().any(|rejection| {
            rejection.graph_id == "studio.graph.phone"
                && rejection.issue_code.as_deref() == Some("studio.issue.no_operator_shell")
        }));
    }

    #[test]
    fn shell_descriptor_rejects_missing_graph() {
        let root = temp_root("shell-descriptor-missing-graph");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();

        let report = shell_descriptor_for_graph(&project, Some(&root), "studio.graph.missing");

        assert_eq!(report.status, StudioShellDescriptorStatus::Rejected);
        assert_eq!(
            report.issue_code.as_deref(),
            Some("studio.issue.graph_missing")
        );
        assert_eq!(report.descriptor, None);
    }

    #[test]
    fn shell_descriptor_rejects_missing_operator_shell() {
        let root = temp_root("shell-descriptor-no-shell");
        write_reference_fixture_tree(&root);
        let project = valid_project_with_relative_references();

        let report = shell_descriptor_for_graph(&project, Some(&root), "studio.graph.test");

        assert_eq!(report.status, StudioShellDescriptorStatus::Rejected);
        assert_eq!(
            report.issue_code.as_deref(),
            Some("studio.issue.no_operator_shell")
        );
        assert_eq!(report.descriptor, None);
    }

    #[test]
    fn retarget_host_profile_updates_host_node_and_bumps_revision() {
        let root = temp_root("retarget-host");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();

        let report = retarget_graph_host_profile(
            &mut project,
            "studio.graph.test",
            "host_run.profile.headset",
            Some(&root),
        );

        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert_eq!(project.revision, 2);
        assert_eq!(
            project.graphs[0].target_host_profile,
            "host_run.profile.headset"
        );
        assert!(project.graphs[0].nodes.iter().any(|node| {
            node.kind == StudioNodeKind::HostProfile
                && node.reference_id == "host_run.profile.headset"
        }));
        assert_eq!(report.validation.status, StudioValidationStatus::Pass);
        assert!(report
            .changed_fields
            .iter()
            .any(|field| field.ends_with("target_host_profile")));
    }

    #[test]
    fn retarget_host_profile_rejects_missing_graph_without_mutating() {
        let root = temp_root("retarget-missing-graph");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();

        let report = retarget_graph_host_profile(
            &mut project,
            "studio.graph.missing",
            "host_run.profile.headset",
            Some(&root),
        );

        assert_eq!(report.status, StudioEditStatus::Rejected);
        assert_eq!(
            report.issue_code.as_deref(),
            Some("studio.issue.graph_missing")
        );
        assert_eq!(project.revision, 1);
        assert_eq!(
            project.graphs[0].target_host_profile,
            "host_run.profile.desktop"
        );
    }

    #[test]
    fn retarget_host_profile_rejects_undeclared_profile_without_mutating() {
        let root = temp_root("retarget-undeclared-profile");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();

        let report = retarget_graph_host_profile(
            &mut project,
            "studio.graph.test",
            "host_run.profile.missing",
            Some(&root),
        );

        assert_eq!(report.status, StudioEditStatus::Rejected);
        assert_eq!(
            report.issue_code.as_deref(),
            Some("studio.issue.host_profile_reference_missing")
        );
        assert_eq!(project.revision, 1);
        assert_eq!(
            project.graphs[0].target_host_profile,
            "host_run.profile.desktop"
        );
    }

    #[test]
    fn add_module_to_graph_adds_module_edge_and_bumps_revision() {
        let root = temp_root("add-module");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();

        let report = add_module_to_graph(
            &mut project,
            "studio.graph.test",
            "package.synthetic",
            "module.synthetic_provider",
            Some("Synthetic Provider"),
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::AddModule);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.requested_reference_id, "module.synthetic_provider");
        assert_eq!(report.requested_host_profile, "host_run.profile.desktop");
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert_eq!(project.revision, 2);
        assert!(project.graphs[0].nodes.iter().any(|node| {
            node.kind == StudioNodeKind::Module
                && node.reference_id == "module.synthetic_provider"
                && node.label == "Synthetic Provider"
        }));
        assert!(project.graphs[0].edges.iter().any(|edge| {
            edge.kind == StudioEdgeKind::PackageProvidesModule
                && edge.source_node_id == "node.package.synthetic"
                && edge.target_node_id == "node.module.synthetic_provider"
        }));
        assert_eq!(report.validation.status, StudioValidationStatus::Pass);
        assert!(report
            .changed_fields
            .iter()
            .any(|field| field.ends_with("nodes.node.module.synthetic_provider")));
        assert!(
            report
                .changed_fields
                .iter()
                .any(|field| field
                    .ends_with("edges.edge.package.synthetic.module.synthetic_provider"))
        );
    }

    #[test]
    fn add_next_catalog_module_to_graph_uses_palette_selection() {
        let root = temp_root("add-next-palette-module");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();

        let report =
            add_next_catalog_module_to_graph(&mut project, "studio.graph.test", Some(&root));

        assert_eq!(report.operation, StudioEditOperation::AddModule);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.requested_reference_id, "module.synthetic_provider");
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert!(project.graphs[0].nodes.iter().any(|node| {
            node.kind == StudioNodeKind::Module && node.reference_id == "module.synthetic_provider"
        }));
        assert!(project.graphs[0].edges.iter().any(|edge| {
            edge.kind == StudioEdgeKind::PackageProvidesModule
                && edge.target_node_id == "node.module.synthetic_provider"
        }));
    }

    #[test]
    fn add_next_catalog_module_from_package_to_graph_uses_selected_package() {
        let root = temp_root("add-next-selected-package-module");
        write_multi_package_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();

        let report = add_next_catalog_module_from_package_to_graph(
            &mut project,
            "studio.graph.test",
            "package.biosignal",
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::AddModule);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.requested_reference_id, "module.biosignal.provider");
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert!(project.graphs[0].nodes.iter().any(|node| {
            node.kind == StudioNodeKind::Package && node.reference_id == "package.biosignal"
        }));
        assert!(project.graphs[0].nodes.iter().any(|node| {
            node.kind == StudioNodeKind::Module && node.reference_id == "module.biosignal.provider"
        }));
        assert!(project.graphs[0].edges.iter().any(|edge| {
            edge.kind == StudioEdgeKind::PackageProvidesModule
                && edge.source_node_id == "node.package.biosignal"
                && edge.target_node_id == "node.module.biosignal.provider"
        }));
        assert!(!project.graphs[0]
            .nodes
            .iter()
            .any(|node| node.reference_id == "module.synthetic_provider"));
    }

    #[test]
    fn add_next_catalog_module_from_package_to_graph_rejects_missing_package() {
        let root = temp_root("add-next-selected-package-missing");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();

        let report = add_next_catalog_module_from_package_to_graph(
            &mut project,
            "studio.graph.test",
            "package.missing",
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::AddModule);
        assert_eq!(report.status, StudioEditStatus::Rejected);
        assert_eq!(
            report.issue_code.as_deref(),
            Some("studio.issue.package_reference_missing")
        );
        assert_eq!(report.requested_reference_id, "package.missing");
        assert_eq!(project.revision, 1);
    }

    #[test]
    fn add_next_catalog_module_to_graph_rejects_when_palette_is_exhausted() {
        let root = temp_root("add-next-palette-module-exhausted");
        write_reference_fixture_tree(&root);
        let mut project = valid_shell_project_with_relative_references();

        let report =
            add_next_catalog_module_to_graph(&mut project, "studio.graph.test", Some(&root));

        assert_eq!(report.operation, StudioEditOperation::AddModule);
        assert_eq!(report.status, StudioEditStatus::Rejected);
        assert_eq!(
            report.issue_code.as_deref(),
            Some("studio.issue.no_available_palette_module")
        );
        assert_eq!(report.requested_reference_id, NEXT_PALETTE_MODULE_REQUEST);
        assert_eq!(project.revision, 1);
    }

    #[test]
    fn add_module_to_graph_is_idempotent_when_link_exists() {
        let root = temp_root("add-module-idempotent");
        write_reference_fixture_tree(&root);
        let mut project = valid_shell_project_with_relative_references();

        let report = add_module_to_graph(
            &mut project,
            "studio.graph.test",
            "package.synthetic",
            "module.synthetic_provider",
            None,
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::AddModule);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 1);
        assert!(report.changed_fields.is_empty());
        assert_eq!(project.revision, 1);
    }

    #[test]
    fn add_module_to_graph_rejects_unexported_module_without_mutating() {
        let root = temp_root("add-module-unexported");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();

        let report = add_module_to_graph(
            &mut project,
            "studio.graph.test",
            "package.synthetic",
            "module.missing",
            None,
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::AddModule);
        assert_eq!(report.status, StudioEditStatus::Rejected);
        assert_eq!(
            report.issue_code.as_deref(),
            Some("studio.issue.package_module_reference_missing")
        );
        assert_eq!(project.revision, 1);
        assert!(!project.graphs[0]
            .nodes
            .iter()
            .any(|node| node.reference_id == "module.missing"));
    }

    #[test]
    fn remove_module_from_graph_removes_module_and_incident_edges() {
        let root = temp_root("remove-module");
        write_reference_fixture_tree(&root);
        let mut project = valid_shell_project_with_relative_references();

        let report = remove_module_from_graph(
            &mut project,
            "studio.graph.test",
            "module.synthetic_provider",
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::RemoveModule);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.requested_reference_id, "module.synthetic_provider");
        assert_eq!(report.requested_host_profile, "host_run.profile.desktop");
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert_eq!(project.revision, 2);
        assert!(!project.graphs[0]
            .nodes
            .iter()
            .any(|node| node.reference_id == "module.synthetic_provider"));
        assert!(!project.graphs[0].edges.iter().any(|edge| {
            edge.source_node_id == "node.module.synthetic_provider"
                || edge.target_node_id == "node.module.synthetic_provider"
        }));
        assert_eq!(report.validation.status, StudioValidationStatus::Pass);
        assert!(report
            .changed_fields
            .iter()
            .any(|field| field.ends_with("nodes.node.module.synthetic_provider")));
        assert!(report
            .changed_fields
            .iter()
            .any(|field| field.ends_with("edges.edge.package_module")));
        assert!(report
            .changed_fields
            .iter()
            .any(|field| field.ends_with("edges.edge.shell_command")));
    }

    #[test]
    fn remove_module_from_graph_is_idempotent_when_module_is_absent() {
        let root = temp_root("remove-module-idempotent");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();

        let report = remove_module_from_graph(
            &mut project,
            "studio.graph.test",
            "module.synthetic_provider",
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::RemoveModule);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 1);
        assert!(report.changed_fields.is_empty());
        assert_eq!(project.revision, 1);
    }

    #[test]
    fn remove_module_from_graph_rejects_missing_graph_without_mutating() {
        let root = temp_root("remove-module-missing-graph");
        write_reference_fixture_tree(&root);
        let mut project = valid_shell_project_with_relative_references();

        let report = remove_module_from_graph(
            &mut project,
            "studio.graph.missing",
            "module.synthetic_provider",
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::RemoveModule);
        assert_eq!(report.status, StudioEditStatus::Rejected);
        assert_eq!(
            report.issue_code.as_deref(),
            Some("studio.issue.graph_missing")
        );
        assert_eq!(project.revision, 1);
        assert!(project.graphs[0]
            .nodes
            .iter()
            .any(|node| node.reference_id == "module.synthetic_provider"));
    }

    #[test]
    fn add_binding_to_graph_adds_command_binding_and_bumps_revision() {
        let root = temp_root("add-binding");
        write_reference_fixture_tree(&root);
        let mut project = valid_shell_project_with_relative_references();
        project.graphs[0]
            .edges
            .retain(|edge| edge.kind != StudioEdgeKind::CommandBinding);

        let report = add_binding_to_graph(
            &mut project,
            "studio.graph.test",
            StudioBindingKind::Command,
            "node.shell.operator",
            "node.module.synthetic_provider",
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::AddBinding);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(
            report.requested_reference_id,
            "edge.command_binding.node.shell.operator.node.module.synthetic_provider"
        );
        assert_eq!(report.requested_host_profile, "host_run.profile.desktop");
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert_eq!(project.revision, 2);
        assert!(project.graphs[0].edges.iter().any(|edge| {
            edge.kind == StudioEdgeKind::CommandBinding
                && edge.source_node_id == "node.shell.operator"
                && edge.target_node_id == "node.module.synthetic_provider"
        }));
        assert_eq!(report.validation.status, StudioValidationStatus::Pass);
        assert!(report.changed_fields.iter().any(|field| {
            field.ends_with(
                "edges.edge.command_binding.node.shell.operator.node.module.synthetic_provider",
            )
        }));
    }

    #[test]
    fn add_binding_to_graph_is_idempotent_when_binding_exists() {
        let root = temp_root("add-binding-idempotent");
        write_reference_fixture_tree(&root);
        let mut project = valid_shell_project_with_relative_references();

        let report = add_binding_to_graph(
            &mut project,
            "studio.graph.test",
            StudioBindingKind::Command,
            "node.shell.operator",
            "node.module.synthetic_provider",
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::AddBinding);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 1);
        assert!(report.changed_fields.is_empty());
        assert_eq!(project.revision, 1);
    }

    #[test]
    fn add_binding_to_graph_rejects_endpoint_kind_mismatch_without_mutating() {
        let root = temp_root("add-binding-kind-mismatch");
        write_reference_fixture_tree(&root);
        let mut project = valid_shell_project_with_relative_references();

        let report = add_binding_to_graph(
            &mut project,
            "studio.graph.test",
            StudioBindingKind::Stream,
            "node.shell.operator",
            "node.module.synthetic_provider",
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::AddBinding);
        assert_eq!(report.status, StudioEditStatus::Rejected);
        assert_eq!(
            report.issue_code.as_deref(),
            Some("studio.issue.binding_endpoint_kind_mismatch")
        );
        assert_eq!(project.revision, 1);
        assert!(!project.graphs[0].edges.iter().any(|edge| {
            edge.kind == StudioEdgeKind::StreamBinding
                && edge.source_node_id == "node.shell.operator"
                && edge.target_node_id == "node.module.synthetic_provider"
        }));
    }

    #[test]
    fn remove_binding_from_graph_removes_matching_binding() {
        let root = temp_root("remove-binding");
        write_reference_fixture_tree(&root);
        let mut project = valid_shell_project_with_relative_references();

        let report = remove_binding_from_graph(
            &mut project,
            "studio.graph.test",
            StudioBindingKind::Command,
            "node.shell.operator",
            "node.module.synthetic_provider",
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::RemoveBinding);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(
            report.requested_reference_id,
            "edge.command_binding.node.shell.operator.node.module.synthetic_provider"
        );
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert_eq!(project.revision, 2);
        assert!(!project.graphs[0].edges.iter().any(|edge| {
            edge.kind == StudioEdgeKind::CommandBinding
                && edge.source_node_id == "node.shell.operator"
                && edge.target_node_id == "node.module.synthetic_provider"
        }));
        assert_eq!(report.validation.status, StudioValidationStatus::Pass);
        assert!(report
            .changed_fields
            .iter()
            .any(|field| field.ends_with("edges.edge.shell_command")));
    }

    #[test]
    fn remove_binding_from_graph_is_idempotent_when_binding_is_absent() {
        let root = temp_root("remove-binding-idempotent");
        write_reference_fixture_tree(&root);
        let mut project = valid_shell_project_with_relative_references();
        project.graphs[0]
            .edges
            .retain(|edge| edge.kind != StudioEdgeKind::CommandBinding);

        let report = remove_binding_from_graph(
            &mut project,
            "studio.graph.test",
            StudioBindingKind::Command,
            "node.shell.operator",
            "node.module.synthetic_provider",
            Some(&root),
        );

        assert_eq!(report.operation, StudioEditOperation::RemoveBinding);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 1);
        assert!(report.changed_fields.is_empty());
        assert_eq!(project.revision, 1);
    }

    #[test]
    fn view_model_projects_graph_rows_for_ui() {
        let root = temp_root("view-model");
        write_reference_fixture_tree(&root);
        let project = valid_project_with_relative_references();
        let model = view_model(&project, Some(&root));
        assert_eq!(model.schema_id, VIEW_MODEL_SCHEMA);
        assert_eq!(model.validation_status, StudioValidationStatus::Pass);
        assert_eq!(model.validation_fail_count, 0);
        assert!(model.validation_issues.is_empty());
        assert!(model.focused_issue.is_none());
        assert_eq!(model.requested_issue_check_id, None);
        assert_eq!(model.selected_issue_index, None);
        assert_eq!(model.selected_issue_check_id, None);
        assert_eq!(model.issue_selection_code, None);
        assert_eq!(model.requested_node_id, None);
        assert_eq!(
            model.selected_node_id.as_deref(),
            Some("node.package.synthetic")
        );
        assert_eq!(model.node_selection_code, None);
        let selected_node = model.selected_node.as_ref().expect("selected node");
        assert_eq!(selected_node.kind, "package");
        assert_eq!(selected_node.reference_id, "package.synthetic");
        assert_eq!(selected_node.reference_status, "resolved");
        assert_eq!(
            selected_node.package_manifest_path.as_deref(),
            Some("packages/synthetic/manifests/package.manifold.json")
        );
        assert_eq!(
            selected_node.package_module_ids,
            vec!["module.synthetic_provider".to_string()]
        );
        assert_eq!(model.requested_edge_id, None);
        assert_eq!(model.selected_edge_id.as_deref(), Some("edge.package_host"));
        assert_eq!(model.edge_selection_code, None);
        let selected_edge = model.selected_edge.as_ref().expect("selected edge");
        assert_eq!(selected_edge.kind, "shell_targets_host_profile");
        assert_eq!(selected_edge.endpoint_status, "endpoints_resolved");
        assert_eq!(
            selected_edge.source_reference_id.as_deref(),
            Some("package.synthetic")
        );
        assert_eq!(
            selected_edge.target_reference_id.as_deref(),
            Some("host_run.profile.desktop")
        );
        assert_eq!(model.graph_count, 1);
        assert_eq!(model.graphs[0].validation_issue_count, 0);
        assert_eq!(model.graphs[0].node_rows[0].kind, "package");
        assert_eq!(model.graphs[0].node_rows[0].validation_issue_count, 0);
        assert_eq!(
            model.graphs[0].edge_rows[0].kind,
            "shell_targets_host_profile"
        );
        assert_eq!(model.graphs[0].edge_rows[0].validation_issue_count, 0);
        let layout = model.graphs[0].layout.as_ref().expect("graph layout");
        assert_eq!(layout.layout_id, "studio.layout.test");
        assert_eq!(layout.coordinate_space, "studio.canvas.logical_2d");
        assert_eq!(layout.node_count, 2);
        assert_eq!(layout.edge_count, 1);
        assert_eq!(layout.nodes[0].node_id, "node.package.synthetic");
        assert_eq!(layout.nodes[0].x, 40);
        assert_eq!(layout.nodes[0].width, 180);
        assert_eq!(layout.nodes[0].validation_issue_count, 0);
        assert_eq!(layout.edges[0].edge_id, "edge.package_host");
        assert_eq!(layout.edges[0].route, "direct");
    }

    #[test]
    fn view_model_includes_selected_shell_preview() {
        let root = temp_root("view-model-shell-preview");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();

        let model = view_model(&project, Some(&root));
        let preview = model.shell_preview.as_ref().expect("shell preview");

        assert_eq!(preview.status, StudioShellDescriptorStatus::Exported);
        assert_eq!(preview.issue_code, None);
        assert_eq!(
            preview.descriptor_id.as_deref(),
            Some("studio.shell_descriptor.studio.graph.test")
        );
        assert_eq!(
            preview.descriptor_path.as_deref(),
            Some("descriptors/studio.graph.test.shell-descriptor.json")
        );
        assert_eq!(
            preview.shell_id.as_deref(),
            Some("shell.synthetic.operator")
        );
        assert_eq!(
            preview.target_host_profile.as_deref(),
            Some("host_run.profile.desktop")
        );
        assert_eq!(preview.target_kind, Some(StudioShellTargetKind::Desktop));
        assert_eq!(preview.package_count, 1);
        assert_eq!(preview.module_count, 1);
        assert_eq!(preview.stream_binding_count, 0);
        assert_eq!(preview.command_binding_count, 1);
        assert_eq!(
            preview.descriptor_validation_status,
            Some(StudioValidationStatus::Pass)
        );
        assert_eq!(
            preview.template_id.as_deref(),
            Some("studio.shell_template.studio.graph.test")
        );
        assert_eq!(
            preview.template_path.as_deref(),
            Some("shells/desktop/studio.graph.test.shell-template.json")
        );
        assert_eq!(
            preview.template_descriptor_path.as_deref(),
            Some("descriptors/studio.graph.test.shell-descriptor.json")
        );
        assert_eq!(
            preview.runtime_command_authority.as_deref(),
            Some("rusty.manifold")
        );
        assert_eq!(
            preview.runtime_host_authority.as_deref(),
            Some("rusty.hostess")
        );
        assert_eq!(
            preview.studio_role.as_deref(),
            Some("authoring.export_planning")
        );
    }

    #[test]
    fn view_model_shell_preview_reports_descriptor_rejection() {
        let root = temp_root("view-model-shell-preview-rejected");
        write_reference_fixture_tree(&root);
        let project = valid_project_with_relative_references();

        let model = view_model(&project, Some(&root));
        let preview = model.shell_preview.as_ref().expect("shell preview");

        assert_eq!(preview.status, StudioShellDescriptorStatus::Rejected);
        assert_eq!(
            preview.issue_code.as_deref(),
            Some("studio.issue.no_operator_shell")
        );
        assert_eq!(preview.descriptor_id, None);
        assert_eq!(preview.template_id, None);
    }

    #[test]
    fn view_model_includes_reference_palette_for_ui() {
        let root = temp_root("view-model-palette");
        write_reference_fixture_tree(&root);
        let project = valid_project_with_relative_references();

        let model = view_model(&project, Some(&root));

        assert_eq!(model.catalog_package_count, 1);
        assert_eq!(model.catalog_module_count, 1);
        assert_eq!(model.host_profile_count, 2);
        assert_eq!(model.catalog_packages.len(), 1);
        assert_eq!(model.catalog_packages[0].package_id, "package.synthetic");
        assert_eq!(
            model.catalog_packages[0].manifest_path,
            "packages/synthetic/manifests/package.manifold.json"
        );
        assert_eq!(
            model.catalog_packages[0].module_ids,
            vec!["module.synthetic_provider".to_string()]
        );
        assert!(model.catalog_packages[0].in_selected_graph);

        let desktop = model
            .host_profiles
            .iter()
            .find(|profile| profile.profile_id == "host_run.profile.desktop")
            .expect("desktop profile");
        assert_eq!(desktop.host_profile.as_deref(), Some("host.desktop"));
        assert_eq!(
            desktop.install_route.as_deref(),
            Some("install.local_process")
        );
        assert!(desktop.targets_selected_graph);

        let headset = model
            .host_profiles
            .iter()
            .find(|profile| profile.profile_id == "host_run.profile.headset")
            .expect("headset profile");
        assert_eq!(headset.host_profile.as_deref(), Some("host.headset"));
        assert!(!headset.targets_selected_graph);
    }

    #[test]
    fn view_model_includes_failed_validation_diagnostics() {
        let root = temp_root("view-model-validation-diagnostics");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();
        project.graphs[0].nodes[0].reference_id = "package.missing".to_string();

        let model = view_model(&project, Some(&root));

        assert_eq!(model.validation_status, StudioValidationStatus::Fail);
        assert!(model.validation_fail_count > 0);
        let issue = model
            .validation_issues
            .iter()
            .find(|issue| {
                issue.issue_code.as_deref() == Some("studio.issue.package_reference_missing")
            })
            .expect("package reference issue");
        assert_eq!(
            issue.check_id,
            "studio.check.graph.studio.graph.test.package_refs"
        );
        assert!(issue
            .evidence
            .contains("package references missing from catalog"));
        assert_eq!(issue.graph_id.as_deref(), Some("studio.graph.test"));
        assert_eq!(issue.node_ids, vec!["node.package.synthetic".to_string()]);
        assert_eq!(issue.reference_ids, vec!["package.missing".to_string()]);
        assert!(issue.targets_selected_graph);
        assert_eq!(model.graphs[0].validation_issue_count, 1);
        let package_row = model.graphs[0]
            .node_rows
            .iter()
            .find(|node| node.node_id == "node.package.synthetic")
            .expect("package node row");
        assert_eq!(package_row.validation_issue_count, 1);
        let focused_issue = model.focused_issue.expect("focused issue");
        assert_eq!(focused_issue.issue_index, 0);
        assert_eq!(
            focused_issue.check_id,
            "studio.check.graph.studio.graph.test.package_refs"
        );
        assert_eq!(
            focused_issue.issue_code.as_deref(),
            Some("studio.issue.package_reference_missing")
        );
        assert_eq!(focused_issue.graph_id, "studio.graph.test");
        assert_eq!(
            focused_issue.node_id.as_deref(),
            Some("node.package.synthetic")
        );
        assert_eq!(focused_issue.edge_id, None);
        assert_eq!(
            focused_issue.reference_id.as_deref(),
            Some("package.missing")
        );
        assert_eq!(model.requested_issue_check_id, None);
        assert_eq!(model.selected_issue_index, Some(0));
        assert_eq!(
            model.selected_issue_check_id.as_deref(),
            Some("studio.check.graph.studio.graph.test.package_refs")
        );
        assert_eq!(model.issue_selection_code, None);
        assert_eq!(
            model.selected_node_id.as_deref(),
            Some("node.package.synthetic")
        );
        let selected_node = model.selected_node.as_ref().expect("selected node");
        assert_eq!(selected_node.reference_id, "package.missing");
        assert_eq!(selected_node.reference_status, "missing");
        assert_eq!(selected_node.validation_issue_count, 1);
    }

    #[test]
    fn view_model_selects_focused_edge_for_inspector() {
        let root = temp_root("view-model-focused-edge");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();
        project.graphs[0].edges[0].target_node_id = "node.missing".to_string();

        let model = view_model_for_graph_issue_node_and_edge(
            &project,
            Some(&root),
            Some("studio.graph.test"),
            None,
            None,
            None,
        );

        assert_eq!(
            model.selected_issue_check_id.as_deref(),
            Some("studio.check.graph.studio.graph.test.edge.edge.package_host.target")
        );
        assert_eq!(model.selected_edge_id.as_deref(), Some("edge.package_host"));
        let selected_edge = model.selected_edge.as_ref().expect("selected edge");
        assert_eq!(selected_edge.endpoint_status, "missing_target");
        assert_eq!(selected_edge.validation_issue_count, 1);
        assert_eq!(selected_edge.target_node_id, "node.missing");
        assert_eq!(selected_edge.target_kind, None);
    }

    #[test]
    fn view_model_selects_requested_validation_issue() {
        let root = temp_root("view-model-requested-issue");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();
        project.graphs[0].nodes[0].reference_id = "package.missing".to_string();
        project.graphs[0].nodes.push(StudioNode {
            node_id: "node.module.missing".to_string(),
            kind: StudioNodeKind::Module,
            reference_id: "module.missing".to_string(),
            label: "Missing Module".to_string(),
        });

        let model = view_model_for_graph_and_issue(
            &project,
            Some(&root),
            Some("studio.graph.test"),
            Some("studio.check.graph.studio.graph.test.module_refs"),
        );

        assert_eq!(
            model.requested_issue_check_id.as_deref(),
            Some("studio.check.graph.studio.graph.test.module_refs")
        );
        assert_eq!(model.issue_selection_code, None);
        assert_eq!(model.selected_issue_index, Some(1));
        assert_eq!(
            model.selected_issue_check_id.as_deref(),
            Some("studio.check.graph.studio.graph.test.module_refs")
        );
        let focused_issue = model.focused_issue.expect("focused issue");
        assert_eq!(focused_issue.issue_index, 1);
        assert_eq!(
            focused_issue.issue_code.as_deref(),
            Some("studio.issue.module_reference_missing")
        );
        assert_eq!(
            focused_issue.node_id.as_deref(),
            Some("node.module.missing")
        );
        assert_eq!(
            focused_issue.reference_id.as_deref(),
            Some("module.missing")
        );
        assert_eq!(
            model.selected_node_id.as_deref(),
            Some("node.module.missing")
        );
        let selected_node = model.selected_node.as_ref().expect("selected node");
        assert_eq!(selected_node.kind, "module");
        assert_eq!(selected_node.reference_status, "missing");
    }

    #[test]
    fn view_model_selects_requested_node_for_inspector() {
        let root = temp_root("view-model-requested-node");
        write_reference_fixture_tree(&root);
        let project = valid_project_with_relative_references();

        let model = view_model_for_graph_issue_and_node(
            &project,
            Some(&root),
            Some("studio.graph.test"),
            None,
            Some("node.host.desktop"),
        );

        assert_eq!(
            model.requested_node_id.as_deref(),
            Some("node.host.desktop")
        );
        assert_eq!(model.selected_node_id.as_deref(), Some("node.host.desktop"));
        assert_eq!(model.node_selection_code, None);
        let selected_node = model.selected_node.as_ref().expect("selected node");
        assert_eq!(selected_node.kind, "host_profile");
        assert_eq!(selected_node.reference_status, "resolved");
        let profile = selected_node
            .host_profile
            .as_ref()
            .expect("host profile details");
        assert_eq!(profile.profile_id, "host_run.profile.desktop");
        assert_eq!(profile.host_profile.as_deref(), Some("host.desktop"));
        assert_eq!(
            profile.install_route.as_deref(),
            Some("install.local_process")
        );
    }

    #[test]
    fn view_model_reports_missing_requested_node() {
        let root = temp_root("view-model-missing-requested-node");
        write_reference_fixture_tree(&root);
        let project = valid_project_with_relative_references();

        let model = view_model_for_graph_issue_and_node(
            &project,
            Some(&root),
            Some("studio.graph.test"),
            None,
            Some("node.missing"),
        );

        assert_eq!(model.requested_node_id.as_deref(), Some("node.missing"));
        assert_eq!(
            model.node_selection_code.as_deref(),
            Some("studio.issue.node_selection_missing")
        );
        assert_eq!(
            model.selected_node_id.as_deref(),
            Some("node.package.synthetic")
        );
    }

    #[test]
    fn view_model_selects_requested_edge_for_inspector() {
        let root = temp_root("view-model-requested-edge");
        write_reference_fixture_tree(&root);
        let project = valid_shell_project_with_relative_references();

        let model = view_model_for_graph_issue_node_and_edge(
            &project,
            Some(&root),
            Some("studio.graph.test"),
            None,
            None,
            Some("edge.shell_command"),
        );

        assert_eq!(
            model.requested_edge_id.as_deref(),
            Some("edge.shell_command")
        );
        assert_eq!(
            model.selected_edge_id.as_deref(),
            Some("edge.shell_command")
        );
        assert_eq!(model.edge_selection_code, None);
        let selected_edge = model.selected_edge.as_ref().expect("selected edge");
        assert_eq!(selected_edge.kind, "command_binding");
        assert_eq!(selected_edge.binding_kind.as_deref(), Some("command"));
        assert_eq!(selected_edge.endpoint_status, "endpoints_resolved");
        assert_eq!(
            selected_edge.source_reference_id.as_deref(),
            Some("shell.synthetic.operator")
        );
        assert_eq!(
            selected_edge.target_reference_id.as_deref(),
            Some("module.synthetic_provider")
        );
    }

    #[test]
    fn view_model_reports_missing_requested_edge() {
        let root = temp_root("view-model-missing-requested-edge");
        write_reference_fixture_tree(&root);
        let project = valid_project_with_relative_references();

        let model = view_model_for_graph_issue_node_and_edge(
            &project,
            Some(&root),
            Some("studio.graph.test"),
            None,
            None,
            Some("edge.missing"),
        );

        assert_eq!(model.requested_edge_id.as_deref(), Some("edge.missing"));
        assert_eq!(
            model.edge_selection_code.as_deref(),
            Some("studio.issue.edge_selection_missing")
        );
        assert_eq!(model.selected_edge_id.as_deref(), Some("edge.package_host"));
    }

    #[test]
    fn view_model_reports_missing_requested_validation_issue() {
        let root = temp_root("view-model-missing-requested-issue");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();
        project.graphs[0].nodes[0].reference_id = "package.missing".to_string();

        let model = view_model_for_graph_and_issue(
            &project,
            Some(&root),
            Some("studio.graph.test"),
            Some("studio.check.graph.studio.graph.test.missing"),
        );

        assert_eq!(
            model.requested_issue_check_id.as_deref(),
            Some("studio.check.graph.studio.graph.test.missing")
        );
        assert_eq!(
            model.issue_selection_code.as_deref(),
            Some("studio.issue.validation_issue_selection_missing")
        );
        assert_eq!(model.selected_issue_index, Some(0));
        assert_eq!(
            model.selected_issue_check_id.as_deref(),
            Some("studio.check.graph.studio.graph.test.package_refs")
        );
    }

    #[test]
    fn view_model_selects_requested_graph() {
        let root = temp_root("view-model-select");
        write_reference_fixture_tree(&root);
        let mut project = valid_project_with_relative_references();
        let mut second = project.graphs[0].clone();
        second.graph_id = "studio.graph.second".to_string();
        second.display_name = "Second Graph".to_string();
        project.graphs.push(second);

        let model = view_model_for_graph(&project, Some(&root), Some("studio.graph.second"));
        assert_eq!(model.graph_count, 2);
        assert_eq!(
            model.requested_graph_id.as_deref(),
            Some("studio.graph.second")
        );
        assert_eq!(model.selected_graph_index, Some(1));
        assert_eq!(
            model.selected_graph_id.as_deref(),
            Some("studio.graph.second")
        );
        assert_eq!(model.selection_issue_code, None);
    }

    #[test]
    fn view_model_reports_missing_requested_graph() {
        let root = temp_root("view-model-missing-select");
        write_reference_fixture_tree(&root);
        let project = valid_project_with_relative_references();

        let model = view_model_for_graph(&project, Some(&root), Some("studio.graph.missing"));
        assert_eq!(
            model.requested_graph_id.as_deref(),
            Some("studio.graph.missing")
        );
        assert_eq!(model.selected_graph_index, None);
        assert_eq!(model.selected_graph_id, None);
        assert_eq!(
            model.selection_issue_code.as_deref(),
            Some("studio.issue.graph_selection_missing")
        );
    }
}
