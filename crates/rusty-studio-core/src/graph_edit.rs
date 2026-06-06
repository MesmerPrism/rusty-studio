use super::*;

mod bindings;

pub use bindings::{add_binding_to_graph, remove_binding_from_graph};

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
