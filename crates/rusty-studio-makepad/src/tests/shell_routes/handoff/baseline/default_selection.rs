use super::super::super::*;

#[test]
fn shell_handoff_acceptance_baseline_promotes_saved_default() {
    let root = temp_root("shell-handoff-acceptance-baseline-promote");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");
    let (_, ready_baseline, ready_index, _, ready_baseline_path, index_path, _) =
        write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("write acceptance baseline");

    let project = load_project(&project_path).expect("load project");
    let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        project_path.parent(),
        &root.join("missing-selected-shells"),
    );
    let blocked_checklist_path = root.join("blocked-checklist.json");
    save_json(&blocked_checklist_path, &blocked_checklist)
        .expect("save blocked acceptance checklist");
    let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &blocked_checklist,
        &blocked_checklist_path,
        Some("studio.project.makepad_edit.rev1.blocked"),
        Some("studio.project.makepad_edit revision 1 blocked acceptance baseline"),
    );
    let blocked_baseline_path = root.join("blocked-baseline.json");
    save_json(&blocked_baseline_path, &blocked_baseline).expect("save blocked baseline");
    let multi_index = rusty_studio_core::append_shell_handoff_acceptance_baseline_index_manifests(
        &ready_index,
        vec![(blocked_baseline, Some(blocked_baseline_path))],
        Some("studio.project.makepad_edit.rev1.blocked"),
    );
    save_json(&index_path, &multi_index).expect("save multi-baseline index");
    assert_eq!(
        multi_index.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.blocked")
    );

    let (baseline, promoted, baseline_path, loaded_index_path) =
        promote_shell_handoff_acceptance_baseline_default_for_project_source(&project_path)
            .expect("promote saved baseline");

    assert_eq!(baseline, ready_baseline);
    assert_eq!(baseline_path, ready_baseline_path);
    assert_eq!(loaded_index_path, index_path);
    assert_eq!(
        promoted.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );
    assert_eq!(promoted.baseline_count, 2);
    assert_eq!(promoted.ready_baseline_count, 1);
    assert_eq!(promoted.blocked_baseline_count, 1);
    let written_index =
        load_shell_handoff_acceptance_baseline_index(&index_path).expect("load written index");
    assert_eq!(written_index, promoted);

    let status = shell_handoff_acceptance_baseline_promote_status(
        &baseline,
        &promoted,
        &baseline_path,
        &loaded_index_path,
    );
    assert!(status.contains("acceptance baseline default promoted"));
    assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
    assert!(status.contains(
            "baseline selection selected; requested studio.project.makepad_edit.rev1.ready; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));
    assert!(
        status.contains("baseline index slots 2; default studio.project.makepad_edit.rev1.ready")
    );
    assert!(status.contains("studio.project.makepad_edit.rev1.blocked [blocked]"));
}

#[test]
fn shell_handoff_acceptance_baseline_cycles_index_default() {
    let root = temp_root("shell-handoff-acceptance-baseline-cycle");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");
    let (_, ready_baseline, _, _, ready_baseline_path, index_path, _) =
        write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("write initial baseline");
    let (_, archived_baseline, archived_index, _, archived_baseline_path, _, _) =
        append_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("append baseline history entry");
    assert_eq!(
        archived_index.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready.archive2")
    );

    let (selected_ready_baseline, selected_ready_index, selected_ready_path, loaded_index_path) =
        select_next_shell_handoff_acceptance_baseline_default_for_project_source(&project_path)
            .expect("select next baseline default");

    assert_eq!(selected_ready_baseline, ready_baseline);
    assert_eq!(selected_ready_path, ready_baseline_path);
    assert_eq!(loaded_index_path, index_path);
    assert_eq!(
        selected_ready_index.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );
    let status = shell_handoff_acceptance_baseline_select_status(
        &selected_ready_baseline,
        &selected_ready_index,
        &selected_ready_path,
        &loaded_index_path,
    );
    assert!(status.contains("acceptance baseline default selected"));
    assert!(status.contains(
            "baseline selection selected; requested studio.project.makepad_edit.rev1.ready; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));
    assert!(status.contains("selected yes; default yes"));

    let (
        selected_archived_baseline,
        selected_archived_index,
        selected_archived_path,
        loaded_index_path,
    ) = select_next_shell_handoff_acceptance_baseline_default_for_project_source(&project_path)
        .expect("cycle baseline default");

    assert_eq!(selected_archived_baseline, archived_baseline);
    assert_eq!(selected_archived_path, archived_baseline_path);
    assert_eq!(
        selected_archived_index.default_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready.archive2")
    );
    let written_index = load_shell_handoff_acceptance_baseline_index(&loaded_index_path)
        .expect("load cycled index");
    assert_eq!(written_index, selected_archived_index);
    let status = shell_handoff_acceptance_baseline_select_status(
        &selected_archived_baseline,
        &selected_archived_index,
        &selected_archived_path,
        &loaded_index_path,
    );
    assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready.archive2"));
    assert!(status.contains(
            "baseline selection selected; requested studio.project.makepad_edit.rev1.ready.archive2; default studio.project.makepad_edit.rev1.ready.archive2; selected studio.project.makepad_edit.rev1.ready.archive2"
        ));
}
