use makepad_widgets::*;

use super::model::StudioGraphCanvasNode;

#[derive(Clone, Copy)]
pub(crate) struct CanvasViewportBounds {
    pub(crate) min_x: f64,
    pub(crate) min_y: f64,
    pub(crate) width: f64,
    pub(crate) height: f64,
}

pub(crate) struct CanvasViewport {
    origin_x: f64,
    origin_y: f64,
    scale: f64,
    bounds: CanvasViewportBounds,
}

impl CanvasViewport {
    pub(crate) fn for_rect(rect: Rect, bounds: CanvasViewportBounds) -> Self {
        let margin = 18.0_f64;
        let content_width = (rect.size.x - margin * 2.0).max(1.0);
        let content_height = (rect.size.y - margin * 2.0).max(1.0);
        let scale = (content_width / bounds.width)
            .min(content_height / bounds.height)
            .max(0.1);
        let drawn_width = bounds.width * scale;
        let drawn_height = bounds.height * scale;
        Self {
            origin_x: rect.pos.x + (rect.size.x - drawn_width) * 0.5,
            origin_y: rect.pos.y + (rect.size.y - drawn_height) * 0.5,
            scale,
            bounds,
        }
    }

    pub(crate) fn node_rect(&self, node: &StudioGraphCanvasNode) -> Rect {
        Rect {
            pos: dvec2(
                self.origin_x + (node.x as f64 - self.bounds.min_x) * self.scale,
                self.origin_y + (node.y as f64 - self.bounds.min_y) * self.scale,
            ),
            size: dvec2(
                (node.width as f64 * self.scale).max(42.0),
                (node.height as f64 * self.scale).max(32.0),
            ),
        }
    }

    pub(crate) fn node_center(&self, node: &StudioGraphCanvasNode) -> DVec2 {
        let rect = self.node_rect(node);
        dvec2(
            rect.pos.x + rect.size.x * 0.5,
            rect.pos.y + rect.size.y * 0.5,
        )
    }
}
