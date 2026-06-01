pub use makepad_widgets;

use makepad_widgets::*;
use rusty_studio_core::{
    add_binding_to_graph, add_next_catalog_module_from_package_to_graph,
    add_next_catalog_module_to_graph, load_project, remove_binding_from_graph,
    remove_module_from_graph, retarget_graph_host_profile, save_project, view_model_for_graph,
    view_model_for_graph_issue_node_and_edge,
};
use rusty_studio_model::{
    StudioBindingKind, StudioEditReport, StudioEditStatus, StudioGraphView, StudioValidationStatus,
    StudioViewModel,
};
use std::path::{Path, PathBuf};

app_main!(App);

const CANVAS_EDGE_HIT_DISTANCE: f64 = 8.0;

script_mod! {
    use mod.prelude.widgets.*

    let PageTitle = Label{
        width: Fit height: Fit
        draw_text.color: #x111827
        draw_text.text_style: theme.font_bold{font_size: 24.0}
    }

    let SectionTitle = Label{
        width: Fit height: Fit
        draw_text.color: #x263238
        draw_text.text_style: theme.font_bold{font_size: 16.0}
    }

    let FieldLabel = Label{
        width: 150.0 height: Fit
        draw_text.color: #x5d6875
        draw_text.text_style.font_size: 12.0
    }

    let FieldValue = Label{
        width: Fill height: Fit
        draw_text.color: #x111827
        draw_text.text_style.font_size: 13.0
    }

    let SmallValue = Label{
        width: Fill height: Fit
        draw_text.color: #x3f4a54
        draw_text.text_style.font_size: 12.0
    }

    let Panel = RoundedView{
        width: Fill height: Fit
        flow: Down
        spacing: 8.0
        padding: 14.0
        draw_bg +: {
            color: #xffffff
            border_color: #xd8dde3
            border_size: 1.0
            border_radius: 8.0
        }
    }

    let Row = View{
        width: Fill height: Fit
        flow: Right
        spacing: 10.0
        align: Align{y: 0.5}
    }

    let ButtonRow = View{
        width: Fill height: Fit
        flow: Right
        spacing: 8.0
        align: Align{y: 0.5}
    }

    let ActionButton = Button{
        width: Fit height: 32.0
        padding: Inset{left: 12.0 right: 12.0 top: 7.0 bottom: 7.0}
        draw_bg +: {
            color: #xeaf0f6
            color_hover: #xdce8f6
            color_down: #xcbd9ea
            color_focus: #xe0edf9
            border_color: #xc7d0dc
            border_color_hover: #xb7c6d8
            border_color_down: #xa8b7ca
            border_color_focus: #x7aa0c8
            border_size: 1.0
            border_radius: 6.0
        }
        draw_text +: {
            color: #x111827
            color_hover: #x111827
            color_down: #x111827
            color_focus: #x111827
            text_style: theme.font_bold{font_size: 12.0}
        }
    }

    let Rule = SolidView{
        width: Fill height: 1.0
        draw_bg.color: #xe7ebef
    }

    let StudioGraphCanvasBase = #(StudioGraphCanvas::register_widget(vm))
    let StudioGraphCanvas = set_type_default() do StudioGraphCanvasBase{
        width: Fill
        height: 280.0
        draw_bg +: {
            draw_depth: 0.0
            color: #xf8fafc
        }
        draw_edge +: {
            draw_depth: 1.0
        }
        draw_node +: {
            draw_depth: 2.0
            color: #xffffffff
        }
        draw_text +: {
            draw_depth: 3.0
            color: #x17202a
            text_style.font_size: 10.0
        }
        bg_color: #xf8fafc
        node_color: #xffffffff
        node_selected_color: #xe7f1ff
        node_issue_color: #xfff4e5
        edge_color: #x64748b
        edge_selected_color: #x1d4ed8
        edge_issue_color: #xd97706
        border_color: #xcbd5e1
        selected_border_color: #x2563eb
        text_color: #x17202a
        issue_text_color: #x9a3412
    }

    let ProjectPanel = Panel{
        SectionTitle{text: "Project"}
        Row{FieldLabel{text: "source"} project_source := SmallValue{text: ""}}
        Row{FieldLabel{text: "project"} project_identity := FieldValue{text: ""}}
        Row{FieldLabel{text: "revision"} project_revision := FieldValue{text: ""}}
        Row{FieldLabel{text: "validation"} validation_status := FieldValue{text: ""}}
    }

    let DiagnosticsPanel = Panel{
        SectionTitle{text: "Validation Diagnostics"}
        ButtonRow{
            next_issue_button := ActionButton{text: "Next Issue"}
        }
        Row{FieldLabel{text: "issues"} validation_issues := SmallValue{text: ""}}
    }

    let GraphPanel = Panel{
        SectionTitle{text: "Graph"}
        ButtonRow{
            previous_graph_button := ActionButton{text: "Prev Graph"}
            next_graph_button := ActionButton{text: "Next Graph"}
        }
        Row{FieldLabel{text: "selected"} graph_selection := FieldValue{text: ""}}
        Row{FieldLabel{text: "graph"} graph_identity := FieldValue{text: ""}}
        Row{FieldLabel{text: "target host"} graph_target := FieldValue{text: ""}}
        Row{FieldLabel{text: "counts"} graph_counts := SmallValue{text: ""}}
    }

    let PalettePanel = Panel{
        SectionTitle{text: "Reference Palette"}
        ButtonRow{
            add_palette_module_button := ActionButton{text: "Add Module From Package"}
        }
        Row{FieldLabel{text: "packages"} catalog_packages := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "profiles"} host_profiles := SmallValue{text: ""}}
    }

    let EditPanel = Panel{
        SectionTitle{text: "Edit Report"}
        ButtonRow{
            target_desktop_button := ActionButton{text: "Target Desktop"}
            target_headset_button := ActionButton{text: "Target Headset"}
        }
        ButtonRow{
            remove_selected_module_button := ActionButton{text: "Remove Selected Module"}
            add_command_binding_button := ActionButton{text: "Add Command To Selected"}
            remove_selected_binding_button := ActionButton{text: "Remove Selected Binding"}
        }
        Row{FieldLabel{text: "status"} edit_status := FieldValue{text: "no edits requested"}}
        Row{FieldLabel{text: "message"} edit_message := SmallValue{text: ""}}
        Row{FieldLabel{text: "changed"} edit_changed_fields := SmallValue{text: ""}}
        Row{FieldLabel{text: "validation"} edit_validation := SmallValue{text: ""}}
    }

    let CanvasPanel = Panel{
        SectionTitle{text: "Read-Only Graph Canvas"}
        graph_canvas := StudioGraphCanvas{}
        Rule{}
        Row{FieldLabel{text: "layout"} graph_layout := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "nodes"} graph_nodes := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "edges"} graph_edges := SmallValue{text: ""}}
    }

    let InspectorPanel = Panel{
        SectionTitle{text: "Inspector"}
        ButtonRow{
            next_node_button := ActionButton{text: "Next Node"}
            next_edge_button := ActionButton{text: "Next Edge"}
        }
        Row{FieldLabel{text: "selected node"} selected_node := FieldValue{text: ""}}
        Row{FieldLabel{text: "selected ref"} selected_reference := SmallValue{text: ""}}
        Row{FieldLabel{text: "details"} selected_node_details := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "selected edge"} selected_edge := FieldValue{text: ""}}
        Row{FieldLabel{text: "edge details"} selected_edge_details := SmallValue{text: ""}}
        Row{FieldLabel{text: "issue focus"} focused_issue := SmallValue{text: ""}}
        Row{FieldLabel{text: "authority"} authority_note := SmallValue{text: ""}}
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                pass.clear_color: #xf4f6f7
                window.inner_size: vec2(1180, 820)
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down
                    spacing: 0.0

                    SolidView{
                        width: Fill height: Fit
                        padding: Inset{left: 24.0 right: 24.0 top: 18.0 bottom: 16.0}
                        flow: Right
                        align: Align{y: 0.5}
                        draw_bg.color: #xfbfcf8

                        View{
                            width: Fill height: Fit
                            flow: Down
                            spacing: 3.0
                            PageTitle{text: "Rusty Studio"}
                            subtitle_label := Label{
                                text: "schema-first package/profile authoring surface"
                                draw_text.color: #x5d6875
                                draw_text.text_style.font_size: 12.0
                            }
                        }
                        mode_label := Label{
                            width: Fit height: Fit
                            text: "core-gated edits"
                            draw_text.color: #x2f6f5e
                            draw_text.text_style: theme.font_bold{font_size: 13.0}
                        }
                    }

                    Rule{}

                    ScrollYView{
                        width: Fill height: Fill
                        padding: 18.0
                        flow: Down
                        spacing: 12.0

                        ProjectPanel{}
                        DiagnosticsPanel{}
                        GraphPanel{}
                        PalettePanel{}
                        EditPanel{}
                        CanvasPanel{}
                        InspectorPanel{}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct StudioGraphCanvasModel {
    layout_id: String,
    coordinate_space: String,
    nodes: Vec<StudioGraphCanvasNode>,
    edges: Vec<StudioGraphCanvasEdge>,
}

#[derive(Clone, Debug, PartialEq)]
struct StudioGraphCanvasNode {
    node_id: String,
    label: String,
    kind: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    validation_issue_count: usize,
    selected: bool,
}

#[derive(Clone, Debug, PartialEq)]
struct StudioGraphCanvasEdge {
    edge_id: String,
    source_node_id: String,
    target_node_id: String,
    route: String,
    validation_issue_count: usize,
    selected: bool,
}

#[derive(Clone, Debug, Default)]
enum StudioGraphCanvasAction {
    #[default]
    None,
    SelectNode(String),
    SelectEdge(String),
}

#[derive(Clone, Debug, PartialEq)]
enum StudioGraphCanvasHit {
    Node(String),
    Edge(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SelectedBindingRequest {
    binding_kind: StudioBindingKind,
    source_node_id: String,
    target_node_id: String,
}

impl StudioGraphCanvasModel {
    fn logical_bounds(&self) -> Option<CanvasViewportBounds> {
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

    fn hit_test_abs(&self, rect: Rect, abs: DVec2) -> Option<StudioGraphCanvasHit> {
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
    fn set_canvas_model(&mut self, cx: &mut Cx, model: StudioGraphCanvasModel) {
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
struct CanvasViewportBounds {
    min_x: f64,
    min_y: f64,
    width: f64,
    height: f64,
}

struct CanvasViewport {
    origin_x: f64,
    origin_y: f64,
    scale: f64,
    bounds: CanvasViewportBounds,
}

impl CanvasViewport {
    fn for_rect(rect: Rect, bounds: CanvasViewportBounds) -> Self {
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

    fn node_rect(&self, node: &StudioGraphCanvasNode) -> Rect {
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

    fn node_center(&self, node: &StudioGraphCanvasNode) -> DVec2 {
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

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    project_source: Option<PathBuf>,
    #[rust]
    model: Option<StudioViewModel>,
    #[rust]
    selected_graph_index: usize,
    #[rust]
    selected_issue_check_id: Option<String>,
    #[rust]
    selected_node_id: Option<String>,
    #[rust]
    selected_edge_id: Option<String>,
    #[rust]
    last_edit_report: Option<StudioEditReport>,
    #[rust]
    last_edit_save_issue: String,
}

impl App {
    fn sync_project(&mut self, cx: &mut Cx) {
        match load_studio_view_model(
            initial_graph_id_from_args().as_deref(),
            initial_issue_check_id_from_args().as_deref(),
            initial_node_id_from_args().as_deref(),
            initial_edge_id_from_args().as_deref(),
        ) {
            Ok((source, model)) => self.set_model(cx, source, model),
            Err(error) => self.sync_error(cx, &error),
        }
    }

    fn set_model(&mut self, cx: &mut Cx, source: PathBuf, model: StudioViewModel) {
        self.selected_graph_index = model.selected_graph_index.unwrap_or(0);
        self.selected_issue_check_id = model.selected_issue_check_id.clone();
        self.selected_node_id = model.selected_node_id.clone();
        self.selected_edge_id = model.selected_edge_id.clone();
        self.project_source = Some(source);
        self.model = Some(model);
        self.sync_loaded_model(cx);
    }

    fn sync_loaded_model(&mut self, cx: &mut Cx) {
        let Some(model) = self.model.clone() else {
            self.sync_error(cx, "no Studio view model loaded");
            return;
        };
        let source = self.project_source.clone().unwrap_or_default();
        self.ui
            .label(cx, ids!(project_source))
            .set_text(cx, &source.display().to_string());
        self.ui.label(cx, ids!(project_identity)).set_text(
            cx,
            &format!("{} ({})", model.display_name, model.project_id),
        );
        self.ui.label(cx, ids!(project_revision)).set_text(
            cx,
            &format!("rev {} / {} graph(s)", model.revision, model.graph_count),
        );
        self.ui
            .label(cx, ids!(validation_status))
            .set_text(cx, &validation_line(&model));
        self.ui
            .label(cx, ids!(validation_issues))
            .set_text(cx, &validation_issue_lines(&model));
        self.ui
            .label(cx, ids!(catalog_packages))
            .set_text(cx, &catalog_package_lines(&model));
        self.ui
            .label(cx, ids!(host_profiles))
            .set_text(cx, &host_profile_lines(&model));

        if let Some(issue_code) = &model.selection_issue_code {
            self.sync_no_graph(cx);
            self.ui.label(cx, ids!(graph_selection)).set_text(
                cx,
                &format!(
                    "requested graph {:?} is unavailable ({issue_code})",
                    model.requested_graph_id
                ),
            );
        } else if let Some(graph) = model.graphs.get(self.selected_graph_index) {
            self.sync_graph(cx, &model, graph);
        } else {
            self.sync_no_graph(cx);
        }
        self.ui.label(cx, ids!(authority_note)).set_text(
            cx,
            "Makepad renders the view model; rusty-studio-core owns validation and resolution.",
        );
        self.sync_edit_report(cx);
    }

    fn sync_graph(&mut self, cx: &mut Cx, model: &StudioViewModel, graph: &StudioGraphView) {
        self.ui.label(cx, ids!(graph_selection)).set_text(
            cx,
            &format!(
                "{} of {} / {}",
                self.selected_graph_index + 1,
                model.graph_count,
                graph.graph_id
            ),
        );
        self.ui
            .label(cx, ids!(graph_identity))
            .set_text(cx, &format!("{} ({})", graph.display_name, graph.graph_id));
        self.ui
            .label(cx, ids!(graph_target))
            .set_text(cx, &graph.target_host_profile);
        self.ui.label(cx, ids!(graph_counts)).set_text(
            cx,
            &format!(
                "{} nodes / {} edges / {} packages / {} modules / {} shells",
                graph.node_count,
                graph.edge_count,
                graph.package_count,
                graph.module_count,
                graph.operator_shell_count
            ),
        );
        self.ui
            .label(cx, ids!(graph_layout))
            .set_text(cx, &layout_lines(graph));
        self.sync_graph_canvas(cx, model, graph);
        self.ui
            .label(cx, ids!(graph_nodes))
            .set_text(cx, &node_lines(graph));
        self.ui
            .label(cx, ids!(graph_edges))
            .set_text(cx, &edge_lines(graph));
        self.sync_inspector(cx, model, graph);
    }

    fn sync_inspector(&mut self, cx: &mut Cx, model: &StudioViewModel, _graph: &StudioGraphView) {
        self.ui
            .label(cx, ids!(focused_issue))
            .set_text(cx, &issue_focus_line(model));
        self.ui
            .label(cx, ids!(selected_node))
            .set_text(cx, &selected_node_line(model));
        self.ui
            .label(cx, ids!(selected_reference))
            .set_text(cx, &selected_reference_line(model));
        self.ui
            .label(cx, ids!(selected_node_details))
            .set_text(cx, &selected_node_detail_lines(model));
        self.ui
            .label(cx, ids!(selected_edge))
            .set_text(cx, &selected_edge_line(model));
        self.ui
            .label(cx, ids!(selected_edge_details))
            .set_text(cx, &selected_edge_detail_lines(model));
    }

    fn retarget_selected_graph(&mut self, cx: &mut Cx, target_host_profile: &str) {
        let Some(source) = self.project_source.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No project source is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No view model is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        match retarget_project_source(
            &source,
            &model,
            self.selected_graph_index,
            target_host_profile,
        ) {
            Ok((report, refreshed_model)) => {
                self.last_edit_report = Some(report);
                self.last_edit_save_issue.clear();
                if let Some(refreshed_model) = refreshed_model {
                    self.selected_graph_index = refreshed_model
                        .selected_graph_index
                        .unwrap_or(self.selected_graph_index);
                    self.model = Some(refreshed_model);
                }
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn add_next_catalog_module_to_selected_graph(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No project source is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No view model is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        let package_reference_id = match selected_package_reference_id(&model) {
            Ok(package_reference_id) => package_reference_id,
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
                return;
            }
        };
        match add_next_catalog_module_to_project_source(
            &source,
            &model,
            self.selected_graph_index,
            Some(&package_reference_id),
        ) {
            Ok((report, refreshed_model)) => {
                self.last_edit_report = Some(report);
                self.last_edit_save_issue.clear();
                if let Some(refreshed_model) = refreshed_model {
                    self.selected_graph_index = refreshed_model
                        .selected_graph_index
                        .unwrap_or(self.selected_graph_index);
                    self.model = Some(refreshed_model);
                }
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn remove_module_from_selected_graph(&mut self, cx: &mut Cx, module_reference_id: &str) {
        let Some(source) = self.project_source.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No project source is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No view model is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        match remove_module_from_project_source(
            &source,
            &model,
            self.selected_graph_index,
            module_reference_id,
        ) {
            Ok((report, refreshed_model)) => {
                self.last_edit_report = Some(report);
                self.last_edit_save_issue.clear();
                if let Some(refreshed_model) = refreshed_model {
                    self.selected_graph_index = refreshed_model
                        .selected_graph_index
                        .unwrap_or(self.selected_graph_index);
                    self.model = Some(refreshed_model);
                }
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn remove_selected_module_from_selected_graph(&mut self, cx: &mut Cx) {
        let Some(model) = self.model.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No view model is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        match selected_module_reference_id(&model) {
            Ok(module_reference_id) => {
                self.remove_module_from_selected_graph(cx, &module_reference_id);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    fn add_binding_to_selected_graph(
        &mut self,
        cx: &mut Cx,
        binding_kind: StudioBindingKind,
        source_node_id: &str,
        target_node_id: &str,
    ) {
        let Some(source) = self.project_source.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No project source is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No view model is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        match add_binding_to_project_source(
            &source,
            &model,
            self.selected_graph_index,
            binding_kind,
            source_node_id,
            target_node_id,
        ) {
            Ok((report, refreshed_model)) => {
                self.last_edit_report = Some(report);
                self.last_edit_save_issue.clear();
                if let Some(refreshed_model) = refreshed_model {
                    self.selected_graph_index = refreshed_model
                        .selected_graph_index
                        .unwrap_or(self.selected_graph_index);
                    self.model = Some(refreshed_model);
                }
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn add_command_binding_to_selected_module(&mut self, cx: &mut Cx) {
        let Some(model) = self.model.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No view model is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        match selected_command_binding_request(&model) {
            Ok(request) => {
                self.add_binding_to_selected_graph(
                    cx,
                    request.binding_kind,
                    &request.source_node_id,
                    &request.target_node_id,
                );
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    fn remove_binding_from_selected_graph(
        &mut self,
        cx: &mut Cx,
        binding_kind: StudioBindingKind,
        source_node_id: &str,
        target_node_id: &str,
    ) {
        let Some(source) = self.project_source.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No project source is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No view model is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        match remove_binding_from_project_source(
            &source,
            &model,
            self.selected_graph_index,
            binding_kind,
            source_node_id,
            target_node_id,
        ) {
            Ok((report, refreshed_model)) => {
                self.last_edit_report = Some(report);
                self.last_edit_save_issue.clear();
                if let Some(refreshed_model) = refreshed_model {
                    self.selected_graph_index = refreshed_model
                        .selected_graph_index
                        .unwrap_or(self.selected_graph_index);
                    self.model = Some(refreshed_model);
                }
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn remove_selected_binding_from_selected_graph(&mut self, cx: &mut Cx) {
        let Some(model) = self.model.clone() else {
            self.last_edit_report = None;
            self.last_edit_save_issue = "No view model is loaded".to_string();
            self.sync_edit_report(cx);
            self.ui.redraw(cx);
            return;
        };
        match selected_binding_request(&model) {
            Ok(request) => {
                self.remove_binding_from_selected_graph(
                    cx,
                    request.binding_kind,
                    &request.source_node_id,
                    &request.target_node_id,
                );
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    fn sync_edit_report(&mut self, cx: &mut Cx) {
        if let Some(report) = self.last_edit_report.clone() {
            let save_issue = self.last_edit_save_issue.clone();
            self.ui
                .label(cx, ids!(edit_status))
                .set_text(cx, &edit_status_line(&report, &save_issue));
            self.ui
                .label(cx, ids!(edit_message))
                .set_text(cx, &report.message);
            self.ui
                .label(cx, ids!(edit_changed_fields))
                .set_text(cx, &changed_fields_line(&report));
            self.ui
                .label(cx, ids!(edit_validation))
                .set_text(cx, &edit_validation_line(&report));
        } else {
            let status = if self.last_edit_save_issue.is_empty() {
                "no edits requested"
            } else {
                self.last_edit_save_issue.as_str()
            };
            self.ui.label(cx, ids!(edit_status)).set_text(cx, status);
            self.ui.label(cx, ids!(edit_message)).set_text(cx, "");
            self.ui
                .label(cx, ids!(edit_changed_fields))
                .set_text(cx, "");
            self.ui.label(cx, ids!(edit_validation)).set_text(cx, "");
        }
    }

    fn sync_no_graph(&mut self, cx: &mut Cx) {
        self.ui.label(cx, ids!(graph_selection)).set_text(cx, "");
        self.ui
            .label(cx, ids!(graph_identity))
            .set_text(cx, "no graph loaded");
        self.ui.label(cx, ids!(graph_target)).set_text(cx, "");
        self.ui.label(cx, ids!(graph_counts)).set_text(cx, "");
        self.ui.label(cx, ids!(graph_layout)).set_text(cx, "");
        if let Some(mut canvas) = self
            .ui
            .widget(cx, ids!(graph_canvas))
            .borrow_mut::<StudioGraphCanvas>()
        {
            canvas.set_canvas_model(cx, StudioGraphCanvasModel::default());
        }
        self.ui.label(cx, ids!(graph_nodes)).set_text(cx, "");
        self.ui.label(cx, ids!(graph_edges)).set_text(cx, "");
        self.ui.label(cx, ids!(selected_node)).set_text(cx, "");
        self.ui.label(cx, ids!(selected_reference)).set_text(cx, "");
        self.ui
            .label(cx, ids!(selected_node_details))
            .set_text(cx, "");
        self.ui.label(cx, ids!(selected_edge)).set_text(cx, "");
        self.ui
            .label(cx, ids!(selected_edge_details))
            .set_text(cx, "");
        self.ui.label(cx, ids!(focused_issue)).set_text(cx, "");
    }

    fn sync_graph_canvas(&mut self, cx: &mut Cx, model: &StudioViewModel, graph: &StudioGraphView) {
        if let Some(mut canvas) = self
            .ui
            .widget(cx, ids!(graph_canvas))
            .borrow_mut::<StudioGraphCanvas>()
        {
            canvas.set_canvas_model(cx, graph_canvas_model(model, graph));
        }
    }

    fn select_previous_graph(&mut self, cx: &mut Cx) {
        let graph_count = self.model.as_ref().map_or(0, |model| model.graphs.len());
        if graph_count == 0 {
            return;
        }
        let next_index = if self.selected_graph_index == 0 {
            graph_count - 1
        } else {
            self.selected_graph_index - 1
        };
        self.select_graph_index(cx, next_index);
    }

    fn select_graph_index(&mut self, cx: &mut Cx, graph_index: usize) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        let Some(graph_id) = model
            .graphs
            .get(graph_index)
            .map(|graph| graph.graph_id.clone())
        else {
            return;
        };
        match load_studio_view_model_for_path(&source, Some(&graph_id), None, None, None) {
            Ok(model) => {
                self.selected_graph_index = model.selected_graph_index.unwrap_or(graph_index);
                self.selected_issue_check_id = model.selected_issue_check_id.clone();
                self.selected_node_id = model.selected_node_id.clone();
                self.selected_edge_id = model.selected_edge_id.clone();
                self.model = Some(model);
                self.sync_loaded_model(cx);
                self.ui.redraw(cx);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    fn select_next_issue(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        let Some(next_issue_check_id) = next_issue_check_id(&model).map(str::to_string) else {
            return;
        };
        let requested_graph_id = model
            .validation_issues
            .iter()
            .find(|issue| issue.check_id == next_issue_check_id)
            .and_then(|issue| issue.graph_id.as_deref())
            .or(model.selected_graph_id.as_deref());
        match load_studio_view_model_for_path(
            &source,
            requested_graph_id,
            Some(&next_issue_check_id),
            None,
            None,
        ) {
            Ok(model) => {
                self.selected_graph_index = model
                    .selected_graph_index
                    .unwrap_or(self.selected_graph_index);
                self.selected_issue_check_id = model.selected_issue_check_id.clone();
                self.selected_node_id = model.selected_node_id.clone();
                self.selected_edge_id = model.selected_edge_id.clone();
                self.model = Some(model);
                self.sync_loaded_model(cx);
                self.ui.redraw(cx);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    fn select_next_node(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        let Some(next_node_id) = next_node_id(&model).map(str::to_string) else {
            return;
        };
        match load_studio_view_model_for_path(
            &source,
            model.selected_graph_id.as_deref(),
            model.selected_issue_check_id.as_deref(),
            Some(&next_node_id),
            model.selected_edge_id.as_deref(),
        ) {
            Ok(model) => {
                self.selected_graph_index = model
                    .selected_graph_index
                    .unwrap_or(self.selected_graph_index);
                self.selected_issue_check_id = model.selected_issue_check_id.clone();
                self.selected_node_id = model.selected_node_id.clone();
                self.selected_edge_id = model.selected_edge_id.clone();
                self.model = Some(model);
                self.sync_loaded_model(cx);
                self.ui.redraw(cx);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    fn select_next_edge(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        let Some(next_edge_id) = next_edge_id(&model).map(str::to_string) else {
            return;
        };
        match load_studio_view_model_for_path(
            &source,
            model.selected_graph_id.as_deref(),
            model.selected_issue_check_id.as_deref(),
            model.selected_node_id.as_deref(),
            Some(&next_edge_id),
        ) {
            Ok(model) => {
                self.selected_graph_index = model
                    .selected_graph_index
                    .unwrap_or(self.selected_graph_index);
                self.selected_issue_check_id = model.selected_issue_check_id.clone();
                self.selected_node_id = model.selected_node_id.clone();
                self.selected_edge_id = model.selected_edge_id.clone();
                self.model = Some(model);
                self.sync_loaded_model(cx);
                self.ui.redraw(cx);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    fn select_canvas_node(&mut self, cx: &mut Cx, node_id: &str) {
        let current_edge_id = self
            .model
            .as_ref()
            .and_then(|model| model.selected_edge_id.clone());
        self.select_canvas_request(cx, Some(node_id), current_edge_id.as_deref());
    }

    fn select_canvas_edge(&mut self, cx: &mut Cx, edge_id: &str) {
        let current_node_id = self
            .model
            .as_ref()
            .and_then(|model| model.selected_node_id.clone());
        self.select_canvas_request(cx, current_node_id.as_deref(), Some(edge_id));
    }

    fn select_canvas_request(
        &mut self,
        cx: &mut Cx,
        requested_node_id: Option<&str>,
        requested_edge_id: Option<&str>,
    ) {
        let Some(source) = self.project_source.clone() else {
            return;
        };
        let Some(model) = self.model.clone() else {
            return;
        };
        match canvas_selection_view_model_for_project_source(
            &source,
            &model,
            self.selected_graph_index,
            requested_node_id,
            requested_edge_id,
        ) {
            Ok(model) => {
                self.selected_graph_index = model
                    .selected_graph_index
                    .unwrap_or(self.selected_graph_index);
                self.selected_issue_check_id = model.selected_issue_check_id.clone();
                self.selected_node_id = model.selected_node_id.clone();
                self.selected_edge_id = model.selected_edge_id.clone();
                self.model = Some(model);
                self.sync_loaded_model(cx);
                self.ui.redraw(cx);
            }
            Err(error) => {
                self.last_edit_report = None;
                self.last_edit_save_issue = error;
                self.sync_edit_report(cx);
                self.ui.redraw(cx);
            }
        }
    }

    fn select_next_graph(&mut self, cx: &mut Cx) {
        let graph_count = self.model.as_ref().map_or(0, |model| model.graphs.len());
        if graph_count == 0 {
            return;
        }
        self.select_graph_index(cx, (self.selected_graph_index + 1) % graph_count);
    }

    fn sync_error(&mut self, cx: &mut Cx, error: &str) {
        self.ui.label(cx, ids!(project_source)).set_text(cx, "");
        self.ui
            .label(cx, ids!(project_identity))
            .set_text(cx, "project load failed");
        self.ui.label(cx, ids!(project_revision)).set_text(cx, "");
        self.ui
            .label(cx, ids!(validation_status))
            .set_text(cx, error);
        self.ui.label(cx, ids!(validation_issues)).set_text(cx, "");
        self.ui.label(cx, ids!(catalog_packages)).set_text(cx, "");
        self.ui.label(cx, ids!(host_profiles)).set_text(cx, "");
        self.sync_no_graph(cx);
        self.last_edit_report = None;
        self.last_edit_save_issue.clear();
        self.sync_edit_report(cx);
    }
}

impl MatchEvent for App {
    fn handle_startup(&mut self, cx: &mut Cx) {
        self.sync_project(cx);
    }

    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let canvas = self.ui.widget(cx, ids!(graph_canvas));
        for action in canvas.filter_actions(actions) {
            match action.cast::<StudioGraphCanvasAction>() {
                StudioGraphCanvasAction::SelectNode(node_id) => {
                    self.select_canvas_node(cx, &node_id);
                }
                StudioGraphCanvasAction::SelectEdge(edge_id) => {
                    self.select_canvas_edge(cx, &edge_id);
                }
                StudioGraphCanvasAction::None => {}
            }
        }
        if self
            .ui
            .button(cx, ids!(previous_graph_button))
            .clicked(actions)
        {
            self.select_previous_graph(cx);
        }
        if self.ui.button(cx, ids!(next_graph_button)).clicked(actions) {
            self.select_next_graph(cx);
        }
        if self.ui.button(cx, ids!(next_issue_button)).clicked(actions) {
            self.select_next_issue(cx);
        }
        if self.ui.button(cx, ids!(next_node_button)).clicked(actions) {
            self.select_next_node(cx);
        }
        if self.ui.button(cx, ids!(next_edge_button)).clicked(actions) {
            self.select_next_edge(cx);
        }
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
        if self
            .ui
            .button(cx, ids!(remove_selected_module_button))
            .clicked(actions)
        {
            self.remove_selected_module_from_selected_graph(cx);
        }
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

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

fn load_studio_view_model(
    requested_graph_id: Option<&str>,
    requested_issue_check_id: Option<&str>,
    requested_node_id: Option<&str>,
    requested_edge_id: Option<&str>,
) -> Result<(PathBuf, StudioViewModel), String> {
    let project_path = project_path_from_args()
        .or_else(find_default_project_path)
        .ok_or_else(|| "no project path supplied and default example was not found".to_string())?;
    let model = load_studio_view_model_for_path(
        &project_path,
        requested_graph_id,
        requested_issue_check_id,
        requested_node_id,
        requested_edge_id,
    )?;
    Ok((project_path, model))
}

fn load_studio_view_model_for_path(
    project_path: &Path,
    requested_graph_id: Option<&str>,
    requested_issue_check_id: Option<&str>,
    requested_node_id: Option<&str>,
    requested_edge_id: Option<&str>,
) -> Result<StudioViewModel, String> {
    let project = load_project(&project_path).map_err(|error| error.to_string())?;
    Ok(view_model_for_graph_issue_node_and_edge(
        &project,
        project_path.parent(),
        requested_graph_id,
        requested_issue_check_id,
        requested_node_id,
        requested_edge_id,
    ))
}

fn canvas_selection_view_model_for_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    requested_node_id: Option<&str>,
    requested_edge_id: Option<&str>,
) -> Result<StudioViewModel, String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    load_studio_view_model_for_path(
        project_path,
        Some(&graph_id),
        model.selected_issue_check_id.as_deref(),
        requested_node_id,
        requested_edge_id,
    )
}

fn retarget_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    target_host_profile: &str,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = retarget_graph_host_profile(
        &mut project,
        &graph_id,
        target_host_profile,
        project_path.parent(),
    );
    if report.status != StudioEditStatus::Applied {
        return Ok((report, None));
    }
    save_project(project_path, &project)
        .map_err(|error| format!("Project save failed: {error}"))?;
    let refreshed_model = view_model_for_graph(&project, project_path.parent(), Some(&graph_id));
    Ok((report, Some(refreshed_model)))
}

#[cfg(test)]
fn add_module_to_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    package_reference_id: &str,
    module_reference_id: &str,
    module_label: Option<&str>,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = rusty_studio_core::add_module_to_graph(
        &mut project,
        &graph_id,
        package_reference_id,
        module_reference_id,
        module_label,
        project_path.parent(),
    );
    if report.status != StudioEditStatus::Applied {
        return Ok((report, None));
    }
    save_project(project_path, &project)
        .map_err(|error| format!("Project save failed: {error}"))?;
    let refreshed_model = view_model_for_graph(&project, project_path.parent(), Some(&graph_id));
    Ok((report, Some(refreshed_model)))
}

fn add_next_catalog_module_to_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    package_reference_id: Option<&str>,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = if let Some(package_reference_id) = package_reference_id {
        add_next_catalog_module_from_package_to_graph(
            &mut project,
            &graph_id,
            package_reference_id,
            project_path.parent(),
        )
    } else {
        add_next_catalog_module_to_graph(&mut project, &graph_id, project_path.parent())
    };
    if report.status != StudioEditStatus::Applied {
        return Ok((report, None));
    }
    save_project(project_path, &project)
        .map_err(|error| format!("Project save failed: {error}"))?;
    let refreshed_model = view_model_for_graph(&project, project_path.parent(), Some(&graph_id));
    Ok((report, Some(refreshed_model)))
}

fn remove_module_from_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    module_reference_id: &str,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = remove_module_from_graph(
        &mut project,
        &graph_id,
        module_reference_id,
        project_path.parent(),
    );
    if report.status != StudioEditStatus::Applied {
        return Ok((report, None));
    }
    save_project(project_path, &project)
        .map_err(|error| format!("Project save failed: {error}"))?;
    let refreshed_model = view_model_for_graph(&project, project_path.parent(), Some(&graph_id));
    Ok((report, Some(refreshed_model)))
}

fn add_binding_to_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    binding_kind: StudioBindingKind,
    source_node_id: &str,
    target_node_id: &str,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = add_binding_to_graph(
        &mut project,
        &graph_id,
        binding_kind,
        source_node_id,
        target_node_id,
        project_path.parent(),
    );
    if report.status != StudioEditStatus::Applied {
        return Ok((report, None));
    }
    save_project(project_path, &project)
        .map_err(|error| format!("Project save failed: {error}"))?;
    let refreshed_model = view_model_for_graph(&project, project_path.parent(), Some(&graph_id));
    Ok((report, Some(refreshed_model)))
}

fn remove_binding_from_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
    binding_kind: StudioBindingKind,
    source_node_id: &str,
    target_node_id: &str,
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = remove_binding_from_graph(
        &mut project,
        &graph_id,
        binding_kind,
        source_node_id,
        target_node_id,
        project_path.parent(),
    );
    if report.status != StudioEditStatus::Applied {
        return Ok((report, None));
    }
    save_project(project_path, &project)
        .map_err(|error| format!("Project save failed: {error}"))?;
    let refreshed_model = view_model_for_graph(&project, project_path.parent(), Some(&graph_id));
    Ok((report, Some(refreshed_model)))
}

fn selected_graph_id_for_model(
    model: &StudioViewModel,
    selected_graph_index: usize,
) -> Option<String> {
    if model.selection_issue_code.is_some() {
        return None;
    }
    model
        .graphs
        .get(selected_graph_index)
        .map(|graph| graph.graph_id.clone())
}

fn project_path_from_args() -> Option<PathBuf> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--project" {
            return args.next().map(PathBuf::from);
        }
    }
    None
}

fn initial_graph_id_from_args() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--graph" {
            return args.next();
        }
    }
    None
}

fn initial_issue_check_id_from_args() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--issue" {
            return args.next();
        }
    }
    None
}

fn initial_node_id_from_args() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--node" {
            return args.next();
        }
    }
    None
}

fn initial_edge_id_from_args() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--edge" {
            return args.next();
        }
    }
    None
}

fn find_default_project_path() -> Option<PathBuf> {
    let current_dir = std::env::current_dir().ok()?;
    let candidates = [
        current_dir.join("examples/synthetic-studio-project.json"),
        current_dir.join("../../examples/synthetic-studio-project.json"),
        current_dir.join("../../../examples/synthetic-studio-project.json"),
    ];
    candidates.into_iter().find(|path| path.is_file())
}

fn validation_line(model: &StudioViewModel) -> String {
    let status = match model.validation_status {
        StudioValidationStatus::Pass => "pass",
        StudioValidationStatus::Fail => "fail",
    };
    format!(
        "{status}; {} passing checks, {} failing checks",
        model.validation_pass_count, model.validation_fail_count
    )
}

fn validation_issue_lines(model: &StudioViewModel) -> String {
    if model.validation_issues.is_empty() {
        return "none".to_string();
    }
    model
        .validation_issues
        .iter()
        .map(|issue| {
            let issue_code = issue.issue_code.as_deref().unwrap_or("unknown_issue");
            let mut lines = vec![format!("{} [{}]", issue.check_id, issue_code)];
            if let Some(graph_id) = issue.graph_id.as_deref() {
                let graph_label = if issue.targets_selected_graph {
                    "selected graph"
                } else {
                    "graph"
                };
                lines.push(format!("  {graph_label}: {graph_id}"));
            }
            if !issue.node_ids.is_empty() {
                lines.push(format!("  nodes: {}", issue.node_ids.join(", ")));
            }
            if !issue.edge_ids.is_empty() {
                lines.push(format!("  edges: {}", issue.edge_ids.join(", ")));
            }
            if !issue.reference_ids.is_empty() {
                lines.push(format!("  refs: {}", issue.reference_ids.join(", ")));
            }
            lines.push(format!("  {}", issue.evidence));
            lines.join("\n")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn issue_focus_line(model: &StudioViewModel) -> String {
    let Some(focus) = model.focused_issue.as_ref() else {
        if let Some(issue_code) = model.issue_selection_code.as_deref() {
            return format!("none [{issue_code}]");
        }
        return "none".to_string();
    };
    let issue_code = focus.issue_code.as_deref().unwrap_or("unknown_issue");
    let mut lines = vec![format!(
        "#{} {} [{}]",
        focus.issue_index + 1,
        focus.check_id,
        issue_code
    )];
    if let Some(selection_issue_code) = model.issue_selection_code.as_deref() {
        lines.push(format!("  selection: {selection_issue_code}"));
    }
    lines.push(format!("  graph: {}", focus.graph_id));
    if let Some(node_id) = focus.node_id.as_deref() {
        lines.push(format!("  node: {node_id}"));
    }
    if let Some(edge_id) = focus.edge_id.as_deref() {
        lines.push(format!("  edge: {edge_id}"));
    }
    if let Some(reference_id) = focus.reference_id.as_deref() {
        lines.push(format!("  ref: {reference_id}"));
    }
    lines.push(format!("  {}", focus.evidence));
    lines.join("\n")
}

fn next_issue_check_id(model: &StudioViewModel) -> Option<&str> {
    if model.validation_issues.is_empty() {
        return None;
    }
    let next_index = model
        .selected_issue_index
        .map(|index| (index + 1) % model.validation_issues.len())
        .unwrap_or(0);
    model
        .validation_issues
        .get(next_index)
        .map(|issue| issue.check_id.as_str())
}

fn selected_node_line(model: &StudioViewModel) -> String {
    let Some(node) = model.selected_node.as_ref() else {
        return "none".to_string();
    };
    let is_issue_node = model
        .focused_issue
        .as_ref()
        .and_then(|focus| focus.node_id.as_deref())
        == Some(node.node_id.as_str());
    let prefix = if is_issue_node { "issue: " } else { "" };
    format!("{prefix}{} / {}", node.label, node.kind)
}

fn selected_reference_line(model: &StudioViewModel) -> String {
    model.selected_node.as_ref().map_or_else(
        || "none".to_string(),
        |node| format!("{} [{}]", node.reference_id, node.reference_status.as_str()),
    )
}

fn selected_node_detail_lines(model: &StudioViewModel) -> String {
    let Some(node) = model.selected_node.as_ref() else {
        if let Some(issue_code) = model.node_selection_code.as_deref() {
            return format!("none [{issue_code}]");
        }
        return "none".to_string();
    };
    let mut lines = Vec::new();
    if let Some(issue_code) = model.node_selection_code.as_deref() {
        lines.push(format!("selection: {issue_code}"));
    }
    lines.push(format!("graph: {}", node.graph_id));
    lines.push(format!("node: {}", node.node_id));
    lines.push(format!(
        "ref: {} [{}]",
        node.reference_id, node.reference_status
    ));
    if node.validation_issue_count > 0 {
        lines.push(format!("issues: {}", node.validation_issue_count));
    }
    if let Some(path) = node.package_manifest_path.as_deref() {
        lines.push(format!("manifest: {path}"));
    }
    if !node.package_module_ids.is_empty() {
        lines.push(format!("modules: {}", node.package_module_ids.join(", ")));
    }
    if !node.module_package_ids.is_empty() {
        lines.push(format!("packages: {}", node.module_package_ids.join(", ")));
    }
    if let Some(profile) = node.host_profile.as_ref() {
        let host = profile.host_profile.as_deref().unwrap_or("unknown host");
        let install = profile
            .install_route
            .as_deref()
            .unwrap_or("install route missing");
        let launch = profile
            .launch_route
            .as_deref()
            .unwrap_or("launch route missing");
        lines.push(format!("host: {host}"));
        lines.push(format!("routes: {install} / {launch}"));
    }
    lines.join("\n")
}

fn next_node_id(model: &StudioViewModel) -> Option<&str> {
    let selected_graph_id = model.selected_graph_id.as_deref()?;
    let graph = model
        .graphs
        .iter()
        .find(|graph| graph.graph_id == selected_graph_id)?;
    if graph.node_rows.is_empty() {
        return None;
    }
    let current_index = model
        .selected_node_id
        .as_deref()
        .and_then(|node_id| {
            graph
                .node_rows
                .iter()
                .position(|node| node.node_id == node_id)
        })
        .unwrap_or(0);
    let next_index = (current_index + 1) % graph.node_rows.len();
    graph
        .node_rows
        .get(next_index)
        .map(|node| node.node_id.as_str())
}

fn selected_edge_line(model: &StudioViewModel) -> String {
    let Some(edge) = model.selected_edge.as_ref() else {
        return "none".to_string();
    };
    let is_issue_edge = model
        .focused_issue
        .as_ref()
        .and_then(|focus| focus.edge_id.as_deref())
        == Some(edge.edge_id.as_str());
    let prefix = if is_issue_edge { "issue: " } else { "" };
    format!("{prefix}{} [{}]", edge.edge_id, edge.kind)
}

fn selected_edge_detail_lines(model: &StudioViewModel) -> String {
    let Some(edge) = model.selected_edge.as_ref() else {
        if let Some(issue_code) = model.edge_selection_code.as_deref() {
            return format!("none [{issue_code}]");
        }
        return "none".to_string();
    };
    let mut lines = Vec::new();
    if let Some(issue_code) = model.edge_selection_code.as_deref() {
        lines.push(format!("selection: {issue_code}"));
    }
    lines.push(format!("graph: {}", edge.graph_id));
    lines.push(format!("status: {}", edge.endpoint_status));
    if edge.validation_issue_count > 0 {
        lines.push(format!("issues: {}", edge.validation_issue_count));
    }
    if let Some(binding_kind) = edge.binding_kind.as_deref() {
        lines.push(format!("binding: {binding_kind}"));
    }
    lines.push(format!(
        "source: {} / {} / {}",
        edge.source_node_id,
        edge.source_kind.as_deref().unwrap_or("missing"),
        edge.source_reference_id.as_deref().unwrap_or("missing")
    ));
    lines.push(format!(
        "target: {} / {} / {}",
        edge.target_node_id,
        edge.target_kind.as_deref().unwrap_or("missing"),
        edge.target_reference_id.as_deref().unwrap_or("missing")
    ));
    lines.join("\n")
}

fn next_edge_id(model: &StudioViewModel) -> Option<&str> {
    let selected_graph_id = model.selected_graph_id.as_deref()?;
    let graph = model
        .graphs
        .iter()
        .find(|graph| graph.graph_id == selected_graph_id)?;
    if graph.edge_rows.is_empty() {
        return None;
    }
    let current_index = model
        .selected_edge_id
        .as_deref()
        .and_then(|edge_id| {
            graph
                .edge_rows
                .iter()
                .position(|edge| edge.edge_id == edge_id)
        })
        .unwrap_or(0);
    let next_index = (current_index + 1) % graph.edge_rows.len();
    graph
        .edge_rows
        .get(next_index)
        .map(|edge| edge.edge_id.as_str())
}

fn catalog_package_lines(model: &StudioViewModel) -> String {
    if model.catalog_packages.is_empty() {
        return "none".to_string();
    }
    model
        .catalog_packages
        .iter()
        .map(|package| {
            let state = if package.in_selected_graph {
                "selected"
            } else {
                "available"
            };
            let modules = if package.module_ids.is_empty() {
                "no module exports".to_string()
            } else {
                package.module_ids.join(", ")
            };
            format!(
                "{} [{}; {} module(s)]\n  {}\n  manifest: {}",
                package.package_id, state, package.module_count, modules, package.manifest_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn host_profile_lines(model: &StudioViewModel) -> String {
    if model.host_profiles.is_empty() {
        return "none".to_string();
    }
    model
        .host_profiles
        .iter()
        .map(|profile| {
            let state = if profile.targets_selected_graph {
                "target"
            } else {
                "available"
            };
            let host = profile.host_profile.as_deref().unwrap_or("unknown host");
            let install = profile
                .install_route
                .as_deref()
                .unwrap_or("install route missing");
            let launch = profile
                .launch_route
                .as_deref()
                .unwrap_or("launch route missing");
            format!(
                "{} [{}]\n  host: {}; routes: {} / {}",
                profile.profile_id, state, host, install, launch
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn edit_status_line(report: &StudioEditReport, save_issue: &str) -> String {
    let status = match report.status {
        StudioEditStatus::Applied => "applied",
        StudioEditStatus::Rejected => "rejected",
    };
    let issue = report.issue_code.as_deref().unwrap_or("none");
    if save_issue.is_empty() {
        format!(
            "{status}; rev {} -> {}; issue {issue}",
            report.original_revision, report.resulting_revision
        )
    } else {
        format!(
            "{status}; rev {} -> {}; issue {issue}; {save_issue}",
            report.original_revision, report.resulting_revision
        )
    }
}

fn changed_fields_line(report: &StudioEditReport) -> String {
    if report.changed_fields.is_empty() {
        "none".to_string()
    } else {
        report.changed_fields.join("\n")
    }
}

fn edit_validation_line(report: &StudioEditReport) -> String {
    let status = match report.validation.status {
        StudioValidationStatus::Pass => "pass",
        StudioValidationStatus::Fail => "fail",
    };
    format!("{status}; {} check(s)", report.validation.checks.len())
}

fn selected_package_reference_id(model: &StudioViewModel) -> Result<String, String> {
    let Some(node) = model.selected_node.as_ref() else {
        return Err("No node is selected".to_string());
    };
    if node.kind != "package" {
        return Err(format!(
            "Selected node {} is {}; select a package node to add a package module",
            node.node_id, node.kind
        ));
    }
    Ok(node.reference_id.clone())
}

fn selected_module_reference_id(model: &StudioViewModel) -> Result<String, String> {
    let Some(node) = model.selected_node.as_ref() else {
        return Err("No node is selected".to_string());
    };
    if node.kind != "module" {
        return Err(format!(
            "Selected node {} is {}; select a module node to remove a module",
            node.node_id, node.kind
        ));
    }
    Ok(node.reference_id.clone())
}

fn selected_command_binding_request(
    model: &StudioViewModel,
) -> Result<SelectedBindingRequest, String> {
    let Some(node) = model.selected_node.as_ref() else {
        return Err("No node is selected".to_string());
    };
    if node.kind != "module" {
        return Err(format!(
            "Selected node {} is {}; select a module node to add a command binding",
            node.node_id, node.kind
        ));
    }
    let graph = model
        .graphs
        .iter()
        .find(|graph| graph.graph_id == node.graph_id)
        .ok_or_else(|| format!("Selected graph {} is unavailable", node.graph_id))?;
    let operator_shell_nodes = graph
        .node_rows
        .iter()
        .filter(|row| row.kind == "operator_shell")
        .collect::<Vec<_>>();
    let Some(source_node) = operator_shell_nodes.first() else {
        return Err(format!(
            "Selected graph {} has no operator shell for command binding",
            graph.graph_id
        ));
    };
    if operator_shell_nodes.len() > 1 {
        return Err(format!(
            "Selected graph {} has multiple operator shells; select one shell before adding a command binding",
            graph.graph_id
        ));
    }
    Ok(SelectedBindingRequest {
        binding_kind: StudioBindingKind::Command,
        source_node_id: source_node.node_id.clone(),
        target_node_id: node.node_id.clone(),
    })
}

fn selected_binding_request(model: &StudioViewModel) -> Result<SelectedBindingRequest, String> {
    let Some(edge) = model.selected_edge.as_ref() else {
        return Err("No edge is selected".to_string());
    };
    let Some(binding_kind) = edge
        .binding_kind
        .as_deref()
        .and_then(studio_binding_kind_from_view)
    else {
        return Err(format!(
            "Selected edge {} is {}; select a stream or command binding to remove a binding",
            edge.edge_id, edge.kind
        ));
    };
    Ok(SelectedBindingRequest {
        binding_kind,
        source_node_id: edge.source_node_id.clone(),
        target_node_id: edge.target_node_id.clone(),
    })
}

fn studio_binding_kind_from_view(value: &str) -> Option<StudioBindingKind> {
    match value {
        "stream" => Some(StudioBindingKind::Stream),
        "command" => Some(StudioBindingKind::Command),
        _ => None,
    }
}

fn graph_canvas_model(model: &StudioViewModel, graph: &StudioGraphView) -> StudioGraphCanvasModel {
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

fn layout_lines(graph: &StudioGraphView) -> String {
    let Some(layout) = graph.layout.as_ref() else {
        return "none".to_string();
    };
    let mut lines = vec![format!(
        "{} / {} / {} node(s) / {} edge(s)",
        layout.layout_id, layout.coordinate_space, layout.node_count, layout.edge_count
    )];
    for node in &layout.nodes {
        lines.push(format!(
            "{} @ {},{} {}x{}{}",
            node.node_id,
            node.x,
            node.y,
            node.width,
            node.height,
            issue_count_line(node.validation_issue_count)
        ));
    }
    for edge in &layout.edges {
        lines.push(format!(
            "{} route: {}{}",
            edge.edge_id,
            edge.route,
            issue_count_line(edge.validation_issue_count)
        ));
    }
    lines.join("\n")
}

fn node_lines(graph: &StudioGraphView) -> String {
    graph
        .node_rows
        .iter()
        .map(|node| {
            format!(
                "{} [{}]\n  ref: {}{}",
                node.label,
                node.kind,
                node.reference_id,
                issue_count_line(node.validation_issue_count)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn edge_lines(graph: &StudioGraphView) -> String {
    graph
        .edge_rows
        .iter()
        .map(|edge| {
            format!(
                "{} [{}]\n  {} -> {}{}",
                edge.edge_id,
                edge.kind,
                edge.source_node_id,
                edge.target_node_id,
                issue_count_line(edge.validation_issue_count)
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn issue_count_line(count: usize) -> String {
    match count {
        0 => String::new(),
        1 => "\n  issues: 1".to_string(),
        _ => format!("\n  issues: {count}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_studio_model::{
        StudioEdge, StudioEdgeKind, StudioEdgeLayout, StudioEdgeRouteKind, StudioEditOperation,
        StudioGraph, StudioGraphLayout, StudioNode, StudioNodeKind, StudioNodeLayout,
        StudioProject, PROJECT_SCHEMA,
    };

    fn temp_root(name: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time before unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!("rusty-studio-makepad-{name}-{unique}"));
        std::fs::create_dir_all(&root).expect("create temp root");
        root
    }

    fn write_fixture(path: &Path, text: &str) {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).expect("create fixture parent");
        }
        std::fs::write(path, text).expect("write fixture");
    }

    fn write_reference_fixture_tree(root: &Path) {
        write_fixture(
            &root.join("packages/catalog.manifold.json"),
            r#"{
  "$schema": "rusty.manifold.package.catalog.v1",
  "catalog_id": "catalog.test",
  "packages": [
    {
      "package_id": "package.synthetic",
      "manifest_path": "packages/synthetic/manifests/package.manifold.json"
    }
  ]
}"#,
        );
        write_fixture(
            &root.join("packages/synthetic/manifests/package.manifold.json"),
            r#"{
  "$schema": "rusty.manifold.package.manifest.v1",
  "package_id": "package.synthetic",
  "version": "0.1.0",
  "exports": {
    "modules": [
      "module.synthetic_provider"
    ],
    "streams": [],
    "commands": []
  }
}"#,
        );
        write_fixture(
            &root.join("profiles/desktop.json"),
            r#"{
  "$schema": "rusty.manifold.host_run.install_launch_profile.v1",
  "profile_id": "host_run.profile.desktop",
  "host_profile": "host.desktop",
  "app_id": "app.host_shell.desktop",
  "install_route": "install.local_process",
  "launch_route": "launch.local_process",
  "command_bridge": "bridge.local_cli",
  "required_permissions": [],
  "evidence_pull_route": "evidence.filesystem"
}"#,
        );
        write_fixture(
            &root.join("profiles/headset.json"),
            r#"{
  "$schema": "rusty.manifold.host_run.install_launch_profile.v1",
  "profile_id": "host_run.profile.headset",
  "host_profile": "host.quest",
  "app_id": "app.host_shell.quest",
  "install_route": "install.adb_package",
  "launch_route": "launch.adb_activity",
  "command_bridge": "bridge.local_cli",
  "required_permissions": [],
  "evidence_pull_route": "evidence.filesystem"
}"#,
        );
    }

    fn editable_project() -> StudioProject {
        StudioProject {
            schema_id: PROJECT_SCHEMA.to_string(),
            project_id: "studio.project.makepad_edit".to_string(),
            revision: 1,
            display_name: "Makepad Edit".to_string(),
            package_catalog_path: "packages/catalog.manifold.json".to_string(),
            host_run_profile_paths: vec![
                "profiles/desktop.json".to_string(),
                "profiles/headset.json".to_string(),
            ],
            graphs: vec![StudioGraph {
                graph_id: "studio.graph.makepad_edit".to_string(),
                display_name: "Makepad Edit Graph".to_string(),
                target_host_profile: "host_run.profile.desktop".to_string(),
                nodes: vec![
                    StudioNode {
                        node_id: "node.package.synthetic".to_string(),
                        kind: StudioNodeKind::Package,
                        reference_id: "package.synthetic".to_string(),
                        label: "Package".to_string(),
                    },
                    StudioNode {
                        node_id: "node.host.profile".to_string(),
                        kind: StudioNodeKind::HostProfile,
                        reference_id: "host_run.profile.desktop".to_string(),
                        label: "Host".to_string(),
                    },
                    StudioNode {
                        node_id: "node.shell.operator".to_string(),
                        kind: StudioNodeKind::OperatorShell,
                        reference_id: "shell.synthetic.operator".to_string(),
                        label: "Shell".to_string(),
                    },
                ],
                edges: vec![StudioEdge {
                    edge_id: "edge.shell_host".to_string(),
                    kind: StudioEdgeKind::ShellTargetsHostProfile,
                    source_node_id: "node.shell.operator".to_string(),
                    target_node_id: "node.host.profile".to_string(),
                }],
                layout: Some(StudioGraphLayout {
                    layout_id: "studio.layout.makepad_edit".to_string(),
                    coordinate_space: "studio.canvas.logical_2d".to_string(),
                    nodes: vec![
                        StudioNodeLayout {
                            node_id: "node.package.synthetic".to_string(),
                            x: 40,
                            y: 40,
                            width: 180,
                            height: 72,
                        },
                        StudioNodeLayout {
                            node_id: "node.shell.operator".to_string(),
                            x: 320,
                            y: 40,
                            width: 180,
                            height: 72,
                        },
                        StudioNodeLayout {
                            node_id: "node.host.profile".to_string(),
                            x: 600,
                            y: 40,
                            width: 180,
                            height: 72,
                        },
                    ],
                    edges: vec![StudioEdgeLayout {
                        edge_id: "edge.shell_host".to_string(),
                        route: StudioEdgeRouteKind::Direct,
                    }],
                }),
            }],
        }
    }

    #[test]
    fn retarget_project_source_saves_and_refreshes_view_model() {
        let root = temp_root("retarget-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");

        let (report, refreshed_model) =
            retarget_project_source(&project_path, &model, 0, "host_run.profile.headset")
                .expect("retarget project source");
        let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
        let saved_project = load_project(&project_path).expect("load saved edited project");

        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert_eq!(saved_project.revision, 2);
        assert_eq!(
            saved_project.graphs[0].target_host_profile,
            "host_run.profile.headset"
        );
        assert_eq!(refreshed_model.revision, 2);
        assert_eq!(
            refreshed_model.graphs[0].target_host_profile,
            "host_run.profile.headset"
        );
    }

    #[test]
    fn palette_lines_render_catalog_and_host_profiles() {
        let root = temp_root("palette-lines");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");

        let package_lines = catalog_package_lines(&model);
        assert!(package_lines.contains("package.synthetic [selected; 1 module(s)]"));
        assert!(package_lines.contains("module.synthetic_provider"));
        assert!(package_lines.contains("packages/synthetic/manifests/package.manifold.json"));

        let profile_lines = host_profile_lines(&model);
        assert!(profile_lines.contains("host_run.profile.desktop [target]"));
        assert!(profile_lines.contains("host: host.desktop"));
        assert!(profile_lines.contains("host_run.profile.headset [available]"));

        assert_eq!(selected_node_line(&model), "Package / package");
        let detail_lines = selected_node_detail_lines(&model);
        assert!(detail_lines.contains("node: node.package.synthetic"));
        assert!(detail_lines.contains("ref: package.synthetic [resolved]"));
        assert!(detail_lines.contains("module.synthetic_provider"));
        let layout = layout_lines(&model.graphs[0]);
        assert!(layout.contains("studio.layout.makepad_edit / studio.canvas.logical_2d"));
        assert!(layout.contains("node.shell.operator @ 320,40 180x72"));
        assert!(layout.contains("edge.shell_host route: direct"));
        let canvas = graph_canvas_model(&model, &model.graphs[0]);
        assert_eq!(canvas.layout_id, "studio.layout.makepad_edit");
        assert_eq!(canvas.nodes.len(), 3);
        assert_eq!(canvas.edges.len(), 1);
        assert!(canvas
            .nodes
            .iter()
            .any(|node| node.node_id == "node.package.synthetic" && node.selected));
        assert!(canvas
            .edges
            .iter()
            .any(|edge| edge.edge_id == "edge.shell_host" && edge.selected));
        let canvas_rect = Rect {
            pos: dvec2(0.0, 0.0),
            size: dvec2(840.0, 220.0),
        };
        let canvas_viewport = CanvasViewport::for_rect(
            canvas_rect,
            canvas.logical_bounds().expect("canvas logical bounds"),
        );
        let host_node = canvas
            .nodes
            .iter()
            .find(|node| node.node_id == "node.host.profile")
            .expect("host node");
        let shell_node = canvas
            .nodes
            .iter()
            .find(|node| node.node_id == "node.shell.operator")
            .expect("shell node");
        assert_eq!(
            canvas.hit_test_abs(canvas_rect, canvas_viewport.node_center(host_node)),
            Some(StudioGraphCanvasHit::Node("node.host.profile".to_string()))
        );
        let shell_center = canvas_viewport.node_center(shell_node);
        let host_center = canvas_viewport.node_center(host_node);
        let edge_midpoint = dvec2(
            (shell_center.x + host_center.x) * 0.5,
            (shell_center.y + host_center.y) * 0.5,
        );
        assert_eq!(
            canvas.hit_test_abs(canvas_rect, edge_midpoint),
            Some(StudioGraphCanvasHit::Edge("edge.shell_host".to_string()))
        );
        assert_eq!(
            selected_edge_line(&model),
            "edge.shell_host [shell_targets_host_profile]"
        );
        let edge_details = selected_edge_detail_lines(&model);
        assert!(edge_details.contains("status: endpoints_resolved"));
        assert!(edge_details.contains("source: node.shell.operator / operator_shell"));
        assert!(edge_details.contains("target: node.host.profile / host_profile"));
    }

    #[test]
    fn canvas_selection_uses_shared_view_model_route() {
        let root = temp_root("canvas-selection-route");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");

        let selected_node_model = canvas_selection_view_model_for_project_source(
            &project_path,
            &model,
            0,
            Some("node.host.profile"),
            model.selected_edge_id.as_deref(),
        )
        .expect("select host node through shared view model");
        assert_eq!(
            selected_node_model.requested_node_id.as_deref(),
            Some("node.host.profile")
        );
        assert_eq!(
            selected_node_model.selected_node_id.as_deref(),
            Some("node.host.profile")
        );
        assert_eq!(
            selected_node_model.selected_edge_id.as_deref(),
            Some("edge.shell_host")
        );

        let selected_edge_model = canvas_selection_view_model_for_project_source(
            &project_path,
            &selected_node_model,
            0,
            selected_node_model.selected_node_id.as_deref(),
            Some("edge.shell_host"),
        )
        .expect("select edge through shared view model");
        assert_eq!(
            selected_edge_model.requested_edge_id.as_deref(),
            Some("edge.shell_host")
        );
        assert_eq!(
            selected_edge_model.selected_node_id.as_deref(),
            Some("node.host.profile")
        );
        assert_eq!(
            selected_edge_model.selected_edge_id.as_deref(),
            Some("edge.shell_host")
        );
    }

    #[test]
    fn validation_issue_lines_render_failed_checks() {
        let root = temp_root("validation-issue-lines");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        let mut project = editable_project();
        project.graphs[0].nodes[0].reference_id = "package.missing".to_string();
        project.graphs[0].nodes.push(StudioNode {
            node_id: "node.module.missing".to_string(),
            kind: StudioNodeKind::Module,
            reference_id: "module.missing".to_string(),
            label: "Missing Module".to_string(),
        });
        save_project(&project_path, &project).expect("save invalid project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");

        let issue_lines = validation_issue_lines(&model);
        assert!(issue_lines.contains("studio.check.graph.studio.graph.makepad_edit.package_refs"));
        assert!(issue_lines.contains("studio.check.graph.studio.graph.makepad_edit.module_refs"));
        assert!(issue_lines.contains("studio.issue.package_reference_missing"));
        assert!(issue_lines.contains("selected graph: studio.graph.makepad_edit"));
        assert!(issue_lines.contains("refs: package.missing"));
        assert!(issue_lines.contains("package references missing from catalog"));

        let focus_line = issue_focus_line(&model);
        assert!(focus_line.contains("studio.check.graph.studio.graph.makepad_edit.package_refs"));
        assert!(focus_line.contains("node: node.package.synthetic"));
        assert!(focus_line.contains("ref: package.missing"));
        assert_eq!(selected_node_line(&model), "issue: Package / package");
        let detail_lines = selected_node_detail_lines(&model);
        assert!(detail_lines.contains("ref: package.missing [missing]"));
        assert!(detail_lines.contains("issues: 1"));

        let node_lines = node_lines(&model.graphs[0]);
        assert!(node_lines.contains("Package [package]"));
        assert!(node_lines.contains("issues: 1"));

        assert_eq!(
            next_issue_check_id(&model),
            Some("studio.check.graph.studio.graph.makepad_edit.module_refs")
        );

        let requested_model = load_studio_view_model_for_path(
            &project_path,
            Some("studio.graph.makepad_edit"),
            Some("studio.check.graph.studio.graph.makepad_edit.module_refs"),
            None,
            None,
        )
        .expect("load requested issue view model");
        assert_eq!(requested_model.selected_issue_index, Some(1));
        let requested_focus_line = issue_focus_line(&requested_model);
        assert!(requested_focus_line
            .contains("#2 studio.check.graph.studio.graph.makepad_edit.module_refs"));
        assert!(requested_focus_line.contains("node: node.module.missing"));
        assert_eq!(
            selected_node_line(&requested_model),
            "issue: Missing Module / module"
        );
        assert_eq!(
            next_issue_check_id(&requested_model),
            Some("studio.check.graph.studio.graph.makepad_edit.package_refs")
        );

        let requested_node_model = load_studio_view_model_for_path(
            &project_path,
            Some("studio.graph.makepad_edit"),
            None,
            Some("node.host.profile"),
            None,
        )
        .expect("load requested node view model");
        let requested_node_details = selected_node_detail_lines(&requested_node_model);
        assert_eq!(
            selected_node_line(&requested_node_model),
            "Host / host_profile"
        );
        assert!(requested_node_details.contains("host: host.desktop"));
        assert!(
            requested_node_details.contains("routes: install.local_process / launch.local_process")
        );
        assert_eq!(
            next_node_id(&requested_node_model),
            Some("node.shell.operator")
        );

        let requested_edge_model = load_studio_view_model_for_path(
            &project_path,
            Some("studio.graph.makepad_edit"),
            None,
            None,
            Some("edge.shell_host"),
        )
        .expect("load requested edge view model");
        assert_eq!(
            selected_edge_line(&requested_edge_model),
            "edge.shell_host [shell_targets_host_profile]"
        );
        let requested_edge_details = selected_edge_detail_lines(&requested_edge_model);
        assert!(requested_edge_details.contains("status: endpoints_resolved"));
        assert!(requested_edge_details.contains("source: node.shell.operator / operator_shell"));
        assert!(requested_edge_details.contains("target: node.host.profile / host_profile"));
        assert_eq!(next_edge_id(&requested_edge_model), Some("edge.shell_host"));

        let mut generated_layout_model = requested_edge_model.clone();
        generated_layout_model.graphs[0].layout = None;
        let generated_canvas =
            graph_canvas_model(&generated_layout_model, &generated_layout_model.graphs[0]);
        assert_eq!(
            generated_canvas.layout_id,
            "studio.layout.generated_readonly"
        );
        assert_eq!(
            generated_canvas.coordinate_space,
            "studio.canvas.generated_2d"
        );
        assert_eq!(generated_canvas.nodes[0].x, 40);
        assert_eq!(generated_canvas.nodes[0].width, 220);
    }

    #[test]
    fn add_next_catalog_module_to_project_source_saves_and_refreshes_view_model() {
        let root = temp_root("add-next-palette-module-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");

        let (report, refreshed_model) =
            add_next_catalog_module_to_project_source(&project_path, &model, 0, None)
                .expect("add next palette module to project source");
        let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
        let saved_project = load_project(&project_path).expect("load saved edited project");

        assert_eq!(report.operation, StudioEditOperation::AddModule);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.requested_reference_id, "module.synthetic_provider");
        assert_eq!(saved_project.revision, 2);
        assert!(saved_project.graphs[0].nodes.iter().any(|node| {
            node.kind == StudioNodeKind::Module && node.reference_id == "module.synthetic_provider"
        }));
        assert_eq!(refreshed_model.revision, 2);
        assert_eq!(refreshed_model.graphs[0].module_count, 1);
    }

    #[test]
    fn selected_package_drives_add_palette_module_request() {
        let root = temp_root("selected-package-palette-module-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(
            &project_path,
            Some("studio.graph.makepad_edit"),
            None,
            Some("node.package.synthetic"),
            None,
        )
        .expect("load selected package view model");

        let package_reference_id =
            selected_package_reference_id(&model).expect("selected package reference");
        let (report, refreshed_model) = add_next_catalog_module_to_project_source(
            &project_path,
            &model,
            0,
            Some(&package_reference_id),
        )
        .expect("add selected package module to project source");
        let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
        let saved_project = load_project(&project_path).expect("load saved edited project");

        assert_eq!(package_reference_id, "package.synthetic");
        assert_eq!(report.operation, StudioEditOperation::AddModule);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.requested_reference_id, "module.synthetic_provider");
        assert!(saved_project.graphs[0].nodes.iter().any(|node| {
            node.kind == StudioNodeKind::Module && node.reference_id == "module.synthetic_provider"
        }));
        assert_eq!(refreshed_model.graphs[0].module_count, 1);
    }

    #[test]
    fn add_module_to_project_source_saves_and_refreshes_view_model() {
        let root = temp_root("add-module-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");

        let (report, refreshed_model) = add_module_to_project_source(
            &project_path,
            &model,
            0,
            "package.synthetic",
            "module.synthetic_provider",
            Some("Synthetic Provider"),
        )
        .expect("add module to project source");
        let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
        let saved_project = load_project(&project_path).expect("load saved edited project");

        assert_eq!(report.operation, StudioEditOperation::AddModule);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert_eq!(saved_project.revision, 2);
        assert!(saved_project.graphs[0].nodes.iter().any(|node| {
            node.kind == StudioNodeKind::Module && node.reference_id == "module.synthetic_provider"
        }));
        assert!(saved_project.graphs[0].edges.iter().any(|edge| {
            edge.kind == StudioEdgeKind::PackageProvidesModule
                && edge.target_node_id == "node.module.synthetic_provider"
        }));
        assert_eq!(refreshed_model.revision, 2);
        assert_eq!(refreshed_model.graphs[0].module_count, 1);
    }

    #[test]
    fn remove_module_from_project_source_saves_and_refreshes_view_model() {
        let root = temp_root("remove-module-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        let mut project = editable_project();
        project.graphs[0].nodes.push(StudioNode {
            node_id: "node.module.synthetic_provider".to_string(),
            kind: StudioNodeKind::Module,
            reference_id: "module.synthetic_provider".to_string(),
            label: "Synthetic Provider".to_string(),
        });
        project.graphs[0].edges.push(StudioEdge {
            edge_id: "edge.package_module".to_string(),
            kind: StudioEdgeKind::PackageProvidesModule,
            source_node_id: "node.package.synthetic".to_string(),
            target_node_id: "node.module.synthetic_provider".to_string(),
        });
        save_project(&project_path, &project).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");

        let (report, refreshed_model) = remove_module_from_project_source(
            &project_path,
            &model,
            0,
            "module.synthetic_provider",
        )
        .expect("remove module from project source");
        let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
        let saved_project = load_project(&project_path).expect("load saved edited project");

        assert_eq!(report.operation, StudioEditOperation::RemoveModule);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert_eq!(saved_project.revision, 2);
        assert!(!saved_project.graphs[0].nodes.iter().any(|node| {
            node.kind == StudioNodeKind::Module && node.reference_id == "module.synthetic_provider"
        }));
        assert!(!saved_project.graphs[0].edges.iter().any(|edge| {
            edge.source_node_id == "node.module.synthetic_provider"
                || edge.target_node_id == "node.module.synthetic_provider"
        }));
        assert_eq!(refreshed_model.revision, 2);
        assert_eq!(refreshed_model.graphs[0].module_count, 0);
    }

    #[test]
    fn selected_module_reference_drives_remove_module_request() {
        let root = temp_root("selected-module-remove-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        let mut project = editable_project();
        project.graphs[0].nodes.push(StudioNode {
            node_id: "node.module.synthetic_provider".to_string(),
            kind: StudioNodeKind::Module,
            reference_id: "module.synthetic_provider".to_string(),
            label: "Synthetic Provider".to_string(),
        });
        project.graphs[0].edges.push(StudioEdge {
            edge_id: "edge.package_module".to_string(),
            kind: StudioEdgeKind::PackageProvidesModule,
            source_node_id: "node.package.synthetic".to_string(),
            target_node_id: "node.module.synthetic_provider".to_string(),
        });
        save_project(&project_path, &project).expect("save editable project");
        let model = load_studio_view_model_for_path(
            &project_path,
            Some("studio.graph.makepad_edit"),
            None,
            Some("node.module.synthetic_provider"),
            None,
        )
        .expect("load selected module view model");

        let module_reference_id =
            selected_module_reference_id(&model).expect("selected module reference");
        assert_eq!(module_reference_id, "module.synthetic_provider");
        let (report, refreshed_model) =
            remove_module_from_project_source(&project_path, &model, 0, &module_reference_id)
                .expect("remove selected module from project source");
        let refreshed_model = refreshed_model.expect("refreshed model after applied edit");

        assert_eq!(report.operation, StudioEditOperation::RemoveModule);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.requested_reference_id, "module.synthetic_provider");
        assert_eq!(refreshed_model.graphs[0].module_count, 0);
    }

    #[test]
    fn add_binding_to_project_source_saves_and_refreshes_view_model() {
        let root = temp_root("add-binding-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        let mut project = editable_project();
        project.graphs[0].nodes.push(StudioNode {
            node_id: "node.module.synthetic_provider".to_string(),
            kind: StudioNodeKind::Module,
            reference_id: "module.synthetic_provider".to_string(),
            label: "Synthetic Provider".to_string(),
        });
        project.graphs[0].edges.push(StudioEdge {
            edge_id: "edge.package_module".to_string(),
            kind: StudioEdgeKind::PackageProvidesModule,
            source_node_id: "node.package.synthetic".to_string(),
            target_node_id: "node.module.synthetic_provider".to_string(),
        });
        save_project(&project_path, &project).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");

        let (report, refreshed_model) = add_binding_to_project_source(
            &project_path,
            &model,
            0,
            StudioBindingKind::Command,
            "node.shell.operator",
            "node.module.synthetic_provider",
        )
        .expect("add binding to project source");
        let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
        let saved_project = load_project(&project_path).expect("load saved edited project");

        assert_eq!(report.operation, StudioEditOperation::AddBinding);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert_eq!(saved_project.revision, 2);
        assert!(saved_project.graphs[0].edges.iter().any(|edge| {
            edge.kind == StudioEdgeKind::CommandBinding
                && edge.source_node_id == "node.shell.operator"
                && edge.target_node_id == "node.module.synthetic_provider"
        }));
        assert_eq!(refreshed_model.revision, 2);
        assert_eq!(refreshed_model.graphs[0].edge_count, 3);
    }

    #[test]
    fn selected_module_drives_add_command_binding_request() {
        let root = temp_root("selected-command-binding-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        let mut project = editable_project();
        project.graphs[0].nodes.push(StudioNode {
            node_id: "node.module.synthetic_provider".to_string(),
            kind: StudioNodeKind::Module,
            reference_id: "module.synthetic_provider".to_string(),
            label: "Synthetic Provider".to_string(),
        });
        project.graphs[0].edges.push(StudioEdge {
            edge_id: "edge.package_module".to_string(),
            kind: StudioEdgeKind::PackageProvidesModule,
            source_node_id: "node.package.synthetic".to_string(),
            target_node_id: "node.module.synthetic_provider".to_string(),
        });
        save_project(&project_path, &project).expect("save editable project");
        let model = load_studio_view_model_for_path(
            &project_path,
            Some("studio.graph.makepad_edit"),
            None,
            Some("node.module.synthetic_provider"),
            None,
        )
        .expect("load selected module view model");

        let request =
            selected_command_binding_request(&model).expect("selected command binding request");
        assert_eq!(request.binding_kind, StudioBindingKind::Command);
        assert_eq!(request.source_node_id, "node.shell.operator");
        assert_eq!(request.target_node_id, "node.module.synthetic_provider");
        let (report, refreshed_model) = add_binding_to_project_source(
            &project_path,
            &model,
            0,
            request.binding_kind,
            &request.source_node_id,
            &request.target_node_id,
        )
        .expect("add selected command binding to project source");
        let refreshed_model = refreshed_model.expect("refreshed model after applied edit");

        assert_eq!(report.operation, StudioEditOperation::AddBinding);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(
            report.requested_reference_id,
            "edge.command_binding.node.shell.operator.node.module.synthetic_provider"
        );
        assert_eq!(refreshed_model.graphs[0].edge_count, 3);
    }

    #[test]
    fn remove_binding_from_project_source_saves_and_refreshes_view_model() {
        let root = temp_root("remove-binding-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        let mut project = editable_project();
        project.graphs[0].nodes.push(StudioNode {
            node_id: "node.module.synthetic_provider".to_string(),
            kind: StudioNodeKind::Module,
            reference_id: "module.synthetic_provider".to_string(),
            label: "Synthetic Provider".to_string(),
        });
        project.graphs[0].edges.push(StudioEdge {
            edge_id: "edge.package_module".to_string(),
            kind: StudioEdgeKind::PackageProvidesModule,
            source_node_id: "node.package.synthetic".to_string(),
            target_node_id: "node.module.synthetic_provider".to_string(),
        });
        project.graphs[0].edges.push(StudioEdge {
            edge_id: "edge.shell_command".to_string(),
            kind: StudioEdgeKind::CommandBinding,
            source_node_id: "node.shell.operator".to_string(),
            target_node_id: "node.module.synthetic_provider".to_string(),
        });
        save_project(&project_path, &project).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");

        let (report, refreshed_model) = remove_binding_from_project_source(
            &project_path,
            &model,
            0,
            StudioBindingKind::Command,
            "node.shell.operator",
            "node.module.synthetic_provider",
        )
        .expect("remove binding from project source");
        let refreshed_model = refreshed_model.expect("refreshed model after applied edit");
        let saved_project = load_project(&project_path).expect("load saved edited project");

        assert_eq!(report.operation, StudioEditOperation::RemoveBinding);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(report.original_revision, 1);
        assert_eq!(report.resulting_revision, 2);
        assert_eq!(saved_project.revision, 2);
        assert!(!saved_project.graphs[0].edges.iter().any(|edge| {
            edge.kind == StudioEdgeKind::CommandBinding
                && edge.source_node_id == "node.shell.operator"
                && edge.target_node_id == "node.module.synthetic_provider"
        }));
        assert_eq!(refreshed_model.revision, 2);
        assert_eq!(refreshed_model.graphs[0].edge_count, 2);
    }

    #[test]
    fn selected_binding_drives_remove_binding_request() {
        let root = temp_root("selected-binding-remove-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        let mut project = editable_project();
        project.graphs[0].nodes.push(StudioNode {
            node_id: "node.module.synthetic_provider".to_string(),
            kind: StudioNodeKind::Module,
            reference_id: "module.synthetic_provider".to_string(),
            label: "Synthetic Provider".to_string(),
        });
        project.graphs[0].edges.push(StudioEdge {
            edge_id: "edge.package_module".to_string(),
            kind: StudioEdgeKind::PackageProvidesModule,
            source_node_id: "node.package.synthetic".to_string(),
            target_node_id: "node.module.synthetic_provider".to_string(),
        });
        project.graphs[0].edges.push(StudioEdge {
            edge_id: "edge.shell_command".to_string(),
            kind: StudioEdgeKind::CommandBinding,
            source_node_id: "node.shell.operator".to_string(),
            target_node_id: "node.module.synthetic_provider".to_string(),
        });
        save_project(&project_path, &project).expect("save editable project");
        let model = load_studio_view_model_for_path(
            &project_path,
            Some("studio.graph.makepad_edit"),
            None,
            None,
            Some("edge.shell_command"),
        )
        .expect("load selected binding view model");

        let request = selected_binding_request(&model).expect("selected binding request");
        assert_eq!(request.binding_kind, StudioBindingKind::Command);
        assert_eq!(request.source_node_id, "node.shell.operator");
        assert_eq!(request.target_node_id, "node.module.synthetic_provider");
        let (report, refreshed_model) = remove_binding_from_project_source(
            &project_path,
            &model,
            0,
            request.binding_kind,
            &request.source_node_id,
            &request.target_node_id,
        )
        .expect("remove selected binding from project source");
        let refreshed_model = refreshed_model.expect("refreshed model after applied edit");

        assert_eq!(report.operation, StudioEditOperation::RemoveBinding);
        assert_eq!(report.status, StudioEditStatus::Applied);
        assert_eq!(
            report.requested_reference_id,
            "edge.command_binding.node.shell.operator.node.module.synthetic_provider"
        );
        assert_eq!(refreshed_model.graphs[0].edge_count, 2);
    }
}
