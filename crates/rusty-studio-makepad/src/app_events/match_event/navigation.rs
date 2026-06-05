use super::super::*;

impl App {
    pub(super) fn handle_navigation_actions(&mut self, cx: &mut Cx, actions: &Actions) {
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
    }
}
