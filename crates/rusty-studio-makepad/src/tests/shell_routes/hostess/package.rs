use super::super::*;

pub(super) struct OwnerIntakeArtifacts {
    pub(super) intake_path: PathBuf,
}

pub(super) fn prepare_hostess_project(root: &Path) -> PathBuf {
    write_reference_fixture_tree(root);
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
    write_shell_release_candidate_review_manifest_for_project_source(&project_path)
        .expect("write release candidate manifest");

    project_path
}

pub(super) fn assert_handoff_package_and_owner_intake(project_path: &Path) -> OwnerIntakeArtifacts {
    let (package, output_path) = shell_hostess_handoff_package_for_project_source(&project_path)
        .expect("review shell Hostess handoff package");

    assert!(output_path.is_file());
    assert_eq!(
        package.schema_id,
        "rusty.studio.shell_hostess_handoff_package.v1"
    );
    assert_eq!(
        package.status,
        StudioShellHostessHandoffPackageStatus::Ready
    );
    assert_eq!(package.issue_code, None);
    assert_eq!(
        package.selected_candidate_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
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
    assert_eq!(package.handoff_ready_count, 1);
    assert_eq!(package.handoff_failed_count, 0);
    assert_eq!(package.handoff_missing_bundle_count, 0);
    assert_eq!(
        package.acceptance_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );
    assert_eq!(
        package.acceptance_comparison_status,
        Some(StudioShellHandoffAcceptanceComparisonStatus::Unchanged)
    );
    assert_eq!(
        package.export_package_baseline_id.as_deref(),
        Some("studio.project.makepad_edit.rev1.ready")
    );
    assert_eq!(
        package.export_package_comparison_status,
        Some(StudioShellExportPackageComparisonStatus::Unchanged)
    );
    assert!(package.required_owner_actions.iter().any(|action| {
        action.action_id == "hostess.collect_install_launch_evidence"
            && action.owner == "rusty.hostess"
            && action.status == StudioShellHostessHandoffPackageActionStatus::Ready
            && action.prohibited_in_studio
    }));
    assert!(package
        .prohibited_actions
        .contains(&"stage_generated_shells".to_string()));
    assert!(package
        .prohibited_actions
        .contains(&"collect_install_launch_evidence".to_string()));
    assert!(package
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let status = shell_hostess_handoff_package_status(&package, &output_path);
    assert!(status.contains("shell Hostess handoff package ready"));
    assert!(status.contains("selected studio.project.makepad_edit.rev1.ready"));
    assert!(status.contains("hostess.collect_install_launch_evidence [ready]"));
    assert!(status.contains("manifold.review_command_session_contract [ready]"));
    assert!(status.contains("prohibited:"));
    assert!(status.contains("stage_generated_shells"));
    assert!(status.contains("checks:"));
    assert!(status.contains("failed 0"));

    let (intake, intake_path) = shell_hostess_owner_intake_for_project_source(&project_path)
        .expect("review shell Hostess owner intake");
    assert!(intake_path.is_file());
    assert_eq!(
        intake.schema_id,
        "rusty.studio.shell_hostess_owner_intake.v1"
    );
    assert_eq!(intake.status, StudioShellHostessOwnerIntakeStatus::Ready);
    assert_eq!(intake.issue_code, None);
    assert_eq!(intake.execution_policy, "not_executed.request_only");
    assert_eq!(intake.intake_owner, "rusty.hostess");
    assert_eq!(
        intake.package_path.as_deref(),
        Some(output_path.display().to_string().as_str())
    );
    assert_eq!(intake.ready_assignment_count, 4);
    assert_eq!(intake.blocked_assignment_count, 0);
    assert_eq!(intake.hostess_ready_action_count, 3);
    assert_eq!(intake.manifold_ready_action_count, 1);
    assert!(intake.assignments.iter().any(|assignment| {
        assignment.action_id == "hostess.stage_generated_shells"
            && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
            && assignment.request_kind == "hostess_owner_action_request"
            && assignment.prohibited_in_studio
    }));
    assert!(intake.assignments.iter().any(|assignment| {
        assignment.action_id == "manifold.review_command_session_contract"
            && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
            && assignment.request_kind == "manifold_owner_review_request"
    }));
    assert!(intake
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let intake_status = shell_hostess_owner_intake_status(&intake, &intake_path);
    assert!(intake_status.contains("shell Hostess owner intake ready"));
    assert!(intake_status.contains("assignments ready 4; blocked 0"));
    assert!(intake_status.contains("Hostess ready 3; Manifold ready 1"));
    assert!(intake_status.contains("not_executed.request_only"));
    assert!(intake_status.contains("hostess_owner_action_request"));
    assert!(intake_status.contains("manifold_owner_review_request"));

    OwnerIntakeArtifacts { intake_path }
}
