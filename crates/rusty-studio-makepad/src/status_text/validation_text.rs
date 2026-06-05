use super::*;

mod focus;
mod issues;

pub(crate) use focus::*;
pub(crate) use issues::*;

pub(crate) fn validation_line(model: &StudioViewModel) -> String {
    let status = match model.validation_status {
        StudioValidationStatus::Pass => "pass",
        StudioValidationStatus::Fail => "fail",
    };
    format!(
        "{status}; {} passing checks, {} failing checks",
        model.validation_pass_count, model.validation_fail_count
    )
}
