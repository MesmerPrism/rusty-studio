mod baseline;
mod review;

use super::super::super::*;

impl App {
    pub(super) fn handle_shell_handoff_acceptance_review_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
    ) {
        self.handle_shell_handoff_acceptance_review_buttons(cx, actions);
    }

    pub(super) fn handle_shell_handoff_acceptance_baseline_actions(
        &mut self,
        cx: &mut Cx,
        actions: &Actions,
    ) {
        self.handle_shell_handoff_acceptance_baseline_buttons(cx, actions);
    }
}
