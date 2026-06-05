use clap::{Parser, Subcommand, ValueEnum};
use rusty_studio_model::StudioBindingKind;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "rusty-studio")]
#[command(about = "Schema-first Rusty Studio project CLI")]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
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
pub(crate) struct ProjectArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
}

#[derive(Debug, Parser)]
pub(crate) struct ViewModelArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) graph: Option<String>,
    #[arg(long)]
    pub(crate) issue: Option<String>,
    #[arg(long)]
    pub(crate) node: Option<String>,
    #[arg(long)]
    pub(crate) edge: Option<String>,
}

#[derive(Debug, Parser)]
pub(crate) struct RetargetHostArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) graph: String,
    #[arg(long)]
    pub(crate) host_profile: String,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
    #[arg(long)]
    pub(crate) write: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct AddModuleArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) graph: String,
    #[arg(long)]
    pub(crate) package: String,
    #[arg(long)]
    pub(crate) module: String,
    #[arg(long)]
    pub(crate) label: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
    #[arg(long)]
    pub(crate) write: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct AddPaletteModuleArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) graph: String,
    #[arg(long)]
    pub(crate) package: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
    #[arg(long)]
    pub(crate) write: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct RemoveModuleArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) graph: String,
    #[arg(long)]
    pub(crate) module: String,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
    #[arg(long)]
    pub(crate) write: bool,
}

#[derive(Debug, Parser)]
pub(crate) struct BindingArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) graph: String,
    #[arg(long)]
    pub(crate) kind: BindingKindArg,
    #[arg(long)]
    pub(crate) source_node: String,
    #[arg(long)]
    pub(crate) target_node: String,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
    #[arg(long)]
    pub(crate) write: bool,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub(crate) enum BindingKindArg {
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
pub(crate) struct ShellDescriptorArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) graph: String,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct DescriptorArgs {
    #[arg(long)]
    pub(crate) descriptor: PathBuf,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellArtifactsArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) output_dir: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ManifestArgs {
    #[arg(long)]
    pub(crate) manifest: PathBuf,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellTemplatesArgs {
    #[arg(long)]
    pub(crate) manifest: PathBuf,
    #[arg(long)]
    pub(crate) output_dir: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct TemplateIndexArgs {
    #[arg(long)]
    pub(crate) index: PathBuf,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellBundleArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) graph: String,
    #[arg(long)]
    pub(crate) output_dir: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellBundleValidationArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) graph: String,
    #[arg(long)]
    pub(crate) bundle_dir: PathBuf,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffReadinessArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) bundle_root: PathBuf,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffManifestArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) bundle_root: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct HandoffManifestArgs {
    #[arg(long)]
    pub(crate) manifest: PathBuf,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffIntakeArgs {
    #[arg(long)]
    pub(crate) manifest: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellRunbookArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) bundle_root: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellExportPackageArgs {
    #[arg(long)]
    pub(crate) project: Option<PathBuf>,
    #[arg(long)]
    pub(crate) bundle_root: Option<PathBuf>,
    #[arg(long)]
    pub(crate) manifest: Option<PathBuf>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellExportPackageBaselineArgs {
    #[arg(long)]
    pub(crate) package_report: PathBuf,
    #[arg(long)]
    pub(crate) baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) label: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellExportPackageBaselineIndexArgs {
    #[arg(long = "baseline-manifest", required = true)]
    pub(crate) baseline_manifests: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) default_baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellExportPackageBaselineIndexAppendArgs {
    #[arg(long)]
    pub(crate) baseline_index: PathBuf,
    #[arg(long = "baseline-manifest", required = true)]
    pub(crate) baseline_manifests: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) default_baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellExportPackageBaselineIndexPromoteArgs {
    #[arg(long)]
    pub(crate) baseline_index: PathBuf,
    #[arg(long)]
    pub(crate) baseline_id: String,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellExportPackageBaselineSelectionArgs {
    #[arg(long)]
    pub(crate) baseline_index: PathBuf,
    #[arg(long)]
    pub(crate) baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellExportPackageComparisonArgs {
    #[arg(long)]
    pub(crate) baseline: Option<PathBuf>,
    #[arg(long)]
    pub(crate) baseline_manifest: Option<PathBuf>,
    #[arg(long)]
    pub(crate) baseline_index: Option<PathBuf>,
    #[arg(long)]
    pub(crate) baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) candidate: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffAcceptanceChecklistArgs {
    #[arg(long)]
    pub(crate) intake: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffAcceptanceSnapshotArgs {
    #[arg(long)]
    pub(crate) project: PathBuf,
    #[arg(long)]
    pub(crate) bundle_root: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffAcceptanceSummaryArgs {
    #[arg(long)]
    pub(crate) checklist: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffAcceptanceBaselineArgs {
    #[arg(long)]
    pub(crate) checklist: PathBuf,
    #[arg(long)]
    pub(crate) baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) label: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffAcceptanceBaselineIndexArgs {
    #[arg(long = "baseline-manifest", required = true)]
    pub(crate) baseline_manifests: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) default_baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffAcceptanceBaselineIndexAppendArgs {
    #[arg(long)]
    pub(crate) baseline_index: PathBuf,
    #[arg(long = "baseline-manifest", required = true)]
    pub(crate) baseline_manifests: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) default_baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffAcceptanceBaselineIndexPromoteArgs {
    #[arg(long)]
    pub(crate) baseline_index: PathBuf,
    #[arg(long)]
    pub(crate) baseline_id: String,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffAcceptanceBaselineSelectionArgs {
    #[arg(long)]
    pub(crate) baseline_index: PathBuf,
    #[arg(long)]
    pub(crate) baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHandoffAcceptanceComparisonArgs {
    #[arg(long)]
    pub(crate) baseline: Option<PathBuf>,
    #[arg(long)]
    pub(crate) baseline_manifest: Option<PathBuf>,
    #[arg(long)]
    pub(crate) baseline_index: Option<PathBuf>,
    #[arg(long)]
    pub(crate) baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) candidate: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellReleaseCandidateReviewArgs {
    #[arg(long)]
    pub(crate) manifest: PathBuf,
    #[arg(long)]
    pub(crate) acceptance_baseline_index: PathBuf,
    #[arg(long)]
    pub(crate) acceptance_baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) export_package_baseline_index: PathBuf,
    #[arg(long)]
    pub(crate) export_package_baseline_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellReleaseCandidateReviewManifestArgs {
    #[arg(long)]
    pub(crate) review: PathBuf,
    #[arg(long)]
    pub(crate) candidate_id: Option<String>,
    #[arg(long)]
    pub(crate) label: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellReleaseCandidateReviewIndexArgs {
    #[arg(long = "candidate-manifest", required = true)]
    pub(crate) candidate_manifests: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) default_candidate_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellReleaseCandidateReviewIndexAppendArgs {
    #[arg(long)]
    pub(crate) review_index: PathBuf,
    #[arg(long = "candidate-manifest", required = true)]
    pub(crate) candidate_manifests: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) default_candidate_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellReleaseCandidateReviewIndexPromoteArgs {
    #[arg(long)]
    pub(crate) review_index: PathBuf,
    #[arg(long)]
    pub(crate) candidate_id: String,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellReleaseCandidateReviewSelectionArgs {
    #[arg(long)]
    pub(crate) review_index: PathBuf,
    #[arg(long)]
    pub(crate) candidate_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessHandoffPackageArgs {
    #[arg(long)]
    pub(crate) review_index: PathBuf,
    #[arg(long)]
    pub(crate) candidate_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessOwnerIntakeArgs {
    #[arg(long)]
    pub(crate) package: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessStagingPreviewArgs {
    #[arg(long)]
    pub(crate) intake: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessStagingFilePlanArgs {
    #[arg(long)]
    pub(crate) preview: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessStagingHandoffArgs {
    #[arg(long)]
    pub(crate) file_plan: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessStagingAcceptanceChecklistArgs {
    #[arg(long)]
    pub(crate) handoff: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessStagingAcceptanceManifestArgs {
    #[arg(long)]
    pub(crate) checklist: PathBuf,
    #[arg(long)]
    pub(crate) acceptance_id: Option<String>,
    #[arg(long)]
    pub(crate) label: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessStagingAcceptanceIndexArgs {
    #[arg(long = "acceptance-manifest", required = true)]
    pub(crate) acceptance_manifests: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) default_acceptance_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessStagingAcceptanceIndexAppendArgs {
    #[arg(long)]
    pub(crate) acceptance_index: PathBuf,
    #[arg(long = "acceptance-manifest", required = true)]
    pub(crate) acceptance_manifests: Vec<PathBuf>,
    #[arg(long)]
    pub(crate) default_acceptance_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessStagingAcceptanceIndexPromoteArgs {
    #[arg(long)]
    pub(crate) acceptance_index: PathBuf,
    #[arg(long)]
    pub(crate) acceptance_id: String,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessStagingAcceptanceSelectionArgs {
    #[arg(long)]
    pub(crate) acceptance_index: PathBuf,
    #[arg(long)]
    pub(crate) acceptance_id: Option<String>,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessStagingAcceptanceComparisonArgs {
    #[arg(long)]
    pub(crate) baseline: Option<PathBuf>,
    #[arg(long)]
    pub(crate) baseline_manifest: Option<PathBuf>,
    #[arg(long)]
    pub(crate) acceptance_index: Option<PathBuf>,
    #[arg(long)]
    pub(crate) acceptance_id: Option<String>,
    #[arg(long)]
    pub(crate) candidate: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ShellHostessStagingExecutionRequestArgs {
    #[arg(long)]
    pub(crate) acceptance_index: PathBuf,
    #[arg(long)]
    pub(crate) acceptance_id: Option<String>,
    #[arg(long)]
    pub(crate) pmb_shell_handoff_review: Option<PathBuf>,
    #[arg(long)]
    pub(crate) require_pmb_shell_handoff_review: bool,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct PackageEvidenceIntakeArgs {
    #[arg(long)]
    pub(crate) report: PathBuf,
    #[arg(long, default_value = "package.projected_motion_breath")]
    pub(crate) package: String,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ProjectedMotionBreathAuthoringReviewArgs {
    #[arg(long)]
    pub(crate) intake: PathBuf,
    #[arg(long)]
    pub(crate) profile: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ProjectedMotionBreathSourceAdapterSelectionArgs {
    #[arg(long)]
    pub(crate) authoring_review: PathBuf,
    #[arg(long)]
    pub(crate) source_descriptors: PathBuf,
    #[arg(long)]
    pub(crate) adapter: String,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ProjectedMotionBreathAdapterNormalizationEvidenceReviewArgs {
    #[arg(long)]
    pub(crate) selection_review: PathBuf,
    #[arg(long)]
    pub(crate) package_report: PathBuf,
    #[arg(long)]
    pub(crate) source_binding: PathBuf,
    #[arg(long)]
    pub(crate) normalization_case: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}

#[derive(Debug, Parser)]
pub(crate) struct ProjectedMotionBreathShellHandoffReviewArgs {
    #[arg(long)]
    pub(crate) evidence: PathBuf,
    #[arg(long)]
    pub(crate) output: Option<PathBuf>,
}
