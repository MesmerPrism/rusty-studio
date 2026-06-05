use super::super::*;

impl App {
    pub(super) fn handle_graph_target_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .ui
            .button(cx, ids!(target_desktop_button))
            .clicked(actions)
        {
            self.retarget_selected_graph(cx, "host_run.profile.desktop");
        }
        if self
            .ui
            .button(cx, ids!(target_headset_button))
            .clicked(actions)
        {
            self.retarget_selected_graph(cx, "host_run.profile.headset");
        }
        if self
            .ui
            .button(cx, ids!(add_palette_module_button))
            .clicked(actions)
        {
            self.add_next_catalog_module_to_selected_graph(cx);
        }
    }

    pub(super) fn handle_graph_removal_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .ui
            .button(cx, ids!(remove_selected_module_button))
            .clicked(actions)
        {
            self.remove_selected_module_from_selected_graph(cx);
        }
    }
}
