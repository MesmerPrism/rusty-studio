use std::path::Path;

use rusty_studio_core::{load_project, view_model_for_graph_issue_node_and_edge};
use rusty_studio_model::StudioViewModel;

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
