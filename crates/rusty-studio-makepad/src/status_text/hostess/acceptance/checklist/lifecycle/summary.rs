use super::super::super::super::super::*;
use super::super::super::index::*;

pub(crate) fn shell_hostess_staging_acceptance_summary_status(
    acceptance: &StudioShellHostessStagingAcceptanceManifest,
    index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_path: &Path,
    index_path: &Path,
) -> String {
    let selection =
        summarize_shell_hostess_staging_acceptance_index_selection(index, Some(index_path), None);
    let status = shell_hostess_staging_acceptance_status_label(acceptance.status);
    let issue = acceptance.issue_code.as_deref().unwrap_or("none");
    format!(
        "Hostess staging acceptance summary {status}; acceptance {} ({}); project {} rev {}; envelope {}; issue {issue}\n  identity: {}\n  checklist: {}\n  items ready {}; blocked {}; rejected {}; requests {}; instructions {}\n  authority: command {}; host {}; studio {}; policy {}; checklist owner {}; handoff owner {}; staging owner {}\n{}\n{}",
        acceptance.acceptance_id,
        acceptance.label,
        acceptance.project_id.as_deref().unwrap_or("unknown"),
        acceptance
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        acceptance.envelope_id,
        acceptance_path.display(),
        acceptance.checklist_path,
        acceptance.ready_item_count,
        acceptance.blocked_item_count,
        acceptance.rejected_item_count,
        acceptance.request_count,
        acceptance.instruction_count,
        acceptance
            .command_session_authority
            .as_deref()
            .unwrap_or("unknown"),
        acceptance
            .install_launch_evidence_authority
            .as_deref()
            .unwrap_or("unknown"),
        acceptance.studio_role.as_deref().unwrap_or("unknown"),
        acceptance.execution_policy,
        acceptance.checklist_owner,
        acceptance.handoff_owner,
        acceptance.staging_owner,
        shell_hostess_staging_acceptance_selection_status(&selection),
        shell_hostess_staging_acceptance_index_status(index, index_path)
    )
}
