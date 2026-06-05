use super::*;

pub(crate) fn count_delta(candidate: usize, baseline: usize) -> isize {
    candidate as isize - baseline as isize
}

pub(crate) fn string_set(values: &[String]) -> BTreeSet<String> {
    values.iter().cloned().collect()
}

pub(crate) fn runtime_authority_matches(authority: &StudioShellRuntimeAuthority) -> bool {
    authority.command_session_authority == "rusty.manifold"
        && authority.install_launch_evidence_authority == "rusty.hostess"
        && authority.studio_role == "authoring.export_planning"
}

pub(crate) fn path_ends_with_shell_templates(path: &str) -> bool {
    !path.trim().is_empty() && path.replace('\\', "/").ends_with("/shell-templates.json")
}

pub(crate) fn same_unique_strings(actual: &[String], expected: &[String]) -> bool {
    actual.len() == expected.len()
        && actual.iter().collect::<BTreeSet<_>>() == expected.iter().collect::<BTreeSet<_>>()
}

pub(crate) fn unique_strings<I>(values: I) -> Vec<String>
where
    I: IntoIterator<Item = String>,
{
    let mut seen = BTreeSet::new();
    let mut unique = Vec::new();
    for value in values {
        if seen.insert(value.clone()) {
            unique.push(value);
        }
    }
    unique
}

pub(crate) fn shell_target_kinds() -> [StudioShellTargetKind; 4] {
    [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
        StudioShellTargetKind::Unknown,
    ]
}

pub(crate) fn shell_handoff_kind_for_target(
    target_kind: StudioShellTargetKind,
) -> StudioShellHandoffKind {
    match target_kind {
        StudioShellTargetKind::Desktop => StudioShellHandoffKind::DesktopShell,
        StudioShellTargetKind::Phone => StudioShellHandoffKind::PhoneShell,
        StudioShellTargetKind::Quest => StudioShellHandoffKind::QuestShell,
        StudioShellTargetKind::Unknown => StudioShellHandoffKind::UnknownShell,
    }
}

pub(crate) fn shell_handoff_consumer_id(target_kind: StudioShellTargetKind) -> &'static str {
    match target_kind {
        StudioShellTargetKind::Desktop => "rusty-studio-desktop-shell",
        StudioShellTargetKind::Phone => "rusty-studio-phone-shell",
        StudioShellTargetKind::Quest => "rusty-studio-quest-shell",
        StudioShellTargetKind::Unknown => "rusty-studio-operator-shell",
    }
}

pub(crate) fn shell_target_kind_label(target_kind: StudioShellTargetKind) -> &'static str {
    match target_kind {
        StudioShellTargetKind::Desktop => "desktop",
        StudioShellTargetKind::Phone => "phone",
        StudioShellTargetKind::Quest => "quest",
        StudioShellTargetKind::Unknown => "unknown",
    }
}
