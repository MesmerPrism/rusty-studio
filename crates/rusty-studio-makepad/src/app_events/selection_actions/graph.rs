use super::super::*;

impl App {
    pub(in crate::app_events) fn select_previous_graph(&mut self, cx: &mut Cx) {
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

    fn select_graph_index(&mut self, cx: &mut Cx, graph_index: usize) {
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

    pub(in crate::app_events) fn select_next_graph(&mut self, cx: &mut Cx) {
        let graph_count = self.model.as_ref().map_or(0, |model| model.graphs.len());
        if graph_count == 0 {
            return;
        }
        self.select_graph_index(cx, (self.selected_graph_index + 1) % graph_count);
    }
}
