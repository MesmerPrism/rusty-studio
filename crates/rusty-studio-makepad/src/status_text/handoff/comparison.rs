use super::super::*;

mod entries;

use entries::shell_handoff_acceptance_comparison_entry_rows;

pub(crate) fn shell_handoff_acceptance_comparison_status(
    report: &StudioShellHandoffAcceptanceComparisonReport,
    baseline_path: &Path,
    bundle_root: &Path,
) -> String {
    let status = shell_handoff_acceptance_comparison_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let baseline_id = report.baseline_id.as_deref().unwrap_or("unnamed");
    let baseline_label = report.baseline_label.as_deref().unwrap_or("unlabeled");
    let baseline_checklist = report
        .baseline_checklist_path
        .as_deref()
        .unwrap_or("unknown");
    let baseline_index_path = report.baseline_index_path.as_deref().unwrap_or("not used");
    let baseline_index_default = report
        .baseline_index_default_baseline_id
        .as_deref()
        .unwrap_or("none");
    let baseline_index_selected = report
        .baseline_index_selected_baseline_id
        .as_deref()
        .unwrap_or("none");
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let rows = shell_handoff_acceptance_comparison_entry_rows(report);

    format!(
        "handoff acceptance comparison {status}; ready {}->{}, delta {}; blocked {}->{}, delta {}; rejected {}->{}, delta {}; issue {issue}\n  baseline: {} ({})\n  baseline source: {} rev {}; manifest {}\n  candidate: {} rev {}; manifest {}\n  baseline identity: {}\n  baseline checklist: {}\n  baseline index: {}; default {}; selected {}\n  current root: {}\n  checks: {}; failed {}\n  entries:\n  {}",
        report.baseline_ready_count,
        report.candidate_ready_count,
        report.ready_delta,
        report.baseline_blocked_count,
        report.candidate_blocked_count,
        report.blocked_delta,
        report.baseline_rejected_count,
        report.candidate_rejected_count,
        report.rejected_delta,
        baseline_id,
        baseline_label,
        report.baseline_project_id,
        report.baseline_project_revision,
        report.baseline_manifest_id,
        report.candidate_project_id,
        report.candidate_project_revision,
        report.candidate_manifest_id,
        baseline_path.display(),
        baseline_checklist,
        baseline_index_path,
        baseline_index_default,
        baseline_index_selected,
        bundle_root.display(),
        report.checks.len(),
        failed_checks,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
