use super::super::super::*;

pub(crate) fn shell_release_candidate_review_selection_status(
    report: &StudioShellReleaseCandidateReviewSelectionReport,
) -> String {
    let status = shell_release_candidate_review_selection_status_label(report.status);
    let requested = report.requested_candidate_id.as_deref().unwrap_or("none");
    let default = report.default_candidate_id.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let index_path = report.index_path.as_deref().unwrap_or("not saved");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_release_candidate_review_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry
                .candidate_manifest_path
                .as_deref()
                .unwrap_or("unknown");
            let selected_flag = if entry.selected { "yes" } else { "no" };
            let default_flag = if entry.default { "yes" } else { "no" };
            let acceptance = entry
                .acceptance_comparison_status
                .map(shell_handoff_acceptance_comparison_status_label)
                .unwrap_or("missing");
            let export_package = entry
                .export_package_comparison_status
                .map(shell_export_package_comparison_status_label)
                .unwrap_or("missing");
            format!(
                "{} [{}] selected {}; default {}; acceptance {}; package {}; checks failed {}; manifest {}; issue {}",
                entry.candidate_id,
                entry_status,
                selected_flag,
                default_flag,
                acceptance,
                export_package,
                entry.failed_check_count,
                manifest_path,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "release candidate selection {status}; requested {requested}; default {default}; selected {selected}; slots {}; ready {}; blocked {}; rejected {}; issue {issue}\n  index: {}\n  entries:\n  {}",
        report.candidate_count,
        report.ready_candidate_count,
        report.blocked_candidate_count,
        report.rejected_candidate_count,
        index_path,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
