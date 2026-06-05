use super::super::*;

impl App {
    pub(in crate::app_events) fn select_next_issue(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        let Some(next_issue_check_id) = next_issue_check_id(&model).map(str::to_string) else {
            return;
        };
        let requested_graph_id = model
            .validation_issues
            .iter()
            .find(|issue| issue.check_id == next_issue_check_id)
            .and_then(|issue| issue.graph_id.as_deref())
            .or(model.selected_graph_id.as_deref());
        match load_studio_view_model_for_path(
            &source,
            requested_graph_id,
            Some(&next_issue_check_id),
            None,
            None,
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
