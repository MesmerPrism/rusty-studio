use super::super::super::*;
use super::owners::shell_handoff_acceptance_owner_summary;

pub(super) fn shell_handoff_acceptance_entry_rows(
    report: &StudioShellHandoffAcceptanceChecklistReport,
) -> String {
    report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_handoff_acceptance_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let failed_checks = entry
                .checks
                .iter()
                .filter(|check| check.status == StudioValidationStatus::Fail)
                .count();
            format!(
                "{} [{}] -> {} / {}; action {}; route {}; owners {}; failed {}; issue {}",
                entry.graph_id,
                shell_target_kind_label(entry.target_kind),
                entry.consumer_id,
                entry_status,
                entry.next_required_action,
                entry.runtime_route_kind,
                shell_handoff_acceptance_owner_summary(report, &entry.graph_id),
                failed_checks,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
