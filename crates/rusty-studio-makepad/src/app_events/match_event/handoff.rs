mod acceptance;
mod generation;

use super::super::*;

impl App {
    pub(super) fn handle_shell_handoff_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.handle_shell_handoff_generation_actions(cx, actions);
    }

    pub(super) fn handle_shell_handoff_acceptance_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
    ) {
        self.handle_shell_handoff_acceptance_review_actions(cx, actions);
        self.handle_shell_handoff_acceptance_baseline_actions(cx, actions);
    }
}
