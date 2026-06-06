use super::*;

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
