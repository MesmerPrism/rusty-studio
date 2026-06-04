use super::super::*;

pub(crate) fn shell_export_package_comparison_status(
    report: &StudioShellExportPackageComparisonReport,
    baseline_path: &Path,
    bundle_root: &Path,
) -> String {
    let status = shell_export_package_comparison_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let baseline_id = report.baseline_id.as_deref().unwrap_or("unnamed");
    let baseline_label = report.baseline_label.as_deref().unwrap_or("unlabeled");
    let baseline_package = report.baseline_package_path.as_deref().unwrap_or("unknown");
    let baseline_index_path = report.baseline_index_path.as_deref().unwrap_or("not used");
    let baseline_index_default = report
        .baseline_index_default_baseline_id
        .as_deref()
        .unwrap_or("none");
    let baseline_index_selected = report
        .baseline_index_selected_baseline_id
        .as_deref()
        .unwrap_or("none");
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let rows = report
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
        .join("\n  ");

    format!(
        "export package comparison {status}; ready {}->{}, delta {}; blocked {}->{}, delta {}; rejected {}->{}, delta {}; descriptors {}->{}, delta {}; templates {}->{}, delta {}; runbook entries {}->{}, delta {}; issue {issue}\n  baseline: {} ({})\n  baseline source: {} rev {}; package {}; manifest {}\n  candidate: {} rev {}; package {}; manifest {}\n  baseline identity: {}\n  baseline package: {}\n  baseline index: {}; default {}; selected {}\n  current root: {}\n  checks: {}; failed {}\n  entries:\n  {}",
        report.baseline_ready_count,
        report.candidate_ready_count,
        report.ready_delta,
        report.baseline_blocked_count,
        report.candidate_blocked_count,
        report.blocked_delta,
        report.baseline_rejected_count,
        report.candidate_rejected_count,
        report.rejected_delta,
        report.baseline_descriptor_count,
        report.candidate_descriptor_count,
        report.descriptor_delta,
        report.baseline_template_manifest_count,
        report.candidate_template_manifest_count,
        report.template_manifest_delta,
        report.baseline_runbook_entry_count,
        report.candidate_runbook_entry_count,
        report.runbook_entry_delta,
        baseline_id,
        baseline_label,
        report.baseline_project_id,
        report.baseline_project_revision,
        report.baseline_package_id,
        report.baseline_manifest_id,
        report.candidate_project_id,
        report.candidate_project_revision,
        report.candidate_package_id,
        report.candidate_manifest_id,
        baseline_path.display(),
        baseline_package,
        baseline_index_path,
        baseline_index_default,
        baseline_index_selected,
        bundle_root.display(),
        report.checks.len(),
        failed_checks,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
