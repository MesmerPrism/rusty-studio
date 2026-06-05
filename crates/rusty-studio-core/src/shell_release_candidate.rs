use super::*;

pub fn shell_release_candidate_review_for_manifest(
    manifest: &StudioShellHandoffManifest,
    manifest_path: Option<&Path>,
    acceptance_baseline_index: &StudioShellHandoffAcceptanceBaselineIndex,
    acceptance_baseline_index_path: Option<&Path>,
    acceptance_baseline_id: Option<&str>,
    export_package_baseline_index: &StudioShellExportPackageBaselineIndex,
    export_package_baseline_index_path: Option<&Path>,
    export_package_baseline_id: Option<&str>,
) -> StudioShellReleaseCandidateReviewReport {
    let manifest_validation = validate_shell_handoff_manifest(manifest);
    let intake = shell_handoff_intake_for_manifest(manifest);
    let candidate_acceptance = shell_handoff_acceptance_checklist_for_intake(&intake);
    let candidate_export_package = shell_export_package_for_manifest(manifest);
    let acceptance_selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        acceptance_baseline_index,
        acceptance_baseline_index_path,
        acceptance_baseline_id,
    );
    let export_package_selection = summarize_shell_export_package_baseline_index_selection(
        export_package_baseline_index,
        export_package_baseline_index_path,
        export_package_baseline_id,
    );

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.source_manifest_schema",
        manifest.schema_id == SHELL_HANDOFF_MANIFEST_SCHEMA,
        "source handoff manifest schema is supported",
        "source handoff manifest schema is unsupported",
        "studio.issue.shell_release_candidate_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.handoff_manifest_validation",
        manifest_validation.status == StudioValidationStatus::Pass,
        "handoff manifest validation passed",
        "handoff manifest validation failed",
        "studio.issue.shell_release_candidate_manifest_validation_failed",
    );
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.handoff_ready",
        manifest.status == StudioValidationStatus::Pass,
        "handoff manifest is ready for downstream review",
        "handoff manifest still has failed or missing generated shell bundles",
        "studio.issue.shell_release_candidate_handoff_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.runtime_command_authority",
        manifest.runtime_authority.command_session_authority == "rusty.manifold",
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.runtime_host_authority",
        manifest.runtime_authority.install_launch_evidence_authority == "rusty.hostess",
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.studio_role",
        manifest.runtime_authority.studio_role == "authoring.export_planning",
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );

    let (acceptance_comparison, acceptance_checks) = shell_release_candidate_acceptance_comparison(
        acceptance_baseline_index,
        acceptance_baseline_index_path,
        acceptance_baseline_id,
        &acceptance_selection,
        &candidate_acceptance,
    );
    checks.extend(acceptance_checks);
    let acceptance_comparison_ok = acceptance_comparison.as_ref().is_some_and(|comparison| {
        matches!(
            comparison.status,
            StudioShellHandoffAcceptanceComparisonStatus::Improved
                | StudioShellHandoffAcceptanceComparisonStatus::Unchanged
        )
    });
    push_release_candidate_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.acceptance_comparison_not_regressed",
        acceptance_comparison_ok,
        "acceptance comparison is unchanged or improved",
        "acceptance comparison is missing, regressed, or incomparable",
        acceptance_comparison
            .as_ref()
            .and_then(|comparison| comparison.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_release_candidate_acceptance_comparison_blocked"),
    );

    let (export_package_comparison, export_package_checks) =
        shell_release_candidate_export_package_comparison(
            export_package_baseline_index,
            export_package_baseline_index_path,
            export_package_baseline_id,
            &export_package_selection,
            &candidate_export_package,
        );
    checks.extend(export_package_checks);
    let export_package_comparison_ok =
        export_package_comparison
            .as_ref()
            .is_some_and(|comparison| {
                matches!(
                    comparison.status,
                    StudioShellExportPackageComparisonStatus::Improved
                        | StudioShellExportPackageComparisonStatus::Unchanged
                )
            });
    push_release_candidate_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.export_package_comparison_not_regressed",
        export_package_comparison_ok,
        "export-package comparison is unchanged or improved",
        "export-package comparison is missing, regressed, or incomparable",
        export_package_comparison
            .as_ref()
            .and_then(|comparison| comparison.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_release_candidate_export_package_comparison_blocked"),
    );

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let status = if manifest_validation.status == StudioValidationStatus::Fail
        || manifest.schema_id != SHELL_HANDOFF_MANIFEST_SCHEMA
    {
        StudioShellReleaseCandidateReviewStatus::Rejected
    } else if has_failed_check {
        StudioShellReleaseCandidateReviewStatus::Blocked
    } else {
        StudioShellReleaseCandidateReviewStatus::Ready
    };
    let issue_code = match status {
        StudioShellReleaseCandidateReviewStatus::Ready => None,
        StudioShellReleaseCandidateReviewStatus::Blocked
        | StudioShellReleaseCandidateReviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };

    StudioShellReleaseCandidateReviewReport {
        schema_id: SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA.to_string(),
        source_manifest_schema: manifest.schema_id.clone(),
        manifest_path: manifest_path.map(|path| path.display().to_string()),
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        review_owner: "rusty.hostess".to_string(),
        command_session_authority: manifest.runtime_authority.command_session_authority.clone(),
        install_launch_evidence_authority: manifest
            .runtime_authority
            .install_launch_evidence_authority
            .clone(),
        studio_role: manifest.runtime_authority.studio_role.clone(),
        handoff_status: manifest.status,
        handoff_ready_count: manifest.ready_count,
        handoff_failed_count: manifest.failed_count,
        handoff_missing_bundle_count: manifest.missing_bundle_count,
        acceptance_baseline_selection: acceptance_selection,
        acceptance_comparison,
        export_package_baseline_selection: export_package_selection,
        export_package_comparison,
        checks,
        prohibited_actions: unique_strings(
            candidate_acceptance
                .prohibited_actions
                .iter()
                .cloned()
                .chain(candidate_export_package.prohibited_actions.iter().cloned()),
        ),
    }
}

fn shell_release_candidate_acceptance_comparison(
    baseline_index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_index_path: Option<&Path>,
    baseline_id: Option<&str>,
    selection: &StudioShellHandoffAcceptanceBaselineSelectionReport,
    candidate: &StudioShellHandoffAcceptanceChecklistReport,
) -> (
    Option<StudioShellHandoffAcceptanceComparisonReport>,
    Vec<StudioValidationCheck>,
) {
    let mut checks = Vec::new();
    let selected =
        selection.status == StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected;
    push_release_candidate_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.acceptance_baseline_selected",
        selected,
        "acceptance baseline index selected a baseline",
        "acceptance baseline index did not select a baseline",
        selection
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_release_candidate_acceptance_baseline_not_selected"),
    );
    if !selected {
        return (None, checks);
    }

    let Some(entry) =
        select_shell_handoff_acceptance_baseline_index_entry(baseline_index, baseline_id)
    else {
        return (None, checks);
    };
    let Some(baseline_manifest_path) = entry.baseline_manifest_path.as_ref().map(PathBuf::from)
    else {
        push_release_candidate_check(
            &mut checks,
            "studio.check.shell_release_candidate_review.acceptance_baseline_manifest_path",
            false,
            "acceptance baseline index entry has a manifest path",
            "acceptance baseline index entry does not include a manifest path",
            "studio.issue.shell_release_candidate_acceptance_baseline_manifest_missing",
        );
        return (None, checks);
    };

    let baseline_identity =
        match load_shell_handoff_acceptance_baseline_manifest(&baseline_manifest_path) {
            Ok(baseline_identity) => baseline_identity,
            Err(error) => {
                checks.push(failed_release_candidate_check(
                    "studio.check.shell_release_candidate_review.acceptance_baseline_manifest_load",
                    error.to_string(),
                    "studio.issue.shell_release_candidate_acceptance_baseline_load_failed",
                ));
                return (None, checks);
            }
        };
    let baseline_path = PathBuf::from(&baseline_identity.checklist_path);
    let baseline = match load_shell_handoff_acceptance_checklist(&baseline_path) {
        Ok(baseline) => baseline,
        Err(error) => {
            checks.push(failed_release_candidate_check(
                "studio.check.shell_release_candidate_review.acceptance_baseline_checklist_load",
                error.to_string(),
                "studio.issue.shell_release_candidate_acceptance_checklist_load_failed",
            ));
            return (None, checks);
        }
    };
    let comparison = compare_shell_handoff_acceptance_against_baseline_index_entry(
        baseline_index,
        baseline_index_path,
        entry,
        Some(&baseline_manifest_path),
        &baseline_identity,
        &baseline,
        candidate,
    );
    (Some(comparison), checks)
}

fn shell_release_candidate_export_package_comparison(
    baseline_index: &StudioShellExportPackageBaselineIndex,
    baseline_index_path: Option<&Path>,
    baseline_id: Option<&str>,
    selection: &StudioShellExportPackageBaselineSelectionReport,
    candidate: &StudioShellExportPackageReport,
) -> (
    Option<StudioShellExportPackageComparisonReport>,
    Vec<StudioValidationCheck>,
) {
    let mut checks = Vec::new();
    let selected = selection.status == StudioShellExportPackageBaselineSelectionStatus::Selected;
    push_release_candidate_check(
        &mut checks,
        "studio.check.shell_release_candidate_review.export_package_baseline_selected",
        selected,
        "export-package baseline index selected a baseline",
        "export-package baseline index did not select a baseline",
        selection
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_release_candidate_export_package_baseline_not_selected"),
    );
    if !selected {
        return (None, checks);
    }

    let Some(entry) = select_shell_export_package_baseline_index_entry(baseline_index, baseline_id)
    else {
        return (None, checks);
    };
    let Some(baseline_manifest_path) = entry.baseline_manifest_path.as_ref().map(PathBuf::from)
    else {
        push_release_candidate_check(
            &mut checks,
            "studio.check.shell_release_candidate_review.export_package_baseline_manifest_path",
            false,
            "export-package baseline index entry has a manifest path",
            "export-package baseline index entry does not include a manifest path",
            "studio.issue.shell_release_candidate_export_package_baseline_manifest_missing",
        );
        return (None, checks);
    };

    let baseline_identity =
        match load_shell_export_package_baseline_manifest(&baseline_manifest_path) {
            Ok(baseline_identity) => baseline_identity,
            Err(error) => {
                checks.push(failed_release_candidate_check(
                "studio.check.shell_release_candidate_review.export_package_baseline_manifest_load",
                error.to_string(),
                "studio.issue.shell_release_candidate_export_package_baseline_load_failed",
            ));
                return (None, checks);
            }
        };
    let baseline_path = PathBuf::from(&baseline_identity.package_path);
    let baseline = match load_shell_export_package_report(&baseline_path) {
        Ok(baseline) => baseline,
        Err(error) => {
            checks.push(failed_release_candidate_check(
                "studio.check.shell_release_candidate_review.export_package_baseline_report_load",
                error.to_string(),
                "studio.issue.shell_release_candidate_export_package_report_load_failed",
            ));
            return (None, checks);
        }
    };
    let comparison = compare_shell_export_packages_against_baseline_index_entry(
        baseline_index,
        baseline_index_path,
        entry,
        Some(&baseline_manifest_path),
        &baseline_identity,
        &baseline,
        candidate,
    );
    (Some(comparison), checks)
}

fn push_release_candidate_check(
    checks: &mut Vec<StudioValidationCheck>,
    check_id: &str,
    valid: bool,
    pass_evidence: &str,
    fail_evidence: &str,
    issue_code: &str,
) {
    push_check(
        checks,
        check_id,
        valid,
        pass_evidence,
        fail_evidence,
        issue_code,
    );
}

fn failed_release_candidate_check(
    check_id: &str,
    evidence: String,
    issue_code: &str,
) -> StudioValidationCheck {
    StudioValidationCheck {
        check_id: check_id.to_string(),
        status: StudioValidationStatus::Fail,
        evidence,
        issue_code: Some(issue_code.to_string()),
        graph_id: None,
        node_ids: Vec::new(),
        edge_ids: Vec::new(),
        reference_ids: Vec::new(),
    }
}

pub fn shell_release_candidate_review_manifest_for_report(
    review: &StudioShellReleaseCandidateReviewReport,
    review_path: &Path,
    candidate_id: Option<&str>,
    label: Option<&str>,
) -> StudioShellReleaseCandidateReviewManifest {
    let candidate_id = candidate_id
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_release_candidate_review_id(review));
    let label = label
        .map(str::to_string)
        .unwrap_or_else(|| default_shell_release_candidate_review_label(review));

    StudioShellReleaseCandidateReviewManifest {
        schema_id: SHELL_RELEASE_CANDIDATE_REVIEW_MANIFEST_SCHEMA.to_string(),
        candidate_id,
        label,
        review_path: review_path.display().to_string(),
        review_schema: review.schema_id.clone(),
        manifest_id: review.manifest_id.clone(),
        project_id: review.project_id.clone(),
        project_revision: review.project_revision,
        status: review.status,
        issue_code: review.issue_code.clone(),
        execution_policy: review.execution_policy.clone(),
        review_owner: review.review_owner.clone(),
        command_session_authority: review.command_session_authority.clone(),
        install_launch_evidence_authority: review.install_launch_evidence_authority.clone(),
        studio_role: review.studio_role.clone(),
        handoff_ready_count: review.handoff_ready_count,
        handoff_failed_count: review.handoff_failed_count,
        handoff_missing_bundle_count: review.handoff_missing_bundle_count,
        acceptance_baseline_status: review.acceptance_baseline_selection.status,
        acceptance_baseline_id: review
            .acceptance_baseline_selection
            .selected_baseline_id
            .clone(),
        acceptance_comparison_status: review
            .acceptance_comparison
            .as_ref()
            .map(|comparison| comparison.status),
        export_package_baseline_status: review.export_package_baseline_selection.status,
        export_package_baseline_id: review
            .export_package_baseline_selection
            .selected_baseline_id
            .clone(),
        export_package_comparison_status: review
            .export_package_comparison
            .as_ref()
            .map(|comparison| comparison.status),
        check_count: review.checks.len(),
        failed_check_count: review
            .checks
            .iter()
            .filter(|check| check.status == StudioValidationStatus::Fail)
            .count(),
        prohibited_actions: review.prohibited_actions.clone(),
    }
}

pub fn shell_release_candidate_review_index_for_manifests(
    candidates: Vec<(StudioShellReleaseCandidateReviewManifest, Option<PathBuf>)>,
    default_candidate_id: Option<&str>,
) -> StudioShellReleaseCandidateReviewIndex {
    let entries = candidates
        .into_iter()
        .map(|(candidate, candidate_manifest_path)| {
            shell_release_candidate_review_index_entry_for_manifest(
                candidate,
                candidate_manifest_path,
            )
        })
        .collect::<Vec<_>>();

    shell_release_candidate_review_index_for_entries(entries, default_candidate_id)
}

pub fn append_shell_release_candidate_review_index_manifests(
    index: &StudioShellReleaseCandidateReviewIndex,
    candidates: Vec<(StudioShellReleaseCandidateReviewManifest, Option<PathBuf>)>,
    default_candidate_id: Option<&str>,
) -> StudioShellReleaseCandidateReviewIndex {
    let entries = index
        .entries
        .iter()
        .cloned()
        .chain(
            candidates
                .into_iter()
                .map(|(candidate, candidate_manifest_path)| {
                    shell_release_candidate_review_index_entry_for_manifest(
                        candidate,
                        candidate_manifest_path,
                    )
                }),
        )
        .collect::<Vec<_>>();
    let default_candidate_id = default_candidate_id.or(index.default_candidate_id.as_deref());

    shell_release_candidate_review_index_for_entries(entries, default_candidate_id)
}

pub fn promote_shell_release_candidate_review_index_default(
    index: &StudioShellReleaseCandidateReviewIndex,
    candidate_id: &str,
) -> Option<StudioShellReleaseCandidateReviewIndex> {
    index
        .entries
        .iter()
        .any(|entry| entry.candidate_id == candidate_id)
        .then(|| {
            shell_release_candidate_review_index_for_entries(
                index.entries.clone(),
                Some(candidate_id),
            )
        })
}

fn shell_release_candidate_review_index_entry_for_manifest(
    candidate: StudioShellReleaseCandidateReviewManifest,
    candidate_manifest_path: Option<PathBuf>,
) -> StudioShellReleaseCandidateReviewIndexEntry {
    StudioShellReleaseCandidateReviewIndexEntry {
        candidate_id: candidate.candidate_id,
        label: candidate.label,
        candidate_manifest_path: candidate_manifest_path.map(|path| path.display().to_string()),
        review_path: candidate.review_path,
        review_schema: candidate.review_schema,
        manifest_id: candidate.manifest_id,
        project_id: candidate.project_id,
        project_revision: candidate.project_revision,
        status: candidate.status,
        issue_code: candidate.issue_code,
        execution_policy: candidate.execution_policy,
        review_owner: candidate.review_owner,
        command_session_authority: candidate.command_session_authority,
        install_launch_evidence_authority: candidate.install_launch_evidence_authority,
        studio_role: candidate.studio_role,
        handoff_ready_count: candidate.handoff_ready_count,
        handoff_failed_count: candidate.handoff_failed_count,
        handoff_missing_bundle_count: candidate.handoff_missing_bundle_count,
        acceptance_baseline_status: candidate.acceptance_baseline_status,
        acceptance_baseline_id: candidate.acceptance_baseline_id,
        acceptance_comparison_status: candidate.acceptance_comparison_status,
        export_package_baseline_status: candidate.export_package_baseline_status,
        export_package_baseline_id: candidate.export_package_baseline_id,
        export_package_comparison_status: candidate.export_package_comparison_status,
        check_count: candidate.check_count,
        failed_check_count: candidate.failed_check_count,
    }
}

fn shell_release_candidate_review_index_for_entries(
    entries: Vec<StudioShellReleaseCandidateReviewIndexEntry>,
    default_candidate_id: Option<&str>,
) -> StudioShellReleaseCandidateReviewIndex {
    let mut by_id = BTreeMap::new();
    for entry in entries {
        by_id.insert(entry.candidate_id.clone(), entry);
    }

    let entries = by_id.into_values().collect::<Vec<_>>();
    let default_candidate_id = default_candidate_id
        .filter(|candidate_id| {
            entries
                .iter()
                .any(|entry| entry.candidate_id == *candidate_id)
        })
        .map(str::to_string)
        .or_else(|| entries.first().map(|entry| entry.candidate_id.clone()));

    StudioShellReleaseCandidateReviewIndex {
        schema_id: SHELL_RELEASE_CANDIDATE_REVIEW_INDEX_SCHEMA.to_string(),
        project_ids: unique_strings(entries.iter().map(|entry| entry.project_id.clone())),
        manifest_ids: unique_strings(entries.iter().map(|entry| entry.manifest_id.clone())),
        default_candidate_id,
        candidate_count: entries.len(),
        ready_candidate_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellReleaseCandidateReviewStatus::Ready)
            .count(),
        blocked_candidate_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellReleaseCandidateReviewStatus::Blocked)
            .count(),
        rejected_candidate_count: entries
            .iter()
            .filter(|entry| entry.status == StudioShellReleaseCandidateReviewStatus::Rejected)
            .count(),
        entries,
    }
}

pub fn select_shell_release_candidate_review_index_entry<'a>(
    index: &'a StudioShellReleaseCandidateReviewIndex,
    candidate_id: Option<&str>,
) -> Option<&'a StudioShellReleaseCandidateReviewIndexEntry> {
    let selected_id = candidate_id.or(index.default_candidate_id.as_deref());
    selected_id
        .and_then(|selected_id| {
            index
                .entries
                .iter()
                .find(|entry| entry.candidate_id == selected_id)
        })
        .or_else(|| {
            candidate_id
                .is_none()
                .then(|| index.entries.first())
                .flatten()
        })
}

pub fn summarize_shell_release_candidate_review_index_selection(
    index: &StudioShellReleaseCandidateReviewIndex,
    index_path: Option<&Path>,
    requested_candidate_id: Option<&str>,
) -> StudioShellReleaseCandidateReviewSelectionReport {
    let selected_entry =
        select_shell_release_candidate_review_index_entry(index, requested_candidate_id);
    let selected_candidate_id = selected_entry.map(|entry| entry.candidate_id.clone());
    let status = if index.entries.is_empty() {
        StudioShellReleaseCandidateReviewSelectionStatus::Empty
    } else if selected_entry.is_some() {
        StudioShellReleaseCandidateReviewSelectionStatus::Selected
    } else {
        StudioShellReleaseCandidateReviewSelectionStatus::Missing
    };
    let issue_code = match status {
        StudioShellReleaseCandidateReviewSelectionStatus::Selected => None,
        StudioShellReleaseCandidateReviewSelectionStatus::Missing => {
            Some("studio.issue.shell_release_candidate_review_not_found".to_string())
        }
        StudioShellReleaseCandidateReviewSelectionStatus::Empty => {
            Some("studio.issue.shell_release_candidate_review_index_empty".to_string())
        }
    };

    StudioShellReleaseCandidateReviewSelectionReport {
        schema_id: SHELL_RELEASE_CANDIDATE_REVIEW_SELECTION_SCHEMA.to_string(),
        source_index_schema: index.schema_id.clone(),
        index_path: index_path.map(|path| path.display().to_string()),
        requested_candidate_id: requested_candidate_id.map(str::to_string),
        default_candidate_id: index.default_candidate_id.clone(),
        selected_candidate_id: selected_candidate_id.clone(),
        status,
        issue_code,
        candidate_count: index.candidate_count,
        ready_candidate_count: index.ready_candidate_count,
        blocked_candidate_count: index.blocked_candidate_count,
        rejected_candidate_count: index.rejected_candidate_count,
        project_ids: index.project_ids.clone(),
        manifest_ids: index.manifest_ids.clone(),
        entries: index
            .entries
            .iter()
            .map(|entry| StudioShellReleaseCandidateReviewSelectionEntry {
                candidate_id: entry.candidate_id.clone(),
                label: entry.label.clone(),
                selected: selected_candidate_id.as_deref() == Some(entry.candidate_id.as_str()),
                default: index.default_candidate_id.as_deref() == Some(entry.candidate_id.as_str()),
                candidate_manifest_path: entry.candidate_manifest_path.clone(),
                review_path: entry.review_path.clone(),
                manifest_id: entry.manifest_id.clone(),
                project_id: entry.project_id.clone(),
                project_revision: entry.project_revision,
                status: entry.status,
                issue_code: entry.issue_code.clone(),
                acceptance_baseline_id: entry.acceptance_baseline_id.clone(),
                acceptance_comparison_status: entry.acceptance_comparison_status,
                export_package_baseline_id: entry.export_package_baseline_id.clone(),
                export_package_comparison_status: entry.export_package_comparison_status,
                check_count: entry.check_count,
                failed_check_count: entry.failed_check_count,
            })
            .collect(),
    }
}
