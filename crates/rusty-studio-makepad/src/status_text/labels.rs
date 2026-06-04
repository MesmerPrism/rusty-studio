use super::*;

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

pub(crate) fn present_label(present: bool) -> &'static str {
    if present {
        "present"
    } else {
        "missing"
    }
}

pub(crate) fn shell_release_candidate_review_selection_status_label(
    status: StudioShellReleaseCandidateReviewSelectionStatus,
) -> &'static str {
    match status {
        StudioShellReleaseCandidateReviewSelectionStatus::Selected => "selected",
        StudioShellReleaseCandidateReviewSelectionStatus::Missing => "missing",
        StudioShellReleaseCandidateReviewSelectionStatus::Empty => "empty",
    }
}

pub(crate) fn shell_release_candidate_review_status_label(
    status: StudioShellReleaseCandidateReviewStatus,
) -> &'static str {
    match status {
        StudioShellReleaseCandidateReviewStatus::Ready => "ready",
        StudioShellReleaseCandidateReviewStatus::Blocked => "blocked",
        StudioShellReleaseCandidateReviewStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_hostess_handoff_package_status_label(
    status: StudioShellHostessHandoffPackageStatus,
) -> &'static str {
    match status {
        StudioShellHostessHandoffPackageStatus::Ready => "ready",
        StudioShellHostessHandoffPackageStatus::Blocked => "blocked",
        StudioShellHostessHandoffPackageStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_hostess_handoff_package_action_status_label(
    status: StudioShellHostessHandoffPackageActionStatus,
) -> &'static str {
    match status {
        StudioShellHostessHandoffPackageActionStatus::Ready => "ready",
        StudioShellHostessHandoffPackageActionStatus::Blocked => "blocked",
    }
}

pub(crate) fn shell_hostess_owner_intake_status_label(
    status: StudioShellHostessOwnerIntakeStatus,
) -> &'static str {
    match status {
        StudioShellHostessOwnerIntakeStatus::Ready => "ready",
        StudioShellHostessOwnerIntakeStatus::Blocked => "blocked",
        StudioShellHostessOwnerIntakeStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_hostess_owner_intake_assignment_status_label(
    status: StudioShellHostessOwnerIntakeAssignmentStatus,
) -> &'static str {
    match status {
        StudioShellHostessOwnerIntakeAssignmentStatus::Ready => "ready",
        StudioShellHostessOwnerIntakeAssignmentStatus::Blocked => "blocked",
    }
}

pub(crate) fn shell_hostess_staging_preview_status_label(
    status: StudioShellHostessStagingPreviewStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingPreviewStatus::Ready => "ready",
        StudioShellHostessStagingPreviewStatus::Blocked => "blocked",
        StudioShellHostessStagingPreviewStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_hostess_staging_preview_group_status_label(
    status: StudioShellHostessStagingPreviewGroupStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingPreviewGroupStatus::Ready => "ready",
        StudioShellHostessStagingPreviewGroupStatus::Blocked => "blocked",
    }
}

pub(crate) fn shell_hostess_staging_file_plan_status_label(
    status: StudioShellHostessStagingFilePlanStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingFilePlanStatus::Ready => "ready",
        StudioShellHostessStagingFilePlanStatus::Blocked => "blocked",
        StudioShellHostessStagingFilePlanStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_hostess_staging_file_request_status_label(
    status: StudioShellHostessStagingFileRequestStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingFileRequestStatus::Ready => "ready",
        StudioShellHostessStagingFileRequestStatus::Blocked => "blocked",
    }
}

pub(crate) fn shell_hostess_staging_handoff_status_label(
    status: StudioShellHostessStagingHandoffEnvelopeStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingHandoffEnvelopeStatus::Ready => "ready",
        StudioShellHostessStagingHandoffEnvelopeStatus::Blocked => "blocked",
        StudioShellHostessStagingHandoffEnvelopeStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_hostess_staging_handoff_instruction_status_label(
    status: StudioShellHostessStagingHandoffInstructionStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingHandoffInstructionStatus::Ready => "ready",
        StudioShellHostessStagingHandoffInstructionStatus::Blocked => "blocked",
    }
}

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

pub(crate) fn shell_hostess_staging_execution_request_status_label(
    status: StudioShellHostessStagingExecutionRequestStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingExecutionRequestStatus::Ready => "ready",
        StudioShellHostessStagingExecutionRequestStatus::Blocked => "blocked",
        StudioShellHostessStagingExecutionRequestStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_hostess_staging_execution_action_status_label(
    status: StudioShellHostessStagingExecutionActionStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingExecutionActionStatus::Ready => "ready",
        StudioShellHostessStagingExecutionActionStatus::Blocked => "blocked",
    }
}

pub(crate) fn shell_hostess_staging_execution_ack_status_label(
    status: rusty_studio_model::StudioShellHostessStagingExecutionAckStatus,
) -> &'static str {
    match status {
        rusty_studio_model::StudioShellHostessStagingExecutionAckStatus::Pending => "pending",
        rusty_studio_model::StudioShellHostessStagingExecutionAckStatus::Accepted => "accepted",
    }
}

pub(crate) fn shell_hostess_staging_execution_reject_status_label(
    status: rusty_studio_model::StudioShellHostessStagingExecutionRejectStatus,
) -> &'static str {
    match status {
        rusty_studio_model::StudioShellHostessStagingExecutionRejectStatus::Pending => "pending",
        rusty_studio_model::StudioShellHostessStagingExecutionRejectStatus::Rejected => "rejected",
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

pub(crate) fn shell_bundle_status_label(status: StudioShellBundleStatus) -> &'static str {
    match status {
        StudioShellBundleStatus::Exported => "exported",
        StudioShellBundleStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_descriptor_status_label(status: StudioShellDescriptorStatus) -> &'static str {
    match status {
        StudioShellDescriptorStatus::Exported => "exported",
        StudioShellDescriptorStatus::Rejected => "rejected",
    }
}

pub(crate) fn shell_target_kind_label(kind: StudioShellTargetKind) -> &'static str {
    match kind {
        StudioShellTargetKind::Desktop => "desktop",
        StudioShellTargetKind::Phone => "phone",
        StudioShellTargetKind::Quest => "quest",
        StudioShellTargetKind::Unknown => "unknown",
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

pub(crate) fn validation_status_label(status: StudioValidationStatus) -> &'static str {
    match status {
        StudioValidationStatus::Pass => "pass",
        StudioValidationStatus::Fail => "fail",
    }
}
