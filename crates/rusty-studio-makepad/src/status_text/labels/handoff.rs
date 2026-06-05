use super::super::*;

pub(crate) fn shell_handoff_acceptance_baseline_selection_status_label(
    status: StudioShellHandoffAcceptanceBaselineSelectionStatus,
) -> &'static str {
    match status {
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected => "selected",
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Missing => "missing",
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Empty => "empty",
    }
}

pub(crate) fn shell_runbook_status_label(status: StudioShellRunbookStatus) -> &'static str {
    match status {
        StudioShellRunbookStatus::Ready => "ready",
        StudioShellRunbookStatus::Blocked => "blocked",
        StudioShellRunbookStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_handoff_acceptance_status_label(
    status: StudioShellHandoffAcceptanceStatus,
) -> &'static str {
    match status {
        StudioShellHandoffAcceptanceStatus::Ready => "ready",
        StudioShellHandoffAcceptanceStatus::Blocked => "blocked",
        StudioShellHandoffAcceptanceStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_handoff_acceptance_comparison_status_label(
    status: StudioShellHandoffAcceptanceComparisonStatus,
) -> &'static str {
    match status {
        StudioShellHandoffAcceptanceComparisonStatus::Improved => "improved",
        StudioShellHandoffAcceptanceComparisonStatus::Unchanged => "unchanged",
        StudioShellHandoffAcceptanceComparisonStatus::Regressed => "regressed",
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable => "incomparable",
    }
}

pub(crate) fn shell_handoff_acceptance_comparison_change_label(
    change: StudioShellHandoffAcceptanceComparisonChange,
) -> &'static str {
    match change {
        StudioShellHandoffAcceptanceComparisonChange::Added => "added",
        StudioShellHandoffAcceptanceComparisonChange::Removed => "removed",
        StudioShellHandoffAcceptanceComparisonChange::Improved => "improved",
        StudioShellHandoffAcceptanceComparisonChange::Unchanged => "unchanged",
        StudioShellHandoffAcceptanceComparisonChange::Regressed => "regressed",
        StudioShellHandoffAcceptanceComparisonChange::Changed => "changed",
    }
}
