use super::super::super::*;
use super::super::review::*;
use super::identity::next_shell_export_package_baseline_archive_identity;
use super::ShellExportPackageBaselineWriteResult;

pub(crate) fn append_shell_export_package_baseline_for_project_source(
    project_path: &Path,
) -> ShellExportPackageBaselineWriteResult {
    let (report, bundle_root) = shell_export_package_for_project_source(project_path)?;
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    let existing_index = if index_path.is_file() {
        Some(
            load_shell_export_package_baseline_index(&index_path)
                .map_err(|error| format!("Export package baseline index load failed: {error}"))?,
        )
    } else {
        None
    };
    let (baseline_id, label) =
        next_shell_export_package_baseline_archive_identity(&report, existing_index.as_ref());
    let package_path =
        shell_export_package_baseline_archive_package_output_path(project_path, &baseline_id);
    save_json(&package_path, &report)
        .map_err(|error| format!("Shell export package baseline review save failed: {error}"))?;
    let baseline = shell_export_package_baseline_manifest_for_report(
        &report,
        &package_path,
        Some(&baseline_id),
        Some(&label),
    );
    let baseline_path =
        shell_export_package_baseline_archive_manifest_output_path(project_path, &baseline_id);
    save_json(&baseline_path, &baseline)
        .map_err(|error| format!("Shell export package baseline identity save failed: {error}"))?;
    let index = if let Some(index) = existing_index.as_ref() {
        append_shell_export_package_baseline_index_manifests(
            index,
            vec![(baseline.clone(), Some(baseline_path.clone()))],
            Some(&baseline.baseline_id),
        )
    } else {
        shell_export_package_baseline_index_for_manifests(
            vec![(baseline.clone(), Some(baseline_path.clone()))],
            Some(&baseline.baseline_id),
        )
    };
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
