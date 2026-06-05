use super::super::super::super::*;

pub(crate) fn shell_hostess_staging_acceptance_index_status(
    index: &StudioShellHostessStagingAcceptanceIndex,
    index_path: &Path,
) -> String {
    let default = index.default_acceptance_id.as_deref().unwrap_or("none");
    let projects = if index.project_ids.is_empty() {
        "none".to_string()
    } else {
        index.project_ids.join(", ")
    };
    let envelopes = if index.envelope_ids.is_empty() {
        "none".to_string()
    } else {
        index.envelope_ids.join(", ")
    };
    let rows = index
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let status = shell_hostess_staging_acceptance_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry
                .acceptance_manifest_path
                .as_deref()
                .unwrap_or("unknown");
            format!(
                "{} [{}] project {} rev {}; envelope {}; items ready {}; blocked {}; rejected {}; manifest {}; issue {}",
                entry.acceptance_id,
                status,
                entry.project_id.as_deref().unwrap_or("unknown"),
                entry
                    .project_revision
                    .map(|revision| revision.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                entry.envelope_id,
                entry.ready_item_count,
                entry.blocked_item_count,
                entry.rejected_item_count,
                manifest_path,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "Hostess staging acceptance index slots {}; default {}; ready {}; blocked {}; rejected {}\n  index: {}\n  projects: {}\n  envelopes: {}\n  entries:\n  {}",
        index.acceptance_count,
        default,
        index.ready_acceptance_count,
        index.blocked_acceptance_count,
        index.rejected_acceptance_count,
        index_path.display(),
        projects,
        envelopes,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
