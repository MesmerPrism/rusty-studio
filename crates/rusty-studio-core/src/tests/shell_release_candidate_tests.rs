use super::*;

#[test]
fn shell_release_candidate_review_reports_ready_from_indexes() {
    let root = temp_root("shell-release-candidate-ready");
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

    let review = shell_release_candidate_review_for_manifest(
        &manifest,
        Some(&manifest_path),
        &acceptance_index,
        Some(&acceptance_index_path),
        Some("synthetic-ready"),
        &export_package_index,
        Some(&export_package_index_path),
        Some("synthetic-ready-package"),
    );

    assert_eq!(review.schema_id, SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA);
    assert_eq!(review.source_manifest_schema, SHELL_HANDOFF_MANIFEST_SCHEMA);
    assert_eq!(
        review.manifest_path.as_deref(),
        Some(manifest_path.display().to_string().as_str())
    );
    assert_eq!(
        review.status,
        StudioShellReleaseCandidateReviewStatus::Ready
    );
    assert_eq!(review.issue_code, None);
    assert_eq!(review.execution_policy, "not_executed.review_only");
    assert_eq!(review.review_owner, "rusty.hostess");
    assert_eq!(review.command_session_authority, "rusty.manifold");
    assert_eq!(review.install_launch_evidence_authority, "rusty.hostess");
    assert_eq!(review.studio_role, "authoring.export_planning");
    assert_eq!(review.handoff_status, StudioValidationStatus::Pass);
    assert_eq!(review.handoff_ready_count, 3);
    assert_eq!(review.handoff_failed_count, 0);
    assert_eq!(review.handoff_missing_bundle_count, 0);
    assert_eq!(
        review.acceptance_baseline_selection.status,
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
    );
    assert_eq!(
        review
            .acceptance_baseline_selection
            .selected_baseline_id
            .as_deref(),
        Some("synthetic-ready")
    );
    assert_eq!(
        review
            .acceptance_comparison
            .as_ref()
            .map(|comparison| comparison.status),
        Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
    );
    assert_eq!(
        review.export_package_baseline_selection.status,
        StudioShellExportPackageBaselineSelectionStatus::Selected
    );
    assert_eq!(
        review
            .export_package_baseline_selection
            .selected_baseline_id
            .as_deref(),
        Some("synthetic-ready-package")
    );
    assert_eq!(
        review
            .export_package_comparison
            .as_ref()
            .map(|comparison| comparison.status),
        Some(StudioShellExportPackageComparisonStatus::Unchanged)
    );
    assert!(review
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));
    assert!(review.prohibited_actions.contains(&"install".to_string()));
    assert!(review
        .prohibited_actions
        .contains(&"open_command_session".to_string()));
}

#[test]
fn shell_release_candidate_review_blocks_regressed_export_package() {
    let root = temp_root("shell-release-candidate-regressed-package");
    write_reference_fixture_tree(&root);
    let project = valid_multi_shell_project_with_relative_references();
    let bundle_root = root.join("selected-shells");
    save_selected_shell_bundles(&project, &root, &bundle_root);
    let manifest = shell_handoff_manifest_for_project(&project, Some(&root), &bundle_root);

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

    std::fs::remove_file(
        bundle_root
            .join("studio.graph.phone")
            .join("shells/phone/studio.graph.phone.shell-template.json"),
    )
    .expect("remove phone template manifest");

    let review = shell_release_candidate_review_for_manifest(
        &manifest,
        None,
        &acceptance_index,
        Some(&acceptance_index_path),
        Some("synthetic-ready"),
        &export_package_index,
        Some(&export_package_index_path),
        Some("synthetic-ready-package"),
    );

    assert_eq!(
        review.status,
        StudioShellReleaseCandidateReviewStatus::Blocked
    );
    assert_eq!(
        review.issue_code.as_deref(),
        Some("studio.issue.shell_export_package_template_load_failed")
    );
    assert_eq!(
        review
            .acceptance_comparison
            .as_ref()
            .map(|comparison| comparison.status),
        Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
    );
    let export_package_comparison = review
        .export_package_comparison
        .as_ref()
        .expect("export package comparison");
    assert_eq!(
        export_package_comparison.status,
        StudioShellExportPackageComparisonStatus::Regressed
    );
    assert_eq!(export_package_comparison.ready_delta, -1);
    assert_eq!(export_package_comparison.blocked_delta, 1);
    assert!(review.checks.iter().any(|check| {
        check.check_id
            == "studio.check.shell_release_candidate_review.export_package_comparison_not_regressed"
            && check.status == StudioValidationStatus::Fail
            && check.issue_code.as_deref()
                == Some("studio.issue.shell_export_package_template_load_failed")
    }));
}

#[test]
fn shell_release_candidate_review_index_lists_and_selects_candidates() {
    let root = temp_root("shell-release-candidate-index");
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
        None,
        None,
    );
    let ready_candidate_path = root.join("shell-release-candidate-review-manifest.json");
    save_json(&ready_candidate_path, &ready_candidate).expect("save ready candidate");

    assert_eq!(
        ready_candidate.schema_id,
        SHELL_RELEASE_CANDIDATE_REVIEW_MANIFEST_SCHEMA
    );
    assert_eq!(
        ready_candidate.candidate_id,
        "studio.project.test.rev1.ready"
    );
    assert_eq!(
        ready_candidate.review_path,
        ready_review_path.display().to_string()
    );
    assert_eq!(
        ready_candidate.status,
        StudioShellReleaseCandidateReviewStatus::Ready
    );
    assert_eq!(
        ready_candidate.acceptance_comparison_status,
        Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
    );
    assert_eq!(
        ready_candidate.export_package_comparison_status,
        Some(StudioShellExportPackageComparisonStatus::Unchanged)
    );
    assert_eq!(ready_candidate.failed_check_count, 0);

    let index = shell_release_candidate_review_index_for_manifests(
        vec![(ready_candidate.clone(), Some(ready_candidate_path.clone()))],
        None,
    );
    let index_path = root.join("shell-release-candidate-reviews.json");
    save_json(&index_path, &index).expect("save release candidate index");
    assert_eq!(index.schema_id, SHELL_RELEASE_CANDIDATE_REVIEW_INDEX_SCHEMA);
    assert_eq!(
        index.default_candidate_id.as_deref(),
        Some("studio.project.test.rev1.ready")
    );
    assert_eq!(index.candidate_count, 1);
    assert_eq!(index.ready_candidate_count, 1);
    assert_eq!(index.blocked_candidate_count, 0);
    assert_eq!(index.rejected_candidate_count, 0);
    assert_eq!(index.entries[0].candidate_id, ready_candidate.candidate_id);
    assert_eq!(
        index.entries[0].candidate_manifest_path.as_deref(),
        Some(ready_candidate_path.display().to_string().as_str())
    );

    let selection =
        summarize_shell_release_candidate_review_index_selection(&index, Some(&index_path), None);
    assert_eq!(
        selection.schema_id,
        SHELL_RELEASE_CANDIDATE_REVIEW_SELECTION_SCHEMA
    );
    assert_eq!(
        selection.status,
        StudioShellReleaseCandidateReviewSelectionStatus::Selected
    );
    assert_eq!(selection.issue_code, None);
    assert_eq!(
        selection.selected_candidate_id.as_deref(),
        Some("studio.project.test.rev1.ready")
    );
    assert_eq!(selection.ready_candidate_count, 1);
    assert_eq!(selection.entries.len(), 1);
    assert!(selection.entries[0].selected);
    assert!(selection.entries[0].default);

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
        Some("synthetic-blocked"),
        Some("Synthetic blocked release candidate"),
    );
    let blocked_candidate_path = root.join("shell-release-candidate-blocked-manifest.json");
    save_json(&blocked_candidate_path, &blocked_candidate).expect("save blocked candidate");

    let appended = append_shell_release_candidate_review_index_manifests(
        &index,
        vec![(
            blocked_candidate.clone(),
            Some(blocked_candidate_path.clone()),
        )],
        Some("synthetic-blocked"),
    );
    assert_eq!(appended.candidate_count, 2);
    assert_eq!(appended.ready_candidate_count, 1);
    assert_eq!(appended.blocked_candidate_count, 1);
    assert_eq!(
        appended.default_candidate_id.as_deref(),
        Some("synthetic-blocked")
    );
    assert_eq!(
        select_shell_release_candidate_review_index_entry(&appended, Some("synthetic-blocked"))
            .map(|entry| entry.status),
        Some(StudioShellReleaseCandidateReviewStatus::Blocked)
    );

    let promoted = promote_shell_release_candidate_review_index_default(
        &appended,
        "studio.project.test.rev1.ready",
    )
    .expect("promote ready candidate");
    assert_eq!(
        promoted.default_candidate_id.as_deref(),
        Some("studio.project.test.rev1.ready")
    );
    let missing =
        summarize_shell_release_candidate_review_index_selection(&promoted, None, Some("missing"));
    assert_eq!(
        missing.status,
        StudioShellReleaseCandidateReviewSelectionStatus::Missing
    );
    assert_eq!(
        missing.issue_code.as_deref(),
        Some("studio.issue.shell_release_candidate_review_not_found")
    );
    assert_eq!(missing.selected_candidate_id, None);
}
