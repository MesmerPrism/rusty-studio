use super::super::*;

impl App {
    pub(in crate::app_events) fn export_shell_bundle_for_selected_graph(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_shell_bundle_status = "No view model is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match export_shell_bundle_for_project_source(&source, &model, self.selected_graph_index) {
            Ok((report, output_dir)) => {
                self.last_shell_bundle_status = shell_bundle_export_status(&report, &output_dir);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    pub(in crate::app_events) fn verify_shell_bundle_for_selected_graph(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_shell_bundle_status = "No view model is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match validate_shell_bundle_for_project_source(&source, &model, self.selected_graph_index) {
            Ok((report, output_dir)) => {
                self.last_shell_bundle_status =
                    shell_bundle_validation_status(&report, &output_dir);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    pub(in crate::app_events) fn prepare_shell_handoff_for_selected_graph(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_shell_bundle_status = "No view model is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_handoff_for_project_source(&source, &model, self.selected_graph_index) {
            Ok((report, output_dir)) => {
                self.last_shell_bundle_status = shell_handoff_status(&report, &output_dir);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    pub(in crate::app_events) fn inspect_shell_handoff_readiness(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_handoff_readiness_for_project_source(&source) {
            Ok((report, bundle_root)) => {
                self.last_shell_bundle_status =
                    shell_handoff_readiness_status(&report, &bundle_root);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

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
