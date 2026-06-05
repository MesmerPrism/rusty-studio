use super::super::super::*;
use super::default_id::next_shell_export_package_baseline_default_id;
use super::ShellExportPackageBaselineSelectionResult;

pub(crate) fn select_next_shell_export_package_baseline_default_for_project_source(
    project_path: &Path,
) -> ShellExportPackageBaselineSelectionResult {
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    let index = load_shell_export_package_baseline_index(&index_path)
        .map_err(|error| format!("Export package baseline index load failed: {error}"))?;
    let baseline_id = next_shell_export_package_baseline_default_id(&index)?;
    let baseline_path = index
        .entries
        .iter()
        .find(|entry| entry.baseline_id == baseline_id)
        .and_then(|entry| entry.baseline_manifest_path.as_ref())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "Export package baseline index entry {baseline_id} does not include a manifest path"
            )
        })?;
    let baseline = load_shell_export_package_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Export package baseline identity load failed: {error}"))?;
    let promoted =
        promote_shell_export_package_baseline_index_default(&index, &baseline.baseline_id)
            .ok_or_else(|| {
                format!(
                    "Export package baseline index does not contain baseline {}",
                    baseline.baseline_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Export package baseline index save failed: {error}"))?;
    Ok((baseline, promoted, baseline_path, index_path))
}
