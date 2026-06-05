use super::*;

mod projected_motion_breath_tests;

mod shell_hostess_workflow_tests;
use rusty_studio_model::{
    StudioBindingKind, StudioEdgeKind, StudioEdgeLayout, StudioEdgeRouteKind, StudioEditOperation,
    StudioEditStatus, StudioGraphLayout, StudioNode, StudioNodeKind, StudioNodeLayout,
    StudioShellArtifactStatus, StudioShellBundleStatus, StudioShellDescriptorStatus,
    StudioShellHandoffAcceptanceComparisonChange, StudioShellHandoffAcceptanceComparisonStatus,
    StudioShellHandoffAcceptanceStatus, StudioShellHandoffIntakeDecision,
    StudioShellHandoffIntakeStatus, StudioShellHandoffKind,
    StudioShellReleaseCandidateReviewStatus, StudioShellTargetKind, StudioShellTemplateStatus,
    SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA, SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA,
    SHELL_HANDOFF_ACCEPTANCE_COMPARISON_SCHEMA, SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA,
    SHELL_HANDOFF_INTAKE_REPORT_SCHEMA, SHELL_HANDOFF_MANIFEST_SCHEMA,
    SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA, SHELL_HANDOFF_READINESS_REPORT_SCHEMA,
    SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA, SHELL_TEMPLATE_INDEX_VALIDATION_REPORT_SCHEMA,
};

fn valid_project() -> StudioProject {
    StudioProject {
        schema_id: PROJECT_SCHEMA.to_string(),
        project_id: "studio.project.test".to_string(),
        revision: 1,
        display_name: "Test".to_string(),
        package_catalog_path: "packages/catalog.manifold.json".to_string(),
        host_run_profile_paths: vec!["fixtures/host-run/install-profile-desktop.json".to_string()],
        graphs: vec![StudioGraph {
            graph_id: "studio.graph.test".to_string(),
            display_name: "Graph".to_string(),
            target_host_profile: "host_run.profile.desktop".to_string(),
            nodes: vec![
                StudioNode {
                    node_id: "node.package.synthetic".to_string(),
                    kind: StudioNodeKind::Package,
                    reference_id: "package.synthetic".to_string(),
                    label: "Package".to_string(),
                },
                StudioNode {
                    node_id: "node.host.desktop".to_string(),
                    kind: StudioNodeKind::HostProfile,
                    reference_id: "host_run.profile.desktop".to_string(),
                    label: "Desktop".to_string(),
                },
            ],
            edges: vec![StudioEdge {
                edge_id: "edge.package_host".to_string(),
                kind: StudioEdgeKind::ShellTargetsHostProfile,
                source_node_id: "node.package.synthetic".to_string(),
                target_node_id: "node.host.desktop".to_string(),
            }],
            layout: Some(StudioGraphLayout {
                layout_id: "studio.layout.test".to_string(),
                coordinate_space: "studio.canvas.logical_2d".to_string(),
                nodes: vec![
                    StudioNodeLayout {
                        node_id: "node.package.synthetic".to_string(),
                        x: 40,
                        y: 40,
                        width: 180,
                        height: 72,
                    },
                    StudioNodeLayout {
                        node_id: "node.host.desktop".to_string(),
                        x: 340,
                        y: 40,
                        width: 180,
                        height: 72,
                    },
                ],
                edges: vec![StudioEdgeLayout {
                    edge_id: "edge.package_host".to_string(),
                    route: StudioEdgeRouteKind::Direct,
                }],
            }),
        }],
    }
}

fn temp_root(name: &str) -> std::path::PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("rusty-studio-{name}-{unique}"));
    std::fs::create_dir_all(&root).expect("create temp root");
    root
}

fn write_fixture(path: &Path, text: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("create fixture parent");
    }
    std::fs::write(path, text).expect("write fixture");
}

fn write_reference_fixture_tree(root: &Path) {
    write_fixture(
        &root.join("packages/catalog.manifold.json"),
        r#"{
  "$schema": "rusty.manifold.package.catalog.v1",
  "catalog_id": "catalog.test",
  "packages": [
{
  "package_id": "package.synthetic",
  "manifest_path": "packages/synthetic/manifests/package.manifold.json"
}
  ]
}"#,
    );
    write_fixture(
        &root.join("packages/synthetic/manifests/package.manifold.json"),
        r#"{
  "$schema": "rusty.manifold.package.manifest.v1",
  "package_id": "package.synthetic",
  "version": "0.1.0",
  "exports": {
"modules": [
  "module.synthetic_provider"
],
"streams": [],
"commands": []
  }
}"#,
    );
    write_fixture(
        &root.join("profiles/desktop.json"),
        r#"{
  "$schema": "rusty.manifold.host_run.install_launch_profile.v1",
  "profile_id": "host_run.profile.desktop",
  "host_profile": "host.desktop",
  "app_id": "app.host_shell.desktop",
  "install_route": "install.local_process",
  "launch_route": "launch.local_process",
  "command_bridge": "bridge.local_cli",
  "required_permissions": [],
  "evidence_pull_route": "evidence.filesystem"
}"#,
    );
    write_fixture(
        &root.join("profiles/headset.json"),
        r#"{
  "$schema": "rusty.manifold.host_run.install_launch_profile.v1",
  "profile_id": "host_run.profile.headset",
  "host_profile": "host.headset",
  "app_id": "app.host_shell.headset",
  "install_route": "install.android_package",
  "launch_route": "launch.android_intent",
  "command_bridge": "bridge.adb_intent_file",
  "required_permissions": [
"permission.bluetooth.scan",
"permission.bluetooth.connect",
"permission.location.fine"
  ],
  "evidence_pull_route": "evidence.adb_pull"
}"#,
    );
    write_fixture(
        &root.join("profiles/mobile.json"),
        r#"{
  "$schema": "rusty.manifold.host_run.install_launch_profile.v1",
  "profile_id": "host_run.profile.mobile",
  "host_profile": "host.mobile",
  "app_id": "app.host_shell.mobile",
  "install_route": "install.android_package",
  "launch_route": "launch.android_intent",
  "command_bridge": "bridge.adb_intent_file",
  "required_permissions": [],
  "evidence_pull_route": "evidence.adb_pull"
}"#,
    );
}

fn write_multi_package_reference_fixture_tree(root: &Path) {
    write_reference_fixture_tree(root);
    write_fixture(
        &root.join("packages/catalog.manifold.json"),
        r#"{
  "$schema": "rusty.manifold.package.catalog.v1",
  "catalog_id": "catalog.test",
  "packages": [
{
  "package_id": "package.synthetic",
  "manifest_path": "packages/synthetic/manifests/package.manifold.json"
},
{
  "package_id": "package.biosignal",
  "manifest_path": "packages/biosignal/manifests/package.manifold.json"
}
  ]
}"#,
    );
    write_fixture(
        &root.join("packages/biosignal/manifests/package.manifold.json"),
        r#"{
  "$schema": "rusty.manifold.package.manifest.v1",
  "package_id": "package.biosignal",
  "version": "0.1.0",
  "exports": {
"modules": [
  "module.biosignal.processor",
  "module.biosignal.provider"
],
"streams": [],
"commands": []
  }
}"#,
    );
}

fn valid_project_with_relative_references() -> StudioProject {
    let mut project = valid_project();
    project.package_catalog_path = "packages/catalog.manifold.json".to_string();
    project.host_run_profile_paths = vec![
        "profiles/desktop.json".to_string(),
        "profiles/headset.json".to_string(),
    ];
    project
}

fn valid_shell_project_with_relative_references() -> StudioProject {
    let mut project = valid_project_with_relative_references();
    project.graphs[0].nodes.push(StudioNode {
        node_id: "node.module.synthetic_provider".to_string(),
        kind: StudioNodeKind::Module,
        reference_id: "module.synthetic_provider".to_string(),
        label: "Synthetic Provider".to_string(),
    });
    project.graphs[0].nodes.push(StudioNode {
        node_id: "node.shell.operator".to_string(),
        kind: StudioNodeKind::OperatorShell,
        reference_id: "shell.synthetic.operator".to_string(),
        label: "Operator Shell".to_string(),
    });
    project.graphs[0].edges.push(StudioEdge {
        edge_id: "edge.package_module".to_string(),
        kind: StudioEdgeKind::PackageProvidesModule,
        source_node_id: "node.package.synthetic".to_string(),
        target_node_id: "node.module.synthetic_provider".to_string(),
    });
    project.graphs[0].edges.push(StudioEdge {
        edge_id: "edge.shell_command".to_string(),
        kind: StudioEdgeKind::CommandBinding,
        source_node_id: "node.shell.operator".to_string(),
        target_node_id: "node.module.synthetic_provider".to_string(),
    });
    project
}

fn valid_multi_shell_project_with_relative_references() -> StudioProject {
    let mut project = valid_shell_project_with_relative_references();
    project.host_run_profile_paths = vec![
        "profiles/desktop.json".to_string(),
        "profiles/mobile.json".to_string(),
        "profiles/headset.json".to_string(),
    ];

    let mut phone_graph = project.graphs[0].clone();
    phone_graph.graph_id = "studio.graph.phone".to_string();
    phone_graph.display_name = "Phone Graph".to_string();
    phone_graph.target_host_profile = "host_run.profile.mobile".to_string();
    for node in &mut phone_graph.nodes {
        if node.kind == StudioNodeKind::HostProfile {
            node.node_id = "node.host.mobile".to_string();
            node.reference_id = "host_run.profile.mobile".to_string();
            node.label = "Phone".to_string();
        }
        if node.kind == StudioNodeKind::OperatorShell {
            node.reference_id = "shell.synthetic.phone_operator".to_string();
            node.label = "Phone Shell".to_string();
        }
    }
    for edge in &mut phone_graph.edges {
        if edge.kind == StudioEdgeKind::ShellTargetsHostProfile {
            edge.target_node_id = "node.host.mobile".to_string();
        }
    }
    if let Some(layout) = phone_graph.layout.as_mut() {
        layout.layout_id = "studio.layout.phone".to_string();
        for node in &mut layout.nodes {
            if node.node_id == "node.host.desktop" {
                node.node_id = "node.host.mobile".to_string();
            }
        }
    }

    let mut quest_graph = project.graphs[0].clone();
    quest_graph.graph_id = "studio.graph.quest".to_string();
    quest_graph.display_name = "Quest Graph".to_string();
    quest_graph.target_host_profile = "host_run.profile.headset".to_string();
    for node in &mut quest_graph.nodes {
        if node.kind == StudioNodeKind::HostProfile {
            node.node_id = "node.host.headset".to_string();
            node.reference_id = "host_run.profile.headset".to_string();
            node.label = "Quest".to_string();
        }
        if node.kind == StudioNodeKind::OperatorShell {
            node.reference_id = "shell.synthetic.quest_operator".to_string();
            node.label = "Quest Shell".to_string();
        }
    }
    for edge in &mut quest_graph.edges {
        if edge.kind == StudioEdgeKind::ShellTargetsHostProfile {
            edge.target_node_id = "node.host.headset".to_string();
        }
    }
    if let Some(layout) = quest_graph.layout.as_mut() {
        layout.layout_id = "studio.layout.quest".to_string();
        for node in &mut layout.nodes {
            if node.node_id == "node.host.desktop" {
                node.node_id = "node.host.headset".to_string();
            }
        }
    }

    project.graphs.push(phone_graph);
    project.graphs.push(quest_graph);
    project
}

fn save_selected_shell_bundles(project: &StudioProject, base_dir: &Path, bundle_root: &Path) {
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(project, Some(base_dir), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
}

#[test]
fn valid_project_passes() {
    let report = validate_project(&valid_project());
    assert_eq!(report.status, StudioValidationStatus::Pass);
}

#[test]
fn save_project_roundtrips_authored_project_json() {
    let root = temp_root("save-project");
    let path = root.join("nested/project.json");
    let project = valid_project();

    save_project(&path, &project).expect("save project");
    let loaded = load_project(&path).expect("load saved project");

    assert_eq!(loaded, project);
}

#[test]
fn duplicate_node_fails() {
    let mut project = valid_project();
    let duplicate = project.graphs[0].nodes[0].clone();
    project.graphs[0].nodes.push(duplicate);
    let report = validate_project(&project);
    assert_eq!(report.status, StudioValidationStatus::Fail);
    let issue = report
        .checks
        .iter()
        .find(|check| check.issue_code.as_deref() == Some("studio.issue.duplicate_node_id"))
        .expect("duplicate node issue");
    assert_eq!(issue.graph_id.as_deref(), Some("studio.graph.test"));
    assert_eq!(issue.node_ids, vec!["node.package.synthetic".to_string()]);
}

#[test]
fn missing_edge_target_fails() {
    let mut project = valid_project();
    project.graphs[0].edges[0].target_node_id = "node.missing".to_string();
    let report = validate_project(&project);
    assert_eq!(report.status, StudioValidationStatus::Fail);
    let issue = report
        .checks
        .iter()
        .find(|check| check.issue_code.as_deref() == Some("studio.issue.missing_edge_target"))
        .expect("missing edge target issue");
    assert_eq!(issue.graph_id.as_deref(), Some("studio.graph.test"));
    assert_eq!(issue.node_ids, vec!["node.missing".to_string()]);
    assert_eq!(issue.edge_ids, vec!["edge.package_host".to_string()]);
}

#[test]
fn invalid_layout_references_fail() {
    let mut project = valid_project();
    let layout = project.graphs[0].layout.as_mut().expect("layout");
    layout.nodes[0].node_id = "node.missing".to_string();
    layout.edges[0].edge_id = "edge.missing".to_string();
    layout.nodes[1].width = 0;

    let report = validate_project(&project);

    assert_eq!(report.status, StudioValidationStatus::Fail);
    assert!(report.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.layout_node_missing")
            && check.node_ids == vec!["node.missing".to_string()]
    }));
    assert!(report.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.layout_edge_missing")
            && check.edge_ids == vec!["edge.missing".to_string()]
    }));
    assert!(report.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.invalid_layout_node_box")
            && check.node_ids == vec!["node.host.desktop".to_string()]
    }));
}

#[test]
fn missing_target_host_profile_fails() {
    let mut project = valid_project();
    project.graphs[0].target_host_profile = "host_run.profile.headset".to_string();
    let report = validate_project(&project);
    assert_eq!(report.status, StudioValidationStatus::Fail);
    let issue = report
        .checks
        .iter()
        .find(|check| {
            check.issue_code.as_deref() == Some("studio.issue.missing_target_host_profile")
        })
        .expect("missing target host issue");
    assert_eq!(issue.graph_id.as_deref(), Some("studio.graph.test"));
    assert_eq!(
        issue.reference_ids,
        vec!["host_run.profile.headset".to_string()]
    );
}

#[test]
fn missing_reference_paths_fail_when_base_dir_is_supplied() {
    let report = validate_project_with_base(&valid_project(), Some(Path::new("missing-base")));
    assert_eq!(report.status, StudioValidationStatus::Fail);
    assert!(report
        .checks
        .iter()
        .any(|check| check.issue_code.as_deref() == Some("studio.issue.package_catalog_missing")));
    assert!(report
        .checks
        .iter()
        .any(|check| check.issue_code.as_deref() == Some("studio.issue.host_run_profile_missing")));
}

#[test]
fn content_reference_resolution_accepts_catalog_manifest_and_profile() {
    let root = temp_root("content-pass");
    write_reference_fixture_tree(&root);
    let report = validate_project_with_base(&valid_project_with_relative_references(), Some(&root));
    assert_eq!(report.status, StudioValidationStatus::Pass);
}

#[test]
fn missing_package_catalog_reference_fails() {
    let root = temp_root("missing-package");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();
    project.graphs[0].nodes[0].reference_id = "package.missing".to_string();
    let report = validate_project_with_base(&project, Some(&root));
    assert_eq!(report.status, StudioValidationStatus::Fail);
    let issue = report
        .checks
        .iter()
        .find(|check| check.issue_code.as_deref() == Some("studio.issue.package_reference_missing"))
        .expect("package reference issue");
    assert_eq!(issue.graph_id.as_deref(), Some("studio.graph.test"));
    assert_eq!(issue.node_ids, vec!["node.package.synthetic".to_string()]);
    assert_eq!(issue.reference_ids, vec!["package.missing".to_string()]);
}

#[test]
fn missing_module_export_reference_fails() {
    let root = temp_root("missing-module");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();
    project.graphs[0].nodes.push(StudioNode {
        node_id: "node.module.missing".to_string(),
        kind: StudioNodeKind::Module,
        reference_id: "module.missing".to_string(),
        label: "Missing Module".to_string(),
    });
    let report = validate_project_with_base(&project, Some(&root));
    assert_eq!(report.status, StudioValidationStatus::Fail);
    assert!(report.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.module_reference_missing")
    }));
}

#[test]
fn resolve_counts_graph_parts() {
    let resolved = resolve_project(&valid_project());
    let graph = &resolved.graphs[0];
    assert_eq!(graph.package_count, 1);
    assert_eq!(graph.module_count, 0);
    assert_eq!(graph.edge_count, 1);
}

#[test]
fn export_plan_collects_bundle_refs() {
    let plan = export_plan(&valid_project());
    assert_eq!(plan.bundles[0].package_ids, vec!["package.synthetic"]);
    assert_eq!(
        plan.bundles[0].target_host_profile,
        "host_run.profile.desktop"
    );
}

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
fn desktop_shell_handoff_reports_validated_schema_file_entrypoint() {
    let root = temp_root("desktop-shell-handoff");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();
    let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");
    let output_dir = root.join("selected-shell");
    save_shell_bundle(&output_dir, &report).expect("save shell bundle");

    let handoff =
        desktop_shell_handoff_for_bundle(&project, Some(&root), "studio.graph.test", &output_dir);

    assert_eq!(handoff.status, StudioValidationStatus::Pass);
    assert_eq!(handoff.handoff_kind, StudioShellHandoffKind::DesktopShell);
    assert_eq!(handoff.consumer_id, "rusty-studio-desktop-shell");
    assert_eq!(handoff.target_kind, StudioShellTargetKind::Desktop);
    assert_eq!(handoff.validation.status, StudioValidationStatus::Pass);
    assert!(
        handoff
            .descriptor_path
            .ends_with("descriptors\\studio.graph.test.shell-descriptor.json")
            || handoff
                .descriptor_path
                .ends_with("descriptors/studio.graph.test.shell-descriptor.json")
    );
    assert!(handoff
        .template_index_path
        .ends_with("shell-templates.json"));
    assert!(
        handoff
            .template_manifest_path
            .ends_with("shells\\desktop\\studio.graph.test.shell-template.json")
            || handoff
                .template_manifest_path
                .ends_with("shells/desktop/studio.graph.test.shell-template.json")
    );
    assert_eq!(
        handoff.consumer_args,
        vec![
            "--templates".to_string(),
            output_dir
                .join("shell-templates.json")
                .display()
                .to_string(),
        ]
    );
    let authority = handoff
        .runtime_authority
        .expect("handoff carries runtime authority summary");
    assert_eq!(authority.command_session_authority, "rusty.manifold");
    assert_eq!(authority.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(authority.studio_role, "authoring.export_planning");
}

#[test]
fn shell_handoff_reports_phone_and_quest_targets() {
    let root = temp_root("multi-target-shell-handoff");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    for (graph_id, expected_kind, expected_handoff, expected_consumer) in [
        (
            "studio.graph.phone",
            StudioShellTargetKind::Phone,
            StudioShellHandoffKind::PhoneShell,
            "rusty-studio-phone-shell",
        ),
        (
            "studio.graph.quest",
            StudioShellTargetKind::Quest,
            StudioShellHandoffKind::QuestShell,
            "rusty-studio-quest-shell",
        ),
    ] {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), graph_id);
        let output_dir = root.join(graph_id);
        save_shell_bundle(&output_dir, &report).expect("save shell bundle");

        let handoff = shell_handoff_for_bundle(&project, Some(&root), graph_id, &output_dir);

        assert_eq!(handoff.status, StudioValidationStatus::Pass);
        assert_eq!(handoff.handoff_kind, expected_handoff);
        assert_eq!(handoff.consumer_id, expected_consumer);
        assert_eq!(handoff.target_kind, expected_kind);
        assert_eq!(handoff.validation.status, StudioValidationStatus::Pass);
        assert_eq!(
            handoff.consumer_args,
            vec![
                "--templates".to_string(),
                output_dir
                    .join("shell-templates.json")
                    .display()
                    .to_string(),
            ]
        );
        assert_eq!(
            handoff
                .runtime_authority
                .as_ref()
                .expect("runtime authority")
                .install_launch_evidence_authority,
            "rusty.hostess"
        );
    }
}

#[test]
fn desktop_shell_handoff_rejects_non_desktop_bundle() {
    let root = temp_root("desktop-shell-handoff-target-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.phone");
    let output_dir = root.join("phone-selected-shell");
    save_shell_bundle(&output_dir, &report).expect("save phone shell bundle");

    let handoff =
        desktop_shell_handoff_for_bundle(&project, Some(&root), "studio.graph.phone", &output_dir);

    assert_eq!(handoff.status, StudioValidationStatus::Fail);
    assert_eq!(
        handoff.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_target_mismatch")
    );
    assert_eq!(handoff.handoff_kind, StudioShellHandoffKind::PhoneShell);
    assert_eq!(handoff.target_kind, StudioShellTargetKind::Phone);
    assert!(handoff.consumer_args.is_empty());
    assert_eq!(handoff.validation.status, StudioValidationStatus::Pass);
}

#[test]
fn shell_handoff_readiness_reports_all_target_graphs() {
    let root = temp_root("shell-handoff-readiness");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }

    let readiness = shell_handoff_readiness_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(readiness.schema_id, SHELL_HANDOFF_READINESS_REPORT_SCHEMA);
    assert_eq!(readiness.status, StudioValidationStatus::Pass);
    assert_eq!(readiness.graph_count, 3);
    assert_eq!(readiness.ready_count, 3);
    assert_eq!(readiness.failed_count, 0);
    assert_eq!(readiness.missing_bundle_count, 0);
    assert_eq!(readiness.entries.len(), 3);
    assert_eq!(readiness.target_summaries.len(), 3);
    for target_kind in [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
    ] {
        let summary = readiness
            .target_summaries
            .iter()
            .find(|summary| summary.target_kind == target_kind)
            .expect("target summary");
        assert_eq!(summary.graph_count, 1);
        assert_eq!(summary.ready_count, 1);
        assert_eq!(summary.failed_count, 0);
        assert_eq!(summary.missing_bundle_count, 0);
        assert_eq!(summary.package_count, 1);
        assert_eq!(summary.module_count, 1);
        assert_eq!(summary.operator_shell_count, 1);
        assert!(summary.issue_codes.is_empty());
        assert_eq!(summary.graph_ids.len(), 1);
        assert_eq!(summary.consumer_ids.len(), 1);
        assert_eq!(summary.bundle_dirs.len(), 1);
        assert_eq!(summary.ready_bundle_dirs.len(), 1);
        assert!(summary.failed_bundle_dirs.is_empty());
        assert!(summary.missing_bundle_dirs.is_empty());
        assert_eq!(summary.template_index_paths.len(), 1);
        assert!(summary.template_index_paths[0].ends_with("shell-templates.json"));
    }
    assert!(readiness.entries.iter().all(|entry| {
        entry.status == StudioValidationStatus::Pass
            && entry.validation_status == StudioValidationStatus::Pass
            && entry.failed_check_count == 0
            && entry.consumer_args.iter().any(|arg| arg == "--templates")
            && entry.export_bundle_id == format!("studio.export.{}", entry.graph_id)
            && entry.package_ids == vec!["package.synthetic".to_string()]
            && entry.module_ids == vec!["module.synthetic_provider".to_string()]
            && entry.package_count == entry.package_ids.len()
            && entry.module_count == entry.module_ids.len()
            && entry.operator_shell_count == entry.operator_shell_ids.len()
    }));
    assert!(readiness.entries.iter().any(|entry| {
        entry.graph_id == "studio.graph.phone"
            && entry.target_host_profile == "host_run.profile.mobile"
            && entry.handoff_kind == StudioShellHandoffKind::PhoneShell
            && entry.consumer_id == "rusty-studio-phone-shell"
            && entry.target_kind == StudioShellTargetKind::Phone
            && entry.operator_shell_ids == vec!["shell.synthetic.phone_operator".to_string()]
    }));
    assert!(readiness.entries.iter().any(|entry| {
        entry.graph_id == "studio.graph.quest"
            && entry.target_host_profile == "host_run.profile.headset"
            && entry.handoff_kind == StudioShellHandoffKind::QuestShell
            && entry.consumer_id == "rusty-studio-quest-shell"
            && entry.target_kind == StudioShellTargetKind::Quest
            && entry.operator_shell_ids == vec!["shell.synthetic.quest_operator".to_string()]
    }));
}

#[test]
fn shell_handoff_readiness_reports_missing_bundles() {
    let root = temp_root("shell-handoff-readiness-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");

    let readiness = shell_handoff_readiness_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(readiness.status, StudioValidationStatus::Fail);
    assert_eq!(readiness.graph_count, 3);
    assert_eq!(readiness.ready_count, 0);
    assert_eq!(readiness.failed_count, 3);
    assert_eq!(readiness.missing_bundle_count, 3);
    assert_eq!(readiness.entries.len(), 3);
    assert_eq!(readiness.target_summaries.len(), 3);
    assert!(readiness.target_summaries.iter().all(|summary| {
        summary.graph_count == 1
            && summary.ready_count == 0
            && summary.failed_count == 1
            && summary.missing_bundle_count == 1
            && summary
                .issue_codes
                .iter()
                .any(|issue| issue == "studio.issue.shell_bundle_file_missing")
            && summary.bundle_dirs.len() == 1
            && summary.ready_bundle_dirs.is_empty()
            && summary.failed_bundle_dirs.len() == 1
            && summary.missing_bundle_dirs.len() == 1
            && summary.template_index_paths.len() == 1
            && summary.template_index_paths[0].ends_with("shell-templates.json")
    }));
    assert!(readiness.entries.iter().all(|entry| {
        entry.status == StudioValidationStatus::Fail
            && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            && entry.export_bundle_id == format!("studio.export.{}", entry.graph_id)
            && entry.package_count == 1
            && entry.module_count == 1
            && entry.operator_shell_count == 1
            && entry.failed_check_count > 0
    }));
}

#[test]
fn shell_handoff_manifest_archives_readiness_paths() {
    let root = temp_root("shell-handoff-manifest");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }

    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(manifest.schema_id, SHELL_HANDOFF_MANIFEST_SCHEMA);
    assert_eq!(
        manifest.manifest_id,
        "studio.shell_handoffs.studio.project.test"
    );
    assert_eq!(
        manifest.source_readiness_schema,
        SHELL_HANDOFF_READINESS_REPORT_SCHEMA
    );
    assert_eq!(manifest.status, StudioValidationStatus::Pass);
    assert_eq!(manifest.graph_count, 3);
    assert_eq!(manifest.ready_count, 3);
    assert_eq!(manifest.failed_count, 0);
    assert_eq!(manifest.missing_bundle_count, 0);
    assert_eq!(manifest.targets.len(), 3);
    assert_eq!(manifest.handoffs.len(), 3);
    assert_eq!(
        manifest.runtime_authority.command_session_authority,
        "rusty.manifold"
    );
    assert_eq!(
        manifest.runtime_authority.install_launch_evidence_authority,
        "rusty.hostess"
    );
    assert_eq!(
        manifest.runtime_authority.studio_role,
        "authoring.export_planning"
    );
    for target_kind in [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
    ] {
        let target = manifest
            .targets
            .iter()
            .find(|target| target.target_kind == target_kind)
            .expect("target manifest row");
        assert_eq!(target.graph_count, 1);
        assert_eq!(target.ready_count, 1);
        assert_eq!(target.failed_count, 0);
        assert_eq!(target.missing_bundle_count, 0);
        assert_eq!(target.bundle_dirs.len(), 1);
        assert_eq!(target.ready_bundle_dirs.len(), 1);
        assert!(target.failed_bundle_dirs.is_empty());
        assert!(target.missing_bundle_dirs.is_empty());
        assert_eq!(target.template_index_paths.len(), 1);
        assert!(target.template_index_paths[0].ends_with("shell-templates.json"));
    }
    assert!(manifest.handoffs.iter().all(|handoff| {
        handoff.status == StudioValidationStatus::Pass
            && handoff.validation_status == StudioValidationStatus::Pass
            && handoff.failed_check_count == 0
            && handoff.consumer_args.iter().any(|arg| arg == "--templates")
            && handoff
                .template_index_path
                .ends_with("shell-templates.json")
            && handoff.runtime_authority.is_some()
    }));
}

#[test]
fn shell_handoff_manifest_archives_missing_bundle_paths() {
    let root = temp_root("shell-handoff-manifest-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");

    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(manifest.schema_id, SHELL_HANDOFF_MANIFEST_SCHEMA);
    assert_eq!(manifest.status, StudioValidationStatus::Fail);
    assert_eq!(manifest.graph_count, 3);
    assert_eq!(manifest.ready_count, 0);
    assert_eq!(manifest.failed_count, 3);
    assert_eq!(manifest.missing_bundle_count, 3);
    assert_eq!(manifest.targets.len(), 3);
    assert_eq!(manifest.handoffs.len(), 3);
    assert!(manifest.targets.iter().all(|target| {
        target.ready_bundle_dirs.is_empty()
            && target.failed_bundle_dirs.len() == 1
            && target.missing_bundle_dirs.len() == 1
            && target
                .issue_codes
                .iter()
                .any(|issue| issue == "studio.issue.shell_bundle_file_missing")
            && target.template_index_paths[0].ends_with("shell-templates.json")
    }));
    assert!(manifest.handoffs.iter().all(|handoff| {
        handoff.status == StudioValidationStatus::Fail
            && handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            && handoff.runtime_authority.is_none()
            && handoff.failed_check_count > 0
    }));
}

#[test]
fn shell_handoff_manifest_validation_accepts_archived_manifest() {
    let root = temp_root("shell-handoff-manifest-validation");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let manifest_path = root.join("shell-handoffs.json");
    save_json(&manifest_path, &manifest).expect("save shell handoff manifest");

    let loaded = load_shell_handoff_manifest(&manifest_path).expect("load handoff manifest");
    let validation = validate_shell_handoff_manifest(&loaded);

    assert_eq!(loaded, manifest);
    assert_eq!(
        validation.schema_id,
        SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA
    );
    assert_eq!(
        validation.manifest_id,
        "studio.shell_handoffs.studio.project.test"
    );
    assert_eq!(validation.status, StudioValidationStatus::Pass);
    assert!(validation
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn shell_handoff_manifest_validation_rejects_authority_mismatch() {
    let root = temp_root("shell-handoff-manifest-authority-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    manifest.runtime_authority.command_session_authority = "rusty.studio".to_string();

    let validation = validate_shell_handoff_manifest(&manifest);

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.runtime_authority_mismatch")
    }));
}

#[test]
fn shell_handoff_manifest_validation_rejects_target_count_mismatch() {
    let root = temp_root("shell-handoff-manifest-target-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    manifest.targets[0].ready_count = 0;

    let validation = validate_shell_handoff_manifest(&manifest);

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.target_summary_count_mismatch")
    }));
}

#[test]
fn shell_handoff_manifest_validation_rejects_missing_consumer_args() {
    let root = temp_root("shell-handoff-manifest-consumer-args");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    manifest.handoffs[0].consumer_args.clear();

    let validation = validate_shell_handoff_manifest(&manifest);

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.handoff_consumer_args_mismatch")
    }));
}

#[test]
fn shell_handoff_intake_accepts_valid_manifest() {
    let root = temp_root("shell-handoff-intake");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

    let intake = shell_handoff_intake_for_manifest(&manifest);

    assert_eq!(intake.schema_id, SHELL_HANDOFF_INTAKE_REPORT_SCHEMA);
    assert_eq!(intake.status, StudioShellHandoffIntakeStatus::Accepted);
    assert_eq!(intake.issue_code, None);
    assert_eq!(intake.validation.status, StudioValidationStatus::Pass);
    assert_eq!(intake.accepted_count, 3);
    assert_eq!(intake.blocked_count, 0);
    assert_eq!(intake.target_summaries.len(), 3);
    assert_eq!(intake.entries.len(), 3);
    assert_eq!(intake.command_session_authority, "rusty.manifold");
    assert_eq!(intake.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(intake.studio_role, "authoring.export_planning");
    for target_kind in [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
    ] {
        let summary = intake
            .target_summaries
            .iter()
            .find(|summary| summary.target_kind == target_kind)
            .expect("intake target summary");
        assert_eq!(summary.accepted_count, 1);
        assert_eq!(summary.blocked_count, 0);
        assert_eq!(summary.graph_ids.len(), 1);
        assert_eq!(summary.consumer_ids.len(), 1);
        assert_eq!(summary.bundle_dirs.len(), 1);
        assert_eq!(summary.template_index_paths.len(), 1);
    }
    assert!(intake.entries.iter().all(|entry| {
        entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner
            && entry.handoff_status == StudioValidationStatus::Pass
            && entry.issue_code.is_none()
            && entry.handoff_request_kind == "operator_shell_handoff"
            && entry.next_required_action == "stage_with_runtime_owner"
            && entry.command_session_authority == "rusty.manifold"
            && entry.install_launch_evidence_authority == "rusty.hostess"
            && entry.studio_role == "authoring.export_planning"
            && entry.consumer_args.iter().any(|arg| arg == "--templates")
    }));
}

#[test]
fn shell_runbook_reports_non_executed_owner_routes() {
    let root = temp_root("shell-runbook");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }

    let runbook = shell_runbook_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(runbook.schema_id, SHELL_RUNBOOK_REPORT_SCHEMA);
    assert_eq!(
        runbook.source_manifest_schema,
        SHELL_HANDOFF_MANIFEST_SCHEMA
    );
    assert_eq!(
        runbook.source_intake_schema,
        SHELL_HANDOFF_INTAKE_REPORT_SCHEMA
    );
    assert_eq!(runbook.status, StudioShellRunbookStatus::Ready);
    assert_eq!(runbook.issue_code, None);
    assert_eq!(runbook.ready_count, 3);
    assert_eq!(runbook.blocked_count, 0);
    assert_eq!(runbook.rejected_count, 0);
    assert_eq!(runbook.target_summaries.len(), 3);
    assert_eq!(
        runbook.prohibited_actions,
        vec![
            "install".to_string(),
            "launch".to_string(),
            "open_command_session".to_string(),
            "collect_device_evidence".to_string(),
        ]
    );
    for target_kind in [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
    ] {
        let summary = runbook
            .target_summaries
            .iter()
            .find(|summary| summary.target_kind == target_kind)
            .expect("runbook target summary");
        assert_eq!(summary.ready_count, 1);
        assert_eq!(summary.blocked_count, 0);
        assert_eq!(summary.rejected_count, 0);
        assert_eq!(summary.responsible_owners, vec!["rusty.hostess"]);
        assert!(summary.runtime_route_kinds[0].ends_with("_operator_shell"));
        assert!(summary.issue_codes.is_empty());
    }

    let desktop = runbook
        .entries
        .iter()
        .find(|entry| entry.target_kind == StudioShellTargetKind::Desktop)
        .expect("desktop runbook row");
    assert_eq!(desktop.status, StudioShellRunbookStatus::Ready);
    assert_eq!(desktop.responsible_owner, "rusty.hostess");
    assert_eq!(desktop.command_session_authority, "rusty.manifold");
    assert_eq!(desktop.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(desktop.execution_policy, "not_executed.request_only");
    assert_eq!(desktop.runtime_route_kind, "desktop_operator_shell");
    assert_eq!(desktop.next_required_action, "stage_with_runtime_owner");
    assert_eq!(
        desktop.host_routes.install_route.as_deref(),
        Some("install.local_process")
    );
    assert_eq!(
        desktop.host_routes.launch_route.as_deref(),
        Some("launch.local_process")
    );
    assert_eq!(
        desktop.host_routes.command_bridge.as_deref(),
        Some("bridge.local_cli")
    );
    assert_eq!(
        desktop.host_routes.evidence_pull_route.as_deref(),
        Some("evidence.filesystem")
    );
    assert_eq!(
        desktop.cli_request[..5],
        ["cargo", "run", "-p", "rusty-studio-desktop-shell", "--"]
    );
    assert!(desktop
        .cli_request
        .iter()
        .any(|arg| arg.ends_with("shell-templates.json")));

    let phone = runbook
        .entries
        .iter()
        .find(|entry| entry.target_kind == StudioShellTargetKind::Phone)
        .expect("phone runbook row");
    assert_eq!(phone.consumer_id, "rusty-studio-phone-shell");
    assert_eq!(
        phone.host_routes.install_route.as_deref(),
        Some("install.android_package")
    );
    assert_eq!(
        phone.host_routes.evidence_pull_route.as_deref(),
        Some("evidence.adb_pull")
    );

    let quest = runbook
        .entries
        .iter()
        .find(|entry| entry.target_kind == StudioShellTargetKind::Quest)
        .expect("quest runbook row");
    assert_eq!(quest.consumer_id, "rusty-studio-quest-shell");
    assert_eq!(
        quest.host_routes.install_route.as_deref(),
        Some("install.android_package")
    );
    assert_eq!(
        quest.host_routes.launch_route.as_deref(),
        Some("launch.android_intent")
    );
}

#[test]
fn shell_runbook_blocks_missing_bundle_requests_without_execution_args() {
    let root = temp_root("shell-runbook-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");

    let runbook = shell_runbook_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(runbook.status, StudioShellRunbookStatus::Blocked);
    assert_eq!(
        runbook.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(runbook.ready_count, 0);
    assert_eq!(runbook.blocked_count, 3);
    assert_eq!(runbook.rejected_count, 0);
    assert!(runbook.entries.iter().all(|entry| {
        entry.status == StudioShellRunbookStatus::Blocked
            && entry.responsible_owner == "rusty.studio"
            && entry.cli_request.is_empty()
            && entry.route_status == StudioValidationStatus::Fail
            && entry.execution_policy == "not_executed.request_only"
    }));
}

#[test]
fn shell_runbook_blocks_template_index_graph_mismatch() {
    let root = temp_root("shell-runbook-template-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let index_path = bundle_root
        .join("studio.graph.test")
        .join("shell-templates.json");
    let mut index = load_shell_template_index(&index_path).expect("load template index");
    index.templates[0].graph_id = "studio.graph.unrelated".to_string();
    save_json(&index_path, &index).expect("save mismatched template index");

    let runbook = shell_runbook_for_manifest(&manifest);

    assert_eq!(runbook.status, StudioShellRunbookStatus::Blocked);
    assert_eq!(runbook.ready_count, 2);
    assert_eq!(runbook.blocked_count, 1);
    let entry = runbook
        .entries
        .iter()
        .find(|entry| entry.graph_id == "studio.graph.test")
        .expect("mismatched runbook entry");
    assert_eq!(entry.status, StudioShellRunbookStatus::Blocked);
    assert_eq!(entry.responsible_owner, "rusty.studio");
    assert!(entry.cli_request.is_empty());
    assert_eq!(entry.route_status, StudioValidationStatus::Fail);
    assert_eq!(
        entry.route_issue_code.as_deref(),
        Some("studio.issue.shell_runbook_template_missing")
    );
    assert_eq!(
        entry.issue_code.as_deref(),
        Some("studio.issue.shell_runbook_template_missing")
    );
}

#[test]
fn shell_export_package_groups_descriptors_templates_and_runbook_rows() {
    let root = temp_root("shell-export-package");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }

    let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(package.schema_id, SHELL_EXPORT_PACKAGE_REPORT_SCHEMA);
    assert_eq!(
        package.source_manifest_schema,
        SHELL_HANDOFF_MANIFEST_SCHEMA
    );
    assert_eq!(package.source_runbook_schema, SHELL_RUNBOOK_REPORT_SCHEMA);
    assert_eq!(
        package.package_id,
        "studio.shell_export_package.studio.project.test"
    );
    assert_eq!(package.status, StudioShellExportPackageStatus::Ready);
    assert_eq!(package.issue_code, None);
    assert_eq!(package.execution_policy, "not_executed.review_only");
    assert_eq!(package.review_owner, "rusty.hostess");
    assert_eq!(package.command_session_authority, "rusty.manifold");
    assert_eq!(package.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(package.studio_role, "authoring.export_planning");
    assert_eq!(package.ready_count, 3);
    assert_eq!(package.blocked_count, 0);
    assert_eq!(package.rejected_count, 0);
    assert_eq!(package.descriptor_count, 3);
    assert_eq!(package.template_manifest_count, 3);
    assert_eq!(package.runbook_entry_count, 3);
    assert_eq!(package.target_summaries.len(), 3);
    assert_eq!(
        package.prohibited_actions,
        vec![
            "install".to_string(),
            "launch".to_string(),
            "open_command_session".to_string(),
            "collect_device_evidence".to_string(),
        ]
    );
    for target_kind in [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
    ] {
        let summary = package
            .target_summaries
            .iter()
            .find(|summary| summary.target_kind == target_kind)
            .expect("export package target summary");
        assert_eq!(summary.ready_count, 1);
        assert_eq!(summary.blocked_count, 0);
        assert_eq!(summary.rejected_count, 0);
        assert_eq!(summary.descriptor_count, 1);
        assert_eq!(summary.template_manifest_count, 1);
        assert_eq!(summary.responsible_owners, vec!["rusty.hostess"]);
        assert!(summary.issue_codes.is_empty());
    }

    let desktop = package
        .entries
        .iter()
        .find(|entry| entry.target_kind == StudioShellTargetKind::Desktop)
        .expect("desktop export package row");
    assert_eq!(desktop.status, StudioShellExportPackageStatus::Ready);
    assert_eq!(desktop.responsible_owner, "rusty.hostess");
    assert_eq!(desktop.execution_policy, "not_executed.review_only");
    assert_eq!(desktop.next_required_action, "review_with_runtime_owner");
    assert_eq!(desktop.runtime_route_kind, "desktop_operator_shell");
    assert_eq!(
        desktop.runbook_cli_request[..5],
        ["cargo", "run", "-p", "rusty-studio-desktop-shell", "--"]
    );
    let descriptor = desktop.descriptor.as_ref().expect("desktop descriptor ref");
    assert_eq!(
        descriptor.descriptor_id,
        "studio.shell_descriptor.studio.graph.test"
    );
    assert_eq!(descriptor.graph_id, desktop.graph_id);
    assert_eq!(descriptor.package_count, 1);
    assert_eq!(descriptor.module_count, 1);
    assert_eq!(descriptor.command_binding_count, 1);
    let template = desktop
        .template_manifest
        .as_ref()
        .expect("desktop template ref");
    assert_eq!(
        template.template_id,
        "studio.shell_template.studio.graph.test"
    );
    assert_eq!(template.graph_id, desktop.graph_id);
    assert_eq!(
        template.host_routes.install_route.as_deref(),
        Some("install.local_process")
    );
    assert_eq!(
        template.runtime_authority.command_session_authority,
        "rusty.manifold"
    );

    let quest = package
        .entries
        .iter()
        .find(|entry| entry.target_kind == StudioShellTargetKind::Quest)
        .expect("quest export package row");
    let quest_template = quest
        .template_manifest
        .as_ref()
        .expect("quest template ref");
    assert_eq!(quest.consumer_id, "rusty-studio-quest-shell");
    assert_eq!(
        quest_template.host_routes.command_bridge.as_deref(),
        Some("bridge.adb_intent_file")
    );
    assert_eq!(
        quest.host_routes.evidence_pull_route.as_deref(),
        Some("evidence.adb_pull")
    );
}

#[test]
fn shell_export_package_blocks_missing_bundle_without_descriptors() {
    let root = temp_root("shell-export-package-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");

    let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(package.status, StudioShellExportPackageStatus::Blocked);
    assert_eq!(
        package.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(package.ready_count, 0);
    assert_eq!(package.blocked_count, 3);
    assert_eq!(package.rejected_count, 0);
    assert_eq!(package.descriptor_count, 0);
    assert_eq!(package.template_manifest_count, 0);
    assert_eq!(package.runbook_entry_count, 3);
    assert!(package.entries.iter().all(|entry| {
        entry.status == StudioShellExportPackageStatus::Blocked
            && entry.responsible_owner == "rusty.studio"
            && entry.descriptor.is_none()
            && entry.template_manifest.is_none()
            && entry.runbook_cli_request.is_empty()
            && entry.execution_policy == "not_executed.review_only"
    }));
}

#[test]
fn shell_export_package_blocks_one_damaged_descriptor_from_valid_manifest() {
    let root = temp_root("shell-export-package-damaged-descriptor");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let runbook = shell_runbook_for_manifest(&manifest);
    assert_eq!(runbook.status, StudioShellRunbookStatus::Ready);

    std::fs::remove_file(
        bundle_root
            .join("studio.graph.phone")
            .join("descriptors/studio.graph.phone.shell-descriptor.json"),
    )
    .expect("remove phone descriptor");

    let package = shell_export_package_for_manifest(&manifest);

    assert_eq!(package.status, StudioShellExportPackageStatus::Blocked);
    assert_eq!(
        package.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_descriptor_load_failed")
    );
    assert_eq!(package.ready_count, 2);
    assert_eq!(package.blocked_count, 1);
    assert_eq!(package.rejected_count, 0);
    assert_eq!(package.descriptor_count, 2);
    assert_eq!(package.template_manifest_count, 3);
    assert_eq!(package.runbook_entry_count, 3);

    let phone = package
        .entries
        .iter()
        .find(|entry| entry.graph_id == "studio.graph.phone")
        .expect("phone export package row");
    assert_eq!(phone.status, StudioShellExportPackageStatus::Blocked);
    assert_eq!(phone.responsible_owner, "rusty.studio");
    assert_eq!(
        phone.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_descriptor_load_failed")
    );
    assert!(phone.descriptor.is_none());
    assert!(phone.template_manifest.is_some());
    assert!(phone.runbook_cli_request.is_empty());
    assert_eq!(
        phone.host_routes.command_bridge.as_deref(),
        Some("bridge.adb_intent_file")
    );

    for target_kind in [StudioShellTargetKind::Desktop, StudioShellTargetKind::Quest] {
        let entry = package
            .entries
            .iter()
            .find(|entry| entry.target_kind == target_kind)
            .expect("undamaged export package row");
        assert_eq!(entry.status, StudioShellExportPackageStatus::Ready);
        assert_eq!(entry.responsible_owner, "rusty.hostess");
        assert!(entry.descriptor.is_some());
        assert!(entry.template_manifest.is_some());
        assert!(!entry.runbook_cli_request.is_empty());
    }

    let phone_summary = package
        .target_summaries
        .iter()
        .find(|summary| summary.target_kind == StudioShellTargetKind::Phone)
        .expect("phone summary");
    assert_eq!(phone_summary.ready_count, 0);
    assert_eq!(phone_summary.blocked_count, 1);
    assert_eq!(phone_summary.descriptor_count, 0);
    assert_eq!(phone_summary.template_manifest_count, 1);
    assert_eq!(
        phone_summary.issue_codes,
        vec!["studio.issue.shell_export_package_descriptor_load_failed"]
    );
}

#[test]
fn shell_export_package_blocks_one_damaged_template_from_valid_manifest() {
    let root = temp_root("shell-export-package-damaged-template");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let runbook = shell_runbook_for_manifest(&manifest);
    assert_eq!(runbook.status, StudioShellRunbookStatus::Ready);

    std::fs::remove_file(
        bundle_root
            .join("studio.graph.phone")
            .join("shells/phone/studio.graph.phone.shell-template.json"),
    )
    .expect("remove phone template manifest");

    let package = shell_export_package_for_manifest(&manifest);

    assert_eq!(package.status, StudioShellExportPackageStatus::Blocked);
    assert_eq!(
        package.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert_eq!(package.ready_count, 2);
    assert_eq!(package.blocked_count, 1);
    assert_eq!(package.rejected_count, 0);
    assert_eq!(package.descriptor_count, 3);
    assert_eq!(package.template_manifest_count, 2);
    assert_eq!(package.runbook_entry_count, 3);

    let phone = package
        .entries
        .iter()
        .find(|entry| entry.graph_id == "studio.graph.phone")
        .expect("phone export package row");
    assert_eq!(phone.status, StudioShellExportPackageStatus::Blocked);
    assert_eq!(phone.responsible_owner, "rusty.studio");
    assert_eq!(
        phone.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert!(phone.descriptor.is_some());
    assert!(phone.template_manifest.is_none());
    assert!(phone.runbook_cli_request.is_empty());
    assert_eq!(phone.host_routes, empty_shell_host_routes());

    for target_kind in [StudioShellTargetKind::Desktop, StudioShellTargetKind::Quest] {
        let entry = package
            .entries
            .iter()
            .find(|entry| entry.target_kind == target_kind)
            .expect("undamaged export package row");
        assert_eq!(entry.status, StudioShellExportPackageStatus::Ready);
        assert_eq!(entry.responsible_owner, "rusty.hostess");
        assert!(entry.descriptor.is_some());
        assert!(entry.template_manifest.is_some());
        assert!(!entry.runbook_cli_request.is_empty());
    }

    let phone_summary = package
        .target_summaries
        .iter()
        .find(|summary| summary.target_kind == StudioShellTargetKind::Phone)
        .expect("phone summary");
    assert_eq!(phone_summary.ready_count, 0);
    assert_eq!(phone_summary.blocked_count, 1);
    assert_eq!(phone_summary.descriptor_count, 1);
    assert_eq!(phone_summary.template_manifest_count, 0);
    assert_eq!(
        phone_summary.issue_codes,
        vec!["studio.issue.shell_export_package_template_load_failed"]
    );
}

#[test]
fn shell_export_package_comparison_reports_unchanged_ready_packages() {
    let root = temp_root("shell-export-package-comparison");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);

    let comparison = compare_shell_export_packages(&package, &package);

    assert_eq!(comparison.schema_id, SHELL_EXPORT_PACKAGE_COMPARISON_SCHEMA);
    assert_eq!(
        comparison.baseline_schema,
        SHELL_EXPORT_PACKAGE_REPORT_SCHEMA
    );
    assert_eq!(
        comparison.status,
        StudioShellExportPackageComparisonStatus::Unchanged
    );
    assert_eq!(comparison.issue_code, None);
    assert_eq!(comparison.ready_delta, 0);
    assert_eq!(comparison.blocked_delta, 0);
    assert_eq!(comparison.rejected_delta, 0);
    assert_eq!(comparison.descriptor_delta, 0);
    assert_eq!(comparison.template_manifest_delta, 0);
    assert_eq!(comparison.runbook_entry_delta, 0);
    assert_eq!(comparison.entries.len(), 3);
    assert!(comparison.entries.iter().all(|entry| {
        entry.change == StudioShellExportPackageComparisonChange::Unchanged
            && entry.baseline_descriptor_present
            && entry.candidate_descriptor_present
            && entry.baseline_template_manifest_present
            && entry.candidate_template_manifest_present
            && entry.baseline_runbook_cli_request_present
            && entry.candidate_runbook_cli_request_present
    }));
    assert!(comparison
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn shell_export_package_comparison_reports_regressed_damaged_template() {
    let root = temp_root("shell-export-package-comparison-damaged-template");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let baseline = shell_export_package_for_manifest(&manifest);
    assert_eq!(baseline.status, StudioShellExportPackageStatus::Ready);

    std::fs::remove_file(
        bundle_root
            .join("studio.graph.phone")
            .join("shells/phone/studio.graph.phone.shell-template.json"),
    )
    .expect("remove phone template manifest");
    let candidate = shell_export_package_for_manifest(&manifest);

    let comparison = compare_shell_export_packages(&baseline, &candidate);

    assert_eq!(
        comparison.status,
        StudioShellExportPackageComparisonStatus::Regressed
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert_eq!(comparison.ready_delta, -1);
    assert_eq!(comparison.blocked_delta, 1);
    assert_eq!(comparison.rejected_delta, 0);
    assert_eq!(comparison.descriptor_delta, 0);
    assert_eq!(comparison.template_manifest_delta, -1);
    assert_eq!(comparison.runbook_entry_delta, 0);
    let phone = comparison
        .entries
        .iter()
        .find(|entry| entry.graph_id == "studio.graph.phone")
        .expect("phone comparison entry");
    assert_eq!(
        phone.change,
        StudioShellExportPackageComparisonChange::Regressed
    );
    assert_eq!(phone.score_delta, -1);
    assert!(phone.baseline_descriptor_present);
    assert!(phone.candidate_descriptor_present);
    assert!(phone.baseline_template_manifest_present);
    assert!(!phone.candidate_template_manifest_present);
    assert!(phone.baseline_runbook_cli_request_present);
    assert!(!phone.candidate_runbook_cli_request_present);
    assert_eq!(
        phone.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
}

#[test]
fn shell_export_package_comparison_rejects_mismatched_projects() {
    let root = temp_root("shell-export-package-comparison-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let baseline = shell_export_package_for_project(&project, Some(&root), &bundle_root);
    let mut candidate = baseline.clone();
    candidate.project_id = "studio.project.other".to_string();

    let comparison = compare_shell_export_packages(&baseline, &candidate);

    assert_eq!(
        comparison.status,
        StudioShellExportPackageComparisonStatus::Incomparable
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_project_mismatch")
    );
    assert!(comparison.entries.is_empty());
    assert!(comparison.checks.iter().any(|check| {
        check.check_id == "studio.check.shell_export_package_comparison.project_id"
            && check.status == StudioValidationStatus::Fail
    }));
}

#[test]
fn shell_export_package_baseline_index_lists_named_review_slots() {
    let root = temp_root("shell-export-package-baseline-index");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &ready_bundle_root);
    let ready_package = shell_export_package_for_manifest(&manifest);
    let mut blocked_package = ready_package.clone();
    blocked_package.status = StudioShellExportPackageStatus::Blocked;
    blocked_package.issue_code =
        Some("studio.issue.shell_export_package_template_load_failed".to_string());
    blocked_package.ready_count = 2;
    blocked_package.blocked_count = 1;
    blocked_package.template_manifest_count = 2;

    let ready_package_path = root.join("shell-export-package-ready.json");
    let blocked_package_path = root.join("shell-export-package-blocked.json");
    let ready_baseline_path = root.join("shell-export-package-baseline-ready.json");
    let blocked_baseline_path = root.join("shell-export-package-baseline-blocked.json");
    let ready_baseline = shell_export_package_baseline_manifest_for_report(
        &ready_package,
        &ready_package_path,
        Some("synthetic-ready-package"),
        Some("Synthetic ready export package baseline"),
    );
    let blocked_baseline = shell_export_package_baseline_manifest_for_report(
        &blocked_package,
        &blocked_package_path,
        Some("synthetic-blocked-package"),
        Some("Synthetic blocked export package baseline"),
    );

    let index = shell_export_package_baseline_index_for_manifests(
        vec![
            (ready_baseline, Some(ready_baseline_path.clone())),
            (blocked_baseline, Some(blocked_baseline_path)),
        ],
        Some("synthetic-ready-package"),
    );
    let selection = summarize_shell_export_package_baseline_index_selection(
        &index,
        Some(&root.join("shell-export-package-baselines.json")),
        None,
    );

    assert_eq!(index.schema_id, SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA);
    assert_eq!(
        index.default_baseline_id.as_deref(),
        Some("synthetic-ready-package")
    );
    assert_eq!(index.baseline_count, 2);
    assert_eq!(index.ready_baseline_count, 1);
    assert_eq!(index.blocked_baseline_count, 1);
    assert_eq!(index.rejected_baseline_count, 0);
    assert_eq!(index.project_ids, vec!["studio.project.test"]);
    assert_eq!(
        index.package_ids,
        vec!["studio.shell_export_package.studio.project.test"]
    );
    let selected = select_shell_export_package_baseline_index_entry(&index, None)
        .expect("selected export package baseline");
    assert_eq!(selected.baseline_id, "synthetic-ready-package");
    assert_eq!(
        selected.baseline_manifest_path.as_deref(),
        Some(ready_baseline_path.display().to_string().as_str())
    );
    assert_eq!(
        selected.package_path,
        ready_package_path.display().to_string()
    );
    assert_eq!(selected.status, StudioShellExportPackageStatus::Ready);
    assert_eq!(selected.ready_count, 3);
    assert_eq!(selected.target_count, 3);
    assert_eq!(
        selection.schema_id,
        SHELL_EXPORT_PACKAGE_BASELINE_SELECTION_SCHEMA
    );
    assert_eq!(
        selection.status,
        StudioShellExportPackageBaselineSelectionStatus::Selected
    );
    assert_eq!(
        selection.selected_baseline_id.as_deref(),
        Some("synthetic-ready-package")
    );
    assert!(
        selection
            .entries
            .iter()
            .find(|entry| entry.baseline_id == "synthetic-ready-package")
            .expect("ready selection entry")
            .selected
    );
}

#[test]
fn shell_export_package_comparison_carries_baseline_index_selection() {
    let root = temp_root("shell-export-package-comparison-baseline-index");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);
    let package_path = root.join("shell-export-package.json");
    let baseline_path = root.join("shell-export-package-baseline.json");
    let index_path = root.join("shell-export-package-baselines.json");
    let baseline_identity = shell_export_package_baseline_manifest_for_report(
        &package,
        &package_path,
        Some("synthetic-ready-package"),
        Some("Synthetic ready export package baseline"),
    );
    let index = shell_export_package_baseline_index_for_manifests(
        vec![(baseline_identity.clone(), Some(baseline_path.clone()))],
        Some("synthetic-ready-package"),
    );
    let selected_entry = select_shell_export_package_baseline_index_entry(&index, None)
        .expect("selected export package baseline index entry");

    let comparison = compare_shell_export_packages_against_baseline_index_entry(
        &index,
        Some(&index_path),
        selected_entry,
        Some(&baseline_path),
        &baseline_identity,
        &package,
        &package,
    );

    assert_eq!(
        comparison.baseline_identity_schema.as_deref(),
        Some(SHELL_EXPORT_PACKAGE_BASELINE_MANIFEST_SCHEMA)
    );
    assert_eq!(
        comparison.baseline_id.as_deref(),
        Some("synthetic-ready-package")
    );
    assert_eq!(
        comparison.baseline_package_path.as_deref(),
        Some(package_path.display().to_string().as_str())
    );
    assert_eq!(
        comparison.baseline_index_schema.as_deref(),
        Some(SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA)
    );
    assert_eq!(
        comparison.baseline_index_path.as_deref(),
        Some(index_path.display().to_string().as_str())
    );
    assert_eq!(
        comparison.baseline_index_selected_baseline_id.as_deref(),
        Some("synthetic-ready-package")
    );
    assert_eq!(
        comparison.status,
        StudioShellExportPackageComparisonStatus::Unchanged
    );
    assert!(comparison
        .checks
        .iter()
        .filter(|check| check.check_id.contains("baseline_identity"))
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert!(comparison
        .checks
        .iter()
        .filter(|check| check.check_id.contains("baseline_index"))
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert_eq!(comparison.entries.len(), 3);
}

#[test]
fn shell_export_package_comparison_rejects_stale_baseline_identity() {
    let root = temp_root("shell-export-package-comparison-stale-baseline");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);
    let package_path = root.join("shell-export-package.json");
    let mut baseline_identity =
        shell_export_package_baseline_manifest_for_report(&package, &package_path, None, None);
    baseline_identity.project_revision += 1;

    let comparison = compare_shell_export_packages_against_baseline_manifest(
        &baseline_identity,
        &package,
        &package,
    );

    assert_eq!(
        comparison.status,
        StudioShellExportPackageComparisonStatus::Incomparable
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_baseline_identity_mismatch")
    );
    assert!(comparison.entries.is_empty());
    assert!(comparison.checks.iter().any(|check| {
        check.check_id == "studio.check.shell_export_package_comparison.baseline_identity_project"
            && check.status == StudioValidationStatus::Fail
    }));
}

#[test]
fn shell_handoff_intake_rejects_invalid_manifest() {
    let root = temp_root("shell-handoff-intake-invalid");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    manifest.runtime_authority.command_session_authority = "rusty.studio".to_string();

    let intake = shell_handoff_intake_for_manifest(&manifest);

    assert_eq!(intake.schema_id, SHELL_HANDOFF_INTAKE_REPORT_SCHEMA);
    assert_eq!(intake.status, StudioShellHandoffIntakeStatus::Rejected);
    assert_eq!(
        intake.issue_code.as_deref(),
        Some("studio.issue.runtime_authority_mismatch")
    );
    assert_eq!(intake.validation.status, StudioValidationStatus::Fail);
    assert_eq!(intake.accepted_count, 0);
    assert_eq!(intake.blocked_count, 0);
    assert!(intake.target_summaries.is_empty());
    assert!(intake.entries.is_empty());
    assert_eq!(intake.command_session_authority, "rusty.manifold");
    assert_eq!(intake.install_launch_evidence_authority, "rusty.hostess");
}

#[test]
fn shell_handoff_intake_blocks_missing_bundles_after_valid_manifest_shape() {
    let root = temp_root("shell-handoff-intake-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

    let intake = shell_handoff_intake_for_manifest(&manifest);

    assert_eq!(intake.status, StudioShellHandoffIntakeStatus::Accepted);
    assert_eq!(intake.validation.status, StudioValidationStatus::Pass);
    assert_eq!(intake.accepted_count, 0);
    assert_eq!(intake.blocked_count, 3);
    assert_eq!(intake.target_summaries.len(), 3);
    assert_eq!(intake.entries.len(), 3);
    assert!(intake.target_summaries.iter().all(|summary| {
        summary.accepted_count == 0
            && summary.blocked_count == 1
            && summary.graph_ids.len() == 1
            && summary.consumer_ids.len() == 1
    }));
    assert!(intake.entries.iter().all(|entry| {
        entry.decision == StudioShellHandoffIntakeDecision::BlockedByHandoffIssue
            && entry.handoff_status == StudioValidationStatus::Fail
            && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            && entry.next_required_action == "repair_export_bundle"
            && entry.consumer_args.is_empty()
    }));
}

#[test]
fn shell_handoff_acceptance_checklist_reports_ready_entries() {
    let root = temp_root("shell-handoff-acceptance");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);

    let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

    assert_eq!(
        checklist.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA
    );
    assert_eq!(
        checklist.source_intake_schema,
        SHELL_HANDOFF_INTAKE_REPORT_SCHEMA
    );
    assert_eq!(checklist.status, StudioShellHandoffAcceptanceStatus::Ready);
    assert_eq!(checklist.issue_code, None);
    assert_eq!(checklist.ready_count, 3);
    assert_eq!(checklist.blocked_count, 0);
    assert_eq!(checklist.rejected_count, 0);
    assert_eq!(checklist.entries.len(), 3);
    assert_eq!(
        checklist.prohibited_actions,
        vec![
            "install".to_string(),
            "launch".to_string(),
            "open_command_session".to_string(),
            "collect_device_evidence".to_string()
        ]
    );
    assert!(checklist
        .intake_checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert!(checklist.entries.iter().all(|entry| {
        entry.status == StudioShellHandoffAcceptanceStatus::Ready
            && entry.issue_code.is_none()
            && entry.next_required_action == "stage_with_runtime_owner"
            && entry.command_session_authority == "rusty.manifold"
            && entry.install_launch_evidence_authority == "rusty.hostess"
            && entry.studio_role == "authoring.export_planning"
            && entry
                .checks
                .iter()
                .all(|check| check.status == StudioValidationStatus::Pass)
    }));
}

#[test]
fn shell_handoff_acceptance_snapshot_reports_ready_project() {
    let root = temp_root("shell-handoff-acceptance-snapshot");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);
    let expected = shell_handoff_acceptance_checklist_for_intake(&intake);

    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(checklist, expected);
    assert_eq!(checklist.status, StudioShellHandoffAcceptanceStatus::Ready);
    assert_eq!(checklist.ready_count, 3);
    assert_eq!(checklist.blocked_count, 0);
    assert_eq!(checklist.rejected_count, 0);
    assert_eq!(checklist.entries.len(), 3);
}

#[test]
fn shell_handoff_acceptance_snapshot_blocks_missing_bundles() {
    let root = temp_root("shell-handoff-acceptance-snapshot-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");

    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(
        checklist.status,
        StudioShellHandoffAcceptanceStatus::Blocked
    );
    assert_eq!(
        checklist.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(checklist.ready_count, 0);
    assert_eq!(checklist.blocked_count, 3);
    assert_eq!(checklist.rejected_count, 0);
    assert_eq!(checklist.entries.len(), 3);
    assert!(checklist.entries.iter().all(|entry| {
        entry.status == StudioShellHandoffAcceptanceStatus::Blocked
            && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
    }));
}

#[test]
fn shell_handoff_acceptance_summary_reports_baseline_metadata() {
    let root = temp_root("shell-handoff-acceptance-summary");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");

    let summary = summarize_shell_handoff_acceptance_checklist(&checklist, Some(&checklist_path));

    assert_eq!(summary.schema_id, SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA);
    assert_eq!(
        summary.checklist_schema,
        SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA
    );
    let checklist_path_text = checklist_path.display().to_string();
    assert_eq!(
        summary.checklist_path.as_deref(),
        Some(checklist_path_text.as_str())
    );
    assert_eq!(summary.manifest_id, checklist.manifest_id);
    assert_eq!(summary.project_id, "studio.project.test");
    assert_eq!(summary.project_revision, 1);
    assert_eq!(summary.status, StudioShellHandoffAcceptanceStatus::Ready);
    assert_eq!(summary.issue_code, None);
    assert_eq!(summary.ready_count, 3);
    assert_eq!(summary.blocked_count, 0);
    assert_eq!(summary.rejected_count, 0);
    assert_eq!(summary.entry_count, 3);
    assert_eq!(summary.failed_intake_check_count, 0);
    assert_eq!(summary.targets.len(), 3);
    assert!(summary.targets.iter().all(|target| {
        target.graph_count == 1
            && target.ready_count == 1
            && target.blocked_count == 0
            && target.rejected_count == 0
            && target.graph_ids.len() == 1
            && target.consumer_ids.len() == 1
            && target.route_kinds.len() == 1
            && target.issue_codes.is_empty()
    }));
}

#[test]
fn shell_handoff_acceptance_summary_reports_blocked_target_metadata() {
    let root = temp_root("shell-handoff-acceptance-summary-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );

    let summary = summarize_shell_handoff_acceptance_checklist(&checklist, None);

    assert_eq!(summary.checklist_path, None);
    assert_eq!(summary.status, StudioShellHandoffAcceptanceStatus::Blocked);
    assert_eq!(
        summary.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(summary.ready_count, 0);
    assert_eq!(summary.blocked_count, 3);
    assert_eq!(summary.rejected_count, 0);
    assert_eq!(summary.entry_count, 3);
    assert_eq!(summary.targets.len(), 3);
    assert!(summary.targets.iter().all(|target| {
        target.ready_count == 0
            && target.blocked_count == 1
            && target.rejected_count == 0
            && target.issue_codes == vec!["studio.issue.shell_bundle_file_missing".to_string()]
    }));
}

#[test]
fn shell_handoff_acceptance_baseline_manifest_names_saved_checklist() {
    let root = temp_root("shell-handoff-acceptance-baseline-manifest");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");

    let baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        None,
        None,
    );

    assert_eq!(
        baseline.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA
    );
    assert_eq!(baseline.baseline_id, "studio.project.test.rev1.ready");
    assert_eq!(
        baseline.label,
        "studio.project.test revision 1 ready acceptance baseline"
    );
    let checklist_path_text = checklist_path.display().to_string();
    assert_eq!(baseline.checklist_path, checklist_path_text);
    assert_eq!(
        baseline.summary.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA
    );
    assert_eq!(
        baseline.summary.checklist_schema,
        SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA
    );
    assert_eq!(baseline.summary.project_id, "studio.project.test");
    assert_eq!(baseline.summary.project_revision, 1);
    assert_eq!(
        baseline.summary.status,
        StudioShellHandoffAcceptanceStatus::Ready
    );
    assert_eq!(baseline.summary.ready_count, 3);
    assert_eq!(baseline.summary.blocked_count, 0);
    assert_eq!(baseline.summary.rejected_count, 0);
    assert_eq!(baseline.summary.entry_count, 3);
    assert_eq!(baseline.summary.targets.len(), 3);
}

#[test]
fn shell_handoff_acceptance_baseline_manifest_accepts_custom_identity() {
    let root = temp_root("shell-handoff-acceptance-baseline-manifest-custom");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );
    let checklist_path = root.join("blocked-checklist.json");

    let baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some("preflight-blocked"),
        Some("Preflight blocked baseline"),
    );

    assert_eq!(baseline.baseline_id, "preflight-blocked");
    assert_eq!(baseline.label, "Preflight blocked baseline");
    assert_eq!(
        baseline.summary.status,
        StudioShellHandoffAcceptanceStatus::Blocked
    );
    assert_eq!(
        baseline.summary.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
}

#[test]
fn shell_handoff_acceptance_baseline_index_lists_named_baselines() {
    let root = temp_root("shell-handoff-acceptance-baseline-index");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &ready_bundle_root);
    let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );
    let ready_checklist_path = root.join("ready-checklist.json");
    let blocked_checklist_path = root.join("blocked-checklist.json");
    let ready_manifest_path = root.join("ready-baseline.json");
    let blocked_manifest_path = root.join("blocked-baseline.json");
    let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &ready_checklist,
        &ready_checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &blocked_checklist,
        &blocked_checklist_path,
        Some("synthetic-blocked"),
        Some("Synthetic blocked acceptance baseline"),
    );

    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![
            (ready_baseline, Some(ready_manifest_path.clone())),
            (blocked_baseline, Some(blocked_manifest_path.clone())),
        ],
        Some("synthetic-ready"),
    );

    assert_eq!(
        index.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA
    );
    assert_eq!(index.project_ids, vec!["studio.project.test"]);
    assert_eq!(
        index.manifest_ids,
        vec!["studio.shell_handoffs.studio.project.test"]
    );
    assert_eq!(
        index.default_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(index.baseline_count, 2);
    assert_eq!(index.ready_baseline_count, 1);
    assert_eq!(index.blocked_baseline_count, 1);
    assert_eq!(index.rejected_baseline_count, 0);
    assert_eq!(index.entries.len(), 2);
    assert_eq!(index.entries[0].baseline_id, "synthetic-blocked");
    assert_eq!(
        index.entries[0].baseline_manifest_path.as_deref(),
        Some(blocked_manifest_path.display().to_string().as_str())
    );
    assert_eq!(
        index.entries[0].checklist_path,
        blocked_checklist_path.display().to_string()
    );
    assert_eq!(
        index.entries[0].status,
        StudioShellHandoffAcceptanceStatus::Blocked
    );
    assert_eq!(index.entries[0].ready_count, 0);
    assert_eq!(index.entries[0].blocked_count, 3);
    assert_eq!(index.entries[0].target_count, 3);
    assert_eq!(index.entries[1].baseline_id, "synthetic-ready");
    assert_eq!(
        index.entries[1].baseline_manifest_path.as_deref(),
        Some(ready_manifest_path.display().to_string().as_str())
    );
    assert_eq!(index.entries[1].ready_count, 3);
    assert_eq!(index.entries[1].blocked_count, 0);
    assert_eq!(
        select_shell_handoff_acceptance_baseline_index_entry(&index, None)
            .map(|entry| entry.baseline_id.as_str()),
        Some("synthetic-ready")
    );
    assert_eq!(
        select_shell_handoff_acceptance_baseline_index_entry(&index, Some("synthetic-blocked"))
            .map(|entry| entry.baseline_id.as_str()),
        Some("synthetic-blocked")
    );
    assert!(
        select_shell_handoff_acceptance_baseline_index_entry(&index, Some("missing")).is_none()
    );
}

#[test]
fn shell_handoff_acceptance_baseline_index_appends_named_baseline() {
    let root = temp_root("shell-handoff-acceptance-baseline-index-append");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &ready_bundle_root);
    let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );
    let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &ready_checklist,
        &root.join("ready-checklist.json"),
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let blocked_manifest_path = root.join("blocked-baseline.json");
    let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &blocked_checklist,
        &root.join("blocked-checklist.json"),
        Some("synthetic-blocked"),
        Some("Synthetic blocked acceptance baseline"),
    );
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(ready_baseline, Some(root.join("ready-baseline.json")))],
        Some("synthetic-ready"),
    );

    let appended = append_shell_handoff_acceptance_baseline_index_manifests(
        &index,
        vec![(blocked_baseline, Some(blocked_manifest_path.clone()))],
        Some("synthetic-blocked"),
    );

    assert_eq!(
        appended.default_baseline_id.as_deref(),
        Some("synthetic-blocked")
    );
    assert_eq!(appended.baseline_count, 2);
    assert_eq!(appended.ready_baseline_count, 1);
    assert_eq!(appended.blocked_baseline_count, 1);
    assert_eq!(appended.entries[0].baseline_id, "synthetic-blocked");
    assert_eq!(
        appended.entries[0].baseline_manifest_path.as_deref(),
        Some(blocked_manifest_path.display().to_string().as_str())
    );
    assert_eq!(
        appended.entries[0].status,
        StudioShellHandoffAcceptanceStatus::Blocked
    );
    assert_eq!(appended.entries[1].baseline_id, "synthetic-ready");
    assert_eq!(
        select_shell_handoff_acceptance_baseline_index_entry(&appended, None)
            .map(|entry| entry.baseline_id.as_str()),
        Some("synthetic-blocked")
    );
}

#[test]
fn shell_handoff_acceptance_baseline_index_promotes_existing_default() {
    let root = temp_root("shell-handoff-acceptance-baseline-index-promote");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &ready_bundle_root);
    let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );
    let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &ready_checklist,
        &root.join("ready-checklist.json"),
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &blocked_checklist,
        &root.join("blocked-checklist.json"),
        Some("synthetic-blocked"),
        Some("Synthetic blocked acceptance baseline"),
    );
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![
            (ready_baseline, Some(root.join("ready-baseline.json"))),
            (blocked_baseline, Some(root.join("blocked-baseline.json"))),
        ],
        Some("synthetic-blocked"),
    );

    let promoted =
        promote_shell_handoff_acceptance_baseline_index_default(&index, "synthetic-ready")
            .expect("promote ready baseline");

    assert_eq!(
        promoted.default_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(promoted.baseline_count, 2);
    assert_eq!(promoted.ready_baseline_count, 1);
    assert_eq!(promoted.blocked_baseline_count, 1);
    assert_eq!(
        select_shell_handoff_acceptance_baseline_index_entry(&promoted, None)
            .map(|entry| entry.baseline_id.as_str()),
        Some("synthetic-ready")
    );
    assert!(promote_shell_handoff_acceptance_baseline_index_default(&index, "missing").is_none());
}

#[test]
fn shell_handoff_acceptance_baseline_selection_reports_default_entry() {
    let root = temp_root("shell-handoff-acceptance-baseline-selection");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &ready_bundle_root);
    let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );
    let ready_checklist_path = root.join("ready-checklist.json");
    let blocked_checklist_path = root.join("blocked-checklist.json");
    let ready_manifest_path = root.join("ready-baseline.json");
    let blocked_manifest_path = root.join("blocked-baseline.json");
    let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &ready_checklist,
        &ready_checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &blocked_checklist,
        &blocked_checklist_path,
        Some("synthetic-blocked"),
        Some("Synthetic blocked acceptance baseline"),
    );
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![
            (ready_baseline, Some(ready_manifest_path.clone())),
            (blocked_baseline, Some(blocked_manifest_path)),
        ],
        Some("synthetic-ready"),
    );
    let index_path = root.join("shell-handoff-acceptance-baselines.json");

    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        &index,
        Some(&index_path),
        None,
    );

    assert_eq!(
        selection.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_BASELINE_SELECTION_SCHEMA
    );
    assert_eq!(
        selection.source_index_schema,
        SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA
    );
    assert_eq!(
        selection.index_path.as_deref(),
        Some(index_path.display().to_string().as_str())
    );
    assert_eq!(selection.requested_baseline_id, None);
    assert_eq!(
        selection.default_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(
        selection.selected_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(
        selection.status,
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
    );
    assert_eq!(selection.issue_code, None);
    assert_eq!(selection.baseline_count, 2);
    assert_eq!(selection.ready_baseline_count, 1);
    assert_eq!(selection.blocked_baseline_count, 1);
    assert_eq!(selection.project_ids, vec!["studio.project.test"]);
    assert_eq!(selection.entries.len(), 2);
    let selected = selection
        .entries
        .iter()
        .find(|entry| entry.baseline_id == "synthetic-ready")
        .expect("selected entry");
    assert!(selected.selected);
    assert!(selected.default);
    assert_eq!(
        selected.baseline_manifest_path.as_deref(),
        Some(ready_manifest_path.display().to_string().as_str())
    );
    assert_eq!(selected.status, StudioShellHandoffAcceptanceStatus::Ready);
    assert_eq!(selected.ready_count, 3);
    assert_eq!(selected.entry_count, 3);
    assert_eq!(selected.target_count, 3);
    let blocked = selection
        .entries
        .iter()
        .find(|entry| entry.baseline_id == "synthetic-blocked")
        .expect("blocked entry");
    assert!(!blocked.selected);
    assert!(!blocked.default);
    assert_eq!(blocked.status, StudioShellHandoffAcceptanceStatus::Blocked);
}

#[test]
fn shell_handoff_acceptance_baseline_selection_reports_missing_request() {
    let root = temp_root("shell-handoff-acceptance-baseline-selection-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("ready-checklist.json");
    let baseline_path = root.join("ready-baseline.json");
    let baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(baseline, Some(baseline_path))],
        Some("synthetic-ready"),
    );

    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        &index,
        None,
        Some("synthetic-missing"),
    );

    assert_eq!(
        selection.status,
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Missing
    );
    assert_eq!(
        selection.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_acceptance_baseline_not_found")
    );
    assert_eq!(
        selection.requested_baseline_id.as_deref(),
        Some("synthetic-missing")
    );
    assert_eq!(selection.selected_baseline_id, None);
    assert_eq!(selection.baseline_count, 1);
    assert!(selection.entries.iter().all(|entry| !entry.selected));
}

#[test]
fn shell_handoff_acceptance_baseline_selection_reports_empty_index() {
    let index = shell_handoff_acceptance_baseline_index_for_manifests(Vec::new(), None);

    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(&index, None, None);

    assert_eq!(
        selection.status,
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Empty
    );
    assert_eq!(
        selection.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_acceptance_baseline_index_empty")
    );
    assert_eq!(selection.baseline_count, 0);
    assert_eq!(selection.selected_baseline_id, None);
    assert!(selection.entries.is_empty());
}

#[test]
fn shell_handoff_acceptance_checklist_rejects_invalid_intake() {
    let root = temp_root("shell-handoff-acceptance-invalid");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    manifest.runtime_authority.command_session_authority = "rusty.studio".to_string();
    let intake = shell_handoff_intake_for_manifest(&manifest);

    let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

    assert_eq!(
        checklist.status,
        StudioShellHandoffAcceptanceStatus::Rejected
    );
    assert_eq!(
        checklist.issue_code.as_deref(),
        Some("studio.issue.runtime_authority_mismatch")
    );
    assert_eq!(checklist.ready_count, 0);
    assert_eq!(checklist.blocked_count, 0);
    assert_eq!(checklist.rejected_count, 0);
    assert!(checklist.entries.is_empty());
    assert!(checklist.intake_checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && check.issue_code.as_deref() == Some("studio.issue.shell_handoff_intake_rejected")
    }));
}

#[test]
fn shell_handoff_acceptance_checklist_blocks_missing_bundles() {
    let root = temp_root("shell-handoff-acceptance-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);

    let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

    assert_eq!(
        checklist.status,
        StudioShellHandoffAcceptanceStatus::Blocked
    );
    assert_eq!(
        checklist.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(checklist.ready_count, 0);
    assert_eq!(checklist.blocked_count, 3);
    assert_eq!(checklist.rejected_count, 0);
    assert_eq!(checklist.entries.len(), 3);
    assert!(checklist
        .intake_checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert!(checklist.entries.iter().all(|entry| {
        entry.status == StudioShellHandoffAcceptanceStatus::Blocked
            && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            && entry.next_required_action == "repair_export_bundle"
            && entry.checks.iter().any(|check| {
                check.status == StudioValidationStatus::Fail
                    && check.issue_code.as_deref()
                        == Some("studio.issue.shell_handoff_acceptance_blocked")
            })
    }));
}

#[test]
fn shell_handoff_acceptance_comparison_reports_unchanged_ready_checklists() {
    let root = temp_root("shell-handoff-acceptance-compare-unchanged");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);
    let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

    let comparison = compare_shell_handoff_acceptance_checklists(&checklist, &checklist);

    assert_eq!(
        comparison.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_COMPARISON_SCHEMA
    );
    assert_eq!(comparison.baseline_identity_schema, None);
    assert_eq!(comparison.baseline_id, None);
    assert_eq!(comparison.baseline_label, None);
    assert_eq!(comparison.baseline_checklist_path, None);
    assert_eq!(comparison.baseline_index_schema, None);
    assert_eq!(comparison.baseline_index_path, None);
    assert_eq!(comparison.baseline_index_default_baseline_id, None);
    assert_eq!(comparison.baseline_index_selected_baseline_id, None);
    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Unchanged
    );
    assert_eq!(comparison.issue_code, None);
    assert_eq!(comparison.ready_delta, 0);
    assert_eq!(comparison.blocked_delta, 0);
    assert_eq!(comparison.rejected_delta, 0);
    assert_eq!(comparison.entries.len(), 3);
    assert!(comparison.entries.iter().all(|entry| entry.change
        == StudioShellHandoffAcceptanceComparisonChange::Unchanged
        && entry.score_delta == 0));
    assert!(comparison
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn shell_handoff_acceptance_comparison_carries_baseline_identity() {
    let root = temp_root("shell-handoff-acceptance-compare-baseline-identity");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    let baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );

    let comparison = compare_shell_handoff_acceptance_against_baseline_manifest(
        &baseline_identity,
        &checklist,
        &checklist,
    );

    assert_eq!(
        comparison.baseline_identity_schema.as_deref(),
        Some(SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA)
    );
    assert_eq!(comparison.baseline_id.as_deref(), Some("synthetic-ready"));
    assert_eq!(
        comparison.baseline_label.as_deref(),
        Some("Synthetic ready acceptance baseline")
    );
    let checklist_path_text = checklist_path.display().to_string();
    assert_eq!(
        comparison.baseline_checklist_path.as_deref(),
        Some(checklist_path_text.as_str())
    );
    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Unchanged
    );
    assert!(comparison
        .checks
        .iter()
        .filter(|check| check.check_id.contains("baseline_identity"))
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert_eq!(comparison.entries.len(), 3);
}

#[test]
fn shell_handoff_acceptance_comparison_carries_baseline_index_selection() {
    let root = temp_root("shell-handoff-acceptance-compare-baseline-index");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    let baseline_path = root.join("shell-handoff-acceptance-baseline.json");
    let index_path = root.join("shell-handoff-acceptance-baselines.json");
    let baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(baseline_identity.clone(), Some(baseline_path.clone()))],
        Some("synthetic-ready"),
    );
    let selected_entry = select_shell_handoff_acceptance_baseline_index_entry(&index, None)
        .expect("selected baseline index entry");

    let comparison = compare_shell_handoff_acceptance_against_baseline_index_entry(
        &index,
        Some(&index_path),
        selected_entry,
        Some(&baseline_path),
        &baseline_identity,
        &checklist,
        &checklist,
    );

    assert_eq!(
        comparison.baseline_index_schema.as_deref(),
        Some(SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA)
    );
    assert_eq!(
        comparison.baseline_index_path.as_deref(),
        Some(index_path.display().to_string().as_str())
    );
    assert_eq!(
        comparison.baseline_index_default_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(
        comparison.baseline_index_selected_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Unchanged
    );
    assert!(comparison
        .checks
        .iter()
        .filter(|check| check.check_id.contains("baseline_index"))
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert_eq!(comparison.entries.len(), 3);
}

#[test]
fn shell_handoff_acceptance_comparison_rejects_stale_baseline_index_selection() {
    let root = temp_root("shell-handoff-acceptance-compare-stale-baseline-index");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    let baseline_path = root.join("shell-handoff-acceptance-baseline.json");
    let index_path = root.join("shell-handoff-acceptance-baselines.json");
    let baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let mut index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(baseline_identity.clone(), Some(baseline_path.clone()))],
        Some("synthetic-ready"),
    );
    index.entries[0].ready_count += 1;
    let selected_entry = select_shell_handoff_acceptance_baseline_index_entry(&index, None)
        .expect("selected baseline index entry");

    let comparison = compare_shell_handoff_acceptance_against_baseline_index_entry(
        &index,
        Some(&index_path),
        selected_entry,
        Some(&baseline_path),
        &baseline_identity,
        &checklist,
        &checklist,
    );

    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_acceptance_baseline_index_mismatch")
    );
    assert!(comparison.entries.is_empty());
    assert!(comparison.checks.iter().any(|check| {
        check.check_id
            == "studio.check.shell_handoff_acceptance_comparison.baseline_index_status_counts"
            && check.status == StudioValidationStatus::Fail
    }));
}

#[test]
fn shell_handoff_acceptance_comparison_rejects_stale_baseline_identity() {
    let root = temp_root("shell-handoff-acceptance-compare-stale-baseline-identity");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    let mut baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        None,
        None,
    );
    baseline_identity.summary.project_revision += 1;

    let comparison = compare_shell_handoff_acceptance_against_baseline_manifest(
        &baseline_identity,
        &checklist,
        &checklist,
    );

    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_acceptance_baseline_identity_mismatch")
    );
    assert!(comparison.entries.is_empty());
    assert!(comparison.checks.iter().any(|check| {
        check.check_id
            == "studio.check.shell_handoff_acceptance_comparison.baseline_identity_project"
            && check.status == StudioValidationStatus::Fail
    }));
}

#[test]
fn shell_handoff_acceptance_comparison_reports_regression_to_missing_bundles() {
    let root = temp_root("shell-handoff-acceptance-compare-regressed");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_manifest =
        shell_handoff_manifest_for_project(&project, Some(&root), &ready_bundle_root);
    let ready_intake = shell_handoff_intake_for_manifest(&ready_manifest);
    let ready_checklist = shell_handoff_acceptance_checklist_for_intake(&ready_intake);
    let missing_manifest =
        shell_handoff_manifest_for_project(&project, Some(&root), &root.join("missing"));
    let missing_intake = shell_handoff_intake_for_manifest(&missing_manifest);
    let missing_checklist = shell_handoff_acceptance_checklist_for_intake(&missing_intake);

    let comparison =
        compare_shell_handoff_acceptance_checklists(&ready_checklist, &missing_checklist);

    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Regressed
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(comparison.ready_delta, -3);
    assert_eq!(comparison.blocked_delta, 3);
    assert_eq!(comparison.rejected_delta, 0);
    assert_eq!(comparison.entries.len(), 3);
    assert!(comparison.entries.iter().all(|entry| {
        entry.change == StudioShellHandoffAcceptanceComparisonChange::Regressed
            && entry.score_delta == -1
            && entry.candidate_issue_code.as_deref()
                == Some("studio.issue.shell_bundle_file_missing")
    }));
}

#[test]
fn shell_handoff_acceptance_comparison_reports_improvement_from_missing_bundles() {
    let root = temp_root("shell-handoff-acceptance-compare-improved");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_manifest =
        shell_handoff_manifest_for_project(&project, Some(&root), &ready_bundle_root);
    let ready_intake = shell_handoff_intake_for_manifest(&ready_manifest);
    let ready_checklist = shell_handoff_acceptance_checklist_for_intake(&ready_intake);
    let missing_manifest =
        shell_handoff_manifest_for_project(&project, Some(&root), &root.join("missing"));
    let missing_intake = shell_handoff_intake_for_manifest(&missing_manifest);
    let missing_checklist = shell_handoff_acceptance_checklist_for_intake(&missing_intake);

    let comparison =
        compare_shell_handoff_acceptance_checklists(&missing_checklist, &ready_checklist);

    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Improved
    );
    assert_eq!(comparison.issue_code, None);
    assert_eq!(comparison.ready_delta, 3);
    assert_eq!(comparison.blocked_delta, -3);
    assert_eq!(comparison.rejected_delta, 0);
    assert!(comparison.entries.iter().all(|entry| {
        entry.change == StudioShellHandoffAcceptanceComparisonChange::Improved
            && entry.score_delta == 1
    }));
}

#[test]
fn shell_handoff_acceptance_comparison_rejects_mismatched_projects() {
    let root = temp_root("shell-handoff-acceptance-compare-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);
    let baseline = shell_handoff_acceptance_checklist_for_intake(&intake);
    let mut candidate = baseline.clone();
    candidate.project_id = "studio.project.other".to_string();

    let comparison = compare_shell_handoff_acceptance_checklists(&baseline, &candidate);

    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_acceptance_project_mismatch")
    );
    assert!(comparison.entries.is_empty());
    assert!(comparison.checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && check.issue_code.as_deref()
                == Some("studio.issue.shell_handoff_acceptance_project_mismatch")
    }));
}

#[test]
fn shell_release_candidate_review_reports_ready_from_indexes() {
    let root = temp_root("shell-release-candidate-ready");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    save_selected_shell_bundles(&project, &root, &bundle_root);
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let manifest_path = root.join("shell-handoffs.json");
    save_json(&manifest_path, &manifest).expect("save shell handoff manifest");

    let acceptance_checklist = shell_handoff_acceptance_checklist_for_intake(
        &shell_handoff_intake_for_manifest(&manifest),
    );
    let acceptance_checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    save_json(&acceptance_checklist_path, &acceptance_checklist)
        .expect("save acceptance checklist");
    let acceptance_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &acceptance_checklist,
        &acceptance_checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let acceptance_baseline_path = root.join("shell-handoff-acceptance-baseline.json");
    save_json(&acceptance_baseline_path, &acceptance_baseline).expect("save acceptance baseline");
    let acceptance_index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(acceptance_baseline, Some(acceptance_baseline_path))],
        Some("synthetic-ready"),
    );
    let acceptance_index_path = root.join("shell-handoff-acceptance-baselines.json");
    save_json(&acceptance_index_path, &acceptance_index).expect("save acceptance index");

    let export_package = shell_export_package_for_manifest(&manifest);
    let export_package_path = root.join("shell-export-package.json");
    save_json(&export_package_path, &export_package).expect("save export package");
    let export_package_baseline = shell_export_package_baseline_manifest_for_report(
        &export_package,
        &export_package_path,
        Some("synthetic-ready-package"),
        Some("Synthetic ready export package baseline"),
    );
    let export_package_baseline_path = root.join("shell-export-package-baseline.json");
    save_json(&export_package_baseline_path, &export_package_baseline)
        .expect("save export package baseline");
    let export_package_index = shell_export_package_baseline_index_for_manifests(
        vec![(export_package_baseline, Some(export_package_baseline_path))],
        Some("synthetic-ready-package"),
    );
    let export_package_index_path = root.join("shell-export-package-baselines.json");
    save_json(&export_package_index_path, &export_package_index)
        .expect("save export package index");

    let review = shell_release_candidate_review_for_manifest(
        &manifest,
        Some(&manifest_path),
        &acceptance_index,
        Some(&acceptance_index_path),
        Some("synthetic-ready"),
        &export_package_index,
        Some(&export_package_index_path),
        Some("synthetic-ready-package"),
    );

    assert_eq!(review.schema_id, SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA);
    assert_eq!(review.source_manifest_schema, SHELL_HANDOFF_MANIFEST_SCHEMA);
    assert_eq!(
        review.manifest_path.as_deref(),
        Some(manifest_path.display().to_string().as_str())
    );
    assert_eq!(
        review.status,
        StudioShellReleaseCandidateReviewStatus::Ready
    );
    assert_eq!(review.issue_code, None);
    assert_eq!(review.execution_policy, "not_executed.review_only");
    assert_eq!(review.review_owner, "rusty.hostess");
    assert_eq!(review.command_session_authority, "rusty.manifold");
    assert_eq!(review.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(review.studio_role, "authoring.export_planning");
    assert_eq!(review.handoff_status, StudioValidationStatus::Pass);
    assert_eq!(review.handoff_ready_count, 3);
    assert_eq!(review.handoff_failed_count, 0);
    assert_eq!(review.handoff_missing_bundle_count, 0);
    assert_eq!(
        review.acceptance_baseline_selection.status,
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
    );
    assert_eq!(
        review
            .acceptance_baseline_selection
            .selected_baseline_id
            .as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(
        review
            .acceptance_comparison
            .as_ref()
            .map(|comparison| comparison.status),
        Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
    );
    assert_eq!(
        review.export_package_baseline_selection.status,
        StudioShellExportPackageBaselineSelectionStatus::Selected
    );
    assert_eq!(
        review
            .export_package_baseline_selection
            .selected_baseline_id
            .as_deref(),
        Some("synthetic-ready-package")
    );
    assert_eq!(
        review
            .export_package_comparison
            .as_ref()
            .map(|comparison| comparison.status),
        Some(StudioShellExportPackageComparisonStatus::Unchanged)
    );
    assert!(review
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert!(review.prohibited_actions.contains(&"install".to_string()));
    assert!(review
        .prohibited_actions
        .contains(&"open_command_session".to_string()));
}

#[test]
fn shell_release_candidate_review_blocks_regressed_export_package() {
    let root = temp_root("shell-release-candidate-regressed-package");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    save_selected_shell_bundles(&project, &root, &bundle_root);
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

    let acceptance_checklist = shell_handoff_acceptance_checklist_for_intake(
        &shell_handoff_intake_for_manifest(&manifest),
    );
    let acceptance_checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    save_json(&acceptance_checklist_path, &acceptance_checklist)
        .expect("save acceptance checklist");
    let acceptance_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &acceptance_checklist,
        &acceptance_checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let acceptance_baseline_path = root.join("shell-handoff-acceptance-baseline.json");
    save_json(&acceptance_baseline_path, &acceptance_baseline).expect("save acceptance baseline");
    let acceptance_index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(acceptance_baseline, Some(acceptance_baseline_path))],
        Some("synthetic-ready"),
    );
    let acceptance_index_path = root.join("shell-handoff-acceptance-baselines.json");

    let export_package = shell_export_package_for_manifest(&manifest);
    let export_package_path = root.join("shell-export-package.json");
    save_json(&export_package_path, &export_package).expect("save export package");
    let export_package_baseline = shell_export_package_baseline_manifest_for_report(
        &export_package,
        &export_package_path,
        Some("synthetic-ready-package"),
        Some("Synthetic ready export package baseline"),
    );
    let export_package_baseline_path = root.join("shell-export-package-baseline.json");
    save_json(&export_package_baseline_path, &export_package_baseline)
        .expect("save export package baseline");
    let export_package_index = shell_export_package_baseline_index_for_manifests(
        vec![(export_package_baseline, Some(export_package_baseline_path))],
        Some("synthetic-ready-package"),
    );
    let export_package_index_path = root.join("shell-export-package-baselines.json");

    std::fs::remove_file(
        bundle_root
            .join("studio.graph.phone")
            .join("shells/phone/studio.graph.phone.shell-template.json"),
    )
    .expect("remove phone template manifest");

    let review = shell_release_candidate_review_for_manifest(
        &manifest,
        None,
        &acceptance_index,
        Some(&acceptance_index_path),
        Some("synthetic-ready"),
        &export_package_index,
        Some(&export_package_index_path),
        Some("synthetic-ready-package"),
    );

    assert_eq!(
        review.status,
        StudioShellReleaseCandidateReviewStatus::Blocked
    );
    assert_eq!(
        review.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert_eq!(
        review
            .acceptance_comparison
            .as_ref()
            .map(|comparison| comparison.status),
        Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
    );
    let export_package_comparison = review
        .export_package_comparison
        .as_ref()
        .expect("export package comparison");
    assert_eq!(
        export_package_comparison.status,
        StudioShellExportPackageComparisonStatus::Regressed
    );
    assert_eq!(export_package_comparison.ready_delta, -1);
    assert_eq!(export_package_comparison.blocked_delta, 1);
    assert!(review.checks.iter().any(|check| {
        check.check_id
            == "studio.check.shell_release_candidate_review.export_package_comparison_not_regressed"
            && check.status == StudioValidationStatus::Fail
            && check.issue_code.as_deref()
                == Some("studio.issue.shell_export_package_template_load_failed")
    }));
}

#[test]
fn shell_release_candidate_review_index_lists_and_selects_candidates() {
    let root = temp_root("shell-release-candidate-index");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    save_selected_shell_bundles(&project, &root, &bundle_root);
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let manifest_path = root.join("shell-handoffs.json");
    save_json(&manifest_path, &manifest).expect("save shell handoff manifest");

    let acceptance_checklist = shell_handoff_acceptance_checklist_for_intake(
        &shell_handoff_intake_for_manifest(&manifest),
    );
    let acceptance_checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    save_json(&acceptance_checklist_path, &acceptance_checklist)
        .expect("save acceptance checklist");
    let acceptance_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &acceptance_checklist,
        &acceptance_checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let acceptance_baseline_path = root.join("shell-handoff-acceptance-baseline.json");
    save_json(&acceptance_baseline_path, &acceptance_baseline).expect("save acceptance baseline");
    let acceptance_index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(acceptance_baseline, Some(acceptance_baseline_path))],
        Some("synthetic-ready"),
    );
    let acceptance_index_path = root.join("shell-handoff-acceptance-baselines.json");
    save_json(&acceptance_index_path, &acceptance_index).expect("save acceptance index");

    let export_package = shell_export_package_for_manifest(&manifest);
    let export_package_path = root.join("shell-export-package.json");
    save_json(&export_package_path, &export_package).expect("save export package");
    let export_package_baseline = shell_export_package_baseline_manifest_for_report(
        &export_package,
        &export_package_path,
        Some("synthetic-ready-package"),
        Some("Synthetic ready export package baseline"),
    );
    let export_package_baseline_path = root.join("shell-export-package-baseline.json");
    save_json(&export_package_baseline_path, &export_package_baseline)
        .expect("save export package baseline");
    let export_package_index = shell_export_package_baseline_index_for_manifests(
        vec![(export_package_baseline, Some(export_package_baseline_path))],
        Some("synthetic-ready-package"),
    );
    let export_package_index_path = root.join("shell-export-package-baselines.json");
    save_json(&export_package_index_path, &export_package_index)
        .expect("save export package index");

    let ready_review = shell_release_candidate_review_for_manifest(
        &manifest,
        Some(&manifest_path),
        &acceptance_index,
        Some(&acceptance_index_path),
        Some("synthetic-ready"),
        &export_package_index,
        Some(&export_package_index_path),
        Some("synthetic-ready-package"),
    );
    let ready_review_path = root.join("shell-release-candidate-review.json");
    save_json(&ready_review_path, &ready_review).expect("save ready review");
    let ready_candidate = shell_release_candidate_review_manifest_for_report(
        &ready_review,
        &ready_review_path,
        None,
        None,
    );
    let ready_candidate_path = root.join("shell-release-candidate-review-manifest.json");
    save_json(&ready_candidate_path, &ready_candidate).expect("save ready candidate");

    assert_eq!(
        ready_candidate.schema_id,
        SHELL_RELEASE_CANDIDATE_REVIEW_MANIFEST_SCHEMA
    );
    assert_eq!(
        ready_candidate.candidate_id,
        "studio.project.test.rev1.ready"
    );
    assert_eq!(
        ready_candidate.review_path,
        ready_review_path.display().to_string()
    );
    assert_eq!(
        ready_candidate.status,
        StudioShellReleaseCandidateReviewStatus::Ready
    );
    assert_eq!(
        ready_candidate.acceptance_comparison_status,
        Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
    );
    assert_eq!(
        ready_candidate.export_package_comparison_status,
        Some(StudioShellExportPackageComparisonStatus::Unchanged)
    );
    assert_eq!(ready_candidate.failed_check_count, 0);

    let index = shell_release_candidate_review_index_for_manifests(
        vec![(ready_candidate.clone(), Some(ready_candidate_path.clone()))],
        None,
    );
    let index_path = root.join("shell-release-candidate-reviews.json");
    save_json(&index_path, &index).expect("save release candidate index");
    assert_eq!(index.schema_id, SHELL_RELEASE_CANDIDATE_REVIEW_INDEX_SCHEMA);
    assert_eq!(
        index.default_candidate_id.as_deref(),
        Some("studio.project.test.rev1.ready")
    );
    assert_eq!(index.candidate_count, 1);
    assert_eq!(index.ready_candidate_count, 1);
    assert_eq!(index.blocked_candidate_count, 0);
    assert_eq!(index.rejected_candidate_count, 0);
    assert_eq!(index.entries[0].candidate_id, ready_candidate.candidate_id);
    assert_eq!(
        index.entries[0].candidate_manifest_path.as_deref(),
        Some(ready_candidate_path.display().to_string().as_str())
    );

    let selection =
        summarize_shell_release_candidate_review_index_selection(&index, Some(&index_path), None);
    assert_eq!(
        selection.schema_id,
        SHELL_RELEASE_CANDIDATE_REVIEW_SELECTION_SCHEMA
    );
    assert_eq!(
        selection.status,
        StudioShellReleaseCandidateReviewSelectionStatus::Selected
    );
    assert_eq!(selection.issue_code, None);
    assert_eq!(
        selection.selected_candidate_id.as_deref(),
        Some("studio.project.test.rev1.ready")
    );
    assert_eq!(selection.ready_candidate_count, 1);
    assert_eq!(selection.entries.len(), 1);
    assert!(selection.entries[0].selected);
    assert!(selection.entries[0].default);

    std::fs::remove_file(
        bundle_root
            .join("studio.graph.phone")
            .join("shells/phone/studio.graph.phone.shell-template.json"),
    )
    .expect("remove phone template manifest");
    let blocked_review = shell_release_candidate_review_for_manifest(
        &manifest,
        Some(&manifest_path),
        &acceptance_index,
        Some(&acceptance_index_path),
        Some("synthetic-ready"),
        &export_package_index,
        Some(&export_package_index_path),
        Some("synthetic-ready-package"),
    );
    let blocked_review_path = root.join("shell-release-candidate-review-blocked.json");
    save_json(&blocked_review_path, &blocked_review).expect("save blocked review");
    let blocked_candidate = shell_release_candidate_review_manifest_for_report(
        &blocked_review,
        &blocked_review_path,
        Some("synthetic-blocked"),
        Some("Synthetic blocked release candidate"),
    );
    let blocked_candidate_path = root.join("shell-release-candidate-blocked-manifest.json");
    save_json(&blocked_candidate_path, &blocked_candidate).expect("save blocked candidate");

    let appended = append_shell_release_candidate_review_index_manifests(
        &index,
        vec![(
            blocked_candidate.clone(),
            Some(blocked_candidate_path.clone()),
        )],
        Some("synthetic-blocked"),
    );
    assert_eq!(appended.candidate_count, 2);
    assert_eq!(appended.ready_candidate_count, 1);
    assert_eq!(appended.blocked_candidate_count, 1);
    assert_eq!(
        appended.default_candidate_id.as_deref(),
        Some("synthetic-blocked")
    );
    assert_eq!(
        select_shell_release_candidate_review_index_entry(&appended, Some("synthetic-blocked"))
            .map(|entry| entry.status),
        Some(StudioShellReleaseCandidateReviewStatus::Blocked)
    );

    let promoted = promote_shell_release_candidate_review_index_default(
        &appended,
        "studio.project.test.rev1.ready",
    )
    .expect("promote ready candidate");
    assert_eq!(
        promoted.default_candidate_id.as_deref(),
        Some("studio.project.test.rev1.ready")
    );
    let missing =
        summarize_shell_release_candidate_review_index_selection(&promoted, None, Some("missing"));
    assert_eq!(
        missing.status,
        StudioShellReleaseCandidateReviewSelectionStatus::Missing
    );
    assert_eq!(
        missing.issue_code.as_deref(),
        Some("studio.issue.shell_release_candidate_review_not_found")
    );
    assert_eq!(missing.selected_candidate_id, None);
}

#[test]
fn desktop_shell_handoff_rejects_unvalidated_bundle() {
    let root = temp_root("desktop-shell-handoff-reject");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();
    let output_dir = root.join("missing-selected-shell");

    let handoff =
        desktop_shell_handoff_for_bundle(&project, Some(&root), "studio.graph.test", &output_dir);

    assert_eq!(handoff.status, StudioValidationStatus::Fail);
    assert_eq!(
        handoff.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert!(handoff.consumer_args.is_empty());
    assert_eq!(handoff.validation.status, StudioValidationStatus::Fail);
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

#[test]
fn retarget_host_profile_updates_host_node_and_bumps_revision() {
    let root = temp_root("retarget-host");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();

    let report = retarget_graph_host_profile(
        &mut project,
        "studio.graph.test",
        "host_run.profile.headset",
        Some(&root),
    );

    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert_eq!(project.revision, 2);
    assert_eq!(
        project.graphs[0].target_host_profile,
        "host_run.profile.headset"
    );
    assert!(project.graphs[0].nodes.iter().any(|node| {
        node.kind == StudioNodeKind::HostProfile && node.reference_id == "host_run.profile.headset"
    }));
    assert_eq!(report.validation.status, StudioValidationStatus::Pass);
    assert!(report
        .changed_fields
        .iter()
        .any(|field| field.ends_with("target_host_profile")));
}

#[test]
fn retarget_host_profile_rejects_missing_graph_without_mutating() {
    let root = temp_root("retarget-missing-graph");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();

    let report = retarget_graph_host_profile(
        &mut project,
        "studio.graph.missing",
        "host_run.profile.headset",
        Some(&root),
    );

    assert_eq!(report.status, StudioEditStatus::Rejected);
    assert_eq!(
        report.issue_code.as_deref(),
        Some("studio.issue.graph_missing")
    );
    assert_eq!(project.revision, 1);
    assert_eq!(
        project.graphs[0].target_host_profile,
        "host_run.profile.desktop"
    );
}

#[test]
fn retarget_host_profile_rejects_undeclared_profile_without_mutating() {
    let root = temp_root("retarget-undeclared-profile");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();

    let report = retarget_graph_host_profile(
        &mut project,
        "studio.graph.test",
        "host_run.profile.missing",
        Some(&root),
    );

    assert_eq!(report.status, StudioEditStatus::Rejected);
    assert_eq!(
        report.issue_code.as_deref(),
        Some("studio.issue.host_profile_reference_missing")
    );
    assert_eq!(project.revision, 1);
    assert_eq!(
        project.graphs[0].target_host_profile,
        "host_run.profile.desktop"
    );
}

#[test]
fn add_module_to_graph_adds_module_edge_and_bumps_revision() {
    let root = temp_root("add-module");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();

    let report = add_module_to_graph(
        &mut project,
        "studio.graph.test",
        "package.synthetic",
        "module.synthetic_provider",
        Some("Synthetic Provider"),
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::AddModule);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.requested_reference_id, "module.synthetic_provider");
    assert_eq!(report.requested_host_profile, "host_run.profile.desktop");
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert_eq!(project.revision, 2);
    assert!(project.graphs[0].nodes.iter().any(|node| {
        node.kind == StudioNodeKind::Module
            && node.reference_id == "module.synthetic_provider"
            && node.label == "Synthetic Provider"
    }));
    assert!(project.graphs[0].edges.iter().any(|edge| {
        edge.kind == StudioEdgeKind::PackageProvidesModule
            && edge.source_node_id == "node.package.synthetic"
            && edge.target_node_id == "node.module.synthetic_provider"
    }));
    assert_eq!(report.validation.status, StudioValidationStatus::Pass);
    assert!(report
        .changed_fields
        .iter()
        .any(|field| field.ends_with("nodes.node.module.synthetic_provider")));
    assert!(report
        .changed_fields
        .iter()
        .any(|field| field.ends_with("edges.edge.package.synthetic.module.synthetic_provider")));
}

#[test]
fn add_next_catalog_module_to_graph_uses_palette_selection() {
    let root = temp_root("add-next-palette-module");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();

    let report = add_next_catalog_module_to_graph(&mut project, "studio.graph.test", Some(&root));

    assert_eq!(report.operation, StudioEditOperation::AddModule);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.requested_reference_id, "module.synthetic_provider");
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert!(project.graphs[0].nodes.iter().any(|node| {
        node.kind == StudioNodeKind::Module && node.reference_id == "module.synthetic_provider"
    }));
    assert!(project.graphs[0].edges.iter().any(|edge| {
        edge.kind == StudioEdgeKind::PackageProvidesModule
            && edge.target_node_id == "node.module.synthetic_provider"
    }));
}

#[test]
fn add_next_catalog_module_from_package_to_graph_uses_selected_package() {
    let root = temp_root("add-next-selected-package-module");
    write_multi_package_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();

    let report = add_next_catalog_module_from_package_to_graph(
        &mut project,
        "studio.graph.test",
        "package.biosignal",
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::AddModule);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.requested_reference_id, "module.biosignal.provider");
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert!(project.graphs[0].nodes.iter().any(|node| {
        node.kind == StudioNodeKind::Package && node.reference_id == "package.biosignal"
    }));
    assert!(project.graphs[0].nodes.iter().any(|node| {
        node.kind == StudioNodeKind::Module && node.reference_id == "module.biosignal.provider"
    }));
    assert!(project.graphs[0].edges.iter().any(|edge| {
        edge.kind == StudioEdgeKind::PackageProvidesModule
            && edge.source_node_id == "node.package.biosignal"
            && edge.target_node_id == "node.module.biosignal.provider"
    }));
    assert!(!project.graphs[0]
        .nodes
        .iter()
        .any(|node| node.reference_id == "module.synthetic_provider"));
}

#[test]
fn add_next_catalog_module_from_package_to_graph_rejects_missing_package() {
    let root = temp_root("add-next-selected-package-missing");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();

    let report = add_next_catalog_module_from_package_to_graph(
        &mut project,
        "studio.graph.test",
        "package.missing",
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::AddModule);
    assert_eq!(report.status, StudioEditStatus::Rejected);
    assert_eq!(
        report.issue_code.as_deref(),
        Some("studio.issue.package_reference_missing")
    );
    assert_eq!(report.requested_reference_id, "package.missing");
    assert_eq!(project.revision, 1);
}

#[test]
fn add_next_catalog_module_to_graph_rejects_when_palette_is_exhausted() {
    let root = temp_root("add-next-palette-module-exhausted");
    write_reference_fixture_tree(&root);
    let mut project = valid_shell_project_with_relative_references();

    let report = add_next_catalog_module_to_graph(&mut project, "studio.graph.test", Some(&root));

    assert_eq!(report.operation, StudioEditOperation::AddModule);
    assert_eq!(report.status, StudioEditStatus::Rejected);
    assert_eq!(
        report.issue_code.as_deref(),
        Some("studio.issue.no_available_palette_module")
    );
    assert_eq!(report.requested_reference_id, NEXT_PALETTE_MODULE_REQUEST);
    assert_eq!(project.revision, 1);
}

#[test]
fn add_module_to_graph_is_idempotent_when_link_exists() {
    let root = temp_root("add-module-idempotent");
    write_reference_fixture_tree(&root);
    let mut project = valid_shell_project_with_relative_references();

    let report = add_module_to_graph(
        &mut project,
        "studio.graph.test",
        "package.synthetic",
        "module.synthetic_provider",
        None,
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::AddModule);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 1);
    assert!(report.changed_fields.is_empty());
    assert_eq!(project.revision, 1);
}

#[test]
fn add_module_to_graph_rejects_unexported_module_without_mutating() {
    let root = temp_root("add-module-unexported");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();

    let report = add_module_to_graph(
        &mut project,
        "studio.graph.test",
        "package.synthetic",
        "module.missing",
        None,
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::AddModule);
    assert_eq!(report.status, StudioEditStatus::Rejected);
    assert_eq!(
        report.issue_code.as_deref(),
        Some("studio.issue.package_module_reference_missing")
    );
    assert_eq!(project.revision, 1);
    assert!(!project.graphs[0]
        .nodes
        .iter()
        .any(|node| node.reference_id == "module.missing"));
}

#[test]
fn remove_module_from_graph_removes_module_and_incident_edges() {
    let root = temp_root("remove-module");
    write_reference_fixture_tree(&root);
    let mut project = valid_shell_project_with_relative_references();

    let report = remove_module_from_graph(
        &mut project,
        "studio.graph.test",
        "module.synthetic_provider",
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::RemoveModule);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.requested_reference_id, "module.synthetic_provider");
    assert_eq!(report.requested_host_profile, "host_run.profile.desktop");
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert_eq!(project.revision, 2);
    assert!(!project.graphs[0]
        .nodes
        .iter()
        .any(|node| node.reference_id == "module.synthetic_provider"));
    assert!(!project.graphs[0].edges.iter().any(|edge| {
        edge.source_node_id == "node.module.synthetic_provider"
            || edge.target_node_id == "node.module.synthetic_provider"
    }));
    assert_eq!(report.validation.status, StudioValidationStatus::Pass);
    assert!(report
        .changed_fields
        .iter()
        .any(|field| field.ends_with("nodes.node.module.synthetic_provider")));
    assert!(report
        .changed_fields
        .iter()
        .any(|field| field.ends_with("edges.edge.package_module")));
    assert!(report
        .changed_fields
        .iter()
        .any(|field| field.ends_with("edges.edge.shell_command")));
}

#[test]
fn remove_module_from_graph_is_idempotent_when_module_is_absent() {
    let root = temp_root("remove-module-idempotent");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();

    let report = remove_module_from_graph(
        &mut project,
        "studio.graph.test",
        "module.synthetic_provider",
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::RemoveModule);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 1);
    assert!(report.changed_fields.is_empty());
    assert_eq!(project.revision, 1);
}

#[test]
fn remove_module_from_graph_rejects_missing_graph_without_mutating() {
    let root = temp_root("remove-module-missing-graph");
    write_reference_fixture_tree(&root);
    let mut project = valid_shell_project_with_relative_references();

    let report = remove_module_from_graph(
        &mut project,
        "studio.graph.missing",
        "module.synthetic_provider",
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::RemoveModule);
    assert_eq!(report.status, StudioEditStatus::Rejected);
    assert_eq!(
        report.issue_code.as_deref(),
        Some("studio.issue.graph_missing")
    );
    assert_eq!(project.revision, 1);
    assert!(project.graphs[0]
        .nodes
        .iter()
        .any(|node| node.reference_id == "module.synthetic_provider"));
}

#[test]
fn add_binding_to_graph_adds_command_binding_and_bumps_revision() {
    let root = temp_root("add-binding");
    write_reference_fixture_tree(&root);
    let mut project = valid_shell_project_with_relative_references();
    project.graphs[0]
        .edges
        .retain(|edge| edge.kind != StudioEdgeKind::CommandBinding);

    let report = add_binding_to_graph(
        &mut project,
        "studio.graph.test",
        StudioBindingKind::Command,
        "node.shell.operator",
        "node.module.synthetic_provider",
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::AddBinding);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(
        report.requested_reference_id,
        "edge.command_binding.node.shell.operator.node.module.synthetic_provider"
    );
    assert_eq!(report.requested_host_profile, "host_run.profile.desktop");
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert_eq!(project.revision, 2);
    assert!(project.graphs[0].edges.iter().any(|edge| {
        edge.kind == StudioEdgeKind::CommandBinding
            && edge.source_node_id == "node.shell.operator"
            && edge.target_node_id == "node.module.synthetic_provider"
    }));
    assert_eq!(report.validation.status, StudioValidationStatus::Pass);
    assert!(report.changed_fields.iter().any(|field| {
        field.ends_with(
            "edges.edge.command_binding.node.shell.operator.node.module.synthetic_provider",
        )
    }));
}

#[test]
fn add_binding_to_graph_is_idempotent_when_binding_exists() {
    let root = temp_root("add-binding-idempotent");
    write_reference_fixture_tree(&root);
    let mut project = valid_shell_project_with_relative_references();

    let report = add_binding_to_graph(
        &mut project,
        "studio.graph.test",
        StudioBindingKind::Command,
        "node.shell.operator",
        "node.module.synthetic_provider",
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::AddBinding);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 1);
    assert!(report.changed_fields.is_empty());
    assert_eq!(project.revision, 1);
}

#[test]
fn add_binding_to_graph_rejects_endpoint_kind_mismatch_without_mutating() {
    let root = temp_root("add-binding-kind-mismatch");
    write_reference_fixture_tree(&root);
    let mut project = valid_shell_project_with_relative_references();

    let report = add_binding_to_graph(
        &mut project,
        "studio.graph.test",
        StudioBindingKind::Stream,
        "node.shell.operator",
        "node.module.synthetic_provider",
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::AddBinding);
    assert_eq!(report.status, StudioEditStatus::Rejected);
    assert_eq!(
        report.issue_code.as_deref(),
        Some("studio.issue.binding_endpoint_kind_mismatch")
    );
    assert_eq!(project.revision, 1);
    assert!(!project.graphs[0].edges.iter().any(|edge| {
        edge.kind == StudioEdgeKind::StreamBinding
            && edge.source_node_id == "node.shell.operator"
            && edge.target_node_id == "node.module.synthetic_provider"
    }));
}

#[test]
fn remove_binding_from_graph_removes_matching_binding() {
    let root = temp_root("remove-binding");
    write_reference_fixture_tree(&root);
    let mut project = valid_shell_project_with_relative_references();

    let report = remove_binding_from_graph(
        &mut project,
        "studio.graph.test",
        StudioBindingKind::Command,
        "node.shell.operator",
        "node.module.synthetic_provider",
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::RemoveBinding);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(
        report.requested_reference_id,
        "edge.command_binding.node.shell.operator.node.module.synthetic_provider"
    );
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 2);
    assert_eq!(project.revision, 2);
    assert!(!project.graphs[0].edges.iter().any(|edge| {
        edge.kind == StudioEdgeKind::CommandBinding
            && edge.source_node_id == "node.shell.operator"
            && edge.target_node_id == "node.module.synthetic_provider"
    }));
    assert_eq!(report.validation.status, StudioValidationStatus::Pass);
    assert!(report
        .changed_fields
        .iter()
        .any(|field| field.ends_with("edges.edge.shell_command")));
}

#[test]
fn remove_binding_from_graph_is_idempotent_when_binding_is_absent() {
    let root = temp_root("remove-binding-idempotent");
    write_reference_fixture_tree(&root);
    let mut project = valid_shell_project_with_relative_references();
    project.graphs[0]
        .edges
        .retain(|edge| edge.kind != StudioEdgeKind::CommandBinding);

    let report = remove_binding_from_graph(
        &mut project,
        "studio.graph.test",
        StudioBindingKind::Command,
        "node.shell.operator",
        "node.module.synthetic_provider",
        Some(&root),
    );

    assert_eq!(report.operation, StudioEditOperation::RemoveBinding);
    assert_eq!(report.status, StudioEditStatus::Applied);
    assert_eq!(report.original_revision, 1);
    assert_eq!(report.resulting_revision, 1);
    assert!(report.changed_fields.is_empty());
    assert_eq!(project.revision, 1);
}

#[test]
fn view_model_projects_graph_rows_for_ui() {
    let root = temp_root("view-model");
    write_reference_fixture_tree(&root);
    let project = valid_project_with_relative_references();
    let model = view_model(&project, Some(&root));
    assert_eq!(model.schema_id, VIEW_MODEL_SCHEMA);
    assert_eq!(model.validation_status, StudioValidationStatus::Pass);
    assert_eq!(model.validation_fail_count, 0);
    assert!(model.validation_issues.is_empty());
    assert!(model.focused_issue.is_none());
    assert_eq!(model.requested_issue_check_id, None);
    assert_eq!(model.selected_issue_index, None);
    assert_eq!(model.selected_issue_check_id, None);
    assert_eq!(model.issue_selection_code, None);
    assert_eq!(model.requested_node_id, None);
    assert_eq!(
        model.selected_node_id.as_deref(),
        Some("node.package.synthetic")
    );
    assert_eq!(model.node_selection_code, None);
    let selected_node = model.selected_node.as_ref().expect("selected node");
    assert_eq!(selected_node.kind, "package");
    assert_eq!(selected_node.reference_id, "package.synthetic");
    assert_eq!(selected_node.reference_status, "resolved");
    assert_eq!(
        selected_node.package_manifest_path.as_deref(),
        Some("packages/synthetic/manifests/package.manifold.json")
    );
    assert_eq!(
        selected_node.package_module_ids,
        vec!["module.synthetic_provider".to_string()]
    );
    assert_eq!(model.requested_edge_id, None);
    assert_eq!(model.selected_edge_id.as_deref(), Some("edge.package_host"));
    assert_eq!(model.edge_selection_code, None);
    let selected_edge = model.selected_edge.as_ref().expect("selected edge");
    assert_eq!(selected_edge.kind, "shell_targets_host_profile");
    assert_eq!(selected_edge.endpoint_status, "endpoints_resolved");
    assert_eq!(
        selected_edge.source_reference_id.as_deref(),
        Some("package.synthetic")
    );
    assert_eq!(
        selected_edge.target_reference_id.as_deref(),
        Some("host_run.profile.desktop")
    );
    assert_eq!(model.graph_count, 1);
    assert_eq!(model.graphs[0].validation_issue_count, 0);
    assert_eq!(model.graphs[0].node_rows[0].kind, "package");
    assert_eq!(model.graphs[0].node_rows[0].validation_issue_count, 0);
    assert_eq!(
        model.graphs[0].edge_rows[0].kind,
        "shell_targets_host_profile"
    );
    assert_eq!(model.graphs[0].edge_rows[0].validation_issue_count, 0);
    let layout = model.graphs[0].layout.as_ref().expect("graph layout");
    assert_eq!(layout.layout_id, "studio.layout.test");
    assert_eq!(layout.coordinate_space, "studio.canvas.logical_2d");
    assert_eq!(layout.node_count, 2);
    assert_eq!(layout.edge_count, 1);
    assert_eq!(layout.nodes[0].node_id, "node.package.synthetic");
    assert_eq!(layout.nodes[0].x, 40);
    assert_eq!(layout.nodes[0].width, 180);
    assert_eq!(layout.nodes[0].validation_issue_count, 0);
    assert_eq!(layout.edges[0].edge_id, "edge.package_host");
    assert_eq!(layout.edges[0].route, "direct");
}

#[test]
fn view_model_includes_selected_shell_preview() {
    let root = temp_root("view-model-shell-preview");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();

    let model = view_model(&project, Some(&root));
    let preview = model.shell_preview.as_ref().expect("shell preview");

    assert_eq!(preview.status, StudioShellDescriptorStatus::Exported);
    assert_eq!(preview.issue_code, None);
    assert_eq!(
        preview.descriptor_id.as_deref(),
        Some("studio.shell_descriptor.studio.graph.test")
    );
    assert_eq!(
        preview.descriptor_path.as_deref(),
        Some("descriptors/studio.graph.test.shell-descriptor.json")
    );
    assert_eq!(
        preview.shell_id.as_deref(),
        Some("shell.synthetic.operator")
    );
    assert_eq!(
        preview.target_host_profile.as_deref(),
        Some("host_run.profile.desktop")
    );
    assert_eq!(preview.target_kind, Some(StudioShellTargetKind::Desktop));
    assert_eq!(preview.package_count, 1);
    assert_eq!(preview.module_count, 1);
    assert_eq!(preview.stream_binding_count, 0);
    assert_eq!(preview.command_binding_count, 1);
    assert_eq!(
        preview.descriptor_validation_status,
        Some(StudioValidationStatus::Pass)
    );
    assert_eq!(
        preview.template_id.as_deref(),
        Some("studio.shell_template.studio.graph.test")
    );
    assert_eq!(
        preview.template_path.as_deref(),
        Some("shells/desktop/studio.graph.test.shell-template.json")
    );
    assert_eq!(
        preview.template_descriptor_path.as_deref(),
        Some("descriptors/studio.graph.test.shell-descriptor.json")
    );
    assert_eq!(
        preview.runtime_command_authority.as_deref(),
        Some("rusty.manifold")
    );
    assert_eq!(
        preview.runtime_host_authority.as_deref(),
        Some("rusty.hostess")
    );
    assert_eq!(
        preview.studio_role.as_deref(),
        Some("authoring.export_planning")
    );
}

#[test]
fn view_model_shell_preview_reports_descriptor_rejection() {
    let root = temp_root("view-model-shell-preview-rejected");
    write_reference_fixture_tree(&root);
    let project = valid_project_with_relative_references();

    let model = view_model(&project, Some(&root));
    let preview = model.shell_preview.as_ref().expect("shell preview");

    assert_eq!(preview.status, StudioShellDescriptorStatus::Rejected);
    assert_eq!(
        preview.issue_code.as_deref(),
        Some("studio.issue.no_operator_shell")
    );
    assert_eq!(preview.descriptor_id, None);
    assert_eq!(preview.template_id, None);
}

#[test]
fn view_model_includes_reference_palette_for_ui() {
    let root = temp_root("view-model-palette");
    write_reference_fixture_tree(&root);
    let project = valid_project_with_relative_references();

    let model = view_model(&project, Some(&root));

    assert_eq!(model.catalog_package_count, 1);
    assert_eq!(model.catalog_module_count, 1);
    assert_eq!(model.host_profile_count, 2);
    assert_eq!(model.catalog_packages.len(), 1);
    assert_eq!(model.catalog_packages[0].package_id, "package.synthetic");
    assert_eq!(
        model.catalog_packages[0].manifest_path,
        "packages/synthetic/manifests/package.manifold.json"
    );
    assert_eq!(
        model.catalog_packages[0].module_ids,
        vec!["module.synthetic_provider".to_string()]
    );
    assert!(model.catalog_packages[0].in_selected_graph);

    let desktop = model
        .host_profiles
        .iter()
        .find(|profile| profile.profile_id == "host_run.profile.desktop")
        .expect("desktop profile");
    assert_eq!(desktop.host_profile.as_deref(), Some("host.desktop"));
    assert_eq!(
        desktop.install_route.as_deref(),
        Some("install.local_process")
    );
    assert!(desktop.targets_selected_graph);

    let headset = model
        .host_profiles
        .iter()
        .find(|profile| profile.profile_id == "host_run.profile.headset")
        .expect("headset profile");
    assert_eq!(headset.host_profile.as_deref(), Some("host.headset"));
    assert!(!headset.targets_selected_graph);
}

#[test]
fn view_model_includes_failed_validation_diagnostics() {
    let root = temp_root("view-model-validation-diagnostics");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();
    project.graphs[0].nodes[0].reference_id = "package.missing".to_string();

    let model = view_model(&project, Some(&root));

    assert_eq!(model.validation_status, StudioValidationStatus::Fail);
    assert!(model.validation_fail_count > 0);
    let issue = model
        .validation_issues
        .iter()
        .find(|issue| issue.issue_code.as_deref() == Some("studio.issue.package_reference_missing"))
        .expect("package reference issue");
    assert_eq!(
        issue.check_id,
        "studio.check.graph.studio.graph.test.package_refs"
    );
    assert!(issue
        .evidence
        .contains("package references missing from catalog"));
    assert_eq!(issue.graph_id.as_deref(), Some("studio.graph.test"));
    assert_eq!(issue.node_ids, vec!["node.package.synthetic".to_string()]);
    assert_eq!(issue.reference_ids, vec!["package.missing".to_string()]);
    assert!(issue.targets_selected_graph);
    assert_eq!(model.graphs[0].validation_issue_count, 1);
    let package_row = model.graphs[0]
        .node_rows
        .iter()
        .find(|node| node.node_id == "node.package.synthetic")
        .expect("package node row");
    assert_eq!(package_row.validation_issue_count, 1);
    let focused_issue = model.focused_issue.expect("focused issue");
    assert_eq!(focused_issue.issue_index, 0);
    assert_eq!(
        focused_issue.check_id,
        "studio.check.graph.studio.graph.test.package_refs"
    );
    assert_eq!(
        focused_issue.issue_code.as_deref(),
        Some("studio.issue.package_reference_missing")
    );
    assert_eq!(focused_issue.graph_id, "studio.graph.test");
    assert_eq!(
        focused_issue.node_id.as_deref(),
        Some("node.package.synthetic")
    );
    assert_eq!(focused_issue.edge_id, None);
    assert_eq!(
        focused_issue.reference_id.as_deref(),
        Some("package.missing")
    );
    assert_eq!(model.requested_issue_check_id, None);
    assert_eq!(model.selected_issue_index, Some(0));
    assert_eq!(
        model.selected_issue_check_id.as_deref(),
        Some("studio.check.graph.studio.graph.test.package_refs")
    );
    assert_eq!(model.issue_selection_code, None);
    assert_eq!(
        model.selected_node_id.as_deref(),
        Some("node.package.synthetic")
    );
    let selected_node = model.selected_node.as_ref().expect("selected node");
    assert_eq!(selected_node.reference_id, "package.missing");
    assert_eq!(selected_node.reference_status, "missing");
    assert_eq!(selected_node.validation_issue_count, 1);
}

#[test]
fn view_model_selects_focused_edge_for_inspector() {
    let root = temp_root("view-model-focused-edge");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();
    project.graphs[0].edges[0].target_node_id = "node.missing".to_string();

    let model = view_model_for_graph_issue_node_and_edge(
        &project,
        Some(&root),
        Some("studio.graph.test"),
        None,
        None,
        None,
    );

    assert_eq!(
        model.selected_issue_check_id.as_deref(),
        Some("studio.check.graph.studio.graph.test.edge.edge.package_host.target")
    );
    assert_eq!(model.selected_edge_id.as_deref(), Some("edge.package_host"));
    let selected_edge = model.selected_edge.as_ref().expect("selected edge");
    assert_eq!(selected_edge.endpoint_status, "missing_target");
    assert_eq!(selected_edge.validation_issue_count, 1);
    assert_eq!(selected_edge.target_node_id, "node.missing");
    assert_eq!(selected_edge.target_kind, None);
}

#[test]
fn view_model_selects_requested_validation_issue() {
    let root = temp_root("view-model-requested-issue");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();
    project.graphs[0].nodes[0].reference_id = "package.missing".to_string();
    project.graphs[0].nodes.push(StudioNode {
        node_id: "node.module.missing".to_string(),
        kind: StudioNodeKind::Module,
        reference_id: "module.missing".to_string(),
        label: "Missing Module".to_string(),
    });

    let model = view_model_for_graph_and_issue(
        &project,
        Some(&root),
        Some("studio.graph.test"),
        Some("studio.check.graph.studio.graph.test.module_refs"),
    );

    assert_eq!(
        model.requested_issue_check_id.as_deref(),
        Some("studio.check.graph.studio.graph.test.module_refs")
    );
    assert_eq!(model.issue_selection_code, None);
    assert_eq!(model.selected_issue_index, Some(1));
    assert_eq!(
        model.selected_issue_check_id.as_deref(),
        Some("studio.check.graph.studio.graph.test.module_refs")
    );
    let focused_issue = model.focused_issue.expect("focused issue");
    assert_eq!(focused_issue.issue_index, 1);
    assert_eq!(
        focused_issue.issue_code.as_deref(),
        Some("studio.issue.module_reference_missing")
    );
    assert_eq!(
        focused_issue.node_id.as_deref(),
        Some("node.module.missing")
    );
    assert_eq!(
        focused_issue.reference_id.as_deref(),
        Some("module.missing")
    );
    assert_eq!(
        model.selected_node_id.as_deref(),
        Some("node.module.missing")
    );
    let selected_node = model.selected_node.as_ref().expect("selected node");
    assert_eq!(selected_node.kind, "module");
    assert_eq!(selected_node.reference_status, "missing");
}

#[test]
fn view_model_selects_requested_node_for_inspector() {
    let root = temp_root("view-model-requested-node");
    write_reference_fixture_tree(&root);
    let project = valid_project_with_relative_references();

    let model = view_model_for_graph_issue_and_node(
        &project,
        Some(&root),
        Some("studio.graph.test"),
        None,
        Some("node.host.desktop"),
    );

    assert_eq!(
        model.requested_node_id.as_deref(),
        Some("node.host.desktop")
    );
    assert_eq!(model.selected_node_id.as_deref(), Some("node.host.desktop"));
    assert_eq!(model.node_selection_code, None);
    let selected_node = model.selected_node.as_ref().expect("selected node");
    assert_eq!(selected_node.kind, "host_profile");
    assert_eq!(selected_node.reference_status, "resolved");
    let profile = selected_node
        .host_profile
        .as_ref()
        .expect("host profile details");
    assert_eq!(profile.profile_id, "host_run.profile.desktop");
    assert_eq!(profile.host_profile.as_deref(), Some("host.desktop"));
    assert_eq!(
        profile.install_route.as_deref(),
        Some("install.local_process")
    );
}

#[test]
fn view_model_reports_missing_requested_node() {
    let root = temp_root("view-model-missing-requested-node");
    write_reference_fixture_tree(&root);
    let project = valid_project_with_relative_references();

    let model = view_model_for_graph_issue_and_node(
        &project,
        Some(&root),
        Some("studio.graph.test"),
        None,
        Some("node.missing"),
    );

    assert_eq!(model.requested_node_id.as_deref(), Some("node.missing"));
    assert_eq!(
        model.node_selection_code.as_deref(),
        Some("studio.issue.node_selection_missing")
    );
    assert_eq!(
        model.selected_node_id.as_deref(),
        Some("node.package.synthetic")
    );
}

#[test]
fn view_model_selects_requested_edge_for_inspector() {
    let root = temp_root("view-model-requested-edge");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();

    let model = view_model_for_graph_issue_node_and_edge(
        &project,
        Some(&root),
        Some("studio.graph.test"),
        None,
        None,
        Some("edge.shell_command"),
    );

    assert_eq!(
        model.requested_edge_id.as_deref(),
        Some("edge.shell_command")
    );
    assert_eq!(
        model.selected_edge_id.as_deref(),
        Some("edge.shell_command")
    );
    assert_eq!(model.edge_selection_code, None);
    let selected_edge = model.selected_edge.as_ref().expect("selected edge");
    assert_eq!(selected_edge.kind, "command_binding");
    assert_eq!(selected_edge.binding_kind.as_deref(), Some("command"));
    assert_eq!(selected_edge.endpoint_status, "endpoints_resolved");
    assert_eq!(
        selected_edge.source_reference_id.as_deref(),
        Some("shell.synthetic.operator")
    );
    assert_eq!(
        selected_edge.target_reference_id.as_deref(),
        Some("module.synthetic_provider")
    );
}

#[test]
fn view_model_reports_missing_requested_edge() {
    let root = temp_root("view-model-missing-requested-edge");
    write_reference_fixture_tree(&root);
    let project = valid_project_with_relative_references();

    let model = view_model_for_graph_issue_node_and_edge(
        &project,
        Some(&root),
        Some("studio.graph.test"),
        None,
        None,
        Some("edge.missing"),
    );

    assert_eq!(model.requested_edge_id.as_deref(), Some("edge.missing"));
    assert_eq!(
        model.edge_selection_code.as_deref(),
        Some("studio.issue.edge_selection_missing")
    );
    assert_eq!(model.selected_edge_id.as_deref(), Some("edge.package_host"));
}

#[test]
fn view_model_reports_missing_requested_validation_issue() {
    let root = temp_root("view-model-missing-requested-issue");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();
    project.graphs[0].nodes[0].reference_id = "package.missing".to_string();

    let model = view_model_for_graph_and_issue(
        &project,
        Some(&root),
        Some("studio.graph.test"),
        Some("studio.check.graph.studio.graph.test.missing"),
    );

    assert_eq!(
        model.requested_issue_check_id.as_deref(),
        Some("studio.check.graph.studio.graph.test.missing")
    );
    assert_eq!(
        model.issue_selection_code.as_deref(),
        Some("studio.issue.validation_issue_selection_missing")
    );
    assert_eq!(model.selected_issue_index, Some(0));
    assert_eq!(
        model.selected_issue_check_id.as_deref(),
        Some("studio.check.graph.studio.graph.test.package_refs")
    );
}

#[test]
fn view_model_selects_requested_graph() {
    let root = temp_root("view-model-select");
    write_reference_fixture_tree(&root);
    let mut project = valid_project_with_relative_references();
    let mut second = project.graphs[0].clone();
    second.graph_id = "studio.graph.second".to_string();
    second.display_name = "Second Graph".to_string();
    project.graphs.push(second);

    let model = view_model_for_graph(&project, Some(&root), Some("studio.graph.second"));
    assert_eq!(model.graph_count, 2);
    assert_eq!(
        model.requested_graph_id.as_deref(),
        Some("studio.graph.second")
    );
    assert_eq!(model.selected_graph_index, Some(1));
    assert_eq!(
        model.selected_graph_id.as_deref(),
        Some("studio.graph.second")
    );
    assert_eq!(model.selection_issue_code, None);
}

#[test]
fn view_model_reports_missing_requested_graph() {
    let root = temp_root("view-model-missing-select");
    write_reference_fixture_tree(&root);
    let project = valid_project_with_relative_references();

    let model = view_model_for_graph(&project, Some(&root), Some("studio.graph.missing"));
    assert_eq!(
        model.requested_graph_id.as_deref(),
        Some("studio.graph.missing")
    );
    assert_eq!(model.selected_graph_index, None);
    assert_eq!(model.selected_graph_id, None);
    assert_eq!(
        model.selection_issue_code.as_deref(),
        Some("studio.issue.graph_selection_missing")
    );
}
