use super::super::super::*;
use super::ShellExportPackageBaselineSelectionResult;

pub(crate) fn shell_export_package_baseline_summary_for_project_source(
    project_path: &Path,
) -> ShellExportPackageBaselineSelectionResult {
    let baseline_path = shell_export_package_baseline_manifest_output_path(project_path);
    let baseline = load_shell_export_package_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Export package baseline identity load failed: {error}"))?;
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    let index = load_shell_export_package_baseline_index(&index_path)
        .map_err(|error| format!("Export package baseline index load failed: {error}"))?;
    Ok((baseline, index, baseline_path, index_path))
}
