use super::super::*;

pub(super) struct StagingArtifacts {
    pub(super) handoff: StudioShellHostessStagingHandoffEnvelope,
    pub(super) handoff_path: PathBuf,
}

pub(super) fn assert_staging_routes(project_path: &Path, intake_path: &Path) -> StagingArtifacts {
    let (staging, staging_path) = shell_hostess_staging_preview_for_project_source(&project_path)
        .expect("preview shell Hostess staging");
    assert!(staging_path.is_file());
    assert_eq!(
        staging.schema_id,
        "rusty.studio.shell_hostess_staging_preview_manifest.v1"
    );
    assert_eq!(
        staging.status,
        StudioShellHostessStagingPreviewStatus::Ready
    );
    assert_eq!(staging.issue_code, None);
    assert_eq!(staging.execution_policy, "not_executed.preview_only");
    assert_eq!(staging.staging_owner, "rusty.hostess");
    assert_eq!(
        staging.intake_path.as_deref(),
        Some(intake_path.display().to_string().as_str())
    );
    assert_eq!(staging.assignment_count, 4);
    assert_eq!(staging.ready_assignment_count, 4);
    assert_eq!(staging.blocked_assignment_count, 0);
    assert_eq!(staging.ready_group_count, 4);
    assert_eq!(staging.blocked_group_count, 0);
    assert!(staging.expected_artifact_count >= 10);
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
        .any(|artifact| artifact.artifact_kind == "shell_template_manifest"));
    assert!(staging
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let staging_status = shell_hostess_staging_preview_status(&staging, &staging_path);
    assert!(staging_status.contains("shell Hostess staging preview ready"));
    assert!(staging_status.contains("route hostess.stage.generated_shells"));
    assert!(staging_status.contains("not_executed.preview_only"));
    assert!(staging_status.contains("groups ready 4; blocked 0"));

    let (file_plan, file_plan_path) =
        shell_hostess_staging_file_plan_for_project_source(&project_path)
            .expect("plan shell Hostess staging files");
    assert!(file_plan_path.is_file());
    assert_eq!(
        file_plan.schema_id,
        "rusty.studio.shell_hostess_staging_file_plan.v1"
    );
    assert_eq!(
        file_plan.status,
        StudioShellHostessStagingFilePlanStatus::Ready
    );
    assert_eq!(file_plan.issue_code, None);
    assert_eq!(file_plan.execution_policy, "not_executed.dry_run_only");
    assert_eq!(file_plan.staging_owner, "rusty.hostess");
    assert_eq!(
        file_plan.preview_path.as_deref(),
        Some(staging_path.display().to_string().as_str())
    );
    assert_eq!(file_plan.ready_preview_group_count, 4);
    assert_eq!(file_plan.blocked_preview_group_count, 0);
    assert_eq!(file_plan.planned_file_count, 9);
    assert!(file_plan.duplicate_artifact_count > 0);
    assert_eq!(file_plan.request_count, 2);
    assert_eq!(file_plan.ready_request_count, 2);
    assert_eq!(file_plan.blocked_request_count, 0);
    assert_eq!(file_plan.target_request_count, 1);
    assert_eq!(file_plan.shared_request_count, 1);
    assert!(file_plan.requests.iter().any(|request| {
        request.target_key == "shared"
            && request.status == StudioShellHostessStagingFileRequestStatus::Ready
            && request
                .planned_files
                .iter()
                .any(|file| file.artifact_kind == "hostess_owner_intake")
    }));
    assert!(file_plan.requests.iter().any(|request| {
        request.target_kind == Some(StudioShellTargetKind::Desktop)
            && request.status == StudioShellHostessStagingFileRequestStatus::Ready
            && request.planned_files.iter().any(|file| {
                file.artifact_kind == "shell_template_manifest"
                    && file
                        .destination_path
                        .contains("hostess-staging/targets/desktop")
            })
    }));
    assert!(file_plan
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let file_plan_status = shell_hostess_staging_file_plan_status(&file_plan, &file_plan_path);
    assert!(file_plan_status.contains("shell Hostess staging file plan ready"));
    assert!(file_plan_status.contains("not_executed.dry_run_only"));
    assert!(file_plan_status.contains("planned files 9"));
    assert!(file_plan_status.contains("target 1; shared 1"));

    let (handoff, handoff_path) = shell_hostess_staging_handoff_for_project_source(&project_path)
        .expect("prepare shell Hostess staging handoff");
    assert!(handoff_path.is_file());
    assert_eq!(
        handoff.schema_id,
        "rusty.studio.shell_hostess_staging_handoff_envelope.v1"
    );
    assert_eq!(
        handoff.status,
        StudioShellHostessStagingHandoffEnvelopeStatus::Ready
    );
    assert_eq!(handoff.issue_code, None);
    assert_eq!(handoff.execution_policy, "not_executed.handoff_only");
    assert_eq!(handoff.handoff_owner, "rusty.hostess");
    assert_eq!(handoff.staging_owner, "rusty.hostess");
    assert_eq!(
        handoff.file_plan_path.as_deref(),
        Some(file_plan_path.display().to_string().as_str())
    );
    assert_eq!(handoff.planned_file_count, file_plan.planned_file_count);
    assert_eq!(handoff.request_count, 2);
    assert_eq!(handoff.ready_request_count, 2);
    assert_eq!(handoff.blocked_request_count, 0);
    assert_eq!(handoff.target_request_count, 1);
    assert_eq!(handoff.shared_request_count, 1);
    assert_eq!(handoff.instruction_count, 4);
    assert_eq!(handoff.ready_instruction_count, 4);
    assert_eq!(handoff.blocked_instruction_count, 0);
    assert_eq!(handoff.provenance.plan_checksum.len(), 16);
    assert!(handoff
        .provenance
        .source_artifact_kinds
        .contains(&"shell_template_manifest".to_string()));
    assert!(handoff.owner_instructions.iter().any(|instruction| {
        instruction.instruction_id == "hostess.copy_staging_files"
            && instruction.owner == "rusty.hostess"
            && instruction.route_kind == "hostess.stage.files_from_plan"
            && instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Ready
            && instruction.prohibited_in_studio
    }));
    assert!(handoff.owner_instructions.iter().any(|instruction| {
        instruction.instruction_id == "manifold.review_command_session_contract"
            && instruction.owner == "rusty.manifold"
            && instruction.route_kind == "manifold.review.command_session_contract"
    }));
    assert!(handoff
        .checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let handoff_status = shell_hostess_staging_handoff_status(&handoff, &handoff_path);
    assert!(handoff_status.contains("shell Hostess staging handoff ready"));
    assert!(handoff_status.contains("not_executed.handoff_only"));
    assert!(handoff_status.contains("instructions ready 4; blocked 0"));
    assert!(handoff_status.contains("hostess.stage.files_from_plan"));

    StagingArtifacts {
        handoff,
        handoff_path,
    }
}
