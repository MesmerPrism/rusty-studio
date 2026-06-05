use super::super::super::super::*;
use super::lists::joined_list_or_none;

pub(super) fn shell_runbook_target_rows(report: &StudioShellRunbookReport) -> String {
    let rows = report
        .target_summaries
        .iter()
        .map(|target| {
            let consumers = joined_list_or_none(&target.consumer_ids, ", ");
            let owners = joined_list_or_none(&target.responsible_owners, ", ");
            let routes = joined_list_or_none(&target.runtime_route_kinds, ", ");
            let issues = joined_list_or_none(&target.issue_codes, ", ");
            format!(
                "{}: ready {}; blocked {}; rejected {}; consumers {}; owners {}; routes {}; issues {}",
                shell_target_kind_label(target.target_kind),
                target.ready_count,
                target.blocked_count,
                target.rejected_count,
                consumers,
                owners,
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
