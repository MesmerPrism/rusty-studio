use super::super::super::*;

impl App {
    pub(in crate::app_events::sync) fn sync_no_graph(&mut self, cx: &mut Cx) {
        self.ui.label(cx, ids!(graph_selection)).set_text(cx, "");
        self.ui
            .label(cx, ids!(graph_identity))
            .set_text(cx, "no graph loaded");
        self.ui.label(cx, ids!(graph_target)).set_text(cx, "");
        self.ui.label(cx, ids!(graph_counts)).set_text(cx, "");
        self.ui.label(cx, ids!(shell_preview)).set_text(cx, "");
        self.ui.label(cx, ids!(shell_routes)).set_text(cx, "");
        self.ui.label(cx, ids!(shell_template)).set_text(cx, "");
        self.ui
            .label(cx, ids!(shell_bundle_status))
            .set_text(cx, "");
        self.ui.label(cx, ids!(graph_layout)).set_text(cx, "");
        if let Some(mut canvas) = self
            .ui
            .widget(cx, ids!(graph_canvas))
            .borrow_mut::<StudioGraphCanvas>()
        {
            canvas.set_canvas_model(cx, StudioGraphCanvasModel::default());
        }
        self.ui.label(cx, ids!(graph_nodes)).set_text(cx, "");
        self.ui.label(cx, ids!(graph_edges)).set_text(cx, "");
        self.ui.label(cx, ids!(selected_node)).set_text(cx, "");
        self.ui.label(cx, ids!(selected_reference)).set_text(cx, "");
        self.ui
            .label(cx, ids!(selected_node_details))
            .set_text(cx, "");
        self.ui.label(cx, ids!(selected_edge)).set_text(cx, "");
        self.ui
            .label(cx, ids!(selected_edge_details))
            .set_text(cx, "");
        self.ui.label(cx, ids!(focused_issue)).set_text(cx, "");
    }
}
