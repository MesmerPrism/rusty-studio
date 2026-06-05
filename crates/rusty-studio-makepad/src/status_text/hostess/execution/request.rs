use super::super::super::*;

use super::actions::shell_hostess_staging_execution_action_rows;
use super::templates::{
    shell_hostess_staging_execution_ack_template_summary,
    shell_hostess_staging_execution_failed_check_count,
    shell_hostess_staging_execution_prohibited_actions,
    shell_hostess_staging_execution_project_revision,
    shell_hostess_staging_execution_reject_template_summary,
};

pub(crate) fn shell_hostess_staging_execution_request_status(
    report: &StudioShellHostessStagingExecutionRequestReport,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_execution_request_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let rows = shell_hostess_staging_execution_action_rows(report);
    let ack_template = shell_hostess_staging_execution_ack_template_summary(report);
    let reject_template = shell_hostess_staging_execution_reject_template_summary(report);
    let prohibited = shell_hostess_staging_execution_prohibited_actions(report);
    let failed_checks = shell_hostess_staging_execution_failed_check_count(report);
    format!(
        "Hostess staging execution request {status}; issue {issue}\n  request: {}\n  output: {}\n  selected acceptance: {}\n  acceptance manifest: {}\n  checklist: {}\n  handoff: {}\n  envelope id: {}\n  project: {} rev {}\n  checksum: {} ({})\n  adapter actions ready {}; blocked {}; total {}\n  authority: adapter {}; requester {}; command {}; host {}; studio {}; policy {}\n  ack template: {ack_template}\n  reject template: {reject_template}\n  prohibited: {prohibited}\n  checks: {}; failed {failed_checks}\n  actions:\n  {}",
        report.request_id,
        output_path.display(),
        report.selected_acceptance_id,
        report
            .acceptance_manifest_path
            .as_deref()
            .unwrap_or("unknown"),
        report.acceptance_checklist_path,
        report.handoff_path.as_deref().unwrap_or("unknown"),
        report.envelope_id,
        report.project_id.as_deref().unwrap_or("unknown"),
        shell_hostess_staging_execution_project_revision(report),
        report.plan_checksum,
        report.checksum_algorithm,
        report.ready_adapter_action_count,
        report.blocked_adapter_action_count,
        report.adapter_action_count,
        report.adapter_owner,
        report.requester_role,
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
        report.checks.len(),
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
