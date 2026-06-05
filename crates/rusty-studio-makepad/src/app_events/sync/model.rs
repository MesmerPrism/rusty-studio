use super::super::*;

impl App {
    pub(in crate::app_events) fn sync_project(&mut self, cx: &mut Cx) {
        match load_studio_view_model(
            initial_graph_id_from_args().as_deref(),
            initial_issue_check_id_from_args().as_deref(),
            initial_node_id_from_args().as_deref(),
            initial_edge_id_from_args().as_deref(),
        ) {
            Ok((source, model)) => self.set_model(cx, source, model),
            Err(error) => self.sync_error(cx, &error),
        }
    }

    fn set_model(&mut self, cx: &mut Cx, source: PathBuf, model: StudioViewModel) {
        self.selected_graph_index = model.selected_graph_index.unwrap_or(0);
        self.selected_issue_check_id = model.selected_issue_check_id.clone();
        self.selected_node_id = model.selected_node_id.clone();
        self.selected_edge_id = model.selected_edge_id.clone();
        self.project_source = Some(source);
        self.model = Some(model);
        self.sync_loaded_model(cx);
    }

    pub(in crate::app_events) fn sync_loaded_model(&mut self, cx: &mut Cx) {
        let Some(model) = self.model.clone() else {
            self.sync_error(cx, "no Studio view model loaded");
            return;
        };
        let source = self.project_source.clone().unwrap_or_default();
        self.ui
            .label(cx, ids!(project_source))
            .set_text(cx, &source.display().to_string());
        self.ui.label(cx, ids!(project_identity)).set_text(
            cx,
            &format!("{} ({})", model.display_name, model.project_id),
        );
        self.ui.label(cx, ids!(project_revision)).set_text(
            cx,
            &format!("rev {} / {} graph(s)", model.revision, model.graph_count),
        );
        self.ui
            .label(cx, ids!(validation_status))
            .set_text(cx, &validation_line(&model));
        self.ui
            .label(cx, ids!(validation_issues))
            .set_text(cx, &validation_issue_lines(&model));
        self.ui
            .label(cx, ids!(catalog_packages))
            .set_text(cx, &catalog_package_lines(&model));
        self.ui
            .label(cx, ids!(host_profiles))
            .set_text(cx, &host_profile_lines(&model));

        if let Some(issue_code) = &model.selection_issue_code {
            self.sync_no_graph(cx);
            self.ui.label(cx, ids!(graph_selection)).set_text(
                cx,
                &format!(
                    "requested graph {:?} is unavailable ({issue_code})",
                    model.requested_graph_id
                ),
            );
        } else if let Some(graph) = model.graphs.get(self.selected_graph_index) {
            self.sync_graph(cx, &model, graph);
        } else {
            self.sync_no_graph(cx);
        }
        self.ui.label(cx, ids!(authority_note)).set_text(
            cx,
            "Makepad renders the view model; rusty-studio-core owns validation and resolution.",
        );
        self.sync_edit_report(cx);
    }
}
