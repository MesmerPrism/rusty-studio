use super::*;

mod handoff;
mod hostess;
mod release_candidate;
mod shell_package;

pub(crate) use handoff::*;
pub(crate) use hostess::*;
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

pub(crate) fn selected_shell_bundle_root_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-selected-shell")
}

pub(crate) fn selected_shell_bundle_output_dir(project_path: &Path, graph_id: &str) -> PathBuf {
    selected_shell_bundle_root_dir(project_path).join(graph_id)
}

pub(crate) fn shell_handoff_manifest_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoffs.json")
}

pub(crate) fn shell_handoff_acceptance_checklist_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoff-acceptance-checklist.json")
}

pub(crate) fn shell_export_package_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-export-package.json")
}

pub(crate) fn shell_export_package_baseline_manifest_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-export-package-baseline.json")
}

pub(crate) fn shell_export_package_baseline_archive_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("export-package-baselines")
}

pub(crate) fn shell_export_package_baseline_archive_package_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_export_package_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.package.json"))
}

pub(crate) fn shell_export_package_baseline_archive_manifest_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_export_package_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.baseline.json"))
}

pub(crate) fn shell_export_package_baseline_index_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-export-package-baselines.json")
}

pub(crate) fn shell_handoff_acceptance_baseline_manifest_output_path(
    project_path: &Path,
) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoff-acceptance-baseline.json")
}

pub(crate) fn shell_handoff_acceptance_baseline_archive_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("baselines")
}

pub(crate) fn shell_handoff_acceptance_baseline_archive_checklist_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_handoff_acceptance_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.checklist.json"))
}

pub(crate) fn shell_handoff_acceptance_baseline_archive_manifest_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_handoff_acceptance_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.baseline.json"))
}

pub(crate) fn shell_handoff_acceptance_baseline_index_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoff-acceptance-baselines.json")
}

pub(crate) fn shell_release_candidate_review_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-release-candidate-review.json")
}

pub(crate) fn shell_release_candidate_review_manifest_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-release-candidate-review-manifest.json")
}

pub(crate) fn shell_release_candidate_review_archive_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("release-candidates")
}

pub(crate) fn shell_release_candidate_review_archive_report_output_path(
    project_path: &Path,
    candidate_id: &str,
) -> PathBuf {
    shell_release_candidate_review_archive_dir(project_path)
        .join(format!("{candidate_id}.review.json"))
}

pub(crate) fn shell_release_candidate_review_archive_manifest_output_path(
    project_path: &Path,
    candidate_id: &str,
) -> PathBuf {
    shell_release_candidate_review_archive_dir(project_path)
        .join(format!("{candidate_id}.candidate.json"))
}

pub(crate) fn shell_release_candidate_review_index_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-release-candidate-reviews.json")
}

pub(crate) fn shell_hostess_handoff_package_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-handoff-package.json")
}

pub(crate) fn shell_hostess_owner_intake_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-owner-intake.json")
}

pub(crate) fn shell_hostess_staging_preview_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-staging-preview.json")
}

pub(crate) fn shell_hostess_staging_file_plan_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-staging-file-plan.json")
}

pub(crate) fn shell_hostess_staging_handoff_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-staging-handoff.json")
}

pub(crate) fn shell_hostess_staging_acceptance_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-staging-acceptance-checklist.json")
}

pub(crate) fn shell_hostess_staging_acceptance_manifest_output_path(
    project_path: &Path,
) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-staging-acceptance-manifest.json")
}

pub(crate) fn shell_hostess_staging_acceptance_archive_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("hostess-staging-acceptances")
}

pub(crate) fn shell_hostess_staging_acceptance_archive_checklist_output_path(
    project_path: &Path,
    acceptance_id: &str,
) -> PathBuf {
    shell_hostess_staging_acceptance_archive_dir(project_path)
        .join(format!("{acceptance_id}.checklist.json"))
}

pub(crate) fn shell_hostess_staging_acceptance_archive_manifest_output_path(
    project_path: &Path,
    acceptance_id: &str,
) -> PathBuf {
    shell_hostess_staging_acceptance_archive_dir(project_path)
        .join(format!("{acceptance_id}.acceptance.json"))
}

pub(crate) fn shell_hostess_staging_acceptance_index_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-staging-acceptances.json")
}

pub(crate) fn shell_hostess_staging_acceptance_comparison_output_path(
    project_path: &Path,
) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-staging-acceptance-comparison.json")
}

pub(crate) fn shell_hostess_staging_execution_request_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-staging-execution-request.json")
}
