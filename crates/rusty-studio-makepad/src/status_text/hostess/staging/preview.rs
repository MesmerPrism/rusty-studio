use super::super::super::*;

mod groups;

use groups::shell_hostess_staging_preview_group_rows;

pub(crate) fn shell_hostess_staging_preview_status(
    report: &StudioShellHostessStagingPreviewManifest,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_preview_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let intake_path = report.intake_path.as_deref().unwrap_or("unknown");
    let package_path = report.package_path.as_deref().unwrap_or("unknown");
    let handoff_path = report.handoff_manifest_path.as_deref().unwrap_or("unknown");
    let groups = shell_hostess_staging_preview_group_rows(report);
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
        "shell Hostess staging preview {status}; selected {selected}; issue {issue}\n  preview: {}\n  intake: {}\n  package: {}\n  handoff manifest: {}\n  project: {} rev {}\n  assignments ready {}; blocked {}; groups ready {}; blocked {}; artifacts {}\n  authority: command {}; host {}; studio {}; policy {}; staging owner {}\n  groups:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        intake_path,
        package_path,
        handoff_path,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.ready_assignment_count,
        report.blocked_assignment_count,
        report.ready_group_count,
        report.blocked_group_count,
        report.expected_artifact_count,
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
        if groups.is_empty() {
            "none".to_string()
        } else {
            groups
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}
