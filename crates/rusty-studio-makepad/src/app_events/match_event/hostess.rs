mod acceptance;
mod execution;
mod package;
mod staging;

use super::super::*;

impl App {
    pub(super) fn handle_hostess_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.handle_hostess_package_actions(cx, actions);
        self.handle_hostess_staging_actions(cx, actions);
        self.handle_hostess_acceptance_actions(cx, actions);
        self.handle_hostess_execution_actions(cx, actions);
    }
}
