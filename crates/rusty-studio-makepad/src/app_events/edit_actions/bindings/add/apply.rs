use super::super::super::super::*;

impl App {
    pub(in crate::app_events) fn add_binding_to_selected_graph(
        &mut self,
        cx: &mut Cx,
        binding_kind: StudioBindingKind,
        source_node_id: &str,
        target_node_id: &str,
    ) {
        let Some(source) = self.project_source.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No project source is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No view model is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        match add_binding_to_project_source(
            &source,
            &model,
            self.selected_graph_index,
            binding_kind,
            source_node_id,
            target_node_id,
        ) {
            Ok((report, refreshed_model)) => {
                self.last_edit_report = Some(report);
                self.last_edit_save_issue.clear();
                if let Some(refreshed_model) = refreshed_model {
                    self.selected_graph_index = refreshed_model
                        .selected_graph_index
                        .unwrap_or(self.selected_graph_index);
                    self.model = Some(refreshed_model);
                }
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }
}
