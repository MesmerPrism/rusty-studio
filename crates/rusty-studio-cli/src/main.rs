use clap::{Parser, Subcommand, ValueEnum};
use rusty_studio_core::{
    add_binding_to_graph, add_module_to_graph, add_next_catalog_module_from_package_to_graph,
    add_next_catalog_module_to_graph, append_shell_export_package_baseline_index_manifests,
    append_shell_handoff_acceptance_baseline_index_manifests,
    append_shell_release_candidate_review_index_manifests, compare_shell_export_packages,
    compare_shell_export_packages_against_baseline_index_entry,
    compare_shell_export_packages_against_baseline_manifest,
    compare_shell_handoff_acceptance_against_baseline_index_entry,
    compare_shell_handoff_acceptance_against_baseline_manifest,
    compare_shell_handoff_acceptance_checklists, desktop_shell_handoff_for_bundle, export_plan,
    load_project, load_shell_artifact_manifest, load_shell_descriptor,
    load_shell_export_package_baseline_index, load_shell_export_package_baseline_manifest,
    load_shell_export_package_report, load_shell_handoff_acceptance_baseline_index,
    load_shell_handoff_acceptance_baseline_manifest, load_shell_handoff_acceptance_checklist,
    load_shell_handoff_intake_report, load_shell_handoff_manifest,
    load_shell_hostess_handoff_package_report, load_shell_hostess_owner_intake_report,
    load_shell_release_candidate_review_index, load_shell_release_candidate_review_manifest,
    load_shell_release_candidate_review_report, load_shell_template_index,
    promote_shell_export_package_baseline_index_default,
    promote_shell_handoff_acceptance_baseline_index_default,
    promote_shell_release_candidate_review_index_default, remove_binding_from_graph,
    remove_module_from_graph, resolve_project, retarget_graph_host_profile, save_json,
    save_project, save_shell_bundle, select_shell_export_package_baseline_index_entry,
    select_shell_handoff_acceptance_baseline_index_entry, selected_shell_bundle_for_graph,
    shell_artifacts_for_project, shell_descriptor_artifact_path, shell_descriptor_for_graph,
    shell_export_package_baseline_index_for_manifests,
    shell_export_package_baseline_manifest_for_report, shell_export_package_for_manifest,
    shell_export_package_for_project, shell_handoff_acceptance_baseline_index_for_manifests,
    shell_handoff_acceptance_baseline_manifest_for_checklist,
    shell_handoff_acceptance_checklist_for_intake, shell_handoff_acceptance_checklist_for_project,
    shell_handoff_for_bundle, shell_handoff_intake_for_manifest,
    shell_handoff_manifest_for_project, shell_handoff_readiness_for_project,
    shell_hostess_handoff_package_for_release_candidate_index,
    shell_hostess_owner_intake_for_handoff_package, shell_hostess_staging_preview_for_owner_intake,
    shell_release_candidate_review_for_manifest,
    shell_release_candidate_review_index_for_manifests,
    shell_release_candidate_review_manifest_for_report, shell_runbook_for_project,
    shell_templates_for_artifact_manifest, summarize_shell_export_package_baseline_index_selection,
    summarize_shell_handoff_acceptance_baseline_index_selection,
    summarize_shell_handoff_acceptance_checklist,
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
        Command::Validate(args) => {
            let project = load_project(&args.project)?;
            let report = validate_project_with_base(&project, args.project.parent());
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::Resolve(args) => {
            let project = load_project(&args.project)?;
            let resolved = resolve_project(&project);
            println!("{}", serde_json::to_string_pretty(&resolved)?);
            Ok(())
        }
        Command::ExportPlan(args) => {
            let project = load_project(&args.project)?;
            let plan = export_plan(&project);
            println!("{}", serde_json::to_string_pretty(&plan)?);
            Ok(())
        }
        Command::ViewModel(args) => {
            let project = load_project(&args.project)?;
            let model = view_model_for_graph_issue_node_and_edge(
                &project,
                args.project.parent(),
                args.graph.as_deref(),
                args.issue.as_deref(),
                args.node.as_deref(),
                args.edge.as_deref(),
            );
            println!("{}", serde_json::to_string_pretty(&model)?);
            Ok(())
        }
        Command::RetargetHost(args) => {
            if args.write && args.output.is_some() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "--write and --output are mutually exclusive",
                )
                .into());
            }

            let mut project = load_project(&args.project)?;
            let report = retarget_graph_host_profile(
                &mut project,
                &args.graph,
                &args.host_profile,
                args.project.parent(),
            );
            if report.status == StudioEditStatus::Applied {
                if args.write {
                    save_project(&args.project, &project)?;
                } else if let Some(output) = args.output.as_ref() {
                    save_project(output, &project)?;
                }
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::AddModule(args) => {
            if args.write && args.output.is_some() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "--write and --output are mutually exclusive",
                )
                .into());
            }

            let mut project = load_project(&args.project)?;
            let report = add_module_to_graph(
                &mut project,
                &args.graph,
                &args.package,
                &args.module,
                args.label.as_deref(),
                args.project.parent(),
            );
            if report.status == StudioEditStatus::Applied {
                if args.write {
                    save_project(&args.project, &project)?;
                } else if let Some(output) = args.output.as_ref() {
                    save_project(output, &project)?;
                }
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::AddPaletteModule(args) => {
            if args.write && args.output.is_some() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "--write and --output are mutually exclusive",
                )
                .into());
            }

            let mut project = load_project(&args.project)?;
            let report = if let Some(package) = args.package.as_deref() {
                add_next_catalog_module_from_package_to_graph(
                    &mut project,
                    &args.graph,
                    package,
                    args.project.parent(),
                )
            } else {
                add_next_catalog_module_to_graph(&mut project, &args.graph, args.project.parent())
            };
            if report.status == StudioEditStatus::Applied {
                if args.write {
                    save_project(&args.project, &project)?;
                } else if let Some(output) = args.output.as_ref() {
                    save_project(output, &project)?;
                }
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::RemoveModule(args) => {
            if args.write && args.output.is_some() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "--write and --output are mutually exclusive",
                )
                .into());
            }

            let mut project = load_project(&args.project)?;
            let report = remove_module_from_graph(
                &mut project,
                &args.graph,
                &args.module,
                args.project.parent(),
            );
            if report.status == StudioEditStatus::Applied {
                if args.write {
                    save_project(&args.project, &project)?;
                } else if let Some(output) = args.output.as_ref() {
                    save_project(output, &project)?;
                }
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::AddBinding(args) => {
            if args.write && args.output.is_some() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "--write and --output are mutually exclusive",
                )
                .into());
            }

            let mut project = load_project(&args.project)?;
            let report = add_binding_to_graph(
                &mut project,
                &args.graph,
                args.kind.into(),
                &args.source_node,
                &args.target_node,
                args.project.parent(),
            );
            if report.status == StudioEditStatus::Applied {
                if args.write {
                    save_project(&args.project, &project)?;
                } else if let Some(output) = args.output.as_ref() {
                    save_project(output, &project)?;
                }
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::RemoveBinding(args) => {
            if args.write && args.output.is_some() {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "--write and --output are mutually exclusive",
                )
                .into());
            }

            let mut project = load_project(&args.project)?;
            let report = remove_binding_from_graph(
                &mut project,
                &args.graph,
                args.kind.into(),
                &args.source_node,
                &args.target_node,
                args.project.parent(),
            );
            if report.status == StudioEditStatus::Applied {
                if args.write {
                    save_project(&args.project, &project)?;
                } else if let Some(output) = args.output.as_ref() {
                    save_project(output, &project)?;
                }
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellDescriptor(args) => {
            let project = load_project(&args.project)?;
            let report = shell_descriptor_for_graph(&project, args.project.parent(), &args.graph);
            if report.status == StudioShellDescriptorStatus::Exported {
                if let (Some(output), Some(descriptor)) =
                    (args.output.as_ref(), report.descriptor.as_ref())
                {
                    save_json(output, descriptor)?;
                }
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ValidateShellDescriptor(args) => {
            let descriptor = load_shell_descriptor(&args.descriptor)?;
            let report = validate_shell_descriptor(&descriptor);
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellArtifacts(args) => {
            let project = load_project(&args.project)?;
            let report = shell_artifacts_for_project(&project, args.project.parent());
            if report.status == StudioShellArtifactStatus::Exported {
                if let (Some(output_dir), Some(manifest)) =
                    (args.output_dir.as_ref(), report.manifest.as_ref())
                {
                    for descriptor in &report.descriptors {
                        let descriptor_path = relative_output_path(
                            output_dir,
                            &shell_descriptor_artifact_path(&descriptor.graph_id),
                        );
                        save_json(&descriptor_path, descriptor)?;
                    }
                    save_json(&output_dir.join("shell-artifacts.json"), manifest)?;
                }
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ValidateShellArtifacts(args) => {
            let manifest = load_shell_artifact_manifest(&args.manifest)?;
            let report = validate_shell_artifact_manifest(&manifest, args.manifest.parent());
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellTemplates(args) => {
            let manifest = load_shell_artifact_manifest(&args.manifest)?;
            let report = shell_templates_for_artifact_manifest(&manifest, args.manifest.parent());
            if report.status == StudioShellTemplateStatus::Exported {
                if let (Some(output_dir), Some(index)) =
                    (args.output_dir.as_ref(), report.index.as_ref())
                {
                    for (entry, template) in index.templates.iter().zip(report.templates.iter()) {
                        save_json(
                            &relative_output_path(output_dir, &entry.template_path),
                            template,
                        )?;
                        copy_manifest_descriptor(
                            args.manifest.parent(),
                            &template.source_descriptor_path,
                            output_dir,
                            &template.descriptor_path,
                        )?;
                    }
                    save_json(&output_dir.join("shell-templates.json"), index)?;
                }
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ValidateShellTemplates(args) => {
            let index = load_shell_template_index(&args.index)?;
            let report = validate_shell_template_index(&index, args.index.parent());
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellBundle(args) => {
            let project = load_project(&args.project)?;
            let report =
                selected_shell_bundle_for_graph(&project, args.project.parent(), &args.graph);
            if report.status == StudioShellBundleStatus::Exported {
                if let Some(output_dir) = args.output_dir.as_ref() {
                    save_shell_bundle(output_dir, &report)?;
                }
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ValidateShellBundle(args) => {
            let project = load_project(&args.project)?;
            let report = validate_selected_shell_bundle(
                &project,
                args.project.parent(),
                &args.graph,
                &args.bundle_dir,
            );
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoff(args) => {
            let project = load_project(&args.project)?;
            let report = shell_handoff_for_bundle(
                &project,
                args.project.parent(),
                &args.graph,
                &args.bundle_dir,
            );
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::DesktopShellHandoff(args) => {
            let project = load_project(&args.project)?;
            let report = desktop_shell_handoff_for_bundle(
                &project,
                args.project.parent(),
                &args.graph,
                &args.bundle_dir,
            );
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffReadiness(args) => {
            let project = load_project(&args.project)?;
            let report = shell_handoff_readiness_for_project(
                &project,
                args.project.parent(),
                &args.bundle_root,
            );
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffManifest(args) => {
            let project = load_project(&args.project)?;
            let manifest = shell_handoff_manifest_for_project(
                &project,
                args.project.parent(),
                &args.bundle_root,
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &manifest)?;
            }
            println!("{}", serde_json::to_string_pretty(&manifest)?);
            Ok(())
        }
        Command::ValidateShellHandoffManifest(args) => {
            let manifest = load_shell_handoff_manifest(&args.manifest)?;
            let report = validate_shell_handoff_manifest(&manifest);
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffIntake(args) => {
            let manifest = load_shell_handoff_manifest(&args.manifest)?;
            let report = shell_handoff_intake_for_manifest(&manifest);
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellRunbook(args) => {
            let project = load_project(&args.project)?;
            let report =
                shell_runbook_for_project(&project, args.project.parent(), &args.bundle_root);
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellExportPackage(args) => {
            let report = if let Some(manifest_path) = args.manifest.as_ref() {
                let manifest = load_shell_handoff_manifest(manifest_path)?;
                shell_export_package_for_manifest(&manifest)
            } else {
                let project_path = args.project.as_ref().ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "--project is required unless --manifest is supplied",
                    )
                })?;
                let bundle_root = args.bundle_root.as_ref().ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "--bundle-root is required unless --manifest is supplied",
                    )
                })?;
                let project = load_project(project_path)?;
                shell_export_package_for_project(&project, project_path.parent(), bundle_root)
            };
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellExportPackageBaseline(args) => {
            let package = load_shell_export_package_report(&args.package_report)?;
            let report = shell_export_package_baseline_manifest_for_report(
                &package,
                &args.package_report,
                args.baseline_id.as_deref(),
                args.label.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellExportPackageBaselineIndex(args) => {
            let baselines = args
                .baseline_manifests
                .iter()
                .map(|path| {
                    load_shell_export_package_baseline_manifest(path)
                        .map(|baseline| (baseline, Some(path.clone())))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let report = shell_export_package_baseline_index_for_manifests(
                baselines,
                args.default_baseline_id.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellExportPackageBaselineIndexAppend(args) => {
            let index = load_shell_export_package_baseline_index(&args.baseline_index)?;
            let baselines = args
                .baseline_manifests
                .iter()
                .map(|path| {
                    load_shell_export_package_baseline_manifest(path)
                        .map(|baseline| (baseline, Some(path.clone())))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let report = append_shell_export_package_baseline_index_manifests(
                &index,
                baselines,
                args.default_baseline_id.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellExportPackageBaselineIndexPromote(args) => {
            let index = load_shell_export_package_baseline_index(&args.baseline_index)?;
            let report =
                promote_shell_export_package_baseline_index_default(&index, &args.baseline_id)
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "--baseline-id was not found in --baseline-index",
                        )
                    })?;
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellExportPackageBaselineSelection(args) => {
            let index = load_shell_export_package_baseline_index(&args.baseline_index)?;
            let report = summarize_shell_export_package_baseline_index_selection(
                &index,
                Some(&args.baseline_index),
                args.baseline_id.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellExportPackageComparison(args) => {
            let candidate = load_shell_export_package_report(&args.candidate)?;
            let report = if let Some(baseline_index_path) = args.baseline_index.as_ref() {
                if args.baseline.is_some() || args.baseline_manifest.is_some() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "--baseline-index cannot be combined with --baseline or --baseline-manifest",
                    )
                    .into());
                }
                let baseline_index = load_shell_export_package_baseline_index(baseline_index_path)?;
                let baseline_index_entry = select_shell_export_package_baseline_index_entry(
                    &baseline_index,
                    args.baseline_id.as_deref(),
                )
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "--baseline-id was not found in --baseline-index",
                    )
                })?;
                let baseline_manifest_path = baseline_index_entry
                    .baseline_manifest_path
                    .as_ref()
                    .map(PathBuf::from)
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "selected baseline index entry does not include baseline_manifest_path",
                        )
                    })?;
                let baseline_manifest =
                    load_shell_export_package_baseline_manifest(&baseline_manifest_path)?;
                let baseline_path = PathBuf::from(&baseline_manifest.package_path);
                let baseline = load_shell_export_package_report(&baseline_path)?;
                compare_shell_export_packages_against_baseline_index_entry(
                    &baseline_index,
                    Some(baseline_index_path),
                    baseline_index_entry,
                    Some(&baseline_manifest_path),
                    &baseline_manifest,
                    &baseline,
                    &candidate,
                )
            } else {
                if args.baseline_id.is_some() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "--baseline-id requires --baseline-index",
                    )
                    .into());
                }
                let baseline_manifest = args
                    .baseline_manifest
                    .as_ref()
                    .map(|path| load_shell_export_package_baseline_manifest(path))
                    .transpose()?;
                let baseline_path = baseline_manifest
                    .as_ref()
                    .map(|identity| PathBuf::from(&identity.package_path))
                    .or(args.baseline.clone())
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "--baseline, --baseline-manifest, or --baseline-index is required",
                        )
                    })?;
                let baseline = load_shell_export_package_report(&baseline_path)?;
                if let Some(baseline_manifest) = baseline_manifest.as_ref() {
                    compare_shell_export_packages_against_baseline_manifest(
                        baseline_manifest,
                        &baseline,
                        &candidate,
                    )
                } else {
                    compare_shell_export_packages(&baseline, &candidate)
                }
            };
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffAcceptanceChecklist(args) => {
            let intake = load_shell_handoff_intake_report(&args.intake)?;
            let report = shell_handoff_acceptance_checklist_for_intake(&intake);
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffAcceptanceSnapshot(args) => {
            let project = load_project(&args.project)?;
            let report = shell_handoff_acceptance_checklist_for_project(
                &project,
                args.project.parent(),
                &args.bundle_root,
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffAcceptanceSummary(args) => {
            let checklist = load_shell_handoff_acceptance_checklist(&args.checklist)?;
            let report =
                summarize_shell_handoff_acceptance_checklist(&checklist, Some(&args.checklist));
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffAcceptanceBaseline(args) => {
            let checklist = load_shell_handoff_acceptance_checklist(&args.checklist)?;
            let report = shell_handoff_acceptance_baseline_manifest_for_checklist(
                &checklist,
                &args.checklist,
                args.baseline_id.as_deref(),
                args.label.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffAcceptanceBaselineIndex(args) => {
            let baselines = args
                .baseline_manifests
                .iter()
                .map(|path| {
                    load_shell_handoff_acceptance_baseline_manifest(path)
                        .map(|baseline| (baseline, Some(path.clone())))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let report = shell_handoff_acceptance_baseline_index_for_manifests(
                baselines,
                args.default_baseline_id.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffAcceptanceBaselineIndexAppend(args) => {
            let index = load_shell_handoff_acceptance_baseline_index(&args.baseline_index)?;
            let baselines = args
                .baseline_manifests
                .iter()
                .map(|path| {
                    load_shell_handoff_acceptance_baseline_manifest(path)
                        .map(|baseline| (baseline, Some(path.clone())))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let report = append_shell_handoff_acceptance_baseline_index_manifests(
                &index,
                baselines,
                args.default_baseline_id.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffAcceptanceBaselineIndexPromote(args) => {
            let index = load_shell_handoff_acceptance_baseline_index(&args.baseline_index)?;
            let report =
                promote_shell_handoff_acceptance_baseline_index_default(&index, &args.baseline_id)
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "--baseline-id was not found in --baseline-index",
                        )
                    })?;
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffAcceptanceBaselineSelection(args) => {
            let index = load_shell_handoff_acceptance_baseline_index(&args.baseline_index)?;
            let report = summarize_shell_handoff_acceptance_baseline_index_selection(
                &index,
                Some(&args.baseline_index),
                args.baseline_id.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHandoffAcceptanceComparison(args) => {
            let candidate = load_shell_handoff_acceptance_checklist(&args.candidate)?;
            let report = if let Some(baseline_index_path) = args.baseline_index.as_ref() {
                if args.baseline.is_some() || args.baseline_manifest.is_some() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "--baseline-index cannot be combined with --baseline or --baseline-manifest",
                    )
                    .into());
                }
                let baseline_index =
                    load_shell_handoff_acceptance_baseline_index(baseline_index_path)?;
                let baseline_index_entry = select_shell_handoff_acceptance_baseline_index_entry(
                    &baseline_index,
                    args.baseline_id.as_deref(),
                )
                .ok_or_else(|| {
                    std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "--baseline-id was not found in --baseline-index",
                    )
                })?;
                let baseline_manifest_path = baseline_index_entry
                    .baseline_manifest_path
                    .as_ref()
                    .map(PathBuf::from)
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "selected baseline index entry does not include baseline_manifest_path",
                        )
                    })?;
                let baseline_manifest =
                    load_shell_handoff_acceptance_baseline_manifest(&baseline_manifest_path)?;
                let baseline_path = PathBuf::from(&baseline_manifest.checklist_path);
                let baseline = load_shell_handoff_acceptance_checklist(&baseline_path)?;
                compare_shell_handoff_acceptance_against_baseline_index_entry(
                    &baseline_index,
                    Some(baseline_index_path),
                    baseline_index_entry,
                    Some(&baseline_manifest_path),
                    &baseline_manifest,
                    &baseline,
                    &candidate,
                )
            } else {
                if args.baseline_id.is_some() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "--baseline-id requires --baseline-index",
                    )
                    .into());
                }
                let baseline_manifest = args
                    .baseline_manifest
                    .as_ref()
                    .map(|path| load_shell_handoff_acceptance_baseline_manifest(path))
                    .transpose()?;
                let baseline_path = baseline_manifest
                    .as_ref()
                    .map(|identity| PathBuf::from(&identity.checklist_path))
                    .or(args.baseline.clone())
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "--baseline, --baseline-manifest, or --baseline-index is required",
                        )
                    })?;
                let baseline = load_shell_handoff_acceptance_checklist(&baseline_path)?;
                if let Some(baseline_manifest) = baseline_manifest.as_ref() {
                    compare_shell_handoff_acceptance_against_baseline_manifest(
                        baseline_manifest,
                        &baseline,
                        &candidate,
                    )
                } else {
                    compare_shell_handoff_acceptance_checklists(&baseline, &candidate)
                }
            };
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellReleaseCandidateReview(args) => {
            let manifest = load_shell_handoff_manifest(&args.manifest)?;
            let acceptance_baseline_index =
                load_shell_handoff_acceptance_baseline_index(&args.acceptance_baseline_index)?;
            let export_package_baseline_index =
                load_shell_export_package_baseline_index(&args.export_package_baseline_index)?;
            let report = shell_release_candidate_review_for_manifest(
                &manifest,
                Some(&args.manifest),
                &acceptance_baseline_index,
                Some(&args.acceptance_baseline_index),
                args.acceptance_baseline_id.as_deref(),
                &export_package_baseline_index,
                Some(&args.export_package_baseline_index),
                args.export_package_baseline_id.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellReleaseCandidateReviewManifest(args) => {
            let review = load_shell_release_candidate_review_report(&args.review)?;
            let report = shell_release_candidate_review_manifest_for_report(
                &review,
                &args.review,
                args.candidate_id.as_deref(),
                args.label.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellReleaseCandidateReviewIndex(args) => {
            let candidates = args
                .candidate_manifests
                .iter()
                .map(|path| {
                    load_shell_release_candidate_review_manifest(path)
                        .map(|candidate| (candidate, Some(path.clone())))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let report = shell_release_candidate_review_index_for_manifests(
                candidates,
                args.default_candidate_id.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellReleaseCandidateReviewIndexAppend(args) => {
            let index = load_shell_release_candidate_review_index(&args.review_index)?;
            let candidates = args
                .candidate_manifests
                .iter()
                .map(|path| {
                    load_shell_release_candidate_review_manifest(path)
                        .map(|candidate| (candidate, Some(path.clone())))
                })
                .collect::<Result<Vec<_>, _>>()?;
            let report = append_shell_release_candidate_review_index_manifests(
                &index,
                candidates,
                args.default_candidate_id.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellReleaseCandidateReviewIndexPromote(args) => {
            let index = load_shell_release_candidate_review_index(&args.review_index)?;
            let report =
                promote_shell_release_candidate_review_index_default(&index, &args.candidate_id)
                    .ok_or_else(|| {
                        std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            "--candidate-id was not found in --review-index",
                        )
                    })?;
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellReleaseCandidateReviewSelection(args) => {
            let index = load_shell_release_candidate_review_index(&args.review_index)?;
            let report = summarize_shell_release_candidate_review_index_selection(
                &index,
                Some(&args.review_index),
                args.candidate_id.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHostessHandoffPackage(args) => {
            let index = load_shell_release_candidate_review_index(&args.review_index)?;
            let report = shell_hostess_handoff_package_for_release_candidate_index(
                &index,
                Some(&args.review_index),
                args.candidate_id.as_deref(),
            );
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHostessOwnerIntake(args) => {
            let package = load_shell_hostess_handoff_package_report(&args.package)?;
            let report =
                shell_hostess_owner_intake_for_handoff_package(&package, Some(&args.package));
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
        Command::ShellHostessStagingPreview(args) => {
            let intake = load_shell_hostess_owner_intake_report(&args.intake)?;
            let report =
                shell_hostess_staging_preview_for_owner_intake(&intake, Some(&args.intake));
            if let Some(output) = args.output.as_ref() {
                save_json(output, &report)?;
            }
            println!("{}", serde_json::to_string_pretty(&report)?);
            Ok(())
        }
    }
}

fn relative_output_path(output_dir: &Path, relative_path: &str) -> PathBuf {
    relative_path
        .split('/')
        .fold(output_dir.to_path_buf(), |path, segment| path.join(segment))
}

fn copy_manifest_descriptor(
    manifest_dir: Option<&Path>,
    source_relative_path: &str,
    output_dir: &Path,
    output_relative_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = manifest_dir.ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "manifest path must have a parent directory",
        )
    })?;
    let source = relative_output_path(manifest_dir, source_relative_path);
    let output = relative_output_path(output_dir, output_relative_path);
    let descriptor = load_shell_descriptor(&source)?;
    save_json(&output, &descriptor)?;
    Ok(())
}
