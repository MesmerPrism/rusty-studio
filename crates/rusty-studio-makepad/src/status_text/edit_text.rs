use super::*;

pub(crate) fn edit_status_line(report: &StudioEditReport, save_issue: &str) -> String {
    let status = match report.status {
        StudioEditStatus::Applied => "applied",
        StudioEditStatus::Rejected => "rejected",
    };
    let issue = report.issue_code.as_deref().unwrap_or("none");
    if save_issue.is_empty() {
        format!(
            "{status}; rev {} -> {}; issue {issue}",
            report.original_revision, report.resulting_revision
        )
    } else {
        format!(
            "{status}; rev {} -> {}; issue {issue}; {save_issue}",
            report.original_revision, report.resulting_revision
        )
    }
}

pub(crate) fn changed_fields_line(report: &StudioEditReport) -> String {
    if report.changed_fields.is_empty() {
        "none".to_string()
    } else {
        report.changed_fields.join("\n")
    }
}

pub(crate) fn edit_validation_line(report: &StudioEditReport) -> String {
    let status = match report.validation.status {
        StudioValidationStatus::Pass => "pass",
        StudioValidationStatus::Fail => "fail",
    };
    format!("{status}; {} check(s)", report.validation.checks.len())
}
