use super::*;

#[derive(Default)]
pub(crate) struct ReferenceIndex {
    pub(crate) package_ids: BTreeSet<String>,
    pub(crate) module_ids: BTreeSet<String>,
    pub(crate) package_modules: BTreeMap<String, BTreeSet<String>>,
    pub(crate) package_manifest_paths: BTreeMap<String, String>,
    pub(crate) host_profiles: BTreeMap<String, HostProfileReference>,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct HostProfileReference {
    pub(crate) profile_id: String,
    pub(crate) host_profile: Option<String>,
    pub(crate) app_id: Option<String>,
    pub(crate) install_route: Option<String>,
    pub(crate) launch_route: Option<String>,
    pub(crate) command_bridge: Option<String>,
    pub(crate) evidence_pull_route: Option<String>,
    pub(crate) required_permissions: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CatalogModuleSelection {
    pub(crate) package_id: String,
    pub(crate) module_id: String,
    pub(crate) label: String,
}

pub(crate) fn reference_index_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
) -> Option<ReferenceIndex> {
    let mut checks = Vec::new();
    validate_project_references(project, base_dir, &mut checks)
}

pub(crate) fn selected_node_reference_ids(
    selected_graph: Option<&StudioGraph>,
    kind: StudioNodeKind,
) -> BTreeSet<String> {
    selected_graph
        .into_iter()
        .flat_map(|graph| graph.nodes.iter())
        .filter(|node| node.kind == kind)
        .map(|node| node.reference_id.clone())
        .collect()
}

pub(crate) fn validate_project_references(
    project: &StudioProject,
    base_dir: Option<&Path>,
    checks: &mut Vec<StudioValidationCheck>,
) -> Option<ReferenceIndex> {
    let Some(base_dir) = base_dir else {
        return None;
    };
    let mut references = ReferenceIndex::default();
    let catalog_path = resolve_project_path(base_dir, &project.package_catalog_path);
    push_check(
        checks,
        "studio.check.package_catalog_path",
        catalog_path.is_file(),
        "package catalog path resolves to a file",
        "package catalog path does not resolve to a file",
        "studio.issue.package_catalog_missing",
    );
    if let Some(catalog) = read_json_value(
        &catalog_path,
        checks,
        "studio.check.package_catalog.parse",
        "studio.issue.package_catalog_parse",
    ) {
        collect_catalog_references(&catalog_path, &catalog, &mut references, checks);
    }
    push_check(
        checks,
        "studio.check.host_run_profile_paths_present",
        !project.host_run_profile_paths.is_empty(),
        "project declares host-run profile paths",
        "project must declare at least one host-run profile path",
        "studio.issue.no_host_run_profiles",
    );
    for (index, path) in project.host_run_profile_paths.iter().enumerate() {
        let profile_path = resolve_project_path(base_dir, path);
        push_check(
            checks,
            &format!("studio.check.host_run_profile_path.{index}"),
            profile_path.is_file(),
            "host-run profile path resolves to a file",
            "host-run profile path does not resolve to a file",
            "studio.issue.host_run_profile_missing",
        );
        if let Some(profile) = read_json_value(
            &profile_path,
            checks,
            &format!("studio.check.host_run_profile_path.{index}.parse"),
            "studio.issue.host_run_profile_parse",
        ) {
            collect_host_profile_reference(index, &profile, &mut references, checks);
        }
    }
    Some(references)
}

fn collect_catalog_references(
    catalog_path: &Path,
    catalog: &Value,
    references: &mut ReferenceIndex,
    checks: &mut Vec<StudioValidationCheck>,
) {
    push_check(
        checks,
        "studio.check.package_catalog.schema",
        string_field(catalog, "$schema") == Some("rusty.manifold.package.catalog.v1"),
        "package catalog schema id is supported",
        "package catalog schema id is unsupported",
        "studio.issue.package_catalog_schema",
    );
    let packages = catalog.get("packages").and_then(Value::as_array);
    push_check(
        checks,
        "studio.check.package_catalog.packages_present",
        packages.is_some_and(|items| !items.is_empty()),
        "package catalog declares packages",
        "package catalog must declare at least one package",
        "studio.issue.package_catalog_empty",
    );
    let Some(packages) = packages else {
        return;
    };
    for (index, package) in packages.iter().enumerate() {
        let package_id = string_field(package, "package_id").unwrap_or_default();
        let manifest_path = string_field(package, "manifest_path").unwrap_or_default();
        let package_key = if package_id.is_empty() {
            format!("index.{index}")
        } else {
            package_id.to_string()
        };
        push_check(
            checks,
            &format!("studio.check.package_catalog.package.{package_key}.id"),
            is_dotted_id(package_id),
            "catalog package id uses dotted-id grammar",
            "catalog package id is missing or invalid",
            "studio.issue.package_catalog_package_id",
        );
        if is_dotted_id(package_id) {
            references.package_ids.insert(package_id.to_string());
            references
                .package_manifest_paths
                .insert(package_id.to_string(), manifest_path.to_string());
        }
        let manifest = resolve_catalog_manifest_path(catalog_path, manifest_path);
        push_check(
            checks,
            &format!("studio.check.package_catalog.package.{package_key}.manifest_path"),
            manifest.is_file(),
            "catalog package manifest path resolves to a file",
            "catalog package manifest path does not resolve to a file",
            "studio.issue.package_manifest_missing",
        );
        if let Some(manifest_json) = read_json_value(
            &manifest,
            checks,
            &format!("studio.check.package_manifest.{package_key}.parse"),
            "studio.issue.package_manifest_parse",
        ) {
            collect_package_manifest_references(package_id, &manifest_json, references, checks);
        }
    }
}

fn collect_package_manifest_references(
    expected_package_id: &str,
    manifest: &Value,
    references: &mut ReferenceIndex,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let package_key = if expected_package_id.is_empty() {
        "unknown"
    } else {
        expected_package_id
    };
    push_check(
        checks,
        &format!("studio.check.package_manifest.{package_key}.schema"),
        string_field(manifest, "$schema") == Some("rusty.manifold.package.manifest.v1"),
        "package manifest schema id is supported",
        "package manifest schema id is unsupported",
        "studio.issue.package_manifest_schema",
    );
    push_check(
        checks,
        &format!("studio.check.package_manifest.{package_key}.id_matches_catalog"),
        string_field(manifest, "package_id") == Some(expected_package_id),
        "package manifest id matches catalog package id",
        "package manifest id does not match catalog package id",
        "studio.issue.package_manifest_id_mismatch",
    );
    let modules = manifest
        .get("exports")
        .and_then(|exports| exports.get("modules"))
        .and_then(Value::as_array);
    push_check(
        checks,
        &format!("studio.check.package_manifest.{package_key}.module_exports"),
        modules.is_some_and(|items| !items.is_empty()),
        "package manifest exports modules",
        "package manifest must export at least one module",
        "studio.issue.package_manifest_no_modules",
    );
    let mut exported_modules = BTreeSet::new();
    if let Some(modules) = modules {
        for module in modules {
            if let Some(module_id) = module.as_str().filter(|value| is_dotted_id(value)) {
                references.module_ids.insert(module_id.to_string());
                exported_modules.insert(module_id.to_string());
            }
        }
    }
    if is_dotted_id(expected_package_id) {
        references
            .package_modules
            .insert(expected_package_id.to_string(), exported_modules);
    }
}

fn collect_host_profile_reference(
    index: usize,
    profile: &Value,
    references: &mut ReferenceIndex,
    checks: &mut Vec<StudioValidationCheck>,
) {
    push_check(
        checks,
        &format!("studio.check.host_run_profile_path.{index}.schema"),
        string_field(profile, "$schema")
            == Some("rusty.manifold.host_run.install_launch_profile.v1"),
        "host-run profile schema id is supported",
        "host-run profile schema id is unsupported",
        "studio.issue.host_run_profile_schema",
    );
    let profile_id = string_field(profile, "profile_id").unwrap_or_default();
    push_check(
        checks,
        &format!("studio.check.host_run_profile_path.{index}.profile_id"),
        is_dotted_id(profile_id),
        "host-run profile id uses dotted-id grammar",
        "host-run profile id is missing or invalid",
        "studio.issue.host_run_profile_id",
    );
    if is_dotted_id(profile_id) {
        references.host_profiles.insert(
            profile_id.to_string(),
            HostProfileReference {
                profile_id: profile_id.to_string(),
                host_profile: string_field(profile, "host_profile").map(str::to_string),
                app_id: string_field(profile, "app_id").map(str::to_string),
                install_route: string_field(profile, "install_route").map(str::to_string),
                launch_route: string_field(profile, "launch_route").map(str::to_string),
                command_bridge: string_field(profile, "command_bridge").map(str::to_string),
                evidence_pull_route: string_field(profile, "evidence_pull_route")
                    .map(str::to_string),
                required_permissions: string_array_field(profile, "required_permissions"),
            },
        );
    }
}

pub(crate) fn validate_graph_references(
    graph: &StudioGraph,
    reference_index: &ReferenceIndex,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let missing_package_targets = graph
        .nodes
        .iter()
        .filter(|node| node.kind == StudioNodeKind::Package)
        .filter(|node| !reference_index.package_ids.contains(&node.reference_id))
        .map(|node| (node.node_id.clone(), node.reference_id.clone()))
        .collect::<Vec<_>>();
    let missing_package_nodes = missing_package_targets
        .iter()
        .map(|(node_id, _)| node_id.clone())
        .collect::<Vec<_>>();
    let missing_packages = missing_package_targets
        .iter()
        .map(|(_, reference_id)| reference_id.clone())
        .collect::<Vec<_>>();
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{}.package_refs", graph.graph_id),
        missing_packages.is_empty(),
        "graph package nodes resolve through package catalog",
        &format!(
            "package references missing from catalog: {}",
            missing_packages.join(", ")
        ),
        "studio.issue.package_reference_missing",
        Some(&graph.graph_id),
        missing_package_nodes,
        Vec::new(),
        missing_packages.clone(),
    );

    let missing_module_targets = graph
        .nodes
        .iter()
        .filter(|node| node.kind == StudioNodeKind::Module)
        .filter(|node| !reference_index.module_ids.contains(&node.reference_id))
        .map(|node| (node.node_id.clone(), node.reference_id.clone()))
        .collect::<Vec<_>>();
    let missing_module_nodes = missing_module_targets
        .iter()
        .map(|(node_id, _)| node_id.clone())
        .collect::<Vec<_>>();
    let missing_modules = missing_module_targets
        .iter()
        .map(|(_, reference_id)| reference_id.clone())
        .collect::<Vec<_>>();
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{}.module_refs", graph.graph_id),
        missing_modules.is_empty(),
        "graph module nodes resolve through selected package manifests",
        &format!(
            "module references missing from package manifests: {}",
            missing_modules.join(", ")
        ),
        "studio.issue.module_reference_missing",
        Some(&graph.graph_id),
        missing_module_nodes,
        Vec::new(),
        missing_modules.clone(),
    );

    let missing_host_profile_targets = graph
        .nodes
        .iter()
        .filter(|node| node.kind == StudioNodeKind::HostProfile)
        .filter(|node| {
            !reference_index
                .host_profiles
                .contains_key(&node.reference_id)
        })
        .map(|node| (node.node_id.clone(), node.reference_id.clone()))
        .collect::<Vec<_>>();
    let missing_host_profile_nodes = missing_host_profile_targets
        .iter()
        .map(|(node_id, _)| node_id.clone())
        .collect::<Vec<_>>();
    let missing_host_profiles = missing_host_profile_targets
        .iter()
        .map(|(_, reference_id)| reference_id.clone())
        .collect::<Vec<_>>();
    push_contextual_check(
        checks,
        &format!("studio.check.graph.{}.host_profile_refs", graph.graph_id),
        missing_host_profiles.is_empty(),
        "graph host-profile nodes resolve through declared host-run profiles",
        &format!(
            "host profile references missing from declared profiles: {}",
            missing_host_profiles.join(", ")
        ),
        "studio.issue.host_profile_reference_missing",
        Some(&graph.graph_id),
        missing_host_profile_nodes,
        Vec::new(),
        missing_host_profiles.clone(),
    );
    push_contextual_check(
        checks,
        &format!(
            "studio.check.graph.{}.target_host_profile_ref",
            graph.graph_id
        ),
        reference_index
            .host_profiles
            .contains_key(&graph.target_host_profile),
        "target host profile resolves through declared host-run profiles",
        "target host profile is missing from declared host-run profiles",
        "studio.issue.target_host_profile_reference_missing",
        Some(&graph.graph_id),
        Vec::new(),
        Vec::new(),
        vec![graph.target_host_profile.clone()],
    );
}

fn read_json_value(
    path: &Path,
    checks: &mut Vec<StudioValidationCheck>,
    check_id: &str,
    issue_code: &str,
) -> Option<Value> {
    match std::fs::read_to_string(path) {
        Ok(text) => match serde_json::from_str::<Value>(&text) {
            Ok(value) => {
                push_check(
                    checks,
                    check_id,
                    true,
                    "JSON document parsed",
                    "JSON document could not parse",
                    issue_code,
                );
                Some(value)
            }
            Err(error) => {
                push_check(
                    checks,
                    check_id,
                    false,
                    "JSON document parsed",
                    &format!("JSON parse error: {error}"),
                    issue_code,
                );
                None
            }
        },
        Err(error) => {
            push_check(
                checks,
                check_id,
                false,
                "JSON document parsed",
                &format!("JSON read error: {error}"),
                issue_code,
            );
            None
        }
    }
}

fn string_field<'a>(value: &'a Value, field: &str) -> Option<&'a str> {
    value.get(field).and_then(Value::as_str)
}

fn string_array_field(value: &Value, field: &str) -> Vec<String> {
    value
        .get(field)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(str::to_string)
        .collect()
}

fn resolve_project_path(base_dir: &Path, path: &str) -> PathBuf {
    let path = Path::new(path);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_dir.join(path)
    }
}

fn resolve_catalog_manifest_path(catalog_path: &Path, manifest_path: &str) -> PathBuf {
    let path = Path::new(manifest_path);
    if path.is_absolute() {
        return path.to_path_buf();
    }
    let Some(catalog_dir) = catalog_path.parent() else {
        return path.to_path_buf();
    };
    let catalog_relative = catalog_dir.join(path);
    if catalog_relative.is_file() {
        return catalog_relative;
    }
    catalog_dir
        .parent()
        .map(|repo_root| repo_root.join(path))
        .unwrap_or(catalog_relative)
}
