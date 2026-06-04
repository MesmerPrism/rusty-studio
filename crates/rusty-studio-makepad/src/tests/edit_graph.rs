use super::*;

#[test]
fn retarget_project_source_saves_and_refreshes_view_model() {
    let root = temp_root("retarget-source");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");

    let (report, refreshed_model) =
        retarget_project_source(&project_path, &model, 0, "host_run.profile.headset")
            .expect("retarget project source");
    let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
    let saved_project = load_project(&project_path).expect("load saved edited project");

    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert_eq!(saved_project.revision, 2);
    assert_eq!(
        saved_project.graphs[0].target_host_profile,
        "host_run.profile.headset"
    );
    assert_eq!(refreshed_model.revision, 2);
    assert_eq!(
        refreshed_model.graphs[0].target_host_profile,
        "host_run.profile.headset"
    );
}

#[test]
fn add_next_catalog_module_to_project_source_saves_and_refreshes_view_model() {
    let root = temp_root("add-next-palette-module-source");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");

    let (report, refreshed_model) =
        add_next_catalog_module_to_project_source(&project_path, &model, 0, None)
            .expect("add next palette module to project source");
    let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
    let saved_project = load_project(&project_path).expect("load saved edited project");

    assert_eq!(report.operation, StudioEditOperation::AddModule);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.requested_reference_id, "module.synthetic_provider");
    assert_eq!(saved_project.revision, 2);
    assert!(saved_project.graphs[0].nodes.iter().any(|node| {
        node.kind == StudioNodeKind::Module && node.reference_id == "module.synthetic_provider"
    }));
    assert_eq!(refreshed_model.revision, 2);
    assert_eq!(refreshed_model.graphs[0].module_count, 1);
}

#[test]
fn selected_package_drives_add_palette_module_request() {
    let root = temp_root("selected-package-palette-module-source");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(
        &project_path,
        Some("studio.graph.makepad_edit"),
        None,
        Some("node.package.synthetic"),
        None,
    )
    .expect("load selected package view model");

    let package_reference_id =
        selected_package_reference_id(&model).expect("selected package reference");
    let (report, refreshed_model) = add_next_catalog_module_to_project_source(
        &project_path,
        &model,
        0,
        Some(&package_reference_id),
    )
    .expect("add selected package module to project source");
    let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
    let saved_project = load_project(&project_path).expect("load saved edited project");

    assert_eq!(package_reference_id, "package.synthetic");
    assert_eq!(report.operation, StudioEditOperation::AddModule);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.requested_reference_id, "module.synthetic_provider");
    assert!(saved_project.graphs[0].nodes.iter().any(|node| {
        node.kind == StudioNodeKind::Module && node.reference_id == "module.synthetic_provider"
    }));
    assert_eq!(refreshed_model.graphs[0].module_count, 1);
}

#[test]
fn add_module_to_project_source_saves_and_refreshes_view_model() {
    let root = temp_root("add-module-source");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");

    let (report, refreshed_model) = add_module_to_project_source(
        &project_path,
        &model,
        0,
        "package.synthetic",
        "module.synthetic_provider",
        Some("Synthetic Provider"),
    )
    .expect("add module to project source");
    let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
    let saved_project = load_project(&project_path).expect("load saved edited project");

    assert_eq!(report.operation, StudioEditOperation::AddModule);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert_eq!(saved_project.revision, 2);
    assert!(saved_project.graphs[0].nodes.iter().any(|node| {
        node.kind == StudioNodeKind::Module && node.reference_id == "module.synthetic_provider"
    }));
    assert!(saved_project.graphs[0].edges.iter().any(|edge| {
        edge.kind == StudioEdgeKind::PackageProvidesModule
            && edge.target_node_id == "node.module.synthetic_provider"
    }));
    assert_eq!(refreshed_model.revision, 2);
    assert_eq!(refreshed_model.graphs[0].module_count, 1);
}

#[test]
fn remove_module_from_project_source_saves_and_refreshes_view_model() {
    let root = temp_root("remove-module-source");
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

    let (report, refreshed_model) =
        remove_module_from_project_source(&project_path, &model, 0, "module.synthetic_provider")
            .expect("remove module from project source");
    let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
    let saved_project = load_project(&project_path).expect("load saved edited project");

    assert_eq!(report.operation, StudioEditOperation::RemoveModule);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert_eq!(saved_project.revision, 2);
    assert!(!saved_project.graphs[0].nodes.iter().any(|node| {
        node.kind == StudioNodeKind::Module && node.reference_id == "module.synthetic_provider"
    }));
    assert!(!saved_project.graphs[0].edges.iter().any(|edge| {
        edge.source_node_id == "node.module.synthetic_provider"
            || edge.target_node_id == "node.module.synthetic_provider"
    }));
    assert_eq!(refreshed_model.revision, 2);
    assert_eq!(refreshed_model.graphs[0].module_count, 0);
}

#[test]
fn selected_module_reference_drives_remove_module_request() {
    let root = temp_root("selected-module-remove-source");
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

    let module_reference_id =
        selected_module_reference_id(&model).expect("selected module reference");
    assert_eq!(module_reference_id, "module.synthetic_provider");
    let (report, refreshed_model) =
        remove_module_from_project_source(&project_path, &model, 0, &module_reference_id)
            .expect("remove selected module from project source");
    let refreshed_model = refreshed_model.expect("refreshed model after applied edit");

    assert_eq!(report.operation, StudioEditOperation::RemoveModule);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.requested_reference_id, "module.synthetic_provider");
    assert_eq!(refreshed_model.graphs[0].module_count, 0);
}
