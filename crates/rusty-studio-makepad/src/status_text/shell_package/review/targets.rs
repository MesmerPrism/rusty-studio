use super::super::super::*;
use super::lists::joined_list_or_none;

pub(super) fn shell_export_package_target_rows(report: &StudioShellExportPackageReport) -> String {
    let rows = report
        .target_summaries
        .iter()
        .map(|target| {
            let consumers = joined_list_or_none(&target.consumer_ids, ", ");
            let owners = joined_list_or_none(&target.responsible_owners, ", ");
            let issues = joined_list_or_none(&target.issue_codes, ", ");
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
    if rows.is_empty() {
        "none".to_string()
    } else {
        rows
    }
}
