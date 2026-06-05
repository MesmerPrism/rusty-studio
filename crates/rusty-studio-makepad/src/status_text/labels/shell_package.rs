use super::super::*;

pub(crate) fn shell_export_package_status_label(
    status: StudioShellExportPackageStatus,
) -> &'static str {
    match status {
        StudioShellExportPackageStatus::Ready => "ready",
        StudioShellExportPackageStatus::Blocked => "blocked",
        StudioShellExportPackageStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_export_package_baseline_selection_status_label(
    status: StudioShellExportPackageBaselineSelectionStatus,
) -> &'static str {
    match status {
        StudioShellExportPackageBaselineSelectionStatus::Selected => "selected",
        StudioShellExportPackageBaselineSelectionStatus::Missing => "missing",
        StudioShellExportPackageBaselineSelectionStatus::Empty => "empty",
    }
}

pub(crate) fn shell_export_package_comparison_status_label(
    status: StudioShellExportPackageComparisonStatus,
) -> &'static str {
    match status {
        StudioShellExportPackageComparisonStatus::Improved => "improved",
        StudioShellExportPackageComparisonStatus::Unchanged => "unchanged",
        StudioShellExportPackageComparisonStatus::Regressed => "regressed",
        StudioShellExportPackageComparisonStatus::Incomparable => "incomparable",
    }
}

pub(crate) fn shell_export_package_comparison_change_label(
    change: StudioShellExportPackageComparisonChange,
) -> &'static str {
    match change {
        StudioShellExportPackageComparisonChange::Added => "added",
        StudioShellExportPackageComparisonChange::Removed => "removed",
        StudioShellExportPackageComparisonChange::Improved => "improved",
        StudioShellExportPackageComparisonChange::Unchanged => "unchanged",
        StudioShellExportPackageComparisonChange::Regressed => "regressed",
        StudioShellExportPackageComparisonChange::Changed => "changed",
    }
}
