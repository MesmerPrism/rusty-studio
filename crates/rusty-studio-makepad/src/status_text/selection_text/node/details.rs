use super::super::super::*;

pub(crate) fn selected_node_detail_lines(model: &StudioViewModel) -> String {
    let Some(node) = model.selected_node.as_ref() else {
        if let Some(issue_code) = model.node_selection_code.as_deref() {
            return format!("none [{issue_code}]");
        }
        return "none".to_string();
    };
    let mut lines = Vec::new();
    if let Some(issue_code) = model.node_selection_code.as_deref() {
        lines.push(format!("selection: {issue_code}"));
    }
    lines.push(format!("graph: {}", node.graph_id));
    lines.push(format!("node: {}", node.node_id));
    lines.push(format!(
        "ref: {} [{}]",
        node.reference_id, node.reference_status
    ));
    if node.validation_issue_count > 0 {
        lines.push(format!("issues: {}", node.validation_issue_count));
    }
    if let Some(path) = node.package_manifest_path.as_deref() {
        lines.push(format!("manifest: {path}"));
    }
    if !node.package_module_ids.is_empty() {
        lines.push(format!("modules: {}", node.package_module_ids.join(", ")));
    }
    if !node.module_package_ids.is_empty() {
        lines.push(format!("packages: {}", node.module_package_ids.join(", ")));
    }
    if let Some(profile) = node.host_profile.as_ref() {
        let host = profile.host_profile.as_deref().unwrap_or("unknown host");
        let install = profile
            .install_route
            .as_deref()
            .unwrap_or("install route missing");
        let launch = profile
            .launch_route
            .as_deref()
            .unwrap_or("launch route missing");
        lines.push(format!("host: {host}"));
        lines.push(format!("routes: {install} / {launch}"));
    }
    lines.join("\n")
}
