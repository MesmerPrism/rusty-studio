use super::super::*;
use super::review::shell_export_package_status;

pub(crate) fn shell_export_package_baseline_status(
    report: &StudioShellExportPackageReport,
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    package_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection =
        summarize_shell_export_package_baseline_index_selection(index, Some(index_path), None);
    format!(
        "export package baseline written\n  baseline: {} ({})\n  identity: {}\n  package: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        package_path.display(),
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path),
        shell_export_package_status(report, bundle_root)
    )
}

pub(crate) fn shell_export_package_baseline_append_status(
    report: &StudioShellExportPackageReport,
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    package_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection = summarize_shell_export_package_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "export package baseline archived\n  baseline: {} ({})\n  identity: {}\n  package: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        package_path.display(),
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path),
        shell_export_package_status(report, bundle_root)
    )
}

pub(crate) fn shell_export_package_baseline_index_status(
    index: &StudioShellExportPackageBaselineIndex,
    index_path: &Path,
) -> String {
    let default = index.default_baseline_id.as_deref().unwrap_or("none");
    let projects = if index.project_ids.is_empty() {
        "none".to_string()
    } else {
        index.project_ids.join(", ")
    };
    let packages = if index.package_ids.is_empty() {
        "none".to_string()
    } else {
        index.package_ids.join(", ")
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
            let status = shell_export_package_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry.baseline_manifest_path.as_deref().unwrap_or("unknown");
            format!(
                "{} [{}] project {} rev {}; ready {}; blocked {}; rejected {}; descriptors {}; templates {}; package {}; manifest {}; issue {}",
                entry.baseline_id,
                status,
                entry.project_id,
                entry.project_revision,
                entry.ready_count,
                entry.blocked_count,
                entry.rejected_count,
                entry.descriptor_count,
                entry.template_manifest_count,
                entry.package_path,
                manifest_path,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "export package baseline index slots {}; default {}; ready {}; blocked {}; rejected {}\n  index: {}\n  projects: {}\n  packages: {}\n  manifests: {}\n  entries:\n  {}",
        index.baseline_count,
        default,
        index.ready_baseline_count,
        index.blocked_baseline_count,
        index.rejected_baseline_count,
        index_path.display(),
        projects,
        packages,
        manifests,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

pub(crate) fn shell_export_package_baseline_selection_status(
    report: &StudioShellExportPackageBaselineSelectionReport,
) -> String {
    let status = shell_export_package_baseline_selection_status_label(report.status);
    let requested = report.requested_baseline_id.as_deref().unwrap_or("none");
    let default = report.default_baseline_id.as_deref().unwrap_or("none");
    let selected = report.selected_baseline_id.as_deref().unwrap_or("none");
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let index_path = report.index_path.as_deref().unwrap_or("not saved");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_export_package_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry.baseline_manifest_path.as_deref().unwrap_or("unknown");
            let selected_flag = if entry.selected { "yes" } else { "no" };
            let default_flag = if entry.default { "yes" } else { "no" };
            format!(
                "{} [{}] selected {}; default {}; ready {}; blocked {}; rejected {}; descriptors {}; templates {}; package {}; manifest {}; issue {}",
                entry.baseline_id,
                entry_status,
                selected_flag,
                default_flag,
                entry.ready_count,
                entry.blocked_count,
                entry.rejected_count,
                entry.descriptor_count,
                entry.template_manifest_count,
                entry.package_path,
                manifest_path,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "export package baseline selection {status}; requested {requested}; default {default}; selected {selected}; slots {}; ready {}; blocked {}; rejected {}; issue {issue}\n  index: {}\n  entries:\n  {}",
        report.baseline_count,
        report.ready_baseline_count,
        report.blocked_baseline_count,
        report.rejected_baseline_count,
        index_path,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

pub(crate) fn shell_export_package_baseline_summary_status(
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection =
        summarize_shell_export_package_baseline_index_selection(index, Some(index_path), None);
    let status = shell_export_package_status_label(baseline.status);
    let issue = baseline.issue_code.as_deref().unwrap_or("none");
    format!(
        "export package baseline summary {status}; baseline {} ({}); project {} rev {}; package {}; manifest {}; ready {}; blocked {}; rejected {}; descriptors {}; templates {}; runbook entries {}; targets {}; issue {issue}\n  identity: {}\n  package review: {}\n  authority: command {}; host {}; studio {}; policy {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline.project_id,
        baseline.project_revision,
        baseline.package_id,
        baseline.manifest_id,
        baseline.ready_count,
        baseline.blocked_count,
        baseline.rejected_count,
        baseline.descriptor_count,
        baseline.template_manifest_count,
        baseline.runbook_entry_count,
        baseline.target_count,
        baseline_path.display(),
        baseline.package_path,
        baseline.command_session_authority,
        baseline.install_launch_evidence_authority,
        baseline.studio_role,
        baseline.execution_policy,
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path)
    )
}

pub(crate) fn shell_export_package_baseline_promote_status(
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_export_package_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "export package baseline default promoted\n  baseline: {} ({})\n  identity: {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path)
    )
}

pub(crate) fn shell_export_package_baseline_select_status(
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_export_package_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "export package baseline default selected\n  baseline: {} ({})\n  identity: {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path)
    )
}
