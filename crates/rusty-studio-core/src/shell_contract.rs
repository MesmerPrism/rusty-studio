use super::*;

pub fn selected_shell_bundle_for_graph(
    project: &StudioProject,
    base_dir: Option<&Path>,
    graph_id: &str,
) -> StudioShellBundleReport {
    let descriptor_report = shell_descriptor_for_graph(project, base_dir, graph_id);
    let (StudioShellDescriptorStatus::Exported, Some(descriptor)) =
        (descriptor_report.status, descriptor_report.descriptor)
    else {
        return shell_bundle_report(
            project,
            graph_id,
            StudioShellBundleStatus::Rejected,
            descriptor_report.issue_code,
            descriptor_report.message,
            Vec::new(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );
    };

    let descriptor_validation = validate_shell_descriptor(&descriptor);
    if descriptor_validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_check_issue_code(&descriptor_validation)
            .unwrap_or_else(|| "studio.issue.shell_descriptor_invalid".to_string());
        return shell_bundle_report(
            project,
            graph_id,
            StudioShellBundleStatus::Rejected,
            Some(issue_code),
            "Generated shell descriptor failed validation".to_string(),
            Vec::new(),
            Some(descriptor_validation),
            None,
            None,
            Some(descriptor),
            None,
            None,
            None,
        );
    }

    let artifact = shell_artifact_for_descriptor(&descriptor);
    let artifact_manifest = StudioShellArtifactManifest {
        schema_id: SHELL_ARTIFACT_MANIFEST_SCHEMA.to_string(),
        manifest_id: selected_shell_bundle_manifest_id(&project.project_id, &descriptor.graph_id),
        project_id: project.project_id.clone(),
        project_revision: project.revision,
        artifacts: vec![artifact.clone()],
    };
    let artifact_validation = validate_shell_artifact_manifest(&artifact_manifest, None);
    if artifact_validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_shell_artifact_manifest_issue_code(&artifact_validation)
            .unwrap_or_else(|| "studio.issue.shell_artifact_manifest_invalid".to_string());
        return shell_bundle_report(
            project,
            graph_id,
            StudioShellBundleStatus::Rejected,
            Some(issue_code),
            "Generated selected shell artifact manifest failed validation".to_string(),
            Vec::new(),
            Some(descriptor_validation),
            Some(artifact_validation),
            None,
            Some(descriptor),
            Some(artifact_manifest),
            None,
            None,
        );
    }

    let template_manifest = shell_template_for_artifact(&artifact);
    let template_entry = shell_template_index_entry(&artifact);
    let template_index = StudioShellTemplateIndex {
        schema_id: SHELL_TEMPLATE_INDEX_SCHEMA.to_string(),
        index_id: selected_shell_bundle_template_index_id(
            &project.project_id,
            &descriptor.graph_id,
        ),
        manifest_id: artifact_manifest.manifest_id.clone(),
        project_id: project.project_id.clone(),
        project_revision: project.revision,
        templates: vec![template_entry.clone()],
    };
    let template_validation = validate_shell_template_index(&template_index, None);
    if template_validation.status == StudioValidationStatus::Fail {
        let issue_code = first_failed_shell_template_index_issue_code(&template_validation)
            .unwrap_or_else(|| "studio.issue.shell_template_index_invalid".to_string());
        return shell_bundle_report(
            project,
            graph_id,
            StudioShellBundleStatus::Rejected,
            Some(issue_code),
            "Generated selected shell template index failed validation".to_string(),
            Vec::new(),
            Some(descriptor_validation),
            Some(artifact_validation),
            Some(template_validation),
            Some(descriptor),
            Some(artifact_manifest),
            Some(template_index),
            Some(template_manifest),
        );
    }

    let bundle_files = selected_shell_bundle_files(&artifact, &template_entry);
    shell_bundle_report(
        project,
        graph_id,
        StudioShellBundleStatus::Exported,
        None,
        "Selected shell bundle exported".to_string(),
        bundle_files,
        Some(descriptor_validation),
        Some(artifact_validation),
        Some(template_validation),
        Some(descriptor),
        Some(artifact_manifest),
        Some(template_index),
        Some(template_manifest),
    )
}

pub fn save_shell_bundle(
    output_dir: &Path,
    report: &StudioShellBundleReport,
) -> Result<Vec<String>, StudioCoreError> {
    if report.status != StudioShellBundleStatus::Exported {
        return Ok(Vec::new());
    }
    let Some(descriptor) = report.descriptor.as_ref() else {
        return Ok(Vec::new());
    };
    let Some(artifact_manifest) = report.artifact_manifest.as_ref() else {
        return Ok(Vec::new());
    };
    let Some(template_index) = report.template_index.as_ref() else {
        return Ok(Vec::new());
    };
    let Some(template_manifest) = report.template_manifest.as_ref() else {
        return Ok(Vec::new());
    };

    let mut written_files = BTreeSet::new();
    for relative_path in descriptor_bundle_paths(report) {
        save_json(
            &relative_output_path(output_dir, &relative_path),
            descriptor,
        )?;
        written_files.insert(relative_path);
    }
    let manifold_handoff_path = shell_manifold_handoff_artifact_path(&descriptor.graph_id);
    save_json(
        &relative_output_path(output_dir, &manifold_handoff_path),
        &manifold_shell_handoff_for_descriptor(descriptor),
    )?;
    written_files.insert(manifold_handoff_path);
    save_json(
        &relative_output_path(output_dir, "shell-artifacts.json"),
        artifact_manifest,
    )?;
    written_files.insert("shell-artifacts.json".to_string());
    save_json(
        &relative_output_path(output_dir, "shell-templates.json"),
        template_index,
    )?;
    written_files.insert("shell-templates.json".to_string());
    for entry in &template_index.templates {
        if entry.template_id == template_manifest.template_id {
            save_json(
                &relative_output_path(output_dir, &entry.template_path),
                template_manifest,
            )?;
            written_files.insert(entry.template_path.clone());
        }
    }
    Ok(written_files.into_iter().collect())
}

pub fn validate_selected_shell_bundle(
    project: &StudioProject,
    base_dir: Option<&Path>,
    graph_id: &str,
    bundle_dir: &Path,
) -> StudioShellBundleValidationReport {
    let expected = selected_shell_bundle_for_graph(project, base_dir, graph_id);
    let expected_bundle_files = expected.bundle_files.clone();
    let mut checks = Vec::new();
    let preview_issue = expected
        .issue_code
        .as_deref()
        .unwrap_or("studio.issue.shell_bundle_preview_rejected");
    push_bundle_check(
        &mut checks,
        graph_id,
        "studio.check.shell_bundle.current_preview",
        expected.status == StudioShellBundleStatus::Exported,
        "current selected graph exports a shell bundle",
        &expected.message,
        preview_issue,
    );

    if expected.status != StudioShellBundleStatus::Exported {
        return shell_bundle_validation_report(project, graph_id, expected_bundle_files, checks);
    }

    for (index, relative_path) in expected.bundle_files.iter().enumerate() {
        let file_path = relative_output_path(bundle_dir, relative_path);
        push_bundle_check(
            &mut checks,
            graph_id,
            &format!("studio.check.shell_bundle.file.{index}.exists"),
            file_path.is_file(),
            "expected bundle file exists",
            &format!("expected bundle file is missing: {relative_path}"),
            "studio.issue.shell_bundle_file_missing",
        );
    }

    let expected_descriptor = expected.descriptor.as_ref();
    if let Some(descriptor_relative_path) = descriptor_bundle_paths(&expected).first().cloned() {
        let descriptor_path = relative_output_path(bundle_dir, &descriptor_relative_path);
        match load_shell_descriptor(&descriptor_path) {
            Ok(descriptor) => {
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.descriptor.parse",
                    true,
                    "descriptor JSON parsed",
                    "descriptor JSON did not parse",
                    "studio.issue.descriptor_parse_failed",
                );
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.descriptor.current_match",
                    expected_descriptor == Some(&descriptor),
                    "descriptor matches the current selected graph preview",
                    "descriptor differs from the current selected graph preview",
                    "studio.issue.shell_bundle_descriptor_mismatch",
                );
            }
            Err(error) => {
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.descriptor.parse",
                    false,
                    "descriptor JSON parsed",
                    &error.to_string(),
                    "studio.issue.descriptor_parse_failed",
                );
            }
        }
    } else {
        push_bundle_check(
            &mut checks,
            graph_id,
            "studio.check.shell_bundle.descriptor.path",
            false,
            "current preview has a descriptor path",
            "current preview has no descriptor path",
            "studio.issue.descriptor_missing",
        );
    }

    let expected_artifact_manifest = expected.artifact_manifest.as_ref();
    let artifact_manifest_path = relative_output_path(bundle_dir, "shell-artifacts.json");
    match load_shell_artifact_manifest(&artifact_manifest_path) {
        Ok(manifest) => {
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.artifact_manifest.parse",
                true,
                "artifact manifest JSON parsed",
                "artifact manifest JSON did not parse",
                "studio.issue.shell_artifact_manifest_parse_failed",
            );
            let validation = validate_shell_artifact_manifest(&manifest, Some(bundle_dir));
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.artifact_manifest.validation",
                validation.status == StudioValidationStatus::Pass,
                "artifact manifest validates against written descriptor files",
                "artifact manifest validation failed against written descriptor files",
                "studio.issue.shell_artifact_manifest_invalid",
            );
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.artifact_manifest.current_match",
                expected_artifact_manifest == Some(&manifest),
                "artifact manifest matches the current selected graph preview",
                "artifact manifest differs from the current selected graph preview",
                "studio.issue.shell_bundle_artifact_manifest_mismatch",
            );
        }
        Err(error) => {
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.artifact_manifest.parse",
                false,
                "artifact manifest JSON parsed",
                &error.to_string(),
                "studio.issue.shell_artifact_manifest_parse_failed",
            );
        }
    }

    let expected_template_index = expected.template_index.as_ref();
    let template_index_path = relative_output_path(bundle_dir, "shell-templates.json");
    let mut template_path_from_index = expected_template_index
        .and_then(|index| index.templates.first())
        .map(|entry| entry.template_path.clone());
    match load_shell_template_index(&template_index_path) {
        Ok(index) => {
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.template_index.parse",
                true,
                "template index JSON parsed",
                "template index JSON did not parse",
                "studio.issue.shell_template_index_parse_failed",
            );
            let validation = validate_shell_template_index(&index, Some(bundle_dir));
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.template_index.validation",
                validation.status == StudioValidationStatus::Pass,
                "template index validates against written template and descriptor files",
                "template index validation failed against written template and descriptor files",
                "studio.issue.shell_template_index_invalid",
            );
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.template_index.current_match",
                expected_template_index == Some(&index),
                "template index matches the current selected graph preview",
                "template index differs from the current selected graph preview",
                "studio.issue.shell_bundle_template_index_mismatch",
            );
            if template_path_from_index.is_none() {
                template_path_from_index = index
                    .templates
                    .first()
                    .map(|entry| entry.template_path.clone());
            }
        }
        Err(error) => {
            push_bundle_check(
                &mut checks,
                graph_id,
                "studio.check.shell_bundle.template_index.parse",
                false,
                "template index JSON parsed",
                &error.to_string(),
                "studio.issue.shell_template_index_parse_failed",
            );
        }
    }

    let expected_template_manifest = expected.template_manifest.as_ref();
    if let Some(template_relative_path) = template_path_from_index {
        let template_path = relative_output_path(bundle_dir, &template_relative_path);
        match load_shell_template_manifest(&template_path) {
            Ok(template) => {
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.template_manifest.parse",
                    true,
                    "template manifest JSON parsed",
                    "template manifest JSON did not parse",
                    "studio.issue.shell_template_manifest_parse_failed",
                );
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.template_manifest.current_match",
                    expected_template_manifest == Some(&template),
                    "template manifest matches the current selected graph preview",
                    "template manifest differs from the current selected graph preview",
                    "studio.issue.shell_bundle_template_manifest_mismatch",
                );
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.template_manifest.runtime_authority",
                    template.runtime_authority.command_session_authority == "rusty.manifold"
                        && template.runtime_authority.install_launch_evidence_authority
                            == "rusty.hostess"
                        && template.runtime_authority.studio_role == "authoring.export_planning",
                    "template manifest preserves Manifold, Hostess, and Studio authority boundaries",
                    "template manifest runtime authority changed",
                    "studio.issue.runtime_authority_mismatch",
                );
            }
            Err(error) => {
                push_bundle_check(
                    &mut checks,
                    graph_id,
                    "studio.check.shell_bundle.template_manifest.parse",
                    false,
                    "template manifest JSON parsed",
                    &error.to_string(),
                    "studio.issue.shell_template_manifest_parse_failed",
                );
            }
        }
    } else {
        push_bundle_check(
            &mut checks,
            graph_id,
            "studio.check.shell_bundle.template_manifest.path",
            false,
            "current preview has a template manifest path",
            "current preview has no template manifest path",
            "studio.issue.template_missing",
        );
    }

    shell_bundle_validation_report(project, graph_id, expected_bundle_files, checks)
}

pub(crate) fn shell_target_kind(host_profile_class: Option<&str>) -> StudioShellTargetKind {
    match host_profile_class {
        Some("host.desktop") => StudioShellTargetKind::Desktop,
        Some("host.mobile") => StudioShellTargetKind::Phone,
        Some("host.headset") | Some("host.quest") => StudioShellTargetKind::Quest,
        _ => StudioShellTargetKind::Unknown,
    }
}

pub(crate) fn shell_handoff_manifest_id(project_id: &str) -> String {
    format!("studio.shell_handoffs.{project_id}")
}

pub(crate) fn shell_runtime_authority() -> StudioShellRuntimeAuthority {
    StudioShellRuntimeAuthority {
        command_session_authority: "rusty.manifold".to_string(),
        install_launch_evidence_authority: "rusty.hostess".to_string(),
        studio_role: "authoring.export_planning".to_string(),
    }
}

fn shell_bundle_report(
    project: &StudioProject,
    graph_id: &str,
    status: StudioShellBundleStatus,
    issue_code: Option<String>,
    message: String,
    bundle_files: Vec<String>,
    descriptor_validation: Option<StudioShellDescriptorValidationReport>,
    artifact_validation: Option<StudioShellArtifactManifestValidationReport>,
    template_validation: Option<StudioShellTemplateIndexValidationReport>,
    descriptor: Option<StudioShellDescriptor>,
    artifact_manifest: Option<StudioShellArtifactManifest>,
    template_index: Option<StudioShellTemplateIndex>,
    template_manifest: Option<StudioShellTemplateManifest>,
) -> StudioShellBundleReport {
    StudioShellBundleReport {
        schema_id: SHELL_BUNDLE_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        graph_id: graph_id.to_string(),
        status,
        issue_code,
        message,
        bundle_files,
        descriptor_validation,
        artifact_validation,
        template_validation,
        descriptor,
        artifact_manifest,
        template_index,
        template_manifest,
    }
}

fn shell_bundle_validation_report(
    project: &StudioProject,
    graph_id: &str,
    expected_bundle_files: Vec<String>,
    checks: Vec<StudioValidationCheck>,
) -> StudioShellBundleValidationReport {
    StudioShellBundleValidationReport {
        schema_id: SHELL_BUNDLE_VALIDATION_REPORT_SCHEMA,
        project_id: project.project_id.clone(),
        revision: project.revision,
        graph_id: graph_id.to_string(),
        status: if checks
            .iter()
            .any(|check| check.status == StudioValidationStatus::Fail)
        {
            StudioValidationStatus::Fail
        } else {
            StudioValidationStatus::Pass
        },
        expected_bundle_files,
        checks,
    }
}

fn selected_shell_bundle_manifest_id(project_id: &str, graph_id: &str) -> String {
    format!("studio.shell_artifacts.{project_id}.{graph_id}")
}

fn selected_shell_bundle_template_index_id(project_id: &str, graph_id: &str) -> String {
    format!("studio.shell_templates.{project_id}.{graph_id}")
}

fn selected_shell_bundle_files(
    artifact: &StudioShellArtifact,
    template_entry: &StudioShellTemplateIndexEntry,
) -> Vec<String> {
    let mut files = BTreeSet::new();
    files.insert(artifact.descriptor_path.clone());
    files.insert(shell_manifold_handoff_artifact_path(&artifact.graph_id));
    files.insert(template_entry.descriptor_path.clone());
    files.insert(template_entry.template_path.clone());
    files.insert("shell-artifacts.json".to_string());
    files.insert("shell-templates.json".to_string());
    files.into_iter().collect()
}

fn descriptor_bundle_paths(report: &StudioShellBundleReport) -> Vec<String> {
    let mut paths = BTreeSet::new();
    if let Some(manifest) = report.artifact_manifest.as_ref() {
        for artifact in &manifest.artifacts {
            paths.insert(artifact.descriptor_path.clone());
        }
    }
    if let Some(index) = report.template_index.as_ref() {
        for entry in &index.templates {
            paths.insert(entry.descriptor_path.clone());
        }
    }
    paths.into_iter().collect()
}

pub(crate) fn relative_output_path(output_dir: &Path, relative_path: &str) -> PathBuf {
    relative_path
        .split('/')
        .fold(output_dir.to_path_buf(), |path, segment| path.join(segment))
}

pub(crate) fn is_safe_relative_manifest_path(value: &str) -> bool {
    if value.is_empty() || value.contains('\\') {
        return false;
    }
    let path = Path::new(value);
    if path.is_absolute() {
        return false;
    }
    path.components()
        .all(|component| matches!(component, std::path::Component::Normal(_)))
}

pub(crate) fn resolve_manifest_relative_path(base_dir: &Path, relative_path: &str) -> PathBuf {
    relative_path
        .split('/')
        .fold(base_dir.to_path_buf(), |path, segment| path.join(segment))
}
