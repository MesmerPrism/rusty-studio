use super::super::super::*;

impl App {
    pub(in crate::app_events) fn select_next_node(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        let Some(next_node_id) = next_node_id(&model).map(str::to_string) else {
            return;
        };
        match load_studio_view_model_for_path(
            &source,
            model.selected_graph_id.as_deref(),
            model.selected_issue_check_id.as_deref(),
            Some(&next_node_id),
            model.selected_edge_id.as_deref(),
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

    pub(in crate::app_events) fn select_next_edge(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        let Some(next_edge_id) = next_edge_id(&model).map(str::to_string) else {
            return;
        };
        match load_studio_view_model_for_path(
            &source,
            model.selected_graph_id.as_deref(),
            model.selected_issue_check_id.as_deref(),
            model.selected_node_id.as_deref(),
            Some(&next_edge_id),
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
