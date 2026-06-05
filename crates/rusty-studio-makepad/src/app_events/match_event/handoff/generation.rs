use super::super::super::*;

impl App {
    pub(super) fn handle_shell_handoff_generation_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
    ) {
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
    }
}
