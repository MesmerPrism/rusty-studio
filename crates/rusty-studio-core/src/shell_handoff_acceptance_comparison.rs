use super::*;

pub fn compare_shell_handoff_acceptance_checklists(
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> StudioShellHandoffAcceptanceComparisonReport {
    compare_shell_handoff_acceptance_checklists_with_identity(baseline, candidate, None, None)
}

pub fn compare_shell_handoff_acceptance_against_baseline_manifest(
    baseline_identity: &StudioShellHandoffAcceptanceBaselineManifest,
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> StudioShellHandoffAcceptanceComparisonReport {
    compare_shell_handoff_acceptance_checklists_with_identity(
        baseline,
        candidate,
        Some(baseline_identity),
        None,
    )
}

pub fn compare_shell_handoff_acceptance_against_baseline_index_entry(
    baseline_index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_index_path: Option<&Path>,
    baseline_index_entry: &StudioShellHandoffAcceptanceBaselineIndexEntry,
    baseline_manifest_path: Option<&Path>,
    baseline_identity: &StudioShellHandoffAcceptanceBaselineManifest,
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> StudioShellHandoffAcceptanceComparisonReport {
    compare_shell_handoff_acceptance_checklists_with_identity(
        baseline,
        candidate,
        Some(baseline_identity),
        Some(ShellHandoffAcceptanceBaselineIndexComparisonContext {
            index: baseline_index,
            index_path: baseline_index_path,
            entry: baseline_index_entry,
            baseline_manifest_path,
        }),
    )
}

struct ShellHandoffAcceptanceBaselineIndexComparisonContext<'a> {
    index: &'a StudioShellHandoffAcceptanceBaselineIndex,
    index_path: Option<&'a Path>,
    entry: &'a StudioShellHandoffAcceptanceBaselineIndexEntry,
    baseline_manifest_path: Option<&'a Path>,
}

fn compare_shell_handoff_acceptance_checklists_with_identity(
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
    baseline_identity: Option<&StudioShellHandoffAcceptanceBaselineManifest>,
    baseline_index: Option<ShellHandoffAcceptanceBaselineIndexComparisonContext<'_>>,
) -> StudioShellHandoffAcceptanceComparisonReport {
    let mut checks = shell_handoff_acceptance_comparison_checks(baseline, candidate);
    if let Some(baseline_identity) = baseline_identity {
        checks.extend(shell_handoff_acceptance_baseline_identity_checks(
            baseline_identity,
            baseline,
        ));
        if let Some(baseline_index) = baseline_index.as_ref() {
            checks.extend(shell_handoff_acceptance_baseline_index_entry_checks(
                baseline_index,
                baseline_identity,
            ));
        }
    }
    let comparable = checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass);

    let entries = if comparable {
        shell_handoff_acceptance_comparison_entries(baseline, candidate)
    } else {
        Vec::new()
    };

    let ready_delta = count_delta(candidate.ready_count, baseline.ready_count);
    let blocked_delta = count_delta(candidate.blocked_count, baseline.blocked_count);
    let rejected_delta = count_delta(candidate.rejected_count, baseline.rejected_count);

    let status = if !comparable {
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable
    } else if acceptance_status_score(candidate.status) < acceptance_status_score(baseline.status)
        || ready_delta < 0
        || blocked_delta > 0
        || rejected_delta > 0
        || entries
            .iter()
            .any(|entry| entry.change == StudioShellHandoffAcceptanceComparisonChange::Regressed)
    {
        StudioShellHandoffAcceptanceComparisonStatus::Regressed
    } else if acceptance_status_score(candidate.status) > acceptance_status_score(baseline.status)
        || ready_delta > 0
        || blocked_delta < 0
        || rejected_delta < 0
        || entries
            .iter()
            .any(|entry| entry.change == StudioShellHandoffAcceptanceComparisonChange::Improved)
    {
        StudioShellHandoffAcceptanceComparisonStatus::Improved
    } else {
        StudioShellHandoffAcceptanceComparisonStatus::Unchanged
    };

    let issue_code = match status {
        StudioShellHandoffAcceptanceComparisonStatus::Incomparable => {
            first_failed_validation_check_issue_code(&checks)
        }
        StudioShellHandoffAcceptanceComparisonStatus::Regressed => entries
            .iter()
            .find(|entry| entry.change == StudioShellHandoffAcceptanceComparisonChange::Regressed)
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| Some("studio.issue.shell_handoff_acceptance_regressed".to_string())),
        StudioShellHandoffAcceptanceComparisonStatus::Improved
        | StudioShellHandoffAcceptanceComparisonStatus::Unchanged => None,
    };

    StudioShellHandoffAcceptanceComparisonReport {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_COMPARISON_SCHEMA.to_string(),
        baseline_identity_schema: baseline_identity.map(|identity| identity.schema_id.clone()),
        baseline_id: baseline_identity.map(|identity| identity.baseline_id.clone()),
        baseline_label: baseline_identity.map(|identity| identity.label.clone()),
        baseline_checklist_path: baseline_identity.map(|identity| identity.checklist_path.clone()),
        baseline_index_schema: baseline_index
            .as_ref()
            .map(|context| context.index.schema_id.clone()),
        baseline_index_path: baseline_index
            .as_ref()
            .and_then(|context| context.index_path.map(|path| path.display().to_string())),
        baseline_index_default_baseline_id: baseline_index
            .as_ref()
            .and_then(|context| context.index.default_baseline_id.clone()),
        baseline_index_selected_baseline_id: baseline_index
            .as_ref()
            .map(|context| context.entry.baseline_id.clone()),
        baseline_schema: baseline.schema_id.clone(),
        candidate_schema: candidate.schema_id.clone(),
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
        baseline_ready_count: baseline.ready_count,
        candidate_ready_count: candidate.ready_count,
        ready_delta,
        baseline_blocked_count: baseline.blocked_count,
        candidate_blocked_count: candidate.blocked_count,
        blocked_delta,
        baseline_rejected_count: baseline.rejected_count,
        candidate_rejected_count: candidate.rejected_count,
        rejected_delta,
        checks,
        entries,
    }
}

fn shell_handoff_acceptance_comparison_checks(
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_schema",
        baseline.schema_id == SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA,
        "baseline checklist schema id is supported",
        "baseline checklist schema id is unsupported",
        "studio.issue.shell_handoff_acceptance_checklist_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.candidate_schema",
        candidate.schema_id == SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA,
        "candidate checklist schema id is supported",
        "candidate checklist schema id is unsupported",
        "studio.issue.shell_handoff_acceptance_checklist_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_source_schema",
        baseline.source_intake_schema == SHELL_HANDOFF_INTAKE_REPORT_SCHEMA,
        "baseline source intake schema id is supported",
        "baseline source intake schema id is unsupported",
        "studio.issue.shell_handoff_intake_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.candidate_source_schema",
        candidate.source_intake_schema == SHELL_HANDOFF_INTAKE_REPORT_SCHEMA,
        "candidate source intake schema id is supported",
        "candidate source intake schema id is unsupported",
        "studio.issue.shell_handoff_intake_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.project_id",
        baseline.project_id == candidate.project_id,
        "baseline and candidate project ids match",
        "baseline and candidate project ids differ",
        "studio.issue.shell_handoff_acceptance_project_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.prohibited_actions",
        string_set(&baseline.prohibited_actions) == string_set(&candidate.prohibited_actions),
        "baseline and candidate prohibited actions match",
        "baseline and candidate prohibited actions differ",
        "studio.issue.shell_handoff_acceptance_prohibited_actions_mismatch",
    );
    checks
}

fn shell_handoff_acceptance_baseline_identity_checks(
    baseline_identity: &StudioShellHandoffAcceptanceBaselineManifest,
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_schema",
        baseline_identity.schema_id == SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA,
        "baseline identity schema id is supported",
        "baseline identity schema id is unsupported",
        "studio.issue.shell_handoff_acceptance_baseline_identity_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_summary_schema",
        baseline_identity.summary.schema_id == SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA,
        "baseline identity summary schema id is supported",
        "baseline identity summary schema id is unsupported",
        "studio.issue.shell_handoff_acceptance_summary_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_checklist_schema",
        baseline_identity.summary.checklist_schema == baseline.schema_id,
        "baseline identity summary names the loaded checklist schema",
        "baseline identity summary does not name the loaded checklist schema",
        "studio.issue.shell_handoff_acceptance_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_manifest",
        baseline_identity.summary.manifest_id == baseline.manifest_id,
        "baseline identity manifest id matches the loaded checklist",
        "baseline identity manifest id differs from the loaded checklist",
        "studio.issue.shell_handoff_acceptance_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_project",
        baseline_identity.summary.project_id == baseline.project_id
            && baseline_identity.summary.project_revision == baseline.project_revision,
        "baseline identity project metadata matches the loaded checklist",
        "baseline identity project metadata differs from the loaded checklist",
        "studio.issue.shell_handoff_acceptance_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_identity_status_counts",
        baseline_identity.summary.status == baseline.status
            && baseline_identity.summary.ready_count == baseline.ready_count
            && baseline_identity.summary.blocked_count == baseline.blocked_count
            && baseline_identity.summary.rejected_count == baseline.rejected_count
            && baseline_identity.summary.entry_count == baseline.entries.len(),
        "baseline identity readiness counts match the loaded checklist",
        "baseline identity readiness counts differ from the loaded checklist",
        "studio.issue.shell_handoff_acceptance_baseline_identity_mismatch",
    );
    checks
}

fn shell_handoff_acceptance_baseline_index_entry_checks(
    context: &ShellHandoffAcceptanceBaselineIndexComparisonContext<'_>,
    baseline_identity: &StudioShellHandoffAcceptanceBaselineManifest,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    let entry = context.entry;
    let summary = &baseline_identity.summary;
    let expected_manifest_path = context
        .baseline_manifest_path
        .map(|path| path.display().to_string());
    let manifest_path_matches = match (
        expected_manifest_path.as_deref(),
        entry.baseline_manifest_path.as_deref(),
    ) {
        (Some(expected), Some(actual)) => actual == expected,
        (None, Some(actual)) => !actual.trim().is_empty(),
        _ => false,
    };

    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_schema",
        context.index.schema_id == SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA,
        "baseline index schema id is supported",
        "baseline index schema id is unsupported",
        "studio.issue.shell_handoff_acceptance_baseline_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_selected_baseline",
        entry.baseline_id == baseline_identity.baseline_id,
        "baseline index selected entry matches the loaded baseline identity",
        "baseline index selected entry differs from the loaded baseline identity",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_manifest_path",
        manifest_path_matches,
        "baseline index entry manifest path names the loaded baseline identity",
        "baseline index entry manifest path is missing or stale",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_checklist_path",
        entry.checklist_path == baseline_identity.checklist_path,
        "baseline index entry checklist path matches the loaded baseline identity",
        "baseline index entry checklist path differs from the loaded baseline identity",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_summary_schema",
        entry.summary_schema == summary.schema_id
            && entry.checklist_schema == summary.checklist_schema,
        "baseline index entry schema references match the loaded baseline identity",
        "baseline index entry schema references differ from the loaded baseline identity",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_source_metadata",
        entry.manifest_id == summary.manifest_id
            && entry.project_id == summary.project_id
            && entry.project_revision == summary.project_revision,
        "baseline index entry source metadata matches the loaded baseline identity",
        "baseline index entry source metadata differs from the loaded baseline identity",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance_comparison.baseline_index_status_counts",
        entry.status == summary.status
            && entry.issue_code == summary.issue_code
            && entry.ready_count == summary.ready_count
            && entry.blocked_count == summary.blocked_count
            && entry.rejected_count == summary.rejected_count
            && entry.entry_count == summary.entry_count
            && entry.target_count == summary.targets.len(),
        "baseline index entry readiness counts match the loaded baseline identity",
        "baseline index entry readiness counts differ from the loaded baseline identity",
        "studio.issue.shell_handoff_acceptance_baseline_index_mismatch",
    );
    checks
}

fn shell_handoff_acceptance_comparison_entries(
    baseline: &StudioShellHandoffAcceptanceChecklistReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> Vec<StudioShellHandoffAcceptanceComparisonEntry> {
    let baseline_entries = baseline
        .entries
        .iter()
        .map(|entry| (entry.graph_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let candidate_entries = candidate
        .entries
        .iter()
        .map(|entry| (entry.graph_id.as_str(), entry))
        .collect::<BTreeMap<_, _>>();
    let graph_ids = baseline_entries
        .keys()
        .chain(candidate_entries.keys())
        .map(|graph_id| (*graph_id).to_string())
        .collect::<BTreeSet<_>>();

    graph_ids
        .into_iter()
        .map(|graph_id| {
            shell_handoff_acceptance_comparison_entry(
                &graph_id,
                baseline_entries.get(graph_id.as_str()).copied(),
                candidate_entries.get(graph_id.as_str()).copied(),
            )
        })
        .collect()
}

fn shell_handoff_acceptance_comparison_entry(
    graph_id: &str,
    baseline: Option<&StudioShellHandoffAcceptanceChecklistEntry>,
    candidate: Option<&StudioShellHandoffAcceptanceChecklistEntry>,
) -> StudioShellHandoffAcceptanceComparisonEntry {
    let baseline_score = baseline.map(|entry| acceptance_status_score(entry.status));
    let candidate_score = candidate.map(|entry| acceptance_status_score(entry.status));
    let score_delta = candidate_score.unwrap_or(0) - baseline_score.unwrap_or(0);
    let change = match (baseline, candidate) {
        (None, Some(_)) => StudioShellHandoffAcceptanceComparisonChange::Added,
        (Some(_), None) => StudioShellHandoffAcceptanceComparisonChange::Removed,
        (Some(_), Some(_)) if score_delta > 0 => {
            StudioShellHandoffAcceptanceComparisonChange::Improved
        }
        (Some(_), Some(_)) if score_delta < 0 => {
            StudioShellHandoffAcceptanceComparisonChange::Regressed
        }
        (Some(baseline), Some(candidate))
            if baseline.consumer_id != candidate.consumer_id
                || baseline.runtime_route_kind != candidate.runtime_route_kind
                || baseline.issue_code != candidate.issue_code =>
        {
            StudioShellHandoffAcceptanceComparisonChange::Changed
        }
        (Some(_), Some(_)) => StudioShellHandoffAcceptanceComparisonChange::Unchanged,
        (None, None) => StudioShellHandoffAcceptanceComparisonChange::Unchanged,
    };
    let issue_code = match change {
        StudioShellHandoffAcceptanceComparisonChange::Regressed
        | StudioShellHandoffAcceptanceComparisonChange::Removed => candidate
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| baseline.and_then(|entry| entry.issue_code.clone()))
            .or_else(|| Some("studio.issue.shell_handoff_acceptance_regressed".to_string())),
        StudioShellHandoffAcceptanceComparisonChange::Added
        | StudioShellHandoffAcceptanceComparisonChange::Improved
        | StudioShellHandoffAcceptanceComparisonChange::Unchanged
        | StudioShellHandoffAcceptanceComparisonChange::Changed => None,
    };

    StudioShellHandoffAcceptanceComparisonEntry {
        graph_id: graph_id.to_string(),
        target_kind: candidate
            .map(|entry| entry.target_kind)
            .or_else(|| baseline.map(|entry| entry.target_kind)),
        baseline_status: baseline.map(|entry| entry.status),
        candidate_status: candidate.map(|entry| entry.status),
        change,
        score_delta,
        baseline_consumer_id: baseline.map(|entry| entry.consumer_id.clone()),
        candidate_consumer_id: candidate.map(|entry| entry.consumer_id.clone()),
        baseline_route_kind: baseline.map(|entry| entry.runtime_route_kind.clone()),
        candidate_route_kind: candidate.map(|entry| entry.runtime_route_kind.clone()),
        baseline_issue_code: baseline.and_then(|entry| entry.issue_code.clone()),
        candidate_issue_code: candidate.and_then(|entry| entry.issue_code.clone()),
        issue_code,
    }
}

fn acceptance_status_score(status: StudioShellHandoffAcceptanceStatus) -> isize {
    match status {
        StudioShellHandoffAcceptanceStatus::Rejected => 0,
        StudioShellHandoffAcceptanceStatus::Blocked => 1,
        StudioShellHandoffAcceptanceStatus::Ready => 2,
    }
}
