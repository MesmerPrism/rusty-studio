use super::*;

#[test]
fn shell_descriptor_exports_valid_graph() {
    let root = temp_root("shell-descriptor");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();

    let report = shell_descriptor_for_graph(&project, Some(&root), "studio.graph.test");
    let descriptor = report.descriptor.expect("descriptor exported");

    assert_eq!(report.status, StudioShellDescriptorStatus::Exported);
    assert_eq!(report.issue_code, None);
    assert_eq!(descriptor.schema_id, SHELL_DESCRIPTOR_SCHEMA);
    assert_eq!(descriptor.project_id, "studio.project.test");
    assert_eq!(descriptor.shell_id, "shell.synthetic.operator");
    assert_eq!(descriptor.target_host_profile, "host_run.profile.desktop");
    assert_eq!(
        descriptor.host_profile.host_profile.as_deref(),
        Some("host.desktop")
    );
    assert_eq!(descriptor.package_ids, vec!["package.synthetic"]);
    assert_eq!(descriptor.module_ids, vec!["module.synthetic_provider"]);
    assert_eq!(descriptor.command_bindings.len(), 1);
}

#[test]
fn shell_descriptor_roundtrips_and_validates() {
    let root = temp_root("shell-descriptor-roundtrip");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();
    let report = shell_descriptor_for_graph(&project, Some(&root), "studio.graph.test");
    let descriptor = report.descriptor.expect("descriptor exported");
    let path = root.join("target/descriptor.json");

    save_json(&path, &descriptor).expect("save descriptor");
    let loaded = load_shell_descriptor(&path).expect("load descriptor");
    let validation = validate_shell_descriptor(&loaded);

    assert_eq!(loaded, descriptor);
    assert_eq!(
        validation.schema_id,
        SHELL_DESCRIPTOR_VALIDATION_REPORT_SCHEMA
    );
    assert_eq!(validation.status, StudioValidationStatus::Pass);
}

#[test]
fn shell_descriptor_validation_rejects_target_mismatch() {
    let root = temp_root("shell-descriptor-target-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();
    let report = shell_descriptor_for_graph(&project, Some(&root), "studio.graph.test");
    let mut descriptor = report.descriptor.expect("descriptor exported");
    descriptor.host_profile.profile_id = "host_run.profile.headset".to_string();

    let validation = validate_shell_descriptor(&descriptor);

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.host_profile_target_mismatch")
    }));
}

#[test]
fn shell_artifacts_export_desktop_phone_and_quest_descriptors() {
    let root = temp_root("shell-artifacts");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();

    let report = shell_artifacts_for_project(&project, Some(&root));
    let manifest = report.manifest.expect("shell artifact manifest");

    assert_eq!(report.status, StudioShellArtifactStatus::Exported);
    assert_eq!(report.issue_code, None);
    assert_eq!(manifest.schema_id, SHELL_ARTIFACT_MANIFEST_SCHEMA);
    assert_eq!(manifest.artifacts.len(), 3);
    assert_eq!(report.descriptors.len(), 3);
    assert!(report.rejections.is_empty());
    assert!(manifest
        .artifacts
        .iter()
        .any(|artifact| artifact.target_kind == StudioShellTargetKind::Desktop));
    assert!(manifest
        .artifacts
        .iter()
        .any(|artifact| artifact.target_kind == StudioShellTargetKind::Phone));
    assert!(manifest
        .artifacts
        .iter()
        .any(|artifact| artifact.target_kind == StudioShellTargetKind::Quest));
    assert!(manifest.artifacts.iter().all(|artifact| {
        artifact
            .descriptor_path
            .starts_with("descriptors/studio.graph.")
            && artifact.descriptor_path.ends_with(".shell-descriptor.json")
    }));
    assert!(report
        .descriptors
        .iter()
        .all(|descriptor| validate_shell_descriptor(descriptor).status
            == StudioValidationStatus::Pass));
}

#[test]
fn shell_artifact_manifest_roundtrips_and_validates_descriptors() {
    let root = temp_root("shell-artifact-manifest-roundtrip");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let report = shell_artifacts_for_project(&project, Some(&root));
    let manifest = report.manifest.expect("shell artifact manifest");
    let manifest_path = root.join("shell-artifacts.json");

    for descriptor in &report.descriptors {
        let descriptor_path = resolve_manifest_relative_path(
            &root,
            &shell_descriptor_artifact_path(&descriptor.graph_id),
        );
        save_json(&descriptor_path, descriptor).expect("save descriptor");
    }
    save_json(&manifest_path, &manifest).expect("save manifest");

    let loaded_manifest =
        load_shell_artifact_manifest(&manifest_path).expect("load shell artifact manifest");
    let validation = validate_shell_artifact_manifest(&loaded_manifest, Some(&root));

    assert_eq!(loaded_manifest, manifest);
    assert_eq!(
        validation.schema_id,
        SHELL_ARTIFACT_MANIFEST_VALIDATION_REPORT_SCHEMA
    );
    assert_eq!(validation.status, StudioValidationStatus::Pass);
}

#[test]
fn shell_artifact_manifest_validation_rejects_descriptor_mismatch() {
    let root = temp_root("shell-artifact-manifest-descriptor-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let report = shell_artifacts_for_project(&project, Some(&root));
    let mut manifest = report.manifest.expect("shell artifact manifest");

    for descriptor in &report.descriptors {
        let descriptor_path = resolve_manifest_relative_path(
            &root,
            &shell_descriptor_artifact_path(&descriptor.graph_id),
        );
        save_json(&descriptor_path, descriptor).expect("save descriptor");
    }
    manifest.artifacts[1].shell_id = "shell.synthetic.changed".to_string();

    let validation = validate_shell_artifact_manifest(&manifest, Some(&root));

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.descriptor_shell_mismatch")
    }));
}

#[test]
fn shell_artifact_manifest_validation_rejects_path_traversal() {
    let root = temp_root("shell-artifact-manifest-path-traversal");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let report = shell_artifacts_for_project(&project, Some(&root));
    let mut manifest = report.manifest.expect("shell artifact manifest");
    manifest.artifacts[0].descriptor_path = "../outside.json".to_string();

    let validation = validate_shell_artifact_manifest(&manifest, Some(&root));

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.invalid_descriptor_path")
    }));
}

#[test]
fn shell_templates_export_manifest_driven_index_and_templates() {
    let root = temp_root("shell-templates");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let report = shell_artifacts_for_project(&project, Some(&root));
    let manifest = report.manifest.expect("shell artifact manifest");

    for descriptor in &report.descriptors {
        let descriptor_path = resolve_manifest_relative_path(
            &root,
            &shell_descriptor_artifact_path(&descriptor.graph_id),
        );
        save_json(&descriptor_path, descriptor).expect("save descriptor");
    }

    let template_report = shell_templates_for_artifact_manifest(&manifest, Some(&root));
    let index = template_report.index.expect("shell template index");

    assert_eq!(template_report.status, StudioShellTemplateStatus::Exported);
    assert_eq!(template_report.issue_code, None);
    assert_eq!(index.schema_id, SHELL_TEMPLATE_INDEX_SCHEMA);
    assert_eq!(index.templates.len(), 3);
    assert_eq!(template_report.templates.len(), 3);
    assert!(index
        .templates
        .iter()
        .any(|entry| entry.template_path.starts_with("shells/desktop/")));
    assert!(index
        .templates
        .iter()
        .any(|entry| entry.template_path.starts_with("shells/phone/")));
    assert!(index
        .templates
        .iter()
        .any(|entry| entry.template_path.starts_with("shells/quest/")));
    assert!(template_report.templates.iter().all(|template| {
        template.schema_id == SHELL_TEMPLATE_MANIFEST_SCHEMA
            && template.descriptor_path.starts_with("descriptors/")
            && template.runtime_authority.command_session_authority == "rusty.manifold"
            && template.runtime_authority.install_launch_evidence_authority == "rusty.hostess"
            && template.runtime_authority.studio_role == "authoring.export_planning"
    }));
}

#[test]
fn selected_shell_bundle_exports_single_graph_contract() {
    let root = temp_root("selected-shell-bundle");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();

    let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");

    assert_eq!(report.status, StudioShellBundleStatus::Exported);
    assert_eq!(report.issue_code, None);
    assert_eq!(report.graph_id, "studio.graph.test");
    assert_eq!(
        report.bundle_files,
        vec![
            "descriptors/studio.graph.test.manifold-shell-handoff.json".to_string(),
            "descriptors/studio.graph.test.shell-descriptor.json".to_string(),
            "shell-artifacts.json".to_string(),
            "shell-templates.json".to_string(),
            "shells/desktop/studio.graph.test.shell-template.json".to_string(),
        ]
    );
    assert_eq!(
        report
            .descriptor_validation
            .as_ref()
            .expect("descriptor validation")
            .status,
        StudioValidationStatus::Pass
    );
    assert_eq!(
        report
            .artifact_validation
            .as_ref()
            .expect("artifact validation")
            .status,
        StudioValidationStatus::Pass
    );
    assert_eq!(
        report
            .template_validation
            .as_ref()
            .expect("template validation")
            .status,
        StudioValidationStatus::Pass
    );
    let manifest = report
        .artifact_manifest
        .as_ref()
        .expect("artifact manifest");
    assert_eq!(manifest.artifacts.len(), 1);
    assert_eq!(
        manifest.manifest_id,
        "studio.shell_artifacts.studio.project.test.studio.graph.test"
    );
    let index = report.template_index.as_ref().expect("template index");
    assert_eq!(index.templates.len(), 1);
    assert_eq!(
        index.index_id,
        "studio.shell_templates.studio.project.test.studio.graph.test"
    );
    assert_eq!(
        report
            .template_manifest
            .as_ref()
            .expect("template manifest")
            .runtime_authority
            .install_launch_evidence_authority,
        "rusty.hostess"
    );
}

#[test]
fn selected_shell_bundle_writes_valid_files() {
    let root = temp_root("selected-shell-bundle-write");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();
    let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");
    let output_dir = root.join("selected-shell");

    let written_files = save_shell_bundle(&output_dir, &report).expect("save shell bundle");

    assert_eq!(written_files, report.bundle_files);
    assert!(output_dir
        .join("descriptors/studio.graph.test.shell-descriptor.json")
        .is_file());
    let manifold_handoff_path =
        output_dir.join("descriptors/studio.graph.test.manifold-shell-handoff.json");
    assert!(manifold_handoff_path.is_file());
    assert!(output_dir.join("shell-artifacts.json").is_file());
    assert!(output_dir.join("shell-templates.json").is_file());
    assert!(output_dir
        .join("shells/desktop/studio.graph.test.shell-template.json")
        .is_file());
    let manifold_handoff: Value = serde_json::from_str(
        &std::fs::read_to_string(&manifold_handoff_path).expect("read Manifold handoff"),
    )
    .expect("parse Manifold handoff");
    assert_eq!(
        manifold_handoff.get("$schema").and_then(Value::as_str),
        Some(MANIFOLD_SHELL_HANDOFF_SCHEMA)
    );
    assert_eq!(
        manifold_handoff.get("handoff_id").and_then(Value::as_str),
        Some("shell_handoff.studio.graph.test")
    );
    assert_eq!(
        manifold_handoff
            .get("validation_slot_id")
            .and_then(Value::as_str),
        Some(DEFAULT_MANIFOLD_SHELL_HANDOFF_VALIDATION_SLOT_ID)
    );

    let manifest = load_shell_artifact_manifest(&output_dir.join("shell-artifacts.json")).unwrap();
    let artifact_validation = validate_shell_artifact_manifest(&manifest, Some(&output_dir));
    assert_eq!(artifact_validation.status, StudioValidationStatus::Pass);
    let index = load_shell_template_index(&output_dir.join("shell-templates.json")).unwrap();
    let template_validation = validate_shell_template_index(&index, Some(&output_dir));
    assert_eq!(template_validation.status, StudioValidationStatus::Pass);
}

#[test]
fn selected_shell_bundle_validation_passes_written_bundle() {
    let root = temp_root("selected-shell-bundle-validate");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();
    let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");
    let output_dir = root.join("selected-shell");
    save_shell_bundle(&output_dir, &report).expect("save shell bundle");

    let validation =
        validate_selected_shell_bundle(&project, Some(&root), "studio.graph.test", &output_dir);

    assert_eq!(validation.status, StudioValidationStatus::Pass);
    assert_eq!(validation.expected_bundle_files, report.bundle_files);
    assert!(validation
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn selected_shell_bundle_validation_rejects_stale_descriptor() {
    let root = temp_root("selected-shell-bundle-stale");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();
    let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");
    let output_dir = root.join("selected-shell");
    save_shell_bundle(&output_dir, &report).expect("save shell bundle");
    let descriptor_path = output_dir.join("descriptors/studio.graph.test.shell-descriptor.json");
    let mut descriptor = load_shell_descriptor(&descriptor_path).expect("load descriptor");
    descriptor.shell_id = "shell.synthetic.stale".to_string();
    save_json(&descriptor_path, &descriptor).expect("save stale descriptor");

    let validation =
        validate_selected_shell_bundle(&project, Some(&root), "studio.graph.test", &output_dir);

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.shell_bundle_descriptor_mismatch")
    }));
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.shell_artifact_manifest_invalid")
    }));
}

#[test]
fn selected_shell_bundle_reports_descriptor_rejection() {
    let root = temp_root("selected-shell-bundle-reject");
    write_reference_fixture_tree(&root);
    let project = valid_project_with_relative_references();

    let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");

    assert_eq!(report.status, StudioShellBundleStatus::Rejected);
    assert_eq!(
        report.issue_code.as_deref(),
        Some("studio.issue.no_operator_shell")
    );
    assert!(report.bundle_files.is_empty());
    assert_eq!(report.descriptor, None);
    assert_eq!(report.template_index, None);
}

#[test]
fn shell_templates_reject_invalid_artifact_manifest() {
    let root = temp_root("shell-template-invalid-manifest");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let report = shell_artifacts_for_project(&project, Some(&root));
    let mut manifest = report.manifest.expect("shell artifact manifest");
    manifest.artifacts[0].descriptor_path = "../outside.json".to_string();

    let template_report = shell_templates_for_artifact_manifest(&manifest, Some(&root));

    assert_eq!(template_report.status, StudioShellTemplateStatus::Rejected);
    assert_eq!(template_report.index, None);
    assert!(template_report.templates.is_empty());
    assert_eq!(
        template_report.issue_code.as_deref(),
        Some("studio.issue.invalid_descriptor_path")
    );
}

#[test]
fn shell_template_index_roundtrips_and_validates_files() {
    let root = temp_root("shell-template-index-roundtrip");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let artifact_report = shell_artifacts_for_project(&project, Some(&root));
    let manifest = artifact_report
        .manifest
        .as_ref()
        .expect("shell artifact manifest");
    for descriptor in &artifact_report.descriptors {
        let descriptor_path = resolve_manifest_relative_path(
            &root,
            &shell_descriptor_artifact_path(&descriptor.graph_id),
        );
        save_json(&descriptor_path, descriptor).expect("save descriptor");
    }
    let template_report = shell_templates_for_artifact_manifest(manifest, Some(&root));
    let index = template_report.index.as_ref().expect("template index");
    for (entry, template) in index.templates.iter().zip(template_report.templates.iter()) {
        save_json(
            &resolve_manifest_relative_path(&root, &entry.template_path),
            template,
        )
        .expect("save template");
    }
    let index_path = root.join("shell-templates.json");
    save_json(&index_path, index).expect("save index");

    let loaded_index = load_shell_template_index(&index_path).expect("load template index");
    let validation = validate_shell_template_index(&loaded_index, Some(&root));

    assert_eq!(loaded_index, *index);
    assert_eq!(
        validation.schema_id,
        SHELL_TEMPLATE_INDEX_VALIDATION_REPORT_SCHEMA
    );
    assert_eq!(validation.status, StudioValidationStatus::Pass);
}

#[test]
fn shell_template_index_validation_rejects_template_mismatch() {
    let root = temp_root("shell-template-index-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let artifact_report = shell_artifacts_for_project(&project, Some(&root));
    let manifest = artifact_report
        .manifest
        .as_ref()
        .expect("shell artifact manifest");
    for descriptor in &artifact_report.descriptors {
        let descriptor_path = resolve_manifest_relative_path(
            &root,
            &shell_descriptor_artifact_path(&descriptor.graph_id),
        );
        save_json(&descriptor_path, descriptor).expect("save descriptor");
    }
    let template_report = shell_templates_for_artifact_manifest(manifest, Some(&root));
    let mut index = template_report.index.expect("template index");
    for (entry, template) in index.templates.iter().zip(template_report.templates.iter()) {
        save_json(
            &resolve_manifest_relative_path(&root, &entry.template_path),
            template,
        )
        .expect("save template");
    }
    index.templates[0].shell_id = "shell.synthetic.changed".to_string();

    let validation = validate_shell_template_index(&index, Some(&root));

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.template_shell_mismatch")
    }));
}

#[test]
fn shell_template_index_validation_rejects_path_traversal() {
    let root = temp_root("shell-template-index-path-traversal");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let artifact_report = shell_artifacts_for_project(&project, Some(&root));
    let manifest = artifact_report
        .manifest
        .as_ref()
        .expect("shell artifact manifest");
    for descriptor in &artifact_report.descriptors {
        let descriptor_path = resolve_manifest_relative_path(
            &root,
            &shell_descriptor_artifact_path(&descriptor.graph_id),
        );
        save_json(&descriptor_path, descriptor).expect("save descriptor");
    }
    let template_report = shell_templates_for_artifact_manifest(manifest, Some(&root));
    let mut index = template_report.index.expect("template index");
    index.templates[0].template_path = "../outside.json".to_string();

    let validation = validate_shell_template_index(&index, Some(&root));

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.invalid_template_path")
    }));
}

#[test]
fn shell_artifacts_reject_invalid_graph_descriptor() {
    let root = temp_root("shell-artifacts-reject");
    write_reference_fixture_tree(&root);
    let mut project = valid_multi_shell_project_with_relative_references();
    for node in &mut project.graphs[1].nodes {
        if node.kind == StudioNodeKind::OperatorShell {
            node.kind = StudioNodeKind::ValidationSlot;
        }
    }
    project.graphs[1]
        .edges
        .retain(|edge| edge.kind != StudioEdgeKind::CommandBinding);

    let report = shell_artifacts_for_project(&project, Some(&root));

    assert_eq!(report.status, StudioShellArtifactStatus::Rejected);
    assert_eq!(report.manifest, None);
    assert!(report.descriptors.is_empty());
    assert!(report.rejections.iter().any(|rejection| {
        rejection.graph_id == "studio.graph.phone"
            && rejection.issue_code.as_deref() == Some("studio.issue.no_operator_shell")
    }));
}

#[test]
fn shell_descriptor_rejects_missing_graph() {
    let root = temp_root("shell-descriptor-missing-graph");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();

    let report = shell_descriptor_for_graph(&project, Some(&root), "studio.graph.missing");

    assert_eq!(report.status, StudioShellDescriptorStatus::Rejected);
    assert_eq!(
        report.issue_code.as_deref(),
        Some("studio.issue.graph_missing")
    );
    assert_eq!(report.descriptor, None);
}

#[test]
fn shell_descriptor_rejects_missing_operator_shell() {
    let root = temp_root("shell-descriptor-no-shell");
    write_reference_fixture_tree(&root);
    let project = valid_project_with_relative_references();

    let report = shell_descriptor_for_graph(&project, Some(&root), "studio.graph.test");

    assert_eq!(report.status, StudioShellDescriptorStatus::Rejected);
    assert_eq!(
        report.issue_code.as_deref(),
        Some("studio.issue.no_operator_shell")
    );
    assert_eq!(report.descriptor, None);
}
