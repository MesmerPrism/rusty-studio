use super::*;

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
