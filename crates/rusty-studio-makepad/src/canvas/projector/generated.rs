use rusty_studio_model::{StudioGraphView, StudioViewModel};

use super::super::model::{StudioGraphCanvasEdge, StudioGraphCanvasModel, StudioGraphCanvasNode};

pub(super) fn generated_canvas_model(
    model: &StudioViewModel,
    graph: &StudioGraphView,
) -> StudioGraphCanvasModel {
    let nodes = graph
        .node_rows
        .iter()
        .enumerate()
        .map(|(index, node)| StudioGraphCanvasNode {
            node_id: node.node_id.clone(),
            label: node.label.clone(),
            kind: node.kind.clone(),
            x: 40 + ((index % 3) as i32 * 260),
            y: 40 + ((index / 3) as i32 * 150),
            width: 220,
            height: 72,
            validation_issue_count: node.validation_issue_count,
            selected: model.selected_node_id.as_deref() == Some(node.node_id.as_str()),
        })
        .collect();
    let edges = graph
        .edge_rows
        .iter()
        .map(|edge| StudioGraphCanvasEdge {
            edge_id: edge.edge_id.clone(),
            source_node_id: edge.source_node_id.clone(),
            target_node_id: edge.target_node_id.clone(),
            route: "direct".to_string(),
            validation_issue_count: edge.validation_issue_count,
            selected: model.selected_edge_id.as_deref() == Some(edge.edge_id.as_str()),
        })
        .collect();
    StudioGraphCanvasModel {
        layout_id: "studio.layout.generated_readonly".to_string(),
        coordinate_space: "studio.canvas.generated_2d".to_string(),
        nodes,
        edges,
    }
}
