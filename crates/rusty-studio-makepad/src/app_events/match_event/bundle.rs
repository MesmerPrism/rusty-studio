use super::super::*;

impl App {
    pub(super) fn handle_shell_bundle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
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
    }
}
