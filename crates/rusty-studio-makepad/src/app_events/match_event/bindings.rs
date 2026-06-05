use super::super::*;

impl App {
    pub(super) fn handle_binding_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .ui
            .button(cx, ids!(add_command_binding_button))
            .clicked(actions)
        {
            self.add_command_binding_to_selected_module(cx);
        }
        if self
            .ui
            .button(cx, ids!(remove_selected_binding_button))
            .clicked(actions)
        {
            self.remove_selected_binding_from_selected_graph(cx);
        }
    }
}
