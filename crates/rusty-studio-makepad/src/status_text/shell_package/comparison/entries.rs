use super::super::super::*;

pub(super) fn shell_export_package_comparison_entry_rows(
    report: &StudioShellExportPackageComparisonReport,
) -> String {
    report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let target = entry
                .target_kind
                .map(shell_target_kind_label)
                .unwrap_or("unknown");
            let baseline = entry
                .baseline_status
                .map(shell_export_package_status_label)
                .unwrap_or("missing");
            let candidate = entry
                .candidate_status
                .map(shell_export_package_status_label)
                .unwrap_or("missing");
            let change = shell_export_package_comparison_change_label(entry.change);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] {baseline}->{candidate}; change {change}; delta {}; descriptor {}->{}; template {}->{}; cli {}->{}; issue {}",
                entry.graph_id,
                target,
                entry.score_delta,
                present_label(entry.baseline_descriptor_present),
                present_label(entry.candidate_descriptor_present),
                present_label(entry.baseline_template_manifest_present),
                present_label(entry.candidate_template_manifest_present),
                present_label(entry.baseline_runbook_cli_request_present),
                present_label(entry.candidate_runbook_cli_request_present),
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
