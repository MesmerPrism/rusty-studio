use super::super::*;

mod owners;
mod rows;

use rows::shell_handoff_acceptance_entry_rows;

pub(crate) fn shell_handoff_acceptance_status(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    bundle_root: &Path,
) -> String {
    let status = shell_handoff_acceptance_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let failed_intake_checks = report
        .intake_checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let rows = shell_handoff_acceptance_entry_rows(report);
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    format!(
        "handoff acceptance {status}; ready {}; blocked {}; rejected {}; issue {issue}\n  root: {}\n  prohibited: {}\n  intake checks: {}; failed {}\n  entries:\n  {}",
        report.ready_count,
        report.blocked_count,
        report.rejected_count,
        bundle_root.display(),
        prohibited,
        report.intake_checks.len(),
        failed_intake_checks,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
