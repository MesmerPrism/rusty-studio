use super::super::super::*;

pub(crate) fn shell_release_candidate_review_index_status(
    index: &StudioShellReleaseCandidateReviewIndex,
    index_path: &Path,
) -> String {
    let default = index.default_candidate_id.as_deref().unwrap_or("none");
    let projects = if index.project_ids.is_empty() {
        "none".to_string()
    } else {
        index.project_ids.join(", ")
    };
    let manifests = if index.manifest_ids.is_empty() {
        "none".to_string()
    } else {
        index.manifest_ids.join(", ")
    };
    let rows = index
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let status = shell_release_candidate_review_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry
                .candidate_manifest_path
                .as_deref()
                .unwrap_or("unknown");
            let acceptance = entry
                .acceptance_comparison_status
                .map(shell_handoff_acceptance_comparison_status_label)
                .unwrap_or("missing");
            let export_package = entry
                .export_package_comparison_status
                .map(shell_export_package_comparison_status_label)
                .unwrap_or("missing");
            format!(
                "{} [{}] project {} rev {}; handoff ready {}; failed {}; missing {}; acceptance {}; package {}; checks failed {}; manifest {}; issue {}",
                entry.candidate_id,
                status,
                entry.project_id,
                entry.project_revision,
                entry.handoff_ready_count,
                entry.handoff_failed_count,
                entry.handoff_missing_bundle_count,
                acceptance,
                export_package,
                entry.failed_check_count,
                manifest_path,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "release candidate index slots {}; default {}; ready {}; blocked {}; rejected {}\n  index: {}\n  projects: {}\n  manifests: {}\n  entries:\n  {}",
        index.candidate_count,
        default,
        index.ready_candidate_count,
        index.blocked_candidate_count,
        index.rejected_candidate_count,
        index_path.display(),
        projects,
        manifests,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
