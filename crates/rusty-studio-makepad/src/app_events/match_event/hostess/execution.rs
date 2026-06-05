use super::super::super::*;

impl App {
    pub(super) fn handle_hostess_execution_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_execution_request_button))
            .clicked(actions)
        {
            self.request_shell_hostess_staging_execution_adapter(cx);
        }
    }
}
