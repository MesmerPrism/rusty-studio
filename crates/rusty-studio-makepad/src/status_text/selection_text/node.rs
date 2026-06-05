use super::super::*;

mod details;
mod navigation;

pub(crate) use details::*;
pub(crate) use navigation::*;

pub(crate) fn selected_node_line(model: &StudioViewModel) -> String {
    let Some(node) = model.selected_node.as_ref() else {
        return "none".to_string();
    };
    let is_issue_node = model
        .focused_issue
        .as_ref()
        .and_then(|focus| focus.node_id.as_deref())
        == Some(node.node_id.as_str());
    let prefix = if is_issue_node { "issue: " } else { "" };
    format!("{prefix}{} / {}", node.label, node.kind)
}

pub(crate) fn selected_reference_line(model: &StudioViewModel) -> String {
    model.selected_node.as_ref().map_or_else(
        || "none".to_string(),
        |node| format!("{} [{}]", node.reference_id, node.reference_status.as_str()),
    )
}
