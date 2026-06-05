use super::*;

pub fn shell_templates_for_artifact_manifest(
    manifest: &StudioShellArtifactManifest,
    base_dir: Option<&Path>,
) -> StudioShellTemplateReport {
    let validation = validate_shell_artifact_manifest(manifest, base_dir);
    if validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_shell_artifact_manifest_issue_code(&validation)
            .unwrap_or_else(|| "studio.issue.shell_artifact_manifest_invalid".to_string());
        return shell_template_report(
            manifest,
            StudioShellTemplateStatus::Rejected,
            Some(issue_code),
            "Shell artifact manifest validation failed; shell templates were not exported"
                .to_string(),
            validation,
            None,
            Vec::new(),
        );
    }

    let templates: Vec<_> = manifest
        .artifacts
        .iter()
        .map(shell_template_for_artifact)
        .collect();
    let index = StudioShellTemplateIndex {
        schema_id: SHELL_TEMPLATE_INDEX_SCHEMA.to_string(),
        index_id: format!("studio.shell_templates.{}", manifest.project_id),
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        templates: manifest
            .artifacts
            .iter()
            .map(shell_template_index_entry)
            .collect(),
    };

    shell_template_report(
        manifest,
        StudioShellTemplateStatus::Exported,
        None,
        "Shell templates exported".to_string(),
        validation,
        Some(index),
        templates,
    )
}

pub fn validate_shell_template_index(
    index: &StudioShellTemplateIndex,
    base_dir: Option<&Path>,
) -> StudioShellTemplateIndexValidationReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_template_index.schema",
        index.schema_id == SHELL_TEMPLATE_INDEX_SCHEMA,
        "shell template index schema id is supported",
        "shell template index schema id is unsupported",
        "studio.issue.shell_template_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_template_index.index_id",
        is_dotted_id(&index.index_id),
        "index id uses dotted-id grammar",
        "index id is not a dotted id",
        "studio.issue.invalid_index_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_template_index.manifest_id",
        is_dotted_id(&index.manifest_id),
        "manifest id uses dotted-id grammar",
        "manifest id is not a dotted id",
        "studio.issue.invalid_manifest_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_template_index.project_id",
        is_dotted_id(&index.project_id),
        "project id uses dotted-id grammar",
        "project id is not a dotted id",
        "studio.issue.invalid_project_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_template_index.project_revision",
        index.project_revision > 0,
        "project revision is positive",
        "project revision must be positive",
        "studio.issue.invalid_revision",
    );
    push_check(
        &mut checks,
        "studio.check.shell_template_index.templates_present",
        !index.templates.is_empty(),
        "index declares shell templates",
        "index must declare at least one shell template",
        "studio.issue.no_shell_templates",
    );

    let duplicate_template_ids =
        duplicate_template_field(&index.templates, |entry| entry.template_id.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_template_index.unique_template_ids",
        duplicate_template_ids.is_empty(),
        "template ids are unique",
        &format!(
            "duplicate template ids: {}",
            duplicate_template_ids.join(", ")
        ),
        "studio.issue.duplicate_template_id",
    );
    let duplicate_artifact_ids =
        duplicate_template_field(&index.templates, |entry| entry.artifact_id.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_template_index.unique_artifact_ids",
        duplicate_artifact_ids.is_empty(),
        "artifact ids are unique",
        &format!(
            "duplicate artifact ids: {}",
            duplicate_artifact_ids.join(", ")
        ),
        "studio.issue.duplicate_artifact_id",
    );
    let duplicate_graph_ids =
        duplicate_template_field(&index.templates, |entry| entry.graph_id.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_template_index.unique_graph_ids",
        duplicate_graph_ids.is_empty(),
        "graph ids are unique",
        &format!("duplicate graph ids: {}", duplicate_graph_ids.join(", ")),
        "studio.issue.duplicate_template_graph_id",
    );
    let duplicate_template_paths =
        duplicate_template_field(&index.templates, |entry| entry.template_path.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_template_index.unique_template_paths",
        duplicate_template_paths.is_empty(),
        "template paths are unique",
        &format!(
            "duplicate template paths: {}",
            duplicate_template_paths.join(", ")
        ),
        "studio.issue.duplicate_template_path",
    );
    let duplicate_descriptor_paths =
        duplicate_template_field(&index.templates, |entry| entry.descriptor_path.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_template_index.unique_descriptor_paths",
        duplicate_descriptor_paths.is_empty(),
        "descriptor paths are unique",
        &format!(
            "duplicate descriptor paths: {}",
            duplicate_descriptor_paths.join(", ")
        ),
        "studio.issue.duplicate_descriptor_path",
    );

    for entry in &index.templates {
        validate_shell_template_index_entry(entry, base_dir, &mut checks);
    }

    StudioShellTemplateIndexValidationReport {
        schema_id: SHELL_TEMPLATE_INDEX_VALIDATION_REPORT_SCHEMA,
        index_id: index.index_id.clone(),
        status: if checks
            .iter()
            .any(|check| check.status == StudioValidationStatus::Fail)
        {
            StudioValidationStatus::Fail
        } else {
            StudioValidationStatus::Pass
        },
        checks,
    }
}

pub fn shell_template_manifest_path(artifact: &StudioShellArtifact) -> String {
    format!(
        "shells/{}/{}.shell-template.json",
        shell_target_kind_path(artifact.target_kind),
        artifact.graph_id
    )
}

pub fn shell_template_descriptor_path(graph_id: &str) -> String {
    format!("descriptors/{graph_id}.shell-descriptor.json")
}

pub(crate) fn shell_target_kind_path(target_kind: StudioShellTargetKind) -> &'static str {
    match target_kind {
        StudioShellTargetKind::Desktop => "desktop",
        StudioShellTargetKind::Phone => "phone",
        StudioShellTargetKind::Quest => "quest",
        StudioShellTargetKind::Unknown => "unknown",
    }
}

fn validate_shell_template_index_entry(
    entry: &StudioShellTemplateIndexEntry,
    base_dir: Option<&Path>,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = entry.template_id.clone();
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.template_id"),
        is_dotted_id(&entry.template_id),
        "template id uses dotted-id grammar",
        "template id is not a dotted id",
        "studio.issue.invalid_template_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.artifact_id"),
        is_dotted_id(&entry.artifact_id),
        "artifact id uses dotted-id grammar",
        "artifact id is not a dotted id",
        "studio.issue.invalid_artifact_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.graph_id"),
        is_dotted_id(&entry.graph_id),
        "graph id uses dotted-id grammar",
        "graph id is not a dotted id",
        "studio.issue.invalid_graph_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.shell_id"),
        is_dotted_id(&entry.shell_id),
        "shell id uses dotted-id grammar",
        "shell id is not a dotted id",
        "studio.issue.invalid_shell_id",
    );
    let template_path_is_safe = is_safe_relative_manifest_path(&entry.template_path);
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.template_path"),
        template_path_is_safe,
        "template path is a safe relative path",
        "template path must be a portable relative path without traversal",
        "studio.issue.invalid_template_path",
    );
    let descriptor_path_is_safe = is_safe_relative_manifest_path(&entry.descriptor_path);
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_path"),
        descriptor_path_is_safe,
        "descriptor path is a safe relative path",
        "descriptor path must be a portable relative path without traversal",
        "studio.issue.invalid_descriptor_path",
    );

    if let Some(base_dir) = base_dir.filter(|_| template_path_is_safe && descriptor_path_is_safe) {
        validate_shell_template_files(entry, base_dir, checks);
    }
}

fn validate_shell_template_files(
    entry: &StudioShellTemplateIndexEntry,
    base_dir: &Path,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = entry.template_id.clone();
    let template_path = resolve_manifest_relative_path(base_dir, &entry.template_path);
    let template_exists = template_path.is_file();
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.template_exists"),
        template_exists,
        "template path resolves to a file",
        "template path does not resolve to a file",
        "studio.issue.template_missing",
    );
    if !template_exists {
        return;
    }

    let template = match load_shell_template_manifest(&template_path) {
        Ok(template) => {
            push_check(
                checks,
                &format!("studio.check.shell_template_index.template.{prefix}.template_parse"),
                true,
                "template JSON parsed",
                "template JSON did not parse",
                "studio.issue.template_parse_failed",
            );
            template
        }
        Err(error) => {
            checks.push(StudioValidationCheck {
                check_id: format!(
                    "studio.check.shell_template_index.template.{prefix}.template_parse"
                ),
                status: StudioValidationStatus::Fail,
                evidence: error.to_string(),
                issue_code: Some("studio.issue.template_parse_failed".to_string()),
                graph_id: Some(entry.graph_id.clone()),
                node_ids: Vec::new(),
                edge_ids: Vec::new(),
                reference_ids: Vec::new(),
            });
            return;
        }
    };

    validate_shell_template_manifest_reference(entry, &template, checks);

    let descriptor_path = resolve_manifest_relative_path(base_dir, &entry.descriptor_path);
    let descriptor_exists = descriptor_path.is_file();
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_exists"),
        descriptor_exists,
        "descriptor path resolves to a file",
        "descriptor path does not resolve to a file",
        "studio.issue.descriptor_missing",
    );
    if !descriptor_exists {
        return;
    }
    let descriptor = match load_shell_descriptor(&descriptor_path) {
        Ok(descriptor) => {
            push_check(
                checks,
                &format!("studio.check.shell_template_index.template.{prefix}.descriptor_parse"),
                true,
                "descriptor JSON parsed",
                "descriptor JSON did not parse",
                "studio.issue.descriptor_parse_failed",
            );
            descriptor
        }
        Err(error) => {
            checks.push(StudioValidationCheck {
                check_id: format!(
                    "studio.check.shell_template_index.template.{prefix}.descriptor_parse"
                ),
                status: StudioValidationStatus::Fail,
                evidence: error.to_string(),
                issue_code: Some("studio.issue.descriptor_parse_failed".to_string()),
                graph_id: Some(entry.graph_id.clone()),
                node_ids: Vec::new(),
                edge_ids: Vec::new(),
                reference_ids: Vec::new(),
            });
            return;
        }
    };

    let descriptor_validation = validate_shell_descriptor(&descriptor);
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_validation"),
        descriptor_validation.status == StudioValidationStatus::Pass,
        "descriptor validation passed",
        "descriptor validation failed",
        "studio.issue.descriptor_validation_failed",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_graph"),
        descriptor.graph_id == entry.graph_id && descriptor.graph_id == template.graph_id,
        "descriptor graph id matches template index and manifest",
        "descriptor graph id does not match template index and manifest",
        "studio.issue.descriptor_graph_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_shell"),
        descriptor.shell_id == entry.shell_id && descriptor.shell_id == template.shell_id,
        "descriptor shell id matches template index and manifest",
        "descriptor shell id does not match template index and manifest",
        "studio.issue.descriptor_shell_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_target"),
        descriptor.target_host_profile == template.target_host_profile,
        "descriptor target host profile matches template manifest",
        "descriptor target host profile does not match template manifest",
        "studio.issue.descriptor_target_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_target_kind"),
        shell_target_kind(descriptor.host_profile.host_profile.as_deref()) == entry.target_kind
            && entry.target_kind == template.target_kind,
        "descriptor target kind matches template index and manifest",
        "descriptor target kind does not match template index and manifest",
        "studio.issue.descriptor_target_kind_mismatch",
    );
}

fn validate_shell_template_manifest_reference(
    entry: &StudioShellTemplateIndexEntry,
    template: &StudioShellTemplateManifest,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = entry.template_id.clone();
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.template_schema"),
        template.schema_id == SHELL_TEMPLATE_MANIFEST_SCHEMA,
        "template manifest schema id is supported",
        "template manifest schema id is unsupported",
        "studio.issue.shell_template_manifest_schema",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.template_id_matches"),
        template.template_id == entry.template_id,
        "template id matches index entry",
        "template id does not match index entry",
        "studio.issue.template_id_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.artifact_id_matches"),
        template.artifact_id == entry.artifact_id,
        "artifact id matches index entry",
        "artifact id does not match index entry",
        "studio.issue.artifact_id_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.graph_id_matches"),
        template.graph_id == entry.graph_id,
        "graph id matches index entry",
        "graph id does not match index entry",
        "studio.issue.template_graph_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.shell_id_matches"),
        template.shell_id == entry.shell_id,
        "shell id matches index entry",
        "shell id does not match index entry",
        "studio.issue.template_shell_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.target_kind_matches"),
        template.target_kind == entry.target_kind,
        "target kind matches index entry",
        "target kind does not match index entry",
        "studio.issue.template_target_kind_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.descriptor_path_matches"),
        template.descriptor_path == entry.descriptor_path,
        "descriptor path matches index entry",
        "descriptor path does not match index entry",
        "studio.issue.template_descriptor_path_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.target_host_profile"),
        is_dotted_id(&template.target_host_profile),
        "target host profile uses dotted-id grammar",
        "target host profile is not a dotted id",
        "studio.issue.invalid_target_host_profile",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.host_profile_class"),
        optional_dotted_id(template.host_profile_class.as_deref()),
        "host profile class is absent or uses dotted-id grammar",
        "host profile class is not a dotted id",
        "studio.issue.invalid_host_profile_class",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.source_descriptor_path"),
        is_safe_relative_manifest_path(&template.source_descriptor_path),
        "source descriptor path is a safe relative path",
        "source descriptor path must be a portable relative path without traversal",
        "studio.issue.invalid_source_descriptor_path",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.runtime_command_authority"),
        template.runtime_authority.command_session_authority == "rusty.manifold",
        "Manifold owns command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.runtime_host_authority"),
        template.runtime_authority.install_launch_evidence_authority == "rusty.hostess",
        "Hostess owns install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.studio_role"),
        template.runtime_authority.studio_role == "authoring.export_planning",
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.host_routes"),
        optional_dotted_id(template.host_routes.app_id.as_deref())
            && optional_dotted_id(template.host_routes.install_route.as_deref())
            && optional_dotted_id(template.host_routes.launch_route.as_deref())
            && optional_dotted_id(template.host_routes.command_bridge.as_deref())
            && optional_dotted_id(template.host_routes.evidence_pull_route.as_deref()),
        "host routes are absent or use dotted-id grammar",
        "one or more host routes are not dotted ids",
        "studio.issue.invalid_host_route",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.package_ids"),
        all_dotted_ids(&template.package_ids),
        "package ids use dotted-id grammar",
        "one or more package ids are not dotted ids",
        "studio.issue.invalid_package_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_template_index.template.{prefix}.module_ids"),
        all_dotted_ids(&template.module_ids),
        "module ids use dotted-id grammar",
        "one or more module ids are not dotted ids",
        "studio.issue.invalid_module_id",
    );
}

pub(crate) fn shell_template_for_artifact(
    artifact: &StudioShellArtifact,
) -> StudioShellTemplateManifest {
    StudioShellTemplateManifest {
        schema_id: SHELL_TEMPLATE_MANIFEST_SCHEMA.to_string(),
        template_id: shell_template_id(&artifact.graph_id),
        artifact_id: artifact.artifact_id.clone(),
        graph_id: artifact.graph_id.clone(),
        shell_id: artifact.shell_id.clone(),
        target_kind: artifact.target_kind,
        target_host_profile: artifact.target_host_profile.clone(),
        host_profile_class: artifact.host_profile_class.clone(),
        source_descriptor_path: artifact.descriptor_path.clone(),
        descriptor_path: shell_template_descriptor_path(&artifact.graph_id),
        runtime_authority: shell_runtime_authority(),
        host_routes: StudioShellHostRoutes {
            app_id: artifact.app_id.clone(),
            install_route: artifact.install_route.clone(),
            launch_route: artifact.launch_route.clone(),
            command_bridge: artifact.command_bridge.clone(),
            evidence_pull_route: artifact.evidence_pull_route.clone(),
        },
        package_ids: artifact.package_ids.clone(),
        module_ids: artifact.module_ids.clone(),
    }
}

pub(crate) fn shell_template_index_entry(
    artifact: &StudioShellArtifact,
) -> StudioShellTemplateIndexEntry {
    StudioShellTemplateIndexEntry {
        template_id: shell_template_id(&artifact.graph_id),
        artifact_id: artifact.artifact_id.clone(),
        graph_id: artifact.graph_id.clone(),
        shell_id: artifact.shell_id.clone(),
        target_kind: artifact.target_kind,
        template_path: shell_template_manifest_path(artifact),
        descriptor_path: shell_template_descriptor_path(&artifact.graph_id),
    }
}

fn shell_template_id(graph_id: &str) -> String {
    format!("studio.shell_template.{graph_id}")
}

fn shell_template_report(
    manifest: &StudioShellArtifactManifest,
    status: StudioShellTemplateStatus,
    issue_code: Option<String>,
    message: String,
    validation: StudioShellArtifactManifestValidationReport,
    index: Option<StudioShellTemplateIndex>,
    templates: Vec<StudioShellTemplateManifest>,
) -> StudioShellTemplateReport {
    StudioShellTemplateReport {
        schema_id: SHELL_TEMPLATE_REPORT_SCHEMA,
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        status,
        issue_code,
        message,
        validation,
        index,
        templates,
    }
}

fn duplicate_template_field<F>(entries: &[StudioShellTemplateIndexEntry], field: F) -> Vec<String>
where
    F: Fn(&StudioShellTemplateIndexEntry) -> &str,
{
    let mut counts = BTreeMap::new();
    for entry in entries {
        *counts.entry(field(entry).to_string()).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .filter_map(|(id, count)| (count > 1).then_some(id))
        .collect()
}
