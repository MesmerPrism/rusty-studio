pub use makepad_widgets;

use makepad_widgets::*;
use rusty_studio_core::{
    add_binding_to_graph, add_next_catalog_module_to_graph, load_project,
    remove_binding_from_graph, remove_module_from_graph, retarget_graph_host_profile, save_project,
    view_model_for_graph,
};
use rusty_studio_model::{
    StudioBindingKind, StudioEditReport, StudioEditStatus, StudioGraphView, StudioValidationStatus,
    StudioViewModel,
};
use std::path::{Path, PathBuf};

app_main!(App);

const DEFAULT_REMOVE_MODULE_REF: &str = "module.biosignal_sensor.provider";
const DEFAULT_COMMAND_SOURCE_NODE: &str = "node.shell.operator";
const DEFAULT_COMMAND_TARGET_NODE: &str = "node.module.synthetic_wave_provider";

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

    let ProjectPanel = Panel{
        SectionTitle{text: "Project"}
        Row{FieldLabel{text: "source"} project_source := SmallValue{text: ""}}
        Row{FieldLabel{text: "project"} project_identity := FieldValue{text: ""}}
        Row{FieldLabel{text: "revision"} project_revision := FieldValue{text: ""}}
        Row{FieldLabel{text: "validation"} validation_status := FieldValue{text: ""}}
    }

    let DiagnosticsPanel = Panel{
        SectionTitle{text: "Validation Diagnostics"}
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
            add_palette_module_button := ActionButton{text: "Add Palette Module"}
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
            remove_biosignal_module_button := ActionButton{text: "Remove Biosignal"}
            add_command_binding_button := ActionButton{text: "Add Command"}
            remove_command_binding_button := ActionButton{text: "Remove Command"}
        }
        Row{FieldLabel{text: "status"} edit_status := FieldValue{text: "no edits requested"}}
        Row{FieldLabel{text: "message"} edit_message := SmallValue{text: ""}}
        Row{FieldLabel{text: "changed"} edit_changed_fields := SmallValue{text: ""}}
        Row{FieldLabel{text: "validation"} edit_validation := SmallValue{text: ""}}
    }

    let CanvasPanel = Panel{
        SectionTitle{text: "Read-Only Graph Canvas"}
        Row{FieldLabel{text: "nodes"} graph_nodes := SmallValue{text: ""}}
        Rule{}
        Row{FieldLabel{text: "edges"} graph_edges := SmallValue{text: ""}}
    }

    let InspectorPanel = Panel{
        SectionTitle{text: "Inspector"}
        Row{FieldLabel{text: "selected node"} selected_node := FieldValue{text: ""}}
        Row{FieldLabel{text: "selected ref"} selected_reference := SmallValue{text: ""}}
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
    last_edit_report: Option<StudioEditReport>,
    #[rust]
    last_edit_save_issue: String,
}

impl App {
    fn sync_project(&mut self, cx: &mut Cx) {
        match load_studio_view_model(initial_graph_id_from_args().as_deref()) {
            Ok((source, model)) => self.set_model(cx, source, model),
            Err(error) => self.sync_error(cx, &error),
        }
    }

    fn set_model(&mut self, cx: &mut Cx, source: PathBuf, model: StudioViewModel) {
        self.selected_graph_index = model.selected_graph_index.unwrap_or(0);
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
            .label(cx, ids!(graph_nodes))
            .set_text(cx, &node_lines(graph));
        self.ui
            .label(cx, ids!(graph_edges))
            .set_text(cx, &edge_lines(graph));
        if let Some(node) = graph.node_rows.first() {
            self.ui
                .label(cx, ids!(selected_node))
                .set_text(cx, &format!("{} / {}", node.label, node.kind));
            self.ui
                .label(cx, ids!(selected_reference))
                .set_text(cx, &node.reference_id);
        }
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
        match add_next_catalog_module_to_project_source(&source, &model, self.selected_graph_index)
        {
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
        self.ui.label(cx, ids!(graph_nodes)).set_text(cx, "");
        self.ui.label(cx, ids!(graph_edges)).set_text(cx, "");
        self.ui.label(cx, ids!(selected_node)).set_text(cx, "");
        self.ui.label(cx, ids!(selected_reference)).set_text(cx, "");
    }

    fn select_previous_graph(&mut self, cx: &mut Cx) {
        let graph_count = self.model.as_ref().map_or(0, |model| model.graphs.len());
        if graph_count == 0 {
            return;
        }
        self.selected_graph_index = if self.selected_graph_index == 0 {
            graph_count - 1
        } else {
            self.selected_graph_index - 1
        };
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
    }

    fn select_next_graph(&mut self, cx: &mut Cx) {
        let graph_count = self.model.as_ref().map_or(0, |model| model.graphs.len());
        if graph_count == 0 {
            return;
        }
        self.selected_graph_index = (self.selected_graph_index + 1) % graph_count;
        self.sync_loaded_model(cx);
        self.ui.redraw(cx);
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
            .button(cx, ids!(remove_biosignal_module_button))
            .clicked(actions)
        {
            self.remove_module_from_selected_graph(cx, DEFAULT_REMOVE_MODULE_REF);
        }
        if self
            .ui
            .button(cx, ids!(add_command_binding_button))
            .clicked(actions)
        {
            self.add_binding_to_selected_graph(
                cx,
                StudioBindingKind::Command,
                DEFAULT_COMMAND_SOURCE_NODE,
                DEFAULT_COMMAND_TARGET_NODE,
            );
        }
        if self
            .ui
            .button(cx, ids!(remove_command_binding_button))
            .clicked(actions)
        {
            self.remove_binding_from_selected_graph(
                cx,
                StudioBindingKind::Command,
                DEFAULT_COMMAND_SOURCE_NODE,
                DEFAULT_COMMAND_TARGET_NODE,
            );
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
) -> Result<(PathBuf, StudioViewModel), String> {
    let project_path = project_path_from_args()
        .or_else(find_default_project_path)
        .ok_or_else(|| "no project path supplied and default example was not found".to_string())?;
    let model = load_studio_view_model_for_path(&project_path, requested_graph_id)?;
    Ok((project_path, model))
}

fn load_studio_view_model_for_path(
    project_path: &Path,
    requested_graph_id: Option<&str>,
) -> Result<StudioViewModel, String> {
    let project = load_project(&project_path).map_err(|error| error.to_string())?;
    Ok(view_model_for_graph(
        &project,
        project_path.parent(),
        requested_graph_id,
    ))
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
) -> Result<(StudioEditReport, Option<StudioViewModel>), String> {
    let graph_id = selected_graph_id_for_model(model, selected_graph_index)
        .ok_or_else(|| "No graph is selected".to_string())?;
    let mut project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let report = add_next_catalog_module_to_graph(&mut project, &graph_id, project_path.parent());
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

fn node_lines(graph: &StudioGraphView) -> String {
    graph
        .node_rows
        .iter()
        .map(|node| {
            format!(
                "{} [{}]\n  ref: {}",
                node.label, node.kind, node.reference_id
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
                "{} [{}]\n  {} -> {}",
                edge.edge_id, edge.kind, edge.source_node_id, edge.target_node_id
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusty_studio_model::{
        StudioEdge, StudioEdgeKind, StudioEditOperation, StudioGraph, StudioNode, StudioNodeKind,
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
            }],
        }
    }

    #[test]
    fn retarget_project_source_saves_and_refreshes_view_model() {
        let root = temp_root("retarget-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None).expect("load view model");

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
        let model = load_studio_view_model_for_path(&project_path, None).expect("load view model");

        let package_lines = catalog_package_lines(&model);
        assert!(package_lines.contains("package.synthetic [selected; 1 module(s)]"));
        assert!(package_lines.contains("module.synthetic_provider"));
        assert!(package_lines.contains("packages/synthetic/manifests/package.manifold.json"));

        let profile_lines = host_profile_lines(&model);
        assert!(profile_lines.contains("host_run.profile.desktop [target]"));
        assert!(profile_lines.contains("host: host.desktop"));
        assert!(profile_lines.contains("host_run.profile.headset [available]"));
    }

    #[test]
    fn validation_issue_lines_render_failed_checks() {
        let root = temp_root("validation-issue-lines");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        let mut project = editable_project();
        project.graphs[0].nodes[0].reference_id = "package.missing".to_string();
        save_project(&project_path, &project).expect("save invalid project");
        let model = load_studio_view_model_for_path(&project_path, None).expect("load view model");

        let issue_lines = validation_issue_lines(&model);
        assert!(issue_lines.contains("studio.check.graph.studio.graph.makepad_edit.package_refs"));
        assert!(issue_lines.contains("studio.issue.package_reference_missing"));
        assert!(issue_lines.contains("selected graph: studio.graph.makepad_edit"));
        assert!(issue_lines.contains("refs: package.missing"));
        assert!(issue_lines.contains("package references missing from catalog"));
    }

    #[test]
    fn add_next_catalog_module_to_project_source_saves_and_refreshes_view_model() {
        let root = temp_root("add-next-palette-module-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None).expect("load view model");

        let (report, refreshed_model) =
            add_next_catalog_module_to_project_source(&project_path, &model, 0)
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
    fn add_module_to_project_source_saves_and_refreshes_view_model() {
        let root = temp_root("add-module-source");
        write_reference_fixture_tree(&root);
        let project_path = root.join("project.json");
        save_project(&project_path, &editable_project()).expect("save editable project");
        let model = load_studio_view_model_for_path(&project_path, None).expect("load view model");

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
        let model = load_studio_view_model_for_path(&project_path, None).expect("load view model");

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
        let model = load_studio_view_model_for_path(&project_path, None).expect("load view model");

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
        let model = load_studio_view_model_for_path(&project_path, None).expect("load view model");

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
}
