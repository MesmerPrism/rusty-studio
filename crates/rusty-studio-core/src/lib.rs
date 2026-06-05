mod model_prelude;

use id_grammar::{all_dotted_ids, optional_dotted_id};
use model_prelude::*;

use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

mod error;
mod graph_edit;
mod id_grammar;
mod io;
mod projected_motion_breath;
mod shell_contract;
mod shell_export_package;
mod shell_handoff;
mod shell_handoff_acceptance;
mod shell_hostess_handoff;
mod shell_hostess_staging_acceptance;
mod shell_hostess_staging_plan;
mod shell_release_candidate;
mod view_model;

pub use error::StudioCoreError;
pub use graph_edit::*;
pub use id_grammar::is_dotted_id;
pub use io::*;
#[cfg(test)]
pub(crate) use projected_motion_breath::PROJECTED_MOTION_BREATH_PACKAGE_ID;
pub use projected_motion_breath::{
    package_evidence_intake_for_validation_report,
    projected_motion_breath_adapter_normalization_evidence_review_for_selection,
    projected_motion_breath_authoring_review_for_intake,
    projected_motion_breath_shell_handoff_review_for_evidence,
    projected_motion_breath_source_adapter_selection_review_for_authoring,
};
#[cfg(test)]
pub(crate) use shell_contract::resolve_manifest_relative_path;
pub(crate) use shell_contract::{
    is_safe_relative_manifest_path, relative_output_path, shell_artifact_for_descriptor,
    shell_handoff_manifest_id, shell_host_profile, shell_manifold_handoff_artifact_path,
    shell_runtime_authority, shell_target_kind, shell_template_for_artifact,
    shell_template_index_entry,
};
pub use shell_contract::{
    save_shell_bundle, selected_shell_bundle_for_graph, shell_artifacts_for_project,
    shell_descriptor_artifact_path, shell_descriptor_for_graph, shell_template_descriptor_path,
    shell_template_manifest_path, shell_templates_for_artifact_manifest,
    validate_selected_shell_bundle, validate_shell_artifact_manifest, validate_shell_descriptor,
    validate_shell_template_index,
};
pub use shell_export_package::{
    append_shell_export_package_baseline_index_manifests, compare_shell_export_packages,
    compare_shell_export_packages_against_baseline_index_entry,
    compare_shell_export_packages_against_baseline_manifest,
    promote_shell_export_package_baseline_index_default,
    select_shell_export_package_baseline_index_entry,
    shell_export_package_baseline_index_for_manifests,
    shell_export_package_baseline_manifest_for_report, shell_export_package_for_manifest,
    shell_export_package_for_project, summarize_shell_export_package_baseline_index_selection,
};
#[cfg(test)]
pub(crate) use shell_handoff::empty_shell_host_routes;
pub use shell_handoff::{
    desktop_shell_handoff_for_bundle, shell_handoff_for_bundle, shell_handoff_intake_for_manifest,
    shell_handoff_manifest_for_project, shell_handoff_readiness_for_project,
    shell_runbook_for_manifest, shell_runbook_for_project, validate_shell_handoff_manifest,
};
pub(crate) use shell_handoff_acceptance::shell_handoff_acceptance_prohibited_actions;
pub use shell_handoff_acceptance::{
    append_shell_handoff_acceptance_baseline_index_manifests,
    compare_shell_handoff_acceptance_against_baseline_index_entry,
    compare_shell_handoff_acceptance_against_baseline_manifest,
    compare_shell_handoff_acceptance_checklists,
    promote_shell_handoff_acceptance_baseline_index_default,
    select_shell_handoff_acceptance_baseline_index_entry,
    shell_handoff_acceptance_baseline_index_for_manifests,
    shell_handoff_acceptance_baseline_manifest_for_checklist,
    shell_handoff_acceptance_checklist_for_intake, shell_handoff_acceptance_checklist_for_project,
    summarize_shell_handoff_acceptance_baseline_index_selection,
    summarize_shell_handoff_acceptance_checklist,
};
pub use shell_hostess_handoff::{
    shell_hostess_handoff_package_for_release_candidate_index,
    shell_hostess_owner_intake_for_handoff_package,
};
pub use shell_hostess_staging_acceptance::{
    append_shell_hostess_staging_acceptance_index_manifests,
    compare_shell_hostess_staging_acceptance_against_index_entry,
    compare_shell_hostess_staging_acceptance_against_manifest,
    compare_shell_hostess_staging_acceptance_checklists,
    promote_shell_hostess_staging_acceptance_index_default,
    select_shell_hostess_staging_acceptance_index_entry,
    shell_hostess_staging_acceptance_checklist_for_handoff,
    shell_hostess_staging_acceptance_index_for_manifests,
    shell_hostess_staging_acceptance_manifest_for_checklist,
    shell_hostess_staging_execution_request_for_acceptance_index_entry,
    shell_hostess_staging_execution_request_for_acceptance_index_entry_with_pmb_review,
    summarize_shell_hostess_staging_acceptance_index_selection,
};
pub use shell_hostess_staging_plan::{
    shell_hostess_staging_file_plan_for_preview,
    shell_hostess_staging_handoff_envelope_for_file_plan,
    shell_hostess_staging_preview_for_owner_intake,
};
pub use shell_release_candidate::{
    append_shell_release_candidate_review_index_manifests,
    promote_shell_release_candidate_review_index_default,
    select_shell_release_candidate_review_index_entry, shell_release_candidate_review_for_manifest,
    shell_release_candidate_review_index_for_manifests,
    shell_release_candidate_review_manifest_for_report,
    summarize_shell_release_candidate_review_index_selection,
};
pub use view_model::*;

const NEXT_PALETTE_MODULE_REQUEST: &str = "module.palette.next_available";
const MANIFOLD_SHELL_HANDOFF_SCHEMA: &str = "rusty.manifold.shell.handoff.v1";
const DEFAULT_MANIFOLD_SHELL_HANDOFF_VALIDATION_SLOT_ID: &str = "host_run.slot.synthetic_smoke";
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

fn first_failed_validation_check_issue_code(checks: &[StudioValidationCheck]) -> Option<String> {
    checks
        .iter()
        .find(|check| check.status == StudioValidationStatus::Fail)
        .and_then(|check| check.issue_code.clone())
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

#[cfg(test)]
mod tests;
