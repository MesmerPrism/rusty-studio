use super::*;

#[test]
fn desktop_shell_handoff_reports_validated_schema_file_entrypoint() {
    let root = temp_root("desktop-shell-handoff");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();
    let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.test");
    let output_dir = root.join("selected-shell");
    save_shell_bundle(&output_dir, &report).expect("save shell bundle");

    let handoff =
        desktop_shell_handoff_for_bundle(&project, Some(&root), "studio.graph.test", &output_dir);

    assert_eq!(handoff.status, StudioValidationStatus::Pass);
    assert_eq!(handoff.handoff_kind, StudioShellHandoffKind::DesktopShell);
    assert_eq!(handoff.consumer_id, "rusty-studio-desktop-shell");
    assert_eq!(handoff.target_kind, StudioShellTargetKind::Desktop);
    assert_eq!(handoff.validation.status, StudioValidationStatus::Pass);
    assert!(
        handoff
            .descriptor_path
            .ends_with("descriptors\\studio.graph.test.shell-descriptor.json")
            || handoff
                .descriptor_path
                .ends_with("descriptors/studio.graph.test.shell-descriptor.json")
    );
    assert!(handoff
        .template_index_path
        .ends_with("shell-templates.json"));
    assert!(
        handoff
            .template_manifest_path
            .ends_with("shells\\desktop\\studio.graph.test.shell-template.json")
            || handoff
                .template_manifest_path
                .ends_with("shells/desktop/studio.graph.test.shell-template.json")
    );
    assert_eq!(
        handoff.consumer_args,
        vec![
            "--templates".to_string(),
            output_dir
                .join("shell-templates.json")
                .display()
                .to_string(),
        ]
    );
    let authority = handoff
        .runtime_authority
        .expect("handoff carries runtime authority summary");
    assert_eq!(authority.command_session_authority, "rusty.manifold");
    assert_eq!(authority.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(authority.studio_role, "authoring.export_planning");
}

#[test]
fn shell_handoff_reports_phone_and_quest_targets() {
    let root = temp_root("multi-target-shell-handoff");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    for (graph_id, expected_kind, expected_handoff, expected_consumer) in [
        (
            "studio.graph.phone",
            StudioShellTargetKind::Phone,
            StudioShellHandoffKind::PhoneShell,
            "rusty-studio-phone-shell",
        ),
        (
            "studio.graph.quest",
            StudioShellTargetKind::Quest,
            StudioShellHandoffKind::QuestShell,
            "rusty-studio-quest-shell",
        ),
    ] {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), graph_id);
        let output_dir = root.join(graph_id);
        save_shell_bundle(&output_dir, &report).expect("save shell bundle");

        let handoff = shell_handoff_for_bundle(&project, Some(&root), graph_id, &output_dir);

        assert_eq!(handoff.status, StudioValidationStatus::Pass);
        assert_eq!(handoff.handoff_kind, expected_handoff);
        assert_eq!(handoff.consumer_id, expected_consumer);
        assert_eq!(handoff.target_kind, expected_kind);
        assert_eq!(handoff.validation.status, StudioValidationStatus::Pass);
        assert_eq!(
            handoff.consumer_args,
            vec![
                "--templates".to_string(),
                output_dir
                    .join("shell-templates.json")
                    .display()
                    .to_string(),
            ]
        );
        assert_eq!(
            handoff
                .runtime_authority
                .as_ref()
                .expect("runtime authority")
                .install_launch_evidence_authority,
            "rusty.hostess"
        );
    }
}

#[test]
fn desktop_shell_handoff_rejects_non_desktop_bundle() {
    let root = temp_root("desktop-shell-handoff-target-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let report = selected_shell_bundle_for_graph(&project, Some(&root), "studio.graph.phone");
    let output_dir = root.join("phone-selected-shell");
    save_shell_bundle(&output_dir, &report).expect("save phone shell bundle");

    let handoff =
        desktop_shell_handoff_for_bundle(&project, Some(&root), "studio.graph.phone", &output_dir);

    assert_eq!(handoff.status, StudioValidationStatus::Fail);
    assert_eq!(
        handoff.issue_code.as_deref(),
        Some("studio.issue.shell_handoff_target_mismatch")
    );
    assert_eq!(handoff.handoff_kind, StudioShellHandoffKind::PhoneShell);
    assert_eq!(handoff.target_kind, StudioShellTargetKind::Phone);
    assert!(handoff.consumer_args.is_empty());
    assert_eq!(handoff.validation.status, StudioValidationStatus::Pass);
}

#[test]
fn shell_handoff_readiness_reports_all_target_graphs() {
    let root = temp_root("shell-handoff-readiness");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }

    let readiness = shell_handoff_readiness_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(readiness.schema_id, SHELL_HANDOFF_READINESS_REPORT_SCHEMA);
    assert_eq!(readiness.status, StudioValidationStatus::Pass);
    assert_eq!(readiness.graph_count, 3);
    assert_eq!(readiness.ready_count, 3);
    assert_eq!(readiness.failed_count, 0);
    assert_eq!(readiness.missing_bundle_count, 0);
    assert_eq!(readiness.entries.len(), 3);
    assert_eq!(readiness.target_summaries.len(), 3);
    for target_kind in [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
    ] {
        let summary = readiness
            .target_summaries
            .iter()
            .find(|summary| summary.target_kind == target_kind)
            .expect("target summary");
        assert_eq!(summary.graph_count, 1);
        assert_eq!(summary.ready_count, 1);
        assert_eq!(summary.failed_count, 0);
        assert_eq!(summary.missing_bundle_count, 0);
        assert_eq!(summary.package_count, 1);
        assert_eq!(summary.module_count, 1);
        assert_eq!(summary.operator_shell_count, 1);
        assert!(summary.issue_codes.is_empty());
        assert_eq!(summary.graph_ids.len(), 1);
        assert_eq!(summary.consumer_ids.len(), 1);
        assert_eq!(summary.bundle_dirs.len(), 1);
        assert_eq!(summary.ready_bundle_dirs.len(), 1);
        assert!(summary.failed_bundle_dirs.is_empty());
        assert!(summary.missing_bundle_dirs.is_empty());
        assert_eq!(summary.template_index_paths.len(), 1);
        assert!(summary.template_index_paths[0].ends_with("shell-templates.json"));
    }
    assert!(readiness.entries.iter().all(|entry| {
        entry.status == StudioValidationStatus::Pass
            && entry.validation_status == StudioValidationStatus::Pass
            && entry.failed_check_count == 0
            && entry.consumer_args.iter().any(|arg| arg == "--templates")
            && entry.export_bundle_id == format!("studio.export.{}", entry.graph_id)
            && entry.package_ids == vec!["package.synthetic".to_string()]
            && entry.module_ids == vec!["module.synthetic_provider".to_string()]
            && entry.package_count == entry.package_ids.len()
            && entry.module_count == entry.module_ids.len()
            && entry.operator_shell_count == entry.operator_shell_ids.len()
    }));
    assert!(readiness.entries.iter().any(|entry| {
        entry.graph_id == "studio.graph.phone"
            && entry.target_host_profile == "host_run.profile.mobile"
            && entry.handoff_kind == StudioShellHandoffKind::PhoneShell
            && entry.consumer_id == "rusty-studio-phone-shell"
            && entry.target_kind == StudioShellTargetKind::Phone
            && entry.operator_shell_ids == vec!["shell.synthetic.phone_operator".to_string()]
    }));
    assert!(readiness.entries.iter().any(|entry| {
        entry.graph_id == "studio.graph.quest"
            && entry.target_host_profile == "host_run.profile.headset"
            && entry.handoff_kind == StudioShellHandoffKind::QuestShell
            && entry.consumer_id == "rusty-studio-quest-shell"
            && entry.target_kind == StudioShellTargetKind::Quest
            && entry.operator_shell_ids == vec!["shell.synthetic.quest_operator".to_string()]
    }));
}

#[test]
fn shell_handoff_readiness_reports_missing_bundles() {
    let root = temp_root("shell-handoff-readiness-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");

    let readiness = shell_handoff_readiness_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(readiness.status, StudioValidationStatus::Fail);
    assert_eq!(readiness.graph_count, 3);
    assert_eq!(readiness.ready_count, 0);
    assert_eq!(readiness.failed_count, 3);
    assert_eq!(readiness.missing_bundle_count, 3);
    assert_eq!(readiness.entries.len(), 3);
    assert_eq!(readiness.target_summaries.len(), 3);
    assert!(readiness.target_summaries.iter().all(|summary| {
        summary.graph_count == 1
            && summary.ready_count == 0
            && summary.failed_count == 1
            && summary.missing_bundle_count == 1
            && summary
                .issue_codes
                .iter()
                .any(|issue| issue == "studio.issue.shell_bundle_file_missing")
            && summary.bundle_dirs.len() == 1
            && summary.ready_bundle_dirs.is_empty()
            && summary.failed_bundle_dirs.len() == 1
            && summary.missing_bundle_dirs.len() == 1
            && summary.template_index_paths.len() == 1
            && summary.template_index_paths[0].ends_with("shell-templates.json")
    }));
    assert!(readiness.entries.iter().all(|entry| {
        entry.status == StudioValidationStatus::Fail
            && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            && entry.export_bundle_id == format!("studio.export.{}", entry.graph_id)
            && entry.package_count == 1
            && entry.module_count == 1
            && entry.operator_shell_count == 1
            && entry.failed_check_count > 0
    }));
}

#[test]
fn shell_handoff_manifest_archives_readiness_paths() {
    let root = temp_root("shell-handoff-manifest");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }

    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(manifest.schema_id, SHELL_HANDOFF_MANIFEST_SCHEMA);
    assert_eq!(
        manifest.manifest_id,
        "studio.shell_handoffs.studio.project.test"
    );
    assert_eq!(
        manifest.source_readiness_schema,
        SHELL_HANDOFF_READINESS_REPORT_SCHEMA
    );
    assert_eq!(manifest.status, StudioValidationStatus::Pass);
    assert_eq!(manifest.graph_count, 3);
    assert_eq!(manifest.ready_count, 3);
    assert_eq!(manifest.failed_count, 0);
    assert_eq!(manifest.missing_bundle_count, 0);
    assert_eq!(manifest.targets.len(), 3);
    assert_eq!(manifest.handoffs.len(), 3);
    assert_eq!(
        manifest.runtime_authority.command_session_authority,
        "rusty.manifold"
    );
    assert_eq!(
        manifest.runtime_authority.install_launch_evidence_authority,
        "rusty.hostess"
    );
    assert_eq!(
        manifest.runtime_authority.studio_role,
        "authoring.export_planning"
    );
    for target_kind in [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
    ] {
        let target = manifest
            .targets
            .iter()
            .find(|target| target.target_kind == target_kind)
            .expect("target manifest row");
        assert_eq!(target.graph_count, 1);
        assert_eq!(target.ready_count, 1);
        assert_eq!(target.failed_count, 0);
        assert_eq!(target.missing_bundle_count, 0);
        assert_eq!(target.bundle_dirs.len(), 1);
        assert_eq!(target.ready_bundle_dirs.len(), 1);
        assert!(target.failed_bundle_dirs.is_empty());
        assert!(target.missing_bundle_dirs.is_empty());
        assert_eq!(target.template_index_paths.len(), 1);
        assert!(target.template_index_paths[0].ends_with("shell-templates.json"));
    }
    assert!(manifest.handoffs.iter().all(|handoff| {
        handoff.status == StudioValidationStatus::Pass
            && handoff.validation_status == StudioValidationStatus::Pass
            && handoff.failed_check_count == 0
            && handoff.consumer_args.iter().any(|arg| arg == "--templates")
            && handoff
                .template_index_path
                .ends_with("shell-templates.json")
            && handoff.runtime_authority.is_some()
    }));
}

#[test]
fn shell_handoff_manifest_archives_missing_bundle_paths() {
    let root = temp_root("shell-handoff-manifest-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");

    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(manifest.schema_id, SHELL_HANDOFF_MANIFEST_SCHEMA);
    assert_eq!(manifest.status, StudioValidationStatus::Fail);
    assert_eq!(manifest.graph_count, 3);
    assert_eq!(manifest.ready_count, 0);
    assert_eq!(manifest.failed_count, 3);
    assert_eq!(manifest.missing_bundle_count, 3);
    assert_eq!(manifest.targets.len(), 3);
    assert_eq!(manifest.handoffs.len(), 3);
    assert!(manifest.targets.iter().all(|target| {
        target.ready_bundle_dirs.is_empty()
            && target.failed_bundle_dirs.len() == 1
            && target.missing_bundle_dirs.len() == 1
            && target
                .issue_codes
                .iter()
                .any(|issue| issue == "studio.issue.shell_bundle_file_missing")
            && target.template_index_paths[0].ends_with("shell-templates.json")
    }));
    assert!(manifest.handoffs.iter().all(|handoff| {
        handoff.status == StudioValidationStatus::Fail
            && handoff.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            && handoff.runtime_authority.is_none()
            && handoff.failed_check_count > 0
    }));
}

#[test]
fn shell_handoff_manifest_validation_accepts_archived_manifest() {
    let root = temp_root("shell-handoff-manifest-validation");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let manifest_path = root.join("shell-handoffs.json");
    save_json(&manifest_path, &manifest).expect("save shell handoff manifest");

    let loaded = load_shell_handoff_manifest(&manifest_path).expect("load handoff manifest");
    let validation = validate_shell_handoff_manifest(&loaded);

    assert_eq!(loaded, manifest);
    assert_eq!(
        validation.schema_id,
        SHELL_HANDOFF_MANIFEST_VALIDATION_REPORT_SCHEMA
    );
    assert_eq!(
        validation.manifest_id,
        "studio.shell_handoffs.studio.project.test"
    );
    assert_eq!(validation.status, StudioValidationStatus::Pass);
    assert!(validation
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
}

#[test]
fn shell_handoff_manifest_validation_rejects_authority_mismatch() {
    let root = temp_root("shell-handoff-manifest-authority-mismatch");
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

    let validation = validate_shell_handoff_manifest(&manifest);

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.runtime_authority_mismatch")
    }));
}

#[test]
fn shell_handoff_manifest_validation_rejects_target_count_mismatch() {
    let root = temp_root("shell-handoff-manifest-target-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    manifest.targets[0].ready_count = 0;

    let validation = validate_shell_handoff_manifest(&manifest);

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.target_summary_count_mismatch")
    }));
}

#[test]
fn shell_handoff_manifest_validation_rejects_missing_consumer_args() {
    let root = temp_root("shell-handoff-manifest-consumer-args");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let mut manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    manifest.handoffs[0].consumer_args.clear();

    let validation = validate_shell_handoff_manifest(&manifest);

    assert_eq!(validation.status, StudioValidationStatus::Fail);
    assert!(validation.checks.iter().any(|check| {
        check.issue_code.as_deref() == Some("studio.issue.handoff_consumer_args_mismatch")
    }));
}

#[test]
fn shell_handoff_intake_accepts_valid_manifest() {
    let root = temp_root("shell-handoff-intake");
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

    assert_eq!(intake.schema_id, SHELL_HANDOFF_INTAKE_REPORT_SCHEMA);
    assert_eq!(intake.status, StudioShellHandoffIntakeStatus::Accepted);
    assert_eq!(intake.issue_code, None);
    assert_eq!(intake.validation.status, StudioValidationStatus::Pass);
    assert_eq!(intake.accepted_count, 3);
    assert_eq!(intake.blocked_count, 0);
    assert_eq!(intake.target_summaries.len(), 3);
    assert_eq!(intake.entries.len(), 3);
    assert_eq!(intake.command_session_authority, "rusty.manifold");
    assert_eq!(intake.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(intake.studio_role, "authoring.export_planning");
    for target_kind in [
        StudioShellTargetKind::Desktop,
        StudioShellTargetKind::Phone,
        StudioShellTargetKind::Quest,
    ] {
        let summary = intake
            .target_summaries
            .iter()
            .find(|summary| summary.target_kind == target_kind)
            .expect("intake target summary");
        assert_eq!(summary.accepted_count, 1);
        assert_eq!(summary.blocked_count, 0);
        assert_eq!(summary.graph_ids.len(), 1);
        assert_eq!(summary.consumer_ids.len(), 1);
        assert_eq!(summary.bundle_dirs.len(), 1);
        assert_eq!(summary.template_index_paths.len(), 1);
    }
    assert!(intake.entries.iter().all(|entry| {
        entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner
            && entry.handoff_status == StudioValidationStatus::Pass
            && entry.issue_code.is_none()
            && entry.handoff_request_kind == "operator_shell_handoff"
            && entry.next_required_action == "stage_with_runtime_owner"
            && entry.command_session_authority == "rusty.manifold"
            && entry.install_launch_evidence_authority == "rusty.hostess"
            && entry.studio_role == "authoring.export_planning"
            && entry.consumer_args.iter().any(|arg| arg == "--templates")
    }));
}

#[test]
fn shell_runbook_reports_non_executed_owner_routes() {
    let root = temp_root("shell-runbook");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }

    let runbook = shell_runbook_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(runbook.schema_id, SHELL_RUNBOOK_REPORT_SCHEMA);
    assert_eq!(
        runbook.source_manifest_schema,
        SHELL_HANDOFF_MANIFEST_SCHEMA
    );
    assert_eq!(
        runbook.source_intake_schema,
        SHELL_HANDOFF_INTAKE_REPORT_SCHEMA
    );
    assert_eq!(runbook.status, StudioShellRunbookStatus::Ready);
    assert_eq!(runbook.issue_code, None);
    assert_eq!(runbook.ready_count, 3);
    assert_eq!(runbook.blocked_count, 0);
    assert_eq!(runbook.rejected_count, 0);
    assert_eq!(runbook.target_summaries.len(), 3);
    assert_eq!(
        runbook.prohibited_actions,
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
        let summary = runbook
            .target_summaries
            .iter()
            .find(|summary| summary.target_kind == target_kind)
            .expect("runbook target summary");
        assert_eq!(summary.ready_count, 1);
        assert_eq!(summary.blocked_count, 0);
        assert_eq!(summary.rejected_count, 0);
        assert_eq!(summary.responsible_owners, vec!["rusty.hostess"]);
        assert!(summary.runtime_route_kinds[0].ends_with("_operator_shell"));
        assert!(summary.issue_codes.is_empty());
    }

    let desktop = runbook
        .entries
        .iter()
        .find(|entry| entry.target_kind == StudioShellTargetKind::Desktop)
        .expect("desktop runbook row");
    assert_eq!(desktop.status, StudioShellRunbookStatus::Ready);
    assert_eq!(desktop.responsible_owner, "rusty.hostess");
    assert_eq!(desktop.command_session_authority, "rusty.manifold");
    assert_eq!(desktop.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(desktop.execution_policy, "not_executed.request_only");
    assert_eq!(desktop.runtime_route_kind, "desktop_operator_shell");
    assert_eq!(desktop.next_required_action, "stage_with_runtime_owner");
    assert_eq!(
        desktop.host_routes.install_route.as_deref(),
        Some("install.local_process")
    );
    assert_eq!(
        desktop.host_routes.launch_route.as_deref(),
        Some("launch.local_process")
    );
    assert_eq!(
        desktop.host_routes.command_bridge.as_deref(),
        Some("bridge.local_cli")
    );
    assert_eq!(
        desktop.host_routes.evidence_pull_route.as_deref(),
        Some("evidence.filesystem")
    );
    assert_eq!(
        desktop.cli_request[..5],
        ["cargo", "run", "-p", "rusty-studio-desktop-shell", "--"]
    );
    assert!(desktop
        .cli_request
        .iter()
        .any(|arg| arg.ends_with("shell-templates.json")));

    let phone = runbook
        .entries
        .iter()
        .find(|entry| entry.target_kind == StudioShellTargetKind::Phone)
        .expect("phone runbook row");
    assert_eq!(phone.consumer_id, "rusty-studio-phone-shell");
    assert_eq!(
        phone.host_routes.install_route.as_deref(),
        Some("install.android_package")
    );
    assert_eq!(
        phone.host_routes.evidence_pull_route.as_deref(),
        Some("evidence.adb_pull")
    );

    let quest = runbook
        .entries
        .iter()
        .find(|entry| entry.target_kind == StudioShellTargetKind::Quest)
        .expect("quest runbook row");
    assert_eq!(quest.consumer_id, "rusty-studio-quest-shell");
    assert_eq!(
        quest.host_routes.install_route.as_deref(),
        Some("install.android_package")
    );
    assert_eq!(
        quest.host_routes.launch_route.as_deref(),
        Some("launch.android_intent")
    );
}

#[test]
fn shell_runbook_blocks_missing_bundle_requests_without_execution_args() {
    let root = temp_root("shell-runbook-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");

    let runbook = shell_runbook_for_project(&project, Some(&root), &bundle_root);

    assert_eq!(runbook.status, StudioShellRunbookStatus::Blocked);
    assert_eq!(
        runbook.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert_eq!(runbook.ready_count, 0);
    assert_eq!(runbook.blocked_count, 3);
    assert_eq!(runbook.rejected_count, 0);
    assert!(runbook.entries.iter().all(|entry| {
        entry.status == StudioShellRunbookStatus::Blocked
            && entry.responsible_owner == "rusty.studio"
            && entry.cli_request.is_empty()
            && entry.route_status == StudioValidationStatus::Fail
            && entry.execution_policy == "not_executed.request_only"
    }));
}

#[test]
fn shell_runbook_blocks_template_index_graph_mismatch() {
    let root = temp_root("shell-runbook-template-mismatch");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    for graph in &project.graphs {
        let report = selected_shell_bundle_for_graph(&project, Some(&root), &graph.graph_id);
        save_shell_bundle(&bundle_root.join(&graph.graph_id), &report)
            .expect("save selected shell bundle");
    }
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let index_path = bundle_root
        .join("studio.graph.test")
        .join("shell-templates.json");
    let mut index = load_shell_template_index(&index_path).expect("load template index");
    index.templates[0].graph_id = "studio.graph.unrelated".to_string();
    save_json(&index_path, &index).expect("save mismatched template index");

    let runbook = shell_runbook_for_manifest(&manifest);

    assert_eq!(runbook.status, StudioShellRunbookStatus::Blocked);
    assert_eq!(runbook.ready_count, 2);
    assert_eq!(runbook.blocked_count, 1);
    let entry = runbook
        .entries
        .iter()
        .find(|entry| entry.graph_id == "studio.graph.test")
        .expect("mismatched runbook entry");
    assert_eq!(entry.status, StudioShellRunbookStatus::Blocked);
    assert_eq!(entry.responsible_owner, "rusty.studio");
    assert!(entry.cli_request.is_empty());
    assert_eq!(entry.route_status, StudioValidationStatus::Fail);
    assert_eq!(
        entry.route_issue_code.as_deref(),
        Some("studio.issue.shell_runbook_template_missing")
    );
    assert_eq!(
        entry.issue_code.as_deref(),
        Some("studio.issue.shell_runbook_template_missing")
    );
}

#[test]
fn shell_handoff_intake_rejects_invalid_manifest() {
    let root = temp_root("shell-handoff-intake-invalid");
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

    assert_eq!(intake.schema_id, SHELL_HANDOFF_INTAKE_REPORT_SCHEMA);
    assert_eq!(intake.status, StudioShellHandoffIntakeStatus::Rejected);
    assert_eq!(
        intake.issue_code.as_deref(),
        Some("studio.issue.runtime_authority_mismatch")
    );
    assert_eq!(intake.validation.status, StudioValidationStatus::Fail);
    assert_eq!(intake.accepted_count, 0);
    assert_eq!(intake.blocked_count, 0);
    assert!(intake.target_summaries.is_empty());
    assert!(intake.entries.is_empty());
    assert_eq!(intake.command_session_authority, "rusty.manifold");
    assert_eq!(intake.install_launch_evidence_authority, "rusty.hostess");
}

#[test]
fn shell_handoff_intake_blocks_missing_bundles_after_valid_manifest_shape() {
    let root = temp_root("shell-handoff-intake-missing");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("missing-selected-shells");
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

    let intake = shell_handoff_intake_for_manifest(&manifest);

    assert_eq!(intake.status, StudioShellHandoffIntakeStatus::Accepted);
    assert_eq!(intake.validation.status, StudioValidationStatus::Pass);
    assert_eq!(intake.accepted_count, 0);
    assert_eq!(intake.blocked_count, 3);
    assert_eq!(intake.target_summaries.len(), 3);
    assert_eq!(intake.entries.len(), 3);
    assert!(intake.target_summaries.iter().all(|summary| {
        summary.accepted_count == 0
            && summary.blocked_count == 1
            && summary.graph_ids.len() == 1
            && summary.consumer_ids.len() == 1
    }));
    assert!(intake.entries.iter().all(|entry| {
        entry.decision == StudioShellHandoffIntakeDecision::BlockedByHandoffIssue
            && entry.handoff_status == StudioValidationStatus::Fail
            && entry.issue_code.as_deref() == Some("studio.issue.shell_bundle_file_missing")
            && entry.next_required_action == "repair_export_bundle"
            && entry.consumer_args.is_empty()
    }));
}

#[test]
fn desktop_shell_handoff_rejects_unvalidated_bundle() {
    let root = temp_root("desktop-shell-handoff-reject");
    write_reference_fixture_tree(&root);
    let project = valid_shell_project_with_relative_references();
    let output_dir = root.join("missing-selected-shell");

    let handoff =
        desktop_shell_handoff_for_bundle(&project, Some(&root), "studio.graph.test", &output_dir);

    assert_eq!(handoff.status, StudioValidationStatus::Fail);
    assert_eq!(
        handoff.issue_code.as_deref(),
        Some("studio.issue.shell_bundle_file_missing")
    );
    assert!(handoff.consumer_args.is_empty());
    assert_eq!(handoff.validation.status, StudioValidationStatus::Fail);
}
