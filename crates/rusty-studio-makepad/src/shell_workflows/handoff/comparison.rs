use super::super::*;
use super::acceptance::*;

pub(crate) fn shell_handoff_acceptance_comparison_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceComparisonReport,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let index = load_shell_handoff_acceptance_baseline_index(&index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    let Some(baseline_index_entry) =
        select_shell_handoff_acceptance_baseline_index_entry(&index, None)
    else {
        return Err("Baseline acceptance index does not contain a selected baseline".to_string());
    };
    let baseline_path = baseline_index_entry
        .baseline_manifest_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Selected baseline index entry does not include a baseline manifest path".to_string()
        })?;
    let baseline_identity = load_shell_handoff_acceptance_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Baseline acceptance identity load failed: {error}"))?;
    let checklist_path = PathBuf::from(&baseline_identity.checklist_path);
    let baseline = load_shell_handoff_acceptance_checklist(&checklist_path)
        .map_err(|error| format!("Baseline acceptance checklist load failed: {error}"))?;
    let (candidate, bundle_root) = shell_handoff_acceptance_for_project_source(project_path)?;
    let report = compare_shell_handoff_acceptance_against_baseline_index_entry(
        &index,
        Some(&index_path),
        baseline_index_entry,
        Some(&baseline_path),
        &baseline_identity,
        &baseline,
        &candidate,
    );
    Ok((report, baseline_path, bundle_root))
}
