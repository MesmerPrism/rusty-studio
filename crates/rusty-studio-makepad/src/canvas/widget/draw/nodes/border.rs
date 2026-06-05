use makepad_widgets::*;

use super::super::super::StudioGraphCanvas;

impl StudioGraphCanvas {
    pub(super) fn draw_node_border(&mut self, cx: &mut Cx2d, rect: Rect, color: Vec4f, width: f64) {
        self.draw_node.color = color;
        self.draw_node.draw_abs(
            cx,
            Rect {
                pos: rect.pos,
                size: dvec2(rect.size.x, width),
            },
        );
        self.draw_node.draw_abs(
            cx,
            Rect {
                pos: dvec2(rect.pos.x, rect.pos.y + rect.size.y - width),
                size: dvec2(rect.size.x, width),
            },
        );
        self.draw_node.draw_abs(
            cx,
            Rect {
                pos: rect.pos,
                size: dvec2(width, rect.size.y),
            },
        );
        self.draw_node.draw_abs(
            cx,
            Rect {
                pos: dvec2(rect.pos.x + rect.size.x - width, rect.pos.y),
                size: dvec2(width, rect.size.y),
            },
        );
    }
}
