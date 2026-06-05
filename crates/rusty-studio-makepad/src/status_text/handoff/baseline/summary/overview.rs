use super::super::super::super::*;
use super::super::index::*;
use super::targets::shell_handoff_acceptance_baseline_target_rows;

pub(crate) fn shell_handoff_acceptance_summary_status(
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection =
        summarize_shell_handoff_acceptance_baseline_index_selection(index, Some(index_path), None);
    let summary = &baseline.summary;
    let status = shell_handoff_acceptance_status_label(summary.status);
    let issue = summary.issue_code.as_deref().unwrap_or("none");
    let target_rows = shell_handoff_acceptance_baseline_target_rows(summary);
    format!(
        "acceptance baseline summary {status}; baseline {} ({}); project {} rev {}; manifest {}; ready {}; blocked {}; rejected {}; entries {}; issue {issue}\n  identity: {}\n  checklist: {}\n  intake checks: {}; failed {}\n  targets:\n  {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        summary.project_id,
        summary.project_revision,
        summary.manifest_id,
        summary.ready_count,
        summary.blocked_count,
        summary.rejected_count,
        summary.entry_count,
        baseline_path.display(),
        baseline.checklist_path,
        summary.intake_check_count,
        summary.failed_intake_check_count,
        target_rows,
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path)
    )
}
