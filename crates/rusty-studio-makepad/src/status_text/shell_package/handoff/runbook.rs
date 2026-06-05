use super::super::super::*;

mod entries;
mod lists;
mod targets;

use entries::shell_runbook_entry_rows;
use lists::joined_list_or_none;
use targets::shell_runbook_target_rows;

pub(crate) fn shell_runbook_status(
    report: &StudioShellRunbookReport,
    bundle_root: &Path,
) -> String {
    let status = shell_runbook_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let prohibited = joined_list_or_none(&report.prohibited_actions, ", ");
    let target_rows = shell_runbook_target_rows(report);
    let rows = shell_runbook_entry_rows(report);

    format!(
        "shell runbook {status}; ready {}; blocked {}; rejected {}; issue {issue}\n  root: {}\n  bundle root: {}\n  prohibited: {}\n  targets:\n  {}\n  entries:\n  {}",
        report.ready_count,
        report.blocked_count,
        report.rejected_count,
        report.bundle_root,
        bundle_root.display(),
        prohibited,
        target_rows,
        rows
    )
}
