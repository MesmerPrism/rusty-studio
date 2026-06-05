use super::super::super::super::*;

pub(super) fn shell_hostess_staging_handoff_request_rows(
    report: &StudioShellHostessStagingHandoffEnvelope,
) -> String {
    report
        .request_summaries
        .iter()
        .map(|request| {
            let request_status = shell_hostess_staging_file_request_status_label(request.status);
            let target_kind = request
                .target_kind
                .map(shell_target_kind_label)
                .unwrap_or("shared");
            format!(
                "{} [{}] target {}; files {}; root {}",
                request.request_id,
                request_status,
                target_kind,
                request.planned_file_count,
                request.destination_root
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
