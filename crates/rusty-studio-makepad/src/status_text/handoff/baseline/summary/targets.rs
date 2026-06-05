use super::super::super::super::*;

pub(super) fn shell_handoff_acceptance_baseline_target_rows(
    summary: &rusty_studio_model::StudioShellHandoffAcceptanceSummaryReport,
) -> String {
    let rows = summary
        .targets
        .iter()
        .map(|target| {
            let consumers = joined_list_or_none(&target.consumer_ids, ", ");
            let routes = joined_list_or_none(&target.route_kinds, ", ");
            let issues = joined_list_or_none(&target.issue_codes, ", ");
            format!(
                "{}: ready {}/{}; blocked {}; rejected {}; consumers {}; routes {}; issues {}",
                shell_target_kind_label(target.target_kind),
                target.ready_count,
                target.graph_count,
                target.blocked_count,
                target.rejected_count,
                consumers,
                routes,
                issues
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    if rows.is_empty() {
        "none".to_string()
    } else {
        rows
    }
}

fn joined_list_or_none(values: &[String], separator: &str) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(separator)
    }
}
