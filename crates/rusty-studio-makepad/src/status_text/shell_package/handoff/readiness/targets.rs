use super::super::super::super::*;

pub(super) fn shell_handoff_readiness_target_rows(
    report: &StudioShellHandoffReadinessReport,
) -> String {
    report
        .target_summaries
        .iter()
        .map(|summary| {
            let ready_path = summary
                .template_index_paths
                .first()
                .map(|path| format!("; templates {path}"))
                .unwrap_or_default();
            let missing_path = summary
                .missing_bundle_dirs
                .first()
                .map(|path| format!("; missing bundle {path}"))
                .unwrap_or_default();
            format!(
                "{}: ready {}/{}; missing {}; packages {}; modules {}; shells {}{}{}",
                shell_target_kind_label(summary.target_kind),
                summary.ready_count,
                summary.graph_count,
                summary.missing_bundle_count,
                summary.package_count,
                summary.module_count,
                summary.operator_shell_count,
                ready_path,
                missing_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
