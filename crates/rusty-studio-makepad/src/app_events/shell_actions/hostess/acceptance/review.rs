use super::super::super::super::*;

impl App {
    pub(in crate::app_events) fn review_shell_hostess_staging_acceptance(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_hostess_staging_acceptance_for_project_source(&source) {
            Ok((report, output_path)) => {
                self.last_shell_bundle_status =
                    shell_hostess_staging_acceptance_status(&report, &output_path);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }
}
