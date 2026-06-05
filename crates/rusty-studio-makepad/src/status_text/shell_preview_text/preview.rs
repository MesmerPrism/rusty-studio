use super::super::*;

pub(crate) fn shell_preview_lines(model: &StudioViewModel) -> String {
    let Some(preview) = model.shell_preview.as_ref() else {
        return "none".to_string();
    };
    let mut lines = Vec::new();
    lines.push(format!(
        "{} [{}]",
        preview.graph_id,
        shell_descriptor_status_label(preview.status)
    ));
    if let Some(issue_code) = preview.issue_code.as_deref() {
        lines.push(format!("issue: {issue_code}"));
    }
    lines.push(preview.message.clone());
    if let Some(descriptor_id) = preview.descriptor_id.as_deref() {
        lines.push(format!("descriptor: {descriptor_id}"));
    }
    if let Some(shell_id) = preview.shell_id.as_deref() {
        lines.push(format!(
            "shell: {} / {}",
            shell_id,
            preview.shell_label.as_deref().unwrap_or("unlabeled")
        ));
    }
    if let Some(target_host_profile) = preview.target_host_profile.as_deref() {
        lines.push(format!(
            "target: {} / {}",
            target_host_profile,
            preview
                .target_kind
                .map(shell_target_kind_label)
                .unwrap_or("unknown")
        ));
    }
    lines.push(format!(
        "graph: {} package(s), {} module(s), {} stream binding(s), {} command binding(s)",
        preview.package_count,
        preview.module_count,
        preview.stream_binding_count,
        preview.command_binding_count
    ));
    if let Some(status) = preview.descriptor_validation_status {
        lines.push(format!(
            "descriptor validation: {}",
            validation_status_label(status)
        ));
    }
    lines.join("\n")
}
