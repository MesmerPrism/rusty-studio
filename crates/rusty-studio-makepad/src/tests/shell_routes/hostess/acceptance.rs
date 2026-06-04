use super::super::*;

pub(super) fn assert_staging_acceptance_routes(
    project_path: &Path,
    staging: &super::staging::StagingArtifacts,
) {
    let handoff = &staging.handoff;
    let handoff_path = &staging.handoff_path;
    let (acceptance, acceptance_path) =
        shell_hostess_staging_acceptance_for_project_source(&project_path)
            .expect("prepare shell Hostess staging acceptance checklist");
    assert!(acceptance_path.is_file());
    assert_eq!(
        acceptance.schema_id,
        "rusty.studio.shell_hostess_staging_acceptance_checklist.v1"
    );
    assert_eq!(
        acceptance.status,
        StudioShellHostessStagingAcceptanceStatus::Ready
    );
    assert_eq!(acceptance.issue_code, None);
    assert_eq!(
        acceptance.execution_policy,
        "not_executed.acceptance_check_only"
    );
    assert_eq!(acceptance.checklist_owner, "rusty.hostess");
    assert_eq!(acceptance.handoff_owner, "rusty.hostess");
    assert_eq!(acceptance.staging_owner, "rusty.hostess");
    assert_eq!(
        acceptance.handoff_path.as_deref(),
        Some(handoff_path.display().to_string().as_str())
    );
    assert_eq!(acceptance.plan_checksum, handoff.provenance.plan_checksum);
    assert_eq!(acceptance.request_count, handoff.request_count);
    assert_eq!(acceptance.ready_request_count, handoff.ready_request_count);
    assert_eq!(acceptance.blocked_request_count, 0);
    assert_eq!(acceptance.instruction_count, 4);
    assert_eq!(acceptance.ready_instruction_count, 4);
    assert_eq!(acceptance.blocked_instruction_count, 0);
    assert_eq!(acceptance.ready_item_count, 6);
    assert_eq!(acceptance.blocked_item_count, 0);
    assert_eq!(acceptance.rejected_item_count, 0);
    assert!(acceptance.entries.iter().any(|entry| {
        entry.item_id == "hostess.copy_staging_files"
            && entry.owner == "rusty.hostess"
            && entry.route_kind == "hostess.stage.files_from_plan"
            && entry.status == StudioShellHostessStagingAcceptanceStatus::Ready
            && entry.prohibited_in_studio
    }));
    assert!(acceptance.entries.iter().any(|entry| {
        entry.item_id == "manifold.review_command_session_contract"
            && entry.owner == "rusty.manifold"
            && entry.route_kind == "manifold.review.command_session_contract"
    }));
    assert!(acceptance
        .handoff_checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass));

    let acceptance_status = shell_hostess_staging_acceptance_status(&acceptance, &acceptance_path);
    assert!(acceptance_status.contains("shell Hostess staging acceptance ready"));
    assert!(acceptance_status.contains("not_executed.acceptance_check_only"));
    assert!(acceptance_status.contains("items ready 6; blocked 0; rejected 0"));
    assert!(acceptance_status.contains("hostess.stage.files_from_plan"));

    let (archived_acceptance, archived_index, archived_acceptance_path, index_path) =
        append_shell_hostess_staging_acceptance_for_project_source(&project_path)
            .expect("archive shell Hostess staging acceptance");
    assert!(archived_acceptance_path.is_file());
    assert!(index_path.is_file());
    assert_eq!(
        archived_acceptance.schema_id,
        "rusty.studio.shell_hostess_staging_acceptance_manifest.v1"
    );
    assert_eq!(
        archived_acceptance.acceptance_id,
        "studio.hostess_staging_acceptance.studio.project.makepad_edit.rev1.ready"
    );
    assert_eq!(
        archived_acceptance.label,
        "studio.project.makepad_edit revision 1 ready Hostess staging acceptance"
    );
    assert_eq!(
        archived_acceptance.checklist_schema,
        "rusty.studio.shell_hostess_staging_acceptance_checklist.v1"
    );
    assert_eq!(
        archived_acceptance.status,
        StudioShellHostessStagingAcceptanceStatus::Ready
    );
    assert_eq!(archived_acceptance.ready_item_count, 6);
    assert_eq!(archived_acceptance.blocked_item_count, 0);
    assert_eq!(
        archived_acceptance.command_session_authority.as_deref(),
        Some("rusty.manifold")
    );
    assert_eq!(
        archived_acceptance
            .install_launch_evidence_authority
            .as_deref(),
        Some("rusty.hostess")
    );
    assert_eq!(
        archived_index.schema_id,
        "rusty.studio.shell_hostess_staging_acceptance_index.v1"
    );
    assert_eq!(
        archived_index.default_acceptance_id.as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.makepad_edit.rev1.ready")
    );
    assert_eq!(archived_index.acceptance_count, 1);
    assert_eq!(archived_index.ready_acceptance_count, 1);
    assert_eq!(archived_index.blocked_acceptance_count, 0);
    assert_eq!(archived_index.entries.len(), 1);
    assert_eq!(
        archived_index.entries[0]
            .acceptance_manifest_path
            .as_deref(),
        Some(archived_acceptance_path.display().to_string().as_str())
    );
    let loaded_index =
        load_shell_hostess_staging_acceptance_index(&index_path).expect("load acceptance index");
    assert_eq!(loaded_index, archived_index);

    let append_status = shell_hostess_staging_acceptance_append_status(
        &archived_acceptance,
        &archived_index,
        &archived_acceptance_path,
        &index_path,
    );
    assert!(append_status.contains("Hostess staging acceptance archived"));
    assert!(append_status.contains("Hostess staging acceptance selection selected"));
    assert!(append_status.contains("Hostess staging acceptance index slots 1"));

    let (second_acceptance, second_index, second_acceptance_path, loaded_index_path) =
        append_shell_hostess_staging_acceptance_for_project_source(&project_path)
            .expect("archive second shell Hostess staging acceptance");
    assert_eq!(loaded_index_path, index_path);
    assert_eq!(
        second_acceptance.acceptance_id,
        "studio.hostess_staging_acceptance.studio.project.makepad_edit.rev1.ready.archive2"
    );
    assert!(second_acceptance_path.is_file());
    assert_eq!(
        second_index.default_acceptance_id.as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.makepad_edit.rev1.ready.archive2")
    );
    assert_eq!(second_index.acceptance_count, 2);
    assert_eq!(second_index.ready_acceptance_count, 2);

    let (selected_acceptance, selected_index, selected_acceptance_path, loaded_index_path) =
        select_next_shell_hostess_staging_acceptance_default_for_project_source(&project_path)
            .expect("select next shell Hostess staging acceptance");
    assert_eq!(selected_acceptance, archived_acceptance);
    assert_eq!(
        selected_index.default_acceptance_id.as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.makepad_edit.rev1.ready")
    );
    assert_eq!(
        selected_acceptance_path,
        shell_hostess_staging_acceptance_manifest_output_path(&project_path)
    );
    let select_status = shell_hostess_staging_acceptance_select_status(
        &selected_acceptance,
        &selected_index,
        &selected_acceptance_path,
        &loaded_index_path,
    );
    assert!(select_status.contains("Hostess staging acceptance default selected"));

    let (summary_acceptance, summary_index, _, _) =
        shell_hostess_staging_acceptance_summary_for_project_source(&project_path)
            .expect("summarize shell Hostess staging acceptance index");
    assert_eq!(summary_acceptance, archived_acceptance);
    assert_eq!(summary_index, selected_index);

    let (promoted_acceptance, promoted_index, promoted_acceptance_path, loaded_index_path) =
        promote_shell_hostess_staging_acceptance_default_for_project_source(&project_path)
            .expect("promote shell Hostess staging acceptance");
    assert_eq!(promoted_acceptance, archived_acceptance);
    assert_eq!(promoted_acceptance_path, selected_acceptance_path);
    assert_eq!(
        promoted_index.default_acceptance_id.as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.makepad_edit.rev1.ready")
    );
    let promote_status = shell_hostess_staging_acceptance_promote_status(
        &promoted_acceptance,
        &promoted_index,
        &promoted_acceptance_path,
        &loaded_index_path,
    );
    assert!(promote_status.contains("Hostess staging acceptance default promoted"));

    let (comparison, comparison_acceptance_path, comparison_output_path) =
        shell_hostess_staging_acceptance_comparison_for_project_source(&project_path)
            .expect("compare shell Hostess staging acceptance");
    assert!(comparison_output_path.is_file());
    assert_eq!(
        comparison.schema_id,
        "rusty.studio.shell_hostess_staging_acceptance_comparison.v1"
    );
    assert_eq!(
        comparison.status,
        StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged
    );
    assert_eq!(comparison.issue_code, None);
    assert_eq!(
        comparison.baseline_acceptance_id.as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.makepad_edit.rev1.ready")
    );
    assert_eq!(
        comparison.baseline_index_selected_acceptance_id.as_deref(),
        Some("studio.hostess_staging_acceptance.studio.project.makepad_edit.rev1.ready")
    );
    assert_eq!(comparison.ready_item_delta, 0);
    assert_eq!(comparison.blocked_item_delta, 0);
    assert_eq!(comparison.rejected_item_delta, 0);
    assert_eq!(comparison.entries.len(), 6);
    assert!(comparison.entries.iter().all(
        |entry| entry.change == StudioShellHostessStagingAcceptanceComparisonChange::Unchanged
    ));
    let comparison_status = shell_hostess_staging_acceptance_comparison_status(
        &comparison,
        &comparison_acceptance_path,
        &comparison_output_path,
    );
    assert!(comparison_status.contains("Hostess staging acceptance comparison unchanged"));
    assert!(comparison_status.contains("delta 0"));
}
