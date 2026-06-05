use super::super::*;

impl App {
    pub(in crate::app_events) fn sync_edit_report(&mut self, cx: &mut Cx) {
        if let Some(report) = self.last_edit_report.clone() {
            let save_issue = self.last_edit_save_issue.clone();
            self.ui
                .label(cx, ids!(edit_status))
                .set_text(cx, &edit_status_line(&report, &save_issue));
            self.ui
                .label(cx, ids!(edit_message))
                .set_text(cx, &report.message);
            self.ui
                .label(cx, ids!(edit_changed_fields))
                .set_text(cx, &changed_fields_line(&report));
            self.ui
                .label(cx, ids!(edit_validation))
                .set_text(cx, &edit_validation_line(&report));
        } else {
            let status = if self.last_edit_save_issue.is_empty() {
                "no edits requested"
            } else {
                self.last_edit_save_issue.as_str()
            };
            self.ui.label(cx, ids!(edit_status)).set_text(cx, status);
            self.ui.label(cx, ids!(edit_message)).set_text(cx, "");
            self.ui
                .label(cx, ids!(edit_changed_fields))
                .set_text(cx, "");
            self.ui.label(cx, ids!(edit_validation)).set_text(cx, "");
        }
    }
}
