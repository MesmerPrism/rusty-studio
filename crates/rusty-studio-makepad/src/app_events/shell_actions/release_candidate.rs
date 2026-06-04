use super::super::*;

impl App {
    pub(in crate::app_events) fn review_shell_release_candidate(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_release_candidate_review_for_project_source(&source) {
            Ok((report, output_path)) => {
                self.last_shell_bundle_status =
                    shell_release_candidate_review_status(&report, &output_path);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    pub(in crate::app_events) fn write_shell_release_candidate_manifest(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match write_shell_release_candidate_review_manifest_for_project_source(&source) {
            Ok((review, candidate, index, review_path, candidate_path, index_path)) => {
                self.last_shell_bundle_status = shell_release_candidate_review_manifest_status(
                    &review,
                    &candidate,
                    &index,
                    &review_path,
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

    pub(in crate::app_events) fn append_shell_release_candidate_manifest(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match append_shell_release_candidate_review_manifest_for_project_source(&source) {
            Ok((review, candidate, index, review_path, candidate_path, index_path)) => {
                self.last_shell_bundle_status =
                    shell_release_candidate_review_manifest_append_status(
                        &review,
                        &candidate,
                        &index,
                        &review_path,
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

    pub(in crate::app_events) fn select_next_shell_release_candidate_default(
        &mut self,
        cx: &mut Cx,
    ) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match select_next_shell_release_candidate_default_for_project_source(&source) {
            Ok((candidate, index, candidate_path, index_path)) => {
                self.last_shell_bundle_status =
                    shell_release_candidate_review_manifest_select_status(
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

    pub(in crate::app_events) fn promote_shell_release_candidate_default(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match promote_shell_release_candidate_default_for_project_source(&source) {
            Ok((candidate, index, candidate_path, index_path)) => {
                self.last_shell_bundle_status =
                    shell_release_candidate_review_manifest_promote_status(
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
