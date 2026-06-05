use super::super::super::*;

impl App {
    pub(in crate::app_events::sync) fn sync_inspector(
        &mut self,
        cx: &mut Cx,
        model: &StudioViewModel,
        _graph: &StudioGraphView,
    ) {
        self.ui
            .label(cx, ids!(focused_issue))
            .set_text(cx, &issue_focus_line(model));
        self.ui
            .label(cx, ids!(selected_node))
            .set_text(cx, &selected_node_line(model));
        self.ui
            .label(cx, ids!(selected_reference))
            .set_text(cx, &selected_reference_line(model));
        self.ui
            .label(cx, ids!(selected_node_details))
            .set_text(cx, &selected_node_detail_lines(model));
        self.ui
            .label(cx, ids!(selected_edge))
            .set_text(cx, &selected_edge_line(model));
        self.ui
            .label(cx, ids!(selected_edge_details))
            .set_text(cx, &selected_edge_detail_lines(model));
    }
}
