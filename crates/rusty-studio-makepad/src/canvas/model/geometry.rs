use makepad_widgets::*;

pub(super) fn point_in_rect(point: DVec2, rect: Rect) -> bool {
    point.x >= rect.pos.x
        && point.y >= rect.pos.y
        && point.x <= rect.pos.x + rect.size.x
        && point.y <= rect.pos.y + rect.size.y
}

pub(super) fn point_segment_distance(point: DVec2, start: DVec2, end: DVec2) -> f64 {
    let segment = end - start;
    let length_squared = segment.x * segment.x + segment.y * segment.y;
    if length_squared <= f64::EPSILON {
        return ((point.x - start.x).powi(2) + (point.y - start.y).powi(2)).sqrt();
    }
    let t = (((point.x - start.x) * segment.x + (point.y - start.y) * segment.y) / length_squared)
        .clamp(0.0, 1.0);
    let projection = dvec2(start.x + segment.x * t, start.y + segment.y * t);
    ((point.x - projection.x).powi(2) + (point.y - projection.y).powi(2)).sqrt()
}
