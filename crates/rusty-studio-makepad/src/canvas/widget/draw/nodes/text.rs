use makepad_widgets::*;

use super::super::super::super::model::StudioGraphCanvasNode;
use super::super::super::StudioGraphCanvas;

impl StudioGraphCanvas {
    pub(super) fn draw_node_text(
        &mut self,
        cx: &mut Cx2d,
        rect: Rect,
        node: &StudioGraphCanvasNode,
    ) {
        self.draw_text.color = if node.validation_issue_count > 0 {
            self.issue_text_color
        } else {
            self.text_color
        };
        self.draw_text.draw_abs(
            cx,
            dvec2(rect.pos.x + 8.0, rect.pos.y + 8.0),
            &short_canvas_label(&node.label, 26),
        );
        self.draw_text.draw_abs(
            cx,
            dvec2(rect.pos.x + 8.0, rect.pos.y + 24.0),
            &format!("{}  {}", node.kind, node.node_id),
        );
        if node.validation_issue_count > 0 {
            self.draw_text.draw_abs(
                cx,
                dvec2(rect.pos.x + 8.0, rect.pos.y + 40.0),
                &format!("issues: {}", node.validation_issue_count),
            );
        }
    }
}

fn short_canvas_label(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    let mut truncated = value
        .chars()
        .take(max_chars.saturating_sub(3))
        .collect::<String>();
    truncated.push_str("...");
    truncated
}
