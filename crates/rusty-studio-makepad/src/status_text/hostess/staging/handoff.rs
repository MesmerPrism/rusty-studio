use super::super::super::*;

mod instructions;
mod requests;

use instructions::shell_hostess_staging_handoff_instruction_rows;
use requests::shell_hostess_staging_handoff_request_rows;

pub(crate) fn shell_hostess_staging_handoff_status(
    report: &StudioShellHostessStagingHandoffEnvelope,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_handoff_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let file_plan_path = report.file_plan_path.as_deref().unwrap_or("unknown");
    let instructions = shell_hostess_staging_handoff_instruction_rows(report);
    let requests = shell_hostess_staging_handoff_request_rows(report);
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    format!(
        "shell Hostess staging handoff {status}; selected {selected}; issue {issue}\n  envelope: {}\n  file plan: {}\n  envelope id: {}\n  project: {} rev {}\n  checksum: {} ({})\n  requests ready {}; blocked {}; target {}; shared {}; planned files {}\n  instructions ready {}; blocked {}\n  authority: command {}; host {}; studio {}; policy {}; handoff owner {}; staging owner {}\n  requests:\n  {}\n  instructions:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        file_plan_path,
        report.envelope_id,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.provenance.plan_checksum,
        report.provenance.checksum_algorithm,
        report.ready_request_count,
        report.blocked_request_count,
        report.target_request_count,
        report.shared_request_count,
        report.planned_file_count,
        report.ready_instruction_count,
        report.blocked_instruction_count,
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
        report.handoff_owner,
        report.staging_owner,
        if requests.is_empty() {
            "none".to_string()
        } else {
            requests
        },
        if instructions.is_empty() {
            "none".to_string()
        } else {
            instructions
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}
