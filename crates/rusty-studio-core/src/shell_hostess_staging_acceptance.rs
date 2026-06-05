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

#[derive(Clone, Debug)]
struct StagingAcceptanceItemSpec {
    item_id: &'static str,
    owner: &'static str,
    item_kind: &'static str,
    route_kind: &'static str,
    source: &'static str,
    evidence: String,
    next_required_action: &'static str,
    prohibited_in_studio: bool,
    expected_input_path: Option<String>,
}

fn shell_hostess_staging_acceptance_item_specs(
    handoff: &StudioShellHostessStagingHandoffEnvelope,
    handoff_path: Option<&Path>,
) -> Vec<StagingAcceptanceItemSpec> {
    let handoff_path = handoff_path.map(|path| path.display().to_string());
    let file_plan_path = handoff.file_plan_path.clone();
    vec![
        StagingAcceptanceItemSpec {
            item_id: "hostess.accept_staging_handoff",
            owner: "rusty.hostess",
            item_kind: "hostess_acceptance_gate",
            route_kind: "hostess.accept.staging_handoff",
            source: "hostess_staging_handoff_envelope",
            evidence: format!(
                "handoff envelope {} is ready for Hostess acceptance",
                handoff.envelope_id
            ),
            next_required_action: "accept_or_reject_handoff_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: handoff_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.verify_staging_file_plan_checksum",
            owner: "rusty.hostess",
            item_kind: "hostess_checksum_gate",
            route_kind: "hostess.verify.staging_file_plan_checksum",
            source: "hostess_staging_handoff_envelope",
            evidence: format!(
                "{} checksum {}",
                handoff.provenance.checksum_algorithm, handoff.provenance.plan_checksum
            ),
            next_required_action: "verify_file_plan_checksum_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.review_staging_file_requests",
            owner: "rusty.hostess",
            item_kind: "hostess_file_plan_review_gate",
            route_kind: "hostess.review.staging_file_requests",
            source: "hostess_staging_handoff_envelope",
            evidence: format!(
                "{} ready requests over {} planned files",
                handoff.ready_request_count, handoff.planned_file_count
            ),
            next_required_action: "review_shared_and_target_requests_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.copy_staging_files",
            owner: "rusty.hostess",
            item_kind: "hostess_file_copy_request",
            route_kind: "hostess.stage.files_from_plan",
            source: "hostess_staging_file_plan",
            evidence: "file copy remains an external Hostess action".to_string(),
            next_required_action: "copy_stage_files_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "manifold.review_command_session_contract",
            owner: "rusty.manifold",
            item_kind: "manifold_contract_review",
            route_kind: "manifold.review.command_session_contract",
            source: "hostess_staging_handoff_envelope",
            evidence: "Manifold remains command/session authority".to_string(),
            next_required_action: "review_command_session_contract_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: handoff_path.clone(),
        },
        StagingAcceptanceItemSpec {
            item_id: "hostess.collect_install_launch_evidence",
            owner: "rusty.hostess",
            item_kind: "hostess_evidence_collection_request",
            route_kind: "hostess.collect.install_launch_evidence",
            source: "hostess_staging_handoff_envelope",
            evidence: "install/launch evidence remains an external Hostess action".to_string(),
            next_required_action: "collect_install_launch_evidence_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: handoff_path,
        },
    ]
}

fn shell_hostess_staging_acceptance_entries(
    specs: Vec<StagingAcceptanceItemSpec>,
    checklist_status: StudioShellHostessStagingAcceptanceStatus,
    checklist_issue_code: Option<&str>,
) -> Vec<StudioShellHostessStagingAcceptanceChecklistEntry> {
    specs
        .into_iter()
        .map(|spec| {
            let status = if checklist_status == StudioShellHostessStagingAcceptanceStatus::Ready {
                StudioShellHostessStagingAcceptanceStatus::Ready
            } else {
                StudioShellHostessStagingAcceptanceStatus::Blocked
            };
            StudioShellHostessStagingAcceptanceChecklistEntry {
                item_id: spec.item_id.to_string(),
                owner: spec.owner.to_string(),
                status,
                issue_code: (status != StudioShellHostessStagingAcceptanceStatus::Ready).then(
                    || {
                        checklist_issue_code
                            .unwrap_or("studio.issue.shell_hostess_staging_acceptance_blocked")
                            .to_string()
                    },
                ),
                item_kind: spec.item_kind.to_string(),
                route_kind: spec.route_kind.to_string(),
                source: spec.source.to_string(),
                evidence: spec.evidence,
                next_required_action: spec.next_required_action.to_string(),
                prohibited_in_studio: spec.prohibited_in_studio,
                expected_input_path: spec.expected_input_path,
            }
        })
        .collect()
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

pub fn shell_hostess_staging_execution_request_for_acceptance_index_entry(
    acceptance_index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_index_path: Option<&Path>,
    acceptance_index_entry: &StudioShellHostessStagingAcceptanceIndexEntry,
    acceptance_manifest_path: Option<&Path>,
    acceptance: &StudioShellHostessStagingAcceptanceManifest,
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
    handoff_path: Option<&Path>,
    handoff: &StudioShellHostessStagingHandoffEnvelope,
) -> StudioShellHostessStagingExecutionRequestReport {
    shell_hostess_staging_execution_request_for_acceptance_index_entry_with_pmb_review(
        acceptance_index,
        acceptance_index_path,
        acceptance_index_entry,
        acceptance_manifest_path,
        acceptance,
        checklist,
        handoff_path,
        handoff,
        None,
        None,
        false,
    )
}

pub fn shell_hostess_staging_execution_request_for_acceptance_index_entry_with_pmb_review(
    acceptance_index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_index_path: Option<&Path>,
    acceptance_index_entry: &StudioShellHostessStagingAcceptanceIndexEntry,
    acceptance_manifest_path: Option<&Path>,
    acceptance: &StudioShellHostessStagingAcceptanceManifest,
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
    handoff_path: Option<&Path>,
    handoff: &StudioShellHostessStagingHandoffEnvelope,
    pmb_shell_handoff_review_path: Option<&Path>,
    pmb_shell_handoff_review: Option<&StudioProjectedMotionBreathShellHandoffReviewReport>,
    require_pmb_shell_handoff_review: bool,
) -> StudioShellHostessStagingExecutionRequestReport {
    let mut checks = Vec::new();
    let expected_manifest_path = acceptance_manifest_path.map(|path| path.display().to_string());
    let manifest_path_matches = match (
        expected_manifest_path.as_deref(),
        acceptance_index_entry.acceptance_manifest_path.as_deref(),
    ) {
        (Some(expected), Some(actual)) => actual == expected,
        (None, Some(actual)) => !actual.trim().is_empty(),
        _ => false,
    };
    let expected_handoff_path = handoff_path
        .map(|path| path.display().to_string())
        .or_else(|| checklist.handoff_path.clone());
    let expected_acceptance_specs =
        shell_hostess_staging_acceptance_item_specs(handoff, handoff_path);
    let expected_acceptance_entries_match = expected_acceptance_specs.len()
        == checklist.entries.len()
        && expected_acceptance_specs.iter().all(|spec| {
            checklist.entries.iter().any(|entry| {
                entry.item_id == spec.item_id
                    && entry.owner == spec.owner
                    && entry.status == StudioShellHostessStagingAcceptanceStatus::Ready
                    && entry.issue_code.is_none()
                    && entry.item_kind == spec.item_kind
                    && entry.route_kind == spec.route_kind
                    && entry.source == spec.source
                    && entry.prohibited_in_studio == spec.prohibited_in_studio
                    && entry.expected_input_path == spec.expected_input_path
            })
        });
    let pmb_shell_handoff_review_required =
        require_pmb_shell_handoff_review || pmb_shell_handoff_review_path.is_some();
    let pmb_shell_handoff_review_path_string =
        pmb_shell_handoff_review_path.map(|path| path.display().to_string());
    let pmb_shell_handoff_review_ready =
        pmb_shell_handoff_review_is_ready(pmb_shell_handoff_review);
    let pmb_shell_handoff_review_issue_code =
        pmb_shell_handoff_review_issue_code(pmb_shell_handoff_review);
    let hostess_operator_start_preflight_cli_args = hostess_operator_start_preflight_pmb_cli_args(
        pmb_shell_handoff_review_required,
        pmb_shell_handoff_review_path_string.as_deref(),
    );

    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_index_schema",
        acceptance_index.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA,
        "source Hostess acceptance index schema is supported",
        "source Hostess acceptance index schema is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.selected_acceptance",
        acceptance_index_entry.acceptance_id == acceptance.acceptance_id,
        "selected acceptance index entry matches the loaded acceptance manifest",
        "selected acceptance index entry differs from the loaded acceptance manifest",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_manifest_path",
        manifest_path_matches,
        "selected acceptance index entry names the loaded acceptance manifest",
        "selected acceptance index entry is missing or stale",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_manifest_schema",
        acceptance.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_MANIFEST_SCHEMA,
        "acceptance manifest schema is supported",
        "acceptance manifest schema is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_checklist_schema",
        acceptance.checklist_schema == checklist.schema_id
            && checklist.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA,
        "acceptance manifest names the loaded checklist schema",
        "acceptance manifest checklist schema differs from the loaded checklist",
        "studio.issue.shell_hostess_staging_acceptance_checklist_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_checklist_path",
        !acceptance.checklist_path.trim().is_empty()
            && acceptance_index_entry.checklist_path == acceptance.checklist_path,
        "acceptance manifest and index agree on the checklist path",
        "acceptance manifest and index checklist paths differ",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_source_metadata",
        acceptance.envelope_id == checklist.envelope_id
            && acceptance.manifest_id == checklist.manifest_id
            && acceptance.project_id == checklist.project_id
            && acceptance.project_revision == checklist.project_revision
            && acceptance_index_entry.envelope_id == acceptance.envelope_id
            && acceptance_index_entry.manifest_id == acceptance.manifest_id
            && acceptance_index_entry.project_id == acceptance.project_id
            && acceptance_index_entry.project_revision == acceptance.project_revision,
        "acceptance manifest, index, and checklist source metadata match",
        "acceptance manifest, index, and checklist source metadata differ",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_status_counts",
        acceptance.status == checklist.status
            && acceptance.issue_code == checklist.issue_code
            && acceptance.ready_item_count == checklist.ready_item_count
            && acceptance.blocked_item_count == checklist.blocked_item_count
            && acceptance.rejected_item_count == checklist.rejected_item_count
            && acceptance.request_count == checklist.request_count
            && acceptance.instruction_count == checklist.instruction_count
            && acceptance_index_entry.status == acceptance.status
            && acceptance_index_entry.issue_code == acceptance.issue_code
            && acceptance_index_entry.ready_item_count == acceptance.ready_item_count
            && acceptance_index_entry.blocked_item_count == acceptance.blocked_item_count
            && acceptance_index_entry.rejected_item_count == acceptance.rejected_item_count,
        "acceptance manifest, index, and checklist readiness counts match",
        "acceptance manifest, index, and checklist readiness counts differ",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_ready",
        checklist.status == StudioShellHostessStagingAcceptanceStatus::Ready,
        "selected Hostess staging acceptance is ready",
        "selected Hostess staging acceptance is not ready",
        checklist
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_staging_acceptance_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_execution_policy",
        acceptance.execution_policy == "not_executed.acceptance_check_only"
            && checklist.execution_policy == "not_executed.acceptance_check_only"
            && acceptance_index_entry.execution_policy == acceptance.execution_policy,
        "acceptance artifacts remain acceptance-check-only",
        "acceptance artifacts changed execution policy",
        "studio.issue.shell_hostess_staging_acceptance_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.handoff_schema",
        handoff.schema_id == SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA,
        "source Hostess staging handoff schema is supported",
        "source Hostess staging handoff schema is unsupported",
        "studio.issue.shell_hostess_staging_handoff_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.handoff_path",
        expected_handoff_path
            .as_deref()
            .is_some_and(|path| checklist.handoff_path.as_deref() == Some(path)),
        "acceptance checklist names the loaded handoff envelope",
        "acceptance checklist handoff path is missing or stale",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.handoff_ready",
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
        "studio.check.shell_hostess_staging_execution_request.handoff_execution_policy",
        handoff.execution_policy == "not_executed.handoff_only",
        "source handoff remains handoff-only",
        "source handoff execution policy changed",
        "studio.issue.shell_hostess_staging_handoff_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.handoff_identity",
        handoff.envelope_id == checklist.envelope_id
            && handoff.manifest_id == checklist.manifest_id
            && handoff.project_id == checklist.project_id
            && handoff.project_revision == checklist.project_revision
            && handoff.selected_candidate_id == checklist.selected_candidate_id,
        "handoff identity matches the selected acceptance checklist",
        "handoff identity differs from the selected acceptance checklist",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.handoff_provenance",
        handoff.file_plan_path == checklist.file_plan_path
            && handoff.preview_path == checklist.preview_path
            && handoff.intake_path == checklist.intake_path
            && handoff.package_path == checklist.package_path
            && handoff.handoff_manifest_path == checklist.handoff_manifest_path
            && handoff.provenance.checksum_algorithm == checklist.checksum_algorithm
            && handoff.provenance.plan_checksum == checklist.plan_checksum,
        "handoff provenance matches the selected acceptance checklist",
        "handoff provenance differs from the selected acceptance checklist",
        "studio.issue.shell_hostess_staging_execution_request_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.authority",
        acceptance.checklist_owner == "rusty.hostess"
            && acceptance.handoff_owner == "rusty.hostess"
            && acceptance.staging_owner == "rusty.hostess"
            && checklist.checklist_owner == "rusty.hostess"
            && checklist.handoff_owner == "rusty.hostess"
            && checklist.staging_owner == "rusty.hostess"
            && handoff.handoff_owner == "rusty.hostess"
            && handoff.staging_owner == "rusty.hostess"
            && checklist.command_session_authority.as_deref() == Some("rusty.manifold")
            && checklist.install_launch_evidence_authority.as_deref() == Some("rusty.hostess")
            && checklist.studio_role.as_deref() == Some("authoring.export_planning"),
        "Hostess, Manifold, and Studio authority fields remain separated",
        "Hostess, Manifold, or Studio authority fields drifted",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.acceptance_entry_contracts",
        expected_acceptance_entries_match,
        "acceptance entries match the expected Hostess/Manifold adapter contracts",
        "acceptance entries drifted from expected Hostess/Manifold adapter contracts",
        "studio.issue.shell_hostess_staging_acceptance_entry_drift",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.no_failed_handoff_checks",
        checklist
            .handoff_checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "acceptance checklist carries no failed handoff checks",
        "acceptance checklist carries failed handoff checks",
        "studio.issue.shell_hostess_staging_handoff_failed_check",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.prohibited_studio_actions",
        [
            "stage_generated_shells",
            "install",
            "launch",
            "open_command_session",
            "collect_device_evidence",
            "collect_install_launch_evidence",
        ]
        .iter()
        .all(|action| {
            acceptance
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action)
                && checklist
                    .prohibited_actions
                    .iter()
                    .any(|candidate| candidate == action)
        }),
        "execution request preserves all Studio prohibitions",
        "execution request is missing one or more Studio prohibitions",
        "studio.issue.shell_hostess_staging_acceptance_prohibited_action_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_execution_request.pmb_shell_handoff_review",
        !pmb_shell_handoff_review_required
            || (pmb_shell_handoff_review_path_string.is_some()
                && pmb_shell_handoff_review_ready
                && pmb_shell_handoff_review_issue_code.is_none()),
        "PMB shell handoff review is ready for Hostess operator-start preflight",
        "PMB shell handoff review is missing, blocked, or invalid",
        pmb_shell_handoff_review_issue_code
            .as_deref()
            .unwrap_or("studio.issue.projected_motion_breath_shell_handoff_review_missing"),
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_staging_acceptance_index_schema")
                    | Some("studio.issue.shell_hostess_staging_acceptance_manifest_schema")
                    | Some("studio.issue.shell_hostess_staging_acceptance_checklist_schema")
                    | Some("studio.issue.shell_hostess_staging_handoff_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessStagingExecutionRequestStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessStagingExecutionRequestStatus::Blocked
    } else {
        StudioShellHostessStagingExecutionRequestStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessStagingExecutionRequestStatus::Ready => None,
        StudioShellHostessStagingExecutionRequestStatus::Blocked
        | StudioShellHostessStagingExecutionRequestStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let request_id =
        default_shell_hostess_staging_execution_request_id(&acceptance.acceptance_id, checklist);
    let actions = shell_hostess_staging_execution_actions(checklist, status, issue_code.as_deref());
    let ready_adapter_action_count = actions
        .iter()
        .filter(|action| action.status == StudioShellHostessStagingExecutionActionStatus::Ready)
        .count();
    let blocked_adapter_action_count = actions
        .iter()
        .filter(|action| action.status == StudioShellHostessStagingExecutionActionStatus::Blocked)
        .count();
    let required_action_ids = actions
        .iter()
        .map(|action| action.action_id.clone())
        .collect::<Vec<_>>();
    let ack_template = shell_hostess_staging_execution_ack_template(
        &request_id,
        required_action_ids.clone(),
        checklist.command_session_authority.clone(),
        checklist.install_launch_evidence_authority.clone(),
    );
    let reject_template =
        shell_hostess_staging_execution_reject_template(&request_id, required_action_ids);

    StudioShellHostessStagingExecutionRequestReport {
        schema_id: SHELL_HOSTESS_STAGING_EXECUTION_REQUEST_SCHEMA.to_string(),
        request_id,
        source_acceptance_index_schema: Some(acceptance_index.schema_id.clone()),
        acceptance_index_path: acceptance_index_path.map(|path| path.display().to_string()),
        selected_acceptance_id: acceptance.acceptance_id.clone(),
        acceptance_manifest_path: acceptance_manifest_path.map(|path| path.display().to_string()),
        acceptance_schema: acceptance.schema_id.clone(),
        acceptance_checklist_path: acceptance.checklist_path.clone(),
        acceptance_checklist_schema: checklist.schema_id.clone(),
        source_acceptance_status: checklist.status,
        source_handoff_schema: handoff.schema_id.clone(),
        handoff_path: expected_handoff_path,
        envelope_id: checklist.envelope_id.clone(),
        manifest_id: checklist.manifest_id.clone(),
        project_id: checklist.project_id.clone(),
        project_revision: checklist.project_revision,
        selected_candidate_id: checklist.selected_candidate_id.clone(),
        file_plan_path: checklist.file_plan_path.clone(),
        preview_path: checklist.preview_path.clone(),
        intake_path: checklist.intake_path.clone(),
        package_path: checklist.package_path.clone(),
        handoff_manifest_path: checklist.handoff_manifest_path.clone(),
        pmb_shell_handoff_review_required,
        pmb_shell_handoff_review_path: pmb_shell_handoff_review_path_string,
        source_pmb_shell_handoff_review_schema: pmb_shell_handoff_review
            .map(|review| review.schema_id.clone()),
        source_pmb_shell_handoff_review_status: pmb_shell_handoff_review
            .map(|review| review.status),
        source_pmb_shell_handoff_review_issue_code: pmb_shell_handoff_review
            .and_then(|review| review.issue_code.clone()),
        source_pmb_shell_handoff_id: pmb_shell_handoff_review
            .and_then(|review| review.handoff_id.clone()),
        source_pmb_shell_app_id: pmb_shell_handoff_review
            .and_then(|review| review.shell_app_id.clone()),
        pmb_shell_handoff_review_ready,
        hostess_operator_start_preflight_cli_args,
        status,
        issue_code,
        execution_policy: "not_executed.hostess_request_only".to_string(),
        adapter_owner: "rusty.hostess".to_string(),
        requester_role: "rusty.studio".to_string(),
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
        prohibited_studio_actions: checklist.prohibited_actions.clone(),
        adapter_action_count: actions.len(),
        ready_adapter_action_count,
        blocked_adapter_action_count,
        actions,
        checks,
        ack_template,
        reject_template,
    }
}

fn pmb_shell_handoff_review_is_ready(
    review: Option<&StudioProjectedMotionBreathShellHandoffReviewReport>,
) -> bool {
    let Some(review) = review else {
        return false;
    };
    review.schema_id == PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA
        && review.status == StudioProjectedMotionBreathShellHandoffReviewStatus::Ready
        && review.issue_code.is_none()
        && review.execution_policy == "not_executed.review_only"
        && review.runtime_authority == "rusty.manifold"
        && review.authoring_authority == "rusty.studio"
        && review.platform_validation_authority == "rusty.hostess"
        && !review.runtime_execution_performed
        && !review.platform_execution_performed
        && !review.broker_transport_used
        && !review.downstream_shell_runtime_used
        && !review.legacy_app_dependency_used
        && review.required_binding_count > 0
        && review.ready_required_binding_count == review.required_binding_count
        && review.feedback_receipt_exported
        && review.feedback_sink_provides_receipt
        && review
            .command_ids
            .iter()
            .any(|command_id| command_id == "command.breath.status")
        && !review.transport_ids.is_empty()
}

fn pmb_shell_handoff_review_issue_code(
    review: Option<&StudioProjectedMotionBreathShellHandoffReviewReport>,
) -> Option<String> {
    let Some(review) = review else {
        return Some(
            "studio.issue.projected_motion_breath_shell_handoff_review_missing".to_string(),
        );
    };
    if review.schema_id != PROJECTED_MOTION_BREATH_SHELL_HANDOFF_REVIEW_SCHEMA {
        return Some(
            "studio.issue.projected_motion_breath_shell_handoff_review_schema".to_string(),
        );
    }
    if review.status != StudioProjectedMotionBreathShellHandoffReviewStatus::Ready {
        return Some(review.issue_code.clone().unwrap_or_else(|| {
            "studio.issue.projected_motion_breath_shell_handoff_review_not_ready".to_string()
        }));
    }
    if !pmb_shell_handoff_review_is_ready(Some(review)) {
        return Some(review.issue_code.clone().unwrap_or_else(|| {
            "studio.issue.projected_motion_breath_shell_handoff_review_boundary".to_string()
        }));
    }
    None
}

fn hostess_operator_start_preflight_pmb_cli_args(
    pmb_shell_handoff_review_required: bool,
    pmb_shell_handoff_review_path: Option<&str>,
) -> Vec<String> {
    if !pmb_shell_handoff_review_required {
        return Vec::new();
    }
    let mut args = Vec::new();
    if let Some(path) = pmb_shell_handoff_review_path {
        args.push("--pmb-shell-handoff-review-in".to_string());
        args.push(path.to_string());
    }
    args.push("--require-pmb-shell-handoff-review".to_string());
    args
}

fn shell_hostess_staging_execution_actions(
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
    request_status: StudioShellHostessStagingExecutionRequestStatus,
    request_issue_code: Option<&str>,
) -> Vec<StudioShellHostessStagingExecutionAction> {
    checklist
        .entries
        .iter()
        .map(|entry| {
            let status = if request_status == StudioShellHostessStagingExecutionRequestStatus::Ready
                && entry.status == StudioShellHostessStagingAcceptanceStatus::Ready
                && entry.prohibited_in_studio
            {
                StudioShellHostessStagingExecutionActionStatus::Ready
            } else {
                StudioShellHostessStagingExecutionActionStatus::Blocked
            };
            StudioShellHostessStagingExecutionAction {
                action_id: format!("adapter.{}", entry.item_id),
                owner: entry.owner.clone(),
                status,
                issue_code: (status == StudioShellHostessStagingExecutionActionStatus::Blocked)
                    .then(|| {
                        entry
                            .issue_code
                            .as_deref()
                            .or(request_issue_code)
                            .unwrap_or(
                                "studio.issue.shell_hostess_staging_execution_request_blocked",
                            )
                            .to_string()
                    }),
                action_kind: entry.item_kind.clone(),
                route_kind: entry.route_kind.clone(),
                source_item_id: entry.item_id.clone(),
                responsible_authority: entry.owner.clone(),
                expected_input_path: entry.expected_input_path.clone(),
                next_required_action: entry.next_required_action.clone(),
                ack_required: true,
                execution_in_studio: false,
            }
        })
        .collect()
}

fn shell_hostess_staging_execution_ack_template(
    request_id: &str,
    required_action_ids: Vec<String>,
    command_session_authority: Option<String>,
    install_launch_evidence_authority: Option<String>,
) -> StudioShellHostessStagingExecutionAck {
    StudioShellHostessStagingExecutionAck {
        schema_id: SHELL_HOSTESS_STAGING_EXECUTION_ACK_SCHEMA.to_string(),
        request_id: request_id.to_string(),
        accepted_by: "rusty.hostess".to_string(),
        ack_status: StudioShellHostessStagingExecutionAckStatus::Pending,
        execution_in_studio: false,
        command_session_authority,
        install_launch_evidence_authority,
        required_action_ids,
        accepted_action_ids: Vec::new(),
        required_evidence_kinds: vec![
            "hostess_staging_request_ack".to_string(),
            "hostess_file_copy_stage_receipt".to_string(),
            "hostess_install_launch_evidence_receipt".to_string(),
            "manifold_command_session_contract_review".to_string(),
        ],
        issue_code: None,
    }
}

fn shell_hostess_staging_execution_reject_template(
    request_id: &str,
    request_action_ids: Vec<String>,
) -> StudioShellHostessStagingExecutionReject {
    StudioShellHostessStagingExecutionReject {
        schema_id: SHELL_HOSTESS_STAGING_EXECUTION_REJECT_SCHEMA.to_string(),
        request_id: request_id.to_string(),
        rejected_by: "rusty.hostess".to_string(),
        reject_status: StudioShellHostessStagingExecutionRejectStatus::Pending,
        execution_in_studio: false,
        request_action_ids,
        rejected_action_ids: Vec::new(),
        reason_code: None,
        next_required_action: "hostess_ack_or_reject_request_outside_studio".to_string(),
        issue_code: None,
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

fn default_shell_hostess_staging_execution_request_id(
    acceptance_id: &str,
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> String {
    format!(
        "studio.hostess_staging_execution_request.{}.rev{}.{}",
        checklist.project_id.as_deref().unwrap_or("unknown_project"),
        checklist
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        acceptance_id
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
