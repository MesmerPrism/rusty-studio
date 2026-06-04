use super::*;

#[test]
fn add_binding_to_project_source_saves_and_refreshes_view_model() {
    let root = temp_root("add-binding-source");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    let mut project = editable_project();
    project.graphs[0].nodes.push(StudioNode {
        node_id: "node.module.synthetic_provider".to_string(),
        kind: StudioNodeKind::Module,
        reference_id: "module.synthetic_provider".to_string(),
        label: "Synthetic Provider".to_string(),
    });
    project.graphs[0].edges.push(StudioEdge {
        edge_id: "edge.package_module".to_string(),
        kind: StudioEdgeKind::PackageProvidesModule,
        source_node_id: "node.package.synthetic".to_string(),
        target_node_id: "node.module.synthetic_provider".to_string(),
    });
    save_project(&project_path, &project).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");

    let (report, refreshed_model) = add_binding_to_project_source(
        &project_path,
        &model,
        0,
        StudioBindingKind::Command,
        "node.shell.operator",
        "node.module.synthetic_provider",
    )
    .expect("add binding to project source");
    let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
    let saved_project = load_project(&project_path).expect("load saved edited project");

    assert_eq!(report.operation, StudioEditOperation::AddBinding);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert_eq!(saved_project.revision, 2);
    assert!(saved_project.graphs[0].edges.iter().any(|edge| {
        edge.kind == StudioEdgeKind::CommandBinding
            && edge.source_node_id == "node.shell.operator"
            && edge.target_node_id == "node.module.synthetic_provider"
    }));
    assert_eq!(refreshed_model.revision, 2);
    assert_eq!(refreshed_model.graphs[0].edge_count, 3);
}

#[test]
fn selected_module_drives_add_command_binding_request() {
    let root = temp_root("selected-command-binding-source");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    let mut project = editable_project();
    project.graphs[0].nodes.push(StudioNode {
        node_id: "node.module.synthetic_provider".to_string(),
        kind: StudioNodeKind::Module,
        reference_id: "module.synthetic_provider".to_string(),
        label: "Synthetic Provider".to_string(),
    });
    project.graphs[0].edges.push(StudioEdge {
        edge_id: "edge.package_module".to_string(),
        kind: StudioEdgeKind::PackageProvidesModule,
        source_node_id: "node.package.synthetic".to_string(),
        target_node_id: "node.module.synthetic_provider".to_string(),
    });
    save_project(&project_path, &project).expect("save editable project");
    let model = load_studio_view_model_for_path(
        &project_path,
        Some("studio.graph.makepad_edit"),
        None,
        Some("node.module.synthetic_provider"),
        None,
    )
    .expect("load selected module view model");

    let request =
        selected_command_binding_request(&model).expect("selected command binding request");
    assert_eq!(request.binding_kind, StudioBindingKind::Command);
    assert_eq!(request.source_node_id, "node.shell.operator");
    assert_eq!(request.target_node_id, "node.module.synthetic_provider");
    let (report, refreshed_model) = add_binding_to_project_source(
        &project_path,
        &model,
        0,
        request.binding_kind,
        &request.source_node_id,
        &request.target_node_id,
    )
    .expect("add selected command binding to project source");
    let refreshed_model = refreshed_model.expect("refreshed model after applied edit");

    assert_eq!(report.operation, StudioEditOperation::AddBinding);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(
        report.requested_reference_id,
        "edge.command_binding.node.shell.operator.node.module.synthetic_provider"
    );
    assert_eq!(refreshed_model.graphs[0].edge_count, 3);
}

#[test]
fn remove_binding_from_project_source_saves_and_refreshes_view_model() {
    let root = temp_root("remove-binding-source");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    let mut project = editable_project();
    project.graphs[0].nodes.push(StudioNode {
        node_id: "node.module.synthetic_provider".to_string(),
        kind: StudioNodeKind::Module,
        reference_id: "module.synthetic_provider".to_string(),
        label: "Synthetic Provider".to_string(),
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
    save_project(&project_path, &project).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");

    let (report, refreshed_model) = remove_binding_from_project_source(
        &project_path,
        &model,
        0,
        StudioBindingKind::Command,
        "node.shell.operator",
        "node.module.synthetic_provider",
    )
    .expect("remove binding from project source");
    let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
    let saved_project = load_project(&project_path).expect("load saved edited project");

    assert_eq!(report.operation, StudioEditOperation::RemoveBinding);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert_eq!(saved_project.revision, 2);
    assert!(!saved_project.graphs[0].edges.iter().any(|edge| {
        edge.kind == StudioEdgeKind::CommandBinding
            && edge.source_node_id == "node.shell.operator"
            && edge.target_node_id == "node.module.synthetic_provider"
    }));
    assert_eq!(refreshed_model.revision, 2);
    assert_eq!(refreshed_model.graphs[0].edge_count, 2);
}

#[test]
fn selected_binding_drives_remove_binding_request() {
    let root = temp_root("selected-binding-remove-source");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    let mut project = editable_project();
    project.graphs[0].nodes.push(StudioNode {
        node_id: "node.module.synthetic_provider".to_string(),
        kind: StudioNodeKind::Module,
        reference_id: "module.synthetic_provider".to_string(),
        label: "Synthetic Provider".to_string(),
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
    save_project(&project_path, &project).expect("save editable project");
    let model = load_studio_view_model_for_path(
        &project_path,
        Some("studio.graph.makepad_edit"),
        None,
        None,
        Some("edge.shell_command"),
    )
    .expect("load selected binding view model");

    let request = selected_binding_request(&model).expect("selected binding request");
    assert_eq!(request.binding_kind, StudioBindingKind::Command);
    assert_eq!(request.source_node_id, "node.shell.operator");
    assert_eq!(request.target_node_id, "node.module.synthetic_provider");
    let (report, refreshed_model) = remove_binding_from_project_source(
        &project_path,
        &model,
        0,
        request.binding_kind,
        &request.source_node_id,
        &request.target_node_id,
    )
    .expect("remove selected binding from project source");
    let refreshed_model = refreshed_model.expect("refreshed model after applied edit");

    assert_eq!(report.operation, StudioEditOperation::RemoveBinding);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(
        report.requested_reference_id,
        "edge.command_binding.node.shell.operator.node.module.synthetic_provider"
    );
    assert_eq!(refreshed_model.graphs[0].edge_count, 2);
}
