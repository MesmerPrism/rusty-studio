use std::path::Path;

use rusty_studio_core::{
    add_binding_to_graph, add_next_catalog_module_from_package_to_graph,
    add_next_catalog_module_to_graph, load_project, remove_binding_from_graph,
    remove_module_from_graph, retarget_graph_host_profile, save_project, view_model_for_graph,
    view_model_for_graph_issue_node_and_edge,
};
use rusty_studio_model::{StudioBindingKind, StudioEditReport, StudioEditStatus, StudioViewModel};

pub(crate) fn load_studio_view_model_for_path(
    project_path: &Path,
    requested_graph_id: Option<&str>,
    requested_issue_check_id: Option<&str>,
    requested_node_id: Option<&str>,
    requested_edge_id: Option<&str>,
) -> Result<StudioViewModel, String> {
    let project = load_project(project_path).map_err(|error| error.to_string())?;
    Ok(view_model_for_graph_issue_node_and_edge(
        &project,
        project_path.parent(),
        requested_graph_id,
        requested_issue_check_id,
        requested_node_id,
        requested_edge_id,
    ))
}

pub(crate) fn canvas_selection_view_model_for_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    requested_node_id: Option<&str>,
    requested_edge_id: Option<&str>,
) -> Result<StudioViewModel, String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    load_studio_view_model_for_path(
        project_path,
        Some(&graph_id),
        model.selected_issue_check_id.as_deref(),
        requested_node_id,
        requested_edge_id,
    )
}

pub(crate) fn retarget_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    target_host_profile: &str,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = retarget_graph_host_profile(
        &mut project,
        &graph_id,
        target_host_profile,
        project_path.parent(),
    );
    save_applied_project_edit(project_path, project, &graph_id, report)
}

#[cfg(test)]
pub(crate) fn add_module_to_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    package_reference_id: &str,
    module_reference_id: &str,
    module_label: Option<&str>,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = rusty_studio_core::add_module_to_graph(
        &mut project,
        &graph_id,
        package_reference_id,
        module_reference_id,
        module_label,
        project_path.parent(),
    );
    save_applied_project_edit(project_path, project, &graph_id, report)
}

pub(crate) fn add_next_catalog_module_to_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    package_reference_id: Option<&str>,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = if let Some(package_reference_id) = package_reference_id {
        add_next_catalog_module_from_package_to_graph(
            &mut project,
            &graph_id,
            package_reference_id,
            project_path.parent(),
        )
    } else {
        add_next_catalog_module_to_graph(&mut project, &graph_id, project_path.parent())
    };
    save_applied_project_edit(project_path, project, &graph_id, report)
}

pub(crate) fn remove_module_from_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    module_reference_id: &str,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = remove_module_from_graph(
        &mut project,
        &graph_id,
        module_reference_id,
        project_path.parent(),
    );
    save_applied_project_edit(project_path, project, &graph_id, report)
}

pub(crate) fn add_binding_to_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    binding_kind: StudioBindingKind,
    source_node_id: &str,
    target_node_id: &str,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = add_binding_to_graph(
        &mut project,
        &graph_id,
        binding_kind,
        source_node_id,
        target_node_id,
        project_path.parent(),
    );
    save_applied_project_edit(project_path, project, &graph_id, report)
}

pub(crate) fn remove_binding_from_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    binding_kind: StudioBindingKind,
    source_node_id: &str,
    target_node_id: &str,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = remove_binding_from_graph(
        &mut project,
        &graph_id,
        binding_kind,
        source_node_id,
        target_node_id,
        project_path.parent(),
    );
    save_applied_project_edit(project_path, project, &graph_id, report)
}

pub(crate) fn selected_graph_id_for_model(
    model: &StudioViewModel,
    selected_graph_index: usize,
) -> Option<String> {
    if model.selection_issue_code.is_some() {
        return None;
    }
    model
        .graphs
        .get(selected_graph_index)
        .map(|graph| graph.graph_id.clone())
}

fn save_applied_project_edit(
    project_path: &Path,
    project: rusty_studio_model::StudioProject,
    graph_id: &str,
    report: StudioEditReport,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    if report.status != StudioEditStatus::Applied {
        return Ok((report, None));
    }
    save_project(project_path, &project)
        .map_err(|error| format!("Project save failed: {error}"))?;
    let refreshed_model = view_model_for_graph(&project, project_path.parent(), Some(graph_id));
    Ok((report, Some(refreshed_model)))
}
