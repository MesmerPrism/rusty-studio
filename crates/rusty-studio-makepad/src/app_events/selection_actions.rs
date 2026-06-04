use super::*;

impl App {
    pub(super) fn select_previous_graph(&mut self, cx: &mut Cx) {
        let graph_count = self.model.as_ref().map_or(0, |model| model.graphs.len());
        if graph_count == 0 {
            return;
        }
        let next_index = if self.selected_graph_index == 0 {
            graph_count - 1
        } else {
            self.selected_graph_index - 1
        };
        self.select_graph_index(cx, next_index);
    }

    pub(super) fn select_graph_index(&mut self, cx: &mut Cx, graph_index: usize) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        let Some(graph_id) = model
            .graphs
            .get(graph_index)
            .map(|graph| graph.graph_id.clone())
        else {
            return;
        };
        match load_studio_view_model_for_path(&source, Some(&graph_id), None, None, None) {
            Ok(model) => {
                self.selected_graph_index = model.selected_graph_index.unwrap_or(graph_index);
                self.selected_issue_check_id = model.selected_issue_check_id.clone();
                self.selected_node_id = model.selected_node_id.clone();
                self.selected_edge_id = model.selected_edge_id.clone();
                self.model = Some(model);
                self.sync_loaded_model(cx);
                self.ui.redraw(cx);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    pub(super) fn select_next_issue(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        let Some(next_issue_check_id) = next_issue_check_id(&model).map(str::to_string) else {
            return;
        };
        let requested_graph_id = model
            .validation_issues
            .iter()
            .find(|issue| issue.check_id == next_issue_check_id)
            .and_then(|issue| issue.graph_id.as_deref())
            .or(model.selected_graph_id.as_deref());
        match load_studio_view_model_for_path(
            &source,
            requested_graph_id,
            Some(&next_issue_check_id),
            None,
            None,
        ) {
            Ok(model) => {
                self.selected_graph_index = model
                    .selected_graph_index
                    .unwrap_or(self.selected_graph_index);
                self.selected_issue_check_id = model.selected_issue_check_id.clone();
                self.selected_node_id = model.selected_node_id.clone();
                self.selected_edge_id = model.selected_edge_id.clone();
                self.model = Some(model);
                self.sync_loaded_model(cx);
                self.ui.redraw(cx);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    pub(super) fn select_next_node(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        let Some(next_node_id) = next_node_id(&model).map(str::to_string) else {
            return;
        };
        match load_studio_view_model_for_path(
            &source,
            model.selected_graph_id.as_deref(),
            model.selected_issue_check_id.as_deref(),
            Some(&next_node_id),
            model.selected_edge_id.as_deref(),
        ) {
            Ok(model) => {
                self.selected_graph_index = model
                    .selected_graph_index
                    .unwrap_or(self.selected_graph_index);
                self.selected_issue_check_id = model.selected_issue_check_id.clone();
                self.selected_node_id = model.selected_node_id.clone();
                self.selected_edge_id = model.selected_edge_id.clone();
                self.model = Some(model);
                self.sync_loaded_model(cx);
                self.ui.redraw(cx);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    pub(super) fn select_next_edge(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        let Some(next_edge_id) = next_edge_id(&model).map(str::to_string) else {
            return;
        };
        match load_studio_view_model_for_path(
            &source,
            model.selected_graph_id.as_deref(),
            model.selected_issue_check_id.as_deref(),
            model.selected_node_id.as_deref(),
            Some(&next_edge_id),
        ) {
            Ok(model) => {
                self.selected_graph_index = model
                    .selected_graph_index
                    .unwrap_or(self.selected_graph_index);
                self.selected_issue_check_id = model.selected_issue_check_id.clone();
                self.selected_node_id = model.selected_node_id.clone();
                self.selected_edge_id = model.selected_edge_id.clone();
                self.model = Some(model);
                self.sync_loaded_model(cx);
                self.ui.redraw(cx);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    pub(super) fn select_canvas_node(&mut self, cx: &mut Cx, node_id: &str) {
        let current_edge_id = self
            .model
            .as_ref()
            .and_then(|model| model.selected_edge_id.clone());
        self.select_canvas_request(cx, Some(node_id), current_edge_id.as_deref());
    }

    pub(super) fn select_canvas_edge(&mut self, cx: &mut Cx, edge_id: &str) {
        let current_node_id = self
            .model
            .as_ref()
            .and_then(|model| model.selected_node_id.clone());
        self.select_canvas_request(cx, current_node_id.as_deref(), Some(edge_id));
    }

    pub(super) fn select_canvas_request(
        &mut self,
        cx: &mut Cx,
        requested_node_id: Option<&str>,
        requested_edge_id: Option<&str>,
    ) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        match canvas_selection_view_model_for_project_source(
            &source,
            &model,
            self.selected_graph_index,
            requested_node_id,
            requested_edge_id,
        ) {
            Ok(model) => {
                self.selected_graph_index = model
                    .selected_graph_index
                    .unwrap_or(self.selected_graph_index);
                self.selected_issue_check_id = model.selected_issue_check_id.clone();
                self.selected_node_id = model.selected_node_id.clone();
                self.selected_edge_id = model.selected_edge_id.clone();
                self.model = Some(model);
                self.sync_loaded_model(cx);
                self.ui.redraw(cx);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    pub(super) fn select_next_graph(&mut self, cx: &mut Cx) {
        let graph_count = self.model.as_ref().map_or(0, |model| model.graphs.len());
        if graph_count == 0 {
            return;
        }
        self.select_graph_index(cx, (self.selected_graph_index + 1) % graph_count);
    }
}
