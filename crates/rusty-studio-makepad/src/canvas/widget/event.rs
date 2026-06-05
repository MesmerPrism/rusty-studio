use makepad_widgets::*;

use super::super::model::{StudioGraphCanvasAction, StudioGraphCanvasHit};
use super::StudioGraphCanvas;

impl StudioGraphCanvas {
    pub(super) fn handle_canvas_event(&mut self, cx: &mut Cx, event: &Event) {
        if !self.area.is_valid(cx) {
            return;
        }
        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe) => {
                if self.hit_test_abs(cx, fe.abs).is_some() {
                    cx.set_cursor(MouseCursor::Hand);
                }
            }
            Hit::FingerDown(fe) if fe.is_primary_hit() => {
                if self.hit_test_abs(cx, fe.abs).is_some() {
                    cx.set_cursor(MouseCursor::Hand);
                }
            }
            Hit::FingerUp(fe) if fe.is_primary_hit() && fe.is_over => {
                if let Some(hit) = self.hit_test_abs(cx, fe.abs) {
                    let action = match hit {
                        StudioGraphCanvasHit::Node(node_id) => {
                            StudioGraphCanvasAction::SelectNode(node_id)
                        }
                        StudioGraphCanvasHit::Edge(edge_id) => {
                            StudioGraphCanvasAction::SelectEdge(edge_id)
                        }
                    };
                    cx.widget_action(self.widget_uid(), action);
                }
            }
            _ => {}
        }
    }

    fn hit_test_abs(&self, cx: &Cx, abs: DVec2) -> Option<StudioGraphCanvasHit> {
        self.model.hit_test_abs(self.area.rect(cx), abs)
    }
}
