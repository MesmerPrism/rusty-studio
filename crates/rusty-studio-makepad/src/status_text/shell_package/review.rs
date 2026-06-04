use super::super::*;

pub(crate) fn shell_export_package_status(
    report: &StudioShellExportPackageReport,
    bundle_root: &Path,
) -> String {
    let status = shell_export_package_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let target_rows = report
        .target_summaries
        .iter()
        .map(|target| {
            let consumers = if target.consumer_ids.is_empty() {
                "none".to_string()
            } else {
                target.consumer_ids.join(", ")
            };
            let owners = if target.responsible_owners.is_empty() {
                "none".to_string()
            } else {
                target.responsible_owners.join(", ")
            };
            let issues = if target.issue_codes.is_empty() {
                "none".to_string()
            } else {
                target.issue_codes.join(", ")
            };
            format!(
                "{}: ready {}; blocked {}; rejected {}; descriptors {}; templates {}; consumers {}; owners {}; issues {}",
                shell_target_kind_label(target.target_kind),
                target.ready_count,
                target.blocked_count,
                target.rejected_count,
                target.descriptor_count,
                target.template_manifest_count,
                consumers,
                owners,
                issues
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_export_package_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let descriptor = entry
                .descriptor
                .as_ref()
                .map(|descriptor| descriptor.descriptor_id.as_str())
                .unwrap_or("none");
            let template = entry
                .template_manifest
                .as_ref()
                .map(|template| template.template_id.as_str())
                .unwrap_or("none");
            let cli = if entry.runbook_cli_request.is_empty() {
                "none".to_string()
            } else {
                entry.runbook_cli_request.join(" ")
            };
            format!(
                "{} [{}] target {}; owner {}; action {}; policy {}; descriptor {}; template {}; cli {}; issue {}",
                entry.graph_id,
                entry_status,
                shell_target_kind_label(entry.target_kind),
                entry.responsible_owner,
                entry.next_required_action,
                entry.execution_policy,
                descriptor,
                template,
                cli,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

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
        if target_rows.is_empty() {
            "none".to_string()
        } else {
            target_rows
        },
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
