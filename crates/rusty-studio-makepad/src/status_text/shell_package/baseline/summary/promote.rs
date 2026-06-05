use super::super::super::super::*;
use super::super::index::*;

pub(crate) fn shell_export_package_baseline_promote_status(
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_export_package_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "export package baseline default promoted\n  baseline: {} ({})\n  identity: {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path)
    )
}
