use super::super::super::super::*;

pub(super) fn next_shell_hostess_staging_acceptance_archive_identity(
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
    index: Option<&StudioShellHostessStagingAcceptanceIndex>,
) -> (String, String) {
    let status = shell_hostess_staging_acceptance_status_label(checklist.status);
    let project_id = checklist.project_id.as_deref().unwrap_or("unknown_project");
    let revision = checklist
        .project_revision
        .map(|revision| revision.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let base_id = format!("studio.hostess_staging_acceptance.{project_id}.rev{revision}.{status}");
    let next_slot = index
        .map(|index| {
            index
                .entries
                .iter()
                .filter(|entry| {
                    entry.acceptance_id == base_id
                        || entry
                            .acceptance_id
                            .strip_prefix(base_id.as_str())
                            .is_some_and(|suffix| suffix.starts_with(".archive"))
                })
                .count()
                + 1
        })
        .unwrap_or(1);
    let acceptance_id = if next_slot == 1 {
        base_id
    } else {
        format!("{base_id}.archive{next_slot}")
    };
    let label = if next_slot == 1 {
        format!("{project_id} revision {revision} {status} Hostess staging acceptance")
    } else {
        format!(
            "{project_id} revision {revision} {status} Hostess staging acceptance archive {next_slot}"
        )
    };
    (acceptance_id, label)
}
