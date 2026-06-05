mod border;
mod text;

use makepad_widgets::*;

use super::super::super::viewport::CanvasViewport;
use super::super::StudioGraphCanvas;

impl StudioGraphCanvas {
    pub(in crate::canvas::widget::draw) fn draw_nodes(
        &mut self,
        cx: &mut Cx2d,
        viewport: &CanvasViewport,
    ) {
        let nodes = self.model.nodes.clone();
        for node in &nodes {
            let rect = viewport.node_rect(node);
            let fill = if node.selected {
                self.node_selected_color
            } else if node.validation_issue_count > 0 {
                self.node_issue_color
            } else {
                self.node_color
            };
            self.draw_node.color = fill;
            self.draw_node.draw_abs(cx, rect);
            self.draw_node_border(
                cx,
                rect,
                if node.selected {
                    self.selected_border_color
                } else {
                    self.border_color
                },
                if node.selected { 2.0 } else { 1.0 },
            );
            self.draw_node_text(cx, rect, node);
        }
    }
}
