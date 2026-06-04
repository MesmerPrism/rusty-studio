use super::super::*;

#[test]
fn shell_export_package_reports_descriptor_template_and_runbook_from_makepad_route() {
    let root = temp_root("shell-export-package");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");

    let (report, bundle_root) =
        shell_export_package_for_project_source(&project_path).expect("review export package");

    assert_eq!(
        report.schema_id,
        "rusty.studio.shell_export_package_report.v1"
    );
    assert_eq!(report.status, StudioShellExportPackageStatus::Ready);
    assert_eq!(report.ready_count, 1);
    assert_eq!(report.blocked_count, 0);
    assert_eq!(report.rejected_count, 0);
    assert_eq!(report.descriptor_count, 1);
    assert_eq!(report.template_manifest_count, 1);
    assert_eq!(report.runbook_entry_count, 1);
    assert_eq!(report.execution_policy, "not_executed.review_only");
    assert_eq!(report.review_owner, "rusty.hostess");
    assert_eq!(report.command_session_authority, "rusty.manifold");
    assert_eq!(report.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(report.studio_role, "authoring.export_planning");
    let entry = &report.entries[0];
    assert_eq!(entry.status, StudioShellExportPackageStatus::Ready);
    assert_eq!(entry.responsible_owner, "rusty.hostess");
    assert_eq!(entry.next_required_action, "review_with_runtime_owner");
    assert_eq!(entry.runtime_route_kind, "desktop_operator_shell");
    assert!(entry.descriptor.is_some());
    assert!(entry.template_manifest.is_some());
    assert!(entry
        .runbook_cli_request
        .iter()
        .any(|arg| arg == "rusty-studio-desktop-shell"));
    assert!(entry
        .runbook_cli_request
        .iter()
        .any(|arg| arg == "--templates"));

    let status = shell_export_package_status(&report, &bundle_root);
    assert!(status.contains("shell export package ready"));
    assert!(status.contains("descriptors 1; templates 1"));
    assert!(status.contains("owner: rusty.hostess"));
    assert!(status.contains("not_executed.review_only"));
    assert!(status.contains("review_with_runtime_owner"));
    assert!(status.contains("studio.shell_descriptor.studio.graph.makepad_edit"));
    assert!(status.contains("studio.shell_template.studio.graph.makepad_edit"));
}
