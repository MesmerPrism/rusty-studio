use super::*;

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

pub(crate) fn shell_host_profile(
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

pub fn shell_descriptor_artifact_path(graph_id: &str) -> String {
    format!("descriptors/{graph_id}.shell-descriptor.json")
}
