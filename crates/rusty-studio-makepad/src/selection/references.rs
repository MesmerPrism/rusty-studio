use rusty_studio_model::StudioViewModel;

pub(crate) fn selected_package_reference_id(model: &StudioViewModel) -> Result<String, String> {
    let Some(node) = model.selected_node.as_ref() else {
        return Err("No node is selected".to_string());
    };
    if node.kind != "package" {
        return Err(format!(
            "Selected node {} is {}; select a package node to add a package module",
            node.node_id, node.kind
        ));
    }
    Ok(node.reference_id.clone())
}

pub(crate) fn selected_module_reference_id(model: &StudioViewModel) -> Result<String, String> {
    let Some(node) = model.selected_node.as_ref() else {
        return Err("No node is selected".to_string());
    };
    if node.kind != "module" {
        return Err(format!(
            "Selected node {} is {}; select a module node to remove a module",
            node.node_id, node.kind
        ));
    }
    Ok(node.reference_id.clone())
}
