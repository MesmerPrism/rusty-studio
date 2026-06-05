use super::*;

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
        node.kind == StudioNodeKind::HostProfile && node.reference_id == "host_run.profile.headset"
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
    assert!(report
        .changed_fields
        .iter()
        .any(|field| field.ends_with("edges.edge.package.synthetic.module.synthetic_provider")));
}

#[test]
fn add_next_catalog_module_to_graph_uses_palette_selection() {
    let root = temp_root("add-next-palette-module");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();

    let report = add_next_catalog_module_to_graph(&mut project, "studio.graph.test", Some(&root));

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

    let report = add_next_catalog_module_to_graph(&mut project, "studio.graph.test", Some(&root));

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
