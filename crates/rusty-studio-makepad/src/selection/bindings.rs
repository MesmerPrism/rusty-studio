use rusty_studio_model::{StudioBindingKind, StudioViewModel};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SelectedBindingRequest {
    pub(crate) binding_kind: StudioBindingKind,
    pub(crate) source_node_id: String,
    pub(crate) target_node_id: String,
}

pub(crate) fn selected_command_binding_request(
    model: &StudioViewModel,
) -> Result<SelectedBindingRequest, String> {
    let Some(node) = model.selected_node.as_ref() else {
        return Err("No node is selected".to_string());
    };
    if node.kind != "module" {
        return Err(format!(
            "Selected node {} is {}; select a module node to add a command binding",
            node.node_id, node.kind
        ));
    }
    let graph = model
        .graphs
        .iter()
        .find(|graph| graph.graph_id == node.graph_id)
        .ok_or_else(|| format!("Selected graph {} is unavailable", node.graph_id))?;
    let operator_shell_nodes = graph
        .node_rows
        .iter()
        .filter(|row| row.kind == "operator_shell")
        .collect::<Vec<_>>();
    let Some(source_node) = operator_shell_nodes.first() else {
        return Err(format!(
            "Selected graph {} has no operator shell for command binding",
            graph.graph_id
        ));
    };
    if operator_shell_nodes.len() > 1 {
        return Err(format!(
            "Selected graph {} has multiple operator shells; select one shell before adding a command binding",
            graph.graph_id
        ));
    }
    Ok(SelectedBindingRequest {
        binding_kind: StudioBindingKind::Command,
        source_node_id: source_node.node_id.clone(),
        target_node_id: node.node_id.clone(),
    })
}

pub(crate) fn selected_binding_request(
    model: &StudioViewModel,
) -> Result<SelectedBindingRequest, String> {
    let Some(edge) = model.selected_edge.as_ref() else {
        return Err("No edge is selected".to_string());
    };
    let Some(binding_kind) = edge
        .binding_kind
        .as_deref()
        .and_then(studio_binding_kind_from_view)
    else {
        return Err(format!(
            "Selected edge {} is {}; select a stream or command binding to remove a binding",
            edge.edge_id, edge.kind
        ));
    };
    Ok(SelectedBindingRequest {
        binding_kind,
        source_node_id: edge.source_node_id.clone(),
        target_node_id: edge.target_node_id.clone(),
    })
}

fn studio_binding_kind_from_view(value: &str) -> Option<StudioBindingKind> {
    match value {
        "stream" => Some(StudioBindingKind::Stream),
        "command" => Some(StudioBindingKind::Command),
        _ => None,
    }
}
