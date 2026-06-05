use super::super::super::*;

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
