use super::super::super::*;

mod requests;

use requests::shell_hostess_staging_file_plan_request_rows;

pub(crate) fn shell_hostess_staging_file_plan_status(
    report: &StudioShellHostessStagingFilePlan,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_file_plan_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let preview_path = report.preview_path.as_deref().unwrap_or("unknown");
    let requests = shell_hostess_staging_file_plan_request_rows(report);
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
        "shell Hostess staging file plan {status}; selected {selected}; issue {issue}\n  file plan: {}\n  preview: {}\n  project: {} rev {}\n  preview groups ready {}; blocked {}; source artifacts {}; planned files {}; duplicates {}\n  requests ready {}; blocked {}; target {}; shared {}\n  authority: command {}; host {}; studio {}; policy {}; staging owner {}\n  requests:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        preview_path,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.ready_preview_group_count,
        report.blocked_preview_group_count,
        report.source_artifact_count,
        report.planned_file_count,
        report.duplicate_artifact_count,
        report.ready_request_count,
        report.blocked_request_count,
        report.target_request_count,
        report.shared_request_count,
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
        report.staging_owner,
        if requests.is_empty() {
            "none".to_string()
        } else {
            requests
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}
