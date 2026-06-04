use super::super::super::*;

#[test]
fn shell_handoff_acceptance_baseline_appends_history_entry() {
    let root = temp_root("shell-handoff-acceptance-baseline-append");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");
    let (_, saved_baseline, saved_index, _, saved_baseline_path, index_path, _) =
        write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("write initial baseline");
    assert_eq!(
        saved_index.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );

    let (
        report,
        archived_baseline,
        archived_index,
        checklist_path,
        baseline_path,
        loaded_index_path,
        bundle_root,
    ) = append_shell_handoff_acceptance_baseline_for_project_source(&project_path)
        .expect("append baseline history entry");

    assert_eq!(loaded_index_path, index_path);
    assert_eq!(
        archived_baseline.baseline_id,
        "studio.project.makepad_edit.rev1.ready.archive2"
    );
    assert_eq!(
        archived_baseline.label,
        "studio.project.makepad_edit revision 1 ready acceptance baseline archive 2"
    );
    assert!(checklist_path.ends_with(
            "target/studio-shell-handoffs/baselines/studio.project.makepad_edit.rev1.ready.archive2.checklist.json"
        ));
    assert!(baseline_path.ends_with(
            "target/studio-shell-handoffs/baselines/studio.project.makepad_edit.rev1.ready.archive2.baseline.json"
        ));
    assert!(checklist_path.is_file());
    assert!(baseline_path.is_file());
    assert_eq!(
        archived_baseline.checklist_path,
        checklist_path.display().to_string()
    );
    assert_eq!(
        archived_index.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready.archive2")
    );
    assert_eq!(archived_index.baseline_count, 2);
    assert_eq!(archived_index.ready_baseline_count, 2);
    assert_eq!(archived_index.blocked_baseline_count, 0);
    assert_eq!(archived_index.rejected_baseline_count, 0);
    assert!(archived_index
        .entries
        .iter()
        .any(|entry| entry.baseline_id == saved_baseline.baseline_id
            && entry.baseline_manifest_path.as_deref()
                == Some(saved_baseline_path.display().to_string().as_str())));
    assert!(archived_index.entries.iter().any(|entry| {
        entry.baseline_id == archived_baseline.baseline_id
            && entry.baseline_manifest_path.as_deref()
                == Some(baseline_path.display().to_string().as_str())
    }));
    let readback =
        load_shell_handoff_acceptance_baseline_index(&index_path).expect("load appended index");
    assert_eq!(readback, archived_index);

    let status = shell_handoff_acceptance_baseline_append_status(
        &report,
        &archived_baseline,
        &archived_index,
        &checklist_path,
        &baseline_path,
        &loaded_index_path,
        &bundle_root,
    );
    assert!(status.contains("acceptance baseline archived"));
    assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready.archive2"));
    assert!(status.contains(
            "baseline selection selected; requested studio.project.makepad_edit.rev1.ready.archive2; default studio.project.makepad_edit.rev1.ready.archive2; selected studio.project.makepad_edit.rev1.ready.archive2"
        ));
    assert!(status.contains(
        "baseline index slots 2; default studio.project.makepad_edit.rev1.ready.archive2"
    ));
    assert!(status.contains("studio.project.makepad_edit.rev1.ready [ready]"));
    assert!(status.contains("studio.project.makepad_edit.rev1.ready.archive2 [ready]"));
}
