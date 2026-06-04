use makepad_widgets::*;
use rusty_studio_model::{StudioGraphView, StudioViewModel};

const CANVAS_EDGE_HIT_DISTANCE: f64 = 8.0;

#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct StudioGraphCanvasModel {
    pub(crate) layout_id: String,
    pub(crate) coordinate_space: String,
    pub(crate) nodes: Vec<StudioGraphCanvasNode>,
    pub(crate) edges: Vec<StudioGraphCanvasEdge>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StudioGraphCanvasNode {
    pub(crate) node_id: String,
    pub(crate) label: String,
    pub(crate) kind: String,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) width: i32,
    pub(crate) height: i32,
    pub(crate) validation_issue_count: usize,
    pub(crate) selected: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct StudioGraphCanvasEdge {
    pub(crate) edge_id: String,
    pub(crate) source_node_id: String,
    pub(crate) target_node_id: String,
    pub(crate) route: String,
    pub(crate) validation_issue_count: usize,
    pub(crate) selected: bool,
}

#[derive(Clone, Debug, Default)]
pub(crate) enum StudioGraphCanvasAction {
    #[default]
    None,
    SelectNode(String),
    SelectEdge(String),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum StudioGraphCanvasHit {
    Node(String),
    Edge(String),
}

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

#[derive(Script, ScriptHook, Widget)]
pub struct StudioGraphCanvas {
    #[uid]
    uid: WidgetUid,
    #[walk]
    walk: Walk,
    #[redraw]
    #[live]
    draw_bg: DrawColor,
    #[live]
    draw_edge: DrawVector,
    #[live]
    draw_node: DrawColor,
    #[live]
    draw_text: DrawText,
    #[live]
    bg_color: Vec4f,
    #[live]
    node_color: Vec4f,
    #[live]
    node_selected_color: Vec4f,
    #[live]
    node_issue_color: Vec4f,
    #[live]
    edge_color: Vec4f,
    #[live]
    edge_selected_color: Vec4f,
    #[live]
    edge_issue_color: Vec4f,
    #[live]
    border_color: Vec4f,
    #[live]
    selected_border_color: Vec4f,
    #[live]
    text_color: Vec4f,
    #[live]
    issue_text_color: Vec4f,
    #[rust]
    area: Area,
    #[rust]
    model: StudioGraphCanvasModel,
}

impl Widget for StudioGraphCanvas {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if !self.area.is_valid(cx) {
            return;
        }
        match event.hits(cx, self.area) {
            Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe) => {
                if self.hit_test_abs(cx, fe.abs).is_some() {
                    cx.set_cursor(MouseCursor::Hand);
                }
            }
            Hit::FingerDown(fe) if fe.is_primary_hit() => {
                if self.hit_test_abs(cx, fe.abs).is_some() {
                    cx.set_cursor(MouseCursor::Hand);
                }
            }
            Hit::FingerUp(fe) if fe.is_primary_hit() && fe.is_over => {
                if let Some(hit) = self.hit_test_abs(cx, fe.abs) {
                    let action = match hit {
                        StudioGraphCanvasHit::Node(node_id) => {
                            StudioGraphCanvasAction::SelectNode(node_id)
                        }
                        StudioGraphCanvasHit::Edge(edge_id) => {
                            StudioGraphCanvasAction::SelectEdge(edge_id)
                        }
                    };
                    cx.widget_action(self.widget_uid(), action);
                }
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let rect = cx.walk_turtle_with_area(&mut self.area, walk);
        self.draw_bg.color = self.bg_color;
        self.draw_bg.draw_abs(cx, rect);
        if rect.size.x <= 2.0 || rect.size.y <= 2.0 {
            return DrawStep::done();
        }

        let Some(bounds) = self.logical_bounds() else {
            self.draw_text.color = self.text_color;
            self.draw_text
                .draw_abs(cx, dvec2(rect.pos.x + 18.0, rect.pos.y + 18.0), "no layout");
            return DrawStep::done();
        };

        let viewport = self.viewport_for_rect(rect, bounds);
        self.draw_edges(cx, &viewport);
        self.draw_nodes(cx, &viewport);
        DrawStep::done()
    }
}

impl StudioGraphCanvas {
    pub(crate) fn set_canvas_model(&mut self, cx: &mut Cx, model: StudioGraphCanvasModel) {
        self.model = model;
        self.redraw(cx);
    }

    fn logical_bounds(&self) -> Option<CanvasViewportBounds> {
        self.model.logical_bounds()
    }

    fn viewport_for_rect(&self, rect: Rect, bounds: CanvasViewportBounds) -> CanvasViewport {
        CanvasViewport::for_rect(rect, bounds)
    }

    fn hit_test_abs(&self, cx: &Cx, abs: DVec2) -> Option<StudioGraphCanvasHit> {
        self.model.hit_test_abs(self.area.rect(cx), abs)
    }

    fn draw_edges(&mut self, cx: &mut Cx2d, viewport: &CanvasViewport) {
        self.draw_edge.begin();
        for edge in &self.model.edges {
            let Some(source) = self
                .model
                .nodes
                .iter()
                .find(|node| node.node_id == edge.source_node_id)
            else {
                continue;
            };
            let Some(target) = self
                .model
                .nodes
                .iter()
                .find(|node| node.node_id == edge.target_node_id)
            else {
                continue;
            };
            let source_center = viewport.node_center(source);
            let target_center = viewport.node_center(target);
            let color = if edge.selected {
                self.edge_selected_color
            } else if edge.validation_issue_count > 0 {
                self.edge_issue_color
            } else {
                self.edge_color
            };
            self.draw_edge.set_color(color.x, color.y, color.z, color.w);
            self.draw_edge
                .move_to(source_center.x as f32, source_center.y as f32);
            if edge.route == "orthogonal" {
                let mid_x = (source_center.x + target_center.x) * 0.5;
                self.draw_edge.line_to(mid_x as f32, source_center.y as f32);
                self.draw_edge.line_to(mid_x as f32, target_center.y as f32);
            }
            self.draw_edge
                .line_to(target_center.x as f32, target_center.y as f32);
            self.draw_edge.stroke(if edge.selected { 3.0 } else { 1.6 });
        }
        self.draw_edge.end(cx);
    }

    fn draw_nodes(&mut self, cx: &mut Cx2d, viewport: &CanvasViewport) {
        let nodes = self.model.nodes.clone();
        for node in &nodes {
            let rect = viewport.node_rect(node);
            let fill = if node.selected {
                self.node_selected_color
            } else if node.validation_issue_count > 0 {
                self.node_issue_color
            } else {
                self.node_color
            };
            self.draw_node.color = fill;
            self.draw_node.draw_abs(cx, rect);
            self.draw_node_border(
                cx,
                rect,
                if node.selected {
                    self.selected_border_color
                } else {
                    self.border_color
                },
                if node.selected { 2.0 } else { 1.0 },
            );
            self.draw_text.color = if node.validation_issue_count > 0 {
                self.issue_text_color
            } else {
                self.text_color
            };
            self.draw_text.draw_abs(
                cx,
                dvec2(rect.pos.x + 8.0, rect.pos.y + 8.0),
                &short_canvas_label(&node.label, 26),
            );
            self.draw_text.draw_abs(
                cx,
                dvec2(rect.pos.x + 8.0, rect.pos.y + 24.0),
                &format!("{}  {}", node.kind, node.node_id),
            );
            if node.validation_issue_count > 0 {
                self.draw_text.draw_abs(
                    cx,
                    dvec2(rect.pos.x + 8.0, rect.pos.y + 40.0),
                    &format!("issues: {}", node.validation_issue_count),
                );
            }
        }
    }

    fn draw_node_border(&mut self, cx: &mut Cx2d, rect: Rect, color: Vec4f, width: f64) {
        self.draw_node.color = color;
        self.draw_node.draw_abs(
            cx,
            Rect {
                pos: rect.pos,
                size: dvec2(rect.size.x, width),
            },
        );
        self.draw_node.draw_abs(
            cx,
            Rect {
                pos: dvec2(rect.pos.x, rect.pos.y + rect.size.y - width),
                size: dvec2(rect.size.x, width),
            },
        );
        self.draw_node.draw_abs(
            cx,
            Rect {
                pos: rect.pos,
                size: dvec2(width, rect.size.y),
            },
        );
        self.draw_node.draw_abs(
            cx,
            Rect {
                pos: dvec2(rect.pos.x + rect.size.x - width, rect.pos.y),
                size: dvec2(width, rect.size.y),
            },
        );
    }
}

#[derive(Clone, Copy)]
pub(crate) struct CanvasViewportBounds {
    min_x: f64,
    min_y: f64,
    width: f64,
    height: f64,
}

pub(crate) struct CanvasViewport {
    origin_x: f64,
    origin_y: f64,
    scale: f64,
    bounds: CanvasViewportBounds,
}

impl CanvasViewport {
    pub(crate) fn for_rect(rect: Rect, bounds: CanvasViewportBounds) -> Self {
        let margin = 18.0_f64;
        let content_width = (rect.size.x - margin * 2.0).max(1.0);
        let content_height = (rect.size.y - margin * 2.0).max(1.0);
        let scale = (content_width / bounds.width)
            .min(content_height / bounds.height)
            .max(0.1);
        let drawn_width = bounds.width * scale;
        let drawn_height = bounds.height * scale;
        Self {
            origin_x: rect.pos.x + (rect.size.x - drawn_width) * 0.5,
            origin_y: rect.pos.y + (rect.size.y - drawn_height) * 0.5,
            scale,
            bounds,
        }
    }

    pub(crate) fn node_rect(&self, node: &StudioGraphCanvasNode) -> Rect {
        Rect {
            pos: dvec2(
                self.origin_x + (node.x as f64 - self.bounds.min_x) * self.scale,
                self.origin_y + (node.y as f64 - self.bounds.min_y) * self.scale,
            ),
            size: dvec2(
                (node.width as f64 * self.scale).max(42.0),
                (node.height as f64 * self.scale).max(32.0),
            ),
        }
    }

    pub(crate) fn node_center(&self, node: &StudioGraphCanvasNode) -> DVec2 {
        let rect = self.node_rect(node);
        dvec2(
            rect.pos.x + rect.size.x * 0.5,
            rect.pos.y + rect.size.y * 0.5,
        )
    }
}

fn point_in_rect(point: DVec2, rect: Rect) -> bool {
    point.x >= rect.pos.x
        && point.y >= rect.pos.y
        && point.x <= rect.pos.x + rect.size.x
        && point.y <= rect.pos.y + rect.size.y
}

fn point_segment_distance(point: DVec2, start: DVec2, end: DVec2) -> f64 {
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

pub(crate) fn graph_canvas_model(
    model: &StudioViewModel,
    graph: &StudioGraphView,
) -> StudioGraphCanvasModel {
    if let Some(layout) = graph.layout.as_ref() {
        return StudioGraphCanvasModel {
            layout_id: layout.layout_id.clone(),
            coordinate_space: layout.coordinate_space.clone(),
            nodes: layout
                .nodes
                .iter()
                .map(|layout_node| {
                    let row = graph
                        .node_rows
                        .iter()
                        .find(|node| node.node_id == layout_node.node_id);
                    StudioGraphCanvasNode {
                        node_id: layout_node.node_id.clone(),
                        label: row
                            .map(|node| node.label.clone())
                            .unwrap_or_else(|| layout_node.node_id.clone()),
                        kind: row
                            .map(|node| node.kind.clone())
                            .unwrap_or_else(|| "missing".to_string()),
                        x: layout_node.x,
                        y: layout_node.y,
                        width: layout_node.width,
                        height: layout_node.height,
                        validation_issue_count: layout_node.validation_issue_count,
                        selected: model.selected_node_id.as_deref()
                            == Some(layout_node.node_id.as_str()),
                    }
                })
                .collect(),
            edges: layout
                .edges
                .iter()
                .map(|layout_edge| {
                    let row = graph
                        .edge_rows
                        .iter()
                        .find(|edge| edge.edge_id == layout_edge.edge_id);
                    StudioGraphCanvasEdge {
                        edge_id: layout_edge.edge_id.clone(),
                        source_node_id: row
                            .map(|edge| edge.source_node_id.clone())
                            .unwrap_or_default(),
                        target_node_id: row
                            .map(|edge| edge.target_node_id.clone())
                            .unwrap_or_default(),
                        route: layout_edge.route.clone(),
                        validation_issue_count: layout_edge.validation_issue_count,
                        selected: model.selected_edge_id.as_deref()
                            == Some(layout_edge.edge_id.as_str()),
                    }
                })
                .collect(),
        };
    }

    generated_canvas_model(model, graph)
}

fn generated_canvas_model(
    model: &StudioViewModel,
    graph: &StudioGraphView,
) -> StudioGraphCanvasModel {
    let nodes = graph
        .node_rows
        .iter()
        .enumerate()
        .map(|(index, node)| StudioGraphCanvasNode {
            node_id: node.node_id.clone(),
            label: node.label.clone(),
            kind: node.kind.clone(),
            x: 40 + ((index % 3) as i32 * 260),
            y: 40 + ((index / 3) as i32 * 150),
            width: 220,
            height: 72,
            validation_issue_count: node.validation_issue_count,
            selected: model.selected_node_id.as_deref() == Some(node.node_id.as_str()),
        })
        .collect();
    let edges = graph
        .edge_rows
        .iter()
        .map(|edge| StudioGraphCanvasEdge {
            edge_id: edge.edge_id.clone(),
            source_node_id: edge.source_node_id.clone(),
            target_node_id: edge.target_node_id.clone(),
            route: "direct".to_string(),
            validation_issue_count: edge.validation_issue_count,
            selected: model.selected_edge_id.as_deref() == Some(edge.edge_id.as_str()),
        })
        .collect();
    StudioGraphCanvasModel {
        layout_id: "studio.layout.generated_readonly".to_string(),
        coordinate_space: "studio.canvas.generated_2d".to_string(),
        nodes,
        edges,
    }
}

fn short_canvas_label(value: &str, max_chars: usize) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }
    let mut truncated = value
        .chars()
        .take(max_chars.saturating_sub(3))
        .collect::<String>();
    truncated.push_str("...");
    truncated
}
