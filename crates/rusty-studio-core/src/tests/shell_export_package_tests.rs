use super::*;

#[test]
fn shell_export_package_groups_descriptors_templates_and_runbook_rows() {
    let root = temp_root("shell-export-package");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }

    let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(package.schema_id, SHELL_EXPORT_PACKAGE_REPORT_SCHEMA);
    assert_eq!(
        package.source_manifest_schema,
        SHELL_HANDOFF_MANIFEST_SCHEMA
    );
    assert_eq!(package.source_runbook_schema, SHELL_RUNBOOK_REPORT_SCHEMA);
    assert_eq!(
        package.package_id,
        "studio.shell_export_package.studio.project.test"
    );
    assert_eq!(package.status, StudioShellExportPackageStatus::Ready);
    assert_eq!(package.issue_code, None);
    assert_eq!(package.execution_policy, "not_executed.review_only");
    assert_eq!(package.review_owner, "rusty.hostess");
    assert_eq!(package.command_session_authority, "rusty.manifold");
    assert_eq!(package.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(package.studio_role, "authoring.export_planning");
    assert_eq!(package.ready_count, 3);
    assert_eq!(package.blocked_count, 0);
    assert_eq!(package.rejected_count, 0);
    assert_eq!(package.descriptor_count, 3);
    assert_eq!(package.template_manifest_count, 3);
    assert_eq!(package.runbook_entry_count, 3);
    assert_eq!(package.target_summaries.len(), 3);
    assert_eq!(
        package.prohibited_actions,
        vec![
            "install".to_string(),
            "launch".to_string(),
            "open_command_session".to_string(),
            "collect_device_evidence".to_string(),
        ]
    );
    for target_kind in [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
    ] {
        let summary = package
            .target_summaries
            .iter()
            .find(|summary| summary.target_kind == target_kind)
            .expect("export package target summary");
        assert_eq!(summary.ready_count, 1);
        assert_eq!(summary.blocked_count, 0);
        assert_eq!(summary.rejected_count, 0);
        assert_eq!(summary.descriptor_count, 1);
        assert_eq!(summary.template_manifest_count, 1);
        assert_eq!(summary.responsible_owners, vec!["rusty.hostess"]);
        assert!(summary.issue_codes.is_empty());
    }

    let desktop = package
        .entries
        .iter()
        .find(|entry| entry.target_kind == StudioShellTargetKind::Desktop)
        .expect("desktop export package row");
    assert_eq!(desktop.status, StudioShellExportPackageStatus::Ready);
    assert_eq!(desktop.responsible_owner, "rusty.hostess");
    assert_eq!(desktop.execution_policy, "not_executed.review_only");
    assert_eq!(desktop.next_required_action, "review_with_runtime_owner");
    assert_eq!(desktop.runtime_route_kind, "desktop_operator_shell");
    assert_eq!(
        desktop.runbook_cli_request[..5],
        ["cargo", "run", "-p", "rusty-studio-desktop-shell", "--"]
    );
    let descriptor = desktop.descriptor.as_ref().expect("desktop descriptor ref");
    assert_eq!(
        descriptor.descriptor_id,
        "studio.shell_descriptor.studio.graph.test"
    );
    assert_eq!(descriptor.graph_id, desktop.graph_id);
    assert_eq!(descriptor.package_count, 1);
    assert_eq!(descriptor.module_count, 1);
    assert_eq!(descriptor.command_binding_count, 1);
    let template = desktop
        .template_manifest
        .as_ref()
        .expect("desktop template ref");
    assert_eq!(
        template.template_id,
        "studio.shell_template.studio.graph.test"
    );
    assert_eq!(template.graph_id, desktop.graph_id);
    assert_eq!(
        template.host_routes.install_route.as_deref(),
        Some("install.local_process")
    );
    assert_eq!(
        template.runtime_authority.command_session_authority,
        "rusty.manifold"
    );

    let quest = package
        .entries
        .iter()
        .find(|entry| entry.target_kind == StudioShellTargetKind::Quest)
        .expect("quest export package row");
    let quest_template = quest
        .template_manifest
        .as_ref()
        .expect("quest template ref");
    assert_eq!(quest.consumer_id, "rusty-studio-quest-shell");
    assert_eq!(
        quest_template.host_routes.command_bridge.as_deref(),
        Some("bridge.adb_intent_file")
    );
    assert_eq!(
        quest.host_routes.evidence_pull_route.as_deref(),
        Some("evidence.adb_pull")
    );
}

#[test]
fn shell_export_package_blocks_missing_bundle_without_descriptors() {
    let root = temp_root("shell-export-package-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");

    let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(package.status, StudioShellExportPackageStatus::Blocked);
    assert_eq!(
        package.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(package.ready_count, 0);
    assert_eq!(package.blocked_count, 3);
    assert_eq!(package.rejected_count, 0);
    assert_eq!(package.descriptor_count, 0);
    assert_eq!(package.template_manifest_count, 0);
    assert_eq!(package.runbook_entry_count, 3);
    assert!(package.entries.iter().all(|entry| {
        entry.status == StudioShellExportPackageStatus::Blocked
            && entry.responsible_owner == "rusty.studio"
            && entry.descriptor.is_none()
            && entry.template_manifest.is_none()
            && entry.runbook_cli_request.is_empty()
            && entry.execution_policy == "not_executed.review_only"
    }));
}

#[test]
fn shell_export_package_blocks_one_damaged_descriptor_from_valid_manifest() {
    let root = temp_root("shell-export-package-damaged-descriptor");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let runbook = shell_runbook_for_manifest(&manifest);
    assert_eq!(runbook.status, StudioShellRunbookStatus::Ready);

    std::fs::remove_file(
        bundle_root
            .join("studio.graph.phone")
            .join("descriptors/studio.graph.phone.shell-descriptor.json"),
    )
    .expect("remove phone descriptor");

    let package = shell_export_package_for_manifest(&manifest);

    assert_eq!(package.status, StudioShellExportPackageStatus::Blocked);
    assert_eq!(
        package.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_descriptor_load_failed")
    );
    assert_eq!(package.ready_count, 2);
    assert_eq!(package.blocked_count, 1);
    assert_eq!(package.rejected_count, 0);
    assert_eq!(package.descriptor_count, 2);
    assert_eq!(package.template_manifest_count, 3);
    assert_eq!(package.runbook_entry_count, 3);

    let phone = package
        .entries
        .iter()
        .find(|entry| entry.graph_id == "studio.graph.phone")
        .expect("phone export package row");
    assert_eq!(phone.status, StudioShellExportPackageStatus::Blocked);
    assert_eq!(phone.responsible_owner, "rusty.studio");
    assert_eq!(
        phone.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_descriptor_load_failed")
    );
    assert!(phone.descriptor.is_none());
    assert!(phone.template_manifest.is_some());
    assert!(phone.runbook_cli_request.is_empty());
    assert_eq!(
        phone.host_routes.command_bridge.as_deref(),
        Some("bridge.adb_intent_file")
    );

    for target_kind in [StudioShellTargetKind::Desktop, StudioShellTargetKind::Quest] {
        let entry = package
            .entries
            .iter()
            .find(|entry| entry.target_kind == target_kind)
            .expect("undamaged export package row");
        assert_eq!(entry.status, StudioShellExportPackageStatus::Ready);
        assert_eq!(entry.responsible_owner, "rusty.hostess");
        assert!(entry.descriptor.is_some());
        assert!(entry.template_manifest.is_some());
        assert!(!entry.runbook_cli_request.is_empty());
    }

    let phone_summary = package
        .target_summaries
        .iter()
        .find(|summary| summary.target_kind == StudioShellTargetKind::Phone)
        .expect("phone summary");
    assert_eq!(phone_summary.ready_count, 0);
    assert_eq!(phone_summary.blocked_count, 1);
    assert_eq!(phone_summary.descriptor_count, 0);
    assert_eq!(phone_summary.template_manifest_count, 1);
    assert_eq!(
        phone_summary.issue_codes,
        vec!["studio.issue.shell_export_package_descriptor_load_failed"]
    );
}

#[test]
fn shell_export_package_blocks_one_damaged_template_from_valid_manifest() {
    let root = temp_root("shell-export-package-damaged-template");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let runbook = shell_runbook_for_manifest(&manifest);
    assert_eq!(runbook.status, StudioShellRunbookStatus::Ready);

    std::fs::remove_file(
        bundle_root
            .join("studio.graph.phone")
            .join("shells/phone/studio.graph.phone.shell-template.json"),
    )
    .expect("remove phone template manifest");

    let package = shell_export_package_for_manifest(&manifest);

    assert_eq!(package.status, StudioShellExportPackageStatus::Blocked);
    assert_eq!(
        package.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert_eq!(package.ready_count, 2);
    assert_eq!(package.blocked_count, 1);
    assert_eq!(package.rejected_count, 0);
    assert_eq!(package.descriptor_count, 3);
    assert_eq!(package.template_manifest_count, 2);
    assert_eq!(package.runbook_entry_count, 3);

    let phone = package
        .entries
        .iter()
        .find(|entry| entry.graph_id == "studio.graph.phone")
        .expect("phone export package row");
    assert_eq!(phone.status, StudioShellExportPackageStatus::Blocked);
    assert_eq!(phone.responsible_owner, "rusty.studio");
    assert_eq!(
        phone.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert!(phone.descriptor.is_some());
    assert!(phone.template_manifest.is_none());
    assert!(phone.runbook_cli_request.is_empty());
    assert_eq!(phone.host_routes, empty_shell_host_routes());

    for target_kind in [StudioShellTargetKind::Desktop, StudioShellTargetKind::Quest] {
        let entry = package
            .entries
            .iter()
            .find(|entry| entry.target_kind == target_kind)
            .expect("undamaged export package row");
        assert_eq!(entry.status, StudioShellExportPackageStatus::Ready);
        assert_eq!(entry.responsible_owner, "rusty.hostess");
        assert!(entry.descriptor.is_some());
        assert!(entry.template_manifest.is_some());
        assert!(!entry.runbook_cli_request.is_empty());
    }

    let phone_summary = package
        .target_summaries
        .iter()
        .find(|summary| summary.target_kind == StudioShellTargetKind::Phone)
        .expect("phone summary");
    assert_eq!(phone_summary.ready_count, 0);
    assert_eq!(phone_summary.blocked_count, 1);
    assert_eq!(phone_summary.descriptor_count, 1);
    assert_eq!(phone_summary.template_manifest_count, 0);
    assert_eq!(
        phone_summary.issue_codes,
        vec!["studio.issue.shell_export_package_template_load_failed"]
    );
}

#[test]
fn shell_export_package_comparison_reports_unchanged_ready_packages() {
    let root = temp_root("shell-export-package-comparison");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);

    let comparison = compare_shell_export_packages(&package, &package);

    assert_eq!(comparison.schema_id, SHELL_EXPORT_PACKAGE_COMPARISON_SCHEMA);
    assert_eq!(
        comparison.baseline_schema,
        SHELL_EXPORT_PACKAGE_REPORT_SCHEMA
    );
    assert_eq!(
        comparison.status,
        StudioShellExportPackageComparisonStatus::Unchanged
    );
    assert_eq!(comparison.issue_code, None);
    assert_eq!(comparison.ready_delta, 0);
    assert_eq!(comparison.blocked_delta, 0);
    assert_eq!(comparison.rejected_delta, 0);
    assert_eq!(comparison.descriptor_delta, 0);
    assert_eq!(comparison.template_manifest_delta, 0);
    assert_eq!(comparison.runbook_entry_delta, 0);
    assert_eq!(comparison.entries.len(), 3);
    assert!(comparison.entries.iter().all(|entry| {
        entry.change == StudioShellExportPackageComparisonChange::Unchanged
            && entry.baseline_descriptor_present
            && entry.candidate_descriptor_present
            && entry.baseline_template_manifest_present
            && entry.candidate_template_manifest_present
            && entry.baseline_runbook_cli_request_present
            && entry.candidate_runbook_cli_request_present
    }));
    assert!(comparison
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn shell_export_package_comparison_reports_regressed_damaged_template() {
    let root = temp_root("shell-export-package-comparison-damaged-template");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let baseline = shell_export_package_for_manifest(&manifest);
    assert_eq!(baseline.status, StudioShellExportPackageStatus::Ready);

    std::fs::remove_file(
        bundle_root
            .join("studio.graph.phone")
            .join("shells/phone/studio.graph.phone.shell-template.json"),
    )
    .expect("remove phone template manifest");
    let candidate = shell_export_package_for_manifest(&manifest);

    let comparison = compare_shell_export_packages(&baseline, &candidate);

    assert_eq!(
        comparison.status,
        StudioShellExportPackageComparisonStatus::Regressed
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert_eq!(comparison.ready_delta, -1);
    assert_eq!(comparison.blocked_delta, 1);
    assert_eq!(comparison.rejected_delta, 0);
    assert_eq!(comparison.descriptor_delta, 0);
    assert_eq!(comparison.template_manifest_delta, -1);
    assert_eq!(comparison.runbook_entry_delta, 0);
    let phone = comparison
        .entries
        .iter()
        .find(|entry| entry.graph_id == "studio.graph.phone")
        .expect("phone comparison entry");
    assert_eq!(
        phone.change,
        StudioShellExportPackageComparisonChange::Regressed
    );
    assert_eq!(phone.score_delta, -1);
    assert!(phone.baseline_descriptor_present);
    assert!(phone.candidate_descriptor_present);
    assert!(phone.baseline_template_manifest_present);
    assert!(!phone.candidate_template_manifest_present);
    assert!(phone.baseline_runbook_cli_request_present);
    assert!(!phone.candidate_runbook_cli_request_present);
    assert_eq!(
        phone.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
}

#[test]
fn shell_export_package_comparison_rejects_mismatched_projects() {
    let root = temp_root("shell-export-package-comparison-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let baseline = shell_export_package_for_project(&project, Some(&root), &bundle_root);
    let mut candidate = baseline.clone();
    candidate.project_id = "studio.project.other".to_string();

    let comparison = compare_shell_export_packages(&baseline, &candidate);

    assert_eq!(
        comparison.status,
        StudioShellExportPackageComparisonStatus::Incomparable
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_project_mismatch")
    );
    assert!(comparison.entries.is_empty());
    assert!(comparison.checks.iter().any(|check| {
        check.check_id == "studio.check.shell_export_package_comparison.project_id"
            && check.status == StudioValidationStatus::Fail
    }));
}

#[test]
fn shell_export_package_baseline_index_lists_named_review_slots() {
    let root = temp_root("shell-export-package-baseline-index");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let ready_bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&ready_bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &ready_bundle_root);
    let ready_package = shell_export_package_for_manifest(&manifest);
    let mut blocked_package = ready_package.clone();
    blocked_package.status = StudioShellExportPackageStatus::Blocked;
    blocked_package.issue_code =
        Some("studio.issue.shell_export_package_template_load_failed".to_string());
    blocked_package.ready_count = 2;
    blocked_package.blocked_count = 1;
    blocked_package.template_manifest_count = 2;

    let ready_package_path = root.join("shell-export-package-ready.json");
    let blocked_package_path = root.join("shell-export-package-blocked.json");
    let ready_baseline_path = root.join("shell-export-package-baseline-ready.json");
    let blocked_baseline_path = root.join("shell-export-package-baseline-blocked.json");
    let ready_baseline = shell_export_package_baseline_manifest_for_report(
        &ready_package,
        &ready_package_path,
        Some("synthetic-ready-package"),
        Some("Synthetic ready export package baseline"),
    );
    let blocked_baseline = shell_export_package_baseline_manifest_for_report(
        &blocked_package,
        &blocked_package_path,
        Some("synthetic-blocked-package"),
        Some("Synthetic blocked export package baseline"),
    );

    let index = shell_export_package_baseline_index_for_manifests(
        vec![
            (ready_baseline, Some(ready_baseline_path.clone())),
            (blocked_baseline, Some(blocked_baseline_path)),
        ],
        Some("synthetic-ready-package"),
    );
    let selection = summarize_shell_export_package_baseline_index_selection(
        &index,
        Some(&root.join("shell-export-package-baselines.json")),
        None,
    );

    assert_eq!(index.schema_id, SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA);
    assert_eq!(
        index.default_baseline_id.as_deref(),
        Some("synthetic-ready-package")
    );
    assert_eq!(index.baseline_count, 2);
    assert_eq!(index.ready_baseline_count, 1);
    assert_eq!(index.blocked_baseline_count, 1);
    assert_eq!(index.rejected_baseline_count, 0);
    assert_eq!(index.project_ids, vec!["studio.project.test"]);
    assert_eq!(
        index.package_ids,
        vec!["studio.shell_export_package.studio.project.test"]
    );
    let selected = select_shell_export_package_baseline_index_entry(&index, None)
        .expect("selected export package baseline");
    assert_eq!(selected.baseline_id, "synthetic-ready-package");
    assert_eq!(
        selected.baseline_manifest_path.as_deref(),
        Some(ready_baseline_path.display().to_string().as_str())
    );
    assert_eq!(
        selected.package_path,
        ready_package_path.display().to_string()
    );
    assert_eq!(selected.status, StudioShellExportPackageStatus::Ready);
    assert_eq!(selected.ready_count, 3);
    assert_eq!(selected.target_count, 3);
    assert_eq!(
        selection.schema_id,
        SHELL_EXPORT_PACKAGE_BASELINE_SELECTION_SCHEMA
    );
    assert_eq!(
        selection.status,
        StudioShellExportPackageBaselineSelectionStatus::Selected
    );
    assert_eq!(
        selection.selected_baseline_id.as_deref(),
        Some("synthetic-ready-package")
    );
    assert!(
        selection
            .entries
            .iter()
            .find(|entry| entry.baseline_id == "synthetic-ready-package")
            .expect("ready selection entry")
            .selected
    );
}

#[test]
fn shell_export_package_comparison_carries_baseline_index_selection() {
    let root = temp_root("shell-export-package-comparison-baseline-index");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);
    let package_path = root.join("shell-export-package.json");
    let baseline_path = root.join("shell-export-package-baseline.json");
    let index_path = root.join("shell-export-package-baselines.json");
    let baseline_identity = shell_export_package_baseline_manifest_for_report(
        &package,
        &package_path,
        Some("synthetic-ready-package"),
        Some("Synthetic ready export package baseline"),
    );
    let index = shell_export_package_baseline_index_for_manifests(
        vec![(baseline_identity.clone(), Some(baseline_path.clone()))],
        Some("synthetic-ready-package"),
    );
    let selected_entry = select_shell_export_package_baseline_index_entry(&index, None)
        .expect("selected export package baseline index entry");

    let comparison = compare_shell_export_packages_against_baseline_index_entry(
        &index,
        Some(&index_path),
        selected_entry,
        Some(&baseline_path),
        &baseline_identity,
        &package,
        &package,
    );

    assert_eq!(
        comparison.baseline_identity_schema.as_deref(),
        Some(SHELL_EXPORT_PACKAGE_BASELINE_MANIFEST_SCHEMA)
    );
    assert_eq!(
        comparison.baseline_id.as_deref(),
        Some("synthetic-ready-package")
    );
    assert_eq!(
        comparison.baseline_package_path.as_deref(),
        Some(package_path.display().to_string().as_str())
    );
    assert_eq!(
        comparison.baseline_index_schema.as_deref(),
        Some(SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA)
    );
    assert_eq!(
        comparison.baseline_index_path.as_deref(),
        Some(index_path.display().to_string().as_str())
    );
    assert_eq!(
        comparison.baseline_index_selected_baseline_id.as_deref(),
        Some("synthetic-ready-package")
    );
    assert_eq!(
        comparison.status,
        StudioShellExportPackageComparisonStatus::Unchanged
    );
    assert!(comparison
        .checks
        .iter()
        .filter(|check| check.check_id.contains("baseline_identity"))
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert!(comparison
        .checks
        .iter()
        .filter(|check| check.check_id.contains("baseline_index"))
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert_eq!(comparison.entries.len(), 3);
}

#[test]
fn shell_export_package_comparison_rejects_stale_baseline_identity() {
    let root = temp_root("shell-export-package-comparison-stale-baseline");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let package = shell_export_package_for_project(&project, Some(&root), &bundle_root);
    let package_path = root.join("shell-export-package.json");
    let mut baseline_identity =
        shell_export_package_baseline_manifest_for_report(&package, &package_path, None, None);
    baseline_identity.project_revision += 1;

    let comparison = compare_shell_export_packages_against_baseline_manifest(
        &baseline_identity,
        &package,
        &package,
    );

    assert_eq!(
        comparison.status,
        StudioShellExportPackageComparisonStatus::Incomparable
    );
    assert_eq!(
        comparison.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_baseline_identity_mismatch")
    );
    assert!(comparison.entries.is_empty());
    assert!(comparison.checks.iter().any(|check| {
        check.check_id == "studio.check.shell_export_package_comparison.baseline_identity_project"
            && check.status == StudioValidationStatus::Fail
    }));
}
