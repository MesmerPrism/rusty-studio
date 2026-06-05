use super::super::super::super::*;

impl App {
    pub(in crate::app_events) fn write_shell_export_package_baseline(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match write_shell_export_package_baseline_for_project_source(&source) {
            Ok((report, baseline, index, package_path, baseline_path, index_path, bundle_root)) => {
                self.last_shell_bundle_status = shell_export_package_baseline_status(
                    &report,
                    &baseline,
                    &index,
                    &package_path,
                    &baseline_path,
                    &index_path,
                    &bundle_root,
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
