use super::*;

#[test]
fn selected_shell_bundle_export_writes_preview_files() {
    let root = temp_root("selected-shell-bundle-export");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");

    let (report, output_dir) = export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");

    assert_eq!(report.status, StudioShellBundleStatus::Exported);
    assert_eq!(
        report.bundle_files,
        vec![
            "descriptors/studio.graph.makepad_edit.manifold-shell-handoff.json".to_string(),
            "descriptors/studio.graph.makepad_edit.shell-descriptor.json".to_string(),
            "shell-artifacts.json".to_string(),
            "shell-templates.json".to_string(),
            "shells/desktop/studio.graph.makepad_edit.shell-template.json".to_string(),
        ]
    );
    for relative_path in &report.bundle_files {
        let path = relative_path
            .split('/')
            .fold(output_dir.clone(), |path, segment| path.join(segment));
        assert!(path.is_file(), "missing {}", path.display());
    }
    let manifest =
        rusty_studio_core::load_shell_artifact_manifest(&output_dir.join("shell-artifacts.json"))
            .expect("load shell artifacts manifest");
    assert_eq!(
        rusty_studio_core::validate_shell_artifact_manifest(&manifest, Some(&output_dir)).status,
        StudioValidationStatus::Pass
    );
    let index =
        rusty_studio_core::load_shell_template_index(&output_dir.join("shell-templates.json"))
            .expect("load shell template index");
    assert_eq!(
        rusty_studio_core::validate_shell_template_index(&index, Some(&output_dir)).status,
        StudioValidationStatus::Pass
    );
    let status = shell_bundle_export_status(&report, &output_dir);
    assert!(status.contains("exported; issue none"));
    assert!(status.contains("studio.graph.makepad_edit"));
    assert!(status.contains("shells/desktop/studio.graph.makepad_edit.shell-template.json"));
}

#[test]
fn selected_shell_bundle_validation_reports_pass() {
    let root = temp_root("selected-shell-bundle-validate");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");

    let (report, output_dir) = validate_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("validate selected shell bundle");

    assert_eq!(report.status, StudioValidationStatus::Pass);
    assert!(report
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
    let status = shell_bundle_validation_status(&report, &output_dir);
    assert!(status.contains("validated; status pass"));
    assert!(status.contains("studio.graph.makepad_edit"));
    assert!(status.contains("files: 5"));
}
