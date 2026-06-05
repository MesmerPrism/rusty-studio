use super::super::super::*;

mod assignments;

use assignments::shell_hostess_owner_intake_assignment_rows;

pub(crate) fn shell_hostess_owner_intake_status(
    report: &StudioShellHostessOwnerIntakeReport,
    output_path: &Path,
) -> String {
    let status = shell_hostess_owner_intake_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let package_path = report.package_path.as_deref().unwrap_or("unknown");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let handoff_path = report.handoff_manifest_path.as_deref().unwrap_or("unknown");
    let assignments = shell_hostess_owner_intake_assignment_rows(report);
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
        "shell Hostess owner intake {status}; selected {selected}; issue {issue}\n  intake: {}\n  package: {}\n  handoff manifest: {}\n  project: {} rev {}\n  assignments ready {}; blocked {}; Hostess ready {}; Manifold ready {}\n  authority: command {}; host {}; studio {}; policy {}; intake owner {}; handoff owner {}\n  assignments:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        package_path,
        handoff_path,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.ready_assignment_count,
        report.blocked_assignment_count,
        report.hostess_ready_action_count,
        report.manifold_ready_action_count,
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
        report.intake_owner,
        report.handoff_owner,
        if assignments.is_empty() {
            "none".to_string()
        } else {
            assignments
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}
