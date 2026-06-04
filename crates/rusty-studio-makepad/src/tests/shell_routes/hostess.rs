use super::*;

mod acceptance;
mod execution;
mod package;
mod staging;

#[test]
fn shell_hostess_handoff_package_reports_ready_from_makepad_route() {
    let root = temp_root("shell-hostess-handoff-package");
    let project_path = package::prepare_hostess_project(&root);
    let intake = package::assert_handoff_package_and_owner_intake(&project_path);
    let staging = staging::assert_staging_routes(&project_path, &intake.intake_path);
    acceptance::assert_staging_acceptance_routes(&project_path, &staging);
    execution::assert_execution_request_route(&project_path);
}
