use super::super::super::super::*;
use super::ShellHandoffAcceptanceBaselineSelectionResult;

pub(crate) fn shell_handoff_acceptance_baseline_summary_for_project_source(
    project_path: &Path,
) -> ShellHandoffAcceptanceBaselineSelectionResult {
    let baseline_path = shell_handoff_acceptance_baseline_manifest_output_path(project_path);
    let baseline = load_shell_handoff_acceptance_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Baseline acceptance identity load failed: {error}"))?;
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let index = load_shell_handoff_acceptance_baseline_index(&index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    Ok((baseline, index, baseline_path, index_path))
}
