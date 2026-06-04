use super::super::*;

pub(crate) fn shell_bundle_status_line(status: &str) -> String {
    if status.is_empty() {
        "not exported".to_string()
    } else {
        status.to_string()
    }
}

pub(crate) fn shell_bundle_export_status(
    report: &StudioShellBundleReport,
    output_dir: &Path,
) -> String {
    let status = shell_bundle_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    if report.status != StudioShellBundleStatus::Exported {
        return format!(
            "{status}; issue {issue}\n  graph: {}\n  {}",
            report.graph_id, report.message
        );
    }
    let files = if report.bundle_files.is_empty() {
        "none".to_string()
    } else {
        report.bundle_files.join("\n  ")
    };
    format!(
        "{status}; issue {issue}\n  graph: {}\n  output: {}\n  files:\n  {}",
        report.graph_id,
        output_dir.display(),
        files
    )
}

pub(crate) fn shell_bundle_validation_status(
    report: &StudioShellBundleValidationReport,
    output_dir: &Path,
) -> String {
    let status = validation_status_label(report.status);
    let failed = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .collect::<Vec<_>>();
    if failed.is_empty() {
        return format!(
            "validated; status {status}\n  graph: {}\n  output: {}\n  files: {}",
            report.graph_id,
            output_dir.display(),
            report.expected_bundle_files.len()
        );
    }
    let issues = failed
        .iter()
        .take(4)
        .map(|check| {
            format!(
                "{}: {}",
                check
                    .issue_code
                    .as_deref()
                    .unwrap_or("studio.issue.unknown"),
                check.evidence
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    format!(
        "validated; status {status}\n  graph: {}\n  output: {}\n  failed: {}\n  {}",
        report.graph_id,
        output_dir.display(),
        failed.len(),
        issues
    )
}
