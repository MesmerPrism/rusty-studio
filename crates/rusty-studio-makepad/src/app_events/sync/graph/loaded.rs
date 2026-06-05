use super::super::super::*;

impl App {
    pub(in crate::app_events::sync) fn sync_graph(
        &mut self,
        cx: &mut Cx,
        model: &StudioViewModel,
        graph: &StudioGraphView,
    ) {
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
}
