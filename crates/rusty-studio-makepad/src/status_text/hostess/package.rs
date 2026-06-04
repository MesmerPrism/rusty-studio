use super::super::*;

pub(crate) fn shell_hostess_handoff_package_status(
    report: &StudioShellHostessHandoffPackageReport,
    output_path: &Path,
) -> String {
    let status = shell_hostess_handoff_package_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let candidate_path = report
        .candidate_manifest_path
        .as_deref()
        .unwrap_or("unknown");
    let review_path = report.review_path.as_deref().unwrap_or("unknown");
    let handoff_path = report.handoff_manifest_path.as_deref().unwrap_or("unknown");
    let acceptance = report
        .acceptance_comparison_status
        .map(shell_handoff_acceptance_comparison_status_label)
        .unwrap_or("missing");
    let export_package = report
        .export_package_comparison_status
        .map(shell_export_package_comparison_status_label)
        .unwrap_or("missing");
    let actions = report
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
        .join("\n  ");
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
        "shell Hostess handoff package {status}; selected {selected}; issue {issue}\n  package: {}\n  candidate: {}\n  review: {}\n  handoff manifest: {}\n  project: {} rev {}\n  handoff ready {}; failed {}; missing {}; acceptance {}; export package {}\n  authority: command {}; host {}; studio {}; policy {}; owner {}\n  actions:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        candidate_path,
        review_path,
        handoff_path,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.handoff_ready_count,
        report.handoff_failed_count,
        report.handoff_missing_bundle_count,
        acceptance,
        export_package,
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
        if actions.is_empty() {
            "none".to_string()
        } else {
            actions
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}

pub(crate) fn shell_hostess_owner_intake_status(
    report: &StudioShellHostessOwnerIntakeReport,
    output_path: &Path,
) -> String {
    let status = shell_hostess_owner_intake_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let package_path = report.package_path.as_deref().unwrap_or("unknown");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let handoff_path = report.handoff_manifest_path.as_deref().unwrap_or("unknown");
    let assignments = report
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
        .join("\n  ");
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
