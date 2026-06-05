use super::super::super::super::*;
use super::super::index::*;

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
