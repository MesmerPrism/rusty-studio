use super::super::super::super::*;

impl App {
    pub(super) fn handle_shell_handoff_acceptance_baseline_buttons(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
    ) {
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
    }
}
