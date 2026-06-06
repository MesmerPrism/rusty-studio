use super::*;

pub fn shell_hostess_staging_preview_for_owner_intake(
    intake: &StudioShellHostessOwnerIntakeReport,
    intake_path: Option<&Path>,
) -> StudioShellHostessStagingPreviewManifest {
    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.source_intake_schema",
        intake.schema_id == SHELL_HOSTESS_OWNER_INTAKE_SCHEMA,
        "source Hostess owner intake schema is supported",
        "source Hostess owner intake schema is unsupported",
        "studio.issue.shell_hostess_owner_intake_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.intake_path",
        intake_path.is_some(),
        "source Hostess owner intake has a durable path",
        "source Hostess owner intake path is missing",
        "studio.issue.shell_hostess_staging_preview_intake_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.intake_ready",
        intake.status == StudioShellHostessOwnerIntakeStatus::Ready,
        "source Hostess owner intake is ready",
        "source Hostess owner intake is not ready",
        intake
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_owner_intake_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.intake_execution_policy",
        intake.execution_policy == "not_executed.request_only",
        "source intake is request-only and not executed",
        "source intake execution policy is not request-only",
        "studio.issue.shell_hostess_owner_intake_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.package_path",
        intake.package_path.is_some(),
        "source intake names a Hostess handoff package",
        "source intake does not name a Hostess handoff package",
        "studio.issue.shell_hostess_staging_preview_package_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.handoff_manifest_path",
        intake.handoff_manifest_path.is_some(),
        "source intake names a shell handoff manifest",
        "source intake does not name a shell handoff manifest",
        "studio.issue.shell_hostess_staging_preview_handoff_manifest_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.runtime_command_authority",
        intake.command_session_authority.as_deref() == Some("rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.runtime_host_authority",
        intake.install_launch_evidence_authority.as_deref() == Some("rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.studio_role",
        intake.studio_role.as_deref() == Some("authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.source_intake_checks_pass",
        intake
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "source Hostess owner intake checks all pass",
        "source Hostess owner intake contains failed checks",
        "studio.issue.shell_hostess_owner_intake_failed_check",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.assignments_ready",
        !intake.assignments.is_empty()
            && intake.assignments.iter().all(|assignment| {
                assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
            }),
        "all owner-intake assignments are ready",
        "one or more owner-intake assignments are blocked",
        "studio.issue.shell_hostess_staging_preview_assignment_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.assignments_prohibited_in_studio",
        !intake.assignments.is_empty()
            && intake
                .assignments
                .iter()
                .all(|assignment| assignment.prohibited_in_studio),
        "all staging preview assignments remain prohibited in Studio",
        "one or more staging preview assignments are not prohibited in Studio",
        "studio.issue.shell_hostess_staging_preview_assignment_not_prohibited",
    );

    for action_id in [
        "hostess.review_release_candidate",
        "hostess.stage_generated_shells",
        "manifold.review_command_session_contract",
        "hostess.collect_install_launch_evidence",
    ] {
        push_check(
            &mut checks,
            &format!("studio.check.shell_hostess_staging_preview.has_{action_id}"),
            intake
                .assignments
                .iter()
                .any(|assignment| assignment.action_id == action_id),
            "source intake includes this downstream assignment",
            "source intake is missing this downstream assignment",
            "studio.issue.shell_hostess_staging_preview_assignment_missing",
        );
    }

    let handoff_manifest = intake.handoff_manifest_path.as_ref().and_then(|path| {
        match load_shell_handoff_manifest(Path::new(path)) {
            Ok(manifest) => Some(manifest),
            Err(error) => {
                checks.push(failed_hostess_staging_preview_check(
                    "studio.check.shell_hostess_staging_preview.handoff_manifest_load",
                    error.to_string(),
                    "studio.issue.shell_hostess_staging_preview_handoff_manifest_load_failed",
                ));
                None
            }
        }
    });
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.handoff_manifest_schema",
        handoff_manifest
            .as_ref()
            .is_some_and(|manifest| manifest.schema_id == SHELL_HANDOFF_MANIFEST_SCHEMA),
        "source handoff manifest schema is supported",
        "source handoff manifest schema is unsupported or unavailable",
        "studio.issue.shell_handoff_manifest_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.handoff_manifest_identity",
        handoff_manifest.as_ref().is_some_and(|manifest| {
            intake.manifest_id.as_deref() == Some(manifest.manifest_id.as_str())
                && intake.project_id.as_deref() == Some(manifest.project_id.as_str())
                && intake.project_revision == Some(manifest.project_revision)
        }),
        "source handoff manifest identity matches the owner intake",
        "source handoff manifest identity does not match the owner intake",
        "studio.issue.shell_hostess_staging_preview_handoff_manifest_identity_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.handoff_manifest_ready",
        handoff_manifest.as_ref().is_some_and(|manifest| {
            manifest.status == StudioValidationStatus::Pass
                && manifest.failed_count == 0
                && manifest.missing_bundle_count == 0
        }),
        "source handoff manifest is ready with no failed or missing bundles",
        "source handoff manifest has failed or missing bundles",
        "studio.issue.shell_hostess_staging_preview_handoff_manifest_blocked",
    );

    let export_package = handoff_manifest
        .as_ref()
        .map(shell_export_package_for_manifest);
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_preview.export_package_ready",
        export_package
            .as_ref()
            .is_some_and(|package| package.status == StudioShellExportPackageStatus::Ready),
        "derived export package has descriptor and template paths for every target",
        "derived export package is not ready for staging preview",
        export_package
            .as_ref()
            .and_then(|package| package.issue_code.as_deref())
            .unwrap_or("studio.issue.shell_hostess_staging_preview_export_package_blocked"),
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
            &format!("studio.check.shell_hostess_staging_preview.prohibits_{action}"),
            intake
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "staging preview explicitly preserves this Studio prohibition",
            "staging preview is missing this Studio prohibition",
            "studio.issue.shell_hostess_staging_preview_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_owner_intake_schema")
                    | Some("studio.issue.shell_handoff_manifest_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessStagingPreviewStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessStagingPreviewStatus::Blocked
    } else {
        StudioShellHostessStagingPreviewStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessStagingPreviewStatus::Ready => None,
        StudioShellHostessStagingPreviewStatus::Blocked
        | StudioShellHostessStagingPreviewStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let intake_artifact_path = intake_path.map(|path| path.display().to_string());
    let groups = shell_hostess_staging_preview_groups(
        intake,
        intake_artifact_path.as_deref(),
        export_package.as_ref(),
        status,
        issue_code.as_deref(),
    );
    let ready_group_count = groups
        .iter()
        .filter(|group| group.status == StudioShellHostessStagingPreviewGroupStatus::Ready)
        .count();
    let blocked_group_count = groups
        .iter()
        .filter(|group| group.status == StudioShellHostessStagingPreviewGroupStatus::Blocked)
        .count();
    let expected_artifact_count = groups
        .iter()
        .map(|group| group.expected_artifact_count)
        .sum();

    StudioShellHostessStagingPreviewManifest {
        schema_id: SHELL_HOSTESS_STAGING_PREVIEW_MANIFEST_SCHEMA.to_string(),
        source_intake_schema: intake.schema_id.clone(),
        source_handoff_manifest_schema: handoff_manifest
            .as_ref()
            .map(|manifest| manifest.schema_id.clone()),
        intake_path: intake_artifact_path,
        package_path: intake.package_path.clone(),
        handoff_manifest_path: intake.handoff_manifest_path.clone(),
        selected_candidate_id: intake.selected_candidate_id.clone(),
        manifest_id: intake.manifest_id.clone(),
        project_id: intake.project_id.clone(),
        project_revision: intake.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.preview_only".to_string(),
        staging_owner: "rusty.hostess".to_string(),
        command_session_authority: intake.command_session_authority.clone(),
        install_launch_evidence_authority: intake.install_launch_evidence_authority.clone(),
        studio_role: intake.studio_role.clone(),
        assignment_count: intake.assignments.len(),
        ready_assignment_count: intake.ready_assignment_count,
        blocked_assignment_count: intake.blocked_assignment_count,
        ready_group_count,
        blocked_group_count,
        expected_artifact_count,
        groups,
        prohibited_actions: intake.prohibited_actions.clone(),
        checks,
    }
}

fn shell_hostess_staging_preview_groups(
    intake: &StudioShellHostessOwnerIntakeReport,
    intake_artifact_path: Option<&str>,
    export_package: Option<&StudioShellExportPackageReport>,
    preview_status: StudioShellHostessStagingPreviewStatus,
    preview_issue_code: Option<&str>,
) -> Vec<StudioShellHostessStagingPreviewGroup> {
    intake
        .assignments
        .iter()
        .map(|assignment| {
            let expected_artifacts = shell_hostess_staging_preview_artifacts_for_assignment(
                assignment,
                intake,
                intake_artifact_path,
                export_package,
            );
            let status = if preview_status == StudioShellHostessStagingPreviewStatus::Ready
                && assignment.status == StudioShellHostessOwnerIntakeAssignmentStatus::Ready
                && !expected_artifacts.is_empty()
            {
                StudioShellHostessStagingPreviewGroupStatus::Ready
            } else {
                StudioShellHostessStagingPreviewGroupStatus::Blocked
            };
            let target_kinds = unique_strings(expected_artifacts.iter().filter_map(|artifact| {
                artifact
                    .target_kind
                    .map(shell_target_kind_label)
                    .map(str::to_string)
            }));
            let graph_ids = unique_strings(
                expected_artifacts
                    .iter()
                    .filter_map(|artifact| artifact.graph_id.clone()),
            );
            StudioShellHostessStagingPreviewGroup {
                action_id: assignment.action_id.clone(),
                owner: assignment.owner.clone(),
                request_kind: assignment.request_kind.clone(),
                route_kind: shell_hostess_staging_preview_route_kind(&assignment.action_id)
                    .to_string(),
                status,
                issue_code: (status == StudioShellHostessStagingPreviewGroupStatus::Blocked).then(
                    || {
                        assignment
                            .issue_code
                            .as_deref()
                            .or(preview_issue_code)
                            .unwrap_or("studio.issue.shell_hostess_staging_preview_blocked")
                            .to_string()
                    },
                ),
                source: assignment.source.clone(),
                next_required_action: assignment.next_required_action.clone(),
                prohibited_in_studio: assignment.prohibited_in_studio,
                expected_artifact_count: expected_artifacts.len(),
                target_kinds,
                graph_ids,
                expected_artifacts,
            }
        })
        .collect()
}

fn shell_hostess_staging_preview_artifacts_for_assignment(
    assignment: &StudioShellHostessOwnerIntakeAssignment,
    intake: &StudioShellHostessOwnerIntakeReport,
    intake_artifact_path: Option<&str>,
    export_package: Option<&StudioShellExportPackageReport>,
) -> Vec<StudioShellHostessStagingPreviewArtifact> {
    let mut artifacts = Vec::new();
    match assignment.action_id.as_str() {
        "hostess.review_release_candidate" => {
            push_optional_staging_artifact(
                &mut artifacts,
                "candidate_manifest",
                intake.candidate_manifest_path.as_deref(),
                None,
                None,
                None,
                None,
            );
            push_optional_staging_artifact(
                &mut artifacts,
                "release_candidate_review",
                intake.review_path.as_deref(),
                None,
                None,
                None,
                None,
            );
            push_optional_staging_artifact(
                &mut artifacts,
                "hostess_handoff_package",
                intake.package_path.as_deref(),
                None,
                None,
                None,
                None,
            );
            push_optional_staging_artifact(
                &mut artifacts,
                "hostess_owner_intake",
                intake_artifact_path,
                None,
                None,
                None,
                None,
            );
        }
        "hostess.stage_generated_shells" => {
            shell_hostess_staging_preview_common_artifacts(
                &mut artifacts,
                intake,
                intake_artifact_path,
            );
            for entry in export_package
                .into_iter()
                .flat_map(|package| package.entries.iter())
            {
                push_staging_artifact(
                    &mut artifacts,
                    "shell_bundle_dir",
                    &entry.bundle_dir,
                    Some(entry.target_kind),
                    Some(entry.graph_id.as_str()),
                    Some(entry.consumer_id.as_str()),
                    None,
                );
                if let Some(descriptor) = entry.descriptor.as_ref() {
                    push_staging_artifact(
                        &mut artifacts,
                        "shell_descriptor",
                        &descriptor.descriptor_path,
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        None,
                    );
                    let manifold_handoff_path = relative_output_path(
                        Path::new(&entry.bundle_dir),
                        &shell_manifold_handoff_artifact_path(&entry.graph_id),
                    );
                    push_staging_artifact(
                        &mut artifacts,
                        "manifold_shell_handoff",
                        &manifold_handoff_path.display().to_string(),
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        None,
                    );
                }
                if let Some(template) = entry.template_manifest.as_ref() {
                    push_staging_artifact(
                        &mut artifacts,
                        "shell_template_manifest",
                        &template.template_manifest_path,
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        entry.host_routes.install_route.as_deref(),
                    );
                }
            }
        }
        "manifold.review_command_session_contract" => {
            push_optional_staging_artifact(
                &mut artifacts,
                "shell_handoff_manifest",
                intake.handoff_manifest_path.as_deref(),
                None,
                None,
                None,
                Some("manifold.command_session_contract"),
            );
            for entry in export_package
                .into_iter()
                .flat_map(|package| package.entries.iter())
            {
                if let Some(descriptor) = entry.descriptor.as_ref() {
                    push_staging_artifact(
                        &mut artifacts,
                        "shell_descriptor",
                        &descriptor.descriptor_path,
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        Some("manifold.command_session_contract"),
                    );
                    let manifold_handoff_path = relative_output_path(
                        Path::new(&entry.bundle_dir),
                        &shell_manifold_handoff_artifact_path(&entry.graph_id),
                    );
                    push_staging_artifact(
                        &mut artifacts,
                        "manifold_shell_handoff",
                        &manifold_handoff_path.display().to_string(),
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        Some("manifold.shell_handoff_review"),
                    );
                }
                if let Some(template) = entry.template_manifest.as_ref() {
                    let route_hint = entry
                        .host_routes
                        .command_bridge
                        .as_deref()
                        .unwrap_or(entry.runtime_route_kind.as_str());
                    push_staging_artifact(
                        &mut artifacts,
                        "shell_template_manifest",
                        &template.template_manifest_path,
                        Some(entry.target_kind),
                        Some(entry.graph_id.as_str()),
                        Some(entry.consumer_id.as_str()),
                        Some(route_hint),
                    );
                }
            }
        }
        "hostess.collect_install_launch_evidence" => {
            push_optional_staging_artifact(
                &mut artifacts,
                "shell_handoff_manifest",
                intake.handoff_manifest_path.as_deref(),
                None,
                None,
                None,
                Some("hostess.install_launch_evidence"),
            );
            for entry in export_package
                .into_iter()
                .flat_map(|package| package.entries.iter())
            {
                push_staging_artifact(
                    &mut artifacts,
                    "shell_bundle_dir",
                    &entry.bundle_dir,
                    Some(entry.target_kind),
                    Some(entry.graph_id.as_str()),
                    Some(entry.consumer_id.as_str()),
                    entry.host_routes.evidence_pull_route.as_deref(),
                );
            }
        }
        _ => {
            shell_hostess_staging_preview_common_artifacts(
                &mut artifacts,
                intake,
                intake_artifact_path,
            );
        }
    }
    artifacts
}

fn shell_hostess_staging_preview_common_artifacts(
    artifacts: &mut Vec<StudioShellHostessStagingPreviewArtifact>,
    intake: &StudioShellHostessOwnerIntakeReport,
    intake_artifact_path: Option<&str>,
) {
    push_optional_staging_artifact(
        artifacts,
        "hostess_handoff_package",
        intake.package_path.as_deref(),
        None,
        None,
        None,
        None,
    );
    push_optional_staging_artifact(
        artifacts,
        "hostess_owner_intake",
        intake_artifact_path,
        None,
        None,
        None,
        None,
    );
    push_optional_staging_artifact(
        artifacts,
        "shell_handoff_manifest",
        intake.handoff_manifest_path.as_deref(),
        None,
        None,
        None,
        None,
    );
}

fn push_optional_staging_artifact(
    artifacts: &mut Vec<StudioShellHostessStagingPreviewArtifact>,
    artifact_kind: &str,
    path: Option<&str>,
    target_kind: Option<StudioShellTargetKind>,
    graph_id: Option<&str>,
    consumer_id: Option<&str>,
    route_hint: Option<&str>,
) {
    if let Some(path) = path {
        push_staging_artifact(
            artifacts,
            artifact_kind,
            path,
            target_kind,
            graph_id,
            consumer_id,
            route_hint,
        );
    }
}

fn push_staging_artifact(
    artifacts: &mut Vec<StudioShellHostessStagingPreviewArtifact>,
    artifact_kind: &str,
    path: &str,
    target_kind: Option<StudioShellTargetKind>,
    graph_id: Option<&str>,
    consumer_id: Option<&str>,
    route_hint: Option<&str>,
) {
    artifacts.push(StudioShellHostessStagingPreviewArtifact {
        artifact_kind: artifact_kind.to_string(),
        path: path.to_string(),
        target_kind,
        graph_id: graph_id.map(str::to_string),
        consumer_id: consumer_id.map(str::to_string),
        route_hint: route_hint.map(str::to_string),
    });
}

fn shell_hostess_staging_preview_route_kind(action_id: &str) -> &'static str {
    match action_id {
        "hostess.review_release_candidate" => "hostess.review.release_candidate",
        "hostess.stage_generated_shells" => "hostess.stage.generated_shells",
        "manifold.review_command_session_contract" => "manifold.review.command_session_contract",
        "hostess.collect_install_launch_evidence" => "hostess.collect.install_launch_evidence",
        _ => "owner.review.assignment",
    }
}

fn failed_hostess_staging_preview_check(
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
