use super::*;

#[test]
fn shell_release_candidate_review_reports_ready_from_makepad_route() {
    let root = temp_root("shell-release-candidate-review");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");
    write_shell_handoff_manifest_for_project_source(&project_path)
        .expect("write shell handoff manifest");
    write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
        .expect("write acceptance baseline");
    write_shell_export_package_baseline_for_project_source(&project_path)
        .expect("write export package baseline");

    let (review, output_path) = shell_release_candidate_review_for_project_source(&project_path)
        .expect("review shell release candidate");

    assert!(output_path.is_file());
    assert_eq!(
        review.schema_id,
        "rusty.studio.shell_release_candidate_review.v1"
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
    assert_eq!(review.handoff_ready_count, 1);
    assert_eq!(review.handoff_failed_count, 0);
    assert_eq!(review.handoff_missing_bundle_count, 0);
    assert_eq!(
        review.acceptance_baseline_selection.status,
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
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
            .export_package_comparison
            .as_ref()
            .map(|comparison| comparison.status),
        Some(StudioShellExportPackageComparisonStatus::Unchanged)
    );
    assert!(review
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let status = shell_release_candidate_review_status(&review, &output_path);
    assert!(status.contains("shell release candidate review ready"));
    assert!(status.contains("acceptance baseline: selected"));
    assert!(status.contains("comparison unchanged"));
    assert!(status.contains("export package baseline: selected"));
    assert!(status.contains("checks:"));
    assert!(status.contains("failed 0"));
    assert!(status.contains("not_executed.review_only"));
}

#[test]
fn shell_release_candidate_review_index_cycles_default_from_makepad_route() {
    let root = temp_root("shell-release-candidate-index");
    write_reference_fixture_tree(&root);
    let project_path = root.join("project.json");
    save_project(&project_path, &editable_project()).expect("save editable project");
    let model = load_studio_view_model_for_path(&project_path, None, None, None, None)
        .expect("load view model");
    export_shell_bundle_for_project_source(&project_path, &model, 0)
        .expect("export selected shell bundle");
    write_shell_handoff_manifest_for_project_source(&project_path)
        .expect("write shell handoff manifest");
    write_shell_handoff_acceptance_baseline_for_project_source(&project_path)
        .expect("write acceptance baseline");
    write_shell_export_package_baseline_for_project_source(&project_path)
        .expect("write export package baseline");

    let (review, candidate, index, review_path, candidate_path, index_path) =
        write_shell_release_candidate_review_manifest_for_project_source(&project_path)
            .expect("write release candidate manifest");

    assert!(review_path.is_file());
    assert!(candidate_path.is_file());
    assert!(index_path.is_file());
    assert_eq!(
        candidate.schema_id,
        "rusty.studio.shell_release_candidate_review_manifest.v1"
    );
    assert_eq!(
        candidate.candidate_id,
        "studio.project.makepad_edit.rev1.ready"
    );
    assert_eq!(
        candidate.status,
        StudioShellReleaseCandidateReviewStatus::Ready
    );
    assert_eq!(candidate.review_path, review_path.display().to_string());
    assert_eq!(
        index.schema_id,
        "rusty.studio.shell_release_candidate_review_index.v1"
    );
    assert_eq!(
        index.default_candidate_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );
    assert_eq!(index.candidate_count, 1);
    assert_eq!(index.ready_candidate_count, 1);
    assert_eq!(index.blocked_candidate_count, 0);
    let status = shell_release_candidate_review_manifest_status(
        &review,
        &candidate,
        &index,
        &review_path,
        &candidate_path,
        &index_path,
    );
    assert!(status.contains("release candidate written"));
    assert!(status.contains("release candidate selection selected"));
    assert!(status.contains("release candidate index slots 1"));

    let (_, archived_candidate, archived_index, _, archived_candidate_path, _) =
        append_shell_release_candidate_review_manifest_for_project_source(&project_path)
            .expect("archive release candidate");

    assert_eq!(
        archived_candidate.candidate_id,
        "studio.project.makepad_edit.rev1.ready.archive2"
    );
    assert_eq!(
        archived_index.default_candidate_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready.archive2")
    );
    assert_eq!(archived_index.candidate_count, 2);
    assert_eq!(archived_index.ready_candidate_count, 2);
    assert_eq!(
        archived_index.entries[1].candidate_manifest_path.as_deref(),
        Some(archived_candidate_path.display().to_string().as_str())
    );
    let loaded_index =
        load_shell_release_candidate_review_index(&index_path).expect("load candidate index");
    assert_eq!(loaded_index, archived_index);

    let (selected_ready, selected_index, selected_candidate_path, loaded_index_path) =
        select_next_shell_release_candidate_default_for_project_source(&project_path)
            .expect("select next release candidate");
    assert_eq!(selected_ready, candidate);
    assert_eq!(selected_candidate_path, candidate_path);
    assert_eq!(
        selected_index.default_candidate_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );
    let status = shell_release_candidate_review_manifest_select_status(
        &selected_ready,
        &selected_index,
        &selected_candidate_path,
        &loaded_index_path,
    );
    assert!(status.contains("release candidate default selected"));
    assert!(status.contains(
            "release candidate selection selected; requested studio.project.makepad_edit.rev1.ready; default studio.project.makepad_edit.rev1.ready; selected studio.project.makepad_edit.rev1.ready"
        ));

    let (promoted_candidate, promoted_index, promoted_candidate_path, loaded_index_path) =
        promote_shell_release_candidate_default_for_project_source(&project_path)
            .expect("promote saved release candidate");
    assert_eq!(promoted_candidate, candidate);
    assert_eq!(promoted_candidate_path, candidate_path);
    assert_eq!(
        promoted_index.default_candidate_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );
    let status = shell_release_candidate_review_manifest_promote_status(
        &promoted_candidate,
        &promoted_index,
        &promoted_candidate_path,
        &loaded_index_path,
    );
    assert!(status.contains("release candidate default promoted"));
    assert!(status.contains("release candidate index slots 2"));
}
