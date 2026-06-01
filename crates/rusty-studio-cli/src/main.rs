use clap::{Parser, Subcommand, ValueEnum};
use rusty_studio_core::{
    add_binding_to_graph, add_module_to_graph, add_next_catalog_module_to_graph, export_plan,
    load_project, load_shell_artifact_manifest, load_shell_descriptor, load_shell_template_index,
    remove_binding_from_graph, remove_module_from_graph, resolve_project,
    retarget_graph_host_profile, save_json, save_project, shell_artifacts_for_project,
    shell_descriptor_artifact_path, shell_descriptor_for_graph,
    shell_templates_for_artifact_manifest, validate_project_with_base,
    validate_shell_artifact_manifest, validate_shell_descriptor, validate_shell_template_index,
    view_model_for_graph_issue_and_node,
};
use rusty_studio_model::{
    StudioBindingKind, StudioEditStatus, StudioShellArtifactStatus, StudioShellDescriptorStatus,
    StudioShellTemplateStatus,
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
            let model = view_model_for_graph_issue_and_node(
                &project,
                args.project.parent(),
                args.graph.as_deref(),
                args.issue.as_deref(),
                args.node.as_deref(),
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
            let report =
                add_next_catalog_module_to_graph(&mut project, &args.graph, args.project.parent());
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
