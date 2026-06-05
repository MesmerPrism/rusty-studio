use super::super::super::super::super::*;

pub(super) fn shell_hostess_staging_acceptance_entry_rows(
    report: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> String {
    report
        .entries
        .iter()
        .map(|entry| {
            let entry_status = shell_hostess_staging_acceptance_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] owner {}; kind {}; route {}; next {}; prohibited in Studio {}; issue {}",
                entry.item_id,
                entry_status,
                entry.owner,
                entry.item_kind,
                entry.route_kind,
                entry.next_required_action,
                if entry.prohibited_in_studio {
                    "yes"
                } else {
                    "no"
                },
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
