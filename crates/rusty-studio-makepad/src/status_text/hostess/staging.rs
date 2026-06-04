use super::super::*;

pub(crate) fn shell_hostess_staging_preview_status(
    report: &StudioShellHostessStagingPreviewManifest,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_preview_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let intake_path = report.intake_path.as_deref().unwrap_or("unknown");
    let package_path = report.package_path.as_deref().unwrap_or("unknown");
    let handoff_path = report.handoff_manifest_path.as_deref().unwrap_or("unknown");
    let groups = report
        .groups
        .iter()
        .map(|group| {
            let group_status = shell_hostess_staging_preview_group_status_label(group.status);
            let issue = group.issue_code.as_deref().unwrap_or("none");
            let target_kinds = if group.target_kinds.is_empty() {
                "none".to_string()
            } else {
                group.target_kinds.join(", ")
            };
            let graph_ids = if group.graph_ids.is_empty() {
                "none".to_string()
            } else {
                group.graph_ids.join(", ")
            };
            format!(
                "{} route {} [{}] owner {}; artifacts {}; targets {}; graphs {}; issue {}",
                group.action_id,
                group.route_kind,
                group_status,
                group.owner,
                group.expected_artifact_count,
                target_kinds,
                graph_ids,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    format!(
        "shell Hostess staging preview {status}; selected {selected}; issue {issue}\n  preview: {}\n  intake: {}\n  package: {}\n  handoff manifest: {}\n  project: {} rev {}\n  assignments ready {}; blocked {}; groups ready {}; blocked {}; artifacts {}\n  authority: command {}; host {}; studio {}; policy {}; staging owner {}\n  groups:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        intake_path,
        package_path,
        handoff_path,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.ready_assignment_count,
        report.blocked_assignment_count,
        report.ready_group_count,
        report.blocked_group_count,
        report.expected_artifact_count,
        report
            .command_session_authority
            .as_deref()
            .unwrap_or("unknown"),
        report
            .install_launch_evidence_authority
            .as_deref()
            .unwrap_or("unknown"),
        report.studio_role.as_deref().unwrap_or("unknown"),
        report.execution_policy,
        report.staging_owner,
        if groups.is_empty() {
            "none".to_string()
        } else {
            groups
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}

pub(crate) fn shell_hostess_staging_file_plan_status(
    report: &StudioShellHostessStagingFilePlan,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_file_plan_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let preview_path = report.preview_path.as_deref().unwrap_or("unknown");
    let requests = report
        .requests
        .iter()
        .map(|request| {
            let request_status = shell_hostess_staging_file_request_status_label(request.status);
            let issue = request.issue_code.as_deref().unwrap_or("none");
            let target_kind = request
                .target_kind
                .map(shell_target_kind_label)
                .unwrap_or("shared");
            let graph = request.graph_id.as_deref().unwrap_or("shared");
            let routes = if request.route_kinds.is_empty() {
                "none".to_string()
            } else {
                request.route_kinds.join(", ")
            };
            format!(
                "{} [{}] kind {}; target {}; graph {}; files {}; root {}; routes {}; issue {}",
                request.request_id,
                request_status,
                request.request_kind,
                target_kind,
                graph,
                request.planned_file_count,
                request.destination_root,
                routes,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    format!(
        "shell Hostess staging file plan {status}; selected {selected}; issue {issue}\n  file plan: {}\n  preview: {}\n  project: {} rev {}\n  preview groups ready {}; blocked {}; source artifacts {}; planned files {}; duplicates {}\n  requests ready {}; blocked {}; target {}; shared {}\n  authority: command {}; host {}; studio {}; policy {}; staging owner {}\n  requests:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        preview_path,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.ready_preview_group_count,
        report.blocked_preview_group_count,
        report.source_artifact_count,
        report.planned_file_count,
        report.duplicate_artifact_count,
        report.ready_request_count,
        report.blocked_request_count,
        report.target_request_count,
        report.shared_request_count,
        report
            .command_session_authority
            .as_deref()
            .unwrap_or("unknown"),
        report
            .install_launch_evidence_authority
            .as_deref()
            .unwrap_or("unknown"),
        report.studio_role.as_deref().unwrap_or("unknown"),
        report.execution_policy,
        report.staging_owner,
        if requests.is_empty() {
            "none".to_string()
        } else {
            requests
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}

pub(crate) fn shell_hostess_staging_handoff_status(
    report: &StudioShellHostessStagingHandoffEnvelope,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_handoff_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let selected = report.selected_candidate_id.as_deref().unwrap_or("none");
    let file_plan_path = report.file_plan_path.as_deref().unwrap_or("unknown");
    let instructions = report
        .owner_instructions
        .iter()
        .map(|instruction| {
            let instruction_status =
                shell_hostess_staging_handoff_instruction_status_label(instruction.status);
            let issue = instruction.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] owner {}; kind {}; route {}; next {}; prohibited in Studio {}; issue {}",
                instruction.instruction_id,
                instruction_status,
                instruction.owner,
                instruction.instruction_kind,
                instruction.route_kind,
                instruction.next_required_action,
                if instruction.prohibited_in_studio {
                    "yes"
                } else {
                    "no"
                },
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let requests = report
        .request_summaries
        .iter()
        .map(|request| {
            let request_status = shell_hostess_staging_file_request_status_label(request.status);
            let target_kind = request
                .target_kind
                .map(shell_target_kind_label)
                .unwrap_or("shared");
            format!(
                "{} [{}] target {}; files {}; root {}",
                request.request_id,
                request_status,
                target_kind,
                request.planned_file_count,
                request.destination_root
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    format!(
        "shell Hostess staging handoff {status}; selected {selected}; issue {issue}\n  envelope: {}\n  file plan: {}\n  envelope id: {}\n  project: {} rev {}\n  checksum: {} ({})\n  requests ready {}; blocked {}; target {}; shared {}; planned files {}\n  instructions ready {}; blocked {}\n  authority: command {}; host {}; studio {}; policy {}; handoff owner {}; staging owner {}\n  requests:\n  {}\n  instructions:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        file_plan_path,
        report.envelope_id,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.provenance.plan_checksum,
        report.provenance.checksum_algorithm,
        report.ready_request_count,
        report.blocked_request_count,
        report.target_request_count,
        report.shared_request_count,
        report.planned_file_count,
        report.ready_instruction_count,
        report.blocked_instruction_count,
        report
            .command_session_authority
            .as_deref()
            .unwrap_or("unknown"),
        report
            .install_launch_evidence_authority
            .as_deref()
            .unwrap_or("unknown"),
        report.studio_role.as_deref().unwrap_or("unknown"),
        report.execution_policy,
        report.handoff_owner,
        report.staging_owner,
        if requests.is_empty() {
            "none".to_string()
        } else {
            requests
        },
        if instructions.is_empty() {
            "none".to_string()
        } else {
            instructions
        },
        prohibited,
        report.checks.len(),
        failed_checks
    )
}
