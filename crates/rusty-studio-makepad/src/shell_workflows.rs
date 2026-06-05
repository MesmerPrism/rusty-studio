use super::*;

mod handoff;
mod hostess;
mod paths;
mod release_candidate;
mod shell_package;

pub(crate) use handoff::*;
pub(crate) use hostess::*;
pub(crate) use paths::*;
pub(crate) use release_candidate::*;
pub(crate) use shell_package::*;

pub(crate) fn export_shell_bundle_for_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
) -> Result<(StudioShellBundleReport, PathBuf), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = selected_shell_bundle_for_graph(&project, project_path.parent(), &graph_id);
    let output_dir = selected_shell_bundle_output_dir(project_path, &graph_id);
    if report.status == StudioShellBundleStatus::Exported {
        save_shell_bundle(&output_dir, &report)
            .map_err(|error| format!("Shell bundle save failed: {error}"))?;
    }
    Ok((report, output_dir))
}

pub(crate) fn validate_shell_bundle_for_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
) -> Result<(StudioShellBundleValidationReport, PathBuf), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let output_dir = selected_shell_bundle_output_dir(project_path, &graph_id);
    let report =
        validate_selected_shell_bundle(&project, project_path.parent(), &graph_id, &output_dir);
    Ok((report, output_dir))
}

pub(crate) fn shell_handoff_for_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
) -> Result<(StudioShellHandoffReport, PathBuf), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let output_dir = selected_shell_bundle_output_dir(project_path, &graph_id);
    let report = shell_handoff_for_bundle(&project, project_path.parent(), &graph_id, &output_dir);
    Ok((report, output_dir))
}

pub(crate) fn shell_handoff_readiness_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHandoffReadinessReport, PathBuf), String> {
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let bundle_root = selected_shell_bundle_root_dir(project_path);
    let report = shell_handoff_readiness_for_project(&project, project_path.parent(), &bundle_root);
    Ok((report, bundle_root))
}

pub(crate) fn shell_runbook_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellRunbookReport, PathBuf), String> {
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let bundle_root = selected_shell_bundle_root_dir(project_path);
    let report = shell_runbook_for_project(&project, project_path.parent(), &bundle_root);
    Ok((report, bundle_root))
}
