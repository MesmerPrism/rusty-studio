use super::super::*;

#[test]
fn shell_handoff_acceptance_reports_ready_from_makepad_route() {
    let root = temp_root("shell-handoff-acceptance");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");

    let (report, bundle_root) = shell_handoff_acceptance_for_project_source(&project_path)
        .expect("review acceptance checklist");

    assert_eq!(
        report.schema_id,
        "rusty.studio.shell_handoff_acceptance_checklist.v1"
    );
    assert_eq!(report.status, StudioShellHandoffAcceptanceStatus::Ready);
    assert_eq!(report.ready_count, 1);
    assert_eq!(report.blocked_count, 0);
    assert_eq!(report.rejected_count, 0);
    assert_eq!(report.entries.len(), 1);
    assert_eq!(
        report.prohibited_actions,
        vec![
            "install".to_string(),
            "launch".to_string(),
            "open_command_session".to_string(),
            "collect_device_evidence".to_string(),
        ]
    );
    assert!(report.entries[0]
        .checks
        .iter()
        .any(|check| check.owner == "rusty.manifold"));
    assert!(report.entries[0]
        .checks
        .iter()
        .any(|check| check.owner == "rusty.hostess"));
    assert!(report.entries[0]
        .checks
        .iter()
        .any(|check| check.owner == "rusty.studio"));

    let status = shell_handoff_acceptance_status(&report, &bundle_root);
    assert!(status.contains("handoff acceptance ready"));
    assert!(status.contains("ready 1; blocked 0; rejected 0"));
    assert!(status
        .contains("prohibited: install, launch, open_command_session, collect_device_evidence"));
    assert!(status.contains("studio.graph.makepad_edit [desktop]"));
    assert!(status.contains("route desktop_operator_shell"));
    assert!(status.contains("owners rusty.manifold:pass, rusty.hostess:pass, rusty.studio:pass"));
}

#[test]
fn shell_handoff_acceptance_blocks_missing_bundle_from_makepad_route() {
    let root = temp_root("shell-handoff-acceptance-missing");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");

    let (report, bundle_root) = shell_handoff_acceptance_for_project_source(&project_path)
        .expect("review missing acceptance checklist");

    assert_eq!(report.status, StudioShellHandoffAcceptanceStatus::Blocked);
    assert_eq!(report.ready_count, 0);
    assert_eq!(report.blocked_count, 1);
    assert_eq!(report.rejected_count, 0);
    assert_eq!(report.entries.len(), 1);
    assert_eq!(
        report.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert!(report.entries[0]
        .checks
        .iter()
        .any(|check| check.issue_code.as_deref()
            == Some("studio.issue.shell_handoff_acceptance_blocked")));
    let failed_check_count = report.entries[0]
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    assert!(failed_check_count > 0);

    let status = shell_handoff_acceptance_status(&report, &bundle_root);
    assert!(status.contains("handoff acceptance blocked"));
    assert!(status.contains("ready 0; blocked 1; rejected 0"));
    assert!(status.contains("issue studio.issue.shell_bundle_file_missing"));
    assert!(status.contains(&format!("failed {failed_check_count}")));
}
