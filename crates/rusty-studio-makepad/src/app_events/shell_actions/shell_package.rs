use super::super::*;

impl App {
    pub(in crate::app_events) fn review_shell_export_package(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_export_package_for_project_source(&source) {
            Ok((report, bundle_root)) => {
                self.last_shell_bundle_status = shell_export_package_status(&report, &bundle_root);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

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

    pub(in crate::app_events) fn append_shell_export_package_baseline(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match append_shell_export_package_baseline_for_project_source(&source) {
            Ok((report, baseline, index, package_path, baseline_path, index_path, bundle_root)) => {
                self.last_shell_bundle_status = shell_export_package_baseline_append_status(
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

    pub(in crate::app_events) fn inspect_shell_export_package_baseline(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_export_package_baseline_summary_for_project_source(&source) {
            Ok((baseline, index, baseline_path, index_path)) => {
                self.last_shell_bundle_status = shell_export_package_baseline_summary_status(
                    &baseline,
                    &index,
                    &baseline_path,
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

    pub(in crate::app_events) fn promote_shell_export_package_baseline_default(
        &mut self,
        cx: &mut Cx,
    ) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match promote_shell_export_package_baseline_default_for_project_source(&source) {
            Ok((baseline, index, baseline_path, index_path)) => {
                self.last_shell_bundle_status = shell_export_package_baseline_promote_status(
                    &baseline,
                    &index,
                    &baseline_path,
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

    pub(in crate::app_events) fn select_next_shell_export_package_baseline_default(
        &mut self,
        cx: &mut Cx,
    ) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match select_next_shell_export_package_baseline_default_for_project_source(&source) {
            Ok((baseline, index, baseline_path, index_path)) => {
                self.last_shell_bundle_status = shell_export_package_baseline_select_status(
                    &baseline,
                    &index,
                    &baseline_path,
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
