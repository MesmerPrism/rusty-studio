use super::super::super::super::*;

pub(super) fn shell_hostess_handoff_package_action_rows(
    report: &StudioShellHostessHandoffPackageReport,
) -> String {
    report
        .required_owner_actions
        .iter()
        .map(|action| {
            let action_status = shell_hostess_handoff_package_action_status_label(action.status);
            let issue = action.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] owner {}; source {}; next {}; prohibited in Studio {}; issue {}",
                action.action_id,
                action_status,
                action.owner,
                action.source,
                action.next_required_action,
                if action.prohibited_in_studio {
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
