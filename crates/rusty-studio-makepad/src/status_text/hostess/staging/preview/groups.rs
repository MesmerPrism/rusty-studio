use super::super::super::super::*;

pub(super) fn shell_hostess_staging_preview_group_rows(
    report: &StudioShellHostessStagingPreviewManifest,
) -> String {
    report
        .groups
        .iter()
        .map(|group| {
            let group_status = shell_hostess_staging_preview_group_status_label(group.status);
            let issue = group.issue_code.as_deref().unwrap_or("none");
            let target_kinds = if group.target_kinds.is_empty() {
                "none".to_string()
            } else {
                group.target_kinds.join(", ")
            };
            let graph_ids = if group.graph_ids.is_empty() {
                "none".to_string()
            } else {
                group.graph_ids.join(", ")
            };
            format!(
                "{} route {} [{}] owner {}; artifacts {}; targets {}; graphs {}; issue {}",
                group.action_id,
                group.route_kind,
                group_status,
                group.owner,
                group.expected_artifact_count,
                target_kinds,
                graph_ids,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
