use super::*;

mod bindings;
mod bundle;
mod canvas;
mod graph;
mod handoff;
mod hostess;
mod navigation;
mod release_candidate;
mod shell_package;

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.sync_project(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.handle_canvas_selection_actions(cx, actions);
        self.handle_navigation_actions(cx, actions);
        self.handle_graph_target_actions(cx, actions);
        self.handle_shell_bundle_actions(cx, actions);
        self.handle_shell_handoff_actions(cx, actions);
        self.handle_shell_package_actions(cx, actions);
        self.handle_shell_handoff_acceptance_actions(cx, actions);
        self.handle_release_candidate_actions(cx, actions);
        self.handle_hostess_actions(cx, actions);
        self.handle_graph_removal_actions(cx, actions);
        self.handle_binding_actions(cx, actions);
    }
}
