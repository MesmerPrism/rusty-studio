use super::super::super::*;

#[test]
fn shell_handoff_acceptance_baseline_summary_reports_revision_metadata() {
    let root = temp_root("shell-handoff-acceptance-baseline-summary");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");
    let (_, expected_baseline, expected_index, checklist_path, baseline_path, index_path, _) =
        write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("write acceptance baseline");

    let (baseline, index, loaded_path, loaded_index_path) =
        shell_handoff_acceptance_baseline_summary_for_project_source(&project_path)
            .expect("summarize acceptance baseline");

    assert_eq!(loaded_path, baseline_path);
    assert_eq!(loaded_index_path, index_path);
    assert_eq!(baseline, expected_baseline);
    assert_eq!(index, expected_index);
    assert_eq!(
        baseline.schema_id,
        "rusty.studio.shell_handoff_acceptance_baseline_manifest.v1"
    );
    assert_eq!(
        baseline.baseline_id,
        "studio.project.makepad_edit.rev1.ready"
    );
    assert_eq!(
        baseline.checklist_path,
        checklist_path.display().to_string()
    );
    let summary = &baseline.summary;
    assert_eq!(
        summary.schema_id,
        "rusty.studio.shell_handoff_acceptance_summary.v1"
    );
    assert_eq!(
        summary.checklist_schema,
        "rusty.studio.shell_handoff_acceptance_checklist.v1"
    );
    assert_eq!(summary.project_id, "studio.project.makepad_edit");
    assert_eq!(summary.project_revision, 1);
    assert_eq!(summary.status, StudioShellHandoffAcceptanceStatus::Ready);
    assert_eq!(summary.ready_count, 1);
    assert_eq!(summary.blocked_count, 0);
    assert_eq!(summary.rejected_count, 0);
    assert_eq!(summary.entry_count, 1);
    assert_eq!(summary.targets.len(), 1);
    assert_eq!(
        summary.targets[0].target_kind,
        StudioShellTargetKind::Desktop
    );
    assert_eq!(
        summary.targets[0].consumer_ids,
        vec!["rusty-studio-desktop-shell"]
    );
    assert_eq!(
        summary.targets[0].route_kinds,
        vec!["desktop_operator_shell"]
    );
    let status = shell_handoff_acceptance_summary_status(
        &baseline,
        &index,
        &loaded_path,
        &loaded_index_path,
    );
    assert!(status.contains("acceptance baseline summary ready"));
    assert!(status.contains("baseline studio.project.makepad_edit.rev1.ready"));
    assert!(status.contains("project studio.project.makepad_edit rev 1"));
    assert!(status.contains("manifest studio.shell_handoffs.studio.project.makepad_edit"));
    assert!(status.contains(&format!("identity: {}", baseline_path.display())));
    assert!(status.contains(&format!("checklist: {}", checklist_path.display())));
    assert!(status.contains(&format!("index: {}", index_path.display())));
    assert!(status.contains(
            "baseline selection selected; requested none; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));
    assert!(status.contains("baseline index slots 1"));
    assert!(status.contains("desktop: ready 1/1; blocked 0; rejected 0"));
    assert!(status.contains("consumers rusty-studio-desktop-shell"));
    assert!(status.contains("routes desktop_operator_shell"));
}
