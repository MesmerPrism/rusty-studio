pub use makepad_widgets;

mod app_events;
mod canvas;
mod paths;
mod project_actions;
mod script_ui;
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
        script_ui::script_mod(vm)
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
