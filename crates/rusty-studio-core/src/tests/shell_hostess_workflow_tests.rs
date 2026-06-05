use super::*;

#[test]
fn shell_hostess_handoff_package_summarizes_selected_candidate() {
    let root = temp_root("shell-hostess-handoff-package");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    save_selected_shell_bundles(&project, &root, &bundle_root);
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);
    let manifest_path = root.join("shell-handoffs.json");
    save_json(&manifest_path, &manifest).expect("save shell handoff manifest");

    let acceptance_checklist = shell_handoff_acceptance_checklist_for_intake(
        &shell_handoff_intake_for_manifest(&manifest),
    );
    let acceptance_checklist_path = root.join("shell-handoff-acceptance-checklist.json");
    save_json(&acceptance_checklist_path, &acceptance_checklist)
        .expect("save acceptance checklist");
    let acceptance_baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &acceptance_checklist,
        &acceptance_checklist_path,
        Some("synthetic-ready"),
        Some("Synthetic ready acceptance baseline"),
    );
    let acceptance_baseline_path = root.join("shell-handoff-acceptance-baseline.json");
    save_json(&acceptance_baseline_path, &acceptance_baseline).expect("save acceptance baseline");
    let acceptance_index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(acceptance_baseline, Some(acceptance_baseline_path))],
        Some("synthetic-ready"),
    );
    let acceptance_index_path = root.join("shell-handoff-acceptance-baselines.json");
    save_json(&acceptance_index_path, &acceptance_index).expect("save acceptance index");

    let export_package = shell_export_package_for_manifest(&manifest);
    let export_package_path = root.join("shell-export-package.json");
    save_json(&export_package_path, &export_package).expect("save export package");
    let export_package_baseline = shell_export_package_baseline_manifest_for_report(
        &export_package,
        &export_package_path,
        Some("synthetic-ready-package"),
        Some("Synthetic ready export package baseline"),
    );
    let export_package_baseline_path = root.join("shell-export-package-baseline.json");
    save_json(&export_package_baseline_path, &export_package_baseline)
        .expect("save export package baseline");
    let export_package_index = shell_export_package_baseline_index_for_manifests(
        vec![(export_package_baseline, Some(export_package_baseline_path))],
        Some("synthetic-ready-package"),
    );
    let export_package_index_path = root.join("shell-export-package-baselines.json");
    save_json(&export_package_index_path, &export_package_index)
        .expect("save export package index");

    let ready_review = shell_release_candidate_review_for_manifest(
        &manifest,
        Some(&manifest_path),
        &acceptance_index,
        Some(&acceptance_index_path),
        Some("synthetic-ready"),
        &export_package_index,
        Some(&export_package_index_path),
        Some("synthetic-ready-package"),
    );
    let ready_review_path = root.join("shell-release-candidate-review.json");
    save_json(&ready_review_path, &ready_review).expect("save ready review");
    let ready_candidate = shell_release_candidate_review_manifest_for_report(
        &ready_review,
        &ready_review_path,
        Some("synthetic-ready-candidate"),
        Some("Synthetic ready release candidate"),
    );
    let ready_candidate_path = root.join("shell-release-candidate-review-manifest.json");
    save_json(&ready_candidate_path, &ready_candidate).expect("save ready candidate");
    let index = shell_release_candidate_review_index_for_manifests(
        vec![(ready_candidate, Some(ready_candidate_path.clone()))],
        Some("synthetic-ready-candidate"),
    );
    let index_path = root.join("shell-release-candidate-reviews.json");
    save_json(&index_path, &index).expect("save release candidate index");

    let package =
        shell_hostess_handoff_package_for_release_candidate_index(&index, Some(&index_path), None);

    assert_eq!(package.schema_id, SHELL_HOSTESS_HANDOFF_PACKAGE_SCHEMA);
    assert_eq!(
        package.source_index_schema,
        SHELL_RELEASE_CANDIDATE_REVIEW_INDEX_SCHEMA
    );
    assert_eq!(
        package.status,
        StudioShellHostessHandoffPackageStatus::Ready
    );
    assert_eq!(package.issue_code, None);
    assert_eq!(
        package.selected_candidate_id.as_deref(),
        Some("synthetic-ready-candidate")
    );
    assert_eq!(
        package.candidate_manifest_schema.as_deref(),
        Some(SHELL_RELEASE_CANDIDATE_REVIEW_MANIFEST_SCHEMA)
    );
    assert_eq!(
        package.candidate_manifest_path.as_deref(),
        Some(ready_candidate_path.display().to_string().as_str())
    );
    assert_eq!(
        package.review_schema.as_deref(),
        Some(SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA)
    );
    assert_eq!(
        package.handoff_manifest_path.as_deref(),
        Some(manifest_path.display().to_string().as_str())
    );
    assert_eq!(
        package.acceptance_baseline_id.as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(
        package.acceptance_comparison_status,
        Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
    );
    assert_eq!(
        package.export_package_baseline_id.as_deref(),
        Some("synthetic-ready-package")
    );
    assert_eq!(
        package.export_package_comparison_status,
        Some(StudioShellExportPackageComparisonStatus::Unchanged)
    );
    assert_eq!(
        package.command_session_authority.as_deref(),
        Some("rusty.manifold")
    );
    assert_eq!(
        package.install_launch_evidence_authority.as_deref(),
        Some("rusty.hostess")
    );
    assert_eq!(
        package.studio_role.as_deref(),
        Some("authoring.export_planning")
    );
    assert!(package
        .required_owner_actions
        .iter()
        .any(
            |action| action.action_id == "hostess.stage_generated_shells"
                && action.owner == "rusty.hostess"
                && action.status == StudioShellHostessHandoffPackageActionStatus::Ready
                && action.prohibited_in_studio
        ));
    assert!(package.required_owner_actions.iter().any(|action| {
        action.action_id == "manifold.review_command_session_contract"
            && action.owner == "rusty.manifold"
            && action.status == StudioShellHostessHandoffPackageActionStatus::Ready
    }));
    for prohibited in [
        "stage_generated_shells",
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
        "collect_install_launch_evidence",
    ] {
        assert!(package.prohibited_actions.contains(&prohibited.to_string()));
    }
    assert!(package
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let package_path = root.join("shell-hostess-handoff-package.json");
    save_json(&package_path, &package).expect("save Hostess handoff package");
    let intake = shell_hostess_owner_intake_for_handoff_package(&package, Some(&package_path));

    assert_eq!(intake.schema_id, SHELL_HOSTESS_OWNER_INTAKE_SCHEMA);
    assert_eq!(
        intake.source_package_schema,
        SHELL_HOSTESS_HANDOFF_PACKAGE_SCHEMA
    );
    assert_eq!(
        intake.package_path.as_deref(),
        Some(package_path.display().to_string().as_str())
    );
    assert_eq!(intake.status, StudioShellHostessOwnerIntakeStatus::Ready);
    assert_eq!(intake.issue_code, None);
    assert_eq!(intake.execution_policy, "not_executed.request_only");
    assert_eq!(intake.intake_owner, "rusty.hostess");
    assert_eq!(intake.handoff_owner, "rusty.hostess");
    assert_eq!(
        intake.command_session_authority.as_deref(),
        Some("rusty.manifold")
    );
    assert_eq!(
        intake.install_launch_evidence_authority.as_deref(),
        Some("rusty.hostess")
    );
    assert_eq!(
        intake.studio_role.as_deref(),
        Some("authoring.export_planning")
    );
    assert_eq!(intake.source_owner_action_count, 4);
    assert_eq!(intake.ready_assignment_count, 4);
    assert_eq!(intake.blocked_assignment_count, 0);
    assert_eq!(intake.hostess_ready_action_count, 3);
    assert_eq!(intake.manifold_ready_action_count, 1);
    assert!(intake.assignments.iter().any(|assignment| {
        assignment.action_id == "hostess.stage_generated_shells"
            && assignment.owner == "rusty.hostess"
            && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
            && assignment.request_kind == "hostess_owner_action_request"
            && assignment.prohibited_in_studio
    }));
    assert!(intake.assignments.iter().any(|assignment| {
        assignment.action_id == "manifold.review_command_session_contract"
            && assignment.owner == "rusty.manifold"
            && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
            && assignment.request_kind == "manifold_owner_review_request"
    }));
    assert!(intake
        .prohibited_actions
        .contains(&"collect_install_launch_evidence".to_string()));
    assert!(intake
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let intake_path = root.join("shell-hostess-owner-intake.json");
    save_json(&intake_path, &intake).expect("save Hostess owner intake");
    let staging = shell_hostess_staging_preview_for_owner_intake(&intake, Some(&intake_path));

    assert_eq!(
        staging.schema_id,
        SHELL_HOSTESS_STAGING_PREVIEW_MANIFEST_SCHEMA
    );
    assert_eq!(
        staging.source_intake_schema,
        SHELL_HOSTESS_OWNER_INTAKE_SCHEMA
    );
    assert_eq!(
        staging.source_handoff_manifest_schema.as_deref(),
        Some(SHELL_HANDOFF_MANIFEST_SCHEMA)
    );
    assert_eq!(
        staging.intake_path.as_deref(),
        Some(intake_path.display().to_string().as_str())
    );
    assert_eq!(
        staging.status,
        StudioShellHostessStagingPreviewStatus::Ready
    );
    assert_eq!(staging.issue_code, None);
    assert_eq!(staging.execution_policy, "not_executed.preview_only");
    assert_eq!(staging.staging_owner, "rusty.hostess");
    assert_eq!(
        staging.command_session_authority.as_deref(),
        Some("rusty.manifold")
    );
    assert_eq!(
        staging.install_launch_evidence_authority.as_deref(),
        Some("rusty.hostess")
    );
    assert_eq!(
        staging.studio_role.as_deref(),
        Some("authoring.export_planning")
    );
    assert_eq!(staging.assignment_count, 4);
    assert_eq!(staging.ready_assignment_count, 4);
    assert_eq!(staging.blocked_assignment_count, 0);
    assert_eq!(staging.ready_group_count, 4);
    assert_eq!(staging.blocked_group_count, 0);
    assert!(staging.expected_artifact_count >= 18);
    let stage_group = staging
        .groups
        .iter()
        .find(|group| group.action_id == "hostess.stage_generated_shells")
        .expect("stage generated shells group");
    assert_eq!(stage_group.route_kind, "hostess.stage.generated_shells");
    assert_eq!(
        stage_group.status,
        StudioShellHostessStagingPreviewGroupStatus::Ready
    );
    assert!(stage_group
        .expected_artifacts
        .iter()
        .any(|artifact| artifact.artifact_kind == "shell_descriptor"));
    assert!(stage_group
        .expected_artifacts
        .iter()
        .any(|artifact| artifact.artifact_kind == "manifold_shell_handoff"));
    assert!(stage_group
        .expected_artifacts
        .iter()
        .any(|artifact| artifact.artifact_kind == "shell_template_manifest"));
    assert!(stage_group
        .expected_artifacts
        .iter()
        .any(|artifact| artifact.artifact_kind == "hostess_owner_intake"));
    let manifold_group = staging
        .groups
        .iter()
        .find(|group| group.action_id == "manifold.review_command_session_contract")
        .expect("Manifold review group");
    assert_eq!(
        manifold_group.route_kind,
        "manifold.review.command_session_contract"
    );
    assert!(manifold_group
        .expected_artifacts
        .iter()
        .any(|artifact| artifact.route_hint.is_some()));
    assert!(manifold_group.expected_artifacts.iter().any(|artifact| {
        artifact.artifact_kind == "manifold_shell_handoff"
            && artifact.route_hint.as_deref() == Some("manifold.shell_handoff_review")
    }));
    assert!(staging
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let staging_path = root.join("shell-hostess-staging-preview.json");
    save_json(&staging_path, &staging).expect("save Hostess staging preview");
    let file_plan = shell_hostess_staging_file_plan_for_preview(&staging, Some(&staging_path));

    assert_eq!(file_plan.schema_id, SHELL_HOSTESS_STAGING_FILE_PLAN_SCHEMA);
    assert_eq!(
        file_plan.source_preview_schema,
        SHELL_HOSTESS_STAGING_PREVIEW_MANIFEST_SCHEMA
    );
    assert_eq!(
        file_plan.preview_path.as_deref(),
        Some(staging_path.display().to_string().as_str())
    );
    assert_eq!(
        file_plan.status,
        StudioShellHostessStagingFilePlanStatus::Ready
    );
    assert_eq!(file_plan.issue_code, None);
    assert_eq!(file_plan.execution_policy, "not_executed.dry_run_only");
    assert_eq!(file_plan.staging_owner, "rusty.hostess");
    assert_eq!(file_plan.preview_group_count, 4);
    assert_eq!(file_plan.ready_preview_group_count, 4);
    assert_eq!(file_plan.blocked_preview_group_count, 0);
    assert_eq!(
        file_plan.source_artifact_count,
        staging.expected_artifact_count
    );
    assert_eq!(file_plan.planned_file_count, 17);
    assert!(file_plan.duplicate_artifact_count > 0);
    assert_eq!(file_plan.request_count, 4);
    assert_eq!(file_plan.ready_request_count, 4);
    assert_eq!(file_plan.blocked_request_count, 0);
    assert_eq!(file_plan.target_request_count, 3);
    assert_eq!(file_plan.shared_request_count, 1);
    let shared_request = file_plan
        .requests
        .iter()
        .find(|request| request.target_key == "shared")
        .expect("shared staging request");
    assert_eq!(
        shared_request.status,
        StudioShellHostessStagingFileRequestStatus::Ready
    );
    assert!(shared_request
        .planned_files
        .iter()
        .any(|file| file.artifact_kind == "hostess_owner_intake"
            && file.destination_path
                == "hostess-staging/shared/hostess/hostess-owner-intake.json"));
    assert!(shared_request
        .planned_files
        .iter()
        .any(|file| file.artifact_kind == "shell_handoff_manifest"
            && file.source_route_kinds.len() > 1));
    let desktop_request = file_plan
        .requests
        .iter()
        .find(|request| request.target_kind == Some(StudioShellTargetKind::Desktop))
        .expect("desktop staging request");
    assert_eq!(
        desktop_request.status,
        StudioShellHostessStagingFileRequestStatus::Ready
    );
    for artifact_kind in [
        "shell_bundle_dir",
        "shell_descriptor",
        "manifold_shell_handoff",
        "shell_template_manifest",
    ] {
        assert!(desktop_request
            .planned_files
            .iter()
            .any(|file| file.artifact_kind == artifact_kind));
    }
    assert!(desktop_request.planned_files.iter().any(|file| {
        file.artifact_kind == "manifold_shell_handoff"
            && file.destination_path
                == "hostess-staging/targets/desktop/studio.graph.test/manifold/studio.graph.test.manifold-shell-handoff.json"
            && file
                .route_hints
                .contains(&"manifold.shell_handoff_review".to_string())
            && file.source_route_kinds.len() > 1
    }));
    assert!(file_plan.requests.iter().all(|request| {
        request.owner == "rusty.hostess"
            && request.planned_file_count == request.planned_files.len()
            && request.planned_files.iter().all(|file| {
                !file.source_action_ids.is_empty()
                    && !file.source_route_kinds.is_empty()
                    && is_safe_relative_manifest_path(&file.destination_path)
            })
    }));
    assert!(file_plan
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let file_plan_path = root.join("shell-hostess-staging-file-plan.json");
    save_json(&file_plan_path, &file_plan).expect("save Hostess staging file plan");
    let envelope =
        shell_hostess_staging_handoff_envelope_for_file_plan(&file_plan, Some(&file_plan_path));

    assert_eq!(
        envelope.schema_id,
        SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA
    );
    assert_eq!(
        envelope.source_file_plan_schema,
        SHELL_HOSTESS_STAGING_FILE_PLAN_SCHEMA
    );
    assert_eq!(
        envelope.file_plan_path.as_deref(),
        Some(file_plan_path.display().to_string().as_str())
    );
    assert_eq!(
        envelope.status,
        StudioShellHostessStagingHandoffEnvelopeStatus::Ready
    );
    assert_eq!(envelope.issue_code, None);
    assert_eq!(envelope.execution_policy, "not_executed.handoff_only");
    assert_eq!(envelope.handoff_owner, "rusty.hostess");
    assert_eq!(envelope.staging_owner, "rusty.hostess");
    assert_eq!(envelope.planned_file_count, file_plan.planned_file_count);
    assert_eq!(envelope.request_count, file_plan.request_count);
    assert_eq!(envelope.ready_request_count, file_plan.ready_request_count);
    assert_eq!(envelope.blocked_request_count, 0);
    assert_eq!(envelope.target_request_count, 3);
    assert_eq!(envelope.shared_request_count, 1);
    assert_eq!(envelope.instruction_count, 4);
    assert_eq!(envelope.ready_instruction_count, 4);
    assert_eq!(envelope.blocked_instruction_count, 0);
    assert_eq!(
        envelope.envelope_id,
        "studio.hostess_staging_handoff.studio.project.test.rev1"
    );
    assert_eq!(
        envelope.provenance.checksum_algorithm,
        "fnv1a64.studio_staging_file_plan.v1"
    );
    assert_eq!(envelope.provenance.plan_checksum.len(), 16);
    assert!(envelope
        .provenance
        .source_artifact_kinds
        .contains(&"shell_template_manifest".to_string()));
    assert!(envelope
        .provenance
        .source_route_kinds
        .contains(&"hostess.stage.generated_shells".to_string()));
    assert!(envelope
        .provenance
        .source_action_ids
        .contains(&"hostess.stage_generated_shells".to_string()));
    assert!(envelope
        .provenance
        .target_keys
        .contains(&"shared".to_string()));
    assert!(envelope
        .request_summaries
        .iter()
        .any(
            |summary| summary.target_kind == Some(StudioShellTargetKind::Desktop)
                && summary.planned_file_count == 4
        ));
    assert!(envelope.owner_instructions.iter().any(|instruction| {
        instruction.instruction_id == "hostess.copy_staging_files"
            && instruction.owner == "rusty.hostess"
            && instruction.route_kind == "hostess.stage.files_from_plan"
            && instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Ready
            && instruction.prohibited_in_studio
    }));
    assert!(envelope.owner_instructions.iter().any(|instruction| {
        instruction.instruction_id == "manifold.review_command_session_contract"
            && instruction.owner == "rusty.manifold"
            && instruction.route_kind == "manifold.review.command_session_contract"
            && instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Ready
    }));
    assert!(envelope
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let handoff_path = root.join("shell-hostess-staging-handoff.json");
    save_json(&handoff_path, &envelope).expect("save Hostess staging handoff");
    let staging_acceptance =
        shell_hostess_staging_acceptance_checklist_for_handoff(&envelope, Some(&handoff_path));

    assert_eq!(
        staging_acceptance.schema_id,
        SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA
    );
    assert_eq!(
        staging_acceptance.source_handoff_schema,
        SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA
    );
    assert_eq!(
        staging_acceptance.handoff_path.as_deref(),
        Some(handoff_path.display().to_string().as_str())
    );
    assert_eq!(
        staging_acceptance.status,
        StudioShellHostessStagingAcceptanceStatus::Ready
    );
    assert_eq!(staging_acceptance.issue_code, None);
    assert_eq!(
        staging_acceptance.execution_policy,
        "not_executed.acceptance_check_only"
    );
    assert_eq!(staging_acceptance.checklist_owner, "rusty.hostess");
    assert_eq!(staging_acceptance.handoff_owner, "rusty.hostess");
    assert_eq!(staging_acceptance.staging_owner, "rusty.hostess");
    assert_eq!(
        staging_acceptance.envelope_id,
        "studio.hostess_staging_handoff.studio.project.test.rev1"
    );
    assert_eq!(
        staging_acceptance.plan_checksum,
        envelope.provenance.plan_checksum
    );
    assert_eq!(staging_acceptance.ready_item_count, 6);
    assert_eq!(staging_acceptance.blocked_item_count, 0);
    assert_eq!(staging_acceptance.rejected_item_count, 0);
    assert_eq!(staging_acceptance.request_count, envelope.request_count);
    assert_eq!(
        staging_acceptance.instruction_count,
        envelope.instruction_count
    );
    assert!(staging_acceptance.entries.iter().any(|entry| {
        entry.item_id == "hostess.copy_staging_files"
            && entry.owner == "rusty.hostess"
            && entry.route_kind == "hostess.stage.files_from_plan"
            && entry.status == StudioShellHostessStagingAcceptanceStatus::Ready
            && entry.prohibited_in_studio
    }));
    assert!(staging_acceptance.entries.iter().any(|entry| {
        entry.item_id == "manifold.review_command_session_contract"
            && entry.owner == "rusty.manifold"
            && entry.route_kind == "manifold.review.command_session_contract"
    }));
    assert!(staging_acceptance
        .handoff_checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    std::fs::remove_file(
        bundle_root
            .join("studio.graph.phone")
            .join("shells/phone/studio.graph.phone.shell-template.json"),
    )
    .expect("remove phone template manifest");
    let blocked_review = shell_release_candidate_review_for_manifest(
        &manifest,
        Some(&manifest_path),
        &acceptance_index,
        Some(&acceptance_index_path),
        Some("synthetic-ready"),
        &export_package_index,
        Some(&export_package_index_path),
        Some("synthetic-ready-package"),
    );
    let blocked_review_path = root.join("shell-release-candidate-review-blocked.json");
    save_json(&blocked_review_path, &blocked_review).expect("save blocked review");
    let blocked_candidate = shell_release_candidate_review_manifest_for_report(
        &blocked_review,
        &blocked_review_path,
        Some("synthetic-blocked-candidate"),
        Some("Synthetic blocked release candidate"),
    );
    let blocked_candidate_path = root.join("shell-release-candidate-blocked-manifest.json");
    save_json(&blocked_candidate_path, &blocked_candidate).expect("save blocked candidate");
    let blocked_index = append_shell_release_candidate_review_index_manifests(
        &index,
        vec![(blocked_candidate, Some(blocked_candidate_path))],
        Some("synthetic-blocked-candidate"),
    );

    let blocked_package = shell_hostess_handoff_package_for_release_candidate_index(
        &blocked_index,
        Some(&index_path),
        None,
    );

    assert_eq!(
        blocked_package.status,
        StudioShellHostessHandoffPackageStatus::Blocked
    );
    assert_eq!(
        blocked_package.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert!(blocked_package.required_owner_actions.iter().all(|action| {
        action.status == StudioShellHostessHandoffPackageActionStatus::Blocked
            && action.issue_code.as_deref()
                == Some("studio.issue.shell_export_package_template_load_failed")
    }));

    let blocked_intake =
        shell_hostess_owner_intake_for_handoff_package(&blocked_package, Some(&package_path));
    assert_eq!(
        blocked_intake.status,
        StudioShellHostessOwnerIntakeStatus::Blocked
    );
    assert_eq!(
        blocked_intake.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert_eq!(blocked_intake.ready_assignment_count, 0);
    assert_eq!(blocked_intake.blocked_assignment_count, 4);
    assert!(blocked_intake.assignments.iter().all(|assignment| {
        assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Blocked
            && assignment.issue_code.as_deref()
                == Some("studio.issue.shell_export_package_template_load_failed")
    }));

    let blocked_staging =
        shell_hostess_staging_preview_for_owner_intake(&blocked_intake, Some(&intake_path));
    assert_eq!(
        blocked_staging.status,
        StudioShellHostessStagingPreviewStatus::Blocked
    );
    assert_eq!(
        blocked_staging.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert_eq!(blocked_staging.ready_group_count, 0);
    assert_eq!(blocked_staging.blocked_group_count, 4);
    assert!(blocked_staging.groups.iter().all(|group| {
        group.status == StudioShellHostessStagingPreviewGroupStatus::Blocked
            && group.issue_code.as_deref()
                == Some("studio.issue.shell_export_package_template_load_failed")
    }));

    let blocked_file_plan =
        shell_hostess_staging_file_plan_for_preview(&blocked_staging, Some(&staging_path));
    assert_eq!(
        blocked_file_plan.status,
        StudioShellHostessStagingFilePlanStatus::Blocked
    );
    assert_eq!(
        blocked_file_plan.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert_eq!(blocked_file_plan.ready_request_count, 0);
    assert_eq!(
        blocked_file_plan.blocked_request_count,
        blocked_file_plan.request_count
    );
    assert!(blocked_file_plan.requests.iter().all(|request| {
        request.status == StudioShellHostessStagingFileRequestStatus::Blocked
            && request.issue_code.as_deref()
                == Some("studio.issue.shell_export_package_template_load_failed")
    }));

    let blocked_envelope = shell_hostess_staging_handoff_envelope_for_file_plan(
        &blocked_file_plan,
        Some(&file_plan_path),
    );
    assert_eq!(
        blocked_envelope.status,
        StudioShellHostessStagingHandoffEnvelopeStatus::Blocked
    );
    assert_eq!(
        blocked_envelope.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert_eq!(blocked_envelope.ready_instruction_count, 0);
    assert_eq!(blocked_envelope.blocked_instruction_count, 4);
    assert!(blocked_envelope
        .owner_instructions
        .iter()
        .all(|instruction| {
            instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Blocked
                && instruction.issue_code.as_deref()
                    == Some("studio.issue.shell_export_package_template_load_failed")
        }));

    let blocked_acceptance = shell_hostess_staging_acceptance_checklist_for_handoff(
        &blocked_envelope,
        Some(&handoff_path),
    );
    assert_eq!(
        blocked_acceptance.status,
        StudioShellHostessStagingAcceptanceStatus::Blocked
    );
    assert_eq!(
        blocked_acceptance.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert_eq!(blocked_acceptance.ready_item_count, 0);
    assert_eq!(blocked_acceptance.blocked_item_count, 6);
    assert_eq!(blocked_acceptance.rejected_item_count, 0);
    assert!(blocked_acceptance.entries.iter().all(|entry| {
        entry.status == StudioShellHostessStagingAcceptanceStatus::Blocked
            && entry.issue_code.as_deref()
                == Some("studio.issue.shell_export_package_template_load_failed")
    }));

    let ready_acceptance_path = root.join("shell-hostess-staging-acceptance-ready.json");
    let blocked_acceptance_path = root.join("shell-hostess-staging-acceptance-blocked.json");
    let ready_manifest_path = root.join("shell-hostess-staging-acceptance-ready-manifest.json");
    let blocked_manifest_path = root.join("shell-hostess-staging-acceptance-blocked-manifest.json");
    let ready_acceptance = shell_hostess_staging_acceptance_manifest_for_checklist(
        &staging_acceptance,
        &ready_acceptance_path,
        None,
        None,
    );
    let blocked_acceptance_manifest = shell_hostess_staging_acceptance_manifest_for_checklist(
        &blocked_acceptance,
        &blocked_acceptance_path,
        Some("synthetic-blocked-hostess-acceptance"),
        Some("Synthetic blocked Hostess staging acceptance"),
    );

    assert_eq!(
        ready_acceptance.schema_id,
        SHELL_HOSTESS_STAGING_ACCEPTANCE_MANIFEST_SCHEMA
    );
    assert_eq!(
        ready_acceptance.acceptance_id,
        "studio.hostess_staging_acceptance.studio.project.test.rev1.ready"
    );
    assert_eq!(
        ready_acceptance.label,
        "studio.project.test revision 1 ready Hostess staging acceptance"
    );
    assert_eq!(
        ready_acceptance.checklist_path,
        ready_acceptance_path.display().to_string()
    );
    assert_eq!(
        ready_acceptance.checklist_schema,
        SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA
    );
    assert_eq!(
        ready_acceptance.envelope_id,
        "studio.hostess_staging_handoff.studio.project.test.rev1"
    );
    assert_eq!(
        ready_acceptance.status,
        StudioShellHostessStagingAcceptanceStatus::Ready
    );
    assert_eq!(ready_acceptance.ready_item_count, 6);
    assert_eq!(ready_acceptance.blocked_item_count, 0);
    assert_eq!(
        ready_acceptance.request_count,
        staging_acceptance.request_count
    );
    assert_eq!(
        ready_acceptance.execution_policy,
        "not_executed.acceptance_check_only"
    );
    assert_eq!(
        ready_acceptance.command_session_authority.as_deref(),
        Some("rusty.manifold")
    );
    assert_eq!(
        ready_acceptance
            .install_launch_evidence_authority
            .as_deref(),
        Some("rusty.hostess")
    );
    assert_eq!(
        ready_acceptance.plan_checksum,
        staging_acceptance.plan_checksum
    );
    assert_eq!(
        blocked_acceptance_manifest.acceptance_id,
        "synthetic-blocked-hostess-acceptance"
    );
    assert_eq!(
        blocked_acceptance_manifest.status,
        StudioShellHostessStagingAcceptanceStatus::Blocked
    );
    assert_eq!(blocked_acceptance_manifest.ready_item_count, 0);
    assert_eq!(blocked_acceptance_manifest.blocked_item_count, 6);

    let index = shell_hostess_staging_acceptance_index_for_manifests(
        vec![
            (ready_acceptance.clone(), Some(ready_manifest_path.clone())),
            (
                blocked_acceptance_manifest.clone(),
                Some(blocked_manifest_path.clone()),
            ),
        ],
        Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready"),
    );

    assert_eq!(
        index.schema_id,
        SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA
    );
    assert_eq!(index.project_ids, vec!["studio.project.test"]);
    assert_eq!(
        index.envelope_ids,
        vec!["studio.hostess_staging_handoff.studio.project.test.rev1"]
    );
    assert_eq!(
        index.default_acceptance_id.as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
    );
    assert_eq!(index.acceptance_count, 2);
    assert_eq!(index.ready_acceptance_count, 1);
    assert_eq!(index.blocked_acceptance_count, 1);
    assert_eq!(index.rejected_acceptance_count, 0);
    assert_eq!(index.entries.len(), 2);
    assert_eq!(
        index.entries[0].acceptance_id,
        "studio.hostess_staging_acceptance.studio.project.test.rev1.ready"
    );
    assert_eq!(index.entries[0].ready_item_count, 6);
    assert_eq!(
        index.entries[0].acceptance_manifest_path.as_deref(),
        Some(ready_manifest_path.display().to_string().as_str())
    );
    assert_eq!(
        index.entries[1].acceptance_id,
        "synthetic-blocked-hostess-acceptance"
    );
    assert_eq!(index.entries[1].blocked_item_count, 6);
    assert_eq!(
        select_shell_hostess_staging_acceptance_index_entry(&index, None)
            .map(|entry| entry.acceptance_id.as_str()),
        Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
    );
    assert_eq!(
        select_shell_hostess_staging_acceptance_index_entry(
            &index,
            Some("synthetic-blocked-hostess-acceptance")
        )
        .map(|entry| entry.status),
        Some(StudioShellHostessStagingAcceptanceStatus::Blocked)
    );
    assert!(select_shell_hostess_staging_acceptance_index_entry(&index, Some("missing")).is_none());

    let selection = summarize_shell_hostess_staging_acceptance_index_selection(
        &index,
        Some(&root.join("shell-hostess-staging-acceptances.json")),
        None,
    );
    assert_eq!(
        selection.schema_id,
        SHELL_HOSTESS_STAGING_ACCEPTANCE_SELECTION_SCHEMA
    );
    assert_eq!(
        selection.source_index_schema,
        SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA
    );
    assert_eq!(
        selection.status,
        StudioShellHostessStagingAcceptanceSelectionStatus::Selected
    );
    assert_eq!(
        selection.selected_acceptance_id.as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
    );
    assert_eq!(selection.acceptance_count, 2);
    assert!(selection.entries.iter().any(|entry| entry.acceptance_id
        == "studio.hostess_staging_acceptance.studio.project.test.rev1.ready"
        && entry.selected
        && entry.default
        && entry.ready_item_count == 6));
    let missing_selection =
        summarize_shell_hostess_staging_acceptance_index_selection(&index, None, Some("missing"));
    assert_eq!(
        missing_selection.status,
        StudioShellHostessStagingAcceptanceSelectionStatus::Missing
    );
    assert_eq!(
        missing_selection.issue_code.as_deref(),
        Some("studio.issue.shell_hostess_staging_acceptance_not_found")
    );
    let empty_index = shell_hostess_staging_acceptance_index_for_manifests(Vec::new(), None);
    let empty_selection =
        summarize_shell_hostess_staging_acceptance_index_selection(&empty_index, None, None);
    assert_eq!(
        empty_selection.status,
        StudioShellHostessStagingAcceptanceSelectionStatus::Empty
    );

    let direct_comparison = compare_shell_hostess_staging_acceptance_against_manifest(
        &ready_acceptance,
        &staging_acceptance,
        &staging_acceptance,
    );
    assert_eq!(
        direct_comparison.schema_id,
        SHELL_HOSTESS_STAGING_ACCEPTANCE_COMPARISON_SCHEMA
    );
    assert_eq!(
        direct_comparison.status,
        StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged
    );
    assert_eq!(
        direct_comparison.baseline_acceptance_id.as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
    );
    assert_eq!(direct_comparison.ready_item_delta, 0);
    assert_eq!(direct_comparison.blocked_item_delta, 0);
    assert_eq!(direct_comparison.rejected_item_delta, 0);
    assert_eq!(direct_comparison.entries.len(), 6);
    assert!(direct_comparison
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert!(direct_comparison.entries.iter().all(|entry| {
        entry.change == StudioShellHostessStagingAcceptanceComparisonChange::Unchanged
    }));

    let mut changed_contract_candidate = staging_acceptance.clone();
    changed_contract_candidate
        .entries
        .iter_mut()
        .find(|entry| entry.item_id == "hostess.accept_staging_handoff")
        .expect("acceptance row")
        .owner = "rusty.studio".to_string();
    changed_contract_candidate
        .entries
        .iter_mut()
        .find(|entry| entry.item_id == "hostess.copy_staging_files")
        .expect("copy row")
        .route_kind = "hostess.stage.files_from_drifted_plan".to_string();
    let review_entry = changed_contract_candidate
        .entries
        .iter_mut()
        .find(|entry| entry.item_id == "hostess.review_staging_file_requests")
        .expect("review row");
    review_entry.prohibited_in_studio = false;
    review_entry.expected_input_path = Some("target/drifted-input.json".to_string());
    let changed_contract_comparison = compare_shell_hostess_staging_acceptance_against_manifest(
        &ready_acceptance,
        &staging_acceptance,
        &changed_contract_candidate,
    );
    assert_eq!(
        changed_contract_comparison.status,
        StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable
    );
    assert_eq!(
        changed_contract_comparison.issue_code.as_deref(),
        Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
    );
    assert_eq!(changed_contract_comparison.ready_item_delta, 0);
    assert_eq!(changed_contract_comparison.blocked_item_delta, 0);
    assert_eq!(changed_contract_comparison.rejected_item_delta, 0);
    assert_eq!(
        changed_contract_comparison
            .entries
            .iter()
            .filter(|entry| entry.change
                == StudioShellHostessStagingAcceptanceComparisonChange::Changed)
            .count(),
        3
    );
    assert!(changed_contract_comparison.entries.iter().all(|entry| {
        entry.change != StudioShellHostessStagingAcceptanceComparisonChange::Changed
            || entry.issue_code.as_deref()
                == Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
    }));
    assert!(changed_contract_comparison.checks.iter().any(|check| {
        check.check_id == "studio.check.shell_hostess_staging_acceptance_comparison.entry_contracts"
            && check.status == StudioValidationStatus::Fail
    }));

    let assert_single_entry_contract_drift =
        |candidate: StudioShellHostessStagingAcceptanceChecklistReport, expected_item_id: &str| {
            let comparison = compare_shell_hostess_staging_acceptance_against_manifest(
                &ready_acceptance,
                &staging_acceptance,
                &candidate,
            );
            assert_eq!(
                comparison.status,
                StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable
            );
            assert_ne!(
                comparison.status,
                StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged
            );
            assert_eq!(
                comparison.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
            );
            let changed_entries = comparison
                .entries
                .iter()
                .filter(|entry| {
                    entry.change == StudioShellHostessStagingAcceptanceComparisonChange::Changed
                })
                .collect::<Vec<_>>();
            assert_eq!(changed_entries.len(), 1);
            assert_eq!(changed_entries[0].item_id, expected_item_id);
            assert_eq!(
                changed_entries[0].issue_code.as_deref(),
                Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
            );
            assert!(comparison.checks.iter().any(|check| {
                check.check_id
                    == "studio.check.shell_hostess_staging_acceptance_comparison.entry_contracts"
                    && check.status == StudioValidationStatus::Fail
                    && check.issue_code.as_deref()
                        == Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
            }));
        };

    let mut owner_drift_candidate = staging_acceptance.clone();
    owner_drift_candidate
        .entries
        .iter_mut()
        .find(|entry| entry.item_id == "hostess.accept_staging_handoff")
        .expect("acceptance row")
        .owner = "rusty.studio".to_string();
    assert_single_entry_contract_drift(owner_drift_candidate, "hostess.accept_staging_handoff");

    let mut route_drift_candidate = staging_acceptance.clone();
    route_drift_candidate
        .entries
        .iter_mut()
        .find(|entry| entry.item_id == "hostess.copy_staging_files")
        .expect("copy row")
        .route_kind = "hostess.stage.files_from_drifted_plan".to_string();
    assert_single_entry_contract_drift(route_drift_candidate, "hostess.copy_staging_files");

    let mut prohibited_drift_candidate = staging_acceptance.clone();
    prohibited_drift_candidate
        .entries
        .iter_mut()
        .find(|entry| entry.item_id == "hostess.review_staging_file_requests")
        .expect("review row")
        .prohibited_in_studio = false;
    assert_single_entry_contract_drift(
        prohibited_drift_candidate,
        "hostess.review_staging_file_requests",
    );

    let mut expected_input_drift_candidate = staging_acceptance.clone();
    expected_input_drift_candidate
        .entries
        .iter_mut()
        .find(|entry| entry.item_id == "hostess.review_staging_file_requests")
        .expect("review row")
        .expected_input_path = Some("target/drifted-input.json".to_string());
    assert_single_entry_contract_drift(
        expected_input_drift_candidate,
        "hostess.review_staging_file_requests",
    );

    let ready_index_entry = select_shell_hostess_staging_acceptance_index_entry(&index, None)
        .expect("select ready Hostess staging acceptance");
    let index_path = root.join("shell-hostess-staging-acceptances.json");
    let index_comparison = compare_shell_hostess_staging_acceptance_against_index_entry(
        &index,
        Some(&index_path),
        ready_index_entry,
        Some(&ready_manifest_path),
        &ready_acceptance,
        &staging_acceptance,
        &staging_acceptance,
    );
    assert_eq!(
        index_comparison.status,
        StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged
    );
    assert_eq!(
        index_comparison
            .baseline_index_selected_acceptance_id
            .as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
    );
    assert_eq!(
        index_comparison
            .baseline_index_default_acceptance_id
            .as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
    );

    let execution_request = shell_hostess_staging_execution_request_for_acceptance_index_entry(
        &index,
        Some(&index_path),
        ready_index_entry,
        Some(&ready_manifest_path),
        &ready_acceptance,
        &staging_acceptance,
        Some(&handoff_path),
        &envelope,
    );
    assert_eq!(
        execution_request.schema_id,
        SHELL_HOSTESS_STAGING_EXECUTION_REQUEST_SCHEMA
    );
    assert_eq!(
        execution_request.status,
        StudioShellHostessStagingExecutionRequestStatus::Ready
    );
    assert_eq!(execution_request.issue_code, None);
    assert_eq!(
        execution_request.execution_policy,
        "not_executed.hostess_request_only"
    );
    assert_eq!(execution_request.adapter_owner, "rusty.hostess");
    assert_eq!(execution_request.requester_role, "rusty.studio");
    assert_eq!(
        execution_request.command_session_authority.as_deref(),
        Some("rusty.manifold")
    );
    assert_eq!(
        execution_request
            .install_launch_evidence_authority
            .as_deref(),
        Some("rusty.hostess")
    );
    assert_eq!(
        execution_request.selected_acceptance_id,
        "studio.hostess_staging_acceptance.studio.project.test.rev1.ready"
    );
    assert_eq!(
        execution_request.acceptance_manifest_path.as_deref(),
        Some(ready_manifest_path.display().to_string().as_str())
    );
    assert_eq!(
        execution_request.handoff_path.as_deref(),
        Some(handoff_path.display().to_string().as_str())
    );
    assert_eq!(execution_request.adapter_action_count, 6);
    assert_eq!(execution_request.ready_adapter_action_count, 6);
    assert_eq!(execution_request.blocked_adapter_action_count, 0);
    assert!(!execution_request.pmb_shell_handoff_review_required);
    assert_eq!(execution_request.pmb_shell_handoff_review_path, None);
    assert!(!execution_request.pmb_shell_handoff_review_ready);
    assert!(execution_request
        .hostess_operator_start_preflight_cli_args
        .is_empty());
    assert!(execution_request.actions.iter().all(|action| {
        action.status == StudioShellHostessStagingExecutionActionStatus::Ready
            && action.ack_required
            && !action.execution_in_studio
    }));
    assert!(execution_request.actions.iter().any(|action| {
        action.source_item_id == "hostess.copy_staging_files"
            && action.owner == "rusty.hostess"
            && action.route_kind == "hostess.stage.files_from_plan"
    }));
    assert!(execution_request.actions.iter().any(|action| {
        action.source_item_id == "manifold.review_command_session_contract"
            && action.owner == "rusty.manifold"
            && action.route_kind == "manifold.review.command_session_contract"
    }));
    assert_eq!(
        execution_request.ack_template.schema_id,
        SHELL_HOSTESS_STAGING_EXECUTION_ACK_SCHEMA
    );
    assert_eq!(
        execution_request.ack_template.ack_status,
        StudioShellHostessStagingExecutionAckStatus::Pending
    );
    assert!(!execution_request.ack_template.execution_in_studio);
    assert_eq!(
        execution_request.ack_template.required_action_ids.len(),
        execution_request.adapter_action_count
    );
    assert_eq!(
        execution_request.reject_template.schema_id,
        SHELL_HOSTESS_STAGING_EXECUTION_REJECT_SCHEMA
    );
    assert_eq!(
        execution_request.reject_template.reject_status,
        StudioShellHostessStagingExecutionRejectStatus::Pending
    );
    assert!(!execution_request.reject_template.execution_in_studio);

    let pmb_review_path = root.join("target/pmb-shell-handoff.studio-review.json");
    let pmb_review = projected_motion_breath_shell_handoff_review_for_evidence(
        &super::projected_motion_breath_tests::projected_motion_shell_handoff_evidence(),
        Some(&pmb_review_path),
    );
    let gated_execution_request =
        shell_hostess_staging_execution_request_for_acceptance_index_entry_with_pmb_review(
            &index,
            Some(&index_path),
            ready_index_entry,
            Some(&ready_manifest_path),
            &ready_acceptance,
            &staging_acceptance,
            Some(&handoff_path),
            &envelope,
            Some(&pmb_review_path),
            Some(&pmb_review),
            true,
        );
    assert_eq!(
        gated_execution_request.status,
        StudioShellHostessStagingExecutionRequestStatus::Ready
    );
    assert!(gated_execution_request.pmb_shell_handoff_review_required);
    assert!(gated_execution_request.pmb_shell_handoff_review_ready);
    assert_eq!(
        gated_execution_request
            .pmb_shell_handoff_review_path
            .as_deref(),
        Some(pmb_review_path.display().to_string().as_str())
    );
    assert_eq!(
        gated_execution_request
            .source_pmb_shell_handoff_review_schema
            .as_deref(),
        Some(PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA)
    );
    assert_eq!(
        gated_execution_request.source_pmb_shell_handoff_review_status,
        Some(StudioProjectedMotionBreathShellHandoffReviewStatus::Ready)
    );
    assert_eq!(
        gated_execution_request
            .source_pmb_shell_handoff_id
            .as_deref(),
        Some("shell_handoff.projected_motion_breath.loopback")
    );
    assert_eq!(
        gated_execution_request.hostess_operator_start_preflight_cli_args,
        vec![
            "--pmb-shell-handoff-review-in".to_string(),
            pmb_review_path.display().to_string(),
            "--require-pmb-shell-handoff-review".to_string(),
        ]
    );
    assert!(gated_execution_request.checks.iter().any(|check| {
        check.check_id
            == "studio.check.shell_hostess_staging_execution_request.pmb_shell_handoff_review"
            && check.status == StudioValidationStatus::Pass
    }));

    let missing_pmb_execution_request =
        shell_hostess_staging_execution_request_for_acceptance_index_entry_with_pmb_review(
            &index,
            Some(&index_path),
            ready_index_entry,
            Some(&ready_manifest_path),
            &ready_acceptance,
            &staging_acceptance,
            Some(&handoff_path),
            &envelope,
            None,
            None,
            true,
        );
    assert_eq!(
        missing_pmb_execution_request.status,
        StudioShellHostessStagingExecutionRequestStatus::Blocked
    );
    assert_eq!(
        missing_pmb_execution_request.issue_code.as_deref(),
        Some("studio.issue.projected_motion_breath_shell_handoff_review_missing")
    );
    assert!(missing_pmb_execution_request.pmb_shell_handoff_review_required);
    assert!(!missing_pmb_execution_request.pmb_shell_handoff_review_ready);
    assert_eq!(
        missing_pmb_execution_request.hostess_operator_start_preflight_cli_args,
        vec!["--require-pmb-shell-handoff-review".to_string()]
    );
    assert_eq!(missing_pmb_execution_request.ready_adapter_action_count, 0);
    assert!(missing_pmb_execution_request.checks.iter().any(|check| {
        check.check_id
            == "studio.check.shell_hostess_staging_execution_request.pmb_shell_handoff_review"
            && check.status == StudioValidationStatus::Fail
    }));

    let changed_execution_request =
        shell_hostess_staging_execution_request_for_acceptance_index_entry(
            &index,
            Some(&index_path),
            ready_index_entry,
            Some(&ready_manifest_path),
            &ready_acceptance,
            &changed_contract_candidate,
            Some(&handoff_path),
            &envelope,
        );
    assert_eq!(
        changed_execution_request.status,
        StudioShellHostessStagingExecutionRequestStatus::Blocked
    );
    assert_eq!(
        changed_execution_request.issue_code.as_deref(),
        Some("studio.issue.shell_hostess_staging_acceptance_entry_drift")
    );
    assert_eq!(changed_execution_request.ready_adapter_action_count, 0);
    assert_eq!(changed_execution_request.blocked_adapter_action_count, 6);
    assert!(changed_execution_request.checks.iter().any(|check| {
        check.check_id
            == "studio.check.shell_hostess_staging_execution_request.acceptance_entry_contracts"
            && check.status == StudioValidationStatus::Fail
    }));
    assert!(changed_execution_request
        .actions
        .iter()
        .all(|action| !action.execution_in_studio));

    let mut regressed_candidate = staging_acceptance.clone();
    regressed_candidate.status = StudioShellHostessStagingAcceptanceStatus::Blocked;
    regressed_candidate.issue_code =
        Some("studio.issue.shell_hostess_staging_acceptance_blocked".to_string());
    regressed_candidate.ready_item_count = 0;
    regressed_candidate.blocked_item_count = regressed_candidate.entries.len();
    for entry in &mut regressed_candidate.entries {
        entry.status = StudioShellHostessStagingAcceptanceStatus::Blocked;
        entry.issue_code =
            Some("studio.issue.shell_hostess_staging_acceptance_blocked".to_string());
    }
    let regressed_comparison = compare_shell_hostess_staging_acceptance_against_manifest(
        &ready_acceptance,
        &staging_acceptance,
        &regressed_candidate,
    );
    assert_eq!(
        regressed_comparison.status,
        StudioShellHostessStagingAcceptanceComparisonStatus::Regressed
    );
    assert_eq!(
        regressed_comparison.issue_code.as_deref(),
        Some("studio.issue.shell_hostess_staging_acceptance_blocked")
    );
    assert_eq!(regressed_comparison.ready_item_delta, -6);
    assert_eq!(regressed_comparison.blocked_item_delta, 6);
    assert_eq!(
        regressed_comparison
            .entries
            .iter()
            .filter(|entry| entry.change
                == StudioShellHostessStagingAcceptanceComparisonChange::Regressed)
            .count(),
        6
    );

    let mut stale_identity = ready_acceptance.clone();
    stale_identity.project_id = Some("studio.project.stale".to_string());
    let stale_identity_comparison = compare_shell_hostess_staging_acceptance_against_manifest(
        &stale_identity,
        &staging_acceptance,
        &staging_acceptance,
    );
    assert_eq!(
        stale_identity_comparison.status,
        StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable
    );
    assert_eq!(
        stale_identity_comparison.issue_code.as_deref(),
        Some("studio.issue.shell_hostess_staging_acceptance_identity_mismatch")
    );

    let mut stale_index = index.clone();
    stale_index.entries[0].ready_item_count = 5;
    let stale_index_entry = select_shell_hostess_staging_acceptance_index_entry(&stale_index, None)
        .expect("select stale Hostess staging acceptance");
    let stale_index_comparison = compare_shell_hostess_staging_acceptance_against_index_entry(
        &stale_index,
        Some(&index_path),
        stale_index_entry,
        Some(&ready_manifest_path),
        &ready_acceptance,
        &staging_acceptance,
        &staging_acceptance,
    );
    assert_eq!(
        stale_index_comparison.status,
        StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable
    );
    assert_eq!(
        stale_index_comparison.issue_code.as_deref(),
        Some("studio.issue.shell_hostess_staging_acceptance_index_mismatch")
    );
    assert!(stale_index_comparison.checks.iter().any(|check| {
        check.check_id
            == "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_status_counts"
            && check.status == StudioValidationStatus::Fail
    }));

    let appended = append_shell_hostess_staging_acceptance_index_manifests(
        &shell_hostess_staging_acceptance_index_for_manifests(
            vec![(ready_acceptance, Some(ready_manifest_path.clone()))],
            None,
        ),
        vec![(
            blocked_acceptance_manifest,
            Some(blocked_manifest_path.clone()),
        )],
        Some("synthetic-blocked-hostess-acceptance"),
    );
    assert_eq!(
        appended.default_acceptance_id.as_deref(),
        Some("synthetic-blocked-hostess-acceptance")
    );
    let promoted = promote_shell_hostess_staging_acceptance_index_default(
        &appended,
        "studio.hostess_staging_acceptance.studio.project.test.rev1.ready",
    )
    .expect("promote ready Hostess staging acceptance");
    assert_eq!(
        promoted.default_acceptance_id.as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.test.rev1.ready")
    );
    assert!(promote_shell_hostess_staging_acceptance_index_default(&appended, "missing").is_none());
}
