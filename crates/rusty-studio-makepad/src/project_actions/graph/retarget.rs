use std::path::Path;

use rusty_studio_core::{load_project, retarget_graph_host_profile};
use rusty_studio_model::{StudioEditReport, StudioViewModel};

use super::super::model::selected_graph_id_for_model;
use super::super::save::save_applied_project_edit;

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
