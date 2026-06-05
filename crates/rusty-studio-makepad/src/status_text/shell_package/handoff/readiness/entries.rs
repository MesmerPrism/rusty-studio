use super::super::super::super::*;

pub(super) fn shell_handoff_readiness_entry_rows(
    report: &StudioShellHandoffReadinessReport,
) -> String {
    report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = validation_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] profile {}; packages {}; modules {}; shell {}; -> {} / {}; issue {}",
                entry.graph_id,
                shell_target_kind_label(entry.target_kind),
                entry.target_host_profile,
                entry.package_count,
                entry.module_count,
                entry.operator_shell_count,
                entry.consumer_id,
                entry_status,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
