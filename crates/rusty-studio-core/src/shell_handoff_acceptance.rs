use super::*;

pub fn shell_handoff_acceptance_checklist_for_intake(
    intake: &StudioShellHandoffIntakeReport,
) -> StudioShellHandoffAcceptanceChecklistReport {
    let intake_checks = shell_handoff_acceptance_intake_checks(intake);
    let intake_is_accepted = intake_checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass);

    let entries = if intake_is_accepted {
        intake
            .entries
            .iter()
            .map(shell_handoff_acceptance_checklist_entry)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    let ready_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Ready)
        .count();
    let blocked_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Blocked)
        .count();
    let rejected_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Rejected)
        .count();
    let status = if !intake_is_accepted || rejected_count > 0 {
        StudioShellHandoffAcceptanceStatus::Rejected
    } else if blocked_count > 0 {
        StudioShellHandoffAcceptanceStatus::Blocked
    } else {
        StudioShellHandoffAcceptanceStatus::Ready
    };
    let issue_code = match status {
        StudioShellHandoffAcceptanceStatus::Ready => None,
        StudioShellHandoffAcceptanceStatus::Blocked => entries
            .iter()
            .find(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Blocked)
            .and_then(|entry| entry.issue_code.clone()),
        StudioShellHandoffAcceptanceStatus::Rejected => intake.issue_code.clone().or_else(|| {
            first_failed_validation_check_issue_code(&intake_checks).or_else(|| {
                entries
                    .iter()
                    .find(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Rejected)
                    .and_then(|entry| entry.issue_code.clone())
            })
        }),
    };

    StudioShellHandoffAcceptanceChecklistReport {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_CHECKLIST_SCHEMA.to_string(),
        source_intake_schema: intake.schema_id.clone(),
        manifest_id: intake.manifest_id.clone(),
        project_id: intake.project_id.clone(),
        project_revision: intake.project_revision,
        status,
        issue_code,
        prohibited_actions: shell_handoff_acceptance_prohibited_actions(),
        ready_count,
        blocked_count,
        rejected_count,
        intake_checks,
        entries,
    }
}

pub fn shell_handoff_acceptance_checklist_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
    bundle_root: &Path,
) -> StudioShellHandoffAcceptanceChecklistReport {
    let manifest = shell_handoff_manifest_for_project(project, base_dir, bundle_root);
    let intake = shell_handoff_intake_for_manifest(&manifest);
    shell_handoff_acceptance_checklist_for_intake(&intake)
}

pub fn summarize_shell_handoff_acceptance_checklist(
    checklist: &StudioShellHandoffAcceptanceChecklistReport,
    checklist_path: Option<&Path>,
) -> StudioShellHandoffAcceptanceSummaryReport {
    let failed_intake_check_count = checklist
        .intake_checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();

    StudioShellHandoffAcceptanceSummaryReport {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_SUMMARY_SCHEMA.to_string(),
        checklist_schema: checklist.schema_id.clone(),
        checklist_path: checklist_path.map(|path| path.display().to_string()),
        manifest_id: checklist.manifest_id.clone(),
        project_id: checklist.project_id.clone(),
        project_revision: checklist.project_revision,
        status: checklist.status,
        issue_code: checklist.issue_code.clone(),
        ready_count: checklist.ready_count,
        blocked_count: checklist.blocked_count,
        rejected_count: checklist.rejected_count,
        entry_count: checklist.entries.len(),
        intake_check_count: checklist.intake_checks.len(),
        failed_intake_check_count,
        prohibited_actions: checklist.prohibited_actions.clone(),
        targets: shell_handoff_acceptance_target_summaries(&checklist.entries),
    }
}

pub fn shell_handoff_acceptance_baseline_manifest_for_checklist(
    checklist: &StudioShellHandoffAcceptanceChecklistReport,
    checklist_path: &Path,
    baseline_id: Option<&str>,
    label: Option<&str>,
) -> StudioShellHandoffAcceptanceBaselineManifest {
    let summary = summarize_shell_handoff_acceptance_checklist(checklist, Some(checklist_path));
    let baseline_id = baseline_id
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_handoff_acceptance_baseline_id(&summary));
    let label = label
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_handoff_acceptance_baseline_label(&summary));

    StudioShellHandoffAcceptanceBaselineManifest {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_BASELINE_MANIFEST_SCHEMA.to_string(),
        baseline_id,
        label,
        checklist_path: checklist_path.display().to_string(),
        summary,
    }
}

pub fn shell_handoff_acceptance_baseline_index_for_manifests(
    baselines: Vec<(
        StudioShellHandoffAcceptanceBaselineManifest,
        Option<PathBuf>,
    )>,
    default_baseline_id: Option<&str>,
) -> StudioShellHandoffAcceptanceBaselineIndex {
    let entries = baselines
        .into_iter()
        .map(|(baseline, baseline_manifest_path)| {
            shell_handoff_acceptance_baseline_index_entry_for_manifest(
                baseline,
                baseline_manifest_path,
            )
        })
        .collect::<Vec<_>>();

    shell_handoff_acceptance_baseline_index_for_entries(entries, default_baseline_id)
}

pub fn append_shell_handoff_acceptance_baseline_index_manifests(
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baselines: Vec<(
        StudioShellHandoffAcceptanceBaselineManifest,
        Option<PathBuf>,
    )>,
    default_baseline_id: Option<&str>,
) -> StudioShellHandoffAcceptanceBaselineIndex {
    let entries = index
        .entries
        .iter()
        .cloned()
        .chain(
            baselines
                .into_iter()
                .map(|(baseline, baseline_manifest_path)| {
                    shell_handoff_acceptance_baseline_index_entry_for_manifest(
                        baseline,
                        baseline_manifest_path,
                    )
                }),
        )
        .collect::<Vec<_>>();
    let default_baseline_id = default_baseline_id.or(index.default_baseline_id.as_deref());

    shell_handoff_acceptance_baseline_index_for_entries(entries, default_baseline_id)
}

pub fn promote_shell_handoff_acceptance_baseline_index_default(
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_id: &str,
) -> Option<StudioShellHandoffAcceptanceBaselineIndex> {
    index
        .entries
        .iter()
        .any(|entry| entry.baseline_id == baseline_id)
        .then(|| {
            shell_handoff_acceptance_baseline_index_for_entries(
                index.entries.clone(),
                Some(baseline_id),
            )
        })
}

fn shell_handoff_acceptance_baseline_index_entry_for_manifest(
    baseline: StudioShellHandoffAcceptanceBaselineManifest,
    baseline_manifest_path: Option<PathBuf>,
) -> StudioShellHandoffAcceptanceBaselineIndexEntry {
    let StudioShellHandoffAcceptanceBaselineManifest {
        baseline_id,
        label,
        checklist_path,
        summary,
        ..
    } = baseline;

    StudioShellHandoffAcceptanceBaselineIndexEntry {
        baseline_id,
        label,
        baseline_manifest_path: baseline_manifest_path.map(|path| path.display().to_string()),
        checklist_path,
        summary_schema: summary.schema_id.clone(),
        checklist_schema: summary.checklist_schema.clone(),
        manifest_id: summary.manifest_id.clone(),
        project_id: summary.project_id.clone(),
        project_revision: summary.project_revision,
        status: summary.status,
        issue_code: summary.issue_code.clone(),
        ready_count: summary.ready_count,
        blocked_count: summary.blocked_count,
        rejected_count: summary.rejected_count,
        entry_count: summary.entry_count,
        target_count: summary.targets.len(),
    }
}

fn shell_handoff_acceptance_baseline_index_for_entries(
    entries: Vec<StudioShellHandoffAcceptanceBaselineIndexEntry>,
    default_baseline_id: Option<&str>,
) -> StudioShellHandoffAcceptanceBaselineIndex {
    let mut by_id = BTreeMap::new();
    for entry in entries {
        by_id.insert(entry.baseline_id.clone(), entry);
    }

    let entries = by_id.into_values().collect::<Vec<_>>();
    let default_baseline_id = default_baseline_id
        .filter(|baseline_id| {
            entries
                .iter()
                .any(|entry| entry.baseline_id == *baseline_id)
        })
        .map(str::to_string)
        .or_else(|| entries.first().map(|entry| entry.baseline_id.clone()));

    StudioShellHandoffAcceptanceBaselineIndex {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_BASELINE_INDEX_SCHEMA.to_string(),
        project_ids: unique_strings(entries.iter().map(|entry| entry.project_id.clone())),
        manifest_ids: unique_strings(entries.iter().map(|entry| entry.manifest_id.clone())),
        default_baseline_id,
        baseline_count: entries.len(),
        ready_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Ready)
            .count(),
        blocked_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Blocked)
            .count(),
        rejected_baseline_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Rejected)
            .count(),
        entries,
    }
}

pub fn select_shell_handoff_acceptance_baseline_index_entry<'a>(
    index: &'a StudioShellHandoffAcceptanceBaselineIndex,
    baseline_id: Option<&str>,
) -> Option<&'a StudioShellHandoffAcceptanceBaselineIndexEntry> {
    let selected_id = baseline_id.or(index.default_baseline_id.as_deref());
    selected_id
        .and_then(|selected_id| {
            index
                .entries
                .iter()
                .find(|entry| entry.baseline_id == selected_id)
        })
        .or_else(|| {
            baseline_id
                .is_none()
                .then(|| index.entries.first())
                .flatten()
        })
}

pub fn summarize_shell_handoff_acceptance_baseline_index_selection(
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    index_path: Option<&Path>,
    requested_baseline_id: Option<&str>,
) -> StudioShellHandoffAcceptanceBaselineSelectionReport {
    let selected_entry =
        select_shell_handoff_acceptance_baseline_index_entry(index, requested_baseline_id);
    let selected_baseline_id = selected_entry.map(|entry| entry.baseline_id.clone());
    let status = if index.entries.is_empty() {
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Empty
    } else if selected_entry.is_some() {
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
    } else {
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Missing
    };
    let issue_code = match status {
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected => None,
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Missing => {
            Some("studio.issue.shell_handoff_acceptance_baseline_not_found".to_string())
        }
        StudioShellHandoffAcceptanceBaselineSelectionStatus::Empty => {
            Some("studio.issue.shell_handoff_acceptance_baseline_index_empty".to_string())
        }
    };

    StudioShellHandoffAcceptanceBaselineSelectionReport {
        schema_id: SHELL_HANDOFF_ACCEPTANCE_BASELINE_SELECTION_SCHEMA.to_string(),
        source_index_schema: index.schema_id.clone(),
        index_path: index_path.map(|path| path.display().to_string()),
        requested_baseline_id: requested_baseline_id.map(str::to_string),
        default_baseline_id: index.default_baseline_id.clone(),
        selected_baseline_id: selected_baseline_id.clone(),
        status,
        issue_code,
        baseline_count: index.baseline_count,
        ready_baseline_count: index.ready_baseline_count,
        blocked_baseline_count: index.blocked_baseline_count,
        rejected_baseline_count: index.rejected_baseline_count,
        project_ids: index.project_ids.clone(),
        manifest_ids: index.manifest_ids.clone(),
        entries: index
            .entries
            .iter()
            .map(|entry| StudioShellHandoffAcceptanceBaselineSelectionEntry {
                baseline_id: entry.baseline_id.clone(),
                label: entry.label.clone(),
                selected: selected_baseline_id.as_deref() == Some(entry.baseline_id.as_str()),
                default: index.default_baseline_id.as_deref() == Some(entry.baseline_id.as_str()),
                baseline_manifest_path: entry.baseline_manifest_path.clone(),
                checklist_path: entry.checklist_path.clone(),
                project_id: entry.project_id.clone(),
                project_revision: entry.project_revision,
                status: entry.status,
                issue_code: entry.issue_code.clone(),
                ready_count: entry.ready_count,
                blocked_count: entry.blocked_count,
                rejected_count: entry.rejected_count,
                entry_count: entry.entry_count,
                target_count: entry.target_count,
            })
            .collect(),
    }
}

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

fn shell_handoff_acceptance_target_summaries(
    entries: &[StudioShellHandoffAcceptanceChecklistEntry],
) -> Vec<StudioShellHandoffAcceptanceTargetSummary> {
    shell_target_kinds()
        .iter()
        .filter_map(|target_kind| shell_handoff_acceptance_target_summary(entries, *target_kind))
        .collect()
}

fn default_shell_handoff_acceptance_baseline_id(
    summary: &StudioShellHandoffAcceptanceSummaryReport,
) -> String {
    format!(
        "{}.rev{}.{}",
        summary.project_id,
        summary.project_revision,
        shell_handoff_acceptance_status_key(summary.status)
    )
}

fn default_shell_handoff_acceptance_baseline_label(
    summary: &StudioShellHandoffAcceptanceSummaryReport,
) -> String {
    format!(
        "{} revision {} {} acceptance baseline",
        summary.project_id,
        summary.project_revision,
        shell_handoff_acceptance_status_key(summary.status)
    )
}

fn shell_handoff_acceptance_status_key(status: StudioShellHandoffAcceptanceStatus) -> &'static str {
    match status {
        StudioShellHandoffAcceptanceStatus::Ready => "ready",
        StudioShellHandoffAcceptanceStatus::Blocked => "blocked",
        StudioShellHandoffAcceptanceStatus::Rejected => "rejected",
    }
}

fn shell_handoff_acceptance_target_summary(
    entries: &[StudioShellHandoffAcceptanceChecklistEntry],
    target_kind: StudioShellTargetKind,
) -> Option<StudioShellHandoffAcceptanceTargetSummary> {
    let target_entries = entries
        .iter()
        .filter(|entry| entry.target_kind == target_kind)
        .collect::<Vec<_>>();
    if target_entries.is_empty() {
        return None;
    }

    Some(StudioShellHandoffAcceptanceTargetSummary {
        target_kind,
        graph_count: target_entries.len(),
        ready_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Ready)
            .count(),
        blocked_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Blocked)
            .count(),
        rejected_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellHandoffAcceptanceStatus::Rejected)
            .count(),
        graph_ids: unique_strings(target_entries.iter().map(|entry| entry.graph_id.clone())),
        consumer_ids: unique_strings(target_entries.iter().map(|entry| entry.consumer_id.clone())),
        route_kinds: unique_strings(
            target_entries
                .iter()
                .map(|entry| entry.runtime_route_kind.clone()),
        ),
        issue_codes: unique_strings(
            target_entries
                .iter()
                .filter_map(|entry| entry.issue_code.clone()),
        ),
    })
}

fn shell_handoff_acceptance_intake_checks(
    intake: &StudioShellHandoffIntakeReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance.intake_schema",
        intake.schema_id == SHELL_HANDOFF_INTAKE_REPORT_SCHEMA,
        "source intake schema id is supported",
        "source intake schema id is unsupported",
        "studio.issue.shell_handoff_intake_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance.intake_status",
        intake.status == StudioShellHandoffIntakeStatus::Accepted,
        "source intake was accepted",
        "source intake was rejected",
        "studio.issue.shell_handoff_intake_rejected",
    );
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance.intake_validation",
        intake.validation.status == StudioValidationStatus::Pass,
        "source intake validation passed",
        "source intake validation failed",
        "studio.issue.shell_handoff_intake_validation_failed",
    );
    let authority = StudioShellRuntimeAuthority {
        command_session_authority: intake.command_session_authority.clone(),
        install_launch_evidence_authority: intake.install_launch_evidence_authority.clone(),
        studio_role: intake.studio_role.clone(),
    };
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance.authority",
        runtime_authority_matches(&authority),
        "source intake preserves Manifold/Hostess/Studio authority boundaries",
        "source intake authority does not preserve Manifold/Hostess/Studio boundaries",
        "studio.issue.runtime_authority_mismatch",
    );
    let accepted_count = intake
        .entries
        .iter()
        .filter(|entry| entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner)
        .count();
    let blocked_count = intake.entries.len().saturating_sub(accepted_count);
    push_check(
        &mut checks,
        "studio.check.shell_handoff_acceptance.counts",
        intake.accepted_count == accepted_count
            && intake.blocked_count == blocked_count
            && intake.entries.len() == intake.accepted_count + intake.blocked_count,
        "source intake counts match entry decisions",
        "source intake counts do not match entry decisions",
        "studio.issue.shell_handoff_intake_count_mismatch",
    );
    checks
}

fn shell_handoff_acceptance_checklist_entry(
    entry: &StudioShellHandoffIntakeEntry,
) -> StudioShellHandoffAcceptanceChecklistEntry {
    let checks = shell_handoff_acceptance_entry_checks(entry);
    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let status = if entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner
        && !has_failed_check
    {
        StudioShellHandoffAcceptanceStatus::Ready
    } else if entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner {
        StudioShellHandoffAcceptanceStatus::Rejected
    } else {
        StudioShellHandoffAcceptanceStatus::Blocked
    };
    let issue_code = match status {
        StudioShellHandoffAcceptanceStatus::Ready => None,
        StudioShellHandoffAcceptanceStatus::Blocked => entry
            .issue_code
            .clone()
            .or_else(|| first_failed_acceptance_check_issue_code(&checks)),
        StudioShellHandoffAcceptanceStatus::Rejected => {
            first_failed_acceptance_check_issue_code(&checks)
        }
    };

    StudioShellHandoffAcceptanceChecklistEntry {
        graph_id: entry.graph_id.clone(),
        target_kind: entry.target_kind,
        consumer_id: entry.consumer_id.clone(),
        runtime_route_kind: entry.runtime_route_kind.clone(),
        source_decision: entry.decision,
        status,
        issue_code,
        next_required_action: entry.next_required_action.clone(),
        command_session_authority: entry.command_session_authority.clone(),
        install_launch_evidence_authority: entry.install_launch_evidence_authority.clone(),
        studio_role: entry.studio_role.clone(),
        checks,
    }
}

fn shell_handoff_acceptance_entry_checks(
    entry: &StudioShellHandoffIntakeEntry,
) -> Vec<StudioShellHandoffAcceptanceCheck> {
    let mut checks = Vec::new();
    let prefix = if entry.graph_id.is_empty() {
        "unknown".to_string()
    } else {
        entry.graph_id.clone()
    };
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.decision"),
        "rusty.studio",
        entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner,
        "intake entry is ready for runtime owner staging",
        "intake entry is blocked before runtime owner staging",
        "studio.issue.shell_handoff_acceptance_blocked",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.command_session_authority"),
        "rusty.manifold",
        entry.command_session_authority == "rusty.manifold",
        "Manifold remains command/session authority",
        "command/session authority is not Manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_acceptance_check(
        &mut checks,
        &format!(
            "studio.check.shell_handoff_acceptance.entry.{prefix}.install_launch_evidence_authority"
        ),
        "rusty.hostess",
        entry.install_launch_evidence_authority == "rusty.hostess",
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority is not Hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.studio_role"),
        "rusty.studio",
        entry.studio_role == "authoring.export_planning",
        "Studio role remains authoring/export planning",
        "Studio role exceeds authoring/export planning",
        "studio.issue.runtime_authority_mismatch",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.route_kind"),
        "rusty.manifold",
        entry.runtime_route_kind
            == format!(
                "{}_operator_shell",
                shell_target_kind_label(entry.target_kind)
            ),
        "runtime route kind matches target kind",
        "runtime route kind does not match target kind",
        "studio.issue.shell_handoff_route_mismatch",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.bundle_dir"),
        "rusty.hostess",
        !entry.bundle_dir.trim().is_empty(),
        "bundle dir is available for downstream staging",
        "bundle dir is missing",
        "studio.issue.handoff_path_missing",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.template_index_path"),
        "rusty.hostess",
        path_ends_with_shell_templates(&entry.template_index_path),
        "template index path points to shell-templates.json",
        "template index path does not point to shell-templates.json",
        "studio.issue.handoff_template_index_path_mismatch",
    );
    let consumer_args_ready = entry.consumer_args.iter().any(|arg| arg == "--templates")
        && entry
            .consumer_args
            .iter()
            .any(|arg| arg == &entry.template_index_path);
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.consumer_args"),
        "rusty.manifold",
        consumer_args_ready,
        "consumer args identify the template index",
        "consumer args do not identify the template index",
        "studio.issue.handoff_consumer_args_mismatch",
    );
    push_acceptance_check(
        &mut checks,
        &format!("studio.check.shell_handoff_acceptance.entry.{prefix}.operator_shell"),
        "rusty.studio",
        !entry.operator_shell_ids.is_empty(),
        "operator shell ids are present",
        "operator shell ids are missing",
        "studio.issue.no_operator_shell",
    );
    checks
}

fn push_acceptance_check(
    checks: &mut Vec<StudioShellHandoffAcceptanceCheck>,
    check_id: &str,
    owner: &str,
    passed: bool,
    pass_evidence: &str,
    fail_evidence: &str,
    issue_code: &str,
) {
    checks.push(StudioShellHandoffAcceptanceCheck {
        check_id: check_id.to_string(),
        owner: owner.to_string(),
        status: if passed {
            StudioValidationStatus::Pass
        } else {
            StudioValidationStatus::Fail
        },
        evidence: if passed { pass_evidence } else { fail_evidence }.to_string(),
        issue_code: (!passed).then(|| issue_code.to_string()),
    });
}

pub(crate) fn shell_handoff_acceptance_prohibited_actions() -> Vec<String> {
    [
        "install",
        "launch",
        "open_command_session",
        "collect_device_evidence",
    ]
    .iter()
    .map(|action| action.to_string())
    .collect()
}

fn first_failed_acceptance_check_issue_code(
    checks: &[StudioShellHandoffAcceptanceCheck],
) -> Option<String> {
    checks
        .iter()
        .find(|check| check.status == StudioValidationStatus::Fail)
        .and_then(|check| check.issue_code.clone())
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
