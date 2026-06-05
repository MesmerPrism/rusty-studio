use super::super::super::super::*;

impl App {
    pub(super) fn handle_shell_handoff_acceptance_review_buttons(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
    ) {
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
    }
}
