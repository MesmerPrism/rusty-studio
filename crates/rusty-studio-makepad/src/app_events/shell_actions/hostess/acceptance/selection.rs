use super::super::super::super::*;

impl App {
    pub(in crate::app_events) fn select_next_shell_hostess_staging_acceptance_default(
        &mut self,
        cx: &mut Cx,
    ) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match select_next_shell_hostess_staging_acceptance_default_for_project_source(&source) {
            Ok((acceptance, index, acceptance_path, index_path)) => {
                self.last_shell_bundle_status = shell_hostess_staging_acceptance_select_status(
                    &acceptance,
                    &index,
                    &acceptance_path,
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

    pub(in crate::app_events) fn promote_shell_hostess_staging_acceptance_default(
        &mut self,
        cx: &mut Cx,
    ) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match promote_shell_hostess_staging_acceptance_default_for_project_source(&source) {
            Ok((acceptance, index, acceptance_path, index_path)) => {
                self.last_shell_bundle_status = shell_hostess_staging_acceptance_promote_status(
                    &acceptance,
                    &index,
                    &acceptance_path,
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
