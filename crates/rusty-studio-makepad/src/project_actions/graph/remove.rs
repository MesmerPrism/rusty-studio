use std::path::Path;

use rusty_studio_core::{load_project, remove_module_from_graph};
use rusty_studio_model::{StudioEditReport, StudioViewModel};

use super::super::model::selected_graph_id_for_model;
use super::super::save::save_applied_project_edit;

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
