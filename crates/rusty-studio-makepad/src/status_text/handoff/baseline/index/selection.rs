use super::super::super::super::*;

pub(crate) fn shell_handoff_acceptance_baseline_selection_status(
    report: &StudioShellHandoffAcceptanceBaselineSelectionReport,
) -> String {
    let status = shell_handoff_acceptance_baseline_selection_status_label(report.status);
    let requested = report.requested_baseline_id.as_deref().unwrap_or("none");
    let default = report.default_baseline_id.as_deref().unwrap_or("none");
    let selected = report.selected_baseline_id.as_deref().unwrap_or("none");
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let index_path = report.index_path.as_deref().unwrap_or("not saved");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_handoff_acceptance_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry.baseline_manifest_path.as_deref().unwrap_or("unknown");
            let selected_flag = if entry.selected { "yes" } else { "no" };
            let default_flag = if entry.default { "yes" } else { "no" };
            format!(
                "{} [{}] selected {}; default {}; ready {}; blocked {}; rejected {}; manifest {}; issue {}",
                entry.baseline_id,
                entry_status,
                selected_flag,
                default_flag,
                entry.ready_count,
                entry.blocked_count,
                entry.rejected_count,
                manifest_path,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "baseline selection {status}; requested {requested}; default {default}; selected {selected}; slots {}; ready {}; blocked {}; rejected {}; issue {issue}\n  index: {}\n  entries:\n  {}",
        report.baseline_count,
        report.ready_baseline_count,
        report.blocked_baseline_count,
        report.rejected_baseline_count,
        index_path,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
