use super::*;

pub(crate) fn catalog_package_lines(model: &StudioViewModel) -> String {
    if model.catalog_packages.is_empty() {
        return "none".to_string();
    }
    model
        .catalog_packages
        .iter()
        .map(|package| {
            let state = if package.in_selected_graph {
                "selected"
            } else {
                "available"
            };
            let modules = if package.module_ids.is_empty() {
                "no module exports".to_string()
            } else {
                package.module_ids.join(", ")
            };
            format!(
                "{} [{}; {} module(s)]\n  {}\n  manifest: {}",
                package.package_id, state, package.module_count, modules, package.manifest_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub(crate) fn host_profile_lines(model: &StudioViewModel) -> String {
    if model.host_profiles.is_empty() {
        return "none".to_string();
    }
    model
        .host_profiles
        .iter()
        .map(|profile| {
            let state = if profile.targets_selected_graph {
                "target"
            } else {
                "available"
            };
            let host = profile.host_profile.as_deref().unwrap_or("unknown host");
            let install = profile
                .install_route
                .as_deref()
                .unwrap_or("install route missing");
            let launch = profile
                .launch_route
                .as_deref()
                .unwrap_or("launch route missing");
            format!(
                "{} [{}]\n  host: {}; routes: {} / {}",
                profile.profile_id, state, host, install, launch
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}
