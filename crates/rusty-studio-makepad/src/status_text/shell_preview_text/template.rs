use super::super::*;

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
