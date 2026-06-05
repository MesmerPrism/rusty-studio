use super::super::super::*;

pub(super) fn shell_handoff_acceptance_comparison_entry_rows(
    report: &StudioShellHandoffAcceptanceComparisonReport,
) -> String {
    report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let target = entry
                .target_kind
                .map(shell_target_kind_label)
                .unwrap_or("unknown");
            let baseline = entry
                .baseline_status
                .map(shell_handoff_acceptance_status_label)
                .unwrap_or("missing");
            let candidate = entry
                .candidate_status
                .map(shell_handoff_acceptance_status_label)
                .unwrap_or("missing");
            let change = shell_handoff_acceptance_comparison_change_label(entry.change);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let route = entry
                .candidate_route_kind
                .as_deref()
                .or(entry.baseline_route_kind.as_deref())
                .unwrap_or("unknown");
            format!(
                "{} [{}] {baseline}->{candidate}; change {change}; delta {}; route {}; issue {}",
                entry.graph_id, target, entry.score_delta, route, issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
