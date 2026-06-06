use super::*;

pub fn shell_hostess_staging_handoff_envelope_for_file_plan(
    file_plan: &StudioShellHostessStagingFilePlan,
    file_plan_path: Option<&Path>,
) -> StudioShellHostessStagingHandoffEnvelope {
    let provenance = shell_hostess_staging_handoff_provenance(file_plan);
    let instruction_specs = shell_hostess_staging_handoff_instruction_specs(file_plan_path);
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.source_file_plan_schema",
        file_plan.schema_id == SHELL_HOSTESS_STAGING_FILE_PLAN_SCHEMA,
        "source Hostess staging file-plan schema is supported",
        "source Hostess staging file-plan schema is unsupported",
        "studio.issue.shell_hostess_staging_file_plan_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.file_plan_path",
        file_plan_path.is_some(),
        "source Hostess staging file plan has a durable path",
        "source Hostess staging file plan path is missing",
        "studio.issue.shell_hostess_staging_handoff_file_plan_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.file_plan_ready",
        file_plan.status == StudioShellHostessStagingFilePlanStatus::Ready,
        "source Hostess staging file plan is ready",
        "source Hostess staging file plan is not ready",
        file_plan
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_staging_file_plan_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.file_plan_execution_policy",
        file_plan.execution_policy == "not_executed.dry_run_only",
        "source file plan is dry-run only and not executed",
        "source file plan execution policy is not dry-run only",
        "studio.issue.shell_hostess_staging_file_plan_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.staging_owner",
        file_plan.staging_owner == "rusty.hostess",
        "Hostess remains staging owner",
        "staging owner must remain rusty.hostess",
        "studio.issue.staging_owner_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.runtime_command_authority",
        file_plan.command_session_authority.as_deref() == Some("rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.runtime_host_authority",
        file_plan.install_launch_evidence_authority.as_deref() == Some("rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.studio_role",
        file_plan.studio_role.as_deref() == Some("authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.source_file_plan_checks_pass",
        file_plan
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "source Hostess staging file-plan checks all pass",
        "source Hostess staging file-plan contains failed checks",
        "studio.issue.shell_hostess_staging_file_plan_failed_check",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.requests_ready",
        !file_plan.requests.is_empty()
            && file_plan
                .requests
                .iter()
                .all(|request| request.status == StudioShellHostessStagingFileRequestStatus::Ready),
        "all source file-plan requests are ready",
        "one or more source file-plan requests are blocked",
        "studio.issue.shell_hostess_staging_handoff_request_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.request_counts",
        file_plan.request_count == file_plan.requests.len()
            && file_plan.ready_request_count == file_plan.requests.len()
            && file_plan.blocked_request_count == 0,
        "source file-plan request counts match request rows",
        "source file-plan request counts do not match request rows",
        "studio.issue.shell_hostess_staging_handoff_request_count_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.planned_file_counts",
        file_plan.planned_file_count
            == file_plan
                .requests
                .iter()
                .map(|request| request.planned_file_count)
                .sum::<usize>(),
        "source file-plan planned-file count matches request rows",
        "source file-plan planned-file count does not match request rows",
        "studio.issue.shell_hostess_staging_handoff_file_count_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.provenance_present",
        !provenance.plan_checksum.is_empty()
            && !provenance.source_artifact_kinds.is_empty()
            && !provenance.source_action_ids.is_empty()
            && !provenance.source_route_kinds.is_empty()
            && !provenance.target_keys.is_empty(),
        "handoff envelope has checksum and source provenance summary",
        "handoff envelope is missing checksum or source provenance summary",
        "studio.issue.shell_hostess_staging_handoff_provenance_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.instructions_present",
        instruction_specs.len() >= 4,
        "handoff envelope includes external-owner instructions",
        "handoff envelope is missing external-owner instructions",
        "studio.issue.shell_hostess_staging_handoff_instruction_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_handoff.instructions_prohibited_in_studio",
        instruction_specs
            .iter()
            .all(|spec| spec.prohibited_in_studio),
        "all handoff instructions remain prohibited in Studio",
        "one or more handoff instructions are not prohibited in Studio",
        "studio.issue.shell_hostess_staging_handoff_instruction_not_prohibited",
    );

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
            &format!("studio.check.shell_hostess_staging_handoff.prohibits_{action}"),
            file_plan
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "staging handoff explicitly preserves this Studio prohibition",
            "staging handoff is missing this Studio prohibition",
            "studio.issue.shell_hostess_staging_handoff_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_staging_file_plan_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessStagingHandoffEnvelopeStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessStagingHandoffEnvelopeStatus::Blocked
    } else {
        StudioShellHostessStagingHandoffEnvelopeStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessStagingHandoffEnvelopeStatus::Ready => None,
        StudioShellHostessStagingHandoffEnvelopeStatus::Blocked
        | StudioShellHostessStagingHandoffEnvelopeStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let owner_instructions = shell_hostess_staging_handoff_instructions(
        instruction_specs,
        status,
        issue_code.as_deref(),
    );
    let ready_instruction_count = owner_instructions
        .iter()
        .filter(|instruction| {
            instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Ready
        })
        .count();
    let blocked_instruction_count = owner_instructions
        .iter()
        .filter(|instruction| {
            instruction.status == StudioShellHostessStagingHandoffInstructionStatus::Blocked
        })
        .count();

    StudioShellHostessStagingHandoffEnvelope {
        schema_id: SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA.to_string(),
        source_file_plan_schema: file_plan.schema_id.clone(),
        file_plan_path: file_plan_path.map(|path| path.display().to_string()),
        preview_path: file_plan.preview_path.clone(),
        intake_path: file_plan.intake_path.clone(),
        package_path: file_plan.package_path.clone(),
        handoff_manifest_path: file_plan.handoff_manifest_path.clone(),
        selected_candidate_id: file_plan.selected_candidate_id.clone(),
        envelope_id: default_shell_hostess_staging_handoff_envelope_id(file_plan),
        manifest_id: file_plan.manifest_id.clone(),
        project_id: file_plan.project_id.clone(),
        project_revision: file_plan.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.handoff_only".to_string(),
        handoff_owner: "rusty.hostess".to_string(),
        staging_owner: file_plan.staging_owner.clone(),
        command_session_authority: file_plan.command_session_authority.clone(),
        install_launch_evidence_authority: file_plan.install_launch_evidence_authority.clone(),
        studio_role: file_plan.studio_role.clone(),
        planned_file_count: file_plan.planned_file_count,
        request_count: file_plan.request_count,
        ready_request_count: file_plan.ready_request_count,
        blocked_request_count: file_plan.blocked_request_count,
        target_request_count: file_plan.target_request_count,
        shared_request_count: file_plan.shared_request_count,
        instruction_count: owner_instructions.len(),
        ready_instruction_count,
        blocked_instruction_count,
        provenance,
        request_summaries: shell_hostess_staging_handoff_request_summaries(file_plan),
        owner_instructions,
        prohibited_actions: file_plan.prohibited_actions.clone(),
        checks,
    }
}

#[derive(Clone, Debug)]
struct StagingHandoffInstructionSpec {
    instruction_id: &'static str,
    owner: &'static str,
    instruction_kind: &'static str,
    route_kind: &'static str,
    source: &'static str,
    next_required_action: &'static str,
    prohibited_in_studio: bool,
    expected_input_path: Option<String>,
}

fn shell_hostess_staging_handoff_instruction_specs(
    file_plan_path: Option<&Path>,
) -> Vec<StagingHandoffInstructionSpec> {
    let file_plan_path = file_plan_path.map(|path| path.display().to_string());
    vec![
        StagingHandoffInstructionSpec {
            instruction_id: "hostess.review_staging_handoff",
            owner: "rusty.hostess",
            instruction_kind: "hostess_handoff_review",
            route_kind: "hostess.review.staging_handoff",
            source: "hostess_staging_handoff_envelope",
            next_required_action: "review_staging_handoff_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingHandoffInstructionSpec {
            instruction_id: "hostess.copy_staging_files",
            owner: "rusty.hostess",
            instruction_kind: "hostess_file_copy_request",
            route_kind: "hostess.stage.files_from_plan",
            source: "hostess_staging_file_plan",
            next_required_action: "copy_stage_files_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingHandoffInstructionSpec {
            instruction_id: "manifold.review_command_session_contract",
            owner: "rusty.manifold",
            instruction_kind: "manifold_contract_review",
            route_kind: "manifold.review.command_session_contract",
            source: "hostess_staging_file_plan",
            next_required_action: "review_command_session_contract_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path.clone(),
        },
        StagingHandoffInstructionSpec {
            instruction_id: "hostess.collect_install_launch_evidence",
            owner: "rusty.hostess",
            instruction_kind: "hostess_evidence_collection_request",
            route_kind: "hostess.collect.install_launch_evidence",
            source: "hostess_staging_file_plan",
            next_required_action: "collect_install_launch_evidence_outside_studio",
            prohibited_in_studio: true,
            expected_input_path: file_plan_path,
        },
    ]
}

fn shell_hostess_staging_handoff_instructions(
    specs: Vec<StagingHandoffInstructionSpec>,
    envelope_status: StudioShellHostessStagingHandoffEnvelopeStatus,
    envelope_issue_code: Option<&str>,
) -> Vec<StudioShellHostessStagingHandoffInstruction> {
    specs
        .into_iter()
        .map(|spec| {
            let status = if envelope_status == StudioShellHostessStagingHandoffEnvelopeStatus::Ready
            {
                StudioShellHostessStagingHandoffInstructionStatus::Ready
            } else {
                StudioShellHostessStagingHandoffInstructionStatus::Blocked
            };
            StudioShellHostessStagingHandoffInstruction {
                instruction_id: spec.instruction_id.to_string(),
                owner: spec.owner.to_string(),
                status,
                issue_code: (status == StudioShellHostessStagingHandoffInstructionStatus::Blocked)
                    .then(|| {
                        envelope_issue_code
                            .unwrap_or("studio.issue.shell_hostess_staging_handoff_blocked")
                            .to_string()
                    }),
                instruction_kind: spec.instruction_kind.to_string(),
                route_kind: spec.route_kind.to_string(),
                source: spec.source.to_string(),
                next_required_action: spec.next_required_action.to_string(),
                prohibited_in_studio: spec.prohibited_in_studio,
                expected_input_path: spec.expected_input_path,
            }
        })
        .collect()
}

fn shell_hostess_staging_handoff_request_summaries(
    file_plan: &StudioShellHostessStagingFilePlan,
) -> Vec<StudioShellHostessStagingHandoffRequestSummary> {
    file_plan
        .requests
        .iter()
        .map(|request| StudioShellHostessStagingHandoffRequestSummary {
            request_id: request.request_id.clone(),
            request_kind: request.request_kind.clone(),
            owner: request.owner.clone(),
            status: request.status,
            target_key: request.target_key.clone(),
            target_kind: request.target_kind,
            graph_id: request.graph_id.clone(),
            consumer_id: request.consumer_id.clone(),
            destination_root: request.destination_root.clone(),
            planned_file_count: request.planned_file_count,
            route_kinds: request.route_kinds.clone(),
            action_ids: request.action_ids.clone(),
        })
        .collect()
}

fn shell_hostess_staging_handoff_provenance(
    file_plan: &StudioShellHostessStagingFilePlan,
) -> StudioShellHostessStagingHandoffProvenance {
    let planned_files = file_plan
        .requests
        .iter()
        .flat_map(|request| request.planned_files.iter());
    let source_artifact_kinds =
        unique_strings(planned_files.clone().map(|file| file.artifact_kind.clone()));
    let source_action_ids = unique_strings(
        file_plan
            .requests
            .iter()
            .flat_map(|request| request.action_ids.iter().cloned()),
    );
    let source_route_kinds = unique_strings(
        file_plan
            .requests
            .iter()
            .flat_map(|request| request.route_kinds.iter().cloned()),
    );
    let target_keys = unique_strings(
        file_plan
            .requests
            .iter()
            .map(|request| request.target_key.clone()),
    );
    let destination_roots = unique_strings(
        file_plan
            .requests
            .iter()
            .map(|request| request.destination_root.clone()),
    );
    StudioShellHostessStagingHandoffProvenance {
        checksum_algorithm: "fnv1a64.studio_staging_file_plan.v1".to_string(),
        plan_checksum: shell_hostess_staging_file_plan_checksum(file_plan),
        source_artifact_kinds,
        source_action_ids,
        source_route_kinds,
        target_keys,
        destination_roots,
    }
}

fn shell_hostess_staging_file_plan_checksum(
    file_plan: &StudioShellHostessStagingFilePlan,
) -> String {
    let mut hasher = Fnv1a64::new();
    hasher.update(&file_plan.schema_id);
    hasher.update(file_plan.project_id.as_deref().unwrap_or(""));
    hasher.update(
        &file_plan
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_default(),
    );
    for request in &file_plan.requests {
        hasher.update(&request.request_id);
        hasher.update(&request.target_key);
        hasher.update(&request.destination_root);
        for file in &request.planned_files {
            hasher.update(&file.artifact_kind);
            hasher.update(&file.source_path);
            hasher.update(&file.destination_path);
            hasher.update(file.graph_id.as_deref().unwrap_or(""));
            hasher.update(file.consumer_id.as_deref().unwrap_or(""));
            for route_hint in &file.route_hints {
                hasher.update(route_hint);
            }
            for action_id in &file.source_action_ids {
                hasher.update(action_id);
            }
            for route_kind in &file.source_route_kinds {
                hasher.update(route_kind);
            }
        }
    }
    format!("{:016x}", hasher.finish())
}

struct Fnv1a64 {
    value: u64,
}

impl Fnv1a64 {
    fn new() -> Self {
        Self {
            value: 0xcbf29ce484222325,
        }
    }

    fn update(&mut self, value: &str) {
        for byte in value.as_bytes().iter().copied().chain([0xff]) {
            self.value ^= u64::from(byte);
            self.value = self.value.wrapping_mul(0x100000001b3);
        }
    }

    fn finish(self) -> u64 {
        self.value
    }
}

fn default_shell_hostess_staging_handoff_envelope_id(
    file_plan: &StudioShellHostessStagingFilePlan,
) -> String {
    format!(
        "studio.hostess_staging_handoff.{}.rev{}",
        file_plan.project_id.as_deref().unwrap_or("unknown_project"),
        file_plan
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string())
    )
}
