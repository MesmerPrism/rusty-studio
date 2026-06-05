use makepad_widgets::*;

use super::super::super::viewport::CanvasViewport;
use super::super::StudioGraphCanvas;

impl StudioGraphCanvas {
    pub(in crate::canvas::widget::draw) fn draw_edges(
        &mut self,
        cx: &mut Cx2d,
        viewport: &CanvasViewport,
    ) {
        self.draw_edge.begin();
        for edge in &self.model.edges {
            let Some(source) = self
                .model
                .nodes
                .iter()
                .find(|node| node.node_id == edge.source_node_id)
            else {
                continue;
            };
            let Some(target) = self
                .model
                .nodes
                .iter()
                .find(|node| node.node_id == edge.target_node_id)
            else {
                continue;
            };
            let source_center = viewport.node_center(source);
            let target_center = viewport.node_center(target);
            let color = if edge.selected {
                self.edge_selected_color
            } else if edge.validation_issue_count > 0 {
                self.edge_issue_color
            } else {
                self.edge_color
            };
            self.draw_edge.set_color(color.x, color.y, color.z, color.w);
            self.draw_edge
                .move_to(source_center.x as f32, source_center.y as f32);
            if edge.route == "orthogonal" {
                let mid_x = (source_center.x + target_center.x) * 0.5;
                self.draw_edge.line_to(mid_x as f32, source_center.y as f32);
                self.draw_edge.line_to(mid_x as f32, target_center.y as f32);
            }
            self.draw_edge
                .line_to(target_center.x as f32, target_center.y as f32);
            self.draw_edge.stroke(if edge.selected { 3.0 } else { 1.6 });
        }
        self.draw_edge.end(cx);
    }
}
