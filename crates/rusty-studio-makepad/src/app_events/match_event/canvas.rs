use super::super::*;

impl App {
    pub(super) fn handle_canvas_selection_actions(&mut self, cx: &mut Cx, actions: &Actions) {
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
    }
}
