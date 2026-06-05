use super::*;

pub fn compare_shell_hostess_staging_acceptance_checklists(
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> StudioShellHostessStagingAcceptanceComparisonReport {
    compare_shell_hostess_staging_acceptance_checklists_with_identity(
        baseline, candidate, None, None,
    )
}

pub fn compare_shell_hostess_staging_acceptance_against_manifest(
    baseline_identity: &StudioShellHostessStagingAcceptanceManifest,
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> StudioShellHostessStagingAcceptanceComparisonReport {
    compare_shell_hostess_staging_acceptance_checklists_with_identity(
        baseline,
        candidate,
        Some(baseline_identity),
        None,
    )
}

pub fn compare_shell_hostess_staging_acceptance_against_index_entry(
    acceptance_index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_index_path: Option<&Path>,
    acceptance_index_entry: &StudioShellHostessStagingAcceptanceIndexEntry,
    acceptance_manifest_path: Option<&Path>,
    baseline_identity: &StudioShellHostessStagingAcceptanceManifest,
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> StudioShellHostessStagingAcceptanceComparisonReport {
    compare_shell_hostess_staging_acceptance_checklists_with_identity(
        baseline,
        candidate,
        Some(baseline_identity),
        Some(ShellHostessStagingAcceptanceIndexComparisonContext {
            index: acceptance_index,
            index_path: acceptance_index_path,
            entry: acceptance_index_entry,
            acceptance_manifest_path,
        }),
    )
}

struct ShellHostessStagingAcceptanceIndexComparisonContext<'a> {
    index: &'a StudioShellHostessStagingAcceptanceIndex,
    index_path: Option<&'a Path>,
    entry: &'a StudioShellHostessStagingAcceptanceIndexEntry,
    acceptance_manifest_path: Option<&'a Path>,
}

fn compare_shell_hostess_staging_acceptance_checklists_with_identity(
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
    baseline_identity: Option<&StudioShellHostessStagingAcceptanceManifest>,
    acceptance_index: Option<ShellHostessStagingAcceptanceIndexComparisonContext<'_>>,
) -> StudioShellHostessStagingAcceptanceComparisonReport {
    let mut checks = shell_hostess_staging_acceptance_comparison_checks(baseline, candidate);
    if let Some(baseline_identity) = baseline_identity {
        checks.extend(shell_hostess_staging_acceptance_baseline_identity_checks(
            baseline_identity,
            baseline,
        ));
        if let Some(acceptance_index) = acceptance_index.as_ref() {
            checks.extend(shell_hostess_staging_acceptance_index_entry_checks(
                acceptance_index,
                baseline_identity,
            ));
        }
    }
    let comparable = checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass);

    let entries = if comparable {
        shell_hostess_staging_acceptance_comparison_entries(baseline, candidate)
    } else {
        Vec::new()
    };
    let has_entry_contract_drift = entries
        .iter()
        .any(|entry| entry.change == StudioShellHostessStagingAcceptanceComparisonChange::Changed);
    if comparable {
        push_check(
            &mut checks,
            "studio.check.shell_hostess_staging_acceptance_comparison.entry_contracts",
            !has_entry_contract_drift,
            "baseline and candidate acceptance entry contracts match",
            "baseline and candidate acceptance entry contracts drifted",
            "studio.issue.shell_hostess_staging_acceptance_entry_drift",
        );
    }
    let comparable = checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass);

    let ready_item_delta = count_delta(candidate.ready_item_count, baseline.ready_item_count);
    let blocked_item_delta = count_delta(candidate.blocked_item_count, baseline.blocked_item_count);
    let rejected_item_delta =
        count_delta(candidate.rejected_item_count, baseline.rejected_item_count);

    let status = if has_entry_contract_drift || !comparable {
        StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable
    } else if shell_hostess_staging_acceptance_status_score(candidate.status)
        < shell_hostess_staging_acceptance_status_score(baseline.status)
        || ready_item_delta < 0
        || blocked_item_delta > 0
        || rejected_item_delta > 0
        || entries.iter().any(|entry| {
            matches!(
                entry.change,
                StudioShellHostessStagingAcceptanceComparisonChange::Regressed
                    | StudioShellHostessStagingAcceptanceComparisonChange::Removed
            )
        })
    {
        StudioShellHostessStagingAcceptanceComparisonStatus::Regressed
    } else if shell_hostess_staging_acceptance_status_score(candidate.status)
        > shell_hostess_staging_acceptance_status_score(baseline.status)
        || ready_item_delta > 0
        || blocked_item_delta < 0
        || rejected_item_delta < 0
        || entries.iter().any(|entry| {
            entry.change == StudioShellHostessStagingAcceptanceComparisonChange::Improved
        })
    {
        StudioShellHostessStagingAcceptanceComparisonStatus::Improved
    } else {
        StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged
    };

    let issue_code = match status {
        StudioShellHostessStagingAcceptanceComparisonStatus::Incomparable => {
            first_failed_validation_check_issue_code(&checks)
        }
        StudioShellHostessStagingAcceptanceComparisonStatus::Regressed => entries
            .iter()
            .find(|entry| {
                matches!(
                    entry.change,
                    StudioShellHostessStagingAcceptanceComparisonChange::Regressed
                        | StudioShellHostessStagingAcceptanceComparisonChange::Removed
                )
            })
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| {
                candidate.issue_code.clone().or_else(|| {
                    Some("studio.issue.shell_hostess_staging_acceptance_regressed".to_string())
                })
            }),
        StudioShellHostessStagingAcceptanceComparisonStatus::Improved
        | StudioShellHostessStagingAcceptanceComparisonStatus::Unchanged => None,
    };

    StudioShellHostessStagingAcceptanceComparisonReport {
        schema_id: SHELL_HOSTESS_STAGING_ACCEPTANCE_COMPARISON_SCHEMA.to_string(),
        baseline_identity_schema: baseline_identity.map(|identity| identity.schema_id.clone()),
        baseline_acceptance_id: baseline_identity.map(|identity| identity.acceptance_id.clone()),
        baseline_label: baseline_identity.map(|identity| identity.label.clone()),
        baseline_checklist_path: baseline_identity.map(|identity| identity.checklist_path.clone()),
        baseline_index_schema: acceptance_index
            .as_ref()
            .map(|context| context.index.schema_id.clone()),
        baseline_index_path: acceptance_index
            .as_ref()
            .and_then(|context| context.index_path.map(|path| path.display().to_string())),
        baseline_index_default_acceptance_id: acceptance_index
            .as_ref()
            .and_then(|context| context.index.default_acceptance_id.clone()),
        baseline_index_selected_acceptance_id: acceptance_index
            .as_ref()
            .map(|context| context.entry.acceptance_id.clone()),
        baseline_schema: baseline.schema_id.clone(),
        candidate_schema: candidate.schema_id.clone(),
        baseline_envelope_id: baseline.envelope_id.clone(),
        candidate_envelope_id: candidate.envelope_id.clone(),
        baseline_manifest_id: baseline.manifest_id.clone(),
        candidate_manifest_id: candidate.manifest_id.clone(),
        baseline_project_id: baseline.project_id.clone(),
        candidate_project_id: candidate.project_id.clone(),
        baseline_project_revision: baseline.project_revision,
        candidate_project_revision: candidate.project_revision,
        baseline_status: baseline.status,
        candidate_status: candidate.status,
        status,
        issue_code,
        baseline_ready_item_count: baseline.ready_item_count,
        candidate_ready_item_count: candidate.ready_item_count,
        ready_item_delta,
        baseline_blocked_item_count: baseline.blocked_item_count,
        candidate_blocked_item_count: candidate.blocked_item_count,
        blocked_item_delta,
        baseline_rejected_item_count: baseline.rejected_item_count,
        candidate_rejected_item_count: candidate.rejected_item_count,
        rejected_item_delta,
        checks,
        entries,
    }
}

fn shell_hostess_staging_acceptance_comparison_checks(
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_schema",
        baseline.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA,
        "baseline Hostess staging acceptance schema id is supported",
        "baseline Hostess staging acceptance schema id is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_checklist_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.candidate_schema",
        candidate.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_CHECKLIST_SCHEMA,
        "candidate Hostess staging acceptance schema id is supported",
        "candidate Hostess staging acceptance schema id is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_checklist_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.source_schema",
        baseline.source_handoff_schema == SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA
            && candidate.source_handoff_schema == SHELL_HOSTESS_STAGING_HANDOFF_ENVELOPE_SCHEMA,
        "baseline and candidate source handoff schemas are supported",
        "baseline or candidate source handoff schema is unsupported",
        "studio.issue.shell_hostess_staging_handoff_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.project_metadata",
        baseline.project_id == candidate.project_id
            && baseline.project_revision == candidate.project_revision
            && baseline.manifest_id == candidate.manifest_id
            && baseline.selected_candidate_id == candidate.selected_candidate_id,
        "baseline and candidate project metadata matches",
        "baseline and candidate project metadata differs",
        "studio.issue.shell_hostess_staging_acceptance_source_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.envelope",
        baseline.envelope_id == candidate.envelope_id,
        "baseline and candidate Hostess staging envelopes match",
        "baseline and candidate Hostess staging envelopes differ",
        "studio.issue.shell_hostess_staging_acceptance_envelope_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.plan_checksum",
        baseline.checksum_algorithm == candidate.checksum_algorithm
            && baseline.plan_checksum == candidate.plan_checksum,
        "baseline and candidate staging file-plan checksums match",
        "baseline and candidate staging file-plan checksums differ",
        "studio.issue.shell_hostess_staging_acceptance_checksum_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.prohibited_actions",
        string_set(&baseline.prohibited_actions) == string_set(&candidate.prohibited_actions),
        "baseline and candidate Studio-prohibited actions match",
        "baseline and candidate Studio-prohibited actions differ",
        "studio.issue.shell_hostess_staging_acceptance_prohibited_actions_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.execution_policy",
        baseline.execution_policy == "not_executed.acceptance_check_only"
            && candidate.execution_policy == "not_executed.acceptance_check_only",
        "baseline and candidate remain acceptance-check-only",
        "baseline or candidate is no longer acceptance-check-only",
        "studio.issue.shell_hostess_staging_acceptance_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.owner_authority",
        baseline.checklist_owner == "rusty.hostess"
            && candidate.checklist_owner == "rusty.hostess"
            && baseline.handoff_owner == "rusty.hostess"
            && candidate.handoff_owner == "rusty.hostess"
            && baseline.staging_owner == "rusty.hostess"
            && candidate.staging_owner == "rusty.hostess",
        "baseline and candidate preserve Hostess ownership",
        "baseline or candidate changed Hostess ownership",
        "studio.issue.shell_hostess_staging_acceptance_owner_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.runtime_authority",
        baseline.command_session_authority.as_deref() == Some("rusty.manifold")
            && candidate.command_session_authority.as_deref() == Some("rusty.manifold")
            && baseline.install_launch_evidence_authority.as_deref() == Some("rusty.hostess")
            && candidate.install_launch_evidence_authority.as_deref() == Some("rusty.hostess")
            && baseline.studio_role.as_deref() == Some("authoring.export_planning")
            && candidate.studio_role.as_deref() == Some("authoring.export_planning"),
        "baseline and candidate preserve Manifold, Hostess, and Studio authority",
        "baseline or candidate changed Manifold, Hostess, or Studio authority",
        "studio.issue.runtime_authority_mismatch",
    );
    checks
}

fn shell_hostess_staging_acceptance_baseline_identity_checks(
    baseline_identity: &StudioShellHostessStagingAcceptanceManifest,
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_schema",
        baseline_identity.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_MANIFEST_SCHEMA,
        "baseline identity schema id is supported",
        "baseline identity schema id is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_checklist_schema",
        baseline_identity.checklist_schema == baseline.schema_id,
        "baseline identity names the loaded checklist schema",
        "baseline identity does not name the loaded checklist schema",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_checklist_path",
        !baseline_identity.checklist_path.trim().is_empty(),
        "baseline identity has a durable checklist path",
        "baseline identity checklist path is missing",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_source_metadata",
        baseline_identity.envelope_id == baseline.envelope_id
            && baseline_identity.manifest_id == baseline.manifest_id
            && baseline_identity.project_id == baseline.project_id
            && baseline_identity.project_revision == baseline.project_revision,
        "baseline identity source metadata matches the loaded checklist",
        "baseline identity source metadata differs from the loaded checklist",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_status_counts",
        baseline_identity.status == baseline.status
            && baseline_identity.issue_code == baseline.issue_code
            && baseline_identity.ready_item_count == baseline.ready_item_count
            && baseline_identity.blocked_item_count == baseline.blocked_item_count
            && baseline_identity.rejected_item_count == baseline.rejected_item_count
            && baseline_identity.request_count == baseline.request_count
            && baseline_identity.instruction_count == baseline.instruction_count,
        "baseline identity readiness counts match the loaded checklist",
        "baseline identity readiness counts differ from the loaded checklist",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_authority",
        baseline_identity.execution_policy == baseline.execution_policy
            && baseline_identity.checklist_owner == baseline.checklist_owner
            && baseline_identity.handoff_owner == baseline.handoff_owner
            && baseline_identity.staging_owner == baseline.staging_owner
            && baseline_identity.command_session_authority == baseline.command_session_authority
            && baseline_identity.install_launch_evidence_authority
                == baseline.install_launch_evidence_authority
            && baseline_identity.studio_role == baseline.studio_role,
        "baseline identity authority fields match the loaded checklist",
        "baseline identity authority fields differ from the loaded checklist",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_identity_provenance",
        baseline_identity.checksum_algorithm == baseline.checksum_algorithm
            && baseline_identity.plan_checksum == baseline.plan_checksum
            && string_set(&baseline_identity.prohibited_actions)
                == string_set(&baseline.prohibited_actions),
        "baseline identity provenance matches the loaded checklist",
        "baseline identity provenance differs from the loaded checklist",
        "studio.issue.shell_hostess_staging_acceptance_identity_mismatch",
    );
    checks
}

fn shell_hostess_staging_acceptance_index_entry_checks(
    context: &ShellHostessStagingAcceptanceIndexComparisonContext<'_>,
    baseline_identity: &StudioShellHostessStagingAcceptanceManifest,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    let entry = context.entry;
    let expected_manifest_path = context
        .acceptance_manifest_path
        .map(|path| path.display().to_string());
    let manifest_path_matches = match (
        expected_manifest_path.as_deref(),
        entry.acceptance_manifest_path.as_deref(),
    ) {
        (Some(expected), Some(actual)) => actual == expected,
        (None, Some(actual)) => !actual.trim().is_empty(),
        _ => false,
    };

    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_schema",
        context.index.schema_id == SHELL_HOSTESS_STAGING_ACCEPTANCE_INDEX_SCHEMA,
        "baseline acceptance index schema id is supported",
        "baseline acceptance index schema id is unsupported",
        "studio.issue.shell_hostess_staging_acceptance_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_selected_acceptance",
        entry.acceptance_id == baseline_identity.acceptance_id,
        "baseline acceptance index selected entry matches the loaded identity",
        "baseline acceptance index selected entry differs from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_manifest_path",
        manifest_path_matches,
        "baseline acceptance index entry manifest path names the loaded identity",
        "baseline acceptance index entry manifest path is missing or stale",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_checklist_path",
        entry.checklist_path == baseline_identity.checklist_path
            && entry.checklist_schema == baseline_identity.checklist_schema,
        "baseline acceptance index checklist references match the loaded identity",
        "baseline acceptance index checklist references differ from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_source_metadata",
        entry.envelope_id == baseline_identity.envelope_id
            && entry.manifest_id == baseline_identity.manifest_id
            && entry.project_id == baseline_identity.project_id
            && entry.project_revision == baseline_identity.project_revision,
        "baseline acceptance index source metadata matches the loaded identity",
        "baseline acceptance index source metadata differs from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_status_counts",
        entry.status == baseline_identity.status
            && entry.issue_code == baseline_identity.issue_code
            && entry.ready_item_count == baseline_identity.ready_item_count
            && entry.blocked_item_count == baseline_identity.blocked_item_count
            && entry.rejected_item_count == baseline_identity.rejected_item_count
            && entry.request_count == baseline_identity.request_count
            && entry.instruction_count == baseline_identity.instruction_count,
        "baseline acceptance index readiness counts match the loaded identity",
        "baseline acceptance index readiness counts differ from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_authority",
        entry.execution_policy == baseline_identity.execution_policy
            && entry.checklist_owner == baseline_identity.checklist_owner
            && entry.handoff_owner == baseline_identity.handoff_owner
            && entry.staging_owner == baseline_identity.staging_owner
            && entry.command_session_authority == baseline_identity.command_session_authority
            && entry.install_launch_evidence_authority
                == baseline_identity.install_launch_evidence_authority
            && entry.studio_role == baseline_identity.studio_role,
        "baseline acceptance index authority fields match the loaded identity",
        "baseline acceptance index authority fields differ from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_acceptance_comparison.baseline_index_provenance",
        entry.checksum_algorithm == baseline_identity.checksum_algorithm
            && entry.plan_checksum == baseline_identity.plan_checksum,
        "baseline acceptance index checksum references match the loaded identity",
        "baseline acceptance index checksum references differ from the loaded identity",
        "studio.issue.shell_hostess_staging_acceptance_index_mismatch",
    );
    checks
}

fn shell_hostess_staging_acceptance_comparison_entries(
    baseline: &StudioShellHostessStagingAcceptanceChecklistReport,
    candidate: &StudioShellHostessStagingAcceptanceChecklistReport,
) -> Vec<StudioShellHostessStagingAcceptanceComparisonEntry> {
    let baseline_entries = baseline
        .entries
        .iter()
        .map(|entry| (entry.item_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let candidate_entries = candidate
        .entries
        .iter()
        .map(|entry| (entry.item_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let item_ids = baseline_entries
        .keys()
        .chain(candidate_entries.keys())
        .map(|item_id| (*item_id).to_string())
        .collect::<BTreeSet<_>>();

    item_ids
        .into_iter()
        .map(|item_id| {
            shell_hostess_staging_acceptance_comparison_entry(
                &item_id,
                baseline_entries.get(item_id.as_str()).copied(),
                candidate_entries.get(item_id.as_str()).copied(),
            )
        })
        .collect()
}

fn shell_hostess_staging_acceptance_comparison_entry(
    item_id: &str,
    baseline: Option<&StudioShellHostessStagingAcceptanceChecklistEntry>,
    candidate: Option<&StudioShellHostessStagingAcceptanceChecklistEntry>,
) -> StudioShellHostessStagingAcceptanceComparisonEntry {
    let baseline_score =
        baseline.map(|entry| shell_hostess_staging_acceptance_status_score(entry.status));
    let candidate_score =
        candidate.map(|entry| shell_hostess_staging_acceptance_status_score(entry.status));
    let score_delta = candidate_score.unwrap_or(0) - baseline_score.unwrap_or(0);
    let change = match (baseline, candidate) {
        (None, Some(_)) => StudioShellHostessStagingAcceptanceComparisonChange::Added,
        (Some(_), None) => StudioShellHostessStagingAcceptanceComparisonChange::Removed,
        (Some(_), Some(_)) if score_delta > 0 => {
            StudioShellHostessStagingAcceptanceComparisonChange::Improved
        }
        (Some(_), Some(_)) if score_delta < 0 => {
            StudioShellHostessStagingAcceptanceComparisonChange::Regressed
        }
        (Some(baseline), Some(candidate))
            if baseline.owner != candidate.owner
                || baseline.route_kind != candidate.route_kind
                || baseline.issue_code != candidate.issue_code
                || baseline.prohibited_in_studio != candidate.prohibited_in_studio
                || baseline.expected_input_path != candidate.expected_input_path =>
        {
            StudioShellHostessStagingAcceptanceComparisonChange::Changed
        }
        (Some(_), Some(_)) => StudioShellHostessStagingAcceptanceComparisonChange::Unchanged,
        (None, None) => StudioShellHostessStagingAcceptanceComparisonChange::Unchanged,
    };
    let issue_code = match change {
        StudioShellHostessStagingAcceptanceComparisonChange::Regressed
        | StudioShellHostessStagingAcceptanceComparisonChange::Removed => candidate
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| baseline.and_then(|entry| entry.issue_code.clone()))
            .or_else(|| {
                Some("studio.issue.shell_hostess_staging_acceptance_regressed".to_string())
            }),
        StudioShellHostessStagingAcceptanceComparisonChange::Added
        | StudioShellHostessStagingAcceptanceComparisonChange::Improved
        | StudioShellHostessStagingAcceptanceComparisonChange::Unchanged => None,
        StudioShellHostessStagingAcceptanceComparisonChange::Changed => {
            Some("studio.issue.shell_hostess_staging_acceptance_entry_drift".to_string())
        }
    };

    StudioShellHostessStagingAcceptanceComparisonEntry {
        item_id: item_id.to_string(),
        owner: candidate
            .map(|entry| entry.owner.clone())
            .or_else(|| baseline.map(|entry| entry.owner.clone()))
            .unwrap_or_else(|| "unknown".to_string()),
        baseline_status: baseline.map(|entry| entry.status),
        candidate_status: candidate.map(|entry| entry.status),
        change,
        score_delta,
        baseline_route_kind: baseline.map(|entry| entry.route_kind.clone()),
        candidate_route_kind: candidate.map(|entry| entry.route_kind.clone()),
        baseline_issue_code: baseline.and_then(|entry| entry.issue_code.clone()),
        candidate_issue_code: candidate.and_then(|entry| entry.issue_code.clone()),
        issue_code,
    }
}

fn shell_hostess_staging_acceptance_status_score(
    status: StudioShellHostessStagingAcceptanceStatus,
) -> isize {
    match status {
        StudioShellHostessStagingAcceptanceStatus::Rejected => 0,
        StudioShellHostessStagingAcceptanceStatus::Blocked => 1,
        StudioShellHostessStagingAcceptanceStatus::Ready => 2,
    }
}
