use super::super::*;
use super::review::*;

pub(crate) fn shell_export_package_comparison_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellExportPackageComparisonReport, PathBuf, PathBuf), String> {
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    let index = load_shell_export_package_baseline_index(&index_path)
        .map_err(|error| format!("Export package baseline index load failed: {error}"))?;
    let Some(baseline_index_entry) = select_shell_export_package_baseline_index_entry(&index, None)
    else {
        return Err(
            "Export package baseline index does not contain a selected baseline".to_string(),
        );
    };
    let baseline_path = baseline_index_entry
        .baseline_manifest_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Selected export package baseline index entry does not include a baseline manifest path"
                .to_string()
        })?;
    let baseline_identity = load_shell_export_package_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Export package baseline identity load failed: {error}"))?;
    let package_path = PathBuf::from(&baseline_identity.package_path);
    let baseline = load_shell_export_package_report(&package_path)
        .map_err(|error| format!("Export package baseline review load failed: {error}"))?;
    let (candidate, bundle_root) = shell_export_package_for_project_source(project_path)?;
    let report = compare_shell_export_packages_against_baseline_index_entry(
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
