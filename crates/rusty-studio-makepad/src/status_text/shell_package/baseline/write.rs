use super::super::super::*;
use super::super::review::shell_export_package_status;
use super::index::*;

pub(crate) fn shell_export_package_baseline_status(
    report: &StudioShellExportPackageReport,
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    package_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection =
        summarize_shell_export_package_baseline_index_selection(index, Some(index_path), None);
    format!(
        "export package baseline written\n  baseline: {} ({})\n  identity: {}\n  package: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        package_path.display(),
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path),
        shell_export_package_status(report, bundle_root)
    )
}

pub(crate) fn shell_export_package_baseline_append_status(
    report: &StudioShellExportPackageReport,
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    package_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection = summarize_shell_export_package_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "export package baseline archived\n  baseline: {} ({})\n  identity: {}\n  package: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        package_path.display(),
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path),
        shell_export_package_status(report, bundle_root)
    )
}
