mod generated;
mod layout;

use rusty_studio_model::{StudioGraphView, StudioViewModel};

use super::model::StudioGraphCanvasModel;
use generated::generated_canvas_model;
use layout::layout_canvas_model;

pub(crate) fn graph_canvas_model(
    model: &StudioViewModel,
    graph: &StudioGraphView,
) -> StudioGraphCanvasModel {
    if let Some(canvas_model) = layout_canvas_model(model, graph) {
        return canvas_model;
    }

    generated_canvas_model(model, graph)
}
