use super::super::super::super::*;
use super::ShellHandoffAcceptanceBaselineSelectionResult;

pub(crate) fn promote_shell_handoff_acceptance_baseline_default_for_project_source(
    project_path: &Path,
) -> ShellHandoffAcceptanceBaselineSelectionResult {
    let baseline_path = shell_handoff_acceptance_baseline_manifest_output_path(project_path);
    let baseline = load_shell_handoff_acceptance_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Baseline acceptance identity load failed: {error}"))?;
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let index = load_shell_handoff_acceptance_baseline_index(&index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    let promoted =
        promote_shell_handoff_acceptance_baseline_index_default(&index, &baseline.baseline_id)
            .ok_or_else(|| {
                format!(
                    "Baseline acceptance index does not contain baseline {}",
                    baseline.baseline_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Baseline acceptance index save failed: {error}"))?;
    Ok((baseline, promoted, baseline_path, index_path))
}
