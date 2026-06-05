use super::super::*;

pub(crate) fn issue_focus_line(model: &StudioViewModel) -> String {
    let Some(focus) = model.focused_issue.as_ref() else {
        if let Some(issue_code) = model.issue_selection_code.as_deref() {
            return format!("none [{issue_code}]");
        }
        return "none".to_string();
    };
    let issue_code = focus.issue_code.as_deref().unwrap_or("unknown_issue");
    let mut lines = vec![format!(
        "#{} {} [{}]",
        focus.issue_index + 1,
        focus.check_id,
        issue_code
    )];
    if let Some(selection_issue_code) = model.issue_selection_code.as_deref() {
        lines.push(format!("  selection: {selection_issue_code}"));
    }
    lines.push(format!("  graph: {}", focus.graph_id));
    if let Some(node_id) = focus.node_id.as_deref() {
        lines.push(format!("  node: {node_id}"));
    }
    if let Some(edge_id) = focus.edge_id.as_deref() {
        lines.push(format!("  edge: {edge_id}"));
    }
    if let Some(reference_id) = focus.reference_id.as_deref() {
        lines.push(format!("  ref: {reference_id}"));
    }
    lines.push(format!("  {}", focus.evidence));
    lines.join("\n")
}

pub(crate) fn next_issue_check_id(model: &StudioViewModel) -> Option<&str> {
    if model.validation_issues.is_empty() {
        return None;
    }
    let next_index = model
        .selected_issue_index
        .map(|index| (index + 1) % model.validation_issues.len())
        .unwrap_or(0);
    model
        .validation_issues
        .get(next_index)
        .map(|issue| issue.check_id.as_str())
}
