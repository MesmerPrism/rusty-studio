use super::super::super::super::*;
use super::super::index::*;

pub(crate) fn shell_handoff_acceptance_baseline_select_status(
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "acceptance baseline default selected\n  baseline: {} ({})\n  identity: {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path)
    )
}
