use rusty_studio_model::{StudioGraphView, StudioViewModel};

use super::super::model::{StudioGraphCanvasEdge, StudioGraphCanvasModel, StudioGraphCanvasNode};

pub(super) fn layout_canvas_model(
    model: &StudioViewModel,
    graph: &StudioGraphView,
) -> Option<StudioGraphCanvasModel> {
    let layout = graph.layout.as_ref()?;
    Some(StudioGraphCanvasModel {
        layout_id: layout.layout_id.clone(),
        coordinate_space: layout.coordinate_space.clone(),
        nodes: layout
            .nodes
            .iter()
            .map(|layout_node| {
                let row = graph
                    .node_rows
                    .iter()
                    .find(|node| node.node_id == layout_node.node_id);
                StudioGraphCanvasNode {
                    node_id: layout_node.node_id.clone(),
                    label: row
                        .map(|node| node.label.clone())
                        .unwrap_or_else(|| layout_node.node_id.clone()),
                    kind: row
                        .map(|node| node.kind.clone())
                        .unwrap_or_else(|| "missing".to_string()),
                    x: layout_node.x,
                    y: layout_node.y,
                    width: layout_node.width,
                    height: layout_node.height,
                    validation_issue_count: layout_node.validation_issue_count,
                    selected: model.selected_node_id.as_deref()
                        == Some(layout_node.node_id.as_str()),
                }
            })
            .collect(),
        edges: layout
            .edges
            .iter()
            .map(|layout_edge| {
                let row = graph
                    .edge_rows
                    .iter()
                    .find(|edge| edge.edge_id == layout_edge.edge_id);
                StudioGraphCanvasEdge {
                    edge_id: layout_edge.edge_id.clone(),
                    source_node_id: row
                        .map(|edge| edge.source_node_id.clone())
                        .unwrap_or_default(),
                    target_node_id: row
                        .map(|edge| edge.target_node_id.clone())
                        .unwrap_or_default(),
                    route: layout_edge.route.clone(),
                    validation_issue_count: layout_edge.validation_issue_count,
                    selected: model.selected_edge_id.as_deref()
                        == Some(layout_edge.edge_id.as_str()),
                }
            })
            .collect(),
    })
}
