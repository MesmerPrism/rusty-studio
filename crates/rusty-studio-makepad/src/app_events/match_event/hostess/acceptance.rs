use super::super::super::*;

impl App {
    pub(super) fn handle_hostess_acceptance_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_button))
            .clicked(actions)
        {
            self.review_shell_hostess_staging_acceptance(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_append_button))
            .clicked(actions)
        {
            self.append_shell_hostess_staging_acceptance(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_summary_button))
            .clicked(actions)
        {
            self.inspect_shell_hostess_staging_acceptance(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_next_button))
            .clicked(actions)
        {
            self.select_next_shell_hostess_staging_acceptance_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_promote_button))
            .clicked(actions)
        {
            self.promote_shell_hostess_staging_acceptance_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_acceptance_compare_button))
            .clicked(actions)
        {
            self.compare_shell_hostess_staging_acceptance(cx);
        }
    }
}
