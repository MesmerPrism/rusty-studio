use super::super::super::*;

impl App {
    pub(in crate::app_events) fn review_shell_handoff_acceptance(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_handoff_acceptance_for_project_source(&source) {
            Ok((report, bundle_root)) => {
                self.last_shell_bundle_status =
                    shell_handoff_acceptance_status(&report, &bundle_root);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }
}
