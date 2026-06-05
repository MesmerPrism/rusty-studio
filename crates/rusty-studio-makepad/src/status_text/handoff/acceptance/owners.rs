use super::super::super::*;

pub(crate) fn shell_handoff_acceptance_owner_summary(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    graph_id: &str,
) -> String {
    let Some(entry) = report
        .entries
        .iter()
        .find(|entry| entry.graph_id == graph_id)
    else {
        return "none".to_string();
    };
    ["rusty.manifold", "rusty.hostess", "rusty.studio"]
        .iter()
        .map(|owner| {
            let owner_checks = entry
                .checks
                .iter()
                .filter(|check| check.owner.as_str() == *owner)
                .collect::<Vec<_>>();
            let status = if owner_checks.is_empty() {
                "none"
            } else if owner_checks
                .iter()
                .any(|check| check.status == StudioValidationStatus::Fail)
            {
                "fail"
            } else {
                "pass"
            };
            format!("{owner}:{status}")
        })
        .collect::<Vec<_>>()
        .join(", ")
}
