use super::super::super::*;

impl App {
    pub(in crate::app_events) fn compare_shell_export_package(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_export_package_comparison_for_project_source(&source) {
            Ok((report, baseline_path, bundle_root)) => {
                self.last_shell_bundle_status =
                    shell_export_package_comparison_status(&report, &baseline_path, &bundle_root);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }
}
