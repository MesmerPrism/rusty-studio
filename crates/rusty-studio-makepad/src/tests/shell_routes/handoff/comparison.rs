use super::super::*;

#[test]
fn shell_handoff_acceptance_comparison_reports_unchanged_from_makepad_route() {
    let root = temp_root("shell-handoff-acceptance-comparison");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");
    let (_, saved_baseline, _, checklist_path, baseline_path, index_path, _) =
        write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("write baseline checklist");

    let (comparison, loaded_baseline_path, bundle_root) =
        shell_handoff_acceptance_comparison_for_project_source(&project_path)
            .expect("compare acceptance checklist");

    assert_eq!(loaded_baseline_path, baseline_path);
    assert_eq!(
        comparison.baseline_identity_schema.as_deref(),
        Some("rusty.studio.shell_handoff_acceptance_baseline_manifest.v1")
    );
    assert_eq!(
        comparison.baseline_id.as_deref(),
        Some(saved_baseline.baseline_id.as_str())
    );
    assert_eq!(
        comparison.baseline_label.as_deref(),
        Some(saved_baseline.label.as_str())
    );
    let checklist_path_text = checklist_path.display().to_string();
    assert_eq!(
        comparison.baseline_checklist_path.as_deref(),
        Some(checklist_path_text.as_str())
    );
    assert_eq!(
        comparison.baseline_index_schema.as_deref(),
        Some("rusty.studio.shell_handoff_acceptance_baseline_index.v1")
    );
    assert_eq!(
        comparison.baseline_index_path.as_deref(),
        Some(index_path.display().to_string().as_str())
    );
    assert_eq!(
        comparison.baseline_index_default_baseline_id.as_deref(),
        Some(saved_baseline.baseline_id.as_str())
    );
    assert_eq!(
        comparison.baseline_index_selected_baseline_id.as_deref(),
        Some(saved_baseline.baseline_id.as_str())
    );
    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Unchanged
    );
    assert_eq!(comparison.ready_delta, 0);
    assert_eq!(comparison.blocked_delta, 0);
    assert_eq!(comparison.rejected_delta, 0);
    assert_eq!(comparison.entries.len(), 1);
    assert_eq!(
        comparison.entries[0].change,
        StudioShellHandoffAcceptanceComparisonChange::Unchanged
    );
    let status =
        shell_handoff_acceptance_comparison_status(&comparison, &baseline_path, &bundle_root);
    assert!(status.contains("handoff acceptance comparison unchanged"));
    assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
    assert!(status.contains("baseline source: studio.project.makepad_edit rev 1"));
    assert!(status.contains("candidate: studio.project.makepad_edit rev 1"));
    assert!(status.contains("manifest studio.shell_handoffs.studio.project.makepad_edit"));
    assert!(status.contains(&format!("baseline index: {}", index_path.display())));
    assert!(status.contains("default studio.project.makepad_edit.rev1.ready"));
    assert!(status.contains("selected studio.project.makepad_edit.rev1.ready"));
    assert!(status.contains("ready 1->1, delta 0"));
    assert!(status.contains("change unchanged"));
    assert!(status.contains("studio.graph.makepad_edit [desktop]"));
}

#[test]
fn shell_handoff_acceptance_comparison_reports_regression_from_makepad_route() {
    let root = temp_root("shell-handoff-acceptance-comparison-regressed");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");
    let (_, _, _, _, baseline_path, index_path, _) =
        write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
            .expect("write baseline checklist");
    std::fs::remove_dir_all(selected_shell_bundle_root_dir(&project_path))
        .expect("remove selected shell bundle root");

    let (comparison, _, bundle_root) =
        shell_handoff_acceptance_comparison_for_project_source(&project_path)
            .expect("compare regressed acceptance checklist");

    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Regressed
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(comparison.ready_delta, -1);
    assert_eq!(comparison.blocked_delta, 1);
    assert_eq!(comparison.rejected_delta, 0);
    assert_eq!(
        comparison.baseline_index_path.as_deref(),
        Some(index_path.display().to_string().as_str())
    );
    assert_eq!(
        comparison.entries[0].change,
        StudioShellHandoffAcceptanceComparisonChange::Regressed
    );
    let status =
        shell_handoff_acceptance_comparison_status(&comparison, &baseline_path, &bundle_root);
    assert!(status.contains("handoff acceptance comparison regressed"));
    assert!(status.contains("baseline: studio.project.makepad_edit.rev1.ready"));
    assert!(status.contains(&format!("baseline index: {}", index_path.display())));
    assert!(status.contains("ready 1->0, delta -1"));
    assert!(status.contains("blocked 0->1, delta 1"));
    assert!(status.contains("issue studio.issue.shell_bundle_file_missing"));
    assert!(status.contains("change regressed"));
}
