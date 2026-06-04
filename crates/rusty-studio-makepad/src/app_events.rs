use super::*;

mod edit_actions;
mod selection_actions;
mod shell_actions;
impl App {
    fn sync_project(&mut self, cx: &mut Cx) {
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

    fn sync_loaded_model(&mut self, cx: &mut Cx) {
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

    fn sync_graph(&mut self, cx: &mut Cx, model: &StudioViewModel, graph: &StudioGraphView) {
        self.ui.label(cx, ids!(graph_selection)).set_text(
            cx,
            &format!(
                "{} of {} / {}",
                self.selected_graph_index + 1,
                model.graph_count,
                graph.graph_id
            ),
        );
        self.ui
            .label(cx, ids!(graph_identity))
            .set_text(cx, &format!("{} ({})", graph.display_name, graph.graph_id));
        self.ui
            .label(cx, ids!(graph_target))
            .set_text(cx, &graph.target_host_profile);
        self.ui.label(cx, ids!(graph_counts)).set_text(
            cx,
            &format!(
                "{} nodes / {} edges / {} packages / {} modules / {} shells",
                graph.node_count,
                graph.edge_count,
                graph.package_count,
                graph.module_count,
                graph.operator_shell_count
            ),
        );
        self.ui
            .label(cx, ids!(shell_preview))
            .set_text(cx, &shell_preview_lines(model));
        self.ui
            .label(cx, ids!(shell_routes))
            .set_text(cx, &shell_route_lines(model));
        self.ui
            .label(cx, ids!(shell_template))
            .set_text(cx, &shell_template_lines(model));
        self.ui.label(cx, ids!(shell_bundle_status)).set_text(
            cx,
            &shell_bundle_status_line(&self.last_shell_bundle_status),
        );
        self.ui
            .label(cx, ids!(graph_layout))
            .set_text(cx, &layout_lines(graph));
        self.sync_graph_canvas(cx, model, graph);
        self.ui
            .label(cx, ids!(graph_nodes))
            .set_text(cx, &node_lines(graph));
        self.ui
            .label(cx, ids!(graph_edges))
            .set_text(cx, &edge_lines(graph));
        self.sync_inspector(cx, model, graph);
    }

    fn sync_inspector(&mut self, cx: &mut Cx, model: &StudioViewModel, _graph: &StudioGraphView) {
        self.ui
            .label(cx, ids!(focused_issue))
            .set_text(cx, &issue_focus_line(model));
        self.ui
            .label(cx, ids!(selected_node))
            .set_text(cx, &selected_node_line(model));
        self.ui
            .label(cx, ids!(selected_reference))
            .set_text(cx, &selected_reference_line(model));
        self.ui
            .label(cx, ids!(selected_node_details))
            .set_text(cx, &selected_node_detail_lines(model));
        self.ui
            .label(cx, ids!(selected_edge))
            .set_text(cx, &selected_edge_line(model));
        self.ui
            .label(cx, ids!(selected_edge_details))
            .set_text(cx, &selected_edge_detail_lines(model));
    }

    fn sync_edit_report(&mut self, cx: &mut Cx) {
        if let Some(report) = self.last_edit_report.clone() {
            let save_issue = self.last_edit_save_issue.clone();
            self.ui
                .label(cx, ids!(edit_status))
                .set_text(cx, &edit_status_line(&report, &save_issue));
            self.ui
                .label(cx, ids!(edit_message))
                .set_text(cx, &report.message);
            self.ui
                .label(cx, ids!(edit_changed_fields))
                .set_text(cx, &changed_fields_line(&report));
            self.ui
                .label(cx, ids!(edit_validation))
                .set_text(cx, &edit_validation_line(&report));
        } else {
            let status = if self.last_edit_save_issue.is_empty() {
                "no edits requested"
            } else {
                self.last_edit_save_issue.as_str()
            };
            self.ui.label(cx, ids!(edit_status)).set_text(cx, status);
            self.ui.label(cx, ids!(edit_message)).set_text(cx, "");
            self.ui
                .label(cx, ids!(edit_changed_fields))
                .set_text(cx, "");
            self.ui.label(cx, ids!(edit_validation)).set_text(cx, "");
        }
    }

    fn sync_no_graph(&mut self, cx: &mut Cx) {
        self.ui.label(cx, ids!(graph_selection)).set_text(cx, "");
        self.ui
            .label(cx, ids!(graph_identity))
            .set_text(cx, "no graph loaded");
        self.ui.label(cx, ids!(graph_target)).set_text(cx, "");
        self.ui.label(cx, ids!(graph_counts)).set_text(cx, "");
        self.ui.label(cx, ids!(shell_preview)).set_text(cx, "");
        self.ui.label(cx, ids!(shell_routes)).set_text(cx, "");
        self.ui.label(cx, ids!(shell_template)).set_text(cx, "");
        self.ui
            .label(cx, ids!(shell_bundle_status))
            .set_text(cx, "");
        self.ui.label(cx, ids!(graph_layout)).set_text(cx, "");
        if let Some(mut canvas) = self
            .ui
            .widget(cx, ids!(graph_canvas))
            .borrow_mut::<StudioGraphCanvas>()
        {
            canvas.set_canvas_model(cx, StudioGraphCanvasModel::default());
        }
        self.ui.label(cx, ids!(graph_nodes)).set_text(cx, "");
        self.ui.label(cx, ids!(graph_edges)).set_text(cx, "");
        self.ui.label(cx, ids!(selected_node)).set_text(cx, "");
        self.ui.label(cx, ids!(selected_reference)).set_text(cx, "");
        self.ui
            .label(cx, ids!(selected_node_details))
            .set_text(cx, "");
        self.ui.label(cx, ids!(selected_edge)).set_text(cx, "");
        self.ui
            .label(cx, ids!(selected_edge_details))
            .set_text(cx, "");
        self.ui.label(cx, ids!(focused_issue)).set_text(cx, "");
    }

    fn sync_graph_canvas(&mut self, cx: &mut Cx, model: &StudioViewModel, graph: &StudioGraphView) {
        if let Some(mut canvas) = self
            .ui
            .widget(cx, ids!(graph_canvas))
            .borrow_mut::<StudioGraphCanvas>()
        {
            canvas.set_canvas_model(cx, graph_canvas_model(model, graph));
        }
    }

    fn sync_error(&mut self, cx: &mut Cx, error: &str) {
        self.ui.label(cx, ids!(project_source)).set_text(cx, "");
        self.ui
            .label(cx, ids!(project_identity))
            .set_text(cx, "project load failed");
        self.ui.label(cx, ids!(project_revision)).set_text(cx, "");
        self.ui
            .label(cx, ids!(validation_status))
            .set_text(cx, error);
        self.ui.label(cx, ids!(validation_issues)).set_text(cx, "");
        self.ui.label(cx, ids!(catalog_packages)).set_text(cx, "");
        self.ui.label(cx, ids!(host_profiles)).set_text(cx, "");
        self.sync_no_graph(cx);
        self.last_edit_report = None;
        self.last_edit_save_issue.clear();
        self.sync_edit_report(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.sync_project(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let canvas = self.ui.widget(cx, ids!(graph_canvas));
        for action in canvas.filter_actions(actions) {
            match action.cast::<StudioGraphCanvasAction>() {
                StudioGraphCanvasAction::SelectNode(node_id) => {
                    self.select_canvas_node(cx, &node_id);
                }
                StudioGraphCanvasAction::SelectEdge(edge_id) => {
                    self.select_canvas_edge(cx, &edge_id);
                }
                StudioGraphCanvasAction::None => {}
            }
        }
        if self
            .ui
            .button(cx, ids!(previous_graph_button))
            .clicked(actions)
        {
            self.select_previous_graph(cx);
        }
        if self.ui.button(cx, ids!(next_graph_button)).clicked(actions) {
            self.select_next_graph(cx);
        }
        if self.ui.button(cx, ids!(next_issue_button)).clicked(actions) {
            self.select_next_issue(cx);
        }
        if self.ui.button(cx, ids!(next_node_button)).clicked(actions) {
            self.select_next_node(cx);
        }
        if self.ui.button(cx, ids!(next_edge_button)).clicked(actions) {
            self.select_next_edge(cx);
        }
        if self
            .ui
            .button(cx, ids!(target_desktop_button))
            .clicked(actions)
        {
            self.retarget_selected_graph(cx, "host_run.profile.desktop");
        }
        if self
            .ui
            .button(cx, ids!(target_headset_button))
            .clicked(actions)
        {
            self.retarget_selected_graph(cx, "host_run.profile.headset");
        }
        if self
            .ui
            .button(cx, ids!(add_palette_module_button))
            .clicked(actions)
        {
            self.add_next_catalog_module_to_selected_graph(cx);
        }
        if self
            .ui
            .button(cx, ids!(export_shell_bundle_button))
            .clicked(actions)
        {
            self.export_shell_bundle_for_selected_graph(cx);
        }
        if self
            .ui
            .button(cx, ids!(verify_shell_bundle_button))
            .clicked(actions)
        {
            self.verify_shell_bundle_for_selected_graph(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_handoff_button))
            .clicked(actions)
        {
            self.prepare_shell_handoff_for_selected_graph(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_readiness_button))
            .clicked(actions)
        {
            self.inspect_shell_handoff_readiness(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_runbook_button))
            .clicked(actions)
        {
            self.inspect_shell_runbook(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_button))
            .clicked(actions)
        {
            self.review_shell_export_package(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_button))
            .clicked(actions)
        {
            self.write_shell_export_package_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_append_button))
            .clicked(actions)
        {
            self.append_shell_export_package_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_summary_button))
            .clicked(actions)
        {
            self.inspect_shell_export_package_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_next_button))
            .clicked(actions)
        {
            self.select_next_shell_export_package_baseline_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_promote_button))
            .clicked(actions)
        {
            self.promote_shell_export_package_baseline_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_compare_button))
            .clicked(actions)
        {
            self.compare_shell_export_package(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_manifest_button))
            .clicked(actions)
        {
            self.write_shell_handoff_manifest(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_button))
            .clicked(actions)
        {
            self.review_shell_handoff_acceptance(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_baseline_button))
            .clicked(actions)
        {
            self.write_shell_handoff_acceptance_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_baseline_append_button))
            .clicked(actions)
        {
            self.append_shell_handoff_acceptance_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_baseline_summary_button))
            .clicked(actions)
        {
            self.inspect_shell_handoff_acceptance_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_baseline_next_button))
            .clicked(actions)
        {
            self.select_next_shell_handoff_acceptance_baseline_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_baseline_promote_button))
            .clicked(actions)
        {
            self.promote_shell_handoff_acceptance_baseline_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_compare_button))
            .clicked(actions)
        {
            self.compare_shell_handoff_acceptance(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_button))
            .clicked(actions)
        {
            self.review_shell_release_candidate(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_manifest_button))
            .clicked(actions)
        {
            self.write_shell_release_candidate_manifest(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_append_button))
            .clicked(actions)
        {
            self.append_shell_release_candidate_manifest(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_summary_button))
            .clicked(actions)
        {
            self.inspect_shell_release_candidate_manifest(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_next_button))
            .clicked(actions)
        {
            self.select_next_shell_release_candidate_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_promote_button))
            .clicked(actions)
        {
            self.promote_shell_release_candidate_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_handoff_package_button))
            .clicked(actions)
        {
            self.review_shell_hostess_handoff_package(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_owner_intake_button))
            .clicked(actions)
        {
            self.review_shell_hostess_owner_intake(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_preview_button))
            .clicked(actions)
        {
            self.review_shell_hostess_staging_preview(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_file_plan_button))
            .clicked(actions)
        {
            self.review_shell_hostess_staging_file_plan(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_handoff_button))
            .clicked(actions)
        {
            self.review_shell_hostess_staging_handoff(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_button))
            .clicked(actions)
        {
            self.review_shell_hostess_staging_acceptance(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_append_button))
            .clicked(actions)
        {
            self.append_shell_hostess_staging_acceptance(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_summary_button))
            .clicked(actions)
        {
            self.inspect_shell_hostess_staging_acceptance(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_next_button))
            .clicked(actions)
        {
            self.select_next_shell_hostess_staging_acceptance_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_promote_button))
            .clicked(actions)
        {
            self.promote_shell_hostess_staging_acceptance_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_compare_button))
            .clicked(actions)
        {
            self.compare_shell_hostess_staging_acceptance(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_execution_request_button))
            .clicked(actions)
        {
            self.request_shell_hostess_staging_execution_adapter(cx);
        }
        if self
            .ui
            .button(cx, ids!(remove_selected_module_button))
            .clicked(actions)
        {
            self.remove_selected_module_from_selected_graph(cx);
        }
        if self
            .ui
            .button(cx, ids!(add_command_binding_button))
            .clicked(actions)
        {
            self.add_command_binding_to_selected_module(cx);
        }
        if self
            .ui
            .button(cx, ids!(remove_selected_binding_button))
            .clicked(actions)
        {
            self.remove_selected_binding_from_selected_graph(cx);
        }
    }
}
