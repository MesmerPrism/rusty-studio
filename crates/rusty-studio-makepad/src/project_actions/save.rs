use std::path::Path;

use rusty_studio_core::{save_project, view_model_for_graph};
use rusty_studio_model::{StudioEditReport, StudioEditStatus, StudioProject, StudioViewModel};

pub(in crate::project_actions) fn save_applied_project_edit(
    project_path: &Path,
    project: StudioProject,
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
