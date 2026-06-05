use clap::{Parser, Subcommand, ValueEnum};
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
    StudioBindingKind, StudioEditStatus, StudioShellArtifactStatus, StudioShellBundleStatus,
    StudioShellDescriptorStatus, StudioShellTemplateStatus,
};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

mod graph_edit_commands;
mod hostess_commands;
mod project_commands;
mod projected_motion_breath_commands;
mod release_candidate_commands;
mod shell_generation_commands;
mod shell_handoff_acceptance_commands;
mod shell_package_commands;

#[derive(Debug, Parser)]
#[command(name = "rusty-studio")]
#[command(about = "Schema-first Rusty Studio project CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Validate(ProjectArgs),
    Resolve(ProjectArgs),
    ExportPlan(ProjectArgs),
    ViewModel(ViewModelArgs),
    RetargetHost(RetargetHostArgs),
    AddModule(AddModuleArgs),
    AddPaletteModule(AddPaletteModuleArgs),
    RemoveModule(RemoveModuleArgs),
    AddBinding(BindingArgs),
    RemoveBinding(BindingArgs),
    ShellDescriptor(ShellDescriptorArgs),
    ValidateShellDescriptor(DescriptorArgs),
    ShellArtifacts(ShellArtifactsArgs),
    ValidateShellArtifacts(ManifestArgs),
    ShellTemplates(ShellTemplatesArgs),
    ValidateShellTemplates(TemplateIndexArgs),
    ShellBundle(ShellBundleArgs),
    ValidateShellBundle(ShellBundleValidationArgs),
    ShellHandoff(ShellBundleValidationArgs),
    DesktopShellHandoff(ShellBundleValidationArgs),
    ShellHandoffReadiness(ShellHandoffReadinessArgs),
    ShellHandoffManifest(ShellHandoffManifestArgs),
    ValidateShellHandoffManifest(HandoffManifestArgs),
    ShellHandoffIntake(ShellHandoffIntakeArgs),
    ShellRunbook(ShellRunbookArgs),
    ShellExportPackage(ShellExportPackageArgs),
    ShellExportPackageBaseline(ShellExportPackageBaselineArgs),
    ShellExportPackageBaselineIndex(ShellExportPackageBaselineIndexArgs),
    ShellExportPackageBaselineIndexAppend(ShellExportPackageBaselineIndexAppendArgs),
    ShellExportPackageBaselineIndexPromote(ShellExportPackageBaselineIndexPromoteArgs),
    ShellExportPackageBaselineSelection(ShellExportPackageBaselineSelectionArgs),
    ShellExportPackageComparison(ShellExportPackageComparisonArgs),
    ShellHandoffAcceptanceChecklist(ShellHandoffAcceptanceChecklistArgs),
    ShellHandoffAcceptanceSnapshot(ShellHandoffAcceptanceSnapshotArgs),
    ShellHandoffAcceptanceSummary(ShellHandoffAcceptanceSummaryArgs),
    ShellHandoffAcceptanceBaseline(ShellHandoffAcceptanceBaselineArgs),
    ShellHandoffAcceptanceBaselineIndex(ShellHandoffAcceptanceBaselineIndexArgs),
    ShellHandoffAcceptanceBaselineIndexAppend(ShellHandoffAcceptanceBaselineIndexAppendArgs),
    ShellHandoffAcceptanceBaselineIndexPromote(ShellHandoffAcceptanceBaselineIndexPromoteArgs),
    ShellHandoffAcceptanceBaselineSelection(ShellHandoffAcceptanceBaselineSelectionArgs),
    ShellHandoffAcceptanceComparison(ShellHandoffAcceptanceComparisonArgs),
    ShellReleaseCandidateReview(ShellReleaseCandidateReviewArgs),
    ShellReleaseCandidateReviewManifest(ShellReleaseCandidateReviewManifestArgs),
    ShellReleaseCandidateReviewIndex(ShellReleaseCandidateReviewIndexArgs),
    ShellReleaseCandidateReviewIndexAppend(ShellReleaseCandidateReviewIndexAppendArgs),
    ShellReleaseCandidateReviewIndexPromote(ShellReleaseCandidateReviewIndexPromoteArgs),
    ShellReleaseCandidateReviewSelection(ShellReleaseCandidateReviewSelectionArgs),
    ShellHostessHandoffPackage(ShellHostessHandoffPackageArgs),
    ShellHostessOwnerIntake(ShellHostessOwnerIntakeArgs),
    ShellHostessStagingPreview(ShellHostessStagingPreviewArgs),
    ShellHostessStagingFilePlan(ShellHostessStagingFilePlanArgs),
    ShellHostessStagingHandoff(ShellHostessStagingHandoffArgs),
    ShellHostessStagingAcceptanceChecklist(ShellHostessStagingAcceptanceChecklistArgs),
    ShellHostessStagingAcceptanceManifest(ShellHostessStagingAcceptanceManifestArgs),
    ShellHostessStagingAcceptanceIndex(ShellHostessStagingAcceptanceIndexArgs),
    ShellHostessStagingAcceptanceIndexAppend(ShellHostessStagingAcceptanceIndexAppendArgs),
    ShellHostessStagingAcceptanceIndexPromote(ShellHostessStagingAcceptanceIndexPromoteArgs),
    ShellHostessStagingAcceptanceSelection(ShellHostessStagingAcceptanceSelectionArgs),
    ShellHostessStagingAcceptanceComparison(ShellHostessStagingAcceptanceComparisonArgs),
    ShellHostessStagingExecutionRequest(ShellHostessStagingExecutionRequestArgs),
    PackageEvidenceIntake(PackageEvidenceIntakeArgs),
    ProjectedMotionBreathAuthoringReview(ProjectedMotionBreathAuthoringReviewArgs),
    ProjectedMotionBreathSourceAdapterSelection(ProjectedMotionBreathSourceAdapterSelectionArgs),
    ProjectedMotionBreathAdapterNormalizationEvidenceReview(
        ProjectedMotionBreathAdapterNormalizationEvidenceReviewArgs,
    ),
    ProjectedMotionBreathShellHandoffReview(ProjectedMotionBreathShellHandoffReviewArgs),
}

#[derive(Debug, Parser)]
struct ProjectArgs {
    #[arg(long)]
    project: PathBuf,
}

#[derive(Debug, Parser)]
struct ViewModelArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    graph: Option<String>,
    #[arg(long)]
    issue: Option<String>,
    #[arg(long)]
    node: Option<String>,
    #[arg(long)]
    edge: Option<String>,
}

#[derive(Debug, Parser)]
struct RetargetHostArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    graph: String,
    #[arg(long)]
    host_profile: String,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long)]
    write: bool,
}

#[derive(Debug, Parser)]
struct AddModuleArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    graph: String,
    #[arg(long)]
    package: String,
    #[arg(long)]
    module: String,
    #[arg(long)]
    label: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long)]
    write: bool,
}

#[derive(Debug, Parser)]
struct AddPaletteModuleArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    graph: String,
    #[arg(long)]
    package: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long)]
    write: bool,
}

#[derive(Debug, Parser)]
struct RemoveModuleArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    graph: String,
    #[arg(long)]
    module: String,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long)]
    write: bool,
}

#[derive(Debug, Parser)]
struct BindingArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    graph: String,
    #[arg(long)]
    kind: BindingKindArg,
    #[arg(long)]
    source_node: String,
    #[arg(long)]
    target_node: String,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long)]
    write: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum BindingKindArg {
    Stream,
    Command,
}

impl From<BindingKindArg> for StudioBindingKind {
    fn from(value: BindingKindArg) -> Self {
        match value {
            BindingKindArg::Stream => StudioBindingKind::Stream,
            BindingKindArg::Command => StudioBindingKind::Command,
        }
    }
}

#[derive(Debug, Parser)]
struct ShellDescriptorArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    graph: String,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct DescriptorArgs {
    #[arg(long)]
    descriptor: PathBuf,
}

#[derive(Debug, Parser)]
struct ShellArtifactsArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    output_dir: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ManifestArgs {
    #[arg(long)]
    manifest: PathBuf,
}

#[derive(Debug, Parser)]
struct ShellTemplatesArgs {
    #[arg(long)]
    manifest: PathBuf,
    #[arg(long)]
    output_dir: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct TemplateIndexArgs {
    #[arg(long)]
    index: PathBuf,
}

#[derive(Debug, Parser)]
struct ShellBundleArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    graph: String,
    #[arg(long)]
    output_dir: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellBundleValidationArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    graph: String,
    #[arg(long)]
    bundle_dir: PathBuf,
}

#[derive(Debug, Parser)]
struct ShellHandoffReadinessArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    bundle_root: PathBuf,
}

#[derive(Debug, Parser)]
struct ShellHandoffManifestArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    bundle_root: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct HandoffManifestArgs {
    #[arg(long)]
    manifest: PathBuf,
}

#[derive(Debug, Parser)]
struct ShellHandoffIntakeArgs {
    #[arg(long)]
    manifest: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellRunbookArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    bundle_root: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellExportPackageArgs {
    #[arg(long)]
    project: Option<PathBuf>,
    #[arg(long)]
    bundle_root: Option<PathBuf>,
    #[arg(long)]
    manifest: Option<PathBuf>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellExportPackageBaselineArgs {
    #[arg(long)]
    package_report: PathBuf,
    #[arg(long)]
    baseline_id: Option<String>,
    #[arg(long)]
    label: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellExportPackageBaselineIndexArgs {
    #[arg(long = "baseline-manifest", required = true)]
    baseline_manifests: Vec<PathBuf>,
    #[arg(long)]
    default_baseline_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellExportPackageBaselineIndexAppendArgs {
    #[arg(long)]
    baseline_index: PathBuf,
    #[arg(long = "baseline-manifest", required = true)]
    baseline_manifests: Vec<PathBuf>,
    #[arg(long)]
    default_baseline_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellExportPackageBaselineIndexPromoteArgs {
    #[arg(long)]
    baseline_index: PathBuf,
    #[arg(long)]
    baseline_id: String,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellExportPackageBaselineSelectionArgs {
    #[arg(long)]
    baseline_index: PathBuf,
    #[arg(long)]
    baseline_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellExportPackageComparisonArgs {
    #[arg(long)]
    baseline: Option<PathBuf>,
    #[arg(long)]
    baseline_manifest: Option<PathBuf>,
    #[arg(long)]
    baseline_index: Option<PathBuf>,
    #[arg(long)]
    baseline_id: Option<String>,
    #[arg(long)]
    candidate: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHandoffAcceptanceChecklistArgs {
    #[arg(long)]
    intake: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHandoffAcceptanceSnapshotArgs {
    #[arg(long)]
    project: PathBuf,
    #[arg(long)]
    bundle_root: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHandoffAcceptanceSummaryArgs {
    #[arg(long)]
    checklist: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHandoffAcceptanceBaselineArgs {
    #[arg(long)]
    checklist: PathBuf,
    #[arg(long)]
    baseline_id: Option<String>,
    #[arg(long)]
    label: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHandoffAcceptanceBaselineIndexArgs {
    #[arg(long = "baseline-manifest", required = true)]
    baseline_manifests: Vec<PathBuf>,
    #[arg(long)]
    default_baseline_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHandoffAcceptanceBaselineIndexAppendArgs {
    #[arg(long)]
    baseline_index: PathBuf,
    #[arg(long = "baseline-manifest", required = true)]
    baseline_manifests: Vec<PathBuf>,
    #[arg(long)]
    default_baseline_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHandoffAcceptanceBaselineIndexPromoteArgs {
    #[arg(long)]
    baseline_index: PathBuf,
    #[arg(long)]
    baseline_id: String,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHandoffAcceptanceBaselineSelectionArgs {
    #[arg(long)]
    baseline_index: PathBuf,
    #[arg(long)]
    baseline_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHandoffAcceptanceComparisonArgs {
    #[arg(long)]
    baseline: Option<PathBuf>,
    #[arg(long)]
    baseline_manifest: Option<PathBuf>,
    #[arg(long)]
    baseline_index: Option<PathBuf>,
    #[arg(long)]
    baseline_id: Option<String>,
    #[arg(long)]
    candidate: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellReleaseCandidateReviewArgs {
    #[arg(long)]
    manifest: PathBuf,
    #[arg(long)]
    acceptance_baseline_index: PathBuf,
    #[arg(long)]
    acceptance_baseline_id: Option<String>,
    #[arg(long)]
    export_package_baseline_index: PathBuf,
    #[arg(long)]
    export_package_baseline_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellReleaseCandidateReviewManifestArgs {
    #[arg(long)]
    review: PathBuf,
    #[arg(long)]
    candidate_id: Option<String>,
    #[arg(long)]
    label: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellReleaseCandidateReviewIndexArgs {
    #[arg(long = "candidate-manifest", required = true)]
    candidate_manifests: Vec<PathBuf>,
    #[arg(long)]
    default_candidate_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellReleaseCandidateReviewIndexAppendArgs {
    #[arg(long)]
    review_index: PathBuf,
    #[arg(long = "candidate-manifest", required = true)]
    candidate_manifests: Vec<PathBuf>,
    #[arg(long)]
    default_candidate_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellReleaseCandidateReviewIndexPromoteArgs {
    #[arg(long)]
    review_index: PathBuf,
    #[arg(long)]
    candidate_id: String,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellReleaseCandidateReviewSelectionArgs {
    #[arg(long)]
    review_index: PathBuf,
    #[arg(long)]
    candidate_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessHandoffPackageArgs {
    #[arg(long)]
    review_index: PathBuf,
    #[arg(long)]
    candidate_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessOwnerIntakeArgs {
    #[arg(long)]
    package: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessStagingPreviewArgs {
    #[arg(long)]
    intake: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessStagingFilePlanArgs {
    #[arg(long)]
    preview: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessStagingHandoffArgs {
    #[arg(long)]
    file_plan: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessStagingAcceptanceChecklistArgs {
    #[arg(long)]
    handoff: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessStagingAcceptanceManifestArgs {
    #[arg(long)]
    checklist: PathBuf,
    #[arg(long)]
    acceptance_id: Option<String>,
    #[arg(long)]
    label: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessStagingAcceptanceIndexArgs {
    #[arg(long = "acceptance-manifest", required = true)]
    acceptance_manifests: Vec<PathBuf>,
    #[arg(long)]
    default_acceptance_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessStagingAcceptanceIndexAppendArgs {
    #[arg(long)]
    acceptance_index: PathBuf,
    #[arg(long = "acceptance-manifest", required = true)]
    acceptance_manifests: Vec<PathBuf>,
    #[arg(long)]
    default_acceptance_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessStagingAcceptanceIndexPromoteArgs {
    #[arg(long)]
    acceptance_index: PathBuf,
    #[arg(long)]
    acceptance_id: String,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessStagingAcceptanceSelectionArgs {
    #[arg(long)]
    acceptance_index: PathBuf,
    #[arg(long)]
    acceptance_id: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessStagingAcceptanceComparisonArgs {
    #[arg(long)]
    baseline: Option<PathBuf>,
    #[arg(long)]
    baseline_manifest: Option<PathBuf>,
    #[arg(long)]
    acceptance_index: Option<PathBuf>,
    #[arg(long)]
    acceptance_id: Option<String>,
    #[arg(long)]
    candidate: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ShellHostessStagingExecutionRequestArgs {
    #[arg(long)]
    acceptance_index: PathBuf,
    #[arg(long)]
    acceptance_id: Option<String>,
    #[arg(long)]
    pmb_shell_handoff_review: Option<PathBuf>,
    #[arg(long)]
    require_pmb_shell_handoff_review: bool,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct PackageEvidenceIntakeArgs {
    #[arg(long)]
    report: PathBuf,
    #[arg(long, default_value = "package.projected_motion_breath")]
    package: String,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ProjectedMotionBreathAuthoringReviewArgs {
    #[arg(long)]
    intake: PathBuf,
    #[arg(long)]
    profile: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ProjectedMotionBreathSourceAdapterSelectionArgs {
    #[arg(long)]
    authoring_review: PathBuf,
    #[arg(long)]
    source_descriptors: PathBuf,
    #[arg(long)]
    adapter: String,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ProjectedMotionBreathAdapterNormalizationEvidenceReviewArgs {
    #[arg(long)]
    selection_review: PathBuf,
    #[arg(long)]
    package_report: PathBuf,
    #[arg(long)]
    source_binding: PathBuf,
    #[arg(long)]
    normalization_case: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
struct ProjectedMotionBreathShellHandoffReviewArgs {
    #[arg(long)]
    evidence: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

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
