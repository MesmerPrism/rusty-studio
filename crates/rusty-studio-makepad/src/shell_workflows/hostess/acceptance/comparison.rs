use super::super::super::*;
use super::checklist::*;

pub(crate) fn shell_hostess_staging_acceptance_comparison_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHostessStagingAcceptanceComparisonReport,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let index = load_shell_hostess_staging_acceptance_index(&index_path)
        .map_err(|error| format!("Shell Hostess staging acceptance index load failed: {error}"))?;
    let Some(acceptance_index_entry) =
        select_shell_hostess_staging_acceptance_index_entry(&index, None)
    else {
        return Err(
            "Shell Hostess staging acceptance index does not contain a selected acceptance"
                .to_string(),
        );
    };
    let acceptance_path = acceptance_index_entry
        .acceptance_manifest_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Selected acceptance index entry does not include an acceptance manifest path"
                .to_string()
        })?;
    let baseline_identity = load_shell_hostess_staging_acceptance_manifest(&acceptance_path)
        .map_err(|error| {
            format!("Shell Hostess staging acceptance identity load failed: {error}")
        })?;
    let checklist_path = PathBuf::from(&baseline_identity.checklist_path);
    let baseline =
        load_shell_hostess_staging_acceptance_checklist(&checklist_path).map_err(|error| {
            format!("Shell Hostess staging acceptance checklist load failed: {error}")
        })?;
    let (candidate, _) = shell_hostess_staging_acceptance_for_project_source(project_path)?;
    let report = compare_shell_hostess_staging_acceptance_against_index_entry(
        &index,
        Some(&index_path),
        acceptance_index_entry,
        Some(&acceptance_path),
        &baseline_identity,
        &baseline,
        &candidate,
    );
    let output_path = shell_hostess_staging_acceptance_comparison_output_path(project_path);
    save_json(&output_path, &report).map_err(|error| {
        format!("Shell Hostess staging acceptance comparison save failed: {error}")
    })?;
    Ok((report, acceptance_path, output_path))
}
