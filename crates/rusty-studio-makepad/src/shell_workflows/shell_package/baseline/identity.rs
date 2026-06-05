use super::super::super::*;

pub(crate) fn next_shell_export_package_baseline_archive_identity(
    report: &StudioShellExportPackageReport,
    index: Option<&StudioShellExportPackageBaselineIndex>,
) -> (String, String) {
    let status = shell_export_package_status_label(report.status);
    let base_id = format!(
        "{}.rev{}.{}",
        report.project_id, report.project_revision, status
    );
    let next_slot = index
        .map(|index| {
            index
                .entries
                .iter()
                .filter(|entry| {
                    entry.baseline_id == base_id
                        || entry
                            .baseline_id
                            .strip_prefix(base_id.as_str())
                            .is_some_and(|suffix| suffix.starts_with(".archive"))
                })
                .count()
                + 1
        })
        .unwrap_or(1);
    let baseline_id = if next_slot == 1 {
        base_id
    } else {
        format!("{base_id}.archive{next_slot}")
    };
    let label = if next_slot == 1 {
        format!(
            "{} revision {} {} export package baseline",
            report.project_id, report.project_revision, status
        )
    } else {
        format!(
            "{} revision {} {} export package baseline archive {}",
            report.project_id, report.project_revision, status, next_slot
        )
    };
    (baseline_id, label)
}
