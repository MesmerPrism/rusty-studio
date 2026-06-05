use super::*;

pub fn shell_hostess_handoff_package_for_release_candidate_index(
    index: &StudioShellReleaseCandidateReviewIndex,
    index_path: Option<&Path>,
    requested_candidate_id: Option<&str>,
) -> StudioShellHostessHandoffPackageReport {
    let selection = summarize_shell_release_candidate_review_index_selection(
        index,
        index_path,
        requested_candidate_id,
    );
    let selected_entry =
        select_shell_release_candidate_review_index_entry(index, requested_candidate_id);
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.source_index_schema",
        index.schema_id == SHELL_RELEASE_CANDIDATE_REVIEW_INDEX_SCHEMA,
        "source release-candidate review index schema is supported",
        "source release-candidate review index schema is unsupported",
        "studio.issue.shell_release_candidate_review_index_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.candidate_selected",
        selection.status == StudioShellReleaseCandidateReviewSelectionStatus::Selected,
        "release-candidate review index selected a candidate",
        "release-candidate review index did not select a candidate",
        selection
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_release_candidate_review_not_selected"),
    );

    let candidate_manifest_path = selected_entry
        .and_then(|entry| entry.candidate_manifest_path.as_ref())
        .map(PathBuf::from);
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.candidate_manifest_path",
        candidate_manifest_path.is_some(),
        "selected candidate has an identity manifest path",
        "selected candidate does not include an identity manifest path",
        "studio.issue.shell_hostess_handoff_candidate_manifest_missing",
    );

    let candidate_manifest = candidate_manifest_path.as_ref().and_then(|path| {
        match load_shell_release_candidate_review_manifest(path) {
            Ok(candidate) => Some(candidate),
            Err(error) => {
                checks.push(failed_hostess_handoff_package_check(
                    "studio.check.shell_hostess_handoff_package.candidate_manifest_load",
                    error.to_string(),
                    "studio.issue.shell_hostess_handoff_candidate_manifest_load_failed",
                ));
                None
            }
        }
    });

    let candidate_manifest_schema = candidate_manifest
        .as_ref()
        .map(|candidate| candidate.schema_id.clone());
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.candidate_manifest_schema",
        candidate_manifest.as_ref().is_some_and(|candidate| {
            candidate.schema_id == SHELL_RELEASE_CANDIDATE_REVIEW_MANIFEST_SCHEMA
        }),
        "selected candidate identity manifest schema is supported",
        "selected candidate identity manifest schema is unsupported or unavailable",
        "studio.issue.shell_release_candidate_review_manifest_schema",
    );
    let candidate_id_matches_index = selected_entry
        .zip(candidate_manifest.as_ref())
        .is_some_and(|(entry, candidate)| entry.candidate_id == candidate.candidate_id);
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.candidate_identity_matches_index",
        candidate_id_matches_index,
        "selected candidate identity matches the index entry",
        "selected candidate identity does not match the index entry",
        "studio.issue.shell_hostess_handoff_candidate_identity_mismatch",
    );

    let review_path = candidate_manifest
        .as_ref()
        .map(|candidate| PathBuf::from(&candidate.review_path))
        .or_else(|| selected_entry.map(|entry| PathBuf::from(&entry.review_path)));
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.review_path",
        review_path.is_some(),
        "selected candidate names a release-candidate review artifact",
        "selected candidate does not name a release-candidate review artifact",
        "studio.issue.shell_hostess_handoff_review_missing",
    );

    let review = review_path.as_ref().and_then(|path| {
        match load_shell_release_candidate_review_report(path) {
            Ok(review) => Some(review),
            Err(error) => {
                checks.push(failed_hostess_handoff_package_check(
                    "studio.check.shell_hostess_handoff_package.review_load",
                    error.to_string(),
                    "studio.issue.shell_hostess_handoff_review_load_failed",
                ));
                None
            }
        }
    });
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.review_schema",
        review
            .as_ref()
            .is_some_and(|review| review.schema_id == SHELL_RELEASE_CANDIDATE_REVIEW_SCHEMA),
        "selected review artifact schema is supported",
        "selected review artifact schema is unsupported or unavailable",
        "studio.issue.shell_release_candidate_review_schema",
    );
    let review_matches_candidate = candidate_manifest
        .as_ref()
        .zip(review_path.as_ref())
        .is_some_and(|(candidate, review_path)| {
            candidate.review_path == review_path.display().to_string()
        });
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.review_matches_candidate",
        review_matches_candidate,
        "selected review artifact path matches the candidate identity",
        "selected review artifact path does not match the candidate identity",
        "studio.issue.shell_hostess_handoff_review_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.review_ready",
        review
            .as_ref()
            .is_some_and(|review| review.status == StudioShellReleaseCandidateReviewStatus::Ready),
        "selected release candidate is ready for Hostess handoff",
        "selected release candidate is not ready for Hostess handoff",
        review
            .as_ref()
            .and_then(|review| review.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_hostess_handoff_release_candidate_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.handoff_manifest_path",
        review
            .as_ref()
            .and_then(|review| review.manifest_path.as_ref())
            .is_some(),
        "selected review names a saved handoff manifest",
        "selected review does not name a saved handoff manifest",
        "studio.issue.shell_hostess_handoff_manifest_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.handoff_ready",
        review.as_ref().is_some_and(|review| {
            review.handoff_status == StudioValidationStatus::Pass
                && review.handoff_failed_count == 0
                && review.handoff_missing_bundle_count == 0
        }),
        "handoff manifest is ready with no failed or missing bundles",
        "handoff manifest has failed or missing bundles",
        "studio.issue.shell_release_candidate_handoff_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.runtime_command_authority",
        review
            .as_ref()
            .is_some_and(|review| review.command_session_authority == "rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.runtime_host_authority",
        review
            .as_ref()
            .is_some_and(|review| review.install_launch_evidence_authority == "rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.studio_role",
        review
            .as_ref()
            .is_some_and(|review| review.studio_role == "authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.acceptance_baseline_selected",
        review.as_ref().is_some_and(|review| {
            review.acceptance_baseline_selection.status
                == StudioShellHandoffAcceptanceBaselineSelectionStatus::Selected
        }),
        "acceptance baseline selection is present",
        "acceptance baseline selection is missing",
        "studio.issue.shell_release_candidate_acceptance_baseline_not_selected",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.acceptance_comparison_ready",
        review.as_ref().is_some_and(|review| {
            review
                .acceptance_comparison
                .as_ref()
                .is_some_and(|comparison| {
                    matches!(
                        comparison.status,
                        StudioShellHandoffAcceptanceComparisonStatus::Improved
                            | StudioShellHandoffAcceptanceComparisonStatus::Unchanged
                    )
                })
        }),
        "acceptance comparison is unchanged or improved",
        "acceptance comparison is missing, regressed, or incomparable",
        review
            .as_ref()
            .and_then(|review| review.acceptance_comparison.as_ref())
            .and_then(|comparison| comparison.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_release_candidate_acceptance_comparison_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.export_package_baseline_selected",
        review.as_ref().is_some_and(|review| {
            review.export_package_baseline_selection.status
                == StudioShellExportPackageBaselineSelectionStatus::Selected
        }),
        "export-package baseline selection is present",
        "export-package baseline selection is missing",
        "studio.issue.shell_release_candidate_export_package_baseline_not_selected",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_handoff_package.export_package_comparison_ready",
        review.as_ref().is_some_and(|review| {
            review
                .export_package_comparison
                .as_ref()
                .is_some_and(|comparison| {
                    matches!(
                        comparison.status,
                        StudioShellExportPackageComparisonStatus::Improved
                            | StudioShellExportPackageComparisonStatus::Unchanged
                    )
                })
        }),
        "export-package comparison is unchanged or improved",
        "export-package comparison is missing, regressed, or incomparable",
        review
            .as_ref()
            .and_then(|review| review.export_package_comparison.as_ref())
            .and_then(|comparison| comparison.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_release_candidate_export_package_comparison_blocked"),
    );

    let prohibited_actions = shell_hostess_handoff_package_prohibited_actions(review.as_ref());
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
            &format!("studio.check.shell_hostess_handoff_package.prohibits_{action}"),
            prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "handoff package explicitly prohibits this Studio action",
            "handoff package does not explicitly prohibit this Studio action",
            "studio.issue.shell_hostess_handoff_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_release_candidate_review_index_schema")
                    | Some("studio.issue.shell_release_candidate_review_manifest_schema")
                    | Some("studio.issue.shell_release_candidate_review_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessHandoffPackageStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessHandoffPackageStatus::Blocked
    } else {
        StudioShellHostessHandoffPackageStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessHandoffPackageStatus::Ready => None,
        StudioShellHostessHandoffPackageStatus::Blocked
        | StudioShellHostessHandoffPackageStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let required_owner_actions =
        shell_hostess_handoff_package_owner_actions(status, issue_code.as_deref());

    StudioShellHostessHandoffPackageReport {
        schema_id: SHELL_HOSTESS_HANDOFF_PACKAGE_SCHEMA.to_string(),
        source_index_schema: index.schema_id.clone(),
        index_path: index_path.map(|path| path.display().to_string()),
        requested_candidate_id: requested_candidate_id.map(str::to_string),
        default_candidate_id: index.default_candidate_id.clone(),
        selected_candidate_id: selection.selected_candidate_id.clone(),
        selection_status: selection.status,
        selection_issue_code: selection.issue_code,
        candidate_manifest_schema,
        candidate_manifest_path: candidate_manifest_path.map(|path| path.display().to_string()),
        candidate_id: candidate_manifest
            .as_ref()
            .map(|candidate| candidate.candidate_id.clone())
            .or_else(|| selected_entry.map(|entry| entry.candidate_id.clone())),
        candidate_label: candidate_manifest
            .as_ref()
            .map(|candidate| candidate.label.clone())
            .or_else(|| selected_entry.map(|entry| entry.label.clone())),
        review_schema: review
            .as_ref()
            .map(|review| review.schema_id.clone())
            .or_else(|| selected_entry.map(|entry| entry.review_schema.clone())),
        review_path: review_path.map(|path| path.display().to_string()),
        handoff_manifest_schema: review
            .as_ref()
            .map(|review| review.source_manifest_schema.clone()),
        handoff_manifest_path: review
            .as_ref()
            .and_then(|review| review.manifest_path.clone()),
        manifest_id: review
            .as_ref()
            .map(|review| review.manifest_id.clone())
            .or_else(|| selected_entry.map(|entry| entry.manifest_id.clone())),
        project_id: review
            .as_ref()
            .map(|review| review.project_id.clone())
            .or_else(|| selected_entry.map(|entry| entry.project_id.clone())),
        project_revision: review
            .as_ref()
            .map(|review| review.project_revision)
            .or_else(|| selected_entry.map(|entry| entry.project_revision)),
        status,
        issue_code,
        execution_policy: "not_executed.review_only".to_string(),
        handoff_owner: "rusty.hostess".to_string(),
        review_owner: review.as_ref().map(|review| review.review_owner.clone()),
        command_session_authority: review
            .as_ref()
            .map(|review| review.command_session_authority.clone()),
        install_launch_evidence_authority: review
            .as_ref()
            .map(|review| review.install_launch_evidence_authority.clone()),
        studio_role: review.as_ref().map(|review| review.studio_role.clone()),
        handoff_ready_count: review
            .as_ref()
            .map(|review| review.handoff_ready_count)
            .unwrap_or(0),
        handoff_failed_count: review
            .as_ref()
            .map(|review| review.handoff_failed_count)
            .unwrap_or(0),
        handoff_missing_bundle_count: review
            .as_ref()
            .map(|review| review.handoff_missing_bundle_count)
            .unwrap_or(0),
        acceptance_baseline_id: review.as_ref().and_then(|review| {
            review
                .acceptance_baseline_selection
                .selected_baseline_id
                .clone()
        }),
        acceptance_baseline_status: review
            .as_ref()
            .map(|review| review.acceptance_baseline_selection.status),
        acceptance_comparison_status: review
            .as_ref()
            .and_then(|review| review.acceptance_comparison.as_ref())
            .map(|comparison| comparison.status),
        export_package_baseline_id: review.as_ref().and_then(|review| {
            review
                .export_package_baseline_selection
                .selected_baseline_id
                .clone()
        }),
        export_package_baseline_status: review
            .as_ref()
            .map(|review| review.export_package_baseline_selection.status),
        export_package_comparison_status: review
            .as_ref()
            .and_then(|review| review.export_package_comparison.as_ref())
            .map(|comparison| comparison.status),
        required_owner_actions,
        prohibited_actions,
        checks,
    }
}

fn shell_hostess_handoff_package_prohibited_actions(
    review: Option<&StudioShellReleaseCandidateReviewReport>,
) -> Vec<String> {
    unique_strings(
        shell_handoff_acceptance_prohibited_actions()
            .into_iter()
            .chain(
                ["stage_generated_shells", "collect_install_launch_evidence"]
                    .into_iter()
                    .map(str::to_string),
            )
            .chain(
                review
                    .into_iter()
                    .flat_map(|review| review.prohibited_actions.iter().cloned()),
            ),
    )
}

fn shell_hostess_handoff_package_owner_actions(
    status: StudioShellHostessHandoffPackageStatus,
    issue_code: Option<&str>,
) -> Vec<StudioShellHostessHandoffPackageAction> {
    [
        (
            "hostess.review_release_candidate",
            "rusty.hostess",
            "release_candidate_review",
            "review_selected_release_candidate",
        ),
        (
            "hostess.stage_generated_shells",
            "rusty.hostess",
            "shell_handoff_manifest",
            "stage_generated_shells_outside_studio",
        ),
        (
            "manifold.review_command_session_contract",
            "rusty.manifold",
            "release_candidate_review",
            "review_command_session_contract_outside_studio",
        ),
        (
            "hostess.collect_install_launch_evidence",
            "rusty.hostess",
            "hostess_handoff_package",
            "collect_install_launch_evidence_outside_studio",
        ),
    ]
    .into_iter()
    .map(|(action_id, owner, source, next_required_action)| {
        StudioShellHostessHandoffPackageAction {
            action_id: action_id.to_string(),
            owner: owner.to_string(),
            status: if status == StudioShellHostessHandoffPackageStatus::Ready {
                StudioShellHostessHandoffPackageActionStatus::Ready
            } else {
                StudioShellHostessHandoffPackageActionStatus::Blocked
            },
            source: source.to_string(),
            next_required_action: next_required_action.to_string(),
            prohibited_in_studio: true,
            issue_code: (status != StudioShellHostessHandoffPackageStatus::Ready).then(|| {
                issue_code
                    .unwrap_or("studio.issue.shell_hostess_handoff_package_blocked")
                    .to_string()
            }),
        }
    })
    .collect()
}

fn failed_hostess_handoff_package_check(
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

pub fn shell_hostess_owner_intake_for_handoff_package(
    package: &StudioShellHostessHandoffPackageReport,
    package_path: Option<&Path>,
) -> StudioShellHostessOwnerIntakeReport {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.source_package_schema",
        package.schema_id == SHELL_HOSTESS_HANDOFF_PACKAGE_SCHEMA,
        "source Hostess handoff package schema is supported",
        "source Hostess handoff package schema is unsupported",
        "studio.issue.shell_hostess_handoff_package_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.package_path",
        package_path.is_some(),
        "source Hostess handoff package has a durable path",
        "source Hostess handoff package path is missing",
        "studio.issue.shell_hostess_owner_intake_package_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.package_ready",
        package.status == StudioShellHostessHandoffPackageStatus::Ready,
        "source Hostess handoff package is ready",
        "source Hostess handoff package is not ready",
        package
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_handoff_package_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.package_execution_policy",
        package.execution_policy == "not_executed.review_only",
        "source package is a review-only Studio artifact",
        "source package execution policy is not review-only",
        "studio.issue.shell_hostess_handoff_package_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.candidate_selected",
        package.selected_candidate_id.is_some() && package.candidate_id.is_some(),
        "source package names a selected release candidate",
        "source package does not name a selected release candidate",
        "studio.issue.shell_hostess_owner_intake_candidate_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.candidate_manifest_path",
        package.candidate_manifest_path.is_some(),
        "source package names a candidate identity manifest",
        "source package does not name a candidate identity manifest",
        "studio.issue.shell_hostess_owner_intake_candidate_manifest_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.review_path",
        package.review_path.is_some(),
        "source package names a release-candidate review artifact",
        "source package does not name a release-candidate review artifact",
        "studio.issue.shell_hostess_owner_intake_review_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.handoff_manifest_path",
        package.handoff_manifest_path.is_some(),
        "source package names a shell handoff manifest",
        "source package does not name a shell handoff manifest",
        "studio.issue.shell_hostess_owner_intake_handoff_manifest_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.handoff_manifest_schema",
        package.handoff_manifest_schema.as_deref() == Some(SHELL_HANDOFF_MANIFEST_SCHEMA),
        "source handoff manifest schema is supported",
        "source handoff manifest schema is unsupported or unavailable",
        "studio.issue.shell_handoff_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.runtime_command_authority",
        package.command_session_authority.as_deref() == Some("rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.runtime_host_authority",
        package.install_launch_evidence_authority.as_deref() == Some("rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.studio_role",
        package.studio_role.as_deref() == Some("authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.source_package_checks_pass",
        package
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "source Hostess handoff package checks all pass",
        "source Hostess handoff package contains failed checks",
        "studio.issue.shell_hostess_handoff_package_failed_check",
    );

    for action_id in [
        "hostess.review_release_candidate",
        "hostess.stage_generated_shells",
        "manifold.review_command_session_contract",
        "hostess.collect_install_launch_evidence",
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_owner_intake.has_{action_id}"),
            package
                .required_owner_actions
                .iter()
                .any(|action| action.action_id == action_id),
            "source package includes this required owner action",
            "source package is missing this required owner action",
            "studio.issue.shell_hostess_owner_intake_action_missing",
        );
    }
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.owner_actions_ready",
        !package.required_owner_actions.is_empty()
            && package
                .required_owner_actions
                .iter()
                .all(|action| action.status == StudioShellHostessHandoffPackageActionStatus::Ready),
        "all source package owner actions are ready",
        "one or more source package owner actions are blocked",
        "studio.issue.shell_hostess_owner_intake_action_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_owner_intake.owner_actions_prohibited_in_studio",
        !package.required_owner_actions.is_empty()
            && package
                .required_owner_actions
                .iter()
                .all(|action| action.prohibited_in_studio),
        "all downstream owner actions are explicitly prohibited in Studio",
        "one or more downstream owner actions are not prohibited in Studio",
        "studio.issue.shell_hostess_owner_intake_action_not_prohibited",
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
            &format!("studio.check.shell_hostess_owner_intake.prohibits_{action}"),
            package
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "owner intake explicitly preserves this Studio prohibition",
            "owner intake is missing this Studio prohibition",
            "studio.issue.shell_hostess_owner_intake_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_handoff_package_schema")
                    | Some("studio.issue.shell_handoff_manifest_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessOwnerIntakeStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessOwnerIntakeStatus::Blocked
    } else {
        StudioShellHostessOwnerIntakeStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessOwnerIntakeStatus::Ready => None,
        StudioShellHostessOwnerIntakeStatus::Blocked
        | StudioShellHostessOwnerIntakeStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let assignments =
        shell_hostess_owner_intake_assignments(package, status, issue_code.as_deref());
    let ready_assignment_count = assignments
        .iter()
        .filter(|assignment| {
            assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
        })
        .count();
    let blocked_assignment_count = assignments
        .iter()
        .filter(|assignment| {
            assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Blocked
        })
        .count();
    let hostess_ready_action_count = assignments
        .iter()
        .filter(|assignment| {
            assignment.owner == "rusty.hostess"
                && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
        })
        .count();
    let manifold_ready_action_count = assignments
        .iter()
        .filter(|assignment| {
            assignment.owner == "rusty.manifold"
                && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
        })
        .count();

    StudioShellHostessOwnerIntakeReport {
        schema_id: SHELL_HOSTESS_OWNER_INTAKE_SCHEMA.to_string(),
        source_package_schema: package.schema_id.clone(),
        package_path: package_path.map(|path| path.display().to_string()),
        selected_candidate_id: package.selected_candidate_id.clone(),
        candidate_manifest_path: package.candidate_manifest_path.clone(),
        review_path: package.review_path.clone(),
        handoff_manifest_path: package.handoff_manifest_path.clone(),
        manifest_id: package.manifest_id.clone(),
        project_id: package.project_id.clone(),
        project_revision: package.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.request_only".to_string(),
        intake_owner: "rusty.hostess".to_string(),
        handoff_owner: package.handoff_owner.clone(),
        review_owner: package.review_owner.clone(),
        command_session_authority: package.command_session_authority.clone(),
        install_launch_evidence_authority: package.install_launch_evidence_authority.clone(),
        studio_role: package.studio_role.clone(),
        handoff_ready_count: package.handoff_ready_count,
        handoff_failed_count: package.handoff_failed_count,
        handoff_missing_bundle_count: package.handoff_missing_bundle_count,
        acceptance_baseline_id: package.acceptance_baseline_id.clone(),
        acceptance_baseline_status: package.acceptance_baseline_status,
        acceptance_comparison_status: package.acceptance_comparison_status,
        export_package_baseline_id: package.export_package_baseline_id.clone(),
        export_package_baseline_status: package.export_package_baseline_status,
        export_package_comparison_status: package.export_package_comparison_status,
        source_owner_action_count: package.required_owner_actions.len(),
        ready_assignment_count,
        blocked_assignment_count,
        hostess_ready_action_count,
        manifold_ready_action_count,
        assignments,
        prohibited_actions: package.prohibited_actions.clone(),
        checks,
    }
}

fn shell_hostess_owner_intake_assignments(
    package: &StudioShellHostessHandoffPackageReport,
    status: StudioShellHostessOwnerIntakeStatus,
    issue_code: Option<&str>,
) -> Vec<StudioShellHostessOwnerIntakeAssignment> {
    package
        .required_owner_actions
        .iter()
        .map(|action| {
            let assignment_status = if status == StudioShellHostessOwnerIntakeStatus::Ready
                && action.status == StudioShellHostessHandoffPackageActionStatus::Ready
            {
                StudioShellHostessOwnerIntakeAssignmentStatus::Ready
            } else {
                StudioShellHostessOwnerIntakeAssignmentStatus::Blocked
            };
            StudioShellHostessOwnerIntakeAssignment {
                action_id: action.action_id.clone(),
                owner: action.owner.clone(),
                status: assignment_status,
                request_kind: shell_hostess_owner_intake_request_kind(&action.owner).to_string(),
                source: action.source.clone(),
                next_required_action: action.next_required_action.clone(),
                prohibited_in_studio: action.prohibited_in_studio,
                issue_code: (assignment_status
                    == StudioShellHostessOwnerIntakeAssignmentStatus::Blocked)
                    .then(|| {
                        action
                            .issue_code
                            .as_deref()
                            .or(issue_code)
                            .unwrap_or("studio.issue.shell_hostess_owner_intake_blocked")
                            .to_string()
                    }),
            }
        })
        .collect()
}

fn shell_hostess_owner_intake_request_kind(owner: &str) -> &'static str {
    match owner {
        "rusty.hostess" => "hostess_owner_action_request",
        "rusty.manifold" => "manifold_owner_review_request",
        _ => "owner_action_request",
    }
}
