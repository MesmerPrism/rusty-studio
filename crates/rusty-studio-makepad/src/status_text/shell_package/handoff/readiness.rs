use super::super::super::*;

mod entries;
mod targets;

use entries::shell_handoff_readiness_entry_rows;
use targets::shell_handoff_readiness_target_rows;

pub(crate) fn shell_handoff_readiness_status(
    report: &StudioShellHandoffReadinessReport,
    bundle_root: &Path,
) -> String {
    let status = validation_status_label(report.status);
    let target_rows = shell_handoff_readiness_target_rows(report);
    let rows = shell_handoff_readiness_entry_rows(report);
    format!(
        "handoff readiness {status}; ready {}/{}; failed {}; missing {}\n  root: {}\n  targets:\n  {}\n  graphs:\n  {}",
        report.ready_count,
        report.graph_count,
        report.failed_count,
        report.missing_bundle_count,
        bundle_root.display(),
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
