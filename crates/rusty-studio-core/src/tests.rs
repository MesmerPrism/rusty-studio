use super::*;

mod graph_edit_tests;
mod projected_motion_breath_tests;

mod shell_contract_tests;
mod shell_export_package_tests;
mod shell_handoff_acceptance_tests;
mod shell_handoff_tests;
mod shell_hostess_workflow_tests;
mod view_model_tests;
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
