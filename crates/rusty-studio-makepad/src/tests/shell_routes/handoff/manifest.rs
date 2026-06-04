use super::super::*;

#[test]
fn shell_handoff_manifest_writes_durable_artifact() {
    let root = temp_root("shell-handoff-manifest");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");

    let (manifest, output_path) = write_shell_handoff_manifest_for_project_source(&project_path)
        .expect("write handoff manifest");

    assert!(output_path.is_file());
    assert_eq!(manifest.schema_id, "rusty.studio.shell_handoff_manifest.v1");
    assert_eq!(manifest.status, StudioValidationStatus::Pass);
    assert_eq!(manifest.graph_count, 1);
    assert_eq!(manifest.ready_count, 1);
    assert_eq!(manifest.failed_count, 0);
    assert_eq!(manifest.missing_bundle_count, 0);
    assert_eq!(manifest.targets.len(), 1);
    assert_eq!(manifest.handoffs.len(), 1);
    assert_eq!(
        manifest.handoffs[0].consumer_id,
        "rusty-studio-desktop-shell"
    );
    assert!(manifest.handoffs[0]
        .template_index_path
        .ends_with("shell-templates.json"));
    let written = std::fs::read_to_string(&output_path).expect("read handoff manifest");
    assert!(written.contains("\"$schema\": \"rusty.studio.shell_handoff_manifest.v1\""));
    let status = shell_handoff_manifest_status(&manifest, &output_path);
    assert!(status.contains("handoff manifest pass"));
    assert!(status.contains("ready 1/1"));
    assert!(status.contains("failed 0; missing 0"));
    assert!(status.contains("rusty.manifold / rusty.hostess / authoring.export_planning"));
    assert!(status.contains("desktop: ready 1/1; failed 0; missing 0"));
}
