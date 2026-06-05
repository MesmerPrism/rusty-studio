use super::super::super::*;

impl App {
    pub(super) fn handle_hostess_package_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .ui
            .button(cx, ids!(shell_hostess_handoff_package_button))
            .clicked(actions)
        {
            self.review_shell_hostess_handoff_package(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_owner_intake_button))
            .clicked(actions)
        {
            self.review_shell_hostess_owner_intake(cx);
        }
    }
}
