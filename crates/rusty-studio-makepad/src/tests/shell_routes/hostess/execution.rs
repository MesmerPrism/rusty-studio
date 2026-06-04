use super::super::*;

pub(super) fn assert_execution_request_route(project_path: &Path) {
    let (execution_request, execution_request_path) =
        shell_hostess_staging_execution_request_for_project_source(&project_path)
            .expect("write shell Hostess staging execution request");
    assert!(execution_request_path.is_file());
    assert_eq!(
        execution_request.schema_id,
        "rusty.studio.shell_hostess_staging_execution_request.v1"
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
    assert_eq!(execution_request.adapter_action_count, 6);
    assert_eq!(execution_request.ready_adapter_action_count, 6);
    assert_eq!(execution_request.blocked_adapter_action_count, 0);
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
        "rusty.studio.shell_hostess_staging_execution_ack.v1"
    );
    assert!(!execution_request.ack_template.execution_in_studio);
    assert_eq!(
        execution_request.ack_template.required_action_ids.len(),
        execution_request.adapter_action_count
    );
    assert_eq!(
        execution_request.reject_template.schema_id,
        "rusty.studio.shell_hostess_staging_execution_reject.v1"
    );
    assert!(!execution_request.reject_template.execution_in_studio);
    let execution_request_status =
        shell_hostess_staging_execution_request_status(&execution_request, &execution_request_path);
    assert!(execution_request_status.contains("Hostess staging execution request ready"));
    assert!(execution_request_status.contains("not_executed.hostess_request_only"));
    assert!(execution_request_status.contains("Studio execution no"));
}
