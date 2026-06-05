use super::super::super::super::*;

pub(super) fn shell_hostess_owner_intake_assignment_rows(
    report: &StudioShellHostessOwnerIntakeReport,
) -> String {
    report
        .assignments
        .iter()
        .map(|assignment| {
            let assignment_status =
                shell_hostess_owner_intake_assignment_status_label(assignment.status);
            let issue = assignment.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] owner {}; request {}; source {}; next {}; prohibited in Studio {}; issue {}",
                assignment.action_id,
                assignment_status,
                assignment.owner,
                assignment.request_kind,
                assignment.source,
                assignment.next_required_action,
                if assignment.prohibited_in_studio {
                    "yes"
                } else {
                    "no"
                },
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ")
}
