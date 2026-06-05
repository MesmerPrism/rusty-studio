use super::super::super::super::*;
use super::super::checklist::*;
use super::identity::next_shell_hostess_staging_acceptance_archive_identity;

pub(crate) fn append_shell_hostess_staging_acceptance_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHostessStagingAcceptanceManifest,
        StudioShellHostessStagingAcceptanceIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let (checklist, _) = shell_hostess_staging_acceptance_for_project_source(project_path)?;
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let existing_index = if index_path.is_file() {
        Some(
            load_shell_hostess_staging_acceptance_index(&index_path).map_err(|error| {
                format!("Shell Hostess staging acceptance index load failed: {error}")
            })?,
        )
    } else {
        None
    };
    let (acceptance_id, label) =
        next_shell_hostess_staging_acceptance_archive_identity(&checklist, existing_index.as_ref());
    let checklist_path = shell_hostess_staging_acceptance_archive_checklist_output_path(
        project_path,
        &acceptance_id,
    );
    save_json(&checklist_path, &checklist).map_err(|error| {
        format!("Shell Hostess staging acceptance checklist archive save failed: {error}")
    })?;
    let acceptance = shell_hostess_staging_acceptance_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some(&acceptance_id),
        Some(&label),
    );
    let acceptance_path =
        shell_hostess_staging_acceptance_archive_manifest_output_path(project_path, &acceptance_id);
    save_json(&acceptance_path, &acceptance).map_err(|error| {
        format!("Shell Hostess staging acceptance identity save failed: {error}")
    })?;
    save_json(
        &shell_hostess_staging_acceptance_manifest_output_path(project_path),
        &acceptance,
    )
    .map_err(|error| {
        format!("Shell Hostess staging acceptance current identity save failed: {error}")
    })?;
    let index = if let Some(index) = existing_index.as_ref() {
        append_shell_hostess_staging_acceptance_index_manifests(
            index,
            vec![(acceptance.clone(), Some(acceptance_path.clone()))],
            Some(&acceptance.acceptance_id),
        )
    } else {
        shell_hostess_staging_acceptance_index_for_manifests(
            vec![(acceptance.clone(), Some(acceptance_path.clone()))],
            Some(&acceptance.acceptance_id),
        )
    };
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell Hostess staging acceptance index save failed: {error}"))?;
    Ok((acceptance, index, acceptance_path, index_path))
}
