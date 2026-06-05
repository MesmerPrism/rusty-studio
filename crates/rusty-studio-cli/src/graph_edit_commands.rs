use super::*;

pub(super) fn retarget_host(args: RetargetHostArgs) -> Result<(), Box<dyn std::error::Error>> {
    validate_output_mode(args.write, args.output.as_ref())?;

    let mut project = load_project(&args.project)?;
    let report = retarget_graph_host_profile(
        &mut project,
        &args.graph,
        &args.host_profile,
        args.project.parent(),
    );
    save_project_if_applied(
        report.status,
        args.write,
        &args.project,
        args.output.as_ref(),
        &project,
    )?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn add_module(args: AddModuleArgs) -> Result<(), Box<dyn std::error::Error>> {
    validate_output_mode(args.write, args.output.as_ref())?;

    let mut project = load_project(&args.project)?;
    let report = add_module_to_graph(
        &mut project,
        &args.graph,
        &args.package,
        &args.module,
        args.label.as_deref(),
        args.project.parent(),
    );
    save_project_if_applied(
        report.status,
        args.write,
        &args.project,
        args.output.as_ref(),
        &project,
    )?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn add_palette_module(
    args: AddPaletteModuleArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    validate_output_mode(args.write, args.output.as_ref())?;

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
    save_project_if_applied(
        report.status,
        args.write,
        &args.project,
        args.output.as_ref(),
        &project,
    )?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn remove_module(args: RemoveModuleArgs) -> Result<(), Box<dyn std::error::Error>> {
    validate_output_mode(args.write, args.output.as_ref())?;

    let mut project = load_project(&args.project)?;
    let report = remove_module_from_graph(
        &mut project,
        &args.graph,
        &args.module,
        args.project.parent(),
    );
    save_project_if_applied(
        report.status,
        args.write,
        &args.project,
        args.output.as_ref(),
        &project,
    )?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn add_binding(args: BindingArgs) -> Result<(), Box<dyn std::error::Error>> {
    validate_output_mode(args.write, args.output.as_ref())?;

    let mut project = load_project(&args.project)?;
    let report = add_binding_to_graph(
        &mut project,
        &args.graph,
        args.kind.into(),
        &args.source_node,
        &args.target_node,
        args.project.parent(),
    );
    save_project_if_applied(
        report.status,
        args.write,
        &args.project,
        args.output.as_ref(),
        &project,
    )?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn remove_binding(args: BindingArgs) -> Result<(), Box<dyn std::error::Error>> {
    validate_output_mode(args.write, args.output.as_ref())?;

    let mut project = load_project(&args.project)?;
    let report = remove_binding_from_graph(
        &mut project,
        &args.graph,
        args.kind.into(),
        &args.source_node,
        &args.target_node,
        args.project.parent(),
    );
    save_project_if_applied(
        report.status,
        args.write,
        &args.project,
        args.output.as_ref(),
        &project,
    )?;
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn validate_output_mode(
    write: bool,
    output: Option<&PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
    if write && output.is_some() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "--write and --output are mutually exclusive",
        )
        .into());
    }
    Ok(())
}

fn save_project_if_applied(
    status: StudioEditStatus,
    write: bool,
    project_path: &Path,
    output: Option<&PathBuf>,
    project: &rusty_studio_model::StudioProject,
) -> Result<(), Box<dyn std::error::Error>> {
    if status == StudioEditStatus::Applied {
        if write {
            save_project(project_path, project)?;
        } else if let Some(output) = output {
            save_project(output, project)?;
        }
    }
    Ok(())
}
