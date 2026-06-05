use super::super::super::*;

mod entries;

use entries::shell_hostess_staging_acceptance_comparison_entry_rows;

pub(crate) fn shell_hostess_staging_acceptance_comparison_status(
    report: &StudioShellHostessStagingAcceptanceComparisonReport,
    acceptance_path: &Path,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_acceptance_comparison_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let acceptance_id = report
        .baseline_acceptance_id
        .as_deref()
        .unwrap_or("unnamed");
    let acceptance_label = report.baseline_label.as_deref().unwrap_or("unlabeled");
    let baseline_checklist = report
        .baseline_checklist_path
        .as_deref()
        .unwrap_or("unknown");
    let index_path = report.baseline_index_path.as_deref().unwrap_or("not used");
    let index_default = report
        .baseline_index_default_acceptance_id
        .as_deref()
        .unwrap_or("none");
    let index_selected = report
        .baseline_index_selected_acceptance_id
        .as_deref()
        .unwrap_or("none");
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let rows = shell_hostess_staging_acceptance_comparison_entry_rows(report);

    format!(
        "Hostess staging acceptance comparison {status}; ready {}->{}, delta {}; blocked {}->{}, delta {}; rejected {}->{}, delta {}; issue {issue}\n  acceptance: {} ({})\n  baseline source: project {} rev {}; envelope {}; manifest {}\n  candidate source: project {} rev {}; envelope {}; manifest {}\n  baseline identity: {}\n  baseline checklist: {}\n  baseline index: {}; default {}; selected {}\n  comparison: {}\n  checks: {}; failed {}\n  entries:\n  {}",
        report.baseline_ready_item_count,
        report.candidate_ready_item_count,
        report.ready_item_delta,
        report.baseline_blocked_item_count,
        report.candidate_blocked_item_count,
        report.blocked_item_delta,
        report.baseline_rejected_item_count,
        report.candidate_rejected_item_count,
        report.rejected_item_delta,
        acceptance_id,
        acceptance_label,
        report.baseline_project_id.as_deref().unwrap_or("unknown"),
        report
            .baseline_project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.baseline_envelope_id,
        report.baseline_manifest_id.as_deref().unwrap_or("unknown"),
        report.candidate_project_id.as_deref().unwrap_or("unknown"),
        report
            .candidate_project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.candidate_envelope_id,
        report.candidate_manifest_id.as_deref().unwrap_or("unknown"),
        acceptance_path.display(),
        baseline_checklist,
        index_path,
        index_default,
        index_selected,
        output_path.display(),
        report.checks.len(),
        failed_checks,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
