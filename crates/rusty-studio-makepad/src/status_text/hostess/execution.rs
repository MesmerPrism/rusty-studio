use super::super::*;

pub(crate) fn shell_hostess_staging_execution_request_status(
    report: &StudioShellHostessStagingExecutionRequestReport,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_execution_request_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let rows = report
        .actions
        .iter()
        .take(6)
        .map(|action| {
            let action_status = shell_hostess_staging_execution_action_status_label(action.status);
            let issue = action.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] owner {}; route {}; ack {}; Studio execution {}; next {}; issue {}",
                action.action_id,
                action_status,
                action.owner,
                action.route_kind,
                if action.ack_required { "yes" } else { "no" },
                if action.execution_in_studio {
                    "yes"
                } else {
                    "no"
                },
                action.next_required_action,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let prohibited = if report.prohibited_studio_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_studio_actions.join(", ")
    };
    format!(
        "Hostess staging execution request {status}; issue {issue}\n  request: {}\n  output: {}\n  selected acceptance: {}\n  acceptance manifest: {}\n  checklist: {}\n  handoff: {}\n  envelope id: {}\n  project: {} rev {}\n  checksum: {} ({})\n  adapter actions ready {}; blocked {}; total {}\n  authority: adapter {}; requester {}; command {}; host {}; studio {}; policy {}\n  ack template: {} [{}]; required actions {}; Studio execution {}\n  reject template: {} [{}]; request actions {}; rejected actions {}; Studio execution {}\n  prohibited: {}\n  checks: {}; failed {}\n  actions:\n  {}",
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
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
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
        report.ack_template.schema_id,
        shell_hostess_staging_execution_ack_status_label(report.ack_template.ack_status),
        report.ack_template.required_action_ids.len(),
        if report.ack_template.execution_in_studio {
            "yes"
        } else {
            "no"
        },
        report.reject_template.schema_id,
        shell_hostess_staging_execution_reject_status_label(report.reject_template.reject_status),
        report.reject_template.request_action_ids.len(),
        report.reject_template.rejected_action_ids.len(),
        if report.reject_template.execution_in_studio {
            "yes"
        } else {
            "no"
        },
        prohibited,
        report.checks.len(),
        failed_checks,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
