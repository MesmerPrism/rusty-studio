use super::super::super::*;

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
