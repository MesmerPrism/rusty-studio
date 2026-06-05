use clap::Parser;
use rusty_studio_core::{
    add_binding_to_graph, add_module_to_graph, add_next_catalog_module_from_package_to_graph,
    add_next_catalog_module_to_graph, append_shell_export_package_baseline_index_manifests,
    append_shell_handoff_acceptance_baseline_index_manifests,
    append_shell_hostess_staging_acceptance_index_manifests,
    append_shell_release_candidate_review_index_manifests, compare_shell_export_packages,
    compare_shell_export_packages_against_baseline_index_entry,
    compare_shell_export_packages_against_baseline_manifest,
    compare_shell_handoff_acceptance_against_baseline_index_entry,
    compare_shell_handoff_acceptance_against_baseline_manifest,
    compare_shell_handoff_acceptance_checklists,
    compare_shell_hostess_staging_acceptance_against_index_entry,
    compare_shell_hostess_staging_acceptance_against_manifest,
    compare_shell_hostess_staging_acceptance_checklists, desktop_shell_handoff_for_bundle,
    load_manifold_package_validation_report, load_motion_breath_profile_document,
    load_package_evidence_intake_report, load_project,
    load_projected_motion_breath_adapter_normalization_case_document,
    load_projected_motion_breath_authoring_review_report,
    load_projected_motion_breath_shell_handoff_evidence,
    load_projected_motion_breath_shell_handoff_review_report,
    load_projected_motion_breath_source_adapter_descriptors,
    load_projected_motion_breath_source_adapter_selection_review_report,
    load_projected_motion_breath_source_binding_document, load_shell_artifact_manifest,
    load_shell_descriptor, load_shell_export_package_baseline_index,
    load_shell_export_package_baseline_manifest, load_shell_export_package_report,
    load_shell_handoff_acceptance_baseline_index, load_shell_handoff_acceptance_baseline_manifest,
    load_shell_handoff_acceptance_checklist, load_shell_handoff_intake_report,
    load_shell_handoff_manifest, load_shell_hostess_handoff_package_report,
    load_shell_hostess_owner_intake_report, load_shell_hostess_staging_acceptance_checklist,
    load_shell_hostess_staging_acceptance_index, load_shell_hostess_staging_acceptance_manifest,
    load_shell_hostess_staging_file_plan, load_shell_hostess_staging_handoff_envelope,
    load_shell_hostess_staging_preview_manifest, load_shell_release_candidate_review_index,
    load_shell_release_candidate_review_manifest, load_shell_release_candidate_review_report,
    load_shell_template_index, package_evidence_intake_for_validation_report,
    projected_motion_breath_adapter_normalization_evidence_review_for_selection,
    projected_motion_breath_authoring_review_for_intake,
    projected_motion_breath_shell_handoff_review_for_evidence,
    projected_motion_breath_source_adapter_selection_review_for_authoring,
    promote_shell_export_package_baseline_index_default,
    promote_shell_handoff_acceptance_baseline_index_default,
    promote_shell_hostess_staging_acceptance_index_default,
    promote_shell_release_candidate_review_index_default, remove_binding_from_graph,
    remove_module_from_graph, resolve_project, retarget_graph_host_profile, save_json,
    save_project, save_shell_bundle, select_shell_export_package_baseline_index_entry,
    select_shell_handoff_acceptance_baseline_index_entry,
    select_shell_hostess_staging_acceptance_index_entry, selected_shell_bundle_for_graph,
    shell_artifacts_for_project, shell_descriptor_artifact_path, shell_descriptor_for_graph,
    shell_export_package_baseline_index_for_manifests,
    shell_export_package_baseline_manifest_for_report, shell_export_package_for_manifest,
    shell_export_package_for_project, shell_handoff_acceptance_baseline_index_for_manifests,
    shell_handoff_acceptance_baseline_manifest_for_checklist,
    shell_handoff_acceptance_checklist_for_intake, shell_handoff_acceptance_checklist_for_project,
    shell_handoff_for_bundle, shell_handoff_intake_for_manifest,
    shell_handoff_manifest_for_project, shell_handoff_readiness_for_project,
    shell_hostess_handoff_package_for_release_candidate_index,
    shell_hostess_owner_intake_for_handoff_package,
    shell_hostess_staging_acceptance_checklist_for_handoff,
    shell_hostess_staging_acceptance_index_for_manifests,
    shell_hostess_staging_acceptance_manifest_for_checklist,
    shell_hostess_staging_execution_request_for_acceptance_index_entry_with_pmb_review,
    shell_hostess_staging_file_plan_for_preview,
    shell_hostess_staging_handoff_envelope_for_file_plan,
    shell_hostess_staging_preview_for_owner_intake, shell_release_candidate_review_for_manifest,
    shell_release_candidate_review_index_for_manifests,
    shell_release_candidate_review_manifest_for_report, shell_runbook_for_project,
    shell_templates_for_artifact_manifest, summarize_shell_export_package_baseline_index_selection,
    summarize_shell_handoff_acceptance_baseline_index_selection,
    summarize_shell_handoff_acceptance_checklist,
    summarize_shell_hostess_staging_acceptance_index_selection,
    summarize_shell_release_candidate_review_index_selection, validate_project_with_base,
    validate_selected_shell_bundle, validate_shell_artifact_manifest, validate_shell_descriptor,
    validate_shell_handoff_manifest, validate_shell_template_index,
    view_model_for_graph_issue_node_and_edge,
};
use rusty_studio_model::{
    StudioEditStatus, StudioShellArtifactStatus, StudioShellBundleStatus,
    StudioShellDescriptorStatus, StudioShellTemplateStatus,
};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod args;
mod graph_edit_commands;
mod hostess_commands;
mod project_commands;
mod projected_motion_breath_commands;
mod release_candidate_commands;
mod shell_generation_commands;
mod shell_handoff_acceptance_commands;
mod shell_package_commands;
use args::*;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Command::Validate(args) => project_commands::validate(args),
        Command::Resolve(args) => project_commands::resolve(args),
        Command::ExportPlan(args) => project_commands::export_plan(args),
        Command::ViewModel(args) => project_commands::view_model(args),
        Command::RetargetHost(args) => graph_edit_commands::retarget_host(args),
        Command::AddModule(args) => graph_edit_commands::add_module(args),
        Command::AddPaletteModule(args) => graph_edit_commands::add_palette_module(args),
        Command::RemoveModule(args) => graph_edit_commands::remove_module(args),
        Command::AddBinding(args) => graph_edit_commands::add_binding(args),
        Command::RemoveBinding(args) => graph_edit_commands::remove_binding(args),
        Command::ShellDescriptor(args) => shell_generation_commands::descriptor(args),
        Command::ValidateShellDescriptor(args) => {
            shell_generation_commands::validate_descriptor(args)
        }
        Command::ShellArtifacts(args) => shell_generation_commands::artifacts(args),
        Command::ValidateShellArtifacts(args) => {
            shell_generation_commands::validate_artifacts(args)
        }
        Command::ShellTemplates(args) => shell_generation_commands::templates(args),
        Command::ValidateShellTemplates(args) => {
            shell_generation_commands::validate_templates(args)
        }
        Command::ShellBundle(args) => shell_generation_commands::bundle(args),
        Command::ValidateShellBundle(args) => shell_generation_commands::validate_bundle(args),
        Command::ShellHandoff(args) => shell_generation_commands::handoff(args),
        Command::DesktopShellHandoff(args) => shell_generation_commands::desktop_handoff(args),
        Command::ShellHandoffReadiness(args) => shell_generation_commands::handoff_readiness(args),
        Command::ShellHandoffManifest(args) => shell_generation_commands::handoff_manifest(args),
        Command::ValidateShellHandoffManifest(args) => {
            shell_generation_commands::validate_handoff_manifest(args)
        }
        Command::ShellHandoffIntake(args) => shell_generation_commands::handoff_intake(args),
        Command::ShellRunbook(args) => shell_generation_commands::runbook(args),
        Command::ShellExportPackage(args) => shell_package_commands::export_package(args),
        Command::ShellExportPackageBaseline(args) => shell_package_commands::baseline(args),
        Command::ShellExportPackageBaselineIndex(args) => {
            shell_package_commands::baseline_index(args)
        }
        Command::ShellExportPackageBaselineIndexAppend(args) => {
            shell_package_commands::baseline_index_append(args)
        }
        Command::ShellExportPackageBaselineIndexPromote(args) => {
            shell_package_commands::baseline_index_promote(args)
        }
        Command::ShellExportPackageBaselineSelection(args) => {
            shell_package_commands::baseline_selection(args)
        }
        Command::ShellExportPackageComparison(args) => shell_package_commands::comparison(args),
        Command::ShellHandoffAcceptanceChecklist(args) => {
            shell_handoff_acceptance_commands::checklist(args)
        }
        Command::ShellHandoffAcceptanceSnapshot(args) => {
            shell_handoff_acceptance_commands::snapshot(args)
        }
        Command::ShellHandoffAcceptanceSummary(args) => {
            shell_handoff_acceptance_commands::summary(args)
        }
        Command::ShellHandoffAcceptanceBaseline(args) => {
            shell_handoff_acceptance_commands::baseline(args)
        }
        Command::ShellHandoffAcceptanceBaselineIndex(args) => {
            shell_handoff_acceptance_commands::baseline_index(args)
        }
        Command::ShellHandoffAcceptanceBaselineIndexAppend(args) => {
            shell_handoff_acceptance_commands::baseline_index_append(args)
        }
        Command::ShellHandoffAcceptanceBaselineIndexPromote(args) => {
            shell_handoff_acceptance_commands::baseline_index_promote(args)
        }
        Command::ShellHandoffAcceptanceBaselineSelection(args) => {
            shell_handoff_acceptance_commands::baseline_selection(args)
        }
        Command::ShellHandoffAcceptanceComparison(args) => {
            shell_handoff_acceptance_commands::comparison(args)
        }
        Command::ShellReleaseCandidateReview(args) => release_candidate_commands::review(args),
        Command::ShellReleaseCandidateReviewManifest(args) => {
            release_candidate_commands::manifest(args)
        }
        Command::ShellReleaseCandidateReviewIndex(args) => release_candidate_commands::index(args),
        Command::ShellReleaseCandidateReviewIndexAppend(args) => {
            release_candidate_commands::index_append(args)
        }
        Command::ShellReleaseCandidateReviewIndexPromote(args) => {
            release_candidate_commands::index_promote(args)
        }
        Command::ShellReleaseCandidateReviewSelection(args) => {
            release_candidate_commands::selection(args)
        }
        Command::ShellHostessHandoffPackage(args) => hostess_commands::handoff_package(args),
        Command::ShellHostessOwnerIntake(args) => hostess_commands::owner_intake(args),
        Command::ShellHostessStagingPreview(args) => hostess_commands::staging_preview(args),
        Command::ShellHostessStagingFilePlan(args) => hostess_commands::staging_file_plan(args),
        Command::ShellHostessStagingHandoff(args) => hostess_commands::staging_handoff(args),
        Command::ShellHostessStagingAcceptanceChecklist(args) => {
            hostess_commands::staging_acceptance_checklist(args)
        }
        Command::ShellHostessStagingAcceptanceManifest(args) => {
            hostess_commands::staging_acceptance_manifest(args)
        }
        Command::ShellHostessStagingAcceptanceIndex(args) => {
            hostess_commands::staging_acceptance_index(args)
        }
        Command::ShellHostessStagingAcceptanceIndexAppend(args) => {
            hostess_commands::staging_acceptance_index_append(args)
        }
        Command::ShellHostessStagingAcceptanceIndexPromote(args) => {
            hostess_commands::staging_acceptance_index_promote(args)
        }
        Command::ShellHostessStagingAcceptanceSelection(args) => {
            hostess_commands::staging_acceptance_selection(args)
        }
        Command::ShellHostessStagingAcceptanceComparison(args) => {
            hostess_commands::staging_acceptance_comparison(args)
        }
        Command::ShellHostessStagingExecutionRequest(args) => {
            hostess_commands::staging_execution_request(args)
        }
        Command::PackageEvidenceIntake(args) => {
            projected_motion_breath_commands::package_evidence_intake(args)
        }
        Command::ProjectedMotionBreathAuthoringReview(args) => {
            projected_motion_breath_commands::authoring_review(args)
        }
        Command::ProjectedMotionBreathSourceAdapterSelection(args) => {
            projected_motion_breath_commands::source_adapter_selection(args)
        }
        Command::ProjectedMotionBreathAdapterNormalizationEvidenceReview(args) => {
            projected_motion_breath_commands::adapter_normalization_evidence_review(args)
        }
        Command::ProjectedMotionBreathShellHandoffReview(args) => {
            projected_motion_breath_commands::shell_handoff_review(args)
        }
    }
}

fn default_pmb_shell_handoff_review_path(acceptance_index_path: &Path) -> Option<PathBuf> {
    let index_parent = acceptance_index_path.parent();
    let current_dir = std::env::current_dir().ok();
    let mut candidates = Vec::new();
    if let Some(parent) = index_parent {
        candidates.push(parent.join("pmb-shell-handoff.studio-review.json"));
        candidates.push(parent.join("../pmb-shell-handoff.studio-review.json"));
        candidates.push(parent.join("../../pmb-shell-handoff.studio-review.json"));
        candidates.push(parent.join("target/pmb-shell-handoff.studio-review.json"));
    }
    if let Some(current_dir) = current_dir.as_ref() {
        candidates.push(current_dir.join("target/pmb-shell-handoff.studio-review.json"));
        candidates.push(current_dir.join("../../target/pmb-shell-handoff.studio-review.json"));
        candidates.push(current_dir.join("../../../target/pmb-shell-handoff.studio-review.json"));
    }
    candidates.into_iter().find(|path| path.is_file())
}

fn canonical_existing_path(path: &Path) -> Result<PathBuf, Box<dyn std::error::Error>> {
    Ok(std::fs::canonicalize(path)?)
}
