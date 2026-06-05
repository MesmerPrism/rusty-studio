use super::super::super::*;

impl App {
    pub(in crate::app_events) fn select_canvas_node(&mut self, cx: &mut Cx, node_id: &str) {
        let current_edge_id = self
            .model
            .as_ref()
            .and_then(|model| model.selected_edge_id.clone());
        self.select_canvas_request(cx, Some(node_id), current_edge_id.as_deref());
    }

    pub(in crate::app_events) fn select_canvas_edge(&mut self, cx: &mut Cx, edge_id: &str) {
        let current_node_id = self
            .model
            .as_ref()
            .and_then(|model| model.selected_node_id.clone());
        self.select_canvas_request(cx, current_node_id.as_deref(), Some(edge_id));
    }

    fn select_canvas_request(
        &mut self,
        cx: &mut Cx,
        requested_node_id: Option<&str>,
        requested_edge_id: Option<&str>,
    ) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        match canvas_selection_view_model_for_project_source(
            &source,
            &model,
            self.selected_graph_index,
            requested_node_id,
            requested_edge_id,
        ) {
            Ok(model) => {
                self.selected_graph_index = model
                    .selected_graph_index
                    .unwrap_or(self.selected_graph_index);
                self.selected_issue_check_id = model.selected_issue_check_id.clone();
                self.selected_node_id = model.selected_node_id.clone();
                self.selected_edge_id = model.selected_edge_id.clone();
                self.model = Some(model);
                self.sync_loaded_model(cx);
                self.ui.redraw(cx);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }
}
