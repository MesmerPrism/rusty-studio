use super::super::super::super::*;

impl App {
    pub(in crate::app_events) fn inspect_shell_export_package_baseline(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_export_package_baseline_summary_for_project_source(&source) {
            Ok((baseline, index, baseline_path, index_path)) => {
                self.last_shell_bundle_status = shell_export_package_baseline_summary_status(
                    &baseline,
                    &index,
                    &baseline_path,
                    &index_path,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }
}
