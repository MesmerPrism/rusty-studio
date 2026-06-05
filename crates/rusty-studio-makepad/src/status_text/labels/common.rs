use super::super::*;

pub(crate) fn present_label(present: bool) -> &'static str {
    if present {
        "present"
    } else {
        "missing"
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

pub(crate) fn validation_status_label(status: StudioValidationStatus) -> &'static str {
    match status {
        StudioValidationStatus::Pass => "pass",
        StudioValidationStatus::Fail => "fail",
    }
}
