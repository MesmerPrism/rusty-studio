use super::super::super::super::*;

impl App {
    pub(in crate::app_events) fn add_command_binding_to_selected_module(&mut self, cx: &mut Cx) {
        let Some(model) = self.model.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No view model is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        match selected_command_binding_request(&model) {
            Ok(request) => {
                self.add_binding_to_selected_graph(
                    cx,
                    request.binding_kind,
                    &request.source_node_id,
                    &request.target_node_id,
                );
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
