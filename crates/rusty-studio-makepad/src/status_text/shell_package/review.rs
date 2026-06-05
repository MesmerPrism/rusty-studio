use super::super::*;

mod entries;
mod lists;
mod targets;

use entries::shell_export_package_entry_rows;
use lists::joined_list_or_none;
use targets::shell_export_package_target_rows;

pub(crate) fn shell_export_package_status(
    report: &StudioShellExportPackageReport,
    bundle_root: &Path,
) -> String {
    let status = shell_export_package_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let prohibited = joined_list_or_none(&report.prohibited_actions, ", ");
    let target_rows = shell_export_package_target_rows(report);
    let rows = shell_export_package_entry_rows(report);

    format!(
        "shell export package {status}; ready {}; blocked {}; rejected {}; descriptors {}; templates {}; issue {issue}\n  package: {}\n  owner: {}; policy: {}\n  authority: command {}; host {}; studio {}\n  root: {}\n  bundle root: {}\n  prohibited: {}\n  targets:\n  {}\n  entries:\n  {}",
        report.ready_count,
        report.blocked_count,
        report.rejected_count,
        report.descriptor_count,
        report.template_manifest_count,
        report.package_id,
        report.review_owner,
        report.execution_policy,
        report.command_session_authority,
        report.install_launch_evidence_authority,
        report.studio_role,
        report.bundle_root,
        bundle_root.display(),
        prohibited,
        target_rows,
        rows
    )
}
