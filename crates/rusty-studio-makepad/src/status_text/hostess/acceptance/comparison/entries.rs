use super::super::super::super::*;

pub(super) fn shell_hostess_staging_acceptance_comparison_entry_rows(
    report: &StudioShellHostessStagingAcceptanceComparisonReport,
) -> String {
    report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let baseline = entry
                .baseline_status
                .map(shell_hostess_staging_acceptance_status_label)
                .unwrap_or("missing");
            let candidate = entry
                .candidate_status
                .map(shell_hostess_staging_acceptance_status_label)
                .unwrap_or("missing");
            let change = shell_hostess_staging_acceptance_comparison_change_label(entry.change);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let route = entry
                .candidate_route_kind
                .as_deref()
                .or(entry.baseline_route_kind.as_deref())
                .unwrap_or("unknown");
            format!(
                "{} owner {}; {baseline}->{candidate}; change {change}; delta {}; route {}; issue {}",
                entry.item_id, entry.owner, entry.score_delta, route, issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
