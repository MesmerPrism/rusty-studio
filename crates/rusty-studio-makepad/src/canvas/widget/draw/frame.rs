use makepad_widgets::*;

use super::super::super::viewport::{CanvasViewport, CanvasViewportBounds};
use super::super::StudioGraphCanvas;

impl StudioGraphCanvas {
    pub(in crate::canvas::widget) fn draw_canvas_walk(
        &mut self,
        cx: &mut Cx2d,
        walk: Walk,
    ) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        self.draw_bg.color = self.bg_color;
        self.draw_bg.draw_abs(cx, rect);
        if rect.size.x <= 2.0 || rect.size.y <= 2.0 {
            return DrawStep::done();
        }

        let Some(bounds) = self.logical_bounds() else {
            self.draw_text.color = self.text_color;
            self.draw_text
                .draw_abs(cx, dvec2(rect.pos.x + 18.0, rect.pos.y + 18.0), "no layout");
            return DrawStep::done();
        };

        let viewport = self.viewport_for_rect(rect, bounds);
        self.draw_edges(cx, &viewport);
        self.draw_nodes(cx, &viewport);
        DrawStep::done()
    }

    fn logical_bounds(&self) -> Option<CanvasViewportBounds> {
        self.model.logical_bounds()
    }

    fn viewport_for_rect(&self, rect: Rect, bounds: CanvasViewportBounds) -> CanvasViewport {
        CanvasViewport::for_rect(rect, bounds)
    }
}
