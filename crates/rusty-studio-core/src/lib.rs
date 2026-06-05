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
mod project_references;
mod projected_motion_breath;
mod shell_contract;
mod shell_descriptor;
mod shell_export_package;
mod shell_handoff;
mod shell_handoff_acceptance;
mod shell_hostess_handoff;
mod shell_hostess_staging_acceptance;
mod shell_hostess_staging_plan;
mod shell_release_candidate;
mod shell_shared;
mod shell_templates;
mod validation_helpers;
mod view_model;

pub use error::StudioCoreError;
pub use graph_edit::*;
pub use id_grammar::is_dotted_id;
pub use io::*;
pub(crate) use project_references::{
    reference_index_for_project, selected_node_reference_ids, validate_graph_references,
    validate_project_references, CatalogModuleSelection, ReferenceIndex,
};
#[cfg(test)]
pub(crate) use projected_motion_breath::PROJECTED_MOTION_BREATH_PACKAGE_ID;
pub use projected_motion_breath::{
    package_evidence_intake_for_validation_report,
    projected_motion_breath_adapter_normalization_evidence_review_for_selection,
    projected_motion_breath_authoring_review_for_intake,
    projected_motion_breath_shell_handoff_review_for_evidence,
    projected_motion_breath_source_adapter_selection_review_for_authoring,
};
pub(crate) use shell_contract::resolve_manifest_relative_path;
pub(crate) use shell_contract::{
    is_safe_relative_manifest_path, relative_output_path, shell_artifact_for_descriptor,
    shell_handoff_manifest_id, shell_manifold_handoff_artifact_path, shell_runtime_authority,
    shell_target_kind,
};
pub use shell_contract::{
    save_shell_bundle, selected_shell_bundle_for_graph, shell_artifacts_for_project,
    validate_selected_shell_bundle, validate_shell_artifact_manifest,
};
pub(crate) use shell_descriptor::shell_host_profile;
pub use shell_descriptor::{
    shell_descriptor_artifact_path, shell_descriptor_for_graph, validate_shell_descriptor,
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
pub(crate) use shell_shared::{
    count_delta, path_ends_with_shell_templates, runtime_authority_matches, same_unique_strings,
    shell_handoff_consumer_id, shell_handoff_kind_for_target, shell_target_kind_label,
    shell_target_kinds, string_set, unique_strings,
};
pub use shell_templates::{
    shell_template_descriptor_path, shell_template_manifest_path,
    shell_templates_for_artifact_manifest, validate_shell_template_index,
};
pub(crate) use shell_templates::{shell_template_for_artifact, shell_template_index_entry};
pub(crate) use validation_helpers::{
    first_failed_check_issue_code, first_failed_issue_code,
    first_failed_shell_artifact_manifest_issue_code,
    first_failed_shell_bundle_validation_issue_code, first_failed_shell_template_index_issue_code,
    first_failed_validation_check_issue_code, push_check, push_contextual_check,
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

fn edge_duplicates(edges: &[StudioEdge]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for edge in edges {
        *counts.entry(edge.edge_id.clone()).or_insert(0) += 1;
    }
    counts.retain(|_, count| *count > 1);
    counts
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

#[cfg(test)]
mod tests;
