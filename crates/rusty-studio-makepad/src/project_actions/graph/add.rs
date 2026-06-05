use std::path::Path;

use rusty_studio_core::{
    add_next_catalog_module_from_package_to_graph, add_next_catalog_module_to_graph, load_project,
};
use rusty_studio_model::{StudioEditReport, StudioViewModel};

use super::super::model::selected_graph_id_for_model;
use super::super::save::save_applied_project_edit;

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
