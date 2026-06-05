use super::*;

pub(super) fn descriptor(args: ShellDescriptorArgs) -> Result<(), Box<dyn std::error::Error>> {
    let project = load_project(&args.project)?;
    let report = shell_descriptor_for_graph(&project, args.project.parent(), &args.graph);
    if report.status == StudioShellDescriptorStatus::Exported {
        if let (Some(output), Some(descriptor)) = (args.output.as_ref(), report.descriptor.as_ref())
        {
            save_json(output, descriptor)?;
        }
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn validate_descriptor(args: DescriptorArgs) -> Result<(), Box<dyn std::error::Error>> {
    let descriptor = load_shell_descriptor(&args.descriptor)?;
    let report = validate_shell_descriptor(&descriptor);
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn artifacts(args: ShellArtifactsArgs) -> Result<(), Box<dyn std::error::Error>> {
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

pub(super) fn validate_artifacts(args: ManifestArgs) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_shell_artifact_manifest(&args.manifest)?;
    let report = validate_shell_artifact_manifest(&manifest, args.manifest.parent());
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn templates(args: ShellTemplatesArgs) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_shell_artifact_manifest(&args.manifest)?;
    let report = shell_templates_for_artifact_manifest(&manifest, args.manifest.parent());
    if report.status == StudioShellTemplateStatus::Exported {
        if let (Some(output_dir), Some(index)) = (args.output_dir.as_ref(), report.index.as_ref()) {
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

pub(super) fn validate_templates(
    args: TemplateIndexArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_template_index(&args.index)?;
    let report = validate_shell_template_index(&index, args.index.parent());
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn bundle(args: ShellBundleArgs) -> Result<(), Box<dyn std::error::Error>> {
    let project = load_project(&args.project)?;
    let report = selected_shell_bundle_for_graph(&project, args.project.parent(), &args.graph);
    if report.status == StudioShellBundleStatus::Exported {
        if let Some(output_dir) = args.output_dir.as_ref() {
            save_shell_bundle(output_dir, &report)?;
        }
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn validate_bundle(
    args: ShellBundleValidationArgs,
) -> Result<(), Box<dyn std::error::Error>> {
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

pub(super) fn handoff(args: ShellBundleValidationArgs) -> Result<(), Box<dyn std::error::Error>> {
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

pub(super) fn desktop_handoff(
    args: ShellBundleValidationArgs,
) -> Result<(), Box<dyn std::error::Error>> {
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

pub(super) fn handoff_readiness(
    args: ShellHandoffReadinessArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let project = load_project(&args.project)?;
    let report =
        shell_handoff_readiness_for_project(&project, args.project.parent(), &args.bundle_root);
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn handoff_manifest(
    args: ShellHandoffManifestArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let project = load_project(&args.project)?;
    let manifest =
        shell_handoff_manifest_for_project(&project, args.project.parent(), &args.bundle_root);
    if let Some(output) = args.output.as_ref() {
        save_json(output, &manifest)?;
    }
    println!("{}", serde_json::to_string_pretty(&manifest)?);
    Ok(())
}

pub(super) fn validate_handoff_manifest(
    args: HandoffManifestArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_shell_handoff_manifest(&args.manifest)?;
    let report = validate_shell_handoff_manifest(&manifest);
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn handoff_intake(
    args: ShellHandoffIntakeArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_shell_handoff_manifest(&args.manifest)?;
    let report = shell_handoff_intake_for_manifest(&manifest);
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn runbook(args: ShellRunbookArgs) -> Result<(), Box<dyn std::error::Error>> {
    let project = load_project(&args.project)?;
    let report = shell_runbook_for_project(&project, args.project.parent(), &args.bundle_root);
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
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
