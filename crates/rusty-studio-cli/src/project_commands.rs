use super::*;

pub(super) fn validate(args: ProjectArgs) -> Result<(), Box<dyn std::error::Error>> {
    let project = load_project(&args.project)?;
    let report = validate_project_with_base(&project, args.project.parent());
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn resolve(args: ProjectArgs) -> Result<(), Box<dyn std::error::Error>> {
    let project = load_project(&args.project)?;
    let resolved = resolve_project(&project);
    println!("{}", serde_json::to_string_pretty(&resolved)?);
    Ok(())
}

pub(super) fn export_plan(args: ProjectArgs) -> Result<(), Box<dyn std::error::Error>> {
    let project = load_project(&args.project)?;
    let plan = rusty_studio_core::export_plan(&project);
    println!("{}", serde_json::to_string_pretty(&plan)?);
    Ok(())
}

pub(super) fn view_model(args: ViewModelArgs) -> Result<(), Box<dyn std::error::Error>> {
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
