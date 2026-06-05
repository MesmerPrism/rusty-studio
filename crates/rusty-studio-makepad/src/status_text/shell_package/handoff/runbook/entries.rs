use super::super::super::super::*;
use super::lists::joined_list_or_none;

pub(super) fn shell_runbook_entry_rows(report: &StudioShellRunbookReport) -> String {
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_runbook_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let install = entry.host_routes.install_route.as_deref().unwrap_or("none");
            let launch = entry.host_routes.launch_route.as_deref().unwrap_or("none");
            let bridge = entry.host_routes.command_bridge.as_deref().unwrap_or("none");
            let evidence = entry
                .host_routes
                .evidence_pull_route
                .as_deref()
                .unwrap_or("none");
            let cli = joined_list_or_none(&entry.cli_request, " ");
            format!(
                "{} [{}] target {}; owner {}; action {}; policy {}; route {}; install {}; launch {}; bridge {}; evidence {}; cli {}; issue {}",
                entry.graph_id,
                entry_status,
                shell_target_kind_label(entry.target_kind),
                entry.responsible_owner,
                entry.next_required_action,
                entry.execution_policy,
                entry.runtime_route_kind,
                install,
                launch,
                bridge,
                evidence,
                cli,
                issue
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
