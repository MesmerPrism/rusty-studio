use super::super::super::*;

impl App {
    pub(in crate::app_events) fn write_shell_handoff_manifest(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match write_shell_handoff_manifest_for_project_source(&source) {
            Ok((manifest, output_path)) => {
                self.last_shell_bundle_status =
                    shell_handoff_manifest_status(&manifest, &output_path);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }
}
