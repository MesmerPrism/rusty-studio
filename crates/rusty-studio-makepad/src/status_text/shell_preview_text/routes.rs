use super::super::*;

pub(crate) fn shell_route_lines(model: &StudioViewModel) -> String {
    let Some(preview) = model.shell_preview.as_ref() else {
        return "none".to_string();
    };
    if preview.status != StudioShellDescriptorStatus::Exported {
        return "none".to_string();
    }
    let mut lines = Vec::new();
    lines.push(format!(
        "host: {}",
        preview.host_profile_class.as_deref().unwrap_or("unknown")
    ));
    lines.push(format!(
        "app: {}",
        preview.app_id.as_deref().unwrap_or("not declared")
    ));
    lines.push(format!(
        "install: {}",
        preview.install_route.as_deref().unwrap_or("not declared")
    ));
    lines.push(format!(
        "launch: {}",
        preview.launch_route.as_deref().unwrap_or("not declared")
    ));
    lines.push(format!(
        "command: {}",
        preview.command_bridge.as_deref().unwrap_or("not declared")
    ));
    lines.push(format!(
        "evidence: {}",
        preview
            .evidence_pull_route
            .as_deref()
            .unwrap_or("not declared")
    ));
    lines.join("\n")
}
