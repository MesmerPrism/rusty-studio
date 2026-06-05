use super::super::super::super::*;

mod entries;

use entries::shell_hostess_staging_acceptance_entry_rows;

pub(crate) fn shell_hostess_staging_acceptance_status(
    report: &StudioShellHostessStagingAcceptanceChecklistReport,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_acceptance_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let handoff_path = report.handoff_path.as_deref().unwrap_or("unknown");
    let file_plan_path = report.file_plan_path.as_deref().unwrap_or("unknown");
    let entries = shell_hostess_staging_acceptance_entry_rows(report);
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let failed_checks = report
        .handoff_checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    format!(
        "shell Hostess staging acceptance {status}; issue {issue}\n  checklist: {}\n  handoff: {}\n  file plan: {}\n  envelope id: {}\n  project: {} rev {}\n  checksum: {} ({})\n  requests ready {}; blocked {}; instructions ready {}; blocked {}\n  items ready {}; blocked {}; rejected {}\n  authority: command {}; host {}; studio {}; policy {}; checklist owner {}; handoff owner {}; staging owner {}\n  entries:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        handoff_path,
        file_plan_path,
        report.envelope_id,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.plan_checksum,
        report.checksum_algorithm,
        report.ready_request_count,
        report.blocked_request_count,
        report.ready_instruction_count,
        report.blocked_instruction_count,
        report.ready_item_count,
        report.blocked_item_count,
        report.rejected_item_count,
        report
            .command_session_authority
            .as_deref()
            .unwrap_or("unknown"),
        report
            .install_launch_evidence_authority
            .as_deref()
            .unwrap_or("unknown"),
        report.studio_role.as_deref().unwrap_or("unknown"),
        report.execution_policy,
        report.checklist_owner,
        report.handoff_owner,
        report.staging_owner,
        if entries.is_empty() {
            "none".to_string()
        } else {
            entries
        },
        prohibited,
        report.handoff_checks.len(),
        failed_checks
    )
}
