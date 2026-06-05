use super::super::super::*;

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
