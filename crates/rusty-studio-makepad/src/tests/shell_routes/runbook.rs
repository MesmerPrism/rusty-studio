use super::*;

#[test]
fn shell_runbook_reports_owner_routes_from_makepad_route() {
    let root = temp_root("shell-runbook");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");

    let (report, bundle_root) =
        shell_runbook_for_project_source(&project_path).expect("inspect shell runbook");

    assert_eq!(report.schema_id, "rusty.studio.shell_runbook_report.v1");
    assert_eq!(report.status, StudioShellRunbookStatus::Ready);
    assert_eq!(report.ready_count, 1);
    assert_eq!(report.blocked_count, 0);
    assert_eq!(report.rejected_count, 0);
    assert_eq!(report.entries.len(), 1);
    assert_eq!(report.prohibited_actions.len(), 4);
    let entry = &report.entries[0];
    assert_eq!(entry.status, StudioShellRunbookStatus::Ready);
    assert_eq!(entry.responsible_owner, "rusty.hostess");
    assert_eq!(entry.execution_policy, "not_executed.request_only");
    assert_eq!(entry.command_session_authority, "rusty.manifold");
    assert_eq!(entry.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(entry.studio_role, "authoring.export_planning");
    assert_eq!(entry.consumer_id, "rusty-studio-desktop-shell");
    assert_eq!(entry.runtime_route_kind, "desktop_operator_shell");
    assert_eq!(
        entry.host_routes.install_route.as_deref(),
        Some("install.local_process")
    );
    assert_eq!(
        entry.host_routes.launch_route.as_deref(),
        Some("launch.local_process")
    );
    assert_eq!(
        entry.host_routes.command_bridge.as_deref(),
        Some("bridge.local_cli")
    );
    assert_eq!(
        entry.host_routes.evidence_pull_route.as_deref(),
        Some("evidence.filesystem")
    );
    assert!(entry
        .cli_request
        .iter()
        .any(|arg| arg == "rusty-studio-desktop-shell"));
    assert!(entry.cli_request.iter().any(|arg| arg == "--templates"));

    let status = shell_runbook_status(&report, &bundle_root);
    assert!(status.contains("shell runbook ready"));
    assert!(status.contains("owner rusty.hostess"));
    assert!(status.contains("not_executed.request_only"));
    assert!(status.contains("install.local_process"));
    assert!(status.contains("launch.local_process"));
    assert!(status.contains("bridge.local_cli"));
    assert!(status.contains("evidence.filesystem"));
    assert!(status.contains("rusty-studio-desktop-shell"));
}
