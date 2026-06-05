use super::super::super::*;

impl App {
    pub(in crate::app_events::sync) fn sync_graph_canvas(
        &mut self,
        cx: &mut Cx,
        model: &StudioViewModel,
        graph: &StudioGraphView,
    ) {
        if let Some(mut canvas) = self
            .ui
            .widget(cx, ids!(graph_canvas))
            .borrow_mut::<StudioGraphCanvas>()
        {
            canvas.set_canvas_model(cx, graph_canvas_model(model, graph));
        }
    }
}
