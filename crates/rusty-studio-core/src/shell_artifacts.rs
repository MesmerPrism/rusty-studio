use super::*;

pub fn shell_artifacts_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
) -> StudioShellArtifactReport {
    let validation = validate_project_with_base(project, base_dir);
    if validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_issue_code(&validation)
            .unwrap_or_else(|| "studio.issue.validation_failed".to_string());
        return shell_artifact_report(
            project,
            StudioShellArtifactStatus::Rejected,
            Some(issue_code),
            "Project validation failed; shell artifacts were not exported".to_string(),
            validation,
            None,
            Vec::new(),
            Vec::new(),
        );
    }

    let mut artifacts = Vec::new();
    let mut descriptors = Vec::new();
    let mut rejections = Vec::new();
    for graph in &project.graphs {
        let descriptor_report = shell_descriptor_for_graph(project, base_dir, &graph.graph_id);
        match (descriptor_report.status, descriptor_report.descriptor) {
            (StudioShellDescriptorStatus::Exported, Some(descriptor)) => {
                let descriptor_validation = validate_shell_descriptor(&descriptor);
                if descriptor_validation.status == StudioValidationStatus::Pass {
                    artifacts.push(shell_artifact_for_descriptor(&descriptor));
                    descriptors.push(descriptor);
                } else {
                    let issue_code = first_failed_check_issue_code(&descriptor_validation)
                        .unwrap_or_else(|| "studio.issue.shell_descriptor_invalid".to_string());
                    rejections.push(StudioShellArtifactRejection {
                        graph_id: graph.graph_id.clone(),
                        issue_code: Some(issue_code),
                        message: "Generated shell descriptor failed validation".to_string(),
                    });
                }
            }
            (_, _) => {
                rejections.push(StudioShellArtifactRejection {
                    graph_id: graph.graph_id.clone(),
                    issue_code: descriptor_report.issue_code,
                    message: descriptor_report.message,
                });
            }
        }
    }

    if !rejections.is_empty() {
        return shell_artifact_report(
            project,
            StudioShellArtifactStatus::Rejected,
            rejections
                .first()
                .and_then(|rejection| rejection.issue_code.clone()),
            "One or more graph shell descriptors could not be exported".to_string(),
            validation,
            None,
            Vec::new(),
            rejections,
        );
    }

    let manifest = StudioShellArtifactManifest {
        schema_id: SHELL_ARTIFACT_MANIFEST_SCHEMA.to_string(),
        manifest_id: format!("studio.shell_artifacts.{}", project.project_id),
        project_id: project.project_id.clone(),
        project_revision: project.revision,
        artifacts,
    };
    shell_artifact_report(
        project,
        StudioShellArtifactStatus::Exported,
        None,
        "Shell artifacts exported".to_string(),
        validation,
        Some(manifest),
        descriptors,
        Vec::new(),
    )
}

pub fn validate_shell_artifact_manifest(
    manifest: &StudioShellArtifactManifest,
    base_dir: Option<&Path>,
) -> StudioShellArtifactManifestValidationReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.schema",
        manifest.schema_id == SHELL_ARTIFACT_MANIFEST_SCHEMA,
        "shell artifact manifest schema id is supported",
        "shell artifact manifest schema id is unsupported",
        "studio.issue.shell_artifact_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.manifest_id",
        is_dotted_id(&manifest.manifest_id),
        "manifest id uses dotted-id grammar",
        "manifest id is not a dotted id",
        "studio.issue.invalid_manifest_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.project_id",
        is_dotted_id(&manifest.project_id),
        "project id uses dotted-id grammar",
        "project id is not a dotted id",
        "studio.issue.invalid_project_id",
    );
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.project_revision",
        manifest.project_revision > 0,
        "project revision is positive",
        "project revision must be positive",
        "studio.issue.invalid_revision",
    );
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.artifacts_present",
        !manifest.artifacts.is_empty(),
        "manifest declares shell artifacts",
        "manifest must declare at least one shell artifact",
        "studio.issue.no_shell_artifacts",
    );

    let duplicate_artifact_ids = duplicate_artifact_field(&manifest.artifacts, |artifact| {
        artifact.artifact_id.as_str()
    });
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.unique_artifact_ids",
        duplicate_artifact_ids.is_empty(),
        "artifact ids are unique",
        &format!(
            "duplicate artifact ids: {}",
            duplicate_artifact_ids.join(", ")
        ),
        "studio.issue.duplicate_artifact_id",
    );
    let duplicate_graph_ids =
        duplicate_artifact_field(&manifest.artifacts, |artifact| artifact.graph_id.as_str());
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.unique_graph_ids",
        duplicate_graph_ids.is_empty(),
        "artifact graph ids are unique",
        &format!(
            "duplicate artifact graph ids: {}",
            duplicate_graph_ids.join(", ")
        ),
        "studio.issue.duplicate_artifact_graph_id",
    );
    let duplicate_descriptor_paths = duplicate_artifact_field(&manifest.artifacts, |artifact| {
        artifact.descriptor_path.as_str()
    });
    push_check(
        &mut checks,
        "studio.check.shell_artifact_manifest.unique_descriptor_paths",
        duplicate_descriptor_paths.is_empty(),
        "descriptor paths are unique",
        &format!(
            "duplicate descriptor paths: {}",
            duplicate_descriptor_paths.join(", ")
        ),
        "studio.issue.duplicate_descriptor_path",
    );

    for artifact in &manifest.artifacts {
        validate_shell_artifact_manifest_entry(artifact, base_dir, &mut checks);
    }

    StudioShellArtifactManifestValidationReport {
        schema_id: SHELL_ARTIFACT_MANIFEST_VALIDATION_REPORT_SCHEMA,
        manifest_id: manifest.manifest_id.clone(),
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

pub(crate) fn shell_artifact_for_descriptor(
    descriptor: &StudioShellDescriptor,
) -> StudioShellArtifact {
    StudioShellArtifact {
        artifact_id: format!("studio.shell_artifact.{}", descriptor.graph_id),
        graph_id: descriptor.graph_id.clone(),
        shell_id: descriptor.shell_id.clone(),
        target_kind: shell_target_kind(descriptor.host_profile.host_profile.as_deref()),
        target_host_profile: descriptor.target_host_profile.clone(),
        host_profile_class: descriptor.host_profile.host_profile.clone(),
        descriptor_path: shell_descriptor_artifact_path(&descriptor.graph_id),
        app_id: descriptor.host_profile.app_id.clone(),
        install_route: descriptor.host_profile.install_route.clone(),
        launch_route: descriptor.host_profile.launch_route.clone(),
        command_bridge: descriptor.host_profile.command_bridge.clone(),
        evidence_pull_route: descriptor.host_profile.evidence_pull_route.clone(),
        package_ids: descriptor.package_ids.clone(),
        module_ids: descriptor.module_ids.clone(),
    }
}

pub(crate) fn shell_manifold_handoff_artifact_path(graph_id: &str) -> String {
    format!("descriptors/{graph_id}.manifold-shell-handoff.json")
}

pub(crate) fn manifold_shell_handoff_for_descriptor(
    descriptor: &StudioShellDescriptor,
) -> StudioGeneratedManifoldShellHandoffManifest {
    StudioGeneratedManifoldShellHandoffManifest {
        schema_id: MANIFOLD_SHELL_HANDOFF_SCHEMA,
        handoff_id: format!("shell_handoff.{}", descriptor.graph_id),
        handoff_revision: descriptor.project_revision,
        target_host_profile: descriptor
            .host_profile
            .host_profile
            .clone()
            .unwrap_or_else(|| descriptor.target_host_profile.clone()),
        shell_app_id: descriptor
            .host_profile
            .app_id
            .clone()
            .unwrap_or_else(|| descriptor.shell_id.clone()),
        validation_slot_id: descriptor
            .validation_slot_ids
            .first()
            .cloned()
            .unwrap_or_else(|| DEFAULT_MANIFOLD_SHELL_HANDOFF_VALIDATION_SLOT_ID.to_string()),
        stream_bindings: descriptor
            .stream_bindings
            .iter()
            .filter(|binding| binding.binding_id.starts_with("stream."))
            .map(manifold_shell_stream_binding)
            .collect(),
        command_ids: descriptor
            .command_bindings
            .iter()
            .filter_map(|binding| {
                binding
                    .binding_id
                    .starts_with("command.")
                    .then(|| binding.binding_id.clone())
            })
            .collect(),
        transport_offers: vec![StudioGeneratedManifoldTransportOffer {
            transport_id: format!("transport.shell_handoff.{}", descriptor.graph_id),
            transport: manifold_transport_for_command_bridge(
                descriptor.host_profile.command_bridge.as_deref(),
            ),
            endpoint_id: None,
        }],
        expected_scorecard_id: format!("scorecard.shell_handoff.{}", descriptor.graph_id),
    }
}

fn manifold_shell_stream_binding(
    binding: &StudioShellBinding,
) -> StudioGeneratedManifoldShellStreamBinding {
    let shell_is_source = binding.source_node_id.contains(".shell.");
    let direction = if shell_is_source {
        StudioGeneratedManifoldShellStreamDirection::Publish
    } else {
        StudioGeneratedManifoldShellStreamDirection::Subscribe
    };
    let role = match direction {
        StudioGeneratedManifoldShellStreamDirection::Publish => "role.shell.publish",
        StudioGeneratedManifoldShellStreamDirection::Subscribe => "role.shell.subscribe",
    };
    StudioGeneratedManifoldShellStreamBinding {
        stream_id: binding.binding_id.clone(),
        direction,
        role: role.to_string(),
        required: true,
    }
}

fn manifold_transport_for_command_bridge(
    command_bridge: Option<&str>,
) -> StudioGeneratedManifoldEndpointTransport {
    match command_bridge {
        Some(bridge) if bridge.contains("http") => StudioGeneratedManifoldEndpointTransport::Http,
        Some(bridge) if bridge.contains("stdio") || bridge.contains("cli") => {
            StudioGeneratedManifoldEndpointTransport::Stdio
        }
        _ => StudioGeneratedManifoldEndpointTransport::InProcess,
    }
}

fn shell_artifact_report(
    project: &StudioProject,
    status: StudioShellArtifactStatus,
    issue_code: Option<String>,
    message: String,
    validation: StudioValidationReport,
    manifest: Option<StudioShellArtifactManifest>,
    descriptors: Vec<StudioShellDescriptor>,
    rejections: Vec<StudioShellArtifactRejection>,
) -> StudioShellArtifactReport {
    StudioShellArtifactReport {
        schema_id: SHELL_ARTIFACT_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        status,
        issue_code,
        message,
        validation,
        manifest,
        descriptors,
        rejections,
    }
}

fn validate_shell_artifact_manifest_entry(
    artifact: &StudioShellArtifact,
    base_dir: Option<&Path>,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = artifact.artifact_id.clone();
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.id"),
        is_dotted_id(&artifact.artifact_id),
        "artifact id uses dotted-id grammar",
        "artifact id is not a dotted id",
        "studio.issue.invalid_artifact_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.graph_id"),
        is_dotted_id(&artifact.graph_id),
        "artifact graph id uses dotted-id grammar",
        "artifact graph id is not a dotted id",
        "studio.issue.invalid_graph_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.shell_id"),
        is_dotted_id(&artifact.shell_id),
        "artifact shell id uses dotted-id grammar",
        "artifact shell id is not a dotted id",
        "studio.issue.invalid_shell_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.target_host_profile"),
        is_dotted_id(&artifact.target_host_profile),
        "target host profile uses dotted-id grammar",
        "target host profile is not a dotted id",
        "studio.issue.invalid_target_host_profile",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.host_profile_class"),
        optional_dotted_id(artifact.host_profile_class.as_deref()),
        "host profile class is absent or uses dotted-id grammar",
        "host profile class is not a dotted id",
        "studio.issue.invalid_host_profile_class",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.app_id"),
        optional_dotted_id(artifact.app_id.as_deref()),
        "app id is absent or uses dotted-id grammar",
        "app id is not a dotted id",
        "studio.issue.invalid_app_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.install_route"),
        optional_dotted_id(artifact.install_route.as_deref()),
        "install route is absent or uses dotted-id grammar",
        "install route is not a dotted id",
        "studio.issue.invalid_install_route",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.launch_route"),
        optional_dotted_id(artifact.launch_route.as_deref()),
        "launch route is absent or uses dotted-id grammar",
        "launch route is not a dotted id",
        "studio.issue.invalid_launch_route",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.command_bridge"),
        optional_dotted_id(artifact.command_bridge.as_deref()),
        "command bridge is absent or uses dotted-id grammar",
        "command bridge is not a dotted id",
        "studio.issue.invalid_command_bridge",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.evidence_pull_route"),
        optional_dotted_id(artifact.evidence_pull_route.as_deref()),
        "evidence pull route is absent or uses dotted-id grammar",
        "evidence pull route is not a dotted id",
        "studio.issue.invalid_evidence_pull_route",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.package_ids"),
        all_dotted_ids(&artifact.package_ids),
        "package ids use dotted-id grammar",
        "one or more package ids are not dotted ids",
        "studio.issue.invalid_package_id",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.module_ids"),
        all_dotted_ids(&artifact.module_ids),
        "module ids use dotted-id grammar",
        "one or more module ids are not dotted ids",
        "studio.issue.invalid_module_id",
    );

    let descriptor_path_is_safe = is_safe_relative_manifest_path(&artifact.descriptor_path);
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_path"),
        descriptor_path_is_safe,
        "descriptor path is a safe relative path",
        "descriptor path must be a portable relative path without traversal",
        "studio.issue.invalid_descriptor_path",
    );

    if let Some(base_dir) = base_dir.filter(|_| descriptor_path_is_safe) {
        validate_shell_artifact_descriptor_reference(artifact, base_dir, checks);
    }
}

fn validate_shell_artifact_descriptor_reference(
    artifact: &StudioShellArtifact,
    base_dir: &Path,
    checks: &mut Vec<StudioValidationCheck>,
) {
    let prefix = artifact.artifact_id.clone();
    let descriptor_path = resolve_manifest_relative_path(base_dir, &artifact.descriptor_path);
    let descriptor_exists = descriptor_path.is_file();
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_exists"),
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
                &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_parse"),
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
                    "studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_parse"
                ),
                status: StudioValidationStatus::Fail,
                evidence: error.to_string(),
                issue_code: Some("studio.issue.descriptor_parse_failed".to_string()),
                graph_id: Some(artifact.graph_id.clone()),
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
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_validation"),
        descriptor_validation.status == StudioValidationStatus::Pass,
        "descriptor validation passed",
        "descriptor validation failed",
        "studio.issue.descriptor_validation_failed",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_graph"),
        descriptor.graph_id == artifact.graph_id,
        "descriptor graph id matches artifact graph id",
        "descriptor graph id does not match artifact graph id",
        "studio.issue.descriptor_graph_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_shell"),
        descriptor.shell_id == artifact.shell_id,
        "descriptor shell id matches artifact shell id",
        "descriptor shell id does not match artifact shell id",
        "studio.issue.descriptor_shell_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_target"),
        descriptor.target_host_profile == artifact.target_host_profile,
        "descriptor target host profile matches artifact target host profile",
        "descriptor target host profile does not match artifact target host profile",
        "studio.issue.descriptor_target_mismatch",
    );
    push_check(
        checks,
        &format!(
            "studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_host_profile_class"
        ),
        descriptor.host_profile.host_profile == artifact.host_profile_class,
        "descriptor host profile class matches artifact host profile class",
        "descriptor host profile class does not match artifact host profile class",
        "studio.issue.descriptor_host_profile_class_mismatch",
    );
    push_check(
        checks,
        &format!("studio.check.shell_artifact_manifest.artifact.{prefix}.descriptor_target_kind"),
        shell_target_kind(descriptor.host_profile.host_profile.as_deref()) == artifact.target_kind,
        "descriptor target kind matches artifact target kind",
        "descriptor target kind does not match artifact target kind",
        "studio.issue.descriptor_target_kind_mismatch",
    );
}

fn duplicate_artifact_field<F>(artifacts: &[StudioShellArtifact], field: F) -> Vec<String>
where
    F: Fn(&StudioShellArtifact) -> &str,
{
    let mut counts = BTreeMap::new();
    for artifact in artifacts {
        *counts.entry(field(artifact).to_string()).or_insert(0) += 1;
    }
    counts
        .into_iter()
        .filter_map(|(id, count)| (count > 1).then_some(id))
        .collect()
}
