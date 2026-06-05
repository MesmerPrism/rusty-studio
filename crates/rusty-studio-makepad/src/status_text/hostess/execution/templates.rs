use super::super::super::*;

pub(super) fn shell_hostess_staging_execution_project_revision(
    report: &StudioShellHostessStagingExecutionRequestReport,
) -> String {
    report
        .project_revision
        .map(|revision| revision.to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

pub(super) fn shell_hostess_staging_execution_failed_check_count(
    report: &StudioShellHostessStagingExecutionRequestReport,
) -> usize {
    report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count()
}

pub(super) fn shell_hostess_staging_execution_prohibited_actions(
    report: &StudioShellHostessStagingExecutionRequestReport,
) -> String {
    if report.prohibited_studio_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_studio_actions.join(", ")
    }
}

pub(super) fn shell_hostess_staging_execution_ack_template_summary(
    report: &StudioShellHostessStagingExecutionRequestReport,
) -> String {
    format!(
        "{} [{}]; required actions {}; Studio execution {}",
        report.ack_template.schema_id,
        shell_hostess_staging_execution_ack_status_label(report.ack_template.ack_status),
        report.ack_template.required_action_ids.len(),
        shell_hostess_staging_execution_bool_label(report.ack_template.execution_in_studio)
    )
}

pub(super) fn shell_hostess_staging_execution_reject_template_summary(
    report: &StudioShellHostessStagingExecutionRequestReport,
) -> String {
    format!(
        "{} [{}]; request actions {}; rejected actions {}; Studio execution {}",
        report.reject_template.schema_id,
        shell_hostess_staging_execution_reject_status_label(report.reject_template.reject_status),
        report.reject_template.request_action_ids.len(),
        report.reject_template.rejected_action_ids.len(),
        shell_hostess_staging_execution_bool_label(report.reject_template.execution_in_studio)
    )
}

fn shell_hostess_staging_execution_bool_label(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}
