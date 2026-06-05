use super::super::super::*;

impl App {
    pub(super) fn handle_hostess_staging_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_preview_button))
            .clicked(actions)
        {
            self.review_shell_hostess_staging_preview(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_file_plan_button))
            .clicked(actions)
        {
            self.review_shell_hostess_staging_file_plan(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_handoff_button))
            .clicked(actions)
        {
            self.review_shell_hostess_staging_handoff(cx);
        }
    }
}
