use super::super::super::*;
use super::ShellExportPackageBaselineSelectionResult;

pub(crate) fn promote_shell_export_package_baseline_default_for_project_source(
    project_path: &Path,
) -> ShellExportPackageBaselineSelectionResult {
    let baseline_path = shell_export_package_baseline_manifest_output_path(project_path);
    let baseline = load_shell_export_package_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Export package baseline identity load failed: {error}"))?;
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    let index = load_shell_export_package_baseline_index(&index_path)
        .map_err(|error| format!("Export package baseline index load failed: {error}"))?;
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
