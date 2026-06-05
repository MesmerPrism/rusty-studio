use super::super::super::super::*;

pub(crate) fn shell_hostess_staging_acceptance_selection_status(
    report: &StudioShellHostessStagingAcceptanceSelectionReport,
) -> String {
    let status = shell_hostess_staging_acceptance_selection_status_label(report.status);
    let requested = report.requested_acceptance_id.as_deref().unwrap_or("none");
    let default = report.default_acceptance_id.as_deref().unwrap_or("none");
    let selected = report.selected_acceptance_id.as_deref().unwrap_or("none");
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let index_path = report.index_path.as_deref().unwrap_or("not saved");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_hostess_staging_acceptance_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry
                .acceptance_manifest_path
                .as_deref()
                .unwrap_or("unknown");
            let selected_flag = if entry.selected { "yes" } else { "no" };
            let default_flag = if entry.default { "yes" } else { "no" };
            format!(
                "{} [{}] selected {}; default {}; items ready {}; blocked {}; rejected {}; manifest {}; issue {}",
                entry.acceptance_id,
                entry_status,
                selected_flag,
                default_flag,
                entry.ready_item_count,
                entry.blocked_item_count,
                entry.rejected_item_count,
                manifest_path,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "Hostess staging acceptance selection {status}; requested {requested}; default {default}; selected {selected}; slots {}; ready {}; blocked {}; rejected {}; issue {issue}\n  index: {}\n  entries:\n  {}",
        report.acceptance_count,
        report.ready_acceptance_count,
        report.blocked_acceptance_count,
        report.rejected_acceptance_count,
        index_path,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
