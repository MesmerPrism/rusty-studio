use super::super::super::*;

pub(crate) fn next_node_id(model: &StudioViewModel) -> Option<&str> {
    let selected_graph_id = model.selected_graph_id.as_deref()?;
    let graph = model
        .graphs
        .iter()
        .find(|graph| graph.graph_id == selected_graph_id)?;
    if graph.node_rows.is_empty() {
        return None;
    }
    let current_index = model
        .selected_node_id
        .as_deref()
        .and_then(|node_id| {
            graph
                .node_rows
                .iter()
                .position(|node| node.node_id == node_id)
        })
        .unwrap_or(0);
    let next_index = (current_index + 1) % graph.node_rows.len();
    graph
        .node_rows
        .get(next_index)
        .map(|node| node.node_id.as_str())
}
