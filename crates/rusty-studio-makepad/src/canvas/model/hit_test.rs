use makepad_widgets::*;

use super::super::viewport::CanvasViewport;
use super::geometry::{point_in_rect, point_segment_distance};
use super::types::{StudioGraphCanvasEdge, StudioGraphCanvasHit, StudioGraphCanvasModel};

const CANVAS_EDGE_HIT_DISTANCE: f64 = 8.0;

impl StudioGraphCanvasModel {
    pub(crate) fn hit_test_abs(&self, rect: Rect, abs: DVec2) -> Option<StudioGraphCanvasHit> {
        let viewport = CanvasViewport::for_rect(rect, self.logical_bounds()?);
        for node in self.nodes.iter().rev() {
            if point_in_rect(abs, viewport.node_rect(node)) {
                return Some(StudioGraphCanvasHit::Node(node.node_id.clone()));
            }
        }

        let mut closest_edge: Option<(f64, &StudioGraphCanvasEdge)> = None;
        for edge in &self.edges {
            let Some(distance) = self.edge_distance_abs(edge, &viewport, abs) else {
                continue;
            };
            if distance <= CANVAS_EDGE_HIT_DISTANCE {
                match closest_edge {
                    Some((closest_distance, _)) if closest_distance <= distance => {}
                    _ => closest_edge = Some((distance, edge)),
                }
            }
        }
        closest_edge.map(|(_, edge)| StudioGraphCanvasHit::Edge(edge.edge_id.clone()))
    }

    fn edge_distance_abs(
        &self,
        edge: &StudioGraphCanvasEdge,
        viewport: &CanvasViewport,
        abs: DVec2,
    ) -> Option<f64> {
        let source = self
            .nodes
            .iter()
            .find(|node| node.node_id == edge.source_node_id)?;
        let target = self
            .nodes
            .iter()
            .find(|node| node.node_id == edge.target_node_id)?;
        let source_center = viewport.node_center(source);
        let target_center = viewport.node_center(target);
        let mut points = Vec::with_capacity(4);
        points.push(source_center);
        if edge.route == "orthogonal" {
            let mid_x = (source_center.x + target_center.x) * 0.5;
            points.push(dvec2(mid_x, source_center.y));
            points.push(dvec2(mid_x, target_center.y));
        }
        points.push(target_center);

        points
            .windows(2)
            .map(|segment| point_segment_distance(abs, segment[0], segment[1]))
            .reduce(f64::min)
    }
}
