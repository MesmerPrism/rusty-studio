mod draw;
mod event;

use makepad_widgets::*;

use super::model::StudioGraphCanvasModel;

#[derive(Script, ScriptHook, Widget)]
pub struct StudioGraphCanvas {
    #[uid]
    uid: WidgetUid,
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_bg: DrawColor,
    #[live]
    draw_edge: DrawVector,
    #[live]
    draw_node: DrawColor,
    #[live]
    draw_text: DrawText,
    #[live]
    bg_color: Vec4f,
    #[live]
    node_color: Vec4f,
    #[live]
    node_selected_color: Vec4f,
    #[live]
    node_issue_color: Vec4f,
    #[live]
    edge_color: Vec4f,
    #[live]
    edge_selected_color: Vec4f,
    #[live]
    edge_issue_color: Vec4f,
    #[live]
    border_color: Vec4f,
    #[live]
    selected_border_color: Vec4f,
    #[live]
    text_color: Vec4f,
    #[live]
    issue_text_color: Vec4f,
    #[rust]
    area: Area,
    #[rust]
    model: StudioGraphCanvasModel,
}

impl Widget for StudioGraphCanvas {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.handle_canvas_event(cx, event);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_canvas_walk(cx, walk)
    }
}

impl StudioGraphCanvas {
    pub(crate) fn set_canvas_model(&mut self, cx: &mut Cx, model: StudioGraphCanvasModel) {
        self.model = model;
        self.redraw(cx);
    }
}
