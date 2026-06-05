use super::super::super::*;
use super::super::acceptance::*;
use super::ShellHandoffAcceptanceBaselineWriteResult;

pub(crate) fn write_shell_handoff_acceptance_baseline_for_project_source(
    project_path: &Path,
) -> ShellHandoffAcceptanceBaselineWriteResult {
    let (report, bundle_root) = shell_handoff_acceptance_for_project_source(project_path)?;
    let output_path = shell_handoff_acceptance_checklist_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell handoff acceptance baseline save failed: {error}"))?;
    let baseline =
        shell_handoff_acceptance_baseline_manifest_for_checklist(&report, &output_path, None, None);
    let baseline_path = shell_handoff_acceptance_baseline_manifest_output_path(project_path);
    save_json(&baseline_path, &baseline).map_err(|error| {
        format!("Shell handoff acceptance baseline identity save failed: {error}")
    })?;
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(baseline.clone(), Some(baseline_path.clone()))],
        Some(&baseline.baseline_id),
    );
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell handoff acceptance baseline index save failed: {error}"))?;
    Ok((
        report,
        baseline,
        index,
        output_path,
        baseline_path,
        index_path,
        bundle_root,
    ))
}
