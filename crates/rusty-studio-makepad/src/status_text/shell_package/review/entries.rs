use super::super::super::*;
use super::lists::joined_list_or_none;

pub(super) fn shell_export_package_entry_rows(report: &StudioShellExportPackageReport) -> String {
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
            let cli = joined_list_or_none(&entry.runbook_cli_request, " ");
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
    if rows.is_empty() {
        "none".to_string()
    } else {
        rows
    }
}
