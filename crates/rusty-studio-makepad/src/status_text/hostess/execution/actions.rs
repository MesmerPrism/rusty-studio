use super::super::super::*;

pub(super) fn shell_hostess_staging_execution_action_rows(
    report: &StudioShellHostessStagingExecutionRequestReport,
) -> String {
    report
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
        .join("\n  ")
}
