use super::*;

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

pub(crate) fn shell_template_lines(model: &StudioViewModel) -> String {
    let Some(preview) = model.shell_preview.as_ref() else {
        return "none".to_string();
    };
    if preview.status != StudioShellDescriptorStatus::Exported {
        return "none".to_string();
    }
    let mut lines = Vec::new();
    if let Some(template_id) = preview.template_id.as_deref() {
        lines.push(format!("template: {template_id}"));
    }
    if let Some(template_path) = preview.template_path.as_deref() {
        lines.push(format!("path: {template_path}"));
    }
    if let Some(descriptor_path) = preview.descriptor_path.as_deref() {
        lines.push(format!("descriptor: {descriptor_path}"));
    }
    if let Some(staged_descriptor_path) = preview.template_descriptor_path.as_deref() {
        lines.push(format!("staged descriptor: {staged_descriptor_path}"));
    }
    lines.push(format!(
        "authority: {} / {} / {}",
        preview
            .runtime_command_authority
            .as_deref()
            .unwrap_or("unknown"),
        preview
            .runtime_host_authority
            .as_deref()
            .unwrap_or("unknown"),
        preview.studio_role.as_deref().unwrap_or("unknown")
    ));
    lines.join("\n")
}
