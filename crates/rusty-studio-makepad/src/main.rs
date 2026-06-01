pub use makepad_widgets;

use makepad_widgets::*;
use rusty_studio_core::{
    add_binding_to_graph, add_next_catalog_module_from_package_to_graph,
    add_next_catalog_module_to_graph, append_shell_export_package_baseline_index_manifests,
    append_shell_handoff_acceptance_baseline_index_manifests,
    append_shell_release_candidate_review_index_manifests,
    compare_shell_export_packages_against_baseline_index_entry,
    compare_shell_handoff_acceptance_against_baseline_index_entry, load_project,
    load_shell_export_package_baseline_index, load_shell_export_package_baseline_manifest,
    load_shell_export_package_report, load_shell_handoff_acceptance_baseline_index,
    load_shell_handoff_acceptance_baseline_manifest, load_shell_handoff_acceptance_checklist,
    load_shell_handoff_manifest, load_shell_hostess_handoff_package_report,
    load_shell_hostess_owner_intake_report, load_shell_release_candidate_review_index,
    load_shell_release_candidate_review_manifest,
    promote_shell_export_package_baseline_index_default,
    promote_shell_handoff_acceptance_baseline_index_default,
    promote_shell_release_candidate_review_index_default, remove_binding_from_graph,
    remove_module_from_graph, retarget_graph_host_profile, save_json, save_project,
    save_shell_bundle, select_shell_export_package_baseline_index_entry,
    select_shell_handoff_acceptance_baseline_index_entry, selected_shell_bundle_for_graph,
    shell_export_package_baseline_index_for_manifests,
    shell_export_package_baseline_manifest_for_report, shell_export_package_for_project,
    shell_handoff_acceptance_baseline_index_for_manifests,
    shell_handoff_acceptance_baseline_manifest_for_checklist,
    shell_handoff_acceptance_checklist_for_project, shell_handoff_for_bundle,
    shell_handoff_manifest_for_project, shell_handoff_readiness_for_project,
    shell_hostess_handoff_package_for_release_candidate_index,
    shell_hostess_owner_intake_for_handoff_package, shell_hostess_staging_preview_for_owner_intake,
    shell_release_candidate_review_for_manifest,
    shell_release_candidate_review_index_for_manifests,
    shell_release_candidate_review_manifest_for_report, shell_runbook_for_project,
    summarize_shell_export_package_baseline_index_selection,
    summarize_shell_handoff_acceptance_baseline_index_selection,
    summarize_shell_release_candidate_review_index_selection, validate_selected_shell_bundle,
    view_model_for_graph, view_model_for_graph_issue_node_and_edge,
};
use rusty_studio_model::{
    StudioBindingKind, StudioEditReport, StudioEditStatus, StudioGraphView,
    StudioShellBundleReport, StudioShellBundleStatus, StudioShellBundleValidationReport,
    StudioShellDescriptorStatus, StudioShellExportPackageBaselineIndex,
    StudioShellExportPackageBaselineManifest, StudioShellExportPackageBaselineSelectionReport,
    StudioShellExportPackageBaselineSelectionStatus, StudioShellExportPackageComparisonChange,
    StudioShellExportPackageComparisonReport, StudioShellExportPackageComparisonStatus,
    StudioShellExportPackageReport, StudioShellExportPackageStatus,
    StudioShellHandoffAcceptanceBaselineIndex, StudioShellHandoffAcceptanceBaselineManifest,
    StudioShellHandoffAcceptanceBaselineSelectionReport,
    StudioShellHandoffAcceptanceBaselineSelectionStatus,
    StudioShellHandoffAcceptanceChecklistReport, StudioShellHandoffAcceptanceComparisonChange,
    StudioShellHandoffAcceptanceComparisonReport, StudioShellHandoffAcceptanceComparisonStatus,
    StudioShellHandoffAcceptanceStatus, StudioShellHandoffManifest,
    StudioShellHandoffReadinessReport, StudioShellHandoffReport,
    StudioShellHostessHandoffPackageActionStatus, StudioShellHostessHandoffPackageReport,
    StudioShellHostessHandoffPackageStatus, StudioShellHostessOwnerIntakeAssignmentStatus,
    StudioShellHostessOwnerIntakeReport, StudioShellHostessOwnerIntakeStatus,
    StudioShellHostessStagingPreviewGroupStatus, StudioShellHostessStagingPreviewManifest,
    StudioShellHostessStagingPreviewStatus, StudioShellReleaseCandidateReviewIndex,
    StudioShellReleaseCandidateReviewManifest, StudioShellReleaseCandidateReviewReport,
    StudioShellReleaseCandidateReviewSelectionReport,
    StudioShellReleaseCandidateReviewSelectionStatus, StudioShellReleaseCandidateReviewStatus,
    StudioShellRunbookReport, StudioShellRunbookStatus, StudioShellTargetKind,
    StudioValidationStatus, StudioViewModel,
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

    let ShellPreviewPanel = Panel{
        SectionTitle{text: "Shell Preview"}
        ButtonRow{
            export_shell_bundle_button := ActionButton{text: "Export Preview Files"}
            verify_shell_bundle_button := ActionButton{text: "Verify Preview Files"}
            shell_handoff_button := ActionButton{text: "Prepare Operator Shell"}
            shell_readiness_button := ActionButton{text: "Inspect All Handoffs"}
            shell_runbook_button := ActionButton{text: "Inspect Runbook"}
            shell_export_package_button := ActionButton{text: "Review Export Package"}
            shell_export_package_baseline_button := ActionButton{text: "Write Package Baseline"}
            shell_export_package_baseline_append_button := ActionButton{text: "Archive Package Baseline"}
            shell_export_package_baseline_summary_button := ActionButton{text: "Inspect Package Baseline"}
            shell_export_package_baseline_next_button := ActionButton{text: "Next Package Baseline"}
            shell_export_package_baseline_promote_button := ActionButton{text: "Promote Package Baseline"}
            shell_export_package_compare_button := ActionButton{text: "Compare Package"}
            shell_manifest_button := ActionButton{text: "Write Handoff Manifest"}
            shell_acceptance_button := ActionButton{text: "Review Acceptance"}
            shell_acceptance_baseline_button := ActionButton{text: "Write Baseline"}
            shell_acceptance_baseline_append_button := ActionButton{text: "Archive Baseline"}
            shell_acceptance_baseline_summary_button := ActionButton{text: "Inspect Baseline"}
            shell_acceptance_baseline_next_button := ActionButton{text: "Next Baseline"}
            shell_acceptance_baseline_promote_button := ActionButton{text: "Promote Baseline"}
            shell_acceptance_compare_button := ActionButton{text: "Compare Acceptance"}
            shell_release_candidate_button := ActionButton{text: "Review Release Candidate"}
            shell_release_candidate_manifest_button := ActionButton{text: "Write Candidate"}
            shell_release_candidate_append_button := ActionButton{text: "Archive Candidate"}
            shell_release_candidate_summary_button := ActionButton{text: "Inspect Candidate"}
            shell_release_candidate_next_button := ActionButton{text: "Next Candidate"}
            shell_release_candidate_promote_button := ActionButton{text: "Promote Candidate"}
            shell_hostess_handoff_package_button := ActionButton{text: "Review Hostess Package"}
            shell_hostess_owner_intake_button := ActionButton{text: "Review Hostess Intake"}
            shell_hostess_staging_preview_button := ActionButton{text: "Preview Hostess Staging"}
        }
        Row{FieldLabel{text: "descriptor"} shell_preview := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "routes"} shell_routes := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "template"} shell_template := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "bundle"} shell_bundle_status := SmallValue{text: ""}}
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
                        ShellPreviewPanel{}
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
    #[rust]
    last_shell_bundle_status: String,
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
            .label(cx, ids!(shell_preview))
            .set_text(cx, &shell_preview_lines(model));
        self.ui
            .label(cx, ids!(shell_routes))
            .set_text(cx, &shell_route_lines(model));
        self.ui
            .label(cx, ids!(shell_template))
            .set_text(cx, &shell_template_lines(model));
        self.ui.label(cx, ids!(shell_bundle_status)).set_text(
            cx,
            &shell_bundle_status_line(&self.last_shell_bundle_status),
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

    fn export_shell_bundle_for_selected_graph(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_shell_bundle_status = "No view model is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match export_shell_bundle_for_project_source(&source, &model, self.selected_graph_index) {
            Ok((report, output_dir)) => {
                self.last_shell_bundle_status = shell_bundle_export_status(&report, &output_dir);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn verify_shell_bundle_for_selected_graph(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_shell_bundle_status = "No view model is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match validate_shell_bundle_for_project_source(&source, &model, self.selected_graph_index) {
            Ok((report, output_dir)) => {
                self.last_shell_bundle_status =
                    shell_bundle_validation_status(&report, &output_dir);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn prepare_shell_handoff_for_selected_graph(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        let Some(model) = self.model.clone() else {
            self.last_shell_bundle_status = "No view model is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_handoff_for_project_source(&source, &model, self.selected_graph_index) {
            Ok((report, output_dir)) => {
                self.last_shell_bundle_status = shell_handoff_status(&report, &output_dir);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn inspect_shell_handoff_readiness(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_handoff_readiness_for_project_source(&source) {
            Ok((report, bundle_root)) => {
                self.last_shell_bundle_status =
                    shell_handoff_readiness_status(&report, &bundle_root);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn inspect_shell_runbook(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_runbook_for_project_source(&source) {
            Ok((report, bundle_root)) => {
                self.last_shell_bundle_status = shell_runbook_status(&report, &bundle_root);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn review_shell_export_package(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_export_package_for_project_source(&source) {
            Ok((report, bundle_root)) => {
                self.last_shell_bundle_status = shell_export_package_status(&report, &bundle_root);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn write_shell_export_package_baseline(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match write_shell_export_package_baseline_for_project_source(&source) {
            Ok((report, baseline, index, package_path, baseline_path, index_path, bundle_root)) => {
                self.last_shell_bundle_status = shell_export_package_baseline_status(
                    &report,
                    &baseline,
                    &index,
                    &package_path,
                    &baseline_path,
                    &index_path,
                    &bundle_root,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn append_shell_export_package_baseline(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match append_shell_export_package_baseline_for_project_source(&source) {
            Ok((report, baseline, index, package_path, baseline_path, index_path, bundle_root)) => {
                self.last_shell_bundle_status = shell_export_package_baseline_append_status(
                    &report,
                    &baseline,
                    &index,
                    &package_path,
                    &baseline_path,
                    &index_path,
                    &bundle_root,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn inspect_shell_export_package_baseline(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_export_package_baseline_summary_for_project_source(&source) {
            Ok((baseline, index, baseline_path, index_path)) => {
                self.last_shell_bundle_status = shell_export_package_baseline_summary_status(
                    &baseline,
                    &index,
                    &baseline_path,
                    &index_path,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn promote_shell_export_package_baseline_default(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match promote_shell_export_package_baseline_default_for_project_source(&source) {
            Ok((baseline, index, baseline_path, index_path)) => {
                self.last_shell_bundle_status = shell_export_package_baseline_promote_status(
                    &baseline,
                    &index,
                    &baseline_path,
                    &index_path,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn select_next_shell_export_package_baseline_default(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match select_next_shell_export_package_baseline_default_for_project_source(&source) {
            Ok((baseline, index, baseline_path, index_path)) => {
                self.last_shell_bundle_status = shell_export_package_baseline_select_status(
                    &baseline,
                    &index,
                    &baseline_path,
                    &index_path,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn compare_shell_export_package(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_export_package_comparison_for_project_source(&source) {
            Ok((report, baseline_path, bundle_root)) => {
                self.last_shell_bundle_status =
                    shell_export_package_comparison_status(&report, &baseline_path, &bundle_root);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn write_shell_handoff_manifest(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match write_shell_handoff_manifest_for_project_source(&source) {
            Ok((manifest, output_path)) => {
                self.last_shell_bundle_status =
                    shell_handoff_manifest_status(&manifest, &output_path);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn review_shell_handoff_acceptance(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_handoff_acceptance_for_project_source(&source) {
            Ok((report, bundle_root)) => {
                self.last_shell_bundle_status =
                    shell_handoff_acceptance_status(&report, &bundle_root);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn write_shell_handoff_acceptance_baseline(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match write_shell_handoff_acceptance_baseline_for_project_source(&source) {
            Ok((
                report,
                baseline,
                index,
                checklist_path,
                baseline_path,
                index_path,
                bundle_root,
            )) => {
                self.last_shell_bundle_status = shell_handoff_acceptance_baseline_status(
                    &report,
                    &baseline,
                    &index,
                    &checklist_path,
                    &baseline_path,
                    &index_path,
                    &bundle_root,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn append_shell_handoff_acceptance_baseline(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match append_shell_handoff_acceptance_baseline_for_project_source(&source) {
            Ok((
                report,
                baseline,
                index,
                checklist_path,
                baseline_path,
                index_path,
                bundle_root,
            )) => {
                self.last_shell_bundle_status = shell_handoff_acceptance_baseline_append_status(
                    &report,
                    &baseline,
                    &index,
                    &checklist_path,
                    &baseline_path,
                    &index_path,
                    &bundle_root,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn inspect_shell_handoff_acceptance_baseline(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_handoff_acceptance_baseline_summary_for_project_source(&source) {
            Ok((baseline, index, baseline_path, index_path)) => {
                self.last_shell_bundle_status = shell_handoff_acceptance_summary_status(
                    &baseline,
                    &index,
                    &baseline_path,
                    &index_path,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn promote_shell_handoff_acceptance_baseline_default(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match promote_shell_handoff_acceptance_baseline_default_for_project_source(&source) {
            Ok((baseline, index, baseline_path, index_path)) => {
                self.last_shell_bundle_status = shell_handoff_acceptance_baseline_promote_status(
                    &baseline,
                    &index,
                    &baseline_path,
                    &index_path,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn select_next_shell_handoff_acceptance_baseline_default(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match select_next_shell_handoff_acceptance_baseline_default_for_project_source(&source) {
            Ok((baseline, index, baseline_path, index_path)) => {
                self.last_shell_bundle_status = shell_handoff_acceptance_baseline_select_status(
                    &baseline,
                    &index,
                    &baseline_path,
                    &index_path,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn compare_shell_handoff_acceptance(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_handoff_acceptance_comparison_for_project_source(&source) {
            Ok((report, baseline_path, bundle_root)) => {
                self.last_shell_bundle_status = shell_handoff_acceptance_comparison_status(
                    &report,
                    &baseline_path,
                    &bundle_root,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn review_shell_release_candidate(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_release_candidate_review_for_project_source(&source) {
            Ok((report, output_path)) => {
                self.last_shell_bundle_status =
                    shell_release_candidate_review_status(&report, &output_path);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn write_shell_release_candidate_manifest(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match write_shell_release_candidate_review_manifest_for_project_source(&source) {
            Ok((review, candidate, index, review_path, candidate_path, index_path)) => {
                self.last_shell_bundle_status = shell_release_candidate_review_manifest_status(
                    &review,
                    &candidate,
                    &index,
                    &review_path,
                    &candidate_path,
                    &index_path,
                );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn append_shell_release_candidate_manifest(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match append_shell_release_candidate_review_manifest_for_project_source(&source) {
            Ok((review, candidate, index, review_path, candidate_path, index_path)) => {
                self.last_shell_bundle_status =
                    shell_release_candidate_review_manifest_append_status(
                        &review,
                        &candidate,
                        &index,
                        &review_path,
                        &candidate_path,
                        &index_path,
                    );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn inspect_shell_release_candidate_manifest(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_release_candidate_review_manifest_summary_for_project_source(&source) {
            Ok((candidate, index, candidate_path, index_path)) => {
                self.last_shell_bundle_status =
                    shell_release_candidate_review_manifest_summary_status(
                        &candidate,
                        &index,
                        &candidate_path,
                        &index_path,
                    );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn select_next_shell_release_candidate_default(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match select_next_shell_release_candidate_default_for_project_source(&source) {
            Ok((candidate, index, candidate_path, index_path)) => {
                self.last_shell_bundle_status =
                    shell_release_candidate_review_manifest_select_status(
                        &candidate,
                        &index,
                        &candidate_path,
                        &index_path,
                    );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn promote_shell_release_candidate_default(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match promote_shell_release_candidate_default_for_project_source(&source) {
            Ok((candidate, index, candidate_path, index_path)) => {
                self.last_shell_bundle_status =
                    shell_release_candidate_review_manifest_promote_status(
                        &candidate,
                        &index,
                        &candidate_path,
                        &index_path,
                    );
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn review_shell_hostess_handoff_package(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_hostess_handoff_package_for_project_source(&source) {
            Ok((report, output_path)) => {
                self.last_shell_bundle_status =
                    shell_hostess_handoff_package_status(&report, &output_path);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn review_shell_hostess_owner_intake(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_hostess_owner_intake_for_project_source(&source) {
            Ok((report, output_path)) => {
                self.last_shell_bundle_status =
                    shell_hostess_owner_intake_status(&report, &output_path);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
            }
        }
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn review_shell_hostess_staging_preview(&mut self, cx: &mut Cx) {
        let Some(source) = self.project_source.clone() else {
            self.last_shell_bundle_status = "No project source is loaded".to_string();
            self.sync_loaded_model(cx);
            self.ui.redraw(cx);
            return;
        };
        match shell_hostess_staging_preview_for_project_source(&source) {
            Ok((report, output_path)) => {
                self.last_shell_bundle_status =
                    shell_hostess_staging_preview_status(&report, &output_path);
            }
            Err(error) => {
                self.last_shell_bundle_status = error;
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
        self.ui.label(cx, ids!(shell_preview)).set_text(cx, "");
        self.ui.label(cx, ids!(shell_routes)).set_text(cx, "");
        self.ui.label(cx, ids!(shell_template)).set_text(cx, "");
        self.ui
            .label(cx, ids!(shell_bundle_status))
            .set_text(cx, "");
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
            .button(cx, ids!(export_shell_bundle_button))
            .clicked(actions)
        {
            self.export_shell_bundle_for_selected_graph(cx);
        }
        if self
            .ui
            .button(cx, ids!(verify_shell_bundle_button))
            .clicked(actions)
        {
            self.verify_shell_bundle_for_selected_graph(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_handoff_button))
            .clicked(actions)
        {
            self.prepare_shell_handoff_for_selected_graph(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_readiness_button))
            .clicked(actions)
        {
            self.inspect_shell_handoff_readiness(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_runbook_button))
            .clicked(actions)
        {
            self.inspect_shell_runbook(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_button))
            .clicked(actions)
        {
            self.review_shell_export_package(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_button))
            .clicked(actions)
        {
            self.write_shell_export_package_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_append_button))
            .clicked(actions)
        {
            self.append_shell_export_package_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_summary_button))
            .clicked(actions)
        {
            self.inspect_shell_export_package_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_next_button))
            .clicked(actions)
        {
            self.select_next_shell_export_package_baseline_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_baseline_promote_button))
            .clicked(actions)
        {
            self.promote_shell_export_package_baseline_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_export_package_compare_button))
            .clicked(actions)
        {
            self.compare_shell_export_package(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_manifest_button))
            .clicked(actions)
        {
            self.write_shell_handoff_manifest(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_button))
            .clicked(actions)
        {
            self.review_shell_handoff_acceptance(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_baseline_button))
            .clicked(actions)
        {
            self.write_shell_handoff_acceptance_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_baseline_append_button))
            .clicked(actions)
        {
            self.append_shell_handoff_acceptance_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_baseline_summary_button))
            .clicked(actions)
        {
            self.inspect_shell_handoff_acceptance_baseline(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_baseline_next_button))
            .clicked(actions)
        {
            self.select_next_shell_handoff_acceptance_baseline_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_baseline_promote_button))
            .clicked(actions)
        {
            self.promote_shell_handoff_acceptance_baseline_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_acceptance_compare_button))
            .clicked(actions)
        {
            self.compare_shell_handoff_acceptance(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_button))
            .clicked(actions)
        {
            self.review_shell_release_candidate(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_manifest_button))
            .clicked(actions)
        {
            self.write_shell_release_candidate_manifest(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_append_button))
            .clicked(actions)
        {
            self.append_shell_release_candidate_manifest(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_summary_button))
            .clicked(actions)
        {
            self.inspect_shell_release_candidate_manifest(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_next_button))
            .clicked(actions)
        {
            self.select_next_shell_release_candidate_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_release_candidate_promote_button))
            .clicked(actions)
        {
            self.promote_shell_release_candidate_default(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_handoff_package_button))
            .clicked(actions)
        {
            self.review_shell_hostess_handoff_package(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_owner_intake_button))
            .clicked(actions)
        {
            self.review_shell_hostess_owner_intake(cx);
        }
        if self
            .ui
            .button(cx, ids!(shell_hostess_staging_preview_button))
            .clicked(actions)
        {
            self.review_shell_hostess_staging_preview(cx);
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
    let project_path = if let Some(project_path) = project_path_from_args() {
        project_path_for_mutable_session(project_path)?
    } else {
        let default_path = find_default_project_path().ok_or_else(|| {
            "no project path supplied and default example was not found".to_string()
        })?;
        default_project_working_copy_path(&default_path)?
    };
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

fn export_shell_bundle_for_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
) -> Result<(StudioShellBundleReport, PathBuf), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = selected_shell_bundle_for_graph(&project, project_path.parent(), &graph_id);
    let output_dir = selected_shell_bundle_output_dir(project_path, &graph_id);
    if report.status == StudioShellBundleStatus::Exported {
        save_shell_bundle(&output_dir, &report)
            .map_err(|error| format!("Shell bundle save failed: {error}"))?;
    }
    Ok((report, output_dir))
}

fn validate_shell_bundle_for_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
) -> Result<(StudioShellBundleValidationReport, PathBuf), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let output_dir = selected_shell_bundle_output_dir(project_path, &graph_id);
    let report =
        validate_selected_shell_bundle(&project, project_path.parent(), &graph_id, &output_dir);
    Ok((report, output_dir))
}

fn shell_handoff_for_project_source(
    project_path: &Path,
    model: &StudioViewModel,
    selected_graph_index: usize,
) -> Result<(StudioShellHandoffReport, PathBuf), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let output_dir = selected_shell_bundle_output_dir(project_path, &graph_id);
    let report = shell_handoff_for_bundle(&project, project_path.parent(), &graph_id, &output_dir);
    Ok((report, output_dir))
}

fn shell_handoff_readiness_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHandoffReadinessReport, PathBuf), String> {
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let bundle_root = selected_shell_bundle_root_dir(project_path);
    let report = shell_handoff_readiness_for_project(&project, project_path.parent(), &bundle_root);
    Ok((report, bundle_root))
}

fn shell_runbook_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellRunbookReport, PathBuf), String> {
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let bundle_root = selected_shell_bundle_root_dir(project_path);
    let report = shell_runbook_for_project(&project, project_path.parent(), &bundle_root);
    Ok((report, bundle_root))
}

fn shell_export_package_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellExportPackageReport, PathBuf), String> {
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let bundle_root = selected_shell_bundle_root_dir(project_path);
    let report = shell_export_package_for_project(&project, project_path.parent(), &bundle_root);
    Ok((report, bundle_root))
}

fn write_shell_export_package_baseline_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellExportPackageReport,
        StudioShellExportPackageBaselineManifest,
        StudioShellExportPackageBaselineIndex,
        PathBuf,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let (report, bundle_root) = shell_export_package_for_project_source(project_path)?;
    let package_path = shell_export_package_output_path(project_path);
    save_json(&package_path, &report)
        .map_err(|error| format!("Shell export package review save failed: {error}"))?;
    let baseline =
        shell_export_package_baseline_manifest_for_report(&report, &package_path, None, None);
    let baseline_path = shell_export_package_baseline_manifest_output_path(project_path);
    save_json(&baseline_path, &baseline)
        .map_err(|error| format!("Shell export package baseline identity save failed: {error}"))?;
    let index = shell_export_package_baseline_index_for_manifests(
        vec![(baseline.clone(), Some(baseline_path.clone()))],
        Some(&baseline.baseline_id),
    );
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell export package baseline index save failed: {error}"))?;
    Ok((
        report,
        baseline,
        index,
        package_path,
        baseline_path,
        index_path,
        bundle_root,
    ))
}

fn append_shell_export_package_baseline_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellExportPackageReport,
        StudioShellExportPackageBaselineManifest,
        StudioShellExportPackageBaselineIndex,
        PathBuf,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let (report, bundle_root) = shell_export_package_for_project_source(project_path)?;
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    let existing_index = if index_path.is_file() {
        Some(
            load_shell_export_package_baseline_index(&index_path)
                .map_err(|error| format!("Export package baseline index load failed: {error}"))?,
        )
    } else {
        None
    };
    let (baseline_id, label) =
        next_shell_export_package_baseline_archive_identity(&report, existing_index.as_ref());
    let package_path =
        shell_export_package_baseline_archive_package_output_path(project_path, &baseline_id);
    save_json(&package_path, &report)
        .map_err(|error| format!("Shell export package baseline review save failed: {error}"))?;
    let baseline = shell_export_package_baseline_manifest_for_report(
        &report,
        &package_path,
        Some(&baseline_id),
        Some(&label),
    );
    let baseline_path =
        shell_export_package_baseline_archive_manifest_output_path(project_path, &baseline_id);
    save_json(&baseline_path, &baseline)
        .map_err(|error| format!("Shell export package baseline identity save failed: {error}"))?;
    let index = if let Some(index) = existing_index.as_ref() {
        append_shell_export_package_baseline_index_manifests(
            index,
            vec![(baseline.clone(), Some(baseline_path.clone()))],
            Some(&baseline.baseline_id),
        )
    } else {
        shell_export_package_baseline_index_for_manifests(
            vec![(baseline.clone(), Some(baseline_path.clone()))],
            Some(&baseline.baseline_id),
        )
    };
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell export package baseline index save failed: {error}"))?;
    Ok((
        report,
        baseline,
        index,
        package_path,
        baseline_path,
        index_path,
        bundle_root,
    ))
}

fn next_shell_export_package_baseline_archive_identity(
    report: &StudioShellExportPackageReport,
    index: Option<&StudioShellExportPackageBaselineIndex>,
) -> (String, String) {
    let status = shell_export_package_status_label(report.status);
    let base_id = format!(
        "{}.rev{}.{}",
        report.project_id, report.project_revision, status
    );
    let next_slot = index
        .map(|index| {
            index
                .entries
                .iter()
                .filter(|entry| {
                    entry.baseline_id == base_id
                        || entry
                            .baseline_id
                            .strip_prefix(base_id.as_str())
                            .is_some_and(|suffix| suffix.starts_with(".archive"))
                })
                .count()
                + 1
        })
        .unwrap_or(1);
    let baseline_id = if next_slot == 1 {
        base_id
    } else {
        format!("{base_id}.archive{next_slot}")
    };
    let label = if next_slot == 1 {
        format!(
            "{} revision {} {} export package baseline",
            report.project_id, report.project_revision, status
        )
    } else {
        format!(
            "{} revision {} {} export package baseline archive {}",
            report.project_id, report.project_revision, status, next_slot
        )
    };
    (baseline_id, label)
}

fn shell_export_package_baseline_summary_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellExportPackageBaselineManifest,
        StudioShellExportPackageBaselineIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let baseline_path = shell_export_package_baseline_manifest_output_path(project_path);
    let baseline = load_shell_export_package_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Export package baseline identity load failed: {error}"))?;
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    let index = load_shell_export_package_baseline_index(&index_path)
        .map_err(|error| format!("Export package baseline index load failed: {error}"))?;
    Ok((baseline, index, baseline_path, index_path))
}

fn promote_shell_export_package_baseline_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellExportPackageBaselineManifest,
        StudioShellExportPackageBaselineIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let baseline_path = shell_export_package_baseline_manifest_output_path(project_path);
    let baseline = load_shell_export_package_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Export package baseline identity load failed: {error}"))?;
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    let index = load_shell_export_package_baseline_index(&index_path)
        .map_err(|error| format!("Export package baseline index load failed: {error}"))?;
    let promoted =
        promote_shell_export_package_baseline_index_default(&index, &baseline.baseline_id)
            .ok_or_else(|| {
                format!(
                    "Export package baseline index does not contain baseline {}",
                    baseline.baseline_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Export package baseline index save failed: {error}"))?;
    Ok((baseline, promoted, baseline_path, index_path))
}

fn select_next_shell_export_package_baseline_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellExportPackageBaselineManifest,
        StudioShellExportPackageBaselineIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    let index = load_shell_export_package_baseline_index(&index_path)
        .map_err(|error| format!("Export package baseline index load failed: {error}"))?;
    let baseline_id = next_shell_export_package_baseline_default_id(&index)?;
    let baseline_path = index
        .entries
        .iter()
        .find(|entry| entry.baseline_id == baseline_id)
        .and_then(|entry| entry.baseline_manifest_path.as_ref())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!("Export package baseline index entry {baseline_id} does not include a manifest path")
        })?;
    let baseline = load_shell_export_package_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Export package baseline identity load failed: {error}"))?;
    let promoted =
        promote_shell_export_package_baseline_index_default(&index, &baseline.baseline_id)
            .ok_or_else(|| {
                format!(
                    "Export package baseline index does not contain baseline {}",
                    baseline.baseline_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Export package baseline index save failed: {error}"))?;
    Ok((baseline, promoted, baseline_path, index_path))
}

fn next_shell_export_package_baseline_default_id(
    index: &StudioShellExportPackageBaselineIndex,
) -> Result<String, String> {
    if index.entries.is_empty() {
        return Err("Export package baseline index has no selectable entries".to_string());
    }
    let default_position = index.default_baseline_id.as_deref().and_then(|default_id| {
        index
            .entries
            .iter()
            .position(|entry| entry.baseline_id == default_id)
    });
    let selected_position = default_position.map_or(0, |position| {
        if position + 1 >= index.entries.len() {
            0
        } else {
            position + 1
        }
    });
    Ok(index.entries[selected_position].baseline_id.clone())
}

fn shell_export_package_comparison_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellExportPackageComparisonReport, PathBuf, PathBuf), String> {
    let index_path = shell_export_package_baseline_index_output_path(project_path);
    let index = load_shell_export_package_baseline_index(&index_path)
        .map_err(|error| format!("Export package baseline index load failed: {error}"))?;
    let Some(baseline_index_entry) = select_shell_export_package_baseline_index_entry(&index, None)
    else {
        return Err(
            "Export package baseline index does not contain a selected baseline".to_string(),
        );
    };
    let baseline_path = baseline_index_entry
        .baseline_manifest_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Selected export package baseline index entry does not include a baseline manifest path"
                .to_string()
        })?;
    let baseline_identity = load_shell_export_package_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Export package baseline identity load failed: {error}"))?;
    let package_path = PathBuf::from(&baseline_identity.package_path);
    let baseline = load_shell_export_package_report(&package_path)
        .map_err(|error| format!("Export package baseline review load failed: {error}"))?;
    let (candidate, bundle_root) = shell_export_package_for_project_source(project_path)?;
    let report = compare_shell_export_packages_against_baseline_index_entry(
        &index,
        Some(&index_path),
        baseline_index_entry,
        Some(&baseline_path),
        &baseline_identity,
        &baseline,
        &candidate,
    );
    Ok((report, baseline_path, bundle_root))
}

fn write_shell_handoff_manifest_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHandoffManifest, PathBuf), String> {
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let bundle_root = selected_shell_bundle_root_dir(project_path);
    let manifest =
        shell_handoff_manifest_for_project(&project, project_path.parent(), &bundle_root);
    let output_path = shell_handoff_manifest_output_path(project_path);
    save_json(&output_path, &manifest)
        .map_err(|error| format!("Shell handoff manifest save failed: {error}"))?;
    Ok((manifest, output_path))
}

fn shell_handoff_acceptance_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHandoffAcceptanceChecklistReport, PathBuf), String> {
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let bundle_root = selected_shell_bundle_root_dir(project_path);
    let report = shell_handoff_acceptance_checklist_for_project(
        &project,
        project_path.parent(),
        &bundle_root,
    );
    Ok((report, bundle_root))
}

fn write_shell_handoff_acceptance_baseline_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceChecklistReport,
        StudioShellHandoffAcceptanceBaselineManifest,
        StudioShellHandoffAcceptanceBaselineIndex,
        PathBuf,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let (report, bundle_root) = shell_handoff_acceptance_for_project_source(project_path)?;
    let output_path = shell_handoff_acceptance_checklist_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell handoff acceptance baseline save failed: {error}"))?;
    let baseline =
        shell_handoff_acceptance_baseline_manifest_for_checklist(&report, &output_path, None, None);
    let baseline_path = shell_handoff_acceptance_baseline_manifest_output_path(project_path);
    save_json(&baseline_path, &baseline).map_err(|error| {
        format!("Shell handoff acceptance baseline identity save failed: {error}")
    })?;
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(baseline.clone(), Some(baseline_path.clone()))],
        Some(&baseline.baseline_id),
    );
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell handoff acceptance baseline index save failed: {error}"))?;
    Ok((
        report,
        baseline,
        index,
        output_path,
        baseline_path,
        index_path,
        bundle_root,
    ))
}

fn append_shell_handoff_acceptance_baseline_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceChecklistReport,
        StudioShellHandoffAcceptanceBaselineManifest,
        StudioShellHandoffAcceptanceBaselineIndex,
        PathBuf,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let (report, bundle_root) = shell_handoff_acceptance_for_project_source(project_path)?;
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let existing_index = if index_path.is_file() {
        Some(
            load_shell_handoff_acceptance_baseline_index(&index_path)
                .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?,
        )
    } else {
        None
    };
    let (baseline_id, label) =
        next_shell_handoff_acceptance_baseline_archive_identity(&report, existing_index.as_ref());
    let checklist_path =
        shell_handoff_acceptance_baseline_archive_checklist_output_path(project_path, &baseline_id);
    save_json(&checklist_path, &report)
        .map_err(|error| format!("Shell handoff acceptance baseline save failed: {error}"))?;
    let baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &report,
        &checklist_path,
        Some(&baseline_id),
        Some(&label),
    );
    let baseline_path =
        shell_handoff_acceptance_baseline_archive_manifest_output_path(project_path, &baseline_id);
    save_json(&baseline_path, &baseline).map_err(|error| {
        format!("Shell handoff acceptance baseline identity save failed: {error}")
    })?;
    let index = if let Some(index) = existing_index.as_ref() {
        append_shell_handoff_acceptance_baseline_index_manifests(
            index,
            vec![(baseline.clone(), Some(baseline_path.clone()))],
            Some(&baseline.baseline_id),
        )
    } else {
        shell_handoff_acceptance_baseline_index_for_manifests(
            vec![(baseline.clone(), Some(baseline_path.clone()))],
            Some(&baseline.baseline_id),
        )
    };
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell handoff acceptance baseline index save failed: {error}"))?;
    Ok((
        report,
        baseline,
        index,
        checklist_path,
        baseline_path,
        index_path,
        bundle_root,
    ))
}

fn next_shell_handoff_acceptance_baseline_archive_identity(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    index: Option<&StudioShellHandoffAcceptanceBaselineIndex>,
) -> (String, String) {
    let status = shell_handoff_acceptance_status_label(report.status);
    let base_id = format!(
        "{}.rev{}.{}",
        report.project_id, report.project_revision, status
    );
    let next_slot = index
        .map(|index| {
            index
                .entries
                .iter()
                .filter(|entry| {
                    entry.baseline_id == base_id
                        || entry
                            .baseline_id
                            .strip_prefix(base_id.as_str())
                            .is_some_and(|suffix| suffix.starts_with(".archive"))
                })
                .count()
                + 1
        })
        .unwrap_or(1);
    let baseline_id = if next_slot == 1 {
        base_id
    } else {
        format!("{base_id}.archive{next_slot}")
    };
    let label = if next_slot == 1 {
        format!(
            "{} revision {} {} acceptance baseline",
            report.project_id, report.project_revision, status
        )
    } else {
        format!(
            "{} revision {} {} acceptance baseline archive {}",
            report.project_id, report.project_revision, status, next_slot
        )
    };
    (baseline_id, label)
}

fn shell_handoff_acceptance_baseline_summary_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceBaselineManifest,
        StudioShellHandoffAcceptanceBaselineIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let baseline_path = shell_handoff_acceptance_baseline_manifest_output_path(project_path);
    let baseline = load_shell_handoff_acceptance_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Baseline acceptance identity load failed: {error}"))?;
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let index = load_shell_handoff_acceptance_baseline_index(&index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    Ok((baseline, index, baseline_path, index_path))
}

fn promote_shell_handoff_acceptance_baseline_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceBaselineManifest,
        StudioShellHandoffAcceptanceBaselineIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let baseline_path = shell_handoff_acceptance_baseline_manifest_output_path(project_path);
    let baseline = load_shell_handoff_acceptance_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Baseline acceptance identity load failed: {error}"))?;
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let index = load_shell_handoff_acceptance_baseline_index(&index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    let promoted =
        promote_shell_handoff_acceptance_baseline_index_default(&index, &baseline.baseline_id)
            .ok_or_else(|| {
                format!(
                    "Baseline acceptance index does not contain baseline {}",
                    baseline.baseline_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Baseline acceptance index save failed: {error}"))?;
    Ok((baseline, promoted, baseline_path, index_path))
}

fn select_next_shell_handoff_acceptance_baseline_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceBaselineManifest,
        StudioShellHandoffAcceptanceBaselineIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let index = load_shell_handoff_acceptance_baseline_index(&index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    let baseline_id = next_shell_handoff_acceptance_baseline_default_id(&index)?;
    let baseline_path = index
        .entries
        .iter()
        .find(|entry| entry.baseline_id == baseline_id)
        .and_then(|entry| entry.baseline_manifest_path.as_ref())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "Baseline acceptance index entry {baseline_id} does not include a manifest path"
            )
        })?;
    let baseline = load_shell_handoff_acceptance_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Baseline acceptance identity load failed: {error}"))?;
    let promoted =
        promote_shell_handoff_acceptance_baseline_index_default(&index, &baseline.baseline_id)
            .ok_or_else(|| {
                format!(
                    "Baseline acceptance index does not contain baseline {}",
                    baseline.baseline_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Baseline acceptance index save failed: {error}"))?;
    Ok((baseline, promoted, baseline_path, index_path))
}

fn next_shell_handoff_acceptance_baseline_default_id(
    index: &StudioShellHandoffAcceptanceBaselineIndex,
) -> Result<String, String> {
    if index.entries.is_empty() {
        return Err("Baseline acceptance index has no selectable entries".to_string());
    }
    let default_position = index.default_baseline_id.as_deref().and_then(|default_id| {
        index
            .entries
            .iter()
            .position(|entry| entry.baseline_id == default_id)
    });
    let selected_position = default_position.map_or(0, |position| {
        if position + 1 >= index.entries.len() {
            0
        } else {
            position + 1
        }
    });
    Ok(index.entries[selected_position].baseline_id.clone())
}

fn shell_handoff_acceptance_comparison_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceComparisonReport,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let index = load_shell_handoff_acceptance_baseline_index(&index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    let Some(baseline_index_entry) =
        select_shell_handoff_acceptance_baseline_index_entry(&index, None)
    else {
        return Err("Baseline acceptance index does not contain a selected baseline".to_string());
    };
    let baseline_path = baseline_index_entry
        .baseline_manifest_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Selected baseline index entry does not include a baseline manifest path".to_string()
        })?;
    let baseline_identity = load_shell_handoff_acceptance_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Baseline acceptance identity load failed: {error}"))?;
    let checklist_path = PathBuf::from(&baseline_identity.checklist_path);
    let baseline = load_shell_handoff_acceptance_checklist(&checklist_path)
        .map_err(|error| format!("Baseline acceptance checklist load failed: {error}"))?;
    let (candidate, bundle_root) = shell_handoff_acceptance_for_project_source(project_path)?;
    let report = compare_shell_handoff_acceptance_against_baseline_index_entry(
        &index,
        Some(&index_path),
        baseline_index_entry,
        Some(&baseline_path),
        &baseline_identity,
        &baseline,
        &candidate,
    );
    Ok((report, baseline_path, bundle_root))
}

fn shell_release_candidate_review_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellReleaseCandidateReviewReport, PathBuf), String> {
    let manifest_path = shell_handoff_manifest_output_path(project_path);
    let manifest = load_shell_handoff_manifest(&manifest_path)
        .map_err(|error| format!("Shell handoff manifest load failed: {error}"))?;
    let acceptance_index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let acceptance_index = load_shell_handoff_acceptance_baseline_index(&acceptance_index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    let export_package_index_path = shell_export_package_baseline_index_output_path(project_path);
    let export_package_index = load_shell_export_package_baseline_index(&export_package_index_path)
        .map_err(|error| format!("Export package baseline index load failed: {error}"))?;
    let report = shell_release_candidate_review_for_manifest(
        &manifest,
        Some(&manifest_path),
        &acceptance_index,
        Some(&acceptance_index_path),
        None,
        &export_package_index,
        Some(&export_package_index_path),
        None,
    );
    let output_path = shell_release_candidate_review_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell release candidate review save failed: {error}"))?;
    Ok((report, output_path))
}

fn write_shell_release_candidate_review_manifest_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellReleaseCandidateReviewReport,
        StudioShellReleaseCandidateReviewManifest,
        StudioShellReleaseCandidateReviewIndex,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let (review, review_path) = shell_release_candidate_review_for_project_source(project_path)?;
    let candidate =
        shell_release_candidate_review_manifest_for_report(&review, &review_path, None, None);
    let candidate_path = shell_release_candidate_review_manifest_output_path(project_path);
    save_json(&candidate_path, &candidate)
        .map_err(|error| format!("Shell release candidate identity save failed: {error}"))?;
    let index = shell_release_candidate_review_index_for_manifests(
        vec![(candidate.clone(), Some(candidate_path.clone()))],
        Some(&candidate.candidate_id),
    );
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell release candidate index save failed: {error}"))?;
    Ok((
        review,
        candidate,
        index,
        review_path,
        candidate_path,
        index_path,
    ))
}

fn append_shell_release_candidate_review_manifest_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellReleaseCandidateReviewReport,
        StudioShellReleaseCandidateReviewManifest,
        StudioShellReleaseCandidateReviewIndex,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let (review, _) = shell_release_candidate_review_for_project_source(project_path)?;
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let existing_index = if index_path.is_file() {
        Some(
            load_shell_release_candidate_review_index(&index_path)
                .map_err(|error| format!("Shell release candidate index load failed: {error}"))?,
        )
    } else {
        None
    };
    let (candidate_id, label) =
        next_shell_release_candidate_archive_identity(&review, existing_index.as_ref());
    let review_path =
        shell_release_candidate_review_archive_report_output_path(project_path, &candidate_id);
    save_json(&review_path, &review)
        .map_err(|error| format!("Shell release candidate review archive save failed: {error}"))?;
    let candidate = shell_release_candidate_review_manifest_for_report(
        &review,
        &review_path,
        Some(&candidate_id),
        Some(&label),
    );
    let candidate_path =
        shell_release_candidate_review_archive_manifest_output_path(project_path, &candidate_id);
    save_json(&candidate_path, &candidate)
        .map_err(|error| format!("Shell release candidate identity save failed: {error}"))?;
    let index = if let Some(index) = existing_index.as_ref() {
        append_shell_release_candidate_review_index_manifests(
            index,
            vec![(candidate.clone(), Some(candidate_path.clone()))],
            Some(&candidate.candidate_id),
        )
    } else {
        shell_release_candidate_review_index_for_manifests(
            vec![(candidate.clone(), Some(candidate_path.clone()))],
            Some(&candidate.candidate_id),
        )
    };
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell release candidate index save failed: {error}"))?;
    Ok((
        review,
        candidate,
        index,
        review_path,
        candidate_path,
        index_path,
    ))
}

fn next_shell_release_candidate_archive_identity(
    review: &StudioShellReleaseCandidateReviewReport,
    index: Option<&StudioShellReleaseCandidateReviewIndex>,
) -> (String, String) {
    let status = shell_release_candidate_review_status_label(review.status);
    let base_id = format!(
        "{}.rev{}.{}",
        review.project_id, review.project_revision, status
    );
    let next_slot = index
        .map(|index| {
            index
                .entries
                .iter()
                .filter(|entry| {
                    entry.candidate_id == base_id
                        || entry
                            .candidate_id
                            .strip_prefix(base_id.as_str())
                            .is_some_and(|suffix| suffix.starts_with(".archive"))
                })
                .count()
                + 1
        })
        .unwrap_or(1);
    let candidate_id = if next_slot == 1 {
        base_id
    } else {
        format!("{base_id}.archive{next_slot}")
    };
    let label = if next_slot == 1 {
        format!(
            "{} revision {} {} release candidate",
            review.project_id, review.project_revision, status
        )
    } else {
        format!(
            "{} revision {} {} release candidate archive {}",
            review.project_id, review.project_revision, status, next_slot
        )
    };
    (candidate_id, label)
}

fn shell_release_candidate_review_manifest_summary_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellReleaseCandidateReviewManifest,
        StudioShellReleaseCandidateReviewIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let candidate_path = shell_release_candidate_review_manifest_output_path(project_path);
    let candidate = load_shell_release_candidate_review_manifest(&candidate_path)
        .map_err(|error| format!("Shell release candidate identity load failed: {error}"))?;
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let index = load_shell_release_candidate_review_index(&index_path)
        .map_err(|error| format!("Shell release candidate index load failed: {error}"))?;
    Ok((candidate, index, candidate_path, index_path))
}

fn promote_shell_release_candidate_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellReleaseCandidateReviewManifest,
        StudioShellReleaseCandidateReviewIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let candidate_path = shell_release_candidate_review_manifest_output_path(project_path);
    let candidate = load_shell_release_candidate_review_manifest(&candidate_path)
        .map_err(|error| format!("Shell release candidate identity load failed: {error}"))?;
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let index = load_shell_release_candidate_review_index(&index_path)
        .map_err(|error| format!("Shell release candidate index load failed: {error}"))?;
    let promoted =
        promote_shell_release_candidate_review_index_default(&index, &candidate.candidate_id)
            .ok_or_else(|| {
                format!(
                    "Shell release candidate index does not contain candidate {}",
                    candidate.candidate_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Shell release candidate index save failed: {error}"))?;
    Ok((candidate, promoted, candidate_path, index_path))
}

fn select_next_shell_release_candidate_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellReleaseCandidateReviewManifest,
        StudioShellReleaseCandidateReviewIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let index = load_shell_release_candidate_review_index(&index_path)
        .map_err(|error| format!("Shell release candidate index load failed: {error}"))?;
    let candidate_id = next_shell_release_candidate_default_id(&index)?;
    let candidate_path = index
        .entries
        .iter()
        .find(|entry| entry.candidate_id == candidate_id)
        .and_then(|entry| entry.candidate_manifest_path.as_ref())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "Shell release candidate index entry {candidate_id} does not include a manifest path"
            )
        })?;
    let candidate = load_shell_release_candidate_review_manifest(&candidate_path)
        .map_err(|error| format!("Shell release candidate identity load failed: {error}"))?;
    let promoted =
        promote_shell_release_candidate_review_index_default(&index, &candidate.candidate_id)
            .ok_or_else(|| {
                format!(
                    "Shell release candidate index does not contain candidate {}",
                    candidate.candidate_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Shell release candidate index save failed: {error}"))?;
    Ok((candidate, promoted, candidate_path, index_path))
}

fn next_shell_release_candidate_default_id(
    index: &StudioShellReleaseCandidateReviewIndex,
) -> Result<String, String> {
    if index.entries.is_empty() {
        return Err("Shell release candidate index has no selectable entries".to_string());
    }
    let default_position = index
        .default_candidate_id
        .as_deref()
        .and_then(|default_id| {
            index
                .entries
                .iter()
                .position(|entry| entry.candidate_id == default_id)
        });
    let selected_position = default_position.map_or(0, |position| {
        if position + 1 >= index.entries.len() {
            0
        } else {
            position + 1
        }
    });
    Ok(index.entries[selected_position].candidate_id.clone())
}

fn shell_hostess_handoff_package_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessHandoffPackageReport, PathBuf), String> {
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let index = load_shell_release_candidate_review_index(&index_path)
        .map_err(|error| format!("Shell release candidate index load failed: {error}"))?;
    let report =
        shell_hostess_handoff_package_for_release_candidate_index(&index, Some(&index_path), None);
    let output_path = shell_hostess_handoff_package_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess handoff package save failed: {error}"))?;
    Ok((report, output_path))
}

fn shell_hostess_owner_intake_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessOwnerIntakeReport, PathBuf), String> {
    let package_path = shell_hostess_handoff_package_output_path(project_path);
    let package = load_shell_hostess_handoff_package_report(&package_path)
        .map_err(|error| format!("Shell Hostess handoff package load failed: {error}"))?;
    let report = shell_hostess_owner_intake_for_handoff_package(&package, Some(&package_path));
    let output_path = shell_hostess_owner_intake_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess owner intake save failed: {error}"))?;
    Ok((report, output_path))
}

fn shell_hostess_staging_preview_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessStagingPreviewManifest, PathBuf), String> {
    let intake_path = shell_hostess_owner_intake_output_path(project_path);
    let intake = load_shell_hostess_owner_intake_report(&intake_path)
        .map_err(|error| format!("Shell Hostess owner intake load failed: {error}"))?;
    let report = shell_hostess_staging_preview_for_owner_intake(&intake, Some(&intake_path));
    let output_path = shell_hostess_staging_preview_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess staging preview save failed: {error}"))?;
    Ok((report, output_path))
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

fn selected_shell_bundle_root_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-selected-shell")
}

fn selected_shell_bundle_output_dir(project_path: &Path, graph_id: &str) -> PathBuf {
    selected_shell_bundle_root_dir(project_path).join(graph_id)
}

fn shell_handoff_manifest_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoffs.json")
}

fn shell_handoff_acceptance_checklist_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoff-acceptance-checklist.json")
}

fn shell_export_package_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-export-package.json")
}

fn shell_export_package_baseline_manifest_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-export-package-baseline.json")
}

fn shell_export_package_baseline_archive_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("export-package-baselines")
}

fn shell_export_package_baseline_archive_package_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_export_package_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.package.json"))
}

fn shell_export_package_baseline_archive_manifest_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_export_package_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.baseline.json"))
}

fn shell_export_package_baseline_index_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-export-package-baselines.json")
}

fn shell_handoff_acceptance_baseline_manifest_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoff-acceptance-baseline.json")
}

fn shell_handoff_acceptance_baseline_archive_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("baselines")
}

fn shell_handoff_acceptance_baseline_archive_checklist_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_handoff_acceptance_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.checklist.json"))
}

fn shell_handoff_acceptance_baseline_archive_manifest_output_path(
    project_path: &Path,
    baseline_id: &str,
) -> PathBuf {
    shell_handoff_acceptance_baseline_archive_dir(project_path)
        .join(format!("{baseline_id}.baseline.json"))
}

fn shell_handoff_acceptance_baseline_index_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-handoff-acceptance-baselines.json")
}

fn shell_release_candidate_review_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-release-candidate-review.json")
}

fn shell_release_candidate_review_manifest_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-release-candidate-review-manifest.json")
}

fn shell_release_candidate_review_archive_dir(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("release-candidates")
}

fn shell_release_candidate_review_archive_report_output_path(
    project_path: &Path,
    candidate_id: &str,
) -> PathBuf {
    shell_release_candidate_review_archive_dir(project_path)
        .join(format!("{candidate_id}.review.json"))
}

fn shell_release_candidate_review_archive_manifest_output_path(
    project_path: &Path,
    candidate_id: &str,
) -> PathBuf {
    shell_release_candidate_review_archive_dir(project_path)
        .join(format!("{candidate_id}.candidate.json"))
}

fn shell_release_candidate_review_index_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-release-candidate-reviews.json")
}

fn shell_hostess_handoff_package_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-handoff-package.json")
}

fn shell_hostess_owner_intake_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-owner-intake.json")
}

fn shell_hostess_staging_preview_output_path(project_path: &Path) -> PathBuf {
    project_path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("target")
        .join("studio-shell-handoffs")
        .join("shell-hostess-staging-preview.json")
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

fn default_project_working_copy_path(default_path: &Path) -> Result<PathBuf, String> {
    let default_path = normalize_verbatim_path(
        std::fs::canonicalize(default_path)
            .map_err(|error| format!("Default example resolve failed: {error}"))?,
    );
    let examples_dir = default_path
        .parent()
        .ok_or_else(|| "default example has no parent directory".to_string())?;
    let repo_root = examples_dir
        .parent()
        .ok_or_else(|| "default example is not inside the repo examples directory".to_string())?;
    let file_name = default_path
        .file_name()
        .ok_or_else(|| "default example has no file name".to_string())?;
    let working_dir = repo_root.join("examples-working");
    let working_path = working_dir.join(file_name);
    std::fs::create_dir_all(&working_dir)
        .map_err(|error| format!("Default example working directory create failed: {error}"))?;
    std::fs::copy(&default_path, &working_path)
        .map_err(|error| format!("Default example working copy failed: {error}"))?;
    Ok(working_path)
}

fn project_path_for_mutable_session(project_path: PathBuf) -> Result<PathBuf, String> {
    if is_tracked_synthetic_example_path(&project_path)? {
        return default_project_working_copy_path(&project_path);
    }

    Ok(project_path)
}

fn is_tracked_synthetic_example_path(project_path: &Path) -> Result<bool, String> {
    if project_path.file_name().and_then(|name| name.to_str())
        != Some("synthetic-studio-project.json")
    {
        return Ok(false);
    }

    if !project_path.is_file() {
        return Ok(false);
    }

    let project_path = normalize_verbatim_path(
        std::fs::canonicalize(project_path)
            .map_err(|error| format!("Project path resolve failed: {error}"))?,
    );
    Ok(project_path
        .parent()
        .and_then(|parent| parent.file_name())
        .and_then(|name| name.to_str())
        == Some("examples"))
}

fn normalize_verbatim_path(path: PathBuf) -> PathBuf {
    #[cfg(windows)]
    {
        let path_text = path.to_string_lossy();
        if let Some(rest) = path_text.strip_prefix(r"\\?\UNC\") {
            return PathBuf::from(format!(r"\\{rest}"));
        }
        if let Some(rest) = path_text.strip_prefix(r"\\?\") {
            return PathBuf::from(rest);
        }
    }
    path
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

fn shell_preview_lines(model: &StudioViewModel) -> String {
    let Some(preview) = model.shell_preview.as_ref() else {
        return "none".to_string();
    };
    let mut lines = Vec::new();
    lines.push(format!(
        "{} [{}]",
        preview.graph_id,
        shell_descriptor_status_label(preview.status)
    ));
    if let Some(issue_code) = preview.issue_code.as_deref() {
        lines.push(format!("issue: {issue_code}"));
    }
    lines.push(preview.message.clone());
    if let Some(descriptor_id) = preview.descriptor_id.as_deref() {
        lines.push(format!("descriptor: {descriptor_id}"));
    }
    if let Some(shell_id) = preview.shell_id.as_deref() {
        lines.push(format!(
            "shell: {} / {}",
            shell_id,
            preview.shell_label.as_deref().unwrap_or("unlabeled")
        ));
    }
    if let Some(target_host_profile) = preview.target_host_profile.as_deref() {
        lines.push(format!(
            "target: {} / {}",
            target_host_profile,
            preview
                .target_kind
                .map(shell_target_kind_label)
                .unwrap_or("unknown")
        ));
    }
    lines.push(format!(
        "graph: {} package(s), {} module(s), {} stream binding(s), {} command binding(s)",
        preview.package_count,
        preview.module_count,
        preview.stream_binding_count,
        preview.command_binding_count
    ));
    if let Some(status) = preview.descriptor_validation_status {
        lines.push(format!(
            "descriptor validation: {}",
            validation_status_label(status)
        ));
    }
    lines.join("\n")
}

fn shell_route_lines(model: &StudioViewModel) -> String {
    let Some(preview) = model.shell_preview.as_ref() else {
        return "none".to_string();
    };
    if preview.status != StudioShellDescriptorStatus::Exported {
        return "none".to_string();
    }
    let mut lines = Vec::new();
    lines.push(format!(
        "host: {}",
        preview.host_profile_class.as_deref().unwrap_or("unknown")
    ));
    lines.push(format!(
        "app: {}",
        preview.app_id.as_deref().unwrap_or("not declared")
    ));
    lines.push(format!(
        "install: {}",
        preview.install_route.as_deref().unwrap_or("not declared")
    ));
    lines.push(format!(
        "launch: {}",
        preview.launch_route.as_deref().unwrap_or("not declared")
    ));
    lines.push(format!(
        "command: {}",
        preview.command_bridge.as_deref().unwrap_or("not declared")
    ));
    lines.push(format!(
        "evidence: {}",
        preview
            .evidence_pull_route
            .as_deref()
            .unwrap_or("not declared")
    ));
    lines.join("\n")
}

fn shell_template_lines(model: &StudioViewModel) -> String {
    let Some(preview) = model.shell_preview.as_ref() else {
        return "none".to_string();
    };
    if preview.status != StudioShellDescriptorStatus::Exported {
        return "none".to_string();
    }
    let mut lines = Vec::new();
    if let Some(template_id) = preview.template_id.as_deref() {
        lines.push(format!("template: {template_id}"));
    }
    if let Some(template_path) = preview.template_path.as_deref() {
        lines.push(format!("path: {template_path}"));
    }
    if let Some(descriptor_path) = preview.descriptor_path.as_deref() {
        lines.push(format!("descriptor: {descriptor_path}"));
    }
    if let Some(staged_descriptor_path) = preview.template_descriptor_path.as_deref() {
        lines.push(format!("staged descriptor: {staged_descriptor_path}"));
    }
    lines.push(format!(
        "authority: {} / {} / {}",
        preview
            .runtime_command_authority
            .as_deref()
            .unwrap_or("unknown"),
        preview
            .runtime_host_authority
            .as_deref()
            .unwrap_or("unknown"),
        preview.studio_role.as_deref().unwrap_or("unknown")
    ));
    lines.join("\n")
}

fn shell_bundle_status_line(status: &str) -> String {
    if status.is_empty() {
        "not exported".to_string()
    } else {
        status.to_string()
    }
}

fn shell_bundle_export_status(report: &StudioShellBundleReport, output_dir: &Path) -> String {
    let status = shell_bundle_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    if report.status != StudioShellBundleStatus::Exported {
        return format!(
            "{status}; issue {issue}\n  graph: {}\n  {}",
            report.graph_id, report.message
        );
    }
    let files = if report.bundle_files.is_empty() {
        "none".to_string()
    } else {
        report.bundle_files.join("\n  ")
    };
    format!(
        "{status}; issue {issue}\n  graph: {}\n  output: {}\n  files:\n  {}",
        report.graph_id,
        output_dir.display(),
        files
    )
}

fn shell_bundle_validation_status(
    report: &StudioShellBundleValidationReport,
    output_dir: &Path,
) -> String {
    let status = validation_status_label(report.status);
    let failed = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .collect::<Vec<_>>();
    if failed.is_empty() {
        return format!(
            "validated; status {status}\n  graph: {}\n  output: {}\n  files: {}",
            report.graph_id,
            output_dir.display(),
            report.expected_bundle_files.len()
        );
    }
    let issues = failed
        .iter()
        .take(4)
        .map(|check| {
            format!(
                "{}: {}",
                check
                    .issue_code
                    .as_deref()
                    .unwrap_or("studio.issue.unknown"),
                check.evidence
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    format!(
        "validated; status {status}\n  graph: {}\n  output: {}\n  failed: {}\n  {}",
        report.graph_id,
        output_dir.display(),
        failed.len(),
        issues
    )
}

fn shell_handoff_status(report: &StudioShellHandoffReport, output_dir: &Path) -> String {
    let status = validation_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    if report.status == StudioValidationStatus::Pass {
        let args = if report.consumer_args.is_empty() {
            "none".to_string()
        } else {
            report.consumer_args.join(" ")
        };
        let authority = report
            .runtime_authority
            .as_ref()
            .map(|authority| {
                format!(
                    "{} / {} / {}",
                    authority.command_session_authority,
                    authority.install_launch_evidence_authority,
                    authority.studio_role
                )
            })
            .unwrap_or_else(|| "none".to_string());
        return format!(
            "shell handoff {status}; issue {issue}\n  graph: {}\n  output: {}\n  consumer: {}\n  target: {}\n  args: {}\n  authority: {}",
            report.graph_id,
            output_dir.display(),
            report.consumer_id,
            shell_target_kind_label(report.target_kind),
            args,
            authority
        );
    }
    format!(
        "shell handoff {status}; issue {issue}\n  graph: {}\n  output: {}\n  target: {}\n  message: {}",
        report.graph_id,
        output_dir.display(),
        shell_target_kind_label(report.target_kind),
        report.message
    )
}

fn shell_handoff_readiness_status(
    report: &StudioShellHandoffReadinessReport,
    bundle_root: &Path,
) -> String {
    let status = validation_status_label(report.status);
    let target_rows = report
        .target_summaries
        .iter()
        .map(|summary| {
            let ready_path = summary
                .template_index_paths
                .first()
                .map(|path| format!("; templates {path}"))
                .unwrap_or_default();
            let missing_path = summary
                .missing_bundle_dirs
                .first()
                .map(|path| format!("; missing bundle {path}"))
                .unwrap_or_default();
            format!(
                "{}: ready {}/{}; missing {}; packages {}; modules {}; shells {}{}{}",
                shell_target_kind_label(summary.target_kind),
                summary.ready_count,
                summary.graph_count,
                summary.missing_bundle_count,
                summary.package_count,
                summary.module_count,
                summary.operator_shell_count,
                ready_path,
                missing_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = validation_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] profile {}; packages {}; modules {}; shell {}; -> {} / {}; issue {}",
                entry.graph_id,
                shell_target_kind_label(entry.target_kind),
                entry.target_host_profile,
                entry.package_count,
                entry.module_count,
                entry.operator_shell_count,
                entry.consumer_id,
                entry_status,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    format!(
        "handoff readiness {status}; ready {}/{}; failed {}; missing {}\n  root: {}\n  targets:\n  {}\n  graphs:\n  {}",
        report.ready_count,
        report.graph_count,
        report.failed_count,
        report.missing_bundle_count,
        bundle_root.display(),
        if target_rows.is_empty() {
            "none".to_string()
        } else {
            target_rows
        },
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_runbook_status(report: &StudioShellRunbookReport, bundle_root: &Path) -> String {
    let status = shell_runbook_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let target_rows = report
        .target_summaries
        .iter()
        .map(|target| {
            let consumers = if target.consumer_ids.is_empty() {
                "none".to_string()
            } else {
                target.consumer_ids.join(", ")
            };
            let owners = if target.responsible_owners.is_empty() {
                "none".to_string()
            } else {
                target.responsible_owners.join(", ")
            };
            let routes = if target.runtime_route_kinds.is_empty() {
                "none".to_string()
            } else {
                target.runtime_route_kinds.join(", ")
            };
            let issues = if target.issue_codes.is_empty() {
                "none".to_string()
            } else {
                target.issue_codes.join(", ")
            };
            format!(
                "{}: ready {}; blocked {}; rejected {}; consumers {}; owners {}; routes {}; issues {}",
                shell_target_kind_label(target.target_kind),
                target.ready_count,
                target.blocked_count,
                target.rejected_count,
                consumers,
                owners,
                routes,
                issues
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_runbook_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let install = entry.host_routes.install_route.as_deref().unwrap_or("none");
            let launch = entry.host_routes.launch_route.as_deref().unwrap_or("none");
            let bridge = entry.host_routes.command_bridge.as_deref().unwrap_or("none");
            let evidence = entry
                .host_routes
                .evidence_pull_route
                .as_deref()
                .unwrap_or("none");
            let cli = if entry.cli_request.is_empty() {
                "none".to_string()
            } else {
                entry.cli_request.join(" ")
            };
            format!(
                "{} [{}] target {}; owner {}; action {}; policy {}; route {}; install {}; launch {}; bridge {}; evidence {}; cli {}; issue {}",
                entry.graph_id,
                entry_status,
                shell_target_kind_label(entry.target_kind),
                entry.responsible_owner,
                entry.next_required_action,
                entry.execution_policy,
                entry.runtime_route_kind,
                install,
                launch,
                bridge,
                evidence,
                cli,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "shell runbook {status}; ready {}; blocked {}; rejected {}; issue {issue}\n  root: {}\n  bundle root: {}\n  prohibited: {}\n  targets:\n  {}\n  entries:\n  {}",
        report.ready_count,
        report.blocked_count,
        report.rejected_count,
        report.bundle_root,
        bundle_root.display(),
        prohibited,
        if target_rows.is_empty() {
            "none".to_string()
        } else {
            target_rows
        },
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_export_package_status(
    report: &StudioShellExportPackageReport,
    bundle_root: &Path,
) -> String {
    let status = shell_export_package_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let target_rows = report
        .target_summaries
        .iter()
        .map(|target| {
            let consumers = if target.consumer_ids.is_empty() {
                "none".to_string()
            } else {
                target.consumer_ids.join(", ")
            };
            let owners = if target.responsible_owners.is_empty() {
                "none".to_string()
            } else {
                target.responsible_owners.join(", ")
            };
            let issues = if target.issue_codes.is_empty() {
                "none".to_string()
            } else {
                target.issue_codes.join(", ")
            };
            format!(
                "{}: ready {}; blocked {}; rejected {}; descriptors {}; templates {}; consumers {}; owners {}; issues {}",
                shell_target_kind_label(target.target_kind),
                target.ready_count,
                target.blocked_count,
                target.rejected_count,
                target.descriptor_count,
                target.template_manifest_count,
                consumers,
                owners,
                issues
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_export_package_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let descriptor = entry
                .descriptor
                .as_ref()
                .map(|descriptor| descriptor.descriptor_id.as_str())
                .unwrap_or("none");
            let template = entry
                .template_manifest
                .as_ref()
                .map(|template| template.template_id.as_str())
                .unwrap_or("none");
            let cli = if entry.runbook_cli_request.is_empty() {
                "none".to_string()
            } else {
                entry.runbook_cli_request.join(" ")
            };
            format!(
                "{} [{}] target {}; owner {}; action {}; policy {}; descriptor {}; template {}; cli {}; issue {}",
                entry.graph_id,
                entry_status,
                shell_target_kind_label(entry.target_kind),
                entry.responsible_owner,
                entry.next_required_action,
                entry.execution_policy,
                descriptor,
                template,
                cli,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "shell export package {status}; ready {}; blocked {}; rejected {}; descriptors {}; templates {}; issue {issue}\n  package: {}\n  owner: {}; policy: {}\n  authority: command {}; host {}; studio {}\n  root: {}\n  bundle root: {}\n  prohibited: {}\n  targets:\n  {}\n  entries:\n  {}",
        report.ready_count,
        report.blocked_count,
        report.rejected_count,
        report.descriptor_count,
        report.template_manifest_count,
        report.package_id,
        report.review_owner,
        report.execution_policy,
        report.command_session_authority,
        report.install_launch_evidence_authority,
        report.studio_role,
        report.bundle_root,
        bundle_root.display(),
        prohibited,
        if target_rows.is_empty() {
            "none".to_string()
        } else {
            target_rows
        },
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_export_package_baseline_status(
    report: &StudioShellExportPackageReport,
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    package_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection =
        summarize_shell_export_package_baseline_index_selection(index, Some(index_path), None);
    format!(
        "export package baseline written\n  baseline: {} ({})\n  identity: {}\n  package: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        package_path.display(),
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path),
        shell_export_package_status(report, bundle_root)
    )
}

fn shell_export_package_baseline_append_status(
    report: &StudioShellExportPackageReport,
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    package_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection = summarize_shell_export_package_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "export package baseline archived\n  baseline: {} ({})\n  identity: {}\n  package: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        package_path.display(),
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path),
        shell_export_package_status(report, bundle_root)
    )
}

fn shell_export_package_baseline_index_status(
    index: &StudioShellExportPackageBaselineIndex,
    index_path: &Path,
) -> String {
    let default = index.default_baseline_id.as_deref().unwrap_or("none");
    let projects = if index.project_ids.is_empty() {
        "none".to_string()
    } else {
        index.project_ids.join(", ")
    };
    let packages = if index.package_ids.is_empty() {
        "none".to_string()
    } else {
        index.package_ids.join(", ")
    };
    let manifests = if index.manifest_ids.is_empty() {
        "none".to_string()
    } else {
        index.manifest_ids.join(", ")
    };
    let rows = index
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let status = shell_export_package_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry.baseline_manifest_path.as_deref().unwrap_or("unknown");
            format!(
                "{} [{}] project {} rev {}; ready {}; blocked {}; rejected {}; descriptors {}; templates {}; package {}; manifest {}; issue {}",
                entry.baseline_id,
                status,
                entry.project_id,
                entry.project_revision,
                entry.ready_count,
                entry.blocked_count,
                entry.rejected_count,
                entry.descriptor_count,
                entry.template_manifest_count,
                entry.package_path,
                manifest_path,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "export package baseline index slots {}; default {}; ready {}; blocked {}; rejected {}\n  index: {}\n  projects: {}\n  packages: {}\n  manifests: {}\n  entries:\n  {}",
        index.baseline_count,
        default,
        index.ready_baseline_count,
        index.blocked_baseline_count,
        index.rejected_baseline_count,
        index_path.display(),
        projects,
        packages,
        manifests,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_export_package_baseline_selection_status(
    report: &StudioShellExportPackageBaselineSelectionReport,
) -> String {
    let status = shell_export_package_baseline_selection_status_label(report.status);
    let requested = report.requested_baseline_id.as_deref().unwrap_or("none");
    let default = report.default_baseline_id.as_deref().unwrap_or("none");
    let selected = report.selected_baseline_id.as_deref().unwrap_or("none");
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let index_path = report.index_path.as_deref().unwrap_or("not saved");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_export_package_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry.baseline_manifest_path.as_deref().unwrap_or("unknown");
            let selected_flag = if entry.selected { "yes" } else { "no" };
            let default_flag = if entry.default { "yes" } else { "no" };
            format!(
                "{} [{}] selected {}; default {}; ready {}; blocked {}; rejected {}; descriptors {}; templates {}; package {}; manifest {}; issue {}",
                entry.baseline_id,
                entry_status,
                selected_flag,
                default_flag,
                entry.ready_count,
                entry.blocked_count,
                entry.rejected_count,
                entry.descriptor_count,
                entry.template_manifest_count,
                entry.package_path,
                manifest_path,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "export package baseline selection {status}; requested {requested}; default {default}; selected {selected}; slots {}; ready {}; blocked {}; rejected {}; issue {issue}\n  index: {}\n  entries:\n  {}",
        report.baseline_count,
        report.ready_baseline_count,
        report.blocked_baseline_count,
        report.rejected_baseline_count,
        index_path,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_export_package_baseline_summary_status(
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection =
        summarize_shell_export_package_baseline_index_selection(index, Some(index_path), None);
    let status = shell_export_package_status_label(baseline.status);
    let issue = baseline.issue_code.as_deref().unwrap_or("none");
    format!(
        "export package baseline summary {status}; baseline {} ({}); project {} rev {}; package {}; manifest {}; ready {}; blocked {}; rejected {}; descriptors {}; templates {}; runbook entries {}; targets {}; issue {issue}\n  identity: {}\n  package review: {}\n  authority: command {}; host {}; studio {}; policy {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline.project_id,
        baseline.project_revision,
        baseline.package_id,
        baseline.manifest_id,
        baseline.ready_count,
        baseline.blocked_count,
        baseline.rejected_count,
        baseline.descriptor_count,
        baseline.template_manifest_count,
        baseline.runbook_entry_count,
        baseline.target_count,
        baseline_path.display(),
        baseline.package_path,
        baseline.command_session_authority,
        baseline.install_launch_evidence_authority,
        baseline.studio_role,
        baseline.execution_policy,
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path)
    )
}

fn shell_export_package_baseline_promote_status(
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_export_package_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "export package baseline default promoted\n  baseline: {} ({})\n  identity: {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path)
    )
}

fn shell_export_package_baseline_select_status(
    baseline: &StudioShellExportPackageBaselineManifest,
    index: &StudioShellExportPackageBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_export_package_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "export package baseline default selected\n  baseline: {} ({})\n  identity: {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        shell_export_package_baseline_selection_status(&selection),
        shell_export_package_baseline_index_status(index, index_path)
    )
}

fn shell_export_package_comparison_status(
    report: &StudioShellExportPackageComparisonReport,
    baseline_path: &Path,
    bundle_root: &Path,
) -> String {
    let status = shell_export_package_comparison_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let baseline_id = report.baseline_id.as_deref().unwrap_or("unnamed");
    let baseline_label = report.baseline_label.as_deref().unwrap_or("unlabeled");
    let baseline_package = report.baseline_package_path.as_deref().unwrap_or("unknown");
    let baseline_index_path = report.baseline_index_path.as_deref().unwrap_or("not used");
    let baseline_index_default = report
        .baseline_index_default_baseline_id
        .as_deref()
        .unwrap_or("none");
    let baseline_index_selected = report
        .baseline_index_selected_baseline_id
        .as_deref()
        .unwrap_or("none");
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let target = entry
                .target_kind
                .map(shell_target_kind_label)
                .unwrap_or("unknown");
            let baseline = entry
                .baseline_status
                .map(shell_export_package_status_label)
                .unwrap_or("missing");
            let candidate = entry
                .candidate_status
                .map(shell_export_package_status_label)
                .unwrap_or("missing");
            let change = shell_export_package_comparison_change_label(entry.change);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] {baseline}->{candidate}; change {change}; delta {}; descriptor {}->{}; template {}->{}; cli {}->{}; issue {}",
                entry.graph_id,
                target,
                entry.score_delta,
                present_label(entry.baseline_descriptor_present),
                present_label(entry.candidate_descriptor_present),
                present_label(entry.baseline_template_manifest_present),
                present_label(entry.candidate_template_manifest_present),
                present_label(entry.baseline_runbook_cli_request_present),
                present_label(entry.candidate_runbook_cli_request_present),
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "export package comparison {status}; ready {}->{}, delta {}; blocked {}->{}, delta {}; rejected {}->{}, delta {}; descriptors {}->{}, delta {}; templates {}->{}, delta {}; runbook entries {}->{}, delta {}; issue {issue}\n  baseline: {} ({})\n  baseline source: {} rev {}; package {}; manifest {}\n  candidate: {} rev {}; package {}; manifest {}\n  baseline identity: {}\n  baseline package: {}\n  baseline index: {}; default {}; selected {}\n  current root: {}\n  checks: {}; failed {}\n  entries:\n  {}",
        report.baseline_ready_count,
        report.candidate_ready_count,
        report.ready_delta,
        report.baseline_blocked_count,
        report.candidate_blocked_count,
        report.blocked_delta,
        report.baseline_rejected_count,
        report.candidate_rejected_count,
        report.rejected_delta,
        report.baseline_descriptor_count,
        report.candidate_descriptor_count,
        report.descriptor_delta,
        report.baseline_template_manifest_count,
        report.candidate_template_manifest_count,
        report.template_manifest_delta,
        report.baseline_runbook_entry_count,
        report.candidate_runbook_entry_count,
        report.runbook_entry_delta,
        baseline_id,
        baseline_label,
        report.baseline_project_id,
        report.baseline_project_revision,
        report.baseline_package_id,
        report.baseline_manifest_id,
        report.candidate_project_id,
        report.candidate_project_revision,
        report.candidate_package_id,
        report.candidate_manifest_id,
        baseline_path.display(),
        baseline_package,
        baseline_index_path,
        baseline_index_default,
        baseline_index_selected,
        bundle_root.display(),
        report.checks.len(),
        failed_checks,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_handoff_manifest_status(
    manifest: &StudioShellHandoffManifest,
    output_path: &Path,
) -> String {
    let status = validation_status_label(manifest.status);
    let target_rows = manifest
        .targets
        .iter()
        .map(|target| {
            let ready_path = target
                .ready_bundle_dirs
                .first()
                .map(|path| format!("; ready {path}"))
                .unwrap_or_default();
            let missing_path = target
                .missing_bundle_dirs
                .first()
                .map(|path| format!("; missing {path}"))
                .unwrap_or_default();
            format!(
                "{}: ready {}/{}; failed {}; missing {}; templates {}{}{}",
                shell_target_kind_label(target.target_kind),
                target.ready_count,
                target.graph_count,
                target.failed_count,
                target.missing_bundle_count,
                target.template_index_paths.len(),
                ready_path,
                missing_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    format!(
        "handoff manifest {status}; ready {}/{}; failed {}; missing {}\n  path: {}\n  authority: {} / {} / {}\n  targets:\n  {}",
        manifest.ready_count,
        manifest.graph_count,
        manifest.failed_count,
        manifest.missing_bundle_count,
        output_path.display(),
        manifest.runtime_authority.command_session_authority,
        manifest.runtime_authority.install_launch_evidence_authority,
        manifest.runtime_authority.studio_role,
        if target_rows.is_empty() {
            "none".to_string()
        } else {
            target_rows
        }
    )
}

fn shell_handoff_acceptance_status(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    bundle_root: &Path,
) -> String {
    let status = shell_handoff_acceptance_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let failed_intake_checks = report
        .intake_checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_handoff_acceptance_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let failed_checks = entry
                .checks
                .iter()
                .filter(|check| check.status == StudioValidationStatus::Fail)
                .count();
            format!(
                "{} [{}] -> {} / {}; action {}; route {}; owners {}; failed {}; issue {}",
                entry.graph_id,
                shell_target_kind_label(entry.target_kind),
                entry.consumer_id,
                entry_status,
                entry.next_required_action,
                entry.runtime_route_kind,
                shell_handoff_acceptance_owner_summary(report, &entry.graph_id),
                failed_checks,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    format!(
        "handoff acceptance {status}; ready {}; blocked {}; rejected {}; issue {issue}\n  root: {}\n  prohibited: {}\n  intake checks: {}; failed {}\n  entries:\n  {}",
        report.ready_count,
        report.blocked_count,
        report.rejected_count,
        bundle_root.display(),
        prohibited,
        report.intake_checks.len(),
        failed_intake_checks,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_handoff_acceptance_baseline_status(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    checklist_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection =
        summarize_shell_handoff_acceptance_baseline_index_selection(index, Some(index_path), None);
    format!(
        "acceptance baseline written\n  baseline: {} ({})\n  identity: {}\n  checklist: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        checklist_path.display(),
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path),
        shell_handoff_acceptance_status(report, bundle_root)
    )
}

fn shell_handoff_acceptance_baseline_append_status(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    checklist_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "acceptance baseline archived\n  baseline: {} ({})\n  identity: {}\n  checklist: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        checklist_path.display(),
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path),
        shell_handoff_acceptance_status(report, bundle_root)
    )
}

fn shell_handoff_acceptance_baseline_index_status(
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    index_path: &Path,
) -> String {
    let default = index.default_baseline_id.as_deref().unwrap_or("none");
    let projects = if index.project_ids.is_empty() {
        "none".to_string()
    } else {
        index.project_ids.join(", ")
    };
    let manifests = if index.manifest_ids.is_empty() {
        "none".to_string()
    } else {
        index.manifest_ids.join(", ")
    };
    let rows = index
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let status = shell_handoff_acceptance_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry.baseline_manifest_path.as_deref().unwrap_or("unknown");
            format!(
                "{} [{}] project {} rev {}; ready {}; blocked {}; rejected {}; manifest {}; issue {}",
                entry.baseline_id,
                status,
                entry.project_id,
                entry.project_revision,
                entry.ready_count,
                entry.blocked_count,
                entry.rejected_count,
                manifest_path,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "baseline index slots {}; default {}; ready {}; blocked {}; rejected {}\n  index: {}\n  projects: {}\n  manifests: {}\n  entries:\n  {}",
        index.baseline_count,
        default,
        index.ready_baseline_count,
        index.blocked_baseline_count,
        index.rejected_baseline_count,
        index_path.display(),
        projects,
        manifests,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_handoff_acceptance_baseline_selection_status(
    report: &StudioShellHandoffAcceptanceBaselineSelectionReport,
) -> String {
    let status = shell_handoff_acceptance_baseline_selection_status_label(report.status);
    let requested = report.requested_baseline_id.as_deref().unwrap_or("none");
    let default = report.default_baseline_id.as_deref().unwrap_or("none");
    let selected = report.selected_baseline_id.as_deref().unwrap_or("none");
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let index_path = report.index_path.as_deref().unwrap_or("not saved");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_handoff_acceptance_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry.baseline_manifest_path.as_deref().unwrap_or("unknown");
            let selected_flag = if entry.selected { "yes" } else { "no" };
            let default_flag = if entry.default { "yes" } else { "no" };
            format!(
                "{} [{}] selected {}; default {}; ready {}; blocked {}; rejected {}; manifest {}; issue {}",
                entry.baseline_id,
                entry_status,
                selected_flag,
                default_flag,
                entry.ready_count,
                entry.blocked_count,
                entry.rejected_count,
                manifest_path,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "baseline selection {status}; requested {requested}; default {default}; selected {selected}; slots {}; ready {}; blocked {}; rejected {}; issue {issue}\n  index: {}\n  entries:\n  {}",
        report.baseline_count,
        report.ready_baseline_count,
        report.blocked_baseline_count,
        report.rejected_baseline_count,
        index_path,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_handoff_acceptance_summary_status(
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection =
        summarize_shell_handoff_acceptance_baseline_index_selection(index, Some(index_path), None);
    let summary = &baseline.summary;
    let status = shell_handoff_acceptance_status_label(summary.status);
    let issue = summary.issue_code.as_deref().unwrap_or("none");
    let target_rows = summary
        .targets
        .iter()
        .map(|target| {
            let consumers = if target.consumer_ids.is_empty() {
                "none".to_string()
            } else {
                target.consumer_ids.join(", ")
            };
            let routes = if target.route_kinds.is_empty() {
                "none".to_string()
            } else {
                target.route_kinds.join(", ")
            };
            let issues = if target.issue_codes.is_empty() {
                "none".to_string()
            } else {
                target.issue_codes.join(", ")
            };
            format!(
                "{}: ready {}/{}; blocked {}; rejected {}; consumers {}; routes {}; issues {}",
                shell_target_kind_label(target.target_kind),
                target.ready_count,
                target.graph_count,
                target.blocked_count,
                target.rejected_count,
                consumers,
                routes,
                issues
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    format!(
        "acceptance baseline summary {status}; baseline {} ({}); project {} rev {}; manifest {}; ready {}; blocked {}; rejected {}; entries {}; issue {issue}\n  identity: {}\n  checklist: {}\n  intake checks: {}; failed {}\n  targets:\n  {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        summary.project_id,
        summary.project_revision,
        summary.manifest_id,
        summary.ready_count,
        summary.blocked_count,
        summary.rejected_count,
        summary.entry_count,
        baseline_path.display(),
        baseline.checklist_path,
        summary.intake_check_count,
        summary.failed_intake_check_count,
        if target_rows.is_empty() {
            "none".to_string()
        } else {
            target_rows
        },
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path)
    )
}

fn shell_handoff_acceptance_baseline_promote_status(
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "acceptance baseline default promoted\n  baseline: {} ({})\n  identity: {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path)
    )
}

fn shell_handoff_acceptance_baseline_select_status(
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "acceptance baseline default selected\n  baseline: {} ({})\n  identity: {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path)
    )
}

fn shell_handoff_acceptance_baseline_selection_status_label(
    status: StudioShellHandoffAcceptanceBaselineSelectionStatus,
) -> &'static str {
    match status {
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected => "selected",
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Missing => "missing",
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Empty => "empty",
    }
}

fn shell_runbook_status_label(status: StudioShellRunbookStatus) -> &'static str {
    match status {
        StudioShellRunbookStatus::Ready => "ready",
        StudioShellRunbookStatus::Blocked => "blocked",
        StudioShellRunbookStatus::Rejected => "rejected",
    }
}

fn shell_export_package_status_label(status: StudioShellExportPackageStatus) -> &'static str {
    match status {
        StudioShellExportPackageStatus::Ready => "ready",
        StudioShellExportPackageStatus::Blocked => "blocked",
        StudioShellExportPackageStatus::Rejected => "rejected",
    }
}

fn shell_export_package_baseline_selection_status_label(
    status: StudioShellExportPackageBaselineSelectionStatus,
) -> &'static str {
    match status {
        StudioShellExportPackageBaselineSelectionStatus::Selected => "selected",
        StudioShellExportPackageBaselineSelectionStatus::Missing => "missing",
        StudioShellExportPackageBaselineSelectionStatus::Empty => "empty",
    }
}

fn shell_export_package_comparison_status_label(
    status: StudioShellExportPackageComparisonStatus,
) -> &'static str {
    match status {
        StudioShellExportPackageComparisonStatus::Improved => "improved",
        StudioShellExportPackageComparisonStatus::Unchanged => "unchanged",
        StudioShellExportPackageComparisonStatus::Regressed => "regressed",
        StudioShellExportPackageComparisonStatus::Incomparable => "incomparable",
    }
}

fn shell_export_package_comparison_change_label(
    change: StudioShellExportPackageComparisonChange,
) -> &'static str {
    match change {
        StudioShellExportPackageComparisonChange::Added => "added",
        StudioShellExportPackageComparisonChange::Removed => "removed",
        StudioShellExportPackageComparisonChange::Improved => "improved",
        StudioShellExportPackageComparisonChange::Unchanged => "unchanged",
        StudioShellExportPackageComparisonChange::Regressed => "regressed",
        StudioShellExportPackageComparisonChange::Changed => "changed",
    }
}

fn present_label(present: bool) -> &'static str {
    if present {
        "present"
    } else {
        "missing"
    }
}

fn shell_handoff_acceptance_owner_summary(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    graph_id: &str,
) -> String {
    let Some(entry) = report
        .entries
        .iter()
        .find(|entry| entry.graph_id == graph_id)
    else {
        return "none".to_string();
    };
    ["rusty.manifold", "rusty.hostess", "rusty.studio"]
        .iter()
        .map(|owner| {
            let owner_checks = entry
                .checks
                .iter()
                .filter(|check| check.owner.as_str() == *owner)
                .collect::<Vec<_>>();
            let status = if owner_checks.is_empty() {
                "none"
            } else if owner_checks
                .iter()
                .any(|check| check.status == StudioValidationStatus::Fail)
            {
                "fail"
            } else {
                "pass"
            };
            format!("{owner}:{status}")
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn shell_handoff_acceptance_comparison_status(
    report: &StudioShellHandoffAcceptanceComparisonReport,
    baseline_path: &Path,
    bundle_root: &Path,
) -> String {
    let status = shell_handoff_acceptance_comparison_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let baseline_id = report.baseline_id.as_deref().unwrap_or("unnamed");
    let baseline_label = report.baseline_label.as_deref().unwrap_or("unlabeled");
    let baseline_checklist = report
        .baseline_checklist_path
        .as_deref()
        .unwrap_or("unknown");
    let baseline_index_path = report.baseline_index_path.as_deref().unwrap_or("not used");
    let baseline_index_default = report
        .baseline_index_default_baseline_id
        .as_deref()
        .unwrap_or("none");
    let baseline_index_selected = report
        .baseline_index_selected_baseline_id
        .as_deref()
        .unwrap_or("none");
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let target = entry
                .target_kind
                .map(shell_target_kind_label)
                .unwrap_or("unknown");
            let baseline = entry
                .baseline_status
                .map(shell_handoff_acceptance_status_label)
                .unwrap_or("missing");
            let candidate = entry
                .candidate_status
                .map(shell_handoff_acceptance_status_label)
                .unwrap_or("missing");
            let change = shell_handoff_acceptance_comparison_change_label(entry.change);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let route = entry
                .candidate_route_kind
                .as_deref()
                .or(entry.baseline_route_kind.as_deref())
                .unwrap_or("unknown");
            format!(
                "{} [{}] {baseline}->{candidate}; change {change}; delta {}; route {}; issue {}",
                entry.graph_id, target, entry.score_delta, route, issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "handoff acceptance comparison {status}; ready {}->{}, delta {}; blocked {}->{}, delta {}; rejected {}->{}, delta {}; issue {issue}\n  baseline: {} ({})\n  baseline source: {} rev {}; manifest {}\n  candidate: {} rev {}; manifest {}\n  baseline identity: {}\n  baseline checklist: {}\n  baseline index: {}; default {}; selected {}\n  current root: {}\n  checks: {}; failed {}\n  entries:\n  {}",
        report.baseline_ready_count,
        report.candidate_ready_count,
        report.ready_delta,
        report.baseline_blocked_count,
        report.candidate_blocked_count,
        report.blocked_delta,
        report.baseline_rejected_count,
        report.candidate_rejected_count,
        report.rejected_delta,
        baseline_id,
        baseline_label,
        report.baseline_project_id,
        report.baseline_project_revision,
        report.baseline_manifest_id,
        report.candidate_project_id,
        report.candidate_project_revision,
        report.candidate_manifest_id,
        baseline_path.display(),
        baseline_checklist,
        baseline_index_path,
        baseline_index_default,
        baseline_index_selected,
        bundle_root.display(),
        report.checks.len(),
        failed_checks,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_release_candidate_review_status(
    report: &StudioShellReleaseCandidateReviewReport,
    output_path: &Path,
) -> String {
    let status = shell_release_candidate_review_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let handoff_status = validation_status_label(report.handoff_status);
    let acceptance_selection = shell_handoff_acceptance_baseline_selection_status_label(
        report.acceptance_baseline_selection.status,
    );
    let acceptance_selected = report
        .acceptance_baseline_selection
        .selected_baseline_id
        .as_deref()
        .unwrap_or("none");
    let acceptance_comparison = report
        .acceptance_comparison
        .as_ref()
        .map(|comparison| shell_handoff_acceptance_comparison_status_label(comparison.status))
        .unwrap_or("missing");
    let export_package_selection = shell_export_package_baseline_selection_status_label(
        report.export_package_baseline_selection.status,
    );
    let export_package_selected = report
        .export_package_baseline_selection
        .selected_baseline_id
        .as_deref()
        .unwrap_or("none");
    let export_package_comparison = report
        .export_package_comparison
        .as_ref()
        .map(|comparison| shell_export_package_comparison_status_label(comparison.status))
        .unwrap_or("missing");
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let first_issue = report
        .checks
        .iter()
        .find(|check| check.status == StudioValidationStatus::Fail)
        .and_then(|check| check.issue_code.as_deref())
        .unwrap_or("none");
    format!(
        "shell release candidate review {status}; issue {issue}\n  review: {}\n  manifest: {} rev {}; handoff {handoff_status}; ready {}; failed {}; missing bundles {}\n  acceptance baseline: {acceptance_selection}; selected {acceptance_selected}; comparison {acceptance_comparison}\n  export package baseline: {export_package_selection}; selected {export_package_selected}; comparison {export_package_comparison}\n  authority: command {}; host {}; studio {}; policy {}; owner {}\n  checks: {}; failed {}; first issue {}\n  prohibited: {}",
        output_path.display(),
        report.project_id,
        report.project_revision,
        report.handoff_ready_count,
        report.handoff_failed_count,
        report.handoff_missing_bundle_count,
        report.command_session_authority,
        report.install_launch_evidence_authority,
        report.studio_role,
        report.execution_policy,
        report.review_owner,
        report.checks.len(),
        failed_checks,
        first_issue,
        if report.prohibited_actions.is_empty() {
            "none".to_string()
        } else {
            report.prohibited_actions.join(", ")
        }
    )
}

fn shell_release_candidate_review_manifest_status(
    review: &StudioShellReleaseCandidateReviewReport,
    candidate: &StudioShellReleaseCandidateReviewManifest,
    index: &StudioShellReleaseCandidateReviewIndex,
    review_path: &Path,
    candidate_path: &Path,
    index_path: &Path,
) -> String {
    let selection =
        summarize_shell_release_candidate_review_index_selection(index, Some(index_path), None);
    format!(
        "release candidate written\n  candidate: {} ({})\n  identity: {}\n  review artifact: {}\n{}\n{}\n{}",
        candidate.candidate_id,
        candidate.label,
        candidate_path.display(),
        review_path.display(),
        shell_release_candidate_review_selection_status(&selection),
        shell_release_candidate_review_index_status(index, index_path),
        shell_release_candidate_review_status(review, review_path)
    )
}

fn shell_release_candidate_review_manifest_append_status(
    review: &StudioShellReleaseCandidateReviewReport,
    candidate: &StudioShellReleaseCandidateReviewManifest,
    index: &StudioShellReleaseCandidateReviewIndex,
    review_path: &Path,
    candidate_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_release_candidate_review_index_selection(
        index,
        Some(index_path),
        Some(&candidate.candidate_id),
    );
    format!(
        "release candidate archived\n  candidate: {} ({})\n  identity: {}\n  review artifact: {}\n{}\n{}\n{}",
        candidate.candidate_id,
        candidate.label,
        candidate_path.display(),
        review_path.display(),
        shell_release_candidate_review_selection_status(&selection),
        shell_release_candidate_review_index_status(index, index_path),
        shell_release_candidate_review_status(review, review_path)
    )
}

fn shell_release_candidate_review_index_status(
    index: &StudioShellReleaseCandidateReviewIndex,
    index_path: &Path,
) -> String {
    let default = index.default_candidate_id.as_deref().unwrap_or("none");
    let projects = if index.project_ids.is_empty() {
        "none".to_string()
    } else {
        index.project_ids.join(", ")
    };
    let manifests = if index.manifest_ids.is_empty() {
        "none".to_string()
    } else {
        index.manifest_ids.join(", ")
    };
    let rows = index
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let status = shell_release_candidate_review_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry
                .candidate_manifest_path
                .as_deref()
                .unwrap_or("unknown");
            let acceptance = entry
                .acceptance_comparison_status
                .map(shell_handoff_acceptance_comparison_status_label)
                .unwrap_or("missing");
            let export_package = entry
                .export_package_comparison_status
                .map(shell_export_package_comparison_status_label)
                .unwrap_or("missing");
            format!(
                "{} [{}] project {} rev {}; handoff ready {}; failed {}; missing {}; acceptance {}; package {}; checks failed {}; manifest {}; issue {}",
                entry.candidate_id,
                status,
                entry.project_id,
                entry.project_revision,
                entry.handoff_ready_count,
                entry.handoff_failed_count,
                entry.handoff_missing_bundle_count,
                acceptance,
                export_package,
                entry.failed_check_count,
                manifest_path,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "release candidate index slots {}; default {}; ready {}; blocked {}; rejected {}\n  index: {}\n  projects: {}\n  manifests: {}\n  entries:\n  {}",
        index.candidate_count,
        default,
        index.ready_candidate_count,
        index.blocked_candidate_count,
        index.rejected_candidate_count,
        index_path.display(),
        projects,
        manifests,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_release_candidate_review_selection_status(
    report: &StudioShellReleaseCandidateReviewSelectionReport,
) -> String {
    let status = shell_release_candidate_review_selection_status_label(report.status);
    let requested = report.requested_candidate_id.as_deref().unwrap_or("none");
    let default = report.default_candidate_id.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let index_path = report.index_path.as_deref().unwrap_or("not saved");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_release_candidate_review_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry
                .candidate_manifest_path
                .as_deref()
                .unwrap_or("unknown");
            let selected_flag = if entry.selected { "yes" } else { "no" };
            let default_flag = if entry.default { "yes" } else { "no" };
            let acceptance = entry
                .acceptance_comparison_status
                .map(shell_handoff_acceptance_comparison_status_label)
                .unwrap_or("missing");
            let export_package = entry
                .export_package_comparison_status
                .map(shell_export_package_comparison_status_label)
                .unwrap_or("missing");
            format!(
                "{} [{}] selected {}; default {}; acceptance {}; package {}; checks failed {}; manifest {}; issue {}",
                entry.candidate_id,
                entry_status,
                selected_flag,
                default_flag,
                acceptance,
                export_package,
                entry.failed_check_count,
                manifest_path,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "release candidate selection {status}; requested {requested}; default {default}; selected {selected}; slots {}; ready {}; blocked {}; rejected {}; issue {issue}\n  index: {}\n  entries:\n  {}",
        report.candidate_count,
        report.ready_candidate_count,
        report.blocked_candidate_count,
        report.rejected_candidate_count,
        index_path,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

fn shell_hostess_handoff_package_status(
    report: &StudioShellHostessHandoffPackageReport,
    output_path: &Path,
) -> String {
    let status = shell_hostess_handoff_package_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let candidate_path = report
        .candidate_manifest_path
        .as_deref()
        .unwrap_or("unknown");
    let review_path = report.review_path.as_deref().unwrap_or("unknown");
    let handoff_path = report.handoff_manifest_path.as_deref().unwrap_or("unknown");
    let acceptance = report
        .acceptance_comparison_status
        .map(shell_handoff_acceptance_comparison_status_label)
        .unwrap_or("missing");
    let export_package = report
        .export_package_comparison_status
        .map(shell_export_package_comparison_status_label)
        .unwrap_or("missing");
    let actions = report
        .required_owner_actions
        .iter()
        .map(|action| {
            let action_status = shell_hostess_handoff_package_action_status_label(action.status);
            let issue = action.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] owner {}; source {}; next {}; prohibited in Studio {}; issue {}",
                action.action_id,
                action_status,
                action.owner,
                action.source,
                action.next_required_action,
                if action.prohibited_in_studio {
                    "yes"
                } else {
                    "no"
                },
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    format!(
        "shell Hostess handoff package {status}; selected {selected}; issue {issue}\n  package: {}\n  candidate: {}\n  review: {}\n  handoff manifest: {}\n  project: {} rev {}\n  handoff ready {}; failed {}; missing {}; acceptance {}; export package {}\n  authority: command {}; host {}; studio {}; policy {}; owner {}\n  actions:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        candidate_path,
        review_path,
        handoff_path,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.handoff_ready_count,
        report.handoff_failed_count,
        report.handoff_missing_bundle_count,
        acceptance,
        export_package,
        report
            .command_session_authority
            .as_deref()
            .unwrap_or("unknown"),
        report
            .install_launch_evidence_authority
            .as_deref()
            .unwrap_or("unknown"),
        report.studio_role.as_deref().unwrap_or("unknown"),
        report.execution_policy,
        report.handoff_owner,
        if actions.is_empty() {
            "none".to_string()
        } else {
            actions
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}

fn shell_hostess_owner_intake_status(
    report: &StudioShellHostessOwnerIntakeReport,
    output_path: &Path,
) -> String {
    let status = shell_hostess_owner_intake_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let package_path = report.package_path.as_deref().unwrap_or("unknown");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let handoff_path = report.handoff_manifest_path.as_deref().unwrap_or("unknown");
    let assignments = report
        .assignments
        .iter()
        .map(|assignment| {
            let assignment_status =
                shell_hostess_owner_intake_assignment_status_label(assignment.status);
            let issue = assignment.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] owner {}; request {}; source {}; next {}; prohibited in Studio {}; issue {}",
                assignment.action_id,
                assignment_status,
                assignment.owner,
                assignment.request_kind,
                assignment.source,
                assignment.next_required_action,
                if assignment.prohibited_in_studio {
                    "yes"
                } else {
                    "no"
                },
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    format!(
        "shell Hostess owner intake {status}; selected {selected}; issue {issue}\n  intake: {}\n  package: {}\n  handoff manifest: {}\n  project: {} rev {}\n  assignments ready {}; blocked {}; Hostess ready {}; Manifold ready {}\n  authority: command {}; host {}; studio {}; policy {}; intake owner {}; handoff owner {}\n  assignments:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        package_path,
        handoff_path,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.ready_assignment_count,
        report.blocked_assignment_count,
        report.hostess_ready_action_count,
        report.manifold_ready_action_count,
        report
            .command_session_authority
            .as_deref()
            .unwrap_or("unknown"),
        report
            .install_launch_evidence_authority
            .as_deref()
            .unwrap_or("unknown"),
        report.studio_role.as_deref().unwrap_or("unknown"),
        report.execution_policy,
        report.intake_owner,
        report.handoff_owner,
        if assignments.is_empty() {
            "none".to_string()
        } else {
            assignments
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}

fn shell_hostess_staging_preview_status(
    report: &StudioShellHostessStagingPreviewManifest,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_preview_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let intake_path = report.intake_path.as_deref().unwrap_or("unknown");
    let package_path = report.package_path.as_deref().unwrap_or("unknown");
    let handoff_path = report.handoff_manifest_path.as_deref().unwrap_or("unknown");
    let groups = report
        .groups
        .iter()
        .map(|group| {
            let group_status = shell_hostess_staging_preview_group_status_label(group.status);
            let issue = group.issue_code.as_deref().unwrap_or("none");
            let target_kinds = if group.target_kinds.is_empty() {
                "none".to_string()
            } else {
                group.target_kinds.join(", ")
            };
            let graph_ids = if group.graph_ids.is_empty() {
                "none".to_string()
            } else {
                group.graph_ids.join(", ")
            };
            format!(
                "{} route {} [{}] owner {}; artifacts {}; targets {}; graphs {}; issue {}",
                group.action_id,
                group.route_kind,
                group_status,
                group.owner,
                group.expected_artifact_count,
                target_kinds,
                graph_ids,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    format!(
        "shell Hostess staging preview {status}; selected {selected}; issue {issue}\n  preview: {}\n  intake: {}\n  package: {}\n  handoff manifest: {}\n  project: {} rev {}\n  assignments ready {}; blocked {}; groups ready {}; blocked {}; artifacts {}\n  authority: command {}; host {}; studio {}; policy {}; staging owner {}\n  groups:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        intake_path,
        package_path,
        handoff_path,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.ready_assignment_count,
        report.blocked_assignment_count,
        report.ready_group_count,
        report.blocked_group_count,
        report.expected_artifact_count,
        report
            .command_session_authority
            .as_deref()
            .unwrap_or("unknown"),
        report
            .install_launch_evidence_authority
            .as_deref()
            .unwrap_or("unknown"),
        report.studio_role.as_deref().unwrap_or("unknown"),
        report.execution_policy,
        report.staging_owner,
        if groups.is_empty() {
            "none".to_string()
        } else {
            groups
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}

fn shell_release_candidate_review_manifest_summary_status(
    candidate: &StudioShellReleaseCandidateReviewManifest,
    index: &StudioShellReleaseCandidateReviewIndex,
    candidate_path: &Path,
    index_path: &Path,
) -> String {
    let selection =
        summarize_shell_release_candidate_review_index_selection(index, Some(index_path), None);
    let status = shell_release_candidate_review_status_label(candidate.status);
    let issue = candidate.issue_code.as_deref().unwrap_or("none");
    let acceptance = candidate
        .acceptance_comparison_status
        .map(shell_handoff_acceptance_comparison_status_label)
        .unwrap_or("missing");
    let export_package = candidate
        .export_package_comparison_status
        .map(shell_export_package_comparison_status_label)
        .unwrap_or("missing");
    format!(
        "release candidate summary {status}; candidate {} ({}); project {} rev {}; manifest {}; issue {issue}\n  identity: {}\n  review artifact: {}\n  handoff ready {}; failed {}; missing {}; acceptance {}; export package {}; checks {}; failed {}\n  authority: command {}; host {}; studio {}; policy {}; owner {}\n{}\n{}",
        candidate.candidate_id,
        candidate.label,
        candidate.project_id,
        candidate.project_revision,
        candidate.manifest_id,
        candidate_path.display(),
        candidate.review_path,
        candidate.handoff_ready_count,
        candidate.handoff_failed_count,
        candidate.handoff_missing_bundle_count,
        acceptance,
        export_package,
        candidate.check_count,
        candidate.failed_check_count,
        candidate.command_session_authority,
        candidate.install_launch_evidence_authority,
        candidate.studio_role,
        candidate.execution_policy,
        candidate.review_owner,
        shell_release_candidate_review_selection_status(&selection),
        shell_release_candidate_review_index_status(index, index_path)
    )
}

fn shell_release_candidate_review_manifest_promote_status(
    candidate: &StudioShellReleaseCandidateReviewManifest,
    index: &StudioShellReleaseCandidateReviewIndex,
    candidate_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_release_candidate_review_index_selection(
        index,
        Some(index_path),
        Some(&candidate.candidate_id),
    );
    format!(
        "release candidate default promoted\n  candidate: {} ({})\n  identity: {}\n{}\n{}",
        candidate.candidate_id,
        candidate.label,
        candidate_path.display(),
        shell_release_candidate_review_selection_status(&selection),
        shell_release_candidate_review_index_status(index, index_path)
    )
}

fn shell_release_candidate_review_manifest_select_status(
    candidate: &StudioShellReleaseCandidateReviewManifest,
    index: &StudioShellReleaseCandidateReviewIndex,
    candidate_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_release_candidate_review_index_selection(
        index,
        Some(index_path),
        Some(&candidate.candidate_id),
    );
    format!(
        "release candidate default selected\n  candidate: {} ({})\n  identity: {}\n{}\n{}",
        candidate.candidate_id,
        candidate.label,
        candidate_path.display(),
        shell_release_candidate_review_selection_status(&selection),
        shell_release_candidate_review_index_status(index, index_path)
    )
}

fn shell_release_candidate_review_selection_status_label(
    status: StudioShellReleaseCandidateReviewSelectionStatus,
) -> &'static str {
    match status {
        StudioShellReleaseCandidateReviewSelectionStatus::Selected => "selected",
        StudioShellReleaseCandidateReviewSelectionStatus::Missing => "missing",
        StudioShellReleaseCandidateReviewSelectionStatus::Empty => "empty",
    }
}

fn shell_release_candidate_review_status_label(
    status: StudioShellReleaseCandidateReviewStatus,
) -> &'static str {
    match status {
        StudioShellReleaseCandidateReviewStatus::Ready => "ready",
        StudioShellReleaseCandidateReviewStatus::Blocked => "blocked",
        StudioShellReleaseCandidateReviewStatus::Rejected => "rejected",
    }
}

fn shell_hostess_handoff_package_status_label(
    status: StudioShellHostessHandoffPackageStatus,
) -> &'static str {
    match status {
        StudioShellHostessHandoffPackageStatus::Ready => "ready",
        StudioShellHostessHandoffPackageStatus::Blocked => "blocked",
        StudioShellHostessHandoffPackageStatus::Rejected => "rejected",
    }
}

fn shell_hostess_handoff_package_action_status_label(
    status: StudioShellHostessHandoffPackageActionStatus,
) -> &'static str {
    match status {
        StudioShellHostessHandoffPackageActionStatus::Ready => "ready",
        StudioShellHostessHandoffPackageActionStatus::Blocked => "blocked",
    }
}

fn shell_hostess_owner_intake_status_label(
    status: StudioShellHostessOwnerIntakeStatus,
) -> &'static str {
    match status {
        StudioShellHostessOwnerIntakeStatus::Ready => "ready",
        StudioShellHostessOwnerIntakeStatus::Blocked => "blocked",
        StudioShellHostessOwnerIntakeStatus::Rejected => "rejected",
    }
}

fn shell_hostess_owner_intake_assignment_status_label(
    status: StudioShellHostessOwnerIntakeAssignmentStatus,
) -> &'static str {
    match status {
        StudioShellHostessOwnerIntakeAssignmentStatus::Ready => "ready",
        StudioShellHostessOwnerIntakeAssignmentStatus::Blocked => "blocked",
    }
}

fn shell_hostess_staging_preview_status_label(
    status: StudioShellHostessStagingPreviewStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingPreviewStatus::Ready => "ready",
        StudioShellHostessStagingPreviewStatus::Blocked => "blocked",
        StudioShellHostessStagingPreviewStatus::Rejected => "rejected",
    }
}

fn shell_hostess_staging_preview_group_status_label(
    status: StudioShellHostessStagingPreviewGroupStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingPreviewGroupStatus::Ready => "ready",
        StudioShellHostessStagingPreviewGroupStatus::Blocked => "blocked",
    }
}

fn shell_bundle_status_label(status: StudioShellBundleStatus) -> &'static str {
    match status {
        StudioShellBundleStatus::Exported => "exported",
        StudioShellBundleStatus::Rejected => "rejected",
    }
}

fn shell_descriptor_status_label(status: StudioShellDescriptorStatus) -> &'static str {
    match status {
        StudioShellDescriptorStatus::Exported => "exported",
        StudioShellDescriptorStatus::Rejected => "rejected",
    }
}

fn shell_target_kind_label(kind: StudioShellTargetKind) -> &'static str {
    match kind {
        StudioShellTargetKind::Desktop => "desktop",
        StudioShellTargetKind::Phone => "phone",
        StudioShellTargetKind::Quest => "quest",
        StudioShellTargetKind::Unknown => "unknown",
    }
}

fn shell_handoff_acceptance_status_label(
    status: StudioShellHandoffAcceptanceStatus,
) -> &'static str {
    match status {
        StudioShellHandoffAcceptanceStatus::Ready => "ready",
        StudioShellHandoffAcceptanceStatus::Blocked => "blocked",
        StudioShellHandoffAcceptanceStatus::Rejected => "rejected",
    }
}

fn shell_handoff_acceptance_comparison_status_label(
    status: StudioShellHandoffAcceptanceComparisonStatus,
) -> &'static str {
    match status {
        StudioShellHandoffAcceptanceComparisonStatus::Improved => "improved",
        StudioShellHandoffAcceptanceComparisonStatus::Unchanged => "unchanged",
        StudioShellHandoffAcceptanceComparisonStatus::Regressed => "regressed",
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable => "incomparable",
    }
}

fn shell_handoff_acceptance_comparison_change_label(
    change: StudioShellHandoffAcceptanceComparisonChange,
) -> &'static str {
    match change {
        StudioShellHandoffAcceptanceComparisonChange::Added => "added",
        StudioShellHandoffAcceptanceComparisonChange::Removed => "removed",
        StudioShellHandoffAcceptanceComparisonChange::Improved => "improved",
        StudioShellHandoffAcceptanceComparisonChange::Unchanged => "unchanged",
        StudioShellHandoffAcceptanceComparisonChange::Regressed => "regressed",
        StudioShellHandoffAcceptanceComparisonChange::Changed => "changed",
    }
}

fn validation_status_label(status: StudioValidationStatus) -> &'static str {
    match status {
        StudioValidationStatus::Pass => "pass",
        StudioValidationStatus::Fail => "fail",
    }
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

    #[test]
    fn default_project_working_copy_uses_ignored_sibling_dir() {
        let root = temp_root("default-project-working-copy");
        let source_path = root.join("examples/synthetic-studio-project.json");
        write_fixture(
            &source_path,
            r#"{"$schema":"rusty.studio.project.v1","project_id":"demo","revision":1,"display_name":"Demo","package_catalog_path":"../../rusty-manifold-packages/packages/catalog.manifold.json","host_run_profile_paths":[],"graphs":[]}"#,
        );

        let working_path =
            default_project_working_copy_path(&source_path).expect("copy default project");

        assert_eq!(
            working_path,
            root.join("examples-working/synthetic-studio-project.json")
        );
        assert_eq!(
            std::fs::read_to_string(&working_path).expect("read working copy"),
            std::fs::read_to_string(&source_path).expect("read source")
        );
    }

    #[test]
    fn requested_synthetic_example_uses_ignored_working_copy() {
        let root = temp_root("requested-synthetic-example-working-copy");
        let source_path = root.join("examples/synthetic-studio-project.json");
        write_fixture(
            &source_path,
            r#"{"$schema":"rusty.studio.project.v1","project_id":"demo","revision":1,"display_name":"Demo","package_catalog_path":"../../rusty-manifold-packages/packages/catalog.manifold.json","host_run_profile_paths":[],"graphs":[]}"#,
        );

        let session_path =
            project_path_for_mutable_session(source_path.clone()).expect("resolve session path");

        assert_eq!(
            session_path,
            root.join("examples-working/synthetic-studio-project.json")
        );
        assert_eq!(
            std::fs::read_to_string(&session_path).expect("read working copy"),
            std::fs::read_to_string(&source_path).expect("read source")
        );
    }

    #[test]
    fn requested_non_default_project_keeps_original_path() {
        let root = temp_root("requested-non-default-project");
        let project_path = root.join("project.json");
        write_fixture(
            &project_path,
            r#"{"$schema":"rusty.studio.project.v1","project_id":"demo","revision":1,"display_name":"Demo","package_catalog_path":"../../rusty-manifold-packages/packages/catalog.manifold.json","host_run_profile_paths":[],"graphs":[]}"#,
        );

        let session_path =
            project_path_for_mutable_session(project_path.clone()).expect("resolve session path");

        assert_eq!(session_path, project_path);
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
  "host_profile": "host.headset",
  "app_id": "app.host_shell.headset",
  "install_route": "install.android_package",
  "launch_route": "launch.android_intent",
  "command_bridge": "bridge.adb_intent_file",
  "required_permissions": [
    "permission.bluetooth.scan",
    "permission.bluetooth.connect",
    "permission.location.fine"
  ],
  "evidence_pull_route": "evidence.adb_pull"
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

        let shell_preview = shell_preview_lines(&model);
        assert!(shell_preview.contains("studio.graph.makepad_edit [exported]"));
        assert!(shell_preview.contains("Shell descriptor exported"));
        assert!(
            shell_preview.contains("descriptor: studio.shell_descriptor.studio.graph.makepad_edit")
        );
        assert!(shell_preview.contains("shell: shell.synthetic.operator / Shell"));
        assert!(shell_preview.contains("target: host_run.profile.desktop / desktop"));
        assert!(shell_preview.contains(
            "graph: 1 package(s), 0 module(s), 0 stream binding(s), 0 command binding(s)"
        ));
        assert!(shell_preview.contains("descriptor validation: pass"));

        let shell_routes = shell_route_lines(&model);
        assert!(shell_routes.contains("host: host.desktop"));
        assert!(shell_routes.contains("app: app.host_shell.desktop"));
        assert!(shell_routes.contains("install: install.local_process"));
        assert!(shell_routes.contains("launch: launch.local_process"));
        assert!(shell_routes.contains("command: bridge.local_cli"));
        assert!(shell_routes.contains("evidence: evidence.filesystem"));

        let shell_template = shell_template_lines(&model);
        assert!(
            shell_template.contains("template: studio.shell_template.studio.graph.makepad_edit")
        );
        assert!(shell_template
            .contains("path: shells/desktop/studio.graph.makepad_edit.shell-template.json"));
        assert!(shell_template
            .contains("descriptor: descriptors/studio.graph.makepad_edit.shell-descriptor.json"));
        assert!(shell_template
            .contains("authority: rusty.manifold / rusty.hostess / authoring.export_planning"));

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
    fn selected_shell_bundle_export_writes_preview_files() {
        let root = temp_root("selected-shell-bundle-export");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");

        let (report, output_dir) = export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");

        assert_eq!(report.status, StudioShellBundleStatus::Exported);
        assert_eq!(
            report.bundle_files,
            vec![
                "descriptors/studio.graph.makepad_edit.shell-descriptor.json".to_string(),
                "shell-artifacts.json".to_string(),
                "shell-templates.json".to_string(),
                "shells/desktop/studio.graph.makepad_edit.shell-template.json".to_string(),
            ]
        );
        for relative_path in &report.bundle_files {
            let path = relative_path
                .split('/')
                .fold(output_dir.clone(), |path, segment| path.join(segment));
            assert!(path.is_file(), "missing {}", path.display());
        }
        let manifest = rusty_studio_core::load_shell_artifact_manifest(
            &output_dir.join("shell-artifacts.json"),
        )
        .expect("load shell artifacts manifest");
        assert_eq!(
            rusty_studio_core::validate_shell_artifact_manifest(&manifest, Some(&output_dir))
                .status,
            StudioValidationStatus::Pass
        );
        let index =
            rusty_studio_core::load_shell_template_index(&output_dir.join("shell-templates.json"))
                .expect("load shell template index");
        assert_eq!(
            rusty_studio_core::validate_shell_template_index(&index, Some(&output_dir)).status,
            StudioValidationStatus::Pass
        );
        let status = shell_bundle_export_status(&report, &output_dir);
        assert!(status.contains("exported; issue none"));
        assert!(status.contains("studio.graph.makepad_edit"));
        assert!(status.contains("shells/desktop/studio.graph.makepad_edit.shell-template.json"));
    }

    #[test]
    fn selected_shell_bundle_validation_reports_pass() {
        let root = temp_root("selected-shell-bundle-validate");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");

        let (report, output_dir) =
            validate_shell_bundle_for_project_source(&project_path, &model, 0)
                .expect("validate selected shell bundle");

        assert_eq!(report.status, StudioValidationStatus::Pass);
        assert!(report
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));
        let status = shell_bundle_validation_status(&report, &output_dir);
        assert!(status.contains("validated; status pass"));
        assert!(status.contains("studio.graph.makepad_edit"));
        assert!(status.contains("files: 4"));
    }

    #[test]
    fn shell_handoff_reports_ready_command_args() {
        let root = temp_root("desktop-shell-handoff");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");

        let (report, output_dir) = shell_handoff_for_project_source(&project_path, &model, 0)
            .expect("prepare shell handoff");

        assert_eq!(report.status, StudioValidationStatus::Pass);
        assert_eq!(report.consumer_id, "rusty-studio-desktop-shell");
        assert!(report
            .consumer_args
            .iter()
            .any(|arg| arg.ends_with("shell-templates.json")));
        let status = shell_handoff_status(&report, &output_dir);
        assert!(status.contains("shell handoff pass"));
        assert!(status.contains("rusty-studio-desktop-shell"));
        assert!(status.contains("target: desktop"));
        assert!(status.contains("--templates"));
        assert!(status.contains("rusty.manifold / rusty.hostess / authoring.export_planning"));
    }

    #[test]
    fn shell_handoff_readiness_reports_exported_graph() {
        let root = temp_root("shell-handoff-readiness");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");

        let (report, bundle_root) = shell_handoff_readiness_for_project_source(&project_path)
            .expect("inspect handoff readiness");

        assert_eq!(report.status, StudioValidationStatus::Pass);
        assert_eq!(report.graph_count, 1);
        assert_eq!(report.ready_count, 1);
        assert_eq!(report.failed_count, 0);
        assert_eq!(report.missing_bundle_count, 0);
        assert_eq!(report.target_summaries.len(), 1);
        assert_eq!(report.target_summaries[0].ready_count, 1);
        assert_eq!(report.target_summaries[0].graph_count, 1);
        assert_eq!(report.target_summaries[0].bundle_dirs.len(), 1);
        assert_eq!(report.target_summaries[0].ready_bundle_dirs.len(), 1);
        assert!(report.target_summaries[0].failed_bundle_dirs.is_empty());
        assert!(report.target_summaries[0].missing_bundle_dirs.is_empty());
        assert_eq!(report.target_summaries[0].template_index_paths.len(), 1);
        assert_eq!(report.entries.len(), 1);
        assert_eq!(
            report.entries[0].export_bundle_id,
            "studio.export.studio.graph.makepad_edit"
        );
        assert_eq!(report.entries[0].consumer_id, "rusty-studio-desktop-shell");
        assert_eq!(report.entries[0].package_count, 1);
        assert_eq!(report.entries[0].module_count, 0);
        assert_eq!(report.entries[0].operator_shell_count, 1);
        assert_eq!(report.entries[0].failed_check_count, 0);
        let status = shell_handoff_readiness_status(&report, &bundle_root);
        assert!(status.contains("handoff readiness pass"));
        assert!(status.contains("ready 1/1"));
        assert!(status.contains("failed 0; missing 0"));
        assert!(status.contains("desktop: ready 1/1; missing 0"));
        assert!(status.contains("templates "));
        assert!(status.contains("shell-templates.json"));
        assert!(status.contains("studio.graph.makepad_edit [desktop]"));
        assert!(status.contains("profile host_run.profile.desktop"));
        assert!(status.contains("packages 1; modules 0; shell 1"));
    }

    #[test]
    fn shell_runbook_reports_owner_routes_from_makepad_route() {
        let root = temp_root("shell-runbook");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");

        let (report, bundle_root) =
            shell_runbook_for_project_source(&project_path).expect("inspect shell runbook");

        assert_eq!(report.schema_id, "rusty.studio.shell_runbook_report.v1");
        assert_eq!(report.status, StudioShellRunbookStatus::Ready);
        assert_eq!(report.ready_count, 1);
        assert_eq!(report.blocked_count, 0);
        assert_eq!(report.rejected_count, 0);
        assert_eq!(report.entries.len(), 1);
        assert_eq!(report.prohibited_actions.len(), 4);
        let entry = &report.entries[0];
        assert_eq!(entry.status, StudioShellRunbookStatus::Ready);
        assert_eq!(entry.responsible_owner, "rusty.hostess");
        assert_eq!(entry.execution_policy, "not_executed.request_only");
        assert_eq!(entry.command_session_authority, "rusty.manifold");
        assert_eq!(entry.install_launch_evidence_authority, "rusty.hostess");
        assert_eq!(entry.studio_role, "authoring.export_planning");
        assert_eq!(entry.consumer_id, "rusty-studio-desktop-shell");
        assert_eq!(entry.runtime_route_kind, "desktop_operator_shell");
        assert_eq!(
            entry.host_routes.install_route.as_deref(),
            Some("install.local_process")
        );
        assert_eq!(
            entry.host_routes.launch_route.as_deref(),
            Some("launch.local_process")
        );
        assert_eq!(
            entry.host_routes.command_bridge.as_deref(),
            Some("bridge.local_cli")
        );
        assert_eq!(
            entry.host_routes.evidence_pull_route.as_deref(),
            Some("evidence.filesystem")
        );
        assert!(entry
            .cli_request
            .iter()
            .any(|arg| arg == "rusty-studio-desktop-shell"));
        assert!(entry.cli_request.iter().any(|arg| arg == "--templates"));

        let status = shell_runbook_status(&report, &bundle_root);
        assert!(status.contains("shell runbook ready"));
        assert!(status.contains("owner rusty.hostess"));
        assert!(status.contains("not_executed.request_only"));
        assert!(status.contains("install.local_process"));
        assert!(status.contains("launch.local_process"));
        assert!(status.contains("bridge.local_cli"));
        assert!(status.contains("evidence.filesystem"));
        assert!(status.contains("rusty-studio-desktop-shell"));
    }

    #[test]
    fn shell_export_package_reports_descriptor_template_and_runbook_from_makepad_route() {
        let root = temp_root("shell-export-package");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");

        let (report, bundle_root) =
            shell_export_package_for_project_source(&project_path).expect("review export package");

        assert_eq!(
            report.schema_id,
            "rusty.studio.shell_export_package_report.v1"
        );
        assert_eq!(report.status, StudioShellExportPackageStatus::Ready);
        assert_eq!(report.ready_count, 1);
        assert_eq!(report.blocked_count, 0);
        assert_eq!(report.rejected_count, 0);
        assert_eq!(report.descriptor_count, 1);
        assert_eq!(report.template_manifest_count, 1);
        assert_eq!(report.runbook_entry_count, 1);
        assert_eq!(report.execution_policy, "not_executed.review_only");
        assert_eq!(report.review_owner, "rusty.hostess");
        assert_eq!(report.command_session_authority, "rusty.manifold");
        assert_eq!(report.install_launch_evidence_authority, "rusty.hostess");
        assert_eq!(report.studio_role, "authoring.export_planning");
        let entry = &report.entries[0];
        assert_eq!(entry.status, StudioShellExportPackageStatus::Ready);
        assert_eq!(entry.responsible_owner, "rusty.hostess");
        assert_eq!(entry.next_required_action, "review_with_runtime_owner");
        assert_eq!(entry.runtime_route_kind, "desktop_operator_shell");
        assert!(entry.descriptor.is_some());
        assert!(entry.template_manifest.is_some());
        assert!(entry
            .runbook_cli_request
            .iter()
            .any(|arg| arg == "rusty-studio-desktop-shell"));
        assert!(entry
            .runbook_cli_request
            .iter()
            .any(|arg| arg == "--templates"));

        let status = shell_export_package_status(&report, &bundle_root);
        assert!(status.contains("shell export package ready"));
        assert!(status.contains("descriptors 1; templates 1"));
        assert!(status.contains("owner: rusty.hostess"));
        assert!(status.contains("not_executed.review_only"));
        assert!(status.contains("review_with_runtime_owner"));
        assert!(status.contains("studio.shell_descriptor.studio.graph.makepad_edit"));
        assert!(status.contains("studio.shell_template.studio.graph.makepad_edit"));
    }

    #[test]
    fn shell_export_package_baseline_writes_durable_artifact() {
        let root = temp_root("shell-export-package-baseline");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");

        let (report, baseline, index, package_path, baseline_path, index_path, bundle_root) =
            write_shell_export_package_baseline_for_project_source(&project_path)
                .expect("write export package baseline");

        assert!(package_path.is_file());
        assert!(baseline_path.is_file());
        assert!(index_path.is_file());
        assert_eq!(report.status, StudioShellExportPackageStatus::Ready);
        assert_eq!(
            baseline.schema_id,
            "rusty.studio.shell_export_package_baseline_manifest.v1"
        );
        assert_eq!(
            baseline.baseline_id,
            "studio.project.makepad_edit.rev1.ready"
        );
        assert_eq!(baseline.package_path, package_path.display().to_string());
        assert_eq!(
            baseline.package_schema,
            "rusty.studio.shell_export_package_report.v1"
        );
        assert_eq!(baseline.project_id, "studio.project.makepad_edit");
        assert_eq!(baseline.project_revision, 1);
        assert_eq!(baseline.status, StudioShellExportPackageStatus::Ready);
        assert_eq!(baseline.ready_count, 1);
        assert_eq!(baseline.blocked_count, 0);
        assert_eq!(baseline.rejected_count, 0);
        assert_eq!(baseline.descriptor_count, 1);
        assert_eq!(baseline.template_manifest_count, 1);
        assert_eq!(baseline.runbook_entry_count, 1);
        assert_eq!(baseline.target_count, 1);
        assert_eq!(baseline.execution_policy, "not_executed.review_only");
        assert_eq!(baseline.command_session_authority, "rusty.manifold");
        assert_eq!(baseline.install_launch_evidence_authority, "rusty.hostess");
        assert_eq!(baseline.studio_role, "authoring.export_planning");
        assert_eq!(
            index.schema_id,
            "rusty.studio.shell_export_package_baseline_index.v1"
        );
        assert_eq!(
            index.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        assert_eq!(index.baseline_count, 1);
        assert_eq!(index.ready_baseline_count, 1);
        assert_eq!(index.blocked_baseline_count, 0);
        assert_eq!(
            index.entries[0].package_path,
            package_path.display().to_string()
        );
        let written = std::fs::read_to_string(&baseline_path).expect("read baseline manifest");
        assert!(written
            .contains("\"$schema\": \"rusty.studio.shell_export_package_baseline_manifest.v1\""));
        let status = shell_export_package_baseline_status(
            &report,
            &baseline,
            &index,
            &package_path,
            &baseline_path,
            &index_path,
            &bundle_root,
        );
        assert!(status.contains("export package baseline written"));
        assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains("export package baseline selection selected"));
        assert!(status.contains("export package baseline index slots 1"));
        assert!(status.contains("shell export package ready"));
    }

    #[test]
    fn shell_export_package_baseline_appends_and_cycles_default() {
        let root = temp_root("shell-export-package-baseline-cycle");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        let (_, ready_baseline, _, _, ready_baseline_path, index_path, _) =
            write_shell_export_package_baseline_for_project_source(&project_path)
                .expect("write initial package baseline");
        let (_, archived_baseline, archived_index, _, archived_baseline_path, _, _) =
            append_shell_export_package_baseline_for_project_source(&project_path)
                .expect("append package baseline");

        assert_eq!(
            archived_baseline.baseline_id,
            "studio.project.makepad_edit.rev1.ready.archive2"
        );
        assert_eq!(
            archived_index.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready.archive2")
        );
        assert_eq!(archived_index.baseline_count, 2);
        assert_eq!(archived_index.ready_baseline_count, 2);
        assert_eq!(
            archived_index.entries[1].baseline_manifest_path.as_deref(),
            Some(archived_baseline_path.display().to_string().as_str())
        );

        let (selected_ready_baseline, selected_ready_index, selected_ready_path, loaded_index_path) =
            select_next_shell_export_package_baseline_default_for_project_source(&project_path)
                .expect("select next package baseline default");
        assert_eq!(selected_ready_baseline, ready_baseline);
        assert_eq!(selected_ready_path, ready_baseline_path);
        assert_eq!(loaded_index_path, index_path);
        assert_eq!(
            selected_ready_index.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        let status = shell_export_package_baseline_select_status(
            &selected_ready_baseline,
            &selected_ready_index,
            &selected_ready_path,
            &loaded_index_path,
        );
        assert!(status.contains("export package baseline default selected"));
        assert!(status.contains(
            "export package baseline selection selected; requested studio.project.makepad_edit.rev1.ready; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));
        assert!(status.contains("selected yes; default yes"));

        let (promoted_baseline, promoted_index, promoted_path, loaded_index_path) =
            promote_shell_export_package_baseline_default_for_project_source(&project_path)
                .expect("promote saved package baseline");
        assert_eq!(promoted_baseline, ready_baseline);
        assert_eq!(promoted_path, ready_baseline_path);
        assert_eq!(
            promoted_index.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        let written_index =
            load_shell_export_package_baseline_index(&loaded_index_path).expect("load index");
        assert_eq!(written_index, promoted_index);
        let status = shell_export_package_baseline_promote_status(
            &promoted_baseline,
            &promoted_index,
            &promoted_path,
            &loaded_index_path,
        );
        assert!(status.contains("export package baseline default promoted"));
        assert!(status.contains("export package baseline index slots 2"));
        assert!(status.contains("studio.project.makepad_edit.rev1.ready.archive2 [ready]"));
    }

    #[test]
    fn shell_export_package_comparison_reports_unchanged_from_makepad_route() {
        let root = temp_root("shell-export-package-comparison");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        let (_, saved_baseline, _, package_path, baseline_path, index_path, _) =
            write_shell_export_package_baseline_for_project_source(&project_path)
                .expect("write package baseline");

        let (comparison, loaded_baseline_path, bundle_root) =
            shell_export_package_comparison_for_project_source(&project_path)
                .expect("compare package review");

        assert_eq!(loaded_baseline_path, baseline_path);
        assert_eq!(
            comparison.baseline_identity_schema.as_deref(),
            Some("rusty.studio.shell_export_package_baseline_manifest.v1")
        );
        assert_eq!(
            comparison.baseline_id.as_deref(),
            Some(saved_baseline.baseline_id.as_str())
        );
        assert_eq!(
            comparison.baseline_package_path.as_deref(),
            Some(package_path.display().to_string().as_str())
        );
        assert_eq!(
            comparison.baseline_index_schema.as_deref(),
            Some("rusty.studio.shell_export_package_baseline_index.v1")
        );
        assert_eq!(
            comparison.baseline_index_path.as_deref(),
            Some(index_path.display().to_string().as_str())
        );
        assert_eq!(
            comparison.status,
            StudioShellExportPackageComparisonStatus::Unchanged
        );
        assert_eq!(comparison.ready_delta, 0);
        assert_eq!(comparison.blocked_delta, 0);
        assert_eq!(comparison.rejected_delta, 0);
        assert_eq!(comparison.descriptor_delta, 0);
        assert_eq!(comparison.template_manifest_delta, 0);
        assert_eq!(comparison.entries.len(), 1);
        assert_eq!(
            comparison.entries[0].change,
            StudioShellExportPackageComparisonChange::Unchanged
        );
        let status = shell_export_package_comparison_status(
            &comparison,
            &loaded_baseline_path,
            &bundle_root,
        );
        assert!(status.contains("export package comparison unchanged"));
        assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains(&format!("baseline package: {}", package_path.display())));
        assert!(status.contains(&format!("baseline index: {}", index_path.display())));
        assert!(status.contains("ready 1->1, delta 0"));
        assert!(status.contains("descriptors 1->1, delta 0"));
        assert!(status.contains("change unchanged"));
        assert!(status.contains("studio.graph.makepad_edit [desktop]"));
    }

    #[test]
    fn shell_export_package_comparison_reports_regression_from_makepad_route() {
        let root = temp_root("shell-export-package-comparison-regressed");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        let (_, _, _, _, baseline_path, index_path, _) =
            write_shell_export_package_baseline_for_project_source(&project_path)
                .expect("write package baseline");
        std::fs::remove_dir_all(selected_shell_bundle_root_dir(&project_path))
            .expect("remove selected shell bundle root");

        let (comparison, _, bundle_root) =
            shell_export_package_comparison_for_project_source(&project_path)
                .expect("compare regressed package review");

        assert_eq!(
            comparison.status,
            StudioShellExportPackageComparisonStatus::Regressed
        );
        assert_eq!(
            comparison.issue_code.as_deref(),
            Some("studio.issue.shell_bundle_file_missing")
        );
        assert_eq!(comparison.ready_delta, -1);
        assert_eq!(comparison.blocked_delta, 1);
        assert_eq!(comparison.descriptor_delta, -1);
        assert_eq!(comparison.template_manifest_delta, -1);
        assert_eq!(
            comparison.baseline_index_path.as_deref(),
            Some(index_path.display().to_string().as_str())
        );
        assert_eq!(
            comparison.entries[0].change,
            StudioShellExportPackageComparisonChange::Regressed
        );
        let status =
            shell_export_package_comparison_status(&comparison, &baseline_path, &bundle_root);
        assert!(status.contains("export package comparison regressed"));
        assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains(&format!("baseline index: {}", index_path.display())));
        assert!(status.contains("ready 1->0, delta -1"));
        assert!(status.contains("blocked 0->1, delta 1"));
        assert!(status.contains("descriptors 1->0, delta -1"));
        assert!(status.contains("templates 1->0, delta -1"));
        assert!(status.contains("issue studio.issue.shell_bundle_file_missing"));
        assert!(status.contains("change regressed"));
    }

    #[test]
    fn shell_handoff_manifest_writes_durable_artifact() {
        let root = temp_root("shell-handoff-manifest");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");

        let (manifest, output_path) =
            write_shell_handoff_manifest_for_project_source(&project_path)
                .expect("write handoff manifest");

        assert!(output_path.is_file());
        assert_eq!(manifest.schema_id, "rusty.studio.shell_handoff_manifest.v1");
        assert_eq!(manifest.status, StudioValidationStatus::Pass);
        assert_eq!(manifest.graph_count, 1);
        assert_eq!(manifest.ready_count, 1);
        assert_eq!(manifest.failed_count, 0);
        assert_eq!(manifest.missing_bundle_count, 0);
        assert_eq!(manifest.targets.len(), 1);
        assert_eq!(manifest.handoffs.len(), 1);
        assert_eq!(
            manifest.handoffs[0].consumer_id,
            "rusty-studio-desktop-shell"
        );
        assert!(manifest.handoffs[0]
            .template_index_path
            .ends_with("shell-templates.json"));
        let written = std::fs::read_to_string(&output_path).expect("read handoff manifest");
        assert!(written.contains("\"$schema\": \"rusty.studio.shell_handoff_manifest.v1\""));
        let status = shell_handoff_manifest_status(&manifest, &output_path);
        assert!(status.contains("handoff manifest pass"));
        assert!(status.contains("ready 1/1"));
        assert!(status.contains("failed 0; missing 0"));
        assert!(status.contains("rusty.manifold / rusty.hostess / authoring.export_planning"));
        assert!(status.contains("desktop: ready 1/1; failed 0; missing 0"));
    }

    #[test]
    fn shell_handoff_acceptance_reports_ready_from_makepad_route() {
        let root = temp_root("shell-handoff-acceptance");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");

        let (report, bundle_root) = shell_handoff_acceptance_for_project_source(&project_path)
            .expect("review acceptance checklist");

        assert_eq!(
            report.schema_id,
            "rusty.studio.shell_handoff_acceptance_checklist.v1"
        );
        assert_eq!(report.status, StudioShellHandoffAcceptanceStatus::Ready);
        assert_eq!(report.ready_count, 1);
        assert_eq!(report.blocked_count, 0);
        assert_eq!(report.rejected_count, 0);
        assert_eq!(report.entries.len(), 1);
        assert_eq!(
            report.prohibited_actions,
            vec![
                "install".to_string(),
                "launch".to_string(),
                "open_command_session".to_string(),
                "collect_device_evidence".to_string(),
            ]
        );
        assert!(report.entries[0]
            .checks
            .iter()
            .any(|check| check.owner == "rusty.manifold"));
        assert!(report.entries[0]
            .checks
            .iter()
            .any(|check| check.owner == "rusty.hostess"));
        assert!(report.entries[0]
            .checks
            .iter()
            .any(|check| check.owner == "rusty.studio"));

        let status = shell_handoff_acceptance_status(&report, &bundle_root);
        assert!(status.contains("handoff acceptance ready"));
        assert!(status.contains("ready 1; blocked 0; rejected 0"));
        assert!(status.contains(
            "prohibited: install, launch, open_command_session, collect_device_evidence"
        ));
        assert!(status.contains("studio.graph.makepad_edit [desktop]"));
        assert!(status.contains("route desktop_operator_shell"));
        assert!(
            status.contains("owners rusty.manifold:pass, rusty.hostess:pass, rusty.studio:pass")
        );
    }

    #[test]
    fn shell_handoff_acceptance_baseline_writes_durable_artifact() {
        let root = temp_root("shell-handoff-acceptance-baseline");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");

        let (report, baseline, index, output_path, baseline_path, index_path, bundle_root) =
            write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
                .expect("write acceptance baseline");

        assert_eq!(
            output_path,
            shell_handoff_acceptance_checklist_output_path(&project_path)
        );
        assert!(output_path.is_file());
        assert_eq!(
            baseline_path,
            shell_handoff_acceptance_baseline_manifest_output_path(&project_path)
        );
        assert!(baseline_path.is_file());
        assert_eq!(
            index_path,
            shell_handoff_acceptance_baseline_index_output_path(&project_path)
        );
        assert!(index_path.is_file());
        assert_eq!(
            baseline.schema_id,
            "rusty.studio.shell_handoff_acceptance_baseline_manifest.v1"
        );
        assert_eq!(
            baseline.baseline_id,
            "studio.project.makepad_edit.rev1.ready"
        );
        assert_eq!(
            baseline.label,
            "studio.project.makepad_edit revision 1 ready acceptance baseline"
        );
        assert_eq!(baseline.checklist_path, output_path.display().to_string());
        assert_eq!(baseline.summary.project_id, "studio.project.makepad_edit");
        assert_eq!(baseline.summary.project_revision, 1);
        assert_eq!(
            report.schema_id,
            "rusty.studio.shell_handoff_acceptance_checklist.v1"
        );
        assert_eq!(report.status, StudioShellHandoffAcceptanceStatus::Ready);
        assert_eq!(report.ready_count, 1);
        assert_eq!(report.blocked_count, 0);
        assert_eq!(report.rejected_count, 0);
        assert_eq!(
            index.schema_id,
            "rusty.studio.shell_handoff_acceptance_baseline_index.v1"
        );
        assert_eq!(
            index.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        assert_eq!(index.baseline_count, 1);
        assert_eq!(index.ready_baseline_count, 1);
        assert_eq!(index.blocked_baseline_count, 0);
        assert_eq!(index.rejected_baseline_count, 0);
        assert_eq!(index.entries.len(), 1);
        assert_eq!(index.entries[0].baseline_id, baseline.baseline_id);
        assert_eq!(
            index.entries[0].baseline_manifest_path.as_deref(),
            Some(baseline_path.display().to_string().as_str())
        );
        assert_eq!(
            index.entries[0].checklist_path,
            output_path.display().to_string()
        );
        let written = std::fs::read_to_string(&output_path).expect("read acceptance baseline");
        assert!(
            written.contains("\"$schema\": \"rusty.studio.shell_handoff_acceptance_checklist.v1\"")
        );
        let manifest_text =
            std::fs::read_to_string(&baseline_path).expect("read baseline identity");
        assert!(manifest_text.contains(
            "\"$schema\": \"rusty.studio.shell_handoff_acceptance_baseline_manifest.v1\""
        ));
        let index_text = std::fs::read_to_string(&index_path).expect("read baseline index");
        assert!(index_text
            .contains("\"$schema\": \"rusty.studio.shell_handoff_acceptance_baseline_index.v1\""));
        let status = shell_handoff_acceptance_baseline_status(
            &report,
            &baseline,
            &index,
            &output_path,
            &baseline_path,
            &index_path,
            &bundle_root,
        );
        assert!(status.contains("acceptance baseline written"));
        assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains(&format!("identity: {}", baseline_path.display())));
        assert!(status.contains(&format!("checklist: {}", output_path.display())));
        assert!(status.contains(&format!("index: {}", index_path.display())));
        assert!(status.contains(
            "baseline selection selected; requested none; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));
        assert!(status.contains("selected yes; default yes"));
        assert!(status
            .contains("baseline index slots 1; default studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains("handoff acceptance ready"));
        assert!(status.contains("ready 1; blocked 0; rejected 0"));
    }

    #[test]
    fn shell_handoff_acceptance_baseline_appends_history_entry() {
        let root = temp_root("shell-handoff-acceptance-baseline-append");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        let (_, saved_baseline, saved_index, _, saved_baseline_path, index_path, _) =
            write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
                .expect("write initial baseline");
        assert_eq!(
            saved_index.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );

        let (
            report,
            archived_baseline,
            archived_index,
            checklist_path,
            baseline_path,
            loaded_index_path,
            bundle_root,
        ) = append_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("append baseline history entry");

        assert_eq!(loaded_index_path, index_path);
        assert_eq!(
            archived_baseline.baseline_id,
            "studio.project.makepad_edit.rev1.ready.archive2"
        );
        assert_eq!(
            archived_baseline.label,
            "studio.project.makepad_edit revision 1 ready acceptance baseline archive 2"
        );
        assert!(checklist_path.ends_with(
            "target/studio-shell-handoffs/baselines/studio.project.makepad_edit.rev1.ready.archive2.checklist.json"
        ));
        assert!(baseline_path.ends_with(
            "target/studio-shell-handoffs/baselines/studio.project.makepad_edit.rev1.ready.archive2.baseline.json"
        ));
        assert!(checklist_path.is_file());
        assert!(baseline_path.is_file());
        assert_eq!(
            archived_baseline.checklist_path,
            checklist_path.display().to_string()
        );
        assert_eq!(
            archived_index.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready.archive2")
        );
        assert_eq!(archived_index.baseline_count, 2);
        assert_eq!(archived_index.ready_baseline_count, 2);
        assert_eq!(archived_index.blocked_baseline_count, 0);
        assert_eq!(archived_index.rejected_baseline_count, 0);
        assert!(archived_index
            .entries
            .iter()
            .any(|entry| entry.baseline_id == saved_baseline.baseline_id
                && entry.baseline_manifest_path.as_deref()
                    == Some(saved_baseline_path.display().to_string().as_str())));
        assert!(archived_index.entries.iter().any(|entry| {
            entry.baseline_id == archived_baseline.baseline_id
                && entry.baseline_manifest_path.as_deref()
                    == Some(baseline_path.display().to_string().as_str())
        }));
        let readback =
            load_shell_handoff_acceptance_baseline_index(&index_path).expect("load appended index");
        assert_eq!(readback, archived_index);

        let status = shell_handoff_acceptance_baseline_append_status(
            &report,
            &archived_baseline,
            &archived_index,
            &checklist_path,
            &baseline_path,
            &loaded_index_path,
            &bundle_root,
        );
        assert!(status.contains("acceptance baseline archived"));
        assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready.archive2"));
        assert!(status.contains(
            "baseline selection selected; requested studio.project.makepad_edit.rev1.ready.archive2; default studio.project.makepad_edit.rev1.ready.archive2; selected studio.project.makepad_edit.rev1.ready.archive2"
        ));
        assert!(status.contains(
            "baseline index slots 2; default studio.project.makepad_edit.rev1.ready.archive2"
        ));
        assert!(status.contains("studio.project.makepad_edit.rev1.ready [ready]"));
        assert!(status.contains("studio.project.makepad_edit.rev1.ready.archive2 [ready]"));
    }

    #[test]
    fn shell_handoff_acceptance_baseline_summary_reports_revision_metadata() {
        let root = temp_root("shell-handoff-acceptance-baseline-summary");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        let (_, expected_baseline, expected_index, checklist_path, baseline_path, index_path, _) =
            write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
                .expect("write acceptance baseline");

        let (baseline, index, loaded_path, loaded_index_path) =
            shell_handoff_acceptance_baseline_summary_for_project_source(&project_path)
                .expect("summarize acceptance baseline");

        assert_eq!(loaded_path, baseline_path);
        assert_eq!(loaded_index_path, index_path);
        assert_eq!(baseline, expected_baseline);
        assert_eq!(index, expected_index);
        assert_eq!(
            baseline.schema_id,
            "rusty.studio.shell_handoff_acceptance_baseline_manifest.v1"
        );
        assert_eq!(
            baseline.baseline_id,
            "studio.project.makepad_edit.rev1.ready"
        );
        assert_eq!(
            baseline.checklist_path,
            checklist_path.display().to_string()
        );
        let summary = &baseline.summary;
        assert_eq!(
            summary.schema_id,
            "rusty.studio.shell_handoff_acceptance_summary.v1"
        );
        assert_eq!(
            summary.checklist_schema,
            "rusty.studio.shell_handoff_acceptance_checklist.v1"
        );
        assert_eq!(summary.project_id, "studio.project.makepad_edit");
        assert_eq!(summary.project_revision, 1);
        assert_eq!(summary.status, StudioShellHandoffAcceptanceStatus::Ready);
        assert_eq!(summary.ready_count, 1);
        assert_eq!(summary.blocked_count, 0);
        assert_eq!(summary.rejected_count, 0);
        assert_eq!(summary.entry_count, 1);
        assert_eq!(summary.targets.len(), 1);
        assert_eq!(
            summary.targets[0].target_kind,
            StudioShellTargetKind::Desktop
        );
        assert_eq!(
            summary.targets[0].consumer_ids,
            vec!["rusty-studio-desktop-shell"]
        );
        assert_eq!(
            summary.targets[0].route_kinds,
            vec!["desktop_operator_shell"]
        );
        let status = shell_handoff_acceptance_summary_status(
            &baseline,
            &index,
            &loaded_path,
            &loaded_index_path,
        );
        assert!(status.contains("acceptance baseline summary ready"));
        assert!(status.contains("baseline studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains("project studio.project.makepad_edit rev 1"));
        assert!(status.contains("manifest studio.shell_handoffs.studio.project.makepad_edit"));
        assert!(status.contains(&format!("identity: {}", baseline_path.display())));
        assert!(status.contains(&format!("checklist: {}", checklist_path.display())));
        assert!(status.contains(&format!("index: {}", index_path.display())));
        assert!(status.contains(
            "baseline selection selected; requested none; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));
        assert!(status.contains("baseline index slots 1"));
        assert!(status.contains("desktop: ready 1/1; blocked 0; rejected 0"));
        assert!(status.contains("consumers rusty-studio-desktop-shell"));
        assert!(status.contains("routes desktop_operator_shell"));
    }

    #[test]
    fn shell_handoff_acceptance_baseline_promotes_saved_default() {
        let root = temp_root("shell-handoff-acceptance-baseline-promote");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        let (_, ready_baseline, ready_index, _, ready_baseline_path, index_path, _) =
            write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
                .expect("write acceptance baseline");

        let project = load_project(&project_path).expect("load project");
        let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
            &project,
            project_path.parent(),
            &root.join("missing-selected-shells"),
        );
        let blocked_checklist_path = root.join("blocked-checklist.json");
        save_json(&blocked_checklist_path, &blocked_checklist)
            .expect("save blocked acceptance checklist");
        let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
            &blocked_checklist,
            &blocked_checklist_path,
            Some("studio.project.makepad_edit.rev1.blocked"),
            Some("studio.project.makepad_edit revision 1 blocked acceptance baseline"),
        );
        let blocked_baseline_path = root.join("blocked-baseline.json");
        save_json(&blocked_baseline_path, &blocked_baseline).expect("save blocked baseline");
        let multi_index =
            rusty_studio_core::append_shell_handoff_acceptance_baseline_index_manifests(
                &ready_index,
                vec![(blocked_baseline, Some(blocked_baseline_path))],
                Some("studio.project.makepad_edit.rev1.blocked"),
            );
        save_json(&index_path, &multi_index).expect("save multi-baseline index");
        assert_eq!(
            multi_index.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.blocked")
        );

        let (baseline, promoted, baseline_path, loaded_index_path) =
            promote_shell_handoff_acceptance_baseline_default_for_project_source(&project_path)
                .expect("promote saved baseline");

        assert_eq!(baseline, ready_baseline);
        assert_eq!(baseline_path, ready_baseline_path);
        assert_eq!(loaded_index_path, index_path);
        assert_eq!(
            promoted.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        assert_eq!(promoted.baseline_count, 2);
        assert_eq!(promoted.ready_baseline_count, 1);
        assert_eq!(promoted.blocked_baseline_count, 1);
        let written_index =
            load_shell_handoff_acceptance_baseline_index(&index_path).expect("load written index");
        assert_eq!(written_index, promoted);

        let status = shell_handoff_acceptance_baseline_promote_status(
            &baseline,
            &promoted,
            &baseline_path,
            &loaded_index_path,
        );
        assert!(status.contains("acceptance baseline default promoted"));
        assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains(
            "baseline selection selected; requested studio.project.makepad_edit.rev1.ready; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));
        assert!(status
            .contains("baseline index slots 2; default studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains("studio.project.makepad_edit.rev1.blocked [blocked]"));
    }

    #[test]
    fn shell_handoff_acceptance_baseline_cycles_index_default() {
        let root = temp_root("shell-handoff-acceptance-baseline-cycle");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        let (_, ready_baseline, _, _, ready_baseline_path, index_path, _) =
            write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
                .expect("write initial baseline");
        let (_, archived_baseline, archived_index, _, archived_baseline_path, _, _) =
            append_shell_handoff_acceptance_baseline_for_project_source(&project_path)
                .expect("append baseline history entry");
        assert_eq!(
            archived_index.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready.archive2")
        );

        let (selected_ready_baseline, selected_ready_index, selected_ready_path, loaded_index_path) =
            select_next_shell_handoff_acceptance_baseline_default_for_project_source(&project_path)
                .expect("select next baseline default");

        assert_eq!(selected_ready_baseline, ready_baseline);
        assert_eq!(selected_ready_path, ready_baseline_path);
        assert_eq!(loaded_index_path, index_path);
        assert_eq!(
            selected_ready_index.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        let status = shell_handoff_acceptance_baseline_select_status(
            &selected_ready_baseline,
            &selected_ready_index,
            &selected_ready_path,
            &loaded_index_path,
        );
        assert!(status.contains("acceptance baseline default selected"));
        assert!(status.contains(
            "baseline selection selected; requested studio.project.makepad_edit.rev1.ready; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));
        assert!(status.contains("selected yes; default yes"));

        let (
            selected_archived_baseline,
            selected_archived_index,
            selected_archived_path,
            loaded_index_path,
        ) = select_next_shell_handoff_acceptance_baseline_default_for_project_source(&project_path)
            .expect("cycle baseline default");

        assert_eq!(selected_archived_baseline, archived_baseline);
        assert_eq!(selected_archived_path, archived_baseline_path);
        assert_eq!(
            selected_archived_index.default_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready.archive2")
        );
        let written_index = load_shell_handoff_acceptance_baseline_index(&loaded_index_path)
            .expect("load cycled index");
        assert_eq!(written_index, selected_archived_index);
        let status = shell_handoff_acceptance_baseline_select_status(
            &selected_archived_baseline,
            &selected_archived_index,
            &selected_archived_path,
            &loaded_index_path,
        );
        assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready.archive2"));
        assert!(status.contains(
            "baseline selection selected; requested studio.project.makepad_edit.rev1.ready.archive2; default studio.project.makepad_edit.rev1.ready.archive2; selected studio.project.makepad_edit.rev1.ready.archive2"
        ));
    }

    #[test]
    fn shell_handoff_acceptance_blocks_missing_bundle_from_makepad_route() {
        let root = temp_root("shell-handoff-acceptance-missing");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");

        let (report, bundle_root) = shell_handoff_acceptance_for_project_source(&project_path)
            .expect("review missing acceptance checklist");

        assert_eq!(report.status, StudioShellHandoffAcceptanceStatus::Blocked);
        assert_eq!(report.ready_count, 0);
        assert_eq!(report.blocked_count, 1);
        assert_eq!(report.rejected_count, 0);
        assert_eq!(report.entries.len(), 1);
        assert_eq!(
            report.issue_code.as_deref(),
            Some("studio.issue.shell_bundle_file_missing")
        );
        assert!(report.entries[0]
            .checks
            .iter()
            .any(|check| check.issue_code.as_deref()
                == Some("studio.issue.shell_handoff_acceptance_blocked")));
        let failed_check_count = report.entries[0]
            .checks
            .iter()
            .filter(|check| check.status == StudioValidationStatus::Fail)
            .count();
        assert!(failed_check_count > 0);

        let status = shell_handoff_acceptance_status(&report, &bundle_root);
        assert!(status.contains("handoff acceptance blocked"));
        assert!(status.contains("ready 0; blocked 1; rejected 0"));
        assert!(status.contains("issue studio.issue.shell_bundle_file_missing"));
        assert!(status.contains(&format!("failed {failed_check_count}")));
    }

    #[test]
    fn shell_handoff_acceptance_comparison_reports_unchanged_from_makepad_route() {
        let root = temp_root("shell-handoff-acceptance-comparison");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        let (_, saved_baseline, _, checklist_path, baseline_path, index_path, _) =
            write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
                .expect("write baseline checklist");

        let (comparison, loaded_baseline_path, bundle_root) =
            shell_handoff_acceptance_comparison_for_project_source(&project_path)
                .expect("compare acceptance checklist");

        assert_eq!(loaded_baseline_path, baseline_path);
        assert_eq!(
            comparison.baseline_identity_schema.as_deref(),
            Some("rusty.studio.shell_handoff_acceptance_baseline_manifest.v1")
        );
        assert_eq!(
            comparison.baseline_id.as_deref(),
            Some(saved_baseline.baseline_id.as_str())
        );
        assert_eq!(
            comparison.baseline_label.as_deref(),
            Some(saved_baseline.label.as_str())
        );
        let checklist_path_text = checklist_path.display().to_string();
        assert_eq!(
            comparison.baseline_checklist_path.as_deref(),
            Some(checklist_path_text.as_str())
        );
        assert_eq!(
            comparison.baseline_index_schema.as_deref(),
            Some("rusty.studio.shell_handoff_acceptance_baseline_index.v1")
        );
        assert_eq!(
            comparison.baseline_index_path.as_deref(),
            Some(index_path.display().to_string().as_str())
        );
        assert_eq!(
            comparison.baseline_index_default_baseline_id.as_deref(),
            Some(saved_baseline.baseline_id.as_str())
        );
        assert_eq!(
            comparison.baseline_index_selected_baseline_id.as_deref(),
            Some(saved_baseline.baseline_id.as_str())
        );
        assert_eq!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Unchanged
        );
        assert_eq!(comparison.ready_delta, 0);
        assert_eq!(comparison.blocked_delta, 0);
        assert_eq!(comparison.rejected_delta, 0);
        assert_eq!(comparison.entries.len(), 1);
        assert_eq!(
            comparison.entries[0].change,
            StudioShellHandoffAcceptanceComparisonChange::Unchanged
        );
        let status =
            shell_handoff_acceptance_comparison_status(&comparison, &baseline_path, &bundle_root);
        assert!(status.contains("handoff acceptance comparison unchanged"));
        assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains("baseline source: studio.project.makepad_edit rev 1"));
        assert!(status.contains("candidate: studio.project.makepad_edit rev 1"));
        assert!(status.contains("manifest studio.shell_handoffs.studio.project.makepad_edit"));
        assert!(status.contains(&format!("baseline index: {}", index_path.display())));
        assert!(status.contains("default studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains("selected studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains("ready 1->1, delta 0"));
        assert!(status.contains("change unchanged"));
        assert!(status.contains("studio.graph.makepad_edit [desktop]"));
    }

    #[test]
    fn shell_handoff_acceptance_comparison_reports_regression_from_makepad_route() {
        let root = temp_root("shell-handoff-acceptance-comparison-regressed");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        let (_, _, _, _, baseline_path, index_path, _) =
            write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
                .expect("write baseline checklist");
        std::fs::remove_dir_all(selected_shell_bundle_root_dir(&project_path))
            .expect("remove selected shell bundle root");

        let (comparison, _, bundle_root) =
            shell_handoff_acceptance_comparison_for_project_source(&project_path)
                .expect("compare regressed acceptance checklist");

        assert_eq!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Regressed
        );
        assert_eq!(
            comparison.issue_code.as_deref(),
            Some("studio.issue.shell_bundle_file_missing")
        );
        assert_eq!(comparison.ready_delta, -1);
        assert_eq!(comparison.blocked_delta, 1);
        assert_eq!(comparison.rejected_delta, 0);
        assert_eq!(
            comparison.baseline_index_path.as_deref(),
            Some(index_path.display().to_string().as_str())
        );
        assert_eq!(
            comparison.entries[0].change,
            StudioShellHandoffAcceptanceComparisonChange::Regressed
        );
        let status =
            shell_handoff_acceptance_comparison_status(&comparison, &baseline_path, &bundle_root);
        assert!(status.contains("handoff acceptance comparison regressed"));
        assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains(&format!("baseline index: {}", index_path.display())));
        assert!(status.contains("ready 1->0, delta -1"));
        assert!(status.contains("blocked 0->1, delta 1"));
        assert!(status.contains("issue studio.issue.shell_bundle_file_missing"));
        assert!(status.contains("change regressed"));
    }

    #[test]
    fn shell_release_candidate_review_reports_ready_from_makepad_route() {
        let root = temp_root("shell-release-candidate-review");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        write_shell_handoff_manifest_for_project_source(&project_path)
            .expect("write shell handoff manifest");
        write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("write acceptance baseline");
        write_shell_export_package_baseline_for_project_source(&project_path)
            .expect("write export package baseline");

        let (review, output_path) =
            shell_release_candidate_review_for_project_source(&project_path)
                .expect("review shell release candidate");

        assert!(output_path.is_file());
        assert_eq!(
            review.schema_id,
            "rusty.studio.shell_release_candidate_review.v1"
        );
        assert_eq!(
            review.status,
            StudioShellReleaseCandidateReviewStatus::Ready
        );
        assert_eq!(review.issue_code, None);
        assert_eq!(review.execution_policy, "not_executed.review_only");
        assert_eq!(review.review_owner, "rusty.hostess");
        assert_eq!(review.command_session_authority, "rusty.manifold");
        assert_eq!(review.install_launch_evidence_authority, "rusty.hostess");
        assert_eq!(review.studio_role, "authoring.export_planning");
        assert_eq!(review.handoff_status, StudioValidationStatus::Pass);
        assert_eq!(review.handoff_ready_count, 1);
        assert_eq!(review.handoff_failed_count, 0);
        assert_eq!(review.handoff_missing_bundle_count, 0);
        assert_eq!(
            review.acceptance_baseline_selection.status,
            StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
        );
        assert_eq!(
            review
                .acceptance_comparison
                .as_ref()
                .map(|comparison| comparison.status),
            Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
        );
        assert_eq!(
            review.export_package_baseline_selection.status,
            StudioShellExportPackageBaselineSelectionStatus::Selected
        );
        assert_eq!(
            review
                .export_package_comparison
                .as_ref()
                .map(|comparison| comparison.status),
            Some(StudioShellExportPackageComparisonStatus::Unchanged)
        );
        assert!(review
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));

        let status = shell_release_candidate_review_status(&review, &output_path);
        assert!(status.contains("shell release candidate review ready"));
        assert!(status.contains("acceptance baseline: selected"));
        assert!(status.contains("comparison unchanged"));
        assert!(status.contains("export package baseline: selected"));
        assert!(status.contains("checks:"));
        assert!(status.contains("failed 0"));
        assert!(status.contains("not_executed.review_only"));
    }

    #[test]
    fn shell_release_candidate_review_index_cycles_default_from_makepad_route() {
        let root = temp_root("shell-release-candidate-index");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        write_shell_handoff_manifest_for_project_source(&project_path)
            .expect("write shell handoff manifest");
        write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("write acceptance baseline");
        write_shell_export_package_baseline_for_project_source(&project_path)
            .expect("write export package baseline");

        let (review, candidate, index, review_path, candidate_path, index_path) =
            write_shell_release_candidate_review_manifest_for_project_source(&project_path)
                .expect("write release candidate manifest");

        assert!(review_path.is_file());
        assert!(candidate_path.is_file());
        assert!(index_path.is_file());
        assert_eq!(
            candidate.schema_id,
            "rusty.studio.shell_release_candidate_review_manifest.v1"
        );
        assert_eq!(
            candidate.candidate_id,
            "studio.project.makepad_edit.rev1.ready"
        );
        assert_eq!(
            candidate.status,
            StudioShellReleaseCandidateReviewStatus::Ready
        );
        assert_eq!(candidate.review_path, review_path.display().to_string());
        assert_eq!(
            index.schema_id,
            "rusty.studio.shell_release_candidate_review_index.v1"
        );
        assert_eq!(
            index.default_candidate_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        assert_eq!(index.candidate_count, 1);
        assert_eq!(index.ready_candidate_count, 1);
        assert_eq!(index.blocked_candidate_count, 0);
        let status = shell_release_candidate_review_manifest_status(
            &review,
            &candidate,
            &index,
            &review_path,
            &candidate_path,
            &index_path,
        );
        assert!(status.contains("release candidate written"));
        assert!(status.contains("release candidate selection selected"));
        assert!(status.contains("release candidate index slots 1"));

        let (_, archived_candidate, archived_index, _, archived_candidate_path, _) =
            append_shell_release_candidate_review_manifest_for_project_source(&project_path)
                .expect("archive release candidate");

        assert_eq!(
            archived_candidate.candidate_id,
            "studio.project.makepad_edit.rev1.ready.archive2"
        );
        assert_eq!(
            archived_index.default_candidate_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready.archive2")
        );
        assert_eq!(archived_index.candidate_count, 2);
        assert_eq!(archived_index.ready_candidate_count, 2);
        assert_eq!(
            archived_index.entries[1].candidate_manifest_path.as_deref(),
            Some(archived_candidate_path.display().to_string().as_str())
        );
        let loaded_index =
            load_shell_release_candidate_review_index(&index_path).expect("load candidate index");
        assert_eq!(loaded_index, archived_index);

        let (selected_ready, selected_index, selected_candidate_path, loaded_index_path) =
            select_next_shell_release_candidate_default_for_project_source(&project_path)
                .expect("select next release candidate");
        assert_eq!(selected_ready, candidate);
        assert_eq!(selected_candidate_path, candidate_path);
        assert_eq!(
            selected_index.default_candidate_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        let status = shell_release_candidate_review_manifest_select_status(
            &selected_ready,
            &selected_index,
            &selected_candidate_path,
            &loaded_index_path,
        );
        assert!(status.contains("release candidate default selected"));
        assert!(status.contains(
            "release candidate selection selected; requested studio.project.makepad_edit.rev1.ready; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));

        let (promoted_candidate, promoted_index, promoted_candidate_path, loaded_index_path) =
            promote_shell_release_candidate_default_for_project_source(&project_path)
                .expect("promote saved release candidate");
        assert_eq!(promoted_candidate, candidate);
        assert_eq!(promoted_candidate_path, candidate_path);
        assert_eq!(
            promoted_index.default_candidate_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        let status = shell_release_candidate_review_manifest_promote_status(
            &promoted_candidate,
            &promoted_index,
            &promoted_candidate_path,
            &loaded_index_path,
        );
        assert!(status.contains("release candidate default promoted"));
        assert!(status.contains("release candidate index slots 2"));
    }

    #[test]
    fn shell_hostess_handoff_package_reports_ready_from_makepad_route() {
        let root = temp_root("shell-hostess-handoff-package");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
            .expect("load view model");
        export_shell_bundle_for_project_source(&project_path, &model, 0)
            .expect("export selected shell bundle");
        write_shell_handoff_manifest_for_project_source(&project_path)
            .expect("write shell handoff manifest");
        write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("write acceptance baseline");
        write_shell_export_package_baseline_for_project_source(&project_path)
            .expect("write export package baseline");
        write_shell_release_candidate_review_manifest_for_project_source(&project_path)
            .expect("write release candidate manifest");

        let (package, output_path) =
            shell_hostess_handoff_package_for_project_source(&project_path)
                .expect("review shell Hostess handoff package");

        assert!(output_path.is_file());
        assert_eq!(
            package.schema_id,
            "rusty.studio.shell_hostess_handoff_package.v1"
        );
        assert_eq!(
            package.status,
            StudioShellHostessHandoffPackageStatus::Ready
        );
        assert_eq!(package.issue_code, None);
        assert_eq!(
            package.selected_candidate_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        assert_eq!(
            package.command_session_authority.as_deref(),
            Some("rusty.manifold")
        );
        assert_eq!(
            package.install_launch_evidence_authority.as_deref(),
            Some("rusty.hostess")
        );
        assert_eq!(
            package.studio_role.as_deref(),
            Some("authoring.export_planning")
        );
        assert_eq!(package.handoff_ready_count, 1);
        assert_eq!(package.handoff_failed_count, 0);
        assert_eq!(package.handoff_missing_bundle_count, 0);
        assert_eq!(
            package.acceptance_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        assert_eq!(
            package.acceptance_comparison_status,
            Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
        );
        assert_eq!(
            package.export_package_baseline_id.as_deref(),
            Some("studio.project.makepad_edit.rev1.ready")
        );
        assert_eq!(
            package.export_package_comparison_status,
            Some(StudioShellExportPackageComparisonStatus::Unchanged)
        );
        assert!(package.required_owner_actions.iter().any(|action| {
            action.action_id == "hostess.collect_install_launch_evidence"
                && action.owner == "rusty.hostess"
                && action.status == StudioShellHostessHandoffPackageActionStatus::Ready
                && action.prohibited_in_studio
        }));
        assert!(package
            .prohibited_actions
            .contains(&"stage_generated_shells".to_string()));
        assert!(package
            .prohibited_actions
            .contains(&"collect_install_launch_evidence".to_string()));
        assert!(package
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));

        let status = shell_hostess_handoff_package_status(&package, &output_path);
        assert!(status.contains("shell Hostess handoff package ready"));
        assert!(status.contains("selected studio.project.makepad_edit.rev1.ready"));
        assert!(status.contains("hostess.collect_install_launch_evidence [ready]"));
        assert!(status.contains("manifold.review_command_session_contract [ready]"));
        assert!(status.contains("prohibited:"));
        assert!(status.contains("stage_generated_shells"));
        assert!(status.contains("checks:"));
        assert!(status.contains("failed 0"));

        let (intake, intake_path) = shell_hostess_owner_intake_for_project_source(&project_path)
            .expect("review shell Hostess owner intake");
        assert!(intake_path.is_file());
        assert_eq!(
            intake.schema_id,
            "rusty.studio.shell_hostess_owner_intake.v1"
        );
        assert_eq!(intake.status, StudioShellHostessOwnerIntakeStatus::Ready);
        assert_eq!(intake.issue_code, None);
        assert_eq!(intake.execution_policy, "not_executed.request_only");
        assert_eq!(intake.intake_owner, "rusty.hostess");
        assert_eq!(
            intake.package_path.as_deref(),
            Some(output_path.display().to_string().as_str())
        );
        assert_eq!(intake.ready_assignment_count, 4);
        assert_eq!(intake.blocked_assignment_count, 0);
        assert_eq!(intake.hostess_ready_action_count, 3);
        assert_eq!(intake.manifold_ready_action_count, 1);
        assert!(intake.assignments.iter().any(|assignment| {
            assignment.action_id == "hostess.stage_generated_shells"
                && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
                && assignment.request_kind == "hostess_owner_action_request"
                && assignment.prohibited_in_studio
        }));
        assert!(intake.assignments.iter().any(|assignment| {
            assignment.action_id == "manifold.review_command_session_contract"
                && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
                && assignment.request_kind == "manifold_owner_review_request"
        }));
        assert!(intake
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));

        let intake_status = shell_hostess_owner_intake_status(&intake, &intake_path);
        assert!(intake_status.contains("shell Hostess owner intake ready"));
        assert!(intake_status.contains("assignments ready 4; blocked 0"));
        assert!(intake_status.contains("Hostess ready 3; Manifold ready 1"));
        assert!(intake_status.contains("not_executed.request_only"));
        assert!(intake_status.contains("hostess_owner_action_request"));
        assert!(intake_status.contains("manifold_owner_review_request"));

        let (staging, staging_path) =
            shell_hostess_staging_preview_for_project_source(&project_path)
                .expect("preview shell Hostess staging");
        assert!(staging_path.is_file());
        assert_eq!(
            staging.schema_id,
            "rusty.studio.shell_hostess_staging_preview_manifest.v1"
        );
        assert_eq!(
            staging.status,
            StudioShellHostessStagingPreviewStatus::Ready
        );
        assert_eq!(staging.issue_code, None);
        assert_eq!(staging.execution_policy, "not_executed.preview_only");
        assert_eq!(staging.staging_owner, "rusty.hostess");
        assert_eq!(
            staging.intake_path.as_deref(),
            Some(intake_path.display().to_string().as_str())
        );
        assert_eq!(staging.assignment_count, 4);
        assert_eq!(staging.ready_assignment_count, 4);
        assert_eq!(staging.blocked_assignment_count, 0);
        assert_eq!(staging.ready_group_count, 4);
        assert_eq!(staging.blocked_group_count, 0);
        assert!(staging.expected_artifact_count >= 10);
        let stage_group = staging
            .groups
            .iter()
            .find(|group| group.action_id == "hostess.stage_generated_shells")
            .expect("stage generated shells group");
        assert_eq!(stage_group.route_kind, "hostess.stage.generated_shells");
        assert_eq!(
            stage_group.status,
            StudioShellHostessStagingPreviewGroupStatus::Ready
        );
        assert!(stage_group
            .expected_artifacts
            .iter()
            .any(|artifact| artifact.artifact_kind == "shell_descriptor"));
        assert!(stage_group
            .expected_artifacts
            .iter()
            .any(|artifact| artifact.artifact_kind == "shell_template_manifest"));
        assert!(staging
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass));

        let staging_status = shell_hostess_staging_preview_status(&staging, &staging_path);
        assert!(staging_status.contains("shell Hostess staging preview ready"));
        assert!(staging_status.contains("route hostess.stage.generated_shells"));
        assert!(staging_status.contains("not_executed.preview_only"));
        assert!(staging_status.contains("groups ready 4; blocked 0"));
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
