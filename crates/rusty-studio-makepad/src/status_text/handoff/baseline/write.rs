use super::super::super::*;
use super::super::acceptance::*;
use super::index::*;

pub(crate) fn shell_handoff_acceptance_baseline_status(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    checklist_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection =
        summarize_shell_handoff_acceptance_baseline_index_selection(index, Some(index_path), None);
    format!(
        "acceptance baseline written\n  baseline: {} ({})\n  identity: {}\n  checklist: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        checklist_path.display(),
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path),
        shell_handoff_acceptance_status(report, bundle_root)
    )
}

pub(crate) fn shell_handoff_acceptance_baseline_append_status(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    checklist_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "acceptance baseline archived\n  baseline: {} ({})\n  identity: {}\n  checklist: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        checklist_path.display(),
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path),
        shell_handoff_acceptance_status(report, bundle_root)
    )
}
