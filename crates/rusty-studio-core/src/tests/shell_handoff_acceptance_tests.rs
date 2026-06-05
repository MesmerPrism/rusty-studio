use super::*;

#[test]
fn shell_handoff_acceptance_checklist_reports_ready_entries() {
    let root = temp_root("shell-handoff-acceptance");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);

    let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

    assert_eq!(
        checklist.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA
    );
    assert_eq!(
        checklist.source_intake_schema,
        SHELL_HANDOFF_INTAKE_REPORT_SCHEMA
    );
    assert_eq!(checklist.status, StudioShellHandoffAcceptanceStatus::Ready);
    assert_eq!(checklist.issue_code, None);
    assert_eq!(checklist.ready_count, 3);
    assert_eq!(checklist.blocked_count, 0);
    assert_eq!(checklist.rejected_count, 0);
    assert_eq!(checklist.entries.len(), 3);
    assert_eq!(
        checklist.prohibited_actions,
        vec![
            "install".to_string(),
            "launch".to_string(),
            "open_command_session".to_string(),
            "collect_device_evidence".to_string()
        ]
    );
    assert!(checklist
        .intake_checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert!(checklist.entries.iter().all(|entry| {
        entry.status == StudioShellHandoffAcceptanceStatus::Ready
            && entry.issue_code.is_none()
            && entry.next_required_action == "stage_with_runtime_owner"
            && entry.command_session_authority == "rusty.manifold"
            && entry.install_launch_evidence_authority == "rusty.hostess"
            && entry.studio_role == "authoring.export_planning"
            && entry
                .checks
                .iter()
                .all(|check| check.status == StudioValidationStatus::Pass)
    }));
}

#[test]
fn shell_handoff_acceptance_snapshot_reports_ready_project() {
    let root = temp_root("shell-handoff-acceptance-snapshot");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);
    let expected = shell_handoff_acceptance_checklist_for_intake(&intake);

    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(checklist, expected);
    assert_eq!(checklist.status, StudioShellHandoffAcceptanceStatus::Ready);
    assert_eq!(checklist.ready_count, 3);
    assert_eq!(checklist.blocked_count, 0);
    assert_eq!(checklist.rejected_count, 0);
    assert_eq!(checklist.entries.len(), 3);
}

#[test]
fn shell_handoff_acceptance_snapshot_blocks_missing_bundles() {
    let root = temp_root("shell-handoff-acceptance-snapshot-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");

    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(
        checklist.status,
        StudioShellHandoffAcceptanceStatus::Blocked
    );
    assert_eq!(
        checklist.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(checklist.ready_count, 0);
    assert_eq!(checklist.blocked_count, 3);
    assert_eq!(checklist.rejected_count, 0);
    assert_eq!(checklist.entries.len(), 3);
    assert!(checklist.entries.iter().all(|entry| {
        entry.status == StudioShellHandoffAcceptanceStatus::Blocked
            && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
    }));
}

#[test]
fn shell_handoff_acceptance_summary_reports_baseline_metadata() {
    let root = temp_root("shell-handoff-acceptance-summary");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");

    let summary = summarize_shell_handoff_acceptance_checklist(&checklist, Some(&checklist_path));

    assert_eq!(summary.schema_id, SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA);
    assert_eq!(
        summary.checklist_schema,
        SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA
    );
    let checklist_path_text = checklist_path.display().to_string();
    assert_eq!(
        summary.checklist_path.as_deref(),
        Some(checklist_path_text.as_str())
    );
    assert_eq!(summary.manifest_id, checklist.manifest_id);
    assert_eq!(summary.project_id, "studio.project.test");
    assert_eq!(summary.project_revision, 1);
    assert_eq!(summary.status, StudioShellHandoffAcceptanceStatus::Ready);
    assert_eq!(summary.issue_code, None);
    assert_eq!(summary.ready_count, 3);
    assert_eq!(summary.blocked_count, 0);
    assert_eq!(summary.rejected_count, 0);
    assert_eq!(summary.entry_count, 3);
    assert_eq!(summary.failed_intake_check_count, 0);
    assert_eq!(summary.targets.len(), 3);
    assert!(summary.targets.iter().all(|target| {
        target.graph_count == 1
            && target.ready_count == 1
            && target.blocked_count == 0
            && target.rejected_count == 0
            && target.graph_ids.len() == 1
            && target.consumer_ids.len() == 1
            && target.route_kinds.len() == 1
            && target.issue_codes.is_empty()
    }));
}

#[test]
fn shell_handoff_acceptance_summary_reports_blocked_target_metadata() {
    let root = temp_root("shell-handoff-acceptance-summary-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );

    let summary = summarize_shell_handoff_acceptance_checklist(&checklist, None);

    assert_eq!(summary.checklist_path, None);
    assert_eq!(summary.status, StudioShellHandoffAcceptanceStatus::Blocked);
    assert_eq!(
        summary.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(summary.ready_count, 0);
    assert_eq!(summary.blocked_count, 3);
    assert_eq!(summary.rejected_count, 0);
    assert_eq!(summary.entry_count, 3);
    assert_eq!(summary.targets.len(), 3);
    assert!(summary.targets.iter().all(|target| {
        target.ready_count == 0
            && target.blocked_count == 1
            && target.rejected_count == 0
            && target.issue_codes == vec!["studio.issue.shell_bundle_file_missing".to_string()]
    }));
}

#[test]
fn shell_handoff_acceptance_baseline_manifest_names_saved_checklist() {
    let root = temp_root("shell-handoff-acceptance-baseline-manifest");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");

    let baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        None,
        None,
    );

    assert_eq!(
        baseline.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA
    );
    assert_eq!(baseline.baseline_id, "studio.project.test.rev1.ready");
    assert_eq!(
        baseline.label,
        "studio.project.test revision 1 ready acceptance baseline"
    );
    let checklist_path_text = checklist_path.display().to_string();
    assert_eq!(baseline.checklist_path, checklist_path_text);
    assert_eq!(
        baseline.summary.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA
    );
    assert_eq!(
        baseline.summary.checklist_schema,
        SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA
    );
    assert_eq!(baseline.summary.project_id, "studio.project.test");
    assert_eq!(baseline.summary.project_revision, 1);
    assert_eq!(
        baseline.summary.status,
        StudioShellHandoffAcceptanceStatus::Ready
    );
    assert_eq!(baseline.summary.ready_count, 3);
    assert_eq!(baseline.summary.blocked_count, 0);
    assert_eq!(baseline.summary.rejected_count, 0);
    assert_eq!(baseline.summary.entry_count, 3);
    assert_eq!(baseline.summary.targets.len(), 3);
}

#[test]
fn shell_handoff_acceptance_baseline_manifest_accepts_custom_identity() {
    let root = temp_root("shell-handoff-acceptance-baseline-manifest-custom");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );
    let checklist_path = root.join("blocked-checklist.json");

    let baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some("preflight-blocked"),
        Some("Preflight blocked baseline"),
    );

    assert_eq!(baseline.baseline_id, "preflight-blocked");
    assert_eq!(baseline.label, "Preflight blocked baseline");
    assert_eq!(
        baseline.summary.status,
        StudioShellHandoffAcceptanceStatus::Blocked
    );
    assert_eq!(
        baseline.summary.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
}

#[test]
fn shell_handoff_acceptance_baseline_index_lists_named_baselines() {
    let root = temp_root("shell-handoff-acceptance-baseline-index");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &ready_bundle_root);
    let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );
    let ready_checklist_path = root.join("ready-checklist.json");
    let blocked_checklist_path = root.join("blocked-checklist.json");
    let ready_manifest_path = root.join("ready-baseline.json");
    let blocked_manifest_path = root.join("blocked-baseline.json");
    let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &ready_checklist,
        &ready_checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &blocked_checklist,
        &blocked_checklist_path,
        Some("synthetic-blocked"),
        Some("Synthetic blocked acceptance baseline"),
    );

    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![
            (ready_baseline, Some(ready_manifest_path.clone())),
            (blocked_baseline, Some(blocked_manifest_path.clone())),
        ],
        Some("synthetic-ready"),
    );

    assert_eq!(
        index.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA
    );
    assert_eq!(index.project_ids, vec!["studio.project.test"]);
    assert_eq!(
        index.manifest_ids,
        vec!["studio.shell_handoffs.studio.project.test"]
    );
    assert_eq!(
        index.default_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(index.baseline_count, 2);
    assert_eq!(index.ready_baseline_count, 1);
    assert_eq!(index.blocked_baseline_count, 1);
    assert_eq!(index.rejected_baseline_count, 0);
    assert_eq!(index.entries.len(), 2);
    assert_eq!(index.entries[0].baseline_id, "synthetic-blocked");
    assert_eq!(
        index.entries[0].baseline_manifest_path.as_deref(),
        Some(blocked_manifest_path.display().to_string().as_str())
    );
    assert_eq!(
        index.entries[0].checklist_path,
        blocked_checklist_path.display().to_string()
    );
    assert_eq!(
        index.entries[0].status,
        StudioShellHandoffAcceptanceStatus::Blocked
    );
    assert_eq!(index.entries[0].ready_count, 0);
    assert_eq!(index.entries[0].blocked_count, 3);
    assert_eq!(index.entries[0].target_count, 3);
    assert_eq!(index.entries[1].baseline_id, "synthetic-ready");
    assert_eq!(
        index.entries[1].baseline_manifest_path.as_deref(),
        Some(ready_manifest_path.display().to_string().as_str())
    );
    assert_eq!(index.entries[1].ready_count, 3);
    assert_eq!(index.entries[1].blocked_count, 0);
    assert_eq!(
        select_shell_handoff_acceptance_baseline_index_entry(&index, None)
            .map(|entry| entry.baseline_id.as_str()),
        Some("synthetic-ready")
    );
    assert_eq!(
        select_shell_handoff_acceptance_baseline_index_entry(&index, Some("synthetic-blocked"))
            .map(|entry| entry.baseline_id.as_str()),
        Some("synthetic-blocked")
    );
    assert!(
        select_shell_handoff_acceptance_baseline_index_entry(&index, Some("missing")).is_none()
    );
}

#[test]
fn shell_handoff_acceptance_baseline_index_appends_named_baseline() {
    let root = temp_root("shell-handoff-acceptance-baseline-index-append");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &ready_bundle_root);
    let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );
    let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &ready_checklist,
        &root.join("ready-checklist.json"),
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let blocked_manifest_path = root.join("blocked-baseline.json");
    let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &blocked_checklist,
        &root.join("blocked-checklist.json"),
        Some("synthetic-blocked"),
        Some("Synthetic blocked acceptance baseline"),
    );
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(ready_baseline, Some(root.join("ready-baseline.json")))],
        Some("synthetic-ready"),
    );

    let appended = append_shell_handoff_acceptance_baseline_index_manifests(
        &index,
        vec![(blocked_baseline, Some(blocked_manifest_path.clone()))],
        Some("synthetic-blocked"),
    );

    assert_eq!(
        appended.default_baseline_id.as_deref(),
        Some("synthetic-blocked")
    );
    assert_eq!(appended.baseline_count, 2);
    assert_eq!(appended.ready_baseline_count, 1);
    assert_eq!(appended.blocked_baseline_count, 1);
    assert_eq!(appended.entries[0].baseline_id, "synthetic-blocked");
    assert_eq!(
        appended.entries[0].baseline_manifest_path.as_deref(),
        Some(blocked_manifest_path.display().to_string().as_str())
    );
    assert_eq!(
        appended.entries[0].status,
        StudioShellHandoffAcceptanceStatus::Blocked
    );
    assert_eq!(appended.entries[1].baseline_id, "synthetic-ready");
    assert_eq!(
        select_shell_handoff_acceptance_baseline_index_entry(&appended, None)
            .map(|entry| entry.baseline_id.as_str()),
        Some("synthetic-blocked")
    );
}

#[test]
fn shell_handoff_acceptance_baseline_index_promotes_existing_default() {
    let root = temp_root("shell-handoff-acceptance-baseline-index-promote");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &ready_bundle_root);
    let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );
    let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &ready_checklist,
        &root.join("ready-checklist.json"),
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &blocked_checklist,
        &root.join("blocked-checklist.json"),
        Some("synthetic-blocked"),
        Some("Synthetic blocked acceptance baseline"),
    );
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![
            (ready_baseline, Some(root.join("ready-baseline.json"))),
            (blocked_baseline, Some(root.join("blocked-baseline.json"))),
        ],
        Some("synthetic-blocked"),
    );

    let promoted =
        promote_shell_handoff_acceptance_baseline_index_default(&index, "synthetic-ready")
            .expect("promote ready baseline");

    assert_eq!(
        promoted.default_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(promoted.baseline_count, 2);
    assert_eq!(promoted.ready_baseline_count, 1);
    assert_eq!(promoted.blocked_baseline_count, 1);
    assert_eq!(
        select_shell_handoff_acceptance_baseline_index_entry(&promoted, None)
            .map(|entry| entry.baseline_id.as_str()),
        Some("synthetic-ready")
    );
    assert!(promote_shell_handoff_acceptance_baseline_index_default(&index, "missing").is_none());
}

#[test]
fn shell_handoff_acceptance_baseline_selection_reports_default_entry() {
    let root = temp_root("shell-handoff-acceptance-baseline-selection");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &ready_bundle_root);
    let blocked_checklist = shell_handoff_acceptance_checklist_for_project(
        &project,
        Some(&root),
        &root.join("missing-selected-shells"),
    );
    let ready_checklist_path = root.join("ready-checklist.json");
    let blocked_checklist_path = root.join("blocked-checklist.json");
    let ready_manifest_path = root.join("ready-baseline.json");
    let blocked_manifest_path = root.join("blocked-baseline.json");
    let ready_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &ready_checklist,
        &ready_checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let blocked_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &blocked_checklist,
        &blocked_checklist_path,
        Some("synthetic-blocked"),
        Some("Synthetic blocked acceptance baseline"),
    );
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![
            (ready_baseline, Some(ready_manifest_path.clone())),
            (blocked_baseline, Some(blocked_manifest_path)),
        ],
        Some("synthetic-ready"),
    );
    let index_path = root.join("shell-handoff-acceptance-baselines.json");

    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        &index,
        Some(&index_path),
        None,
    );

    assert_eq!(
        selection.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_BASELINE_SELECTION_SCHEMA
    );
    assert_eq!(
        selection.source_index_schema,
        SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA
    );
    assert_eq!(
        selection.index_path.as_deref(),
        Some(index_path.display().to_string().as_str())
    );
    assert_eq!(selection.requested_baseline_id, None);
    assert_eq!(
        selection.default_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(
        selection.selected_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(
        selection.status,
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
    );
    assert_eq!(selection.issue_code, None);
    assert_eq!(selection.baseline_count, 2);
    assert_eq!(selection.ready_baseline_count, 1);
    assert_eq!(selection.blocked_baseline_count, 1);
    assert_eq!(selection.project_ids, vec!["studio.project.test"]);
    assert_eq!(selection.entries.len(), 2);
    let selected = selection
        .entries
        .iter()
        .find(|entry| entry.baseline_id == "synthetic-ready")
        .expect("selected entry");
    assert!(selected.selected);
    assert!(selected.default);
    assert_eq!(
        selected.baseline_manifest_path.as_deref(),
        Some(ready_manifest_path.display().to_string().as_str())
    );
    assert_eq!(selected.status, StudioShellHandoffAcceptanceStatus::Ready);
    assert_eq!(selected.ready_count, 3);
    assert_eq!(selected.entry_count, 3);
    assert_eq!(selected.target_count, 3);
    let blocked = selection
        .entries
        .iter()
        .find(|entry| entry.baseline_id == "synthetic-blocked")
        .expect("blocked entry");
    assert!(!blocked.selected);
    assert!(!blocked.default);
    assert_eq!(blocked.status, StudioShellHandoffAcceptanceStatus::Blocked);
}

#[test]
fn shell_handoff_acceptance_baseline_selection_reports_missing_request() {
    let root = temp_root("shell-handoff-acceptance-baseline-selection-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("ready-checklist.json");
    let baseline_path = root.join("ready-baseline.json");
    let baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(baseline, Some(baseline_path))],
        Some("synthetic-ready"),
    );

    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        &index,
        None,
        Some("synthetic-missing"),
    );

    assert_eq!(
        selection.status,
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Missing
    );
    assert_eq!(
        selection.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_acceptance_baseline_not_found")
    );
    assert_eq!(
        selection.requested_baseline_id.as_deref(),
        Some("synthetic-missing")
    );
    assert_eq!(selection.selected_baseline_id, None);
    assert_eq!(selection.baseline_count, 1);
    assert!(selection.entries.iter().all(|entry| !entry.selected));
}

#[test]
fn shell_handoff_acceptance_baseline_selection_reports_empty_index() {
    let index = shell_handoff_acceptance_baseline_index_for_manifests(Vec::new(), None);

    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(&index, None, None);

    assert_eq!(
        selection.status,
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Empty
    );
    assert_eq!(
        selection.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_acceptance_baseline_index_empty")
    );
    assert_eq!(selection.baseline_count, 0);
    assert_eq!(selection.selected_baseline_id, None);
    assert!(selection.entries.is_empty());
}

#[test]
fn shell_handoff_acceptance_checklist_rejects_invalid_intake() {
    let root = temp_root("shell-handoff-acceptance-invalid");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    manifest.runtime_authority.command_session_authority = "rusty.studio".to_string();
    let intake = shell_handoff_intake_for_manifest(&manifest);

    let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

    assert_eq!(
        checklist.status,
        StudioShellHandoffAcceptanceStatus::Rejected
    );
    assert_eq!(
        checklist.issue_code.as_deref(),
        Some("studio.issue.runtime_authority_mismatch")
    );
    assert_eq!(checklist.ready_count, 0);
    assert_eq!(checklist.blocked_count, 0);
    assert_eq!(checklist.rejected_count, 0);
    assert!(checklist.entries.is_empty());
    assert!(checklist.intake_checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && check.issue_code.as_deref() == Some("studio.issue.shell_handoff_intake_rejected")
    }));
}

#[test]
fn shell_handoff_acceptance_checklist_blocks_missing_bundles() {
    let root = temp_root("shell-handoff-acceptance-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);

    let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

    assert_eq!(
        checklist.status,
        StudioShellHandoffAcceptanceStatus::Blocked
    );
    assert_eq!(
        checklist.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(checklist.ready_count, 0);
    assert_eq!(checklist.blocked_count, 3);
    assert_eq!(checklist.rejected_count, 0);
    assert_eq!(checklist.entries.len(), 3);
    assert!(checklist
        .intake_checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert!(checklist.entries.iter().all(|entry| {
        entry.status == StudioShellHandoffAcceptanceStatus::Blocked
            && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            && entry.next_required_action == "repair_export_bundle"
            && entry.checks.iter().any(|check| {
                check.status == StudioValidationStatus::Fail
                    && check.issue_code.as_deref()
                        == Some("studio.issue.shell_handoff_acceptance_blocked")
            })
    }));
}

#[test]
fn shell_handoff_acceptance_comparison_reports_unchanged_ready_checklists() {
    let root = temp_root("shell-handoff-acceptance-compare-unchanged");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);
    let checklist = shell_handoff_acceptance_checklist_for_intake(&intake);

    let comparison = compare_shell_handoff_acceptance_checklists(&checklist, &checklist);

    assert_eq!(
        comparison.schema_id,
        SHELL_HANDOFF_ACCEPTANCE_COMPARISON_SCHEMA
    );
    assert_eq!(comparison.baseline_identity_schema, None);
    assert_eq!(comparison.baseline_id, None);
    assert_eq!(comparison.baseline_label, None);
    assert_eq!(comparison.baseline_checklist_path, None);
    assert_eq!(comparison.baseline_index_schema, None);
    assert_eq!(comparison.baseline_index_path, None);
    assert_eq!(comparison.baseline_index_default_baseline_id, None);
    assert_eq!(comparison.baseline_index_selected_baseline_id, None);
    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Unchanged
    );
    assert_eq!(comparison.issue_code, None);
    assert_eq!(comparison.ready_delta, 0);
    assert_eq!(comparison.blocked_delta, 0);
    assert_eq!(comparison.rejected_delta, 0);
    assert_eq!(comparison.entries.len(), 3);
    assert!(comparison.entries.iter().all(|entry| entry.change
        == StudioShellHandoffAcceptanceComparisonChange::Unchanged
        && entry.score_delta == 0));
    assert!(comparison
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn shell_handoff_acceptance_comparison_carries_baseline_identity() {
    let root = temp_root("shell-handoff-acceptance-compare-baseline-identity");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    let baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );

    let comparison = compare_shell_handoff_acceptance_against_baseline_manifest(
        &baseline_identity,
        &checklist,
        &checklist,
    );

    assert_eq!(
        comparison.baseline_identity_schema.as_deref(),
        Some(SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA)
    );
    assert_eq!(comparison.baseline_id.as_deref(), Some("synthetic-ready"));
    assert_eq!(
        comparison.baseline_label.as_deref(),
        Some("Synthetic ready acceptance baseline")
    );
    let checklist_path_text = checklist_path.display().to_string();
    assert_eq!(
        comparison.baseline_checklist_path.as_deref(),
        Some(checklist_path_text.as_str())
    );
    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Unchanged
    );
    assert!(comparison
        .checks
        .iter()
        .filter(|check| check.check_id.contains("baseline_identity"))
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert_eq!(comparison.entries.len(), 3);
}

#[test]
fn shell_handoff_acceptance_comparison_carries_baseline_index_selection() {
    let root = temp_root("shell-handoff-acceptance-compare-baseline-index");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    let baseline_path = root.join("shell-handoff-acceptance-baseline.json");
    let index_path = root.join("shell-handoff-acceptance-baselines.json");
    let baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(baseline_identity.clone(), Some(baseline_path.clone()))],
        Some("synthetic-ready"),
    );
    let selected_entry = select_shell_handoff_acceptance_baseline_index_entry(&index, None)
        .expect("selected baseline index entry");

    let comparison = compare_shell_handoff_acceptance_against_baseline_index_entry(
        &index,
        Some(&index_path),
        selected_entry,
        Some(&baseline_path),
        &baseline_identity,
        &checklist,
        &checklist,
    );

    assert_eq!(
        comparison.baseline_index_schema.as_deref(),
        Some(SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA)
    );
    assert_eq!(
        comparison.baseline_index_path.as_deref(),
        Some(index_path.display().to_string().as_str())
    );
    assert_eq!(
        comparison.baseline_index_default_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(
        comparison.baseline_index_selected_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Unchanged
    );
    assert!(comparison
        .checks
        .iter()
        .filter(|check| check.check_id.contains("baseline_index"))
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert_eq!(comparison.entries.len(), 3);
}

#[test]
fn shell_handoff_acceptance_comparison_rejects_stale_baseline_index_selection() {
    let root = temp_root("shell-handoff-acceptance-compare-stale-baseline-index");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    let baseline_path = root.join("shell-handoff-acceptance-baseline.json");
    let index_path = root.join("shell-handoff-acceptance-baselines.json");
    let baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let mut index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(baseline_identity.clone(), Some(baseline_path.clone()))],
        Some("synthetic-ready"),
    );
    index.entries[0].ready_count += 1;
    let selected_entry = select_shell_handoff_acceptance_baseline_index_entry(&index, None)
        .expect("selected baseline index entry");

    let comparison = compare_shell_handoff_acceptance_against_baseline_index_entry(
        &index,
        Some(&index_path),
        selected_entry,
        Some(&baseline_path),
        &baseline_identity,
        &checklist,
        &checklist,
    );

    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_acceptance_baseline_index_mismatch")
    );
    assert!(comparison.entries.is_empty());
    assert!(comparison.checks.iter().any(|check| {
        check.check_id
            == "studio.check.shell_handoff_acceptance_comparison.baseline_index_status_counts"
            && check.status == StudioValidationStatus::Fail
    }));
}

#[test]
fn shell_handoff_acceptance_comparison_rejects_stale_baseline_identity() {
    let root = temp_root("shell-handoff-acceptance-compare-stale-baseline-identity");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let checklist =
        shell_handoff_acceptance_checklist_for_project(&project, Some(&root), &bundle_root);
    let checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    let mut baseline_identity = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &checklist_path,
        None,
        None,
    );
    baseline_identity.summary.project_revision += 1;

    let comparison = compare_shell_handoff_acceptance_against_baseline_manifest(
        &baseline_identity,
        &checklist,
        &checklist,
    );

    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_acceptance_baseline_identity_mismatch")
    );
    assert!(comparison.entries.is_empty());
    assert!(comparison.checks.iter().any(|check| {
        check.check_id
            == "studio.check.shell_handoff_acceptance_comparison.baseline_identity_project"
            && check.status == StudioValidationStatus::Fail
    }));
}

#[test]
fn shell_handoff_acceptance_comparison_reports_regression_to_missing_bundles() {
    let root = temp_root("shell-handoff-acceptance-compare-regressed");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_manifest =
        shell_handoff_manifest_for_project(&project, Some(&root), &ready_bundle_root);
    let ready_intake = shell_handoff_intake_for_manifest(&ready_manifest);
    let ready_checklist = shell_handoff_acceptance_checklist_for_intake(&ready_intake);
    let missing_manifest =
        shell_handoff_manifest_for_project(&project, Some(&root), &root.join("missing"));
    let missing_intake = shell_handoff_intake_for_manifest(&missing_manifest);
    let missing_checklist = shell_handoff_acceptance_checklist_for_intake(&missing_intake);

    let comparison =
        compare_shell_handoff_acceptance_checklists(&ready_checklist, &missing_checklist);

    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Regressed
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(comparison.ready_delta, -3);
    assert_eq!(comparison.blocked_delta, 3);
    assert_eq!(comparison.rejected_delta, 0);
    assert_eq!(comparison.entries.len(), 3);
    assert!(comparison.entries.iter().all(|entry| {
        entry.change == StudioShellHandoffAcceptanceComparisonChange::Regressed
            && entry.score_delta == -1
            && entry.candidate_issue_code.as_deref()
                == Some("studio.issue.shell_bundle_file_missing")
    }));
}

#[test]
fn shell_handoff_acceptance_comparison_reports_improvement_from_missing_bundles() {
    let root = temp_root("shell-handoff-acceptance-compare-improved");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let ready_manifest =
        shell_handoff_manifest_for_project(&project, Some(&root), &ready_bundle_root);
    let ready_intake = shell_handoff_intake_for_manifest(&ready_manifest);
    let ready_checklist = shell_handoff_acceptance_checklist_for_intake(&ready_intake);
    let missing_manifest =
        shell_handoff_manifest_for_project(&project, Some(&root), &root.join("missing"));
    let missing_intake = shell_handoff_intake_for_manifest(&missing_manifest);
    let missing_checklist = shell_handoff_acceptance_checklist_for_intake(&missing_intake);

    let comparison =
        compare_shell_handoff_acceptance_checklists(&missing_checklist, &ready_checklist);

    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Improved
    );
    assert_eq!(comparison.issue_code, None);
    assert_eq!(comparison.ready_delta, 3);
    assert_eq!(comparison.blocked_delta, -3);
    assert_eq!(comparison.rejected_delta, 0);
    assert!(comparison.entries.iter().all(|entry| {
        entry.change == StudioShellHandoffAcceptanceComparisonChange::Improved
            && entry.score_delta == 1
    }));
}

#[test]
fn shell_handoff_acceptance_comparison_rejects_mismatched_projects() {
    let root = temp_root("shell-handoff-acceptance-compare-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);
    let baseline = shell_handoff_acceptance_checklist_for_intake(&intake);
    let mut candidate = baseline.clone();
    candidate.project_id = "studio.project.other".to_string();

    let comparison = compare_shell_handoff_acceptance_checklists(&baseline, &candidate);

    assert_eq!(
        comparison.status,
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_acceptance_project_mismatch")
    );
    assert!(comparison.entries.is_empty());
    assert!(comparison.checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && check.issue_code.as_deref()
                == Some("studio.issue.shell_handoff_acceptance_project_mismatch")
    }));
}
