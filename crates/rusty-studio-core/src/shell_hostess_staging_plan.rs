use super::*;

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
