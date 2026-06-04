use super::super::*;

#[test]
fn shell_handoff_reports_ready_command_args() {
    let root = temp_root("desktop-shell-handoff");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");

    let (report, output_dir) =
        shell_handoff_for_project_source(&project_path, &model, 0).expect("prepare shell handoff");

    assert_eq!(report.status, StudioValidationStatus::Pass);
    assert_eq!(report.consumer_id, "rusty-studio-desktop-shell");
    assert!(report
        .consumer_args
        .iter()
        .any(|arg| arg.ends_with("shell-templates.json")));
    let status = shell_handoff_status(&report, &output_dir);
    assert!(status.contains("shell handoff pass"));
    assert!(status.contains("rusty-studio-desktop-shell"));
    assert!(status.contains("target: desktop"));
    assert!(status.contains("--templates"));
    assert!(status.contains("rusty.manifold / rusty.hostess / authoring.export_planning"));
}

#[test]
fn shell_handoff_readiness_reports_exported_graph() {
    let root = temp_root("shell-handoff-readiness");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");

    let (report, bundle_root) = shell_handoff_readiness_for_project_source(&project_path)
        .expect("inspect handoff readiness");

    assert_eq!(report.status, StudioValidationStatus::Pass);
    assert_eq!(report.graph_count, 1);
    assert_eq!(report.ready_count, 1);
    assert_eq!(report.failed_count, 0);
    assert_eq!(report.missing_bundle_count, 0);
    assert_eq!(report.target_summaries.len(), 1);
    assert_eq!(report.target_summaries[0].ready_count, 1);
    assert_eq!(report.target_summaries[0].graph_count, 1);
    assert_eq!(report.target_summaries[0].bundle_dirs.len(), 1);
    assert_eq!(report.target_summaries[0].ready_bundle_dirs.len(), 1);
    assert!(report.target_summaries[0].failed_bundle_dirs.is_empty());
    assert!(report.target_summaries[0].missing_bundle_dirs.is_empty());
    assert_eq!(report.target_summaries[0].template_index_paths.len(), 1);
    assert_eq!(report.entries.len(), 1);
    assert_eq!(
        report.entries[0].export_bundle_id,
        "studio.export.studio.graph.makepad_edit"
    );
    assert_eq!(report.entries[0].consumer_id, "rusty-studio-desktop-shell");
    assert_eq!(report.entries[0].package_count, 1);
    assert_eq!(report.entries[0].module_count, 0);
    assert_eq!(report.entries[0].operator_shell_count, 1);
    assert_eq!(report.entries[0].failed_check_count, 0);
    let status = shell_handoff_readiness_status(&report, &bundle_root);
    assert!(status.contains("handoff readiness pass"));
    assert!(status.contains("ready 1/1"));
    assert!(status.contains("failed 0; missing 0"));
    assert!(status.contains("desktop: ready 1/1; missing 0"));
    assert!(status.contains("templates "));
    assert!(status.contains("shell-templates.json"));
    assert!(status.contains("studio.graph.makepad_edit [desktop]"));
    assert!(status.contains("profile host_run.profile.desktop"));
    assert!(status.contains("packages 1; modules 0; shell 1"));
}
