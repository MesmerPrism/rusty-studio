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

pub fn shell_hostess_staging_file_plan_for_preview(
    preview: &StudioShellHostessStagingPreviewManifest,
    preview_path: Option<&Path>,
) -> StudioShellHostessStagingFilePlan {
    let planned_files = shell_hostess_staging_planned_files(preview);
    let source_artifact_count: usize = preview
        .groups
        .iter()
        .map(|group| group.expected_artifact_count)
        .sum();
    let duplicate_artifact_count = source_artifact_count.saturating_sub(planned_files.len());
    let ready_preview_group_count = preview
        .groups
        .iter()
        .filter(|group| group.status == StudioShellHostessStagingPreviewGroupStatus::Ready)
        .count();
    let blocked_preview_group_count = preview
        .groups
        .iter()
        .filter(|group| group.status == StudioShellHostessStagingPreviewGroupStatus::Blocked)
        .count();

    let mut checks = Vec::new();
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.source_preview_schema",
        preview.schema_id == SHELL_HOSTESS_STAGING_PREVIEW_MANIFEST_SCHEMA,
        "source Hostess staging preview schema is supported",
        "source Hostess staging preview schema is unsupported",
        "studio.issue.shell_hostess_staging_preview_schema",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.preview_path",
        preview_path.is_some(),
        "source Hostess staging preview has a durable path",
        "source Hostess staging preview path is missing",
        "studio.issue.shell_hostess_staging_file_plan_preview_path_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.preview_ready",
        preview.status == StudioShellHostessStagingPreviewStatus::Ready,
        "source Hostess staging preview is ready",
        "source Hostess staging preview is not ready",
        preview
            .issue_code
            .as_deref()
            .unwrap_or("studio.issue.shell_hostess_staging_preview_blocked"),
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.preview_execution_policy",
        preview.execution_policy == "not_executed.preview_only",
        "source preview is preview-only and not executed",
        "source preview execution policy is not preview-only",
        "studio.issue.shell_hostess_staging_preview_execution_policy",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.staging_owner",
        preview.staging_owner == "rusty.hostess",
        "Hostess remains staging owner",
        "staging owner must remain rusty.hostess",
        "studio.issue.staging_owner_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.runtime_command_authority",
        preview.command_session_authority.as_deref() == Some("rusty.manifold"),
        "Manifold remains command/session authority",
        "command/session authority must remain rusty.manifold",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.runtime_host_authority",
        preview.install_launch_evidence_authority.as_deref() == Some("rusty.hostess"),
        "Hostess remains install/launch/evidence authority",
        "install/launch/evidence authority must remain rusty.hostess",
        "studio.issue.runtime_authority_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.studio_role",
        preview.studio_role.as_deref() == Some("authoring.export_planning"),
        "Studio remains authoring/export-planning authority",
        "Studio role must remain authoring.export_planning",
        "studio.issue.studio_role_mismatch",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.source_preview_checks_pass",
        preview
            .checks
            .iter()
            .all(|check| check.status == StudioValidationStatus::Pass),
        "source Hostess staging preview checks all pass",
        "source Hostess staging preview contains failed checks",
        "studio.issue.shell_hostess_staging_preview_failed_check",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.preview_groups_ready",
        !preview.groups.is_empty()
            && preview
                .groups
                .iter()
                .all(|group| group.status == StudioShellHostessStagingPreviewGroupStatus::Ready),
        "all source preview groups are ready",
        "one or more source preview groups are blocked",
        "studio.issue.shell_hostess_staging_file_plan_group_blocked",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.preview_groups_prohibited_in_studio",
        !preview.groups.is_empty()
            && preview
                .groups
                .iter()
                .all(|group| group.prohibited_in_studio),
        "all staging file-plan groups remain prohibited in Studio",
        "one or more staging file-plan groups are not prohibited in Studio",
        "studio.issue.shell_hostess_staging_file_plan_group_not_prohibited",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.artifacts_present",
        source_artifact_count > 0 && !planned_files.is_empty(),
        "source preview exposes artifacts to plan",
        "source preview does not expose artifacts to plan",
        "studio.issue.shell_hostess_staging_file_plan_artifacts_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.destination_paths_unique",
        shell_hostess_staging_destination_paths_are_unique(&planned_files),
        "planned destination paths are unique after deduplication",
        "planned destination paths collide after deduplication",
        "studio.issue.shell_hostess_staging_file_plan_destination_collision",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.destination_paths_safe",
        planned_files
            .iter()
            .all(|file| is_safe_relative_manifest_path(&file.destination_path)),
        "planned destination paths are portable relative paths",
        "one or more planned destination paths are unsafe",
        "studio.issue.shell_hostess_staging_file_plan_destination_path_unsafe",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.has_shared_request",
        planned_files.iter().any(|file| file.target_kind.is_none()),
        "file plan includes a shared Hostess staging request",
        "file plan is missing shared Hostess staging artifacts",
        "studio.issue.shell_hostess_staging_file_plan_shared_request_missing",
    );
    push_check(
        &mut checks,
        "studio.check.shell_hostess_staging_file_plan.has_target_requests",
        planned_files.iter().any(|file| file.target_kind.is_some()),
        "file plan includes per-target Hostess staging requests",
        "file plan is missing per-target Hostess staging artifacts",
        "studio.issue.shell_hostess_staging_file_plan_target_request_missing",
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
            &format!("studio.check.shell_hostess_staging_file_plan.prohibits_{action}"),
            preview
                .prohibited_actions
                .iter()
                .any(|candidate| candidate == action),
            "staging file plan explicitly preserves this Studio prohibition",
            "staging file plan is missing this Studio prohibition",
            "studio.issue.shell_hostess_staging_file_plan_prohibited_action_missing",
        );
    }

    let has_failed_check = checks
        .iter()
        .any(|check| check.status == StudioValidationStatus::Fail);
    let has_rejected_check = checks.iter().any(|check| {
        check.status == StudioValidationStatus::Fail
            && matches!(
                check.issue_code.as_deref(),
                Some("studio.issue.shell_hostess_staging_preview_schema")
            )
    });
    let status = if has_rejected_check {
        StudioShellHostessStagingFilePlanStatus::Rejected
    } else if has_failed_check {
        StudioShellHostessStagingFilePlanStatus::Blocked
    } else {
        StudioShellHostessStagingFilePlanStatus::Ready
    };
    let issue_code = match status {
        StudioShellHostessStagingFilePlanStatus::Ready => None,
        StudioShellHostessStagingFilePlanStatus::Blocked
        | StudioShellHostessStagingFilePlanStatus::Rejected => {
            first_failed_validation_check_issue_code(&checks)
        }
    };
    let requests =
        shell_hostess_staging_file_requests(planned_files, status, issue_code.as_deref());
    let ready_request_count = requests
        .iter()
        .filter(|request| request.status == StudioShellHostessStagingFileRequestStatus::Ready)
        .count();
    let blocked_request_count = requests
        .iter()
        .filter(|request| request.status == StudioShellHostessStagingFileRequestStatus::Blocked)
        .count();
    let target_request_count = requests
        .iter()
        .filter(|request| request.target_kind.is_some())
        .count();
    let shared_request_count = requests.len().saturating_sub(target_request_count);
    let planned_file_count = requests
        .iter()
        .map(|request| request.planned_file_count)
        .sum();

    StudioShellHostessStagingFilePlan {
        schema_id: SHELL_HOSTESS_STAGING_FILE_PLAN_SCHEMA.to_string(),
        source_preview_schema: preview.schema_id.clone(),
        preview_path: preview_path.map(|path| path.display().to_string()),
        intake_path: preview.intake_path.clone(),
        package_path: preview.package_path.clone(),
        handoff_manifest_path: preview.handoff_manifest_path.clone(),
        selected_candidate_id: preview.selected_candidate_id.clone(),
        manifest_id: preview.manifest_id.clone(),
        project_id: preview.project_id.clone(),
        project_revision: preview.project_revision,
        status,
        issue_code,
        execution_policy: "not_executed.dry_run_only".to_string(),
        staging_owner: "rusty.hostess".to_string(),
        command_session_authority: preview.command_session_authority.clone(),
        install_launch_evidence_authority: preview.install_launch_evidence_authority.clone(),
        studio_role: preview.studio_role.clone(),
        preview_group_count: preview.groups.len(),
        ready_preview_group_count,
        blocked_preview_group_count,
        source_artifact_count,
        planned_file_count,
        duplicate_artifact_count,
        request_count: requests.len(),
        ready_request_count,
        blocked_request_count,
        target_request_count,
        shared_request_count,
        requests,
        prohibited_actions: preview.prohibited_actions.clone(),
        checks,
    }
}

#[derive(Clone, Debug)]
struct StagingPlannedFileBuilder {
    artifact_kind: String,
    source_path: String,
    destination_path: String,
    target_kind: Option<StudioShellTargetKind>,
    graph_id: Option<String>,
    consumer_id: Option<String>,
    route_hints: BTreeSet<String>,
    source_action_ids: BTreeSet<String>,
    source_route_kinds: BTreeSet<String>,
}

fn shell_hostess_staging_planned_files(
    preview: &StudioShellHostessStagingPreviewManifest,
) -> Vec<StudioShellHostessStagingPlannedFile> {
    let mut files: BTreeMap<String, StagingPlannedFileBuilder> = BTreeMap::new();
    for group in &preview.groups {
        for artifact in &group.expected_artifacts {
            let key = shell_hostess_staging_artifact_key(artifact);
            let destination_path = shell_hostess_staging_destination_path(artifact);
            let entry = files
                .entry(key)
                .or_insert_with(|| StagingPlannedFileBuilder {
                    artifact_kind: artifact.artifact_kind.clone(),
                    source_path: artifact.path.clone(),
                    destination_path,
                    target_kind: artifact.target_kind,
                    graph_id: artifact.graph_id.clone(),
                    consumer_id: artifact.consumer_id.clone(),
                    route_hints: BTreeSet::new(),
                    source_action_ids: BTreeSet::new(),
                    source_route_kinds: BTreeSet::new(),
                });
            if let Some(route_hint) = artifact.route_hint.as_ref() {
                entry.route_hints.insert(route_hint.clone());
            }
            entry.source_action_ids.insert(group.action_id.clone());
            entry.source_route_kinds.insert(group.route_kind.clone());
        }
    }
    files
        .into_values()
        .map(|file| StudioShellHostessStagingPlannedFile {
            artifact_kind: file.artifact_kind,
            source_path: file.source_path,
            destination_path: file.destination_path,
            target_kind: file.target_kind,
            graph_id: file.graph_id,
            consumer_id: file.consumer_id,
            route_hints: file.route_hints.into_iter().collect(),
            source_action_ids: file.source_action_ids.into_iter().collect(),
            source_route_kinds: file.source_route_kinds.into_iter().collect(),
        })
        .collect()
}

fn shell_hostess_staging_file_requests(
    planned_files: Vec<StudioShellHostessStagingPlannedFile>,
    plan_status: StudioShellHostessStagingFilePlanStatus,
    plan_issue_code: Option<&str>,
) -> Vec<StudioShellHostessStagingFileRequest> {
    let mut grouped: BTreeMap<String, Vec<StudioShellHostessStagingPlannedFile>> = BTreeMap::new();
    for file in planned_files {
        grouped
            .entry(shell_hostess_staging_target_key(&file))
            .or_default()
            .push(file);
    }
    grouped
        .into_iter()
        .map(|(target_key, mut planned_files)| {
            planned_files.sort_by(|left, right| left.destination_path.cmp(&right.destination_path));
            let target_kind = planned_files.iter().find_map(|file| file.target_kind);
            let graph_id = planned_files.iter().find_map(|file| file.graph_id.clone());
            let consumer_id = planned_files
                .iter()
                .find_map(|file| file.consumer_id.clone());
            let action_ids = unique_strings(
                planned_files
                    .iter()
                    .flat_map(|file| file.source_action_ids.iter().cloned()),
            );
            let route_kinds = unique_strings(
                planned_files
                    .iter()
                    .flat_map(|file| file.source_route_kinds.iter().cloned()),
            );
            let status = if plan_status == StudioShellHostessStagingFilePlanStatus::Ready
                && !planned_files.is_empty()
            {
                StudioShellHostessStagingFileRequestStatus::Ready
            } else {
                StudioShellHostessStagingFileRequestStatus::Blocked
            };
            let destination_root =
                shell_hostess_staging_destination_root(target_kind, graph_id.as_deref());
            StudioShellHostessStagingFileRequest {
                request_id: format!(
                    "hostess.staging_file_plan.{}",
                    shell_hostess_staging_request_id_segment(&target_key)
                ),
                request_kind: if target_kind.is_some() {
                    "hostess_target_staging_file_plan".to_string()
                } else {
                    "hostess_shared_staging_file_plan".to_string()
                },
                owner: "rusty.hostess".to_string(),
                status,
                issue_code: (status == StudioShellHostessStagingFileRequestStatus::Blocked).then(
                    || {
                        plan_issue_code
                            .unwrap_or("studio.issue.shell_hostess_staging_file_plan_blocked")
                            .to_string()
                    },
                ),
                target_key,
                target_kind,
                graph_id,
                consumer_id,
                destination_root,
                action_ids,
                route_kinds,
                planned_file_count: planned_files.len(),
                planned_files,
            }
        })
        .collect()
}

fn shell_hostess_staging_destination_paths_are_unique(
    planned_files: &[StudioShellHostessStagingPlannedFile],
) -> bool {
    let mut seen = BTreeSet::new();
    planned_files
        .iter()
        .all(|file| seen.insert(file.destination_path.clone()))
}

fn shell_hostess_staging_artifact_key(
    artifact: &StudioShellHostessStagingPreviewArtifact,
) -> String {
    format!(
        "{}|{}|{}|{}|{}",
        artifact.artifact_kind,
        artifact.path,
        artifact
            .target_kind
            .map(shell_target_kind_label)
            .unwrap_or("shared"),
        artifact.graph_id.as_deref().unwrap_or("shared"),
        artifact.consumer_id.as_deref().unwrap_or("shared")
    )
}

fn shell_hostess_staging_target_key(file: &StudioShellHostessStagingPlannedFile) -> String {
    match file.target_kind {
        Some(target_kind) => format!(
            "{}/{}",
            shell_target_kind_label(target_kind),
            file.graph_id
                .as_deref()
                .map(shell_hostess_staging_safe_segment)
                .unwrap_or_else(|| "unknown_graph".to_string())
        ),
        None => "shared".to_string(),
    }
}

fn shell_hostess_staging_destination_root(
    target_kind: Option<StudioShellTargetKind>,
    graph_id: Option<&str>,
) -> String {
    match target_kind {
        Some(target_kind) => format!(
            "hostess-staging/targets/{}/{}",
            shell_target_kind_label(target_kind),
            graph_id
                .map(shell_hostess_staging_safe_segment)
                .unwrap_or_else(|| "unknown_graph".to_string())
        ),
        None => "hostess-staging/shared".to_string(),
    }
}

fn shell_hostess_staging_destination_path(
    artifact: &StudioShellHostessStagingPreviewArtifact,
) -> String {
    let root =
        shell_hostess_staging_destination_root(artifact.target_kind, artifact.graph_id.as_deref());
    match artifact.target_kind {
        Some(_) => match artifact.artifact_kind.as_str() {
            "shell_bundle_dir" => format!("{root}/bundle"),
            "shell_descriptor" => {
                format!(
                    "{root}/descriptor/{}",
                    source_path_file_name(&artifact.path)
                )
            }
            "manifold_shell_handoff" => {
                format!("{root}/manifold/{}", source_path_file_name(&artifact.path))
            }
            "shell_template_manifest" => {
                format!("{root}/template/{}", source_path_file_name(&artifact.path))
            }
            other => format!(
                "{root}/{}/{}",
                shell_hostess_staging_safe_segment(other),
                source_path_file_name(&artifact.path)
            ),
        },
        None => shell_hostess_staging_shared_destination_path(&root, artifact),
    }
}

fn shell_hostess_staging_shared_destination_path(
    root: &str,
    artifact: &StudioShellHostessStagingPreviewArtifact,
) -> String {
    match artifact.artifact_kind.as_str() {
        "candidate_manifest" => format!("{root}/release-candidate/candidate-manifest.json"),
        "release_candidate_review" => {
            format!("{root}/release-candidate/release-candidate-review.json")
        }
        "hostess_handoff_package" => format!("{root}/hostess/hostess-handoff-package.json"),
        "hostess_owner_intake" => format!("{root}/hostess/hostess-owner-intake.json"),
        "shell_handoff_manifest" => format!("{root}/handoffs/shell-handoffs.json"),
        other => format!(
            "{root}/{}/{}",
            shell_hostess_staging_safe_segment(other),
            source_path_file_name(&artifact.path)
        ),
    }
}

fn source_path_file_name(path: &str) -> String {
    path.replace('\\', "/")
        .split('/')
        .filter(|segment| !segment.is_empty())
        .next_back()
        .map(shell_hostess_staging_safe_segment)
        .unwrap_or_else(|| "artifact.json".to_string())
}

fn shell_hostess_staging_request_id_segment(value: &str) -> String {
    value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => character,
            '/' | '.' => '.',
            _ => '_',
        })
        .collect()
}

fn shell_hostess_staging_safe_segment(value: &str) -> String {
    let segment = value
        .chars()
        .map(|character| match character {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '.' | '_' | '-' => character,
            _ => '_',
        })
        .collect::<String>();
    if segment.is_empty() {
        "unknown".to_string()
    } else {
        segment
    }
}

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
