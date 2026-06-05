use super::super::*;

pub(crate) fn selected_edge_line(model: &StudioViewModel) -> String {
    let Some(edge) = model.selected_edge.as_ref() else {
        return "none".to_string();
    };
    let is_issue_edge = model
        .focused_issue
        .as_ref()
        .and_then(|focus| focus.edge_id.as_deref())
        == Some(edge.edge_id.as_str());
    let prefix = if is_issue_edge { "issue: " } else { "" };
    format!("{prefix}{} [{}]", edge.edge_id, edge.kind)
}

pub(crate) fn selected_edge_detail_lines(model: &StudioViewModel) -> String {
    let Some(edge) = model.selected_edge.as_ref() else {
        if let Some(issue_code) = model.edge_selection_code.as_deref() {
            return format!("none [{issue_code}]");
        }
        return "none".to_string();
    };
    let mut lines = Vec::new();
    if let Some(issue_code) = model.edge_selection_code.as_deref() {
        lines.push(format!("selection: {issue_code}"));
    }
    lines.push(format!("graph: {}", edge.graph_id));
    lines.push(format!("status: {}", edge.endpoint_status));
    if edge.validation_issue_count > 0 {
        lines.push(format!("issues: {}", edge.validation_issue_count));
    }
    if let Some(binding_kind) = edge.binding_kind.as_deref() {
        lines.push(format!("binding: {binding_kind}"));
    }
    lines.push(format!(
        "source: {} / {} / {}",
        edge.source_node_id,
        edge.source_kind.as_deref().unwrap_or("missing"),
        edge.source_reference_id.as_deref().unwrap_or("missing")
    ));
    lines.push(format!(
        "target: {} / {} / {}",
        edge.target_node_id,
        edge.target_kind.as_deref().unwrap_or("missing"),
        edge.target_reference_id.as_deref().unwrap_or("missing")
    ));
    lines.join("\n")
}

pub(crate) fn next_edge_id(model: &StudioViewModel) -> Option<&str> {
    let selected_graph_id = model.selected_graph_id.as_deref()?;
    let graph = model
        .graphs
        .iter()
        .find(|graph| graph.graph_id == selected_graph_id)?;
    if graph.edge_rows.is_empty() {
        return None;
    }
    let current_index = model
        .selected_edge_id
        .as_deref()
        .and_then(|edge_id| {
            graph
                .edge_rows
                .iter()
                .position(|edge| edge.edge_id == edge_id)
        })
        .unwrap_or(0);
    let next_index = (current_index + 1) % graph.edge_rows.len();
    graph
        .edge_rows
        .get(next_index)
        .map(|edge| edge.edge_id.as_str())
}
