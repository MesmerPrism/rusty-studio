use super::super::super::*;

impl App {
    pub(in crate::app_events) fn export_shell_bundle_for_selected_graph(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_shell_bundle_status = "No view model is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match export_shell_bundle_for_project_source(&source, &model, self.selected_graph_index) {
            Ok((report, output_dir)) => {
                self.last_shell_bundle_status = shell_bundle_export_status(&report, &output_dir);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }
}
