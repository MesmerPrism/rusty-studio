use super::super::super::*;

pub(crate) fn shell_hostess_staging_acceptance_status_label(
    status: StudioShellHostessStagingAcceptanceStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingAcceptanceStatus::Ready => "ready",
        StudioShellHostessStagingAcceptanceStatus::Blocked => "blocked",
        StudioShellHostessStagingAcceptanceStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_hostess_staging_acceptance_comparison_status_label(
    status: StudioShellHostessStagingAcceptanceComparisonStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingAcceptanceComparisonStatus::Improved => "improved",
        StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged => "unchanged",
        StudioShellHostessStagingAcceptanceComparisonStatus::Regressed => "regressed",
        StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable => "incomparable",
    }
}

pub(crate) fn shell_hostess_staging_acceptance_comparison_change_label(
    change: StudioShellHostessStagingAcceptanceComparisonChange,
) -> &'static str {
    match change {
        StudioShellHostessStagingAcceptanceComparisonChange::Added => "added",
        StudioShellHostessStagingAcceptanceComparisonChange::Removed => "removed",
        StudioShellHostessStagingAcceptanceComparisonChange::Improved => "improved",
        StudioShellHostessStagingAcceptanceComparisonChange::Unchanged => "unchanged",
        StudioShellHostessStagingAcceptanceComparisonChange::Regressed => "regressed",
        StudioShellHostessStagingAcceptanceComparisonChange::Changed => "changed",
    }
}

pub(crate) fn shell_hostess_staging_acceptance_selection_status_label(
    status: StudioShellHostessStagingAcceptanceSelectionStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingAcceptanceSelectionStatus::Selected => "selected",
        StudioShellHostessStagingAcceptanceSelectionStatus::Missing => "missing",
        StudioShellHostessStagingAcceptanceSelectionStatus::Empty => "empty",
    }
}
