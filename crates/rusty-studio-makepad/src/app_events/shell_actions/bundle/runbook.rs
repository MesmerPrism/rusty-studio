use super::super::super::*;

impl App {
    pub(in crate::app_events) fn inspect_shell_runbook(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_runbook_for_project_source(&source) {
            Ok((report, bundle_root)) => {
                self.last_shell_bundle_status = shell_runbook_status(&report, &bundle_root);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }
}
