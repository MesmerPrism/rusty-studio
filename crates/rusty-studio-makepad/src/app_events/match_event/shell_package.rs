use super::super::*;

impl App {
    pub(super) fn handle_shell_package_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .ui
            .button(cx, ids!(shell_export_package_button))
            .clicked(actions)
        {
            self.review_shell_export_package(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_button))
            .clicked(actions)
        {
            self.write_shell_export_package_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_append_button))
            .clicked(actions)
        {
            self.append_shell_export_package_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_summary_button))
            .clicked(actions)
        {
            self.inspect_shell_export_package_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_next_button))
            .clicked(actions)
        {
            self.select_next_shell_export_package_baseline_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_promote_button))
            .clicked(actions)
        {
            self.promote_shell_export_package_baseline_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_compare_button))
            .clicked(actions)
        {
            self.compare_shell_export_package(cx);
        }
    }
}
