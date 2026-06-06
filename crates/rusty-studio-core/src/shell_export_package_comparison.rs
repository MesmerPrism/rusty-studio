use super::*;

pub fn compare_shell_export_packages(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> StudioShellExportPackageComparisonReport {
    compare_shell_export_packages_with_identity(baseline, candidate, None, None)
}

pub fn compare_shell_export_packages_against_baseline_manifest(
    baseline_identity: &StudioShellExportPackageBaselineManifest,
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> StudioShellExportPackageComparisonReport {
    compare_shell_export_packages_with_identity(baseline, candidate, Some(baseline_identity), None)
}

pub fn compare_shell_export_packages_against_baseline_index_entry(
    baseline_index: &StudioShellExportPackageBaselineIndex,
    baseline_index_path: Option<&Path>,
    baseline_index_entry: &StudioShellExportPackageBaselineIndexEntry,
    baseline_manifest_path: Option<&Path>,
    baseline_identity: &StudioShellExportPackageBaselineManifest,
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> StudioShellExportPackageComparisonReport {
    compare_shell_export_packages_with_identity(
        baseline,
        candidate,
        Some(baseline_identity),
        Some(ShellExportPackageBaselineIndexComparisonContext {
            index: baseline_index,
            index_path: baseline_index_path,
            entry: baseline_index_entry,
            baseline_manifest_path,
        }),
    )
}

struct ShellExportPackageBaselineIndexComparisonContext<'a> {
    index: &'a StudioShellExportPackageBaselineIndex,
    index_path: Option<&'a Path>,
    entry: &'a StudioShellExportPackageBaselineIndexEntry,
    baseline_manifest_path: Option<&'a Path>,
}

fn compare_shell_export_packages_with_identity(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
    baseline_identity: Option<&StudioShellExportPackageBaselineManifest>,
    baseline_index: Option<ShellExportPackageBaselineIndexComparisonContext<'_>>,
) -> StudioShellExportPackageComparisonReport {
    let mut checks = shell_export_package_comparison_checks(baseline, candidate);
    if let Some(baseline_identity) = baseline_identity {
        checks.extend(shell_export_package_baseline_identity_checks(
            baseline_identity,
            baseline,
        ));
        if let Some(baseline_index) = baseline_index.as_ref() {
            checks.extend(shell_export_package_baseline_index_entry_checks(
                baseline_index,
                baseline_identity,
            ));
        }
    }
    let comparable = checks
        .iter()
        .all(|check| check.status == StudioValidationStatus::Pass);

    let entries = if comparable {
        shell_export_package_comparison_entries(baseline, candidate)
    } else {
        Vec::new()
    };

    let ready_delta = count_delta(candidate.ready_count, baseline.ready_count);
    let blocked_delta = count_delta(candidate.blocked_count, baseline.blocked_count);
    let rejected_delta = count_delta(candidate.rejected_count, baseline.rejected_count);
    let descriptor_delta = count_delta(candidate.descriptor_count, baseline.descriptor_count);
    let template_manifest_delta = count_delta(
        candidate.template_manifest_count,
        baseline.template_manifest_count,
    );
    let runbook_entry_delta =
        count_delta(candidate.runbook_entry_count, baseline.runbook_entry_count);

    let status = if !comparable {
        StudioShellExportPackageComparisonStatus::Incomparable
    } else if export_package_status_score(candidate.status)
        < export_package_status_score(baseline.status)
        || ready_delta < 0
        || blocked_delta > 0
        || rejected_delta > 0
        || descriptor_delta < 0
        || template_manifest_delta < 0
        || runbook_entry_delta < 0
        || entries.iter().any(|entry| {
            matches!(
                entry.change,
                StudioShellExportPackageComparisonChange::Regressed
                    | StudioShellExportPackageComparisonChange::Removed
                    | StudioShellExportPackageComparisonChange::Changed
            )
        })
    {
        StudioShellExportPackageComparisonStatus::Regressed
    } else if export_package_status_score(candidate.status)
        > export_package_status_score(baseline.status)
        || ready_delta > 0
        || blocked_delta < 0
        || rejected_delta < 0
        || descriptor_delta > 0
        || template_manifest_delta > 0
        || runbook_entry_delta > 0
        || entries
            .iter()
            .any(|entry| entry.change == StudioShellExportPackageComparisonChange::Improved)
    {
        StudioShellExportPackageComparisonStatus::Improved
    } else {
        StudioShellExportPackageComparisonStatus::Unchanged
    };

    let issue_code = match status {
        StudioShellExportPackageComparisonStatus::Incomparable => {
            first_failed_validation_check_issue_code(&checks)
        }
        StudioShellExportPackageComparisonStatus::Regressed => entries
            .iter()
            .find(|entry| {
                matches!(
                    entry.change,
                    StudioShellExportPackageComparisonChange::Regressed
                        | StudioShellExportPackageComparisonChange::Removed
                        | StudioShellExportPackageComparisonChange::Changed
                )
            })
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| Some("studio.issue.shell_export_package_regressed".to_string())),
        StudioShellExportPackageComparisonStatus::Improved
        | StudioShellExportPackageComparisonStatus::Unchanged => None,
    };

    StudioShellExportPackageComparisonReport {
        schema_id: SHELL_EXPORT_PACKAGE_COMPARISON_SCHEMA.to_string(),
        baseline_identity_schema: baseline_identity.map(|identity| identity.schema_id.clone()),
        baseline_id: baseline_identity.map(|identity| identity.baseline_id.clone()),
        baseline_label: baseline_identity.map(|identity| identity.label.clone()),
        baseline_package_path: baseline_identity.map(|identity| identity.package_path.clone()),
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
        baseline_package_id: baseline.package_id.clone(),
        candidate_package_id: candidate.package_id.clone(),
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
        baseline_descriptor_count: baseline.descriptor_count,
        candidate_descriptor_count: candidate.descriptor_count,
        descriptor_delta,
        baseline_template_manifest_count: baseline.template_manifest_count,
        candidate_template_manifest_count: candidate.template_manifest_count,
        template_manifest_delta,
        baseline_runbook_entry_count: baseline.runbook_entry_count,
        candidate_runbook_entry_count: candidate.runbook_entry_count,
        runbook_entry_delta,
        checks,
        entries,
    }
}

fn shell_export_package_comparison_checks(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_schema",
        baseline.schema_id == SHELL_EXPORT_PACKAGE_REPORT_SCHEMA,
        "baseline export-package schema id is supported",
        "baseline export-package schema id is unsupported",
        "studio.issue.shell_export_package_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.candidate_schema",
        candidate.schema_id == SHELL_EXPORT_PACKAGE_REPORT_SCHEMA,
        "candidate export-package schema id is supported",
        "candidate export-package schema id is unsupported",
        "studio.issue.shell_export_package_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_source_schemas",
        baseline.source_manifest_schema == SHELL_HANDOFF_MANIFEST_SCHEMA
            && baseline.source_runbook_schema == SHELL_RUNBOOK_REPORT_SCHEMA,
        "baseline source schemas are supported",
        "baseline source schemas are unsupported",
        "studio.issue.shell_export_package_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.candidate_source_schemas",
        candidate.source_manifest_schema == SHELL_HANDOFF_MANIFEST_SCHEMA
            && candidate.source_runbook_schema == SHELL_RUNBOOK_REPORT_SCHEMA,
        "candidate source schemas are supported",
        "candidate source schemas are unsupported",
        "studio.issue.shell_export_package_source_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.project_id",
        baseline.project_id == candidate.project_id,
        "baseline and candidate project ids match",
        "baseline and candidate project ids differ",
        "studio.issue.shell_export_package_project_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.execution_policy",
        baseline.execution_policy == candidate.execution_policy
            && baseline.execution_policy == "not_executed.review_only",
        "baseline and candidate use review-only execution policy",
        "baseline and candidate execution policies differ or are executable",
        "studio.issue.shell_export_package_execution_policy_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.authority",
        baseline.command_session_authority == candidate.command_session_authority
            && baseline.command_session_authority == "rusty.manifold"
            && baseline.install_launch_evidence_authority
                == candidate.install_launch_evidence_authority
            && baseline.install_launch_evidence_authority == "rusty.hostess"
            && baseline.studio_role == candidate.studio_role
            && baseline.studio_role == "authoring.export_planning",
        "baseline and candidate keep Manifold/Hostess/Studio authority",
        "baseline and candidate authority fields differ or drifted",
        "studio.issue.shell_export_package_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.prohibited_actions",
        string_set(&baseline.prohibited_actions) == string_set(&candidate.prohibited_actions),
        "baseline and candidate prohibited actions match",
        "baseline and candidate prohibited actions differ",
        "studio.issue.shell_export_package_prohibited_actions_mismatch",
    );
    checks
}

fn shell_export_package_baseline_identity_checks(
    baseline_identity: &StudioShellExportPackageBaselineManifest,
    baseline: &StudioShellExportPackageReport,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_schema",
        baseline_identity.schema_id == SHELL_EXPORT_PACKAGE_BASELINE_MANIFEST_SCHEMA,
        "baseline identity schema id is supported",
        "baseline identity schema id is unsupported",
        "studio.issue.shell_export_package_baseline_identity_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_package_schema",
        baseline_identity.package_schema == baseline.schema_id,
        "baseline identity names the loaded package schema",
        "baseline identity does not name the loaded package schema",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_package",
        baseline_identity.package_id == baseline.package_id
            && baseline_identity.manifest_id == baseline.manifest_id,
        "baseline identity package ids match the loaded package",
        "baseline identity package ids differ from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_project",
        baseline_identity.project_id == baseline.project_id
            && baseline_identity.project_revision == baseline.project_revision,
        "baseline identity project metadata matches the loaded package",
        "baseline identity project metadata differs from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_status_counts",
        baseline_identity.status == baseline.status
            && baseline_identity.ready_count == baseline.ready_count
            && baseline_identity.blocked_count == baseline.blocked_count
            && baseline_identity.rejected_count == baseline.rejected_count
            && baseline_identity.descriptor_count == baseline.descriptor_count
            && baseline_identity.template_manifest_count == baseline.template_manifest_count
            && baseline_identity.runbook_entry_count == baseline.runbook_entry_count
            && baseline_identity.target_count == baseline.target_summaries.len(),
        "baseline identity review counts match the loaded package",
        "baseline identity review counts differ from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_identity_authority",
        baseline_identity.execution_policy == baseline.execution_policy
            && baseline_identity.review_owner == baseline.review_owner
            && baseline_identity.command_session_authority == baseline.command_session_authority
            && baseline_identity.install_launch_evidence_authority
                == baseline.install_launch_evidence_authority
            && baseline_identity.studio_role == baseline.studio_role
            && string_set(&baseline_identity.prohibited_actions)
                == string_set(&baseline.prohibited_actions),
        "baseline identity authority fields match the loaded package",
        "baseline identity authority fields differ from the loaded package",
        "studio.issue.shell_export_package_baseline_identity_mismatch",
    );
    checks
}

fn shell_export_package_baseline_index_entry_checks(
    context: &ShellExportPackageBaselineIndexComparisonContext<'_>,
    baseline_identity: &StudioShellExportPackageBaselineManifest,
) -> Vec<StudioValidationCheck> {
    let mut checks = Vec::new();
    let entry = context.entry;
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
        "studio.check.shell_export_package_comparison.baseline_index_schema",
        context.index.schema_id == SHELL_EXPORT_PACKAGE_BASELINE_INDEX_SCHEMA,
        "baseline index schema id is supported",
        "baseline index schema id is unsupported",
        "studio.issue.shell_export_package_baseline_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_selected_baseline",
        entry.baseline_id == baseline_identity.baseline_id,
        "baseline index selected entry matches the loaded baseline identity",
        "baseline index selected entry differs from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_manifest_path",
        manifest_path_matches,
        "baseline index entry records the selected baseline manifest path",
        "baseline index entry is missing or mismatches the selected manifest path",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_package_path",
        entry.package_path == baseline_identity.package_path,
        "baseline index entry package path matches the loaded baseline identity",
        "baseline index entry package path differs from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_package",
        entry.package_schema == baseline_identity.package_schema
            && entry.package_id == baseline_identity.package_id
            && entry.manifest_id == baseline_identity.manifest_id,
        "baseline index entry package ids match the loaded baseline identity",
        "baseline index entry package ids differ from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_project",
        entry.project_id == baseline_identity.project_id
            && entry.project_revision == baseline_identity.project_revision,
        "baseline index entry project metadata matches the loaded baseline identity",
        "baseline index entry project metadata differs from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_export_package_comparison.baseline_index_status_counts",
        entry.status == baseline_identity.status
            && entry.ready_count == baseline_identity.ready_count
            && entry.blocked_count == baseline_identity.blocked_count
            && entry.rejected_count == baseline_identity.rejected_count
            && entry.descriptor_count == baseline_identity.descriptor_count
            && entry.template_manifest_count == baseline_identity.template_manifest_count
            && entry.runbook_entry_count == baseline_identity.runbook_entry_count
            && entry.target_count == baseline_identity.target_count,
        "baseline index entry review counts match the loaded baseline identity",
        "baseline index entry review counts differ from the loaded baseline identity",
        "studio.issue.shell_export_package_baseline_index_mismatch",
    );
    checks
}

fn shell_export_package_comparison_entries(
    baseline: &StudioShellExportPackageReport,
    candidate: &StudioShellExportPackageReport,
) -> Vec<StudioShellExportPackageComparisonEntry> {
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
            shell_export_package_comparison_entry(
                &graph_id,
                baseline_entries.get(graph_id.as_str()).copied(),
                candidate_entries.get(graph_id.as_str()).copied(),
            )
        })
        .collect()
}

fn shell_export_package_comparison_entry(
    graph_id: &str,
    baseline: Option<&StudioShellExportPackageEntry>,
    candidate: Option<&StudioShellExportPackageEntry>,
) -> StudioShellExportPackageComparisonEntry {
    let baseline_score = baseline.map(|entry| export_package_status_score(entry.status));
    let candidate_score = candidate.map(|entry| export_package_status_score(entry.status));
    let score_delta = candidate_score.unwrap_or(0) - baseline_score.unwrap_or(0);
    let change = match (baseline, candidate) {
        (None, Some(_)) => StudioShellExportPackageComparisonChange::Added,
        (Some(_), None) => StudioShellExportPackageComparisonChange::Removed,
        (Some(_), Some(_)) if score_delta > 0 => StudioShellExportPackageComparisonChange::Improved,
        (Some(_), Some(_)) if score_delta < 0 => {
            StudioShellExportPackageComparisonChange::Regressed
        }
        (Some(baseline), Some(candidate))
            if baseline.consumer_id != candidate.consumer_id
                || baseline.issue_code != candidate.issue_code
                || baseline.descriptor.is_some() != candidate.descriptor.is_some()
                || baseline.template_manifest.is_some()
                    != candidate.template_manifest.is_some()
                || baseline.runbook_cli_request.is_empty()
                    != candidate.runbook_cli_request.is_empty() =>
        {
            StudioShellExportPackageComparisonChange::Changed
        }
        (Some(_), Some(_)) => StudioShellExportPackageComparisonChange::Unchanged,
        (None, None) => StudioShellExportPackageComparisonChange::Unchanged,
    };
    let issue_code = match change {
        StudioShellExportPackageComparisonChange::Regressed
        | StudioShellExportPackageComparisonChange::Removed => candidate
            .and_then(|entry| entry.issue_code.clone())
            .or_else(|| baseline.and_then(|entry| entry.issue_code.clone()))
            .or_else(|| Some("studio.issue.shell_export_package_regressed".to_string())),
        StudioShellExportPackageComparisonChange::Added
        | StudioShellExportPackageComparisonChange::Improved
        | StudioShellExportPackageComparisonChange::Unchanged
        | StudioShellExportPackageComparisonChange::Changed => None,
    };

    StudioShellExportPackageComparisonEntry {
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
        baseline_descriptor_present: baseline
            .map(|entry| entry.descriptor.is_some())
            .unwrap_or(false),
        candidate_descriptor_present: candidate
            .map(|entry| entry.descriptor.is_some())
            .unwrap_or(false),
        baseline_template_manifest_present: baseline
            .map(|entry| entry.template_manifest.is_some())
            .unwrap_or(false),
        candidate_template_manifest_present: candidate
            .map(|entry| entry.template_manifest.is_some())
            .unwrap_or(false),
        baseline_runbook_cli_request_present: baseline
            .map(|entry| !entry.runbook_cli_request.is_empty())
            .unwrap_or(false),
        candidate_runbook_cli_request_present: candidate
            .map(|entry| !entry.runbook_cli_request.is_empty())
            .unwrap_or(false),
        baseline_issue_code: baseline.and_then(|entry| entry.issue_code.clone()),
        candidate_issue_code: candidate.and_then(|entry| entry.issue_code.clone()),
        issue_code,
    }
}

fn export_package_status_score(status: StudioShellExportPackageStatus) -> isize {
    match status {
        StudioShellExportPackageStatus::Rejected => 0,
        StudioShellExportPackageStatus::Blocked => 1,
        StudioShellExportPackageStatus::Ready => 2,
    }
}
