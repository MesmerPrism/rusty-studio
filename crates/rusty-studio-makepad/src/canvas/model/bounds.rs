use super::super::viewport::CanvasViewportBounds;
use super::types::StudioGraphCanvasModel;

impl StudioGraphCanvasModel {
    pub(crate) fn logical_bounds(&self) -> Option<CanvasViewportBounds> {
        let first = self.nodes.first()?;
        let mut min_x = first.x;
        let mut min_y = first.y;
        let mut max_x = first.x + first.width;
        let mut max_y = first.y + first.height;
        for node in &self.nodes {
            min_x = min_x.min(node.x);
            min_y = min_y.min(node.y);
            max_x = max_x.max(node.x + node.width);
            max_y = max_y.max(node.y + node.height);
        }
        Some(CanvasViewportBounds {
            min_x: min_x as f64,
            min_y: min_y as f64,
            width: (max_x - min_x).max(1) as f64,
            height: (max_y - min_y).max(1) as f64,
        })
    }
}
