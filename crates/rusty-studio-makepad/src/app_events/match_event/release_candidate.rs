use super::super::*;

impl App {
    pub(super) fn handle_release_candidate_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_button))
            .clicked(actions)
        {
            self.review_shell_release_candidate(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_manifest_button))
            .clicked(actions)
        {
            self.write_shell_release_candidate_manifest(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_append_button))
            .clicked(actions)
        {
            self.append_shell_release_candidate_manifest(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_summary_button))
            .clicked(actions)
        {
            self.inspect_shell_release_candidate_manifest(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_next_button))
            .clicked(actions)
        {
            self.select_next_shell_release_candidate_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_promote_button))
            .clicked(actions)
        {
            self.promote_shell_release_candidate_default(cx);
        }
    }
}
