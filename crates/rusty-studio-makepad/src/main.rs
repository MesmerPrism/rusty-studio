pub use makepad_widgets;

mod app_events;
mod canvas;
mod paths;
mod project_actions;
mod selection;
mod shell_workflows;
mod status_text;

use canvas::{
    graph_canvas_model, StudioGraphCanvas, StudioGraphCanvasAction, StudioGraphCanvasModel,
};
use makepad_widgets::*;
use paths::{
    default_project_working_copy_path, find_default_project_path, initial_edge_id_from_args,
    initial_graph_id_from_args, initial_issue_check_id_from_args, initial_node_id_from_args,
    project_path_for_mutable_session, project_path_from_args,
};
use project_actions::{
    add_binding_to_project_source, add_next_catalog_module_to_project_source,
    canvas_selection_view_model_for_project_source, load_studio_view_model_for_path,
    remove_binding_from_project_source, remove_module_from_project_source, retarget_project_source,
    selected_graph_id_for_model,
};
use rusty_studio_core::{
    append_shell_export_package_baseline_index_manifests,
    append_shell_handoff_acceptance_baseline_index_manifests,
    append_shell_hostess_staging_acceptance_index_manifests,
    append_shell_release_candidate_review_index_manifests,
    compare_shell_export_packages_against_baseline_index_entry,
    compare_shell_handoff_acceptance_against_baseline_index_entry,
    compare_shell_hostess_staging_acceptance_against_index_entry, load_project,
    load_shell_export_package_baseline_index, load_shell_export_package_baseline_manifest,
    load_shell_export_package_report, load_shell_handoff_acceptance_baseline_index,
    load_shell_handoff_acceptance_baseline_manifest, load_shell_handoff_acceptance_checklist,
    load_shell_handoff_manifest, load_shell_hostess_handoff_package_report,
    load_shell_hostess_owner_intake_report, load_shell_hostess_staging_acceptance_checklist,
    load_shell_hostess_staging_acceptance_index, load_shell_hostess_staging_acceptance_manifest,
    load_shell_hostess_staging_file_plan, load_shell_hostess_staging_handoff_envelope,
    load_shell_hostess_staging_preview_manifest, load_shell_release_candidate_review_index,
    load_shell_release_candidate_review_manifest,
    promote_shell_export_package_baseline_index_default,
    promote_shell_handoff_acceptance_baseline_index_default,
    promote_shell_hostess_staging_acceptance_index_default,
    promote_shell_release_candidate_review_index_default, save_json, save_shell_bundle,
    select_shell_export_package_baseline_index_entry,
    select_shell_handoff_acceptance_baseline_index_entry,
    select_shell_hostess_staging_acceptance_index_entry, selected_shell_bundle_for_graph,
    shell_export_package_baseline_index_for_manifests,
    shell_export_package_baseline_manifest_for_report, shell_export_package_for_project,
    shell_handoff_acceptance_baseline_index_for_manifests,
    shell_handoff_acceptance_baseline_manifest_for_checklist,
    shell_handoff_acceptance_checklist_for_project, shell_handoff_for_bundle,
    shell_handoff_manifest_for_project, shell_handoff_readiness_for_project,
    shell_hostess_handoff_package_for_release_candidate_index,
    shell_hostess_owner_intake_for_handoff_package,
    shell_hostess_staging_acceptance_checklist_for_handoff,
    shell_hostess_staging_acceptance_index_for_manifests,
    shell_hostess_staging_acceptance_manifest_for_checklist,
    shell_hostess_staging_execution_request_for_acceptance_index_entry,
    shell_hostess_staging_file_plan_for_preview,
    shell_hostess_staging_handoff_envelope_for_file_plan,
    shell_hostess_staging_preview_for_owner_intake, shell_release_candidate_review_for_manifest,
    shell_release_candidate_review_index_for_manifests,
    shell_release_candidate_review_manifest_for_report, shell_runbook_for_project,
    validate_selected_shell_bundle,
};
use rusty_studio_model::{
    StudioBindingKind, StudioEditReport, StudioGraphView, StudioShellBundleReport,
    StudioShellBundleStatus, StudioShellBundleValidationReport,
    StudioShellExportPackageBaselineIndex, StudioShellExportPackageBaselineManifest,
    StudioShellExportPackageComparisonReport, StudioShellExportPackageReport,
    StudioShellHandoffAcceptanceBaselineIndex, StudioShellHandoffAcceptanceBaselineManifest,
    StudioShellHandoffAcceptanceChecklistReport, StudioShellHandoffAcceptanceComparisonReport,
    StudioShellHandoffManifest, StudioShellHandoffReadinessReport, StudioShellHandoffReport,
    StudioShellHostessHandoffPackageReport, StudioShellHostessOwnerIntakeReport,
    StudioShellHostessStagingAcceptanceChecklistReport,
    StudioShellHostessStagingAcceptanceComparisonReport, StudioShellHostessStagingAcceptanceIndex,
    StudioShellHostessStagingAcceptanceManifest, StudioShellHostessStagingExecutionRequestReport,
    StudioShellHostessStagingFilePlan, StudioShellHostessStagingHandoffEnvelope,
    StudioShellHostessStagingPreviewManifest, StudioShellReleaseCandidateReviewIndex,
    StudioShellReleaseCandidateReviewManifest, StudioShellReleaseCandidateReviewReport,
    StudioShellRunbookReport, StudioViewModel,
};
use selection::{
    selected_binding_request, selected_command_binding_request, selected_module_reference_id,
    selected_package_reference_id,
};
use shell_workflows::*;
use status_text::*;
use std::path::{Path, PathBuf};

app_main!(App);

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
            shell_hostess_staging_file_plan_button := ActionButton{text: "Plan Staging Files"}
            shell_hostess_staging_handoff_button := ActionButton{text: "Prepare Hostess Handoff"}
            shell_hostess_staging_acceptance_button := ActionButton{text: "Check Hostess Handoff"}
            shell_hostess_staging_acceptance_append_button := ActionButton{text: "Archive Hostess Check"}
            shell_hostess_staging_acceptance_summary_button := ActionButton{text: "Inspect Hostess Checks"}
            shell_hostess_staging_acceptance_next_button := ActionButton{text: "Next Hostess Check"}
            shell_hostess_staging_acceptance_promote_button := ActionButton{text: "Promote Hostess Check"}
            shell_hostess_staging_acceptance_compare_button := ActionButton{text: "Compare Hostess Check"}
            shell_hostess_staging_execution_request_button := ActionButton{text: "Request Hostess Adapter"}
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

#[cfg(test)]
mod tests;
