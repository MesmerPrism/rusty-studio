use super::*;

pub fn shell_hostess_staging_acceptance_checklist_for_handoff(
    handoff: &StudioShellHostessStagingHandoffEnvelope,
    handoff_path: Option<&Path>,
) -> StudioShellHostessStagingAcceptanceChecklistReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.source_handoff_schema",
        handoff.schema_id == SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA,
        "source Hostess staging handoff schema is supported",
        "source Hostess staging handoff schema is unsupported",
        "studio.issue.shell_hostess_staging_handoff_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.handoff_path",
        handoff_path.is_some(),
        "source Hostess staging handoff has a durable path",
        "source Hostess staging handoff path is missing",
        "studio.issue.shell_hostess_staging_acceptance_handoff_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.handoff_ready",
        handoff.status == StudioShellHostessStagingHandoffEnvelopeStatus::Ready,
        "source Hostess staging handoff is ready",
        "source Hostess staging handoff is not ready",
        handoff
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_staging_handoff_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.handoff_execution_policy",
        handoff.execution_policy == "not_executed.handoff_only",
        "source handoff is handoff-only and not executed",
        "source handoff execution policy is not handoff-only",
        "studio.issue.shell_hostess_staging_handoff_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.handoff_owner",
        handoff.handoff_owner == "rusty.hostess" && handoff.staging_owner == "rusty.hostess",
        "Hostess remains handoff and staging owner",
        "handoff and staging owners must remain rusty.hostess",
        "studio.issue.shell_hostess_staging_handoff_owner_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.runtime_command_authority",
        handoff.command_session_authority.as_deref() == Some("rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.runtime_host_authority",
        handoff.install_launch_evidence_authority.as_deref() == Some("rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.studio_role",
        handoff.studio_role.as_deref() == Some("authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.source_handoff_checks_pass",
        handoff
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "source Hostess staging handoff checks all pass",
        "source Hostess staging handoff contains failed checks",
        "studio.issue.shell_hostess_staging_handoff_failed_check",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.requests_ready",
        !handoff.request_summaries.is_empty()
            && handoff
                .request_summaries
                .iter()
                .all(|request| request.status == StudioShellHostessStagingFileRequestStatus::Ready),
        "all handoff request summaries are ready",
        "one or more handoff request summaries are blocked",
        "studio.issue.shell_hostess_staging_acceptance_request_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.request_counts",
        handoff.request_count == handoff.request_summaries.len()
            && handoff.ready_request_count == handoff.request_summaries.len()
            && handoff.blocked_request_count == 0,
        "handoff request counts match request summaries",
        "handoff request counts do not match request summaries",
        "studio.issue.shell_hostess_staging_acceptance_request_count_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.instructions_ready",
        !handoff.owner_instructions.is_empty()
            && handoff.owner_instructions.iter().all(|instruction| {
                instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Ready
            }),
        "all handoff owner instructions are ready",
        "one or more handoff owner instructions are blocked",
        "studio.issue.shell_hostess_staging_acceptance_instruction_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.instruction_counts",
        handoff.instruction_count == handoff.owner_instructions.len()
            && handoff.ready_instruction_count == handoff.owner_instructions.len()
            && handoff.blocked_instruction_count == 0,
        "handoff instruction counts match instruction rows",
        "handoff instruction counts do not match instruction rows",
        "studio.issue.shell_hostess_staging_acceptance_instruction_count_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.instructions_prohibited_in_studio",
        !handoff.owner_instructions.is_empty()
            && handoff
                .owner_instructions
                .iter()
                .all(|instruction| instruction.prohibited_in_studio),
        "all handoff instructions remain prohibited in Studio",
        "one or more handoff instructions are not prohibited in Studio",
        "studio.issue.shell_hostess_staging_acceptance_instruction_not_prohibited",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.provenance_checksum",
        handoff.provenance.checksum_algorithm == "fnv1a64.studio_staging_file_plan.v1"
            && handoff.provenance.plan_checksum.len() == 16,
        "handoff checksum uses the expected staging file-plan algorithm",
        "handoff checksum is missing or uses an unexpected algorithm",
        "studio.issue.shell_hostess_staging_acceptance_checksum_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance.provenance_sources",
        !handoff.provenance.source_artifact_kinds.is_empty()
            && !handoff.provenance.source_action_ids.is_empty()
            && !handoff.provenance.source_route_kinds.is_empty()
            && !handoff.provenance.target_keys.is_empty()
            && !handoff.provenance.destination_roots.is_empty(),
        "handoff provenance names artifacts, actions, routes, targets, and roots",
        "handoff provenance is missing artifacts, actions, routes, targets, or roots",
        "studio.issue.shell_hostess_staging_acceptance_provenance_missing",
    );

    for (instruction_id, owner, route_kind) in [
        (
            "hostess.review_staging_handoff",
            "rusty.hostess",
            "hostess.review.staging_handoff",
        ),
        (
            "hostess.copy_staging_files",
            "rusty.hostess",
            "hostess.stage.files_from_plan",
        ),
        (
            "manifold.review_command_session_contract",
            "rusty.manifold",
            "manifold.review.command_session_contract",
        ),
        (
            "hostess.collect_install_launch_evidence",
            "rusty.hostess",
            "hostess.collect.install_launch_evidence",
        ),
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_staging_acceptance.has_{instruction_id}"),
            shell_hostess_staging_handoff_has_ready_instruction(
                handoff,
                instruction_id,
                owner,
                route_kind,
            ),
            "handoff includes this ready external-owner instruction",
            "handoff is missing this ready external-owner instruction",
            "studio.issue.shell_hostess_staging_acceptance_instruction_missing",
        );
    }

    for action in [
        "stage_generated_shells",
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
        "collect_install_launch_evidence",
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_staging_acceptance.prohibits_{action}"),
            handoff
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "staging acceptance explicitly preserves this Studio prohibition",
            "staging acceptance is missing this Studio prohibition",
            "studio.issue.shell_hostess_staging_acceptance_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_staging_handoff_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessStagingAcceptanceStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessStagingAcceptanceStatus::Blocked
    } else {
        StudioShellHostessStagingAcceptanceStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessStagingAcceptanceStatus::Ready => None,
        StudioShellHostessStagingAcceptanceStatus::Blocked
        | StudioShellHostessStagingAcceptanceStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let entries = if status == StudioShellHostessStagingAcceptanceStatus::Rejected {
        Vec::new()
    } else {
        shell_hostess_staging_acceptance_entries(
            shell_hostess_staging_acceptance_item_specs(handoff, handoff_path),
            status,
            issue_code.as_deref(),
        )
    };
    let ready_item_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Ready)
        .count();
    let blocked_item_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Blocked)
        .count();
    let rejected_item_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Rejected)
        .count();

    StudioShellHostessStagingAcceptanceChecklistReport {
        schema_id: SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA.to_string(),
        source_handoff_schema: handoff.schema_id.clone(),
        handoff_path: handoff_path.map(|path| path.display().to_string()),
        file_plan_path: handoff.file_plan_path.clone(),
        preview_path: handoff.preview_path.clone(),
        intake_path: handoff.intake_path.clone(),
        package_path: handoff.package_path.clone(),
        handoff_manifest_path: handoff.handoff_manifest_path.clone(),
        selected_candidate_id: handoff.selected_candidate_id.clone(),
        envelope_id: handoff.envelope_id.clone(),
        manifest_id: handoff.manifest_id.clone(),
        project_id: handoff.project_id.clone(),
        project_revision: handoff.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.acceptance_check_only".to_string(),
        checklist_owner: "rusty.hostess".to_string(),
        handoff_owner: handoff.handoff_owner.clone(),
        staging_owner: handoff.staging_owner.clone(),
        command_session_authority: handoff.command_session_authority.clone(),
        install_launch_evidence_authority: handoff.install_launch_evidence_authority.clone(),
        studio_role: handoff.studio_role.clone(),
        request_count: handoff.request_count,
        ready_request_count: handoff.ready_request_count,
        blocked_request_count: handoff.blocked_request_count,
        instruction_count: handoff.instruction_count,
        ready_instruction_count: handoff.ready_instruction_count,
        blocked_instruction_count: handoff.blocked_instruction_count,
        checksum_algorithm: handoff.provenance.checksum_algorithm.clone(),
        plan_checksum: handoff.provenance.plan_checksum.clone(),
        ready_item_count,
        blocked_item_count,
        rejected_item_count,
        prohibited_actions: handoff.prohibited_actions.clone(),
        handoff_checks: checks,
        entries,
    }
}

fn shell_hostess_staging_handoff_has_ready_instruction(
    handoff: &StudioShellHostessStagingHandoffEnvelope,
    instruction_id: &str,
    owner: &str,
    route_kind: &str,
) -> bool {
    handoff.owner_instructions.iter().any(|instruction| {
        instruction.instruction_id == instruction_id
            && instruction.owner == owner
            && instruction.route_kind == route_kind
            && instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Ready
            && instruction.prohibited_in_studio
    })
}

pub fn shell_hostess_staging_acceptance_manifest_for_checklist(
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
    checklist_path: &Path,
    acceptance_id: Option<&str>,
    label: Option<&str>,
) -> StudioShellHostessStagingAcceptanceManifest {
    let acceptance_id = acceptance_id
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_hostess_staging_acceptance_id(checklist));
    let label = label
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_hostess_staging_acceptance_label(checklist));

    StudioShellHostessStagingAcceptanceManifest {
        schema_id: SHELL_HOSTESS_STAGING_ACCEPTANCE_MANIFEST_SCHEMA.to_string(),
        acceptance_id,
        label,
        checklist_path: checklist_path.display().to_string(),
        checklist_schema: checklist.schema_id.clone(),
        envelope_id: checklist.envelope_id.clone(),
        manifest_id: checklist.manifest_id.clone(),
        project_id: checklist.project_id.clone(),
        project_revision: checklist.project_revision,
        status: checklist.status,
        issue_code: checklist.issue_code.clone(),
        execution_policy: checklist.execution_policy.clone(),
        checklist_owner: checklist.checklist_owner.clone(),
        handoff_owner: checklist.handoff_owner.clone(),
        staging_owner: checklist.staging_owner.clone(),
        command_session_authority: checklist.command_session_authority.clone(),
        install_launch_evidence_authority: checklist.install_launch_evidence_authority.clone(),
        studio_role: checklist.studio_role.clone(),
        request_count: checklist.request_count,
        ready_request_count: checklist.ready_request_count,
        blocked_request_count: checklist.blocked_request_count,
        instruction_count: checklist.instruction_count,
        ready_instruction_count: checklist.ready_instruction_count,
        blocked_instruction_count: checklist.blocked_instruction_count,
        checksum_algorithm: checklist.checksum_algorithm.clone(),
        plan_checksum: checklist.plan_checksum.clone(),
        ready_item_count: checklist.ready_item_count,
        blocked_item_count: checklist.blocked_item_count,
        rejected_item_count: checklist.rejected_item_count,
        prohibited_actions: checklist.prohibited_actions.clone(),
    }
}

pub fn shell_hostess_staging_acceptance_index_for_manifests(
    acceptances: Vec<(StudioShellHostessStagingAcceptanceManifest, Option<PathBuf>)>,
    default_acceptance_id: Option<&str>,
) -> StudioShellHostessStagingAcceptanceIndex {
    let entries = acceptances
        .into_iter()
        .map(|(acceptance, acceptance_manifest_path)| {
            shell_hostess_staging_acceptance_index_entry_for_manifest(
                acceptance,
                acceptance_manifest_path,
            )
        })
        .collect::<Vec<_>>();

    shell_hostess_staging_acceptance_index_for_entries(entries, default_acceptance_id)
}

pub fn append_shell_hostess_staging_acceptance_index_manifests(
    index: &StudioShellHostessStagingAcceptanceIndex,
    acceptances: Vec<(StudioShellHostessStagingAcceptanceManifest, Option<PathBuf>)>,
    default_acceptance_id: Option<&str>,
) -> StudioShellHostessStagingAcceptanceIndex {
    let entries = index
        .entries
        .iter()
        .cloned()
        .chain(
            acceptances
                .into_iter()
                .map(|(acceptance, acceptance_manifest_path)| {
                    shell_hostess_staging_acceptance_index_entry_for_manifest(
                        acceptance,
                        acceptance_manifest_path,
                    )
                }),
        )
        .collect::<Vec<_>>();
    let default_acceptance_id = default_acceptance_id.or(index.default_acceptance_id.as_deref());

    shell_hostess_staging_acceptance_index_for_entries(entries, default_acceptance_id)
}

pub fn promote_shell_hostess_staging_acceptance_index_default(
    index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_id: &str,
) -> Option<StudioShellHostessStagingAcceptanceIndex> {
    index
        .entries
        .iter()
        .any(|entry| entry.acceptance_id == acceptance_id)
        .then(|| {
            shell_hostess_staging_acceptance_index_for_entries(
                index.entries.clone(),
                Some(acceptance_id),
            )
        })
}

fn shell_hostess_staging_acceptance_index_entry_for_manifest(
    acceptance: StudioShellHostessStagingAcceptanceManifest,
    acceptance_manifest_path: Option<PathBuf>,
) -> StudioShellHostessStagingAcceptanceIndexEntry {
    StudioShellHostessStagingAcceptanceIndexEntry {
        acceptance_id: acceptance.acceptance_id,
        label: acceptance.label,
        acceptance_manifest_path: acceptance_manifest_path.map(|path| path.display().to_string()),
        checklist_path: acceptance.checklist_path,
        checklist_schema: acceptance.checklist_schema,
        envelope_id: acceptance.envelope_id,
        manifest_id: acceptance.manifest_id,
        project_id: acceptance.project_id,
        project_revision: acceptance.project_revision,
        status: acceptance.status,
        issue_code: acceptance.issue_code,
        execution_policy: acceptance.execution_policy,
        checklist_owner: acceptance.checklist_owner,
        handoff_owner: acceptance.handoff_owner,
        staging_owner: acceptance.staging_owner,
        command_session_authority: acceptance.command_session_authority,
        install_launch_evidence_authority: acceptance.install_launch_evidence_authority,
        studio_role: acceptance.studio_role,
        request_count: acceptance.request_count,
        ready_request_count: acceptance.ready_request_count,
        blocked_request_count: acceptance.blocked_request_count,
        instruction_count: acceptance.instruction_count,
        ready_instruction_count: acceptance.ready_instruction_count,
        blocked_instruction_count: acceptance.blocked_instruction_count,
        checksum_algorithm: acceptance.checksum_algorithm,
        plan_checksum: acceptance.plan_checksum,
        ready_item_count: acceptance.ready_item_count,
        blocked_item_count: acceptance.blocked_item_count,
        rejected_item_count: acceptance.rejected_item_count,
    }
}

fn shell_hostess_staging_acceptance_index_for_entries(
    entries: Vec<StudioShellHostessStagingAcceptanceIndexEntry>,
    default_acceptance_id: Option<&str>,
) -> StudioShellHostessStagingAcceptanceIndex {
    let mut by_id = BTreeMap::new();
    for entry in entries {
        by_id.insert(entry.acceptance_id.clone(), entry);
    }

    let entries = by_id.into_values().collect::<Vec<_>>();
    let default_acceptance_id = default_acceptance_id
        .filter(|acceptance_id| {
            entries
                .iter()
                .any(|entry| entry.acceptance_id == *acceptance_id)
        })
        .map(str::to_string)
        .or_else(|| entries.first().map(|entry| entry.acceptance_id.clone()));

    StudioShellHostessStagingAcceptanceIndex {
        schema_id: SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA.to_string(),
        project_ids: unique_strings(entries.iter().filter_map(|entry| entry.project_id.clone())),
        envelope_ids: unique_strings(entries.iter().map(|entry| entry.envelope_id.clone())),
        manifest_ids: unique_strings(entries.iter().filter_map(|entry| entry.manifest_id.clone())),
        default_acceptance_id,
        acceptance_count: entries.len(),
        ready_acceptance_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Ready)
            .count(),
        blocked_acceptance_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Blocked)
            .count(),
        rejected_acceptance_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHostessStagingAcceptanceStatus::Rejected)
            .count(),
        entries,
    }
}

pub fn select_shell_hostess_staging_acceptance_index_entry<'a>(
    index: &'a StudioShellHostessStagingAcceptanceIndex,
    acceptance_id: Option<&str>,
) -> Option<&'a StudioShellHostessStagingAcceptanceIndexEntry> {
    let selected_id = acceptance_id.or(index.default_acceptance_id.as_deref());
    selected_id
        .and_then(|selected_id| {
            index
                .entries
                .iter()
                .find(|entry| entry.acceptance_id == selected_id)
        })
        .or_else(|| {
            acceptance_id
                .is_none()
                .then(|| index.entries.first())
                .flatten()
        })
}

pub fn summarize_shell_hostess_staging_acceptance_index_selection(
    index: &StudioShellHostessStagingAcceptanceIndex,
    index_path: Option<&Path>,
    requested_acceptance_id: Option<&str>,
) -> StudioShellHostessStagingAcceptanceSelectionReport {
    let selected_entry =
        select_shell_hostess_staging_acceptance_index_entry(index, requested_acceptance_id);
    let selected_acceptance_id = selected_entry.map(|entry| entry.acceptance_id.clone());
    let status = if index.entries.is_empty() {
        StudioShellHostessStagingAcceptanceSelectionStatus::Empty
    } else if selected_entry.is_some() {
        StudioShellHostessStagingAcceptanceSelectionStatus::Selected
    } else {
        StudioShellHostessStagingAcceptanceSelectionStatus::Missing
    };
    let issue_code = match status {
        StudioShellHostessStagingAcceptanceSelectionStatus::Selected => None,
        StudioShellHostessStagingAcceptanceSelectionStatus::Missing => {
            Some("studio.issue.shell_hostess_staging_acceptance_not_found".to_string())
        }
        StudioShellHostessStagingAcceptanceSelectionStatus::Empty => {
            Some("studio.issue.shell_hostess_staging_acceptance_index_empty".to_string())
        }
    };

    StudioShellHostessStagingAcceptanceSelectionReport {
        schema_id: SHELL_HOSTESS_STAGING_ACCEPTANCE_SELECTION_SCHEMA.to_string(),
        source_index_schema: index.schema_id.clone(),
        index_path: index_path.map(|path| path.display().to_string()),
        requested_acceptance_id: requested_acceptance_id.map(str::to_string),
        default_acceptance_id: index.default_acceptance_id.clone(),
        selected_acceptance_id: selected_acceptance_id.clone(),
        status,
        issue_code,
        acceptance_count: index.acceptance_count,
        ready_acceptance_count: index.ready_acceptance_count,
        blocked_acceptance_count: index.blocked_acceptance_count,
        rejected_acceptance_count: index.rejected_acceptance_count,
        project_ids: index.project_ids.clone(),
        envelope_ids: index.envelope_ids.clone(),
        manifest_ids: index.manifest_ids.clone(),
        entries: index
            .entries
            .iter()
            .map(|entry| StudioShellHostessStagingAcceptanceSelectionEntry {
                acceptance_id: entry.acceptance_id.clone(),
                label: entry.label.clone(),
                selected: selected_acceptance_id.as_deref() == Some(entry.acceptance_id.as_str()),
                default: index.default_acceptance_id.as_deref()
                    == Some(entry.acceptance_id.as_str()),
                acceptance_manifest_path: entry.acceptance_manifest_path.clone(),
                checklist_path: entry.checklist_path.clone(),
                envelope_id: entry.envelope_id.clone(),
                project_id: entry.project_id.clone(),
                project_revision: entry.project_revision,
                status: entry.status,
                issue_code: entry.issue_code.clone(),
                ready_item_count: entry.ready_item_count,
                blocked_item_count: entry.blocked_item_count,
                rejected_item_count: entry.rejected_item_count,
                request_count: entry.request_count,
                instruction_count: entry.instruction_count,
            })
            .collect(),
    }
}

fn default_shell_hostess_staging_acceptance_id(
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> String {
    format!(
        "studio.hostess_staging_acceptance.{}.rev{}.{}",
        checklist.project_id.as_deref().unwrap_or("unknown_project"),
        checklist
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        shell_hostess_staging_acceptance_status_key(checklist.status)
    )
}

fn default_shell_hostess_staging_acceptance_label(
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> String {
    format!(
        "{} revision {} {} Hostess staging acceptance",
        checklist.project_id.as_deref().unwrap_or("unknown project"),
        checklist
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        shell_hostess_staging_acceptance_status_key(checklist.status)
    )
}

fn shell_hostess_staging_acceptance_status_key(
    status: StudioShellHostessStagingAcceptanceStatus,
) -> &'static str {
    match status {
        StudioShellHostessStagingAcceptanceStatus::Ready => "ready",
        StudioShellHostessStagingAcceptanceStatus::Blocked => "blocked",
        StudioShellHostessStagingAcceptanceStatus::Rejected => "rejected",
    }
}
