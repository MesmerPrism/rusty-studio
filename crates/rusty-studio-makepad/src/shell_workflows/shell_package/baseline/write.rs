use super::super::super::*;
use super::super::review::*;
use super::ShellExportPackageBaselineWriteResult;

pub(crate) fn write_shell_export_package_baseline_for_project_source(
    project_path: &Path,
) -> ShellExportPackageBaselineWriteResult {
    let (report, bundle_root) = shell_export_package_for_project_source(project_path)?;
    let package_path = shell_export_package_output_path(project_path);
    save_json(&package_path, &report)
        .map_err(|error| format!("Shell export package review save failed: {error}"))?;
    let baseline =
        shell_export_package_baseline_manifest_for_report(&report, &package_path, None, None);
    let baseline_path = shell_export_package_baseline_manifest_output_path(project_path);
    save_json(&baseline_path, &baseline)
        .map_err(|error| format!("Shell export package baseline identity save failed: {error}"))?;
    let index = shell_export_package_baseline_index_for_manifests(
        vec![(baseline.clone(), Some(baseline_path.clone()))],
        Some(&baseline.baseline_id),
    );
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell export package baseline index save failed: {error}"))?;
    Ok((
        report,
        baseline,
        index,
        package_path,
        baseline_path,
        index_path,
        bundle_root,
    ))
}
