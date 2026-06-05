use super::super::super::super::*;

pub(super) fn shell_hostess_staging_file_plan_request_rows(
    report: &StudioShellHostessStagingFilePlan,
) -> String {
    report
        .requests
        .iter()
        .map(|request| {
            let request_status = shell_hostess_staging_file_request_status_label(request.status);
            let issue = request.issue_code.as_deref().unwrap_or("none");
            let target_kind = request
                .target_kind
                .map(shell_target_kind_label)
                .unwrap_or("shared");
            let graph = request.graph_id.as_deref().unwrap_or("shared");
            let routes = if request.route_kinds.is_empty() {
                "none".to_string()
            } else {
                request.route_kinds.join(", ")
            };
            format!(
                "{} [{}] kind {}; target {}; graph {}; files {}; root {}; routes {}; issue {}",
                request.request_id,
                request_status,
                request.request_kind,
                target_kind,
                graph,
                request.planned_file_count,
                request.destination_root,
                routes,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
