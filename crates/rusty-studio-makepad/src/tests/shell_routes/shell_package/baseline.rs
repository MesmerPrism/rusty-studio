use super::super::*;

#[test]
fn shell_export_package_baseline_writes_durable_artifact() {
    let root = temp_root("shell-export-package-baseline");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");

    let (report, baseline, index, package_path, baseline_path, index_path, bundle_root) =
        write_shell_export_package_baseline_for_project_source(&project_path)
            .expect("write export package baseline");

    assert!(package_path.is_file());
    assert!(baseline_path.is_file());
    assert!(index_path.is_file());
    assert_eq!(report.status, StudioShellExportPackageStatus::Ready);
    assert_eq!(
        baseline.schema_id,
        "rusty.studio.shell_export_package_baseline_manifest.v1"
    );
    assert_eq!(
        baseline.baseline_id,
        "studio.project.makepad_edit.rev1.ready"
    );
    assert_eq!(baseline.package_path, package_path.display().to_string());
    assert_eq!(
        baseline.package_schema,
        "rusty.studio.shell_export_package_report.v1"
    );
    assert_eq!(baseline.project_id, "studio.project.makepad_edit");
    assert_eq!(baseline.project_revision, 1);
    assert_eq!(baseline.status, StudioShellExportPackageStatus::Ready);
    assert_eq!(baseline.ready_count, 1);
    assert_eq!(baseline.blocked_count, 0);
    assert_eq!(baseline.rejected_count, 0);
    assert_eq!(baseline.descriptor_count, 1);
    assert_eq!(baseline.template_manifest_count, 1);
    assert_eq!(baseline.runbook_entry_count, 1);
    assert_eq!(baseline.target_count, 1);
    assert_eq!(baseline.execution_policy, "not_executed.review_only");
    assert_eq!(baseline.command_session_authority, "rusty.manifold");
    assert_eq!(baseline.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(baseline.studio_role, "authoring.export_planning");
    assert_eq!(
        index.schema_id,
        "rusty.studio.shell_export_package_baseline_index.v1"
    );
    assert_eq!(
        index.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );
    assert_eq!(index.baseline_count, 1);
    assert_eq!(index.ready_baseline_count, 1);
    assert_eq!(index.blocked_baseline_count, 0);
    assert_eq!(
        index.entries[0].package_path,
        package_path.display().to_string()
    );
    let written = std::fs::read_to_string(&baseline_path).expect("read baseline manifest");
    assert!(
        written.contains("\"$schema\": \"rusty.studio.shell_export_package_baseline_manifest.v1\"")
    );
    let status = shell_export_package_baseline_status(
        &report,
        &baseline,
        &index,
        &package_path,
        &baseline_path,
        &index_path,
        &bundle_root,
    );
    assert!(status.contains("export package baseline written"));
    assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
    assert!(status.contains("export package baseline selection selected"));
    assert!(status.contains("export package baseline index slots 1"));
    assert!(status.contains("shell export package ready"));
}

#[test]
fn shell_export_package_baseline_appends_and_cycles_default() {
    let root = temp_root("shell-export-package-baseline-cycle");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");
    let (_, ready_baseline, _, _, ready_baseline_path, index_path, _) =
        write_shell_export_package_baseline_for_project_source(&project_path)
            .expect("write initial package baseline");
    let (_, archived_baseline, archived_index, _, archived_baseline_path, _, _) =
        append_shell_export_package_baseline_for_project_source(&project_path)
            .expect("append package baseline");

    assert_eq!(
        archived_baseline.baseline_id,
        "studio.project.makepad_edit.rev1.ready.archive2"
    );
    assert_eq!(
        archived_index.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready.archive2")
    );
    assert_eq!(archived_index.baseline_count, 2);
    assert_eq!(archived_index.ready_baseline_count, 2);
    assert_eq!(
        archived_index.entries[1].baseline_manifest_path.as_deref(),
        Some(archived_baseline_path.display().to_string().as_str())
    );

    let (selected_ready_baseline, selected_ready_index, selected_ready_path, loaded_index_path) =
        select_next_shell_export_package_baseline_default_for_project_source(&project_path)
            .expect("select next package baseline default");
    assert_eq!(selected_ready_baseline, ready_baseline);
    assert_eq!(selected_ready_path, ready_baseline_path);
    assert_eq!(loaded_index_path, index_path);
    assert_eq!(
        selected_ready_index.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );
    let status = shell_export_package_baseline_select_status(
        &selected_ready_baseline,
        &selected_ready_index,
        &selected_ready_path,
        &loaded_index_path,
    );
    assert!(status.contains("export package baseline default selected"));
    assert!(status.contains(
            "export package baseline selection selected; requested studio.project.makepad_edit.rev1.ready; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));
    assert!(status.contains("selected yes; default yes"));

    let (promoted_baseline, promoted_index, promoted_path, loaded_index_path) =
        promote_shell_export_package_baseline_default_for_project_source(&project_path)
            .expect("promote saved package baseline");
    assert_eq!(promoted_baseline, ready_baseline);
    assert_eq!(promoted_path, ready_baseline_path);
    assert_eq!(
        promoted_index.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );
    let written_index =
        load_shell_export_package_baseline_index(&loaded_index_path).expect("load index");
    assert_eq!(written_index, promoted_index);
    let status = shell_export_package_baseline_promote_status(
        &promoted_baseline,
        &promoted_index,
        &promoted_path,
        &loaded_index_path,
    );
    assert!(status.contains("export package baseline default promoted"));
    assert!(status.contains("export package baseline index slots 2"));
    assert!(status.contains("studio.project.makepad_edit.rev1.ready.archive2 [ready]"));
}
