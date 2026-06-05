use super::*;

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
        .find(|issue| issue.issue_code.as_deref() == Some("studio.issue.package_reference_missing"))
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
