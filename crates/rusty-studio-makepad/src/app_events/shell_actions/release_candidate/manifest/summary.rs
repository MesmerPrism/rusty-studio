use super::super::super::super::*;

impl App {
    pub(in crate::app_events) fn inspect_shell_release_candidate_manifest(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_release_candidate_review_manifest_summary_for_project_source(&source) {
            Ok((candidate, index, candidate_path, index_path)) => {
                self.last_shell_bundle_status =
                    shell_release_candidate_review_manifest_summary_status(
                        &candidate,
                        &index,
                        &candidate_path,
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
