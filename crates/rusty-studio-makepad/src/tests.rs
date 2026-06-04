use super::*;
use crate::canvas::{CanvasViewport, StudioGraphCanvasHit};
use crate::project_actions::add_module_to_project_source;
use rusty_studio_core::save_project;
use rusty_studio_model::{
    StudioEdge, StudioEdgeKind, StudioEdgeLayout, StudioEdgeRouteKind, StudioEditOperation,
    StudioEditStatus, StudioGraph, StudioGraphLayout, StudioNode, StudioNodeKind, StudioNodeLayout,
    StudioProject, StudioShellExportPackageBaselineSelectionStatus,
    StudioShellExportPackageComparisonChange, StudioShellExportPackageComparisonStatus,
    StudioShellExportPackageStatus, StudioShellHandoffAcceptanceBaselineSelectionStatus,
    StudioShellHandoffAcceptanceComparisonChange, StudioShellHandoffAcceptanceComparisonStatus,
    StudioShellHandoffAcceptanceStatus, StudioShellHostessHandoffPackageActionStatus,
    StudioShellHostessHandoffPackageStatus, StudioShellHostessOwnerIntakeAssignmentStatus,
    StudioShellHostessOwnerIntakeStatus, StudioShellHostessStagingAcceptanceComparisonChange,
    StudioShellHostessStagingAcceptanceComparisonStatus, StudioShellHostessStagingAcceptanceStatus,
    StudioShellHostessStagingExecutionActionStatus,
    StudioShellHostessStagingExecutionRequestStatus, StudioShellHostessStagingFilePlanStatus,
    StudioShellHostessStagingFileRequestStatus, StudioShellHostessStagingHandoffEnvelopeStatus,
    StudioShellHostessStagingHandoffInstructionStatus, StudioShellHostessStagingPreviewGroupStatus,
    StudioShellHostessStagingPreviewStatus, StudioShellReleaseCandidateReviewStatus,
    StudioShellRunbookStatus, StudioShellTargetKind, StudioValidationStatus, PROJECT_SCHEMA,
};

mod edit_bindings;
mod edit_graph;
mod path_session;
mod shell_routes;
mod view_model_text;

fn temp_root(name: &str) -> PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before unix epoch")
        .as_nanos();
    let root = std::env::temp_dir().join(format!("rusty-studio-makepad-{name}-{unique}"));
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
}

fn editable_project() -> StudioProject {
    StudioProject {
        schema_id: PROJECT_SCHEMA.to_string(),
        project_id: "studio.project.makepad_edit".to_string(),
        revision: 1,
        display_name: "Makepad Edit".to_string(),
        package_catalog_path: "packages/catalog.manifold.json".to_string(),
        host_run_profile_paths: vec![
            "profiles/desktop.json".to_string(),
            "profiles/headset.json".to_string(),
        ],
        graphs: vec![StudioGraph {
            graph_id: "studio.graph.makepad_edit".to_string(),
            display_name: "Makepad Edit Graph".to_string(),
            target_host_profile: "host_run.profile.desktop".to_string(),
            nodes: vec![
                StudioNode {
                    node_id: "node.package.synthetic".to_string(),
                    kind: StudioNodeKind::Package,
                    reference_id: "package.synthetic".to_string(),
                    label: "Package".to_string(),
                },
                StudioNode {
                    node_id: "node.host.profile".to_string(),
                    kind: StudioNodeKind::HostProfile,
                    reference_id: "host_run.profile.desktop".to_string(),
                    label: "Host".to_string(),
                },
                StudioNode {
                    node_id: "node.shell.operator".to_string(),
                    kind: StudioNodeKind::OperatorShell,
                    reference_id: "shell.synthetic.operator".to_string(),
                    label: "Shell".to_string(),
                },
            ],
            edges: vec![StudioEdge {
                edge_id: "edge.shell_host".to_string(),
                kind: StudioEdgeKind::ShellTargetsHostProfile,
                source_node_id: "node.shell.operator".to_string(),
                target_node_id: "node.host.profile".to_string(),
            }],
            layout: Some(StudioGraphLayout {
                layout_id: "studio.layout.makepad_edit".to_string(),
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
                        node_id: "node.shell.operator".to_string(),
                        x: 320,
                        y: 40,
                        width: 180,
                        height: 72,
                    },
                    StudioNodeLayout {
                        node_id: "node.host.profile".to_string(),
                        x: 600,
                        y: 40,
                        width: 180,
                        height: 72,
                    },
                ],
                edges: vec![StudioEdgeLayout {
                    edge_id: "edge.shell_host".to_string(),
                    route: StudioEdgeRouteKind::Direct,
                }],
            }),
        }],
    }
}
