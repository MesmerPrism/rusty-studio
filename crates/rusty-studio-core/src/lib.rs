use rusty_studio_model::{
    StudioBindingKind, StudioEdge, StudioEdgeKind, StudioEdgeRouteKind, StudioEditOperation,
    StudioEditReport, StudioEditStatus, StudioExportBundle, StudioExportPlan, StudioGraph,
    StudioHostProfileView, StudioIssueFocusView, StudioNode, StudioNodeKind, StudioProject,
    StudioResolvedGraph, StudioResolvedProject, StudioShellArtifact, StudioShellArtifactManifest,
    StudioShellArtifactManifestValidationReport, StudioShellArtifactRejection,
    StudioShellArtifactReport, StudioShellArtifactStatus, StudioShellBinding,
    StudioShellBundleReport, StudioShellBundleStatus, StudioShellBundleValidationReport,
    StudioShellDescriptor, StudioShellDescriptorReport, StudioShellDescriptorStatus,
    StudioShellDescriptorValidationReport, StudioShellHandoffKind, StudioShellHandoffManifest,
    StudioShellHandoffManifestEntry, StudioShellHandoffManifestTarget,
    StudioShellHandoffManifestValidationReport, StudioShellHandoffReadinessEntry,
    StudioShellHandoffReadinessReport, StudioShellHandoffReadinessTargetSummary,
    StudioShellHandoffReport, StudioShellHostProfile, StudioShellHostRoutes,
    StudioShellRuntimeAuthority, StudioShellTargetKind, StudioShellTemplateIndex,
    StudioShellTemplateIndexEntry, StudioShellTemplateIndexValidationReport,
    StudioShellTemplateManifest, StudioShellTemplateReport, StudioShellTemplateStatus,
    StudioValidationCheck, StudioValidationIssueView, StudioValidationReport,
    StudioValidationStatus, StudioViewModel, EDIT_REPORT_SCHEMA, EXPORT_PLAN_SCHEMA,
    PROJECT_SCHEMA, RESOLVED_PROJECT_SCHEMA, SHELL_ARTIFACT_MANIFEST_SCHEMA,
    SHELL_ARTIFACT_MANIFEST_VALIDATION_REPORT_SCHEMA, SHELL_ARTIFACT_REPORT_SCHEMA,
    SHELL_BUNDLE_REPORT_SCHEMA, SHELL_BUNDLE_VALIDATION_REPORT_SCHEMA,
    SHELL_DESCRIPTOR_REPORT_SCHEMA, SHELL_DESCRIPTOR_SCHEMA,
    SHELL_DESCRIPTOR_VALIDATION_REPORT_SCHEMA, SHELL_HANDOFF_MANIFEST_SCHEMA,
    SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA, SHELL_HANDOFF_READINESS_REPORT_SCHEMA,
    SHELL_HANDOFF_REPORT_SCHEMA, SHELL_TEMPLATE_INDEX_SCHEMA,
    SHELL_TEMPLATE_INDEX_VALIDATION_REPORT_SCHEMA, SHELL_TEMPLATE_MANIFEST_SCHEMA,
    SHELL_TEMPLATE_REPORT_SCHEMA, VALIDATION_REPORT_SCHEMA, VIEW_MODEL_SCHEMA,
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
use thiserror::Error;

const NEXT_PALETTE_MODULE_REQUEST: &str = "module.palette.next_available";

#[derive(Debug, Error)]
pub enum StudioCoreError {
    #[error("{path}: {source}")]
    ReadProject {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("{path}: {source}")]
    ParseProject {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellDescriptor {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellArtifactManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellTemplateIndex {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellTemplateManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHandoffManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    SerializeProject {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    WriteProject {
        path: String,
        #[source]
        source: std::io::Error,
    },
}

pub fn load_project(path: &Path) -> Result<StudioProject, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseProject {
        path: path.display().to_string(),
        source,
    })
}

pub fn save_project(path: &Path, project: &StudioProject) -> Result<(), StudioCoreError> {
    save_json(path, project)
}

pub fn load_shell_descriptor(path: &Path) -> Result<StudioShellDescriptor, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellDescriptor {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_artifact_manifest(
    path: &Path,
) -> Result<StudioShellArtifactManifest, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellArtifactManifest {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_template_index(path: &Path) -> Result<StudioShellTemplateIndex, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellTemplateIndex {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_template_manifest(
    path: &Path,
) -> Result<StudioShellTemplateManifest, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellTemplateManifest {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_handoff_manifest(
    path: &Path,
) -> Result<StudioShellHandoffManifest, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellHandoffManifest {
        path: path.display().to_string(),
        source,
    })
}

pub fn save_json<T: Serialize>(path: &Path, value: &T) -> Result<(), StudioCoreError> {
    let mut text = serde_json::to_string_pretty(value).map_err(|source| {
        StudioCoreError::SerializeProject {
            path: path.display().to_string(),
            source,
        }
    })?;
    text.push('\n');
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent).map_err(|source| StudioCoreError::WriteProject {
            path: path.display().to_string(),
            source,
        })?;
    }
    std::fs::write(path, text).map_err(|source| StudioCoreError::WriteProject {
        path: path.display().to_string(),
        source,
    })
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
        schema_id: SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA,
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
    let target_kind = if handoff.target_kind == StudioShellTargetKind::Unknown
        && handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
    {
        intended_target_kind
    } else {
        handoff.target_kind
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
        handoff_kind: handoff.handoff_kind,
        consumer_id: handoff.consumer_id,
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
        StudioShellDescriptorStatus, StudioShellHandoffKind, StudioShellTargetKind,
        StudioShellTemplateStatus, SHELL_HANDOFF_MANIFEST_SCHEMA,
        SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA, SHELL_HANDOFF_READINESS_REPORT_SCHEMA,
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
  "host_profile": "host.quest",
  "app_id": "app.host_shell.quest",
  "install_route": "install.adb_package",
  "launch_route": "launch.adb_activity",
  "command_bridge": "bridge.local_cli",
  "required_permissions": [],
  "evidence_pull_route": "evidence.filesystem"
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
        assert!(output_dir.join("shell-artifacts.json").is_file());
        assert!(output_dir.join("shell-templates.json").is_file());
        assert!(output_dir
            .join("shells/desktop/studio.graph.test.shell-template.json")
            .is_file());

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
        assert_eq!(headset.host_profile.as_deref(), Some("host.quest"));
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
