use super::super::*;

pub(crate) fn shell_hostess_staging_acceptance_status(
    report: &StudioShellHostessStagingAcceptanceChecklistReport,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_acceptance_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let handoff_path = report.handoff_path.as_deref().unwrap_or("unknown");
    let file_plan_path = report.file_plan_path.as_deref().unwrap_or("unknown");
    let entries = report
        .entries
        .iter()
        .map(|entry| {
            let entry_status = shell_hostess_staging_acceptance_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] owner {}; kind {}; route {}; next {}; prohibited in Studio {}; issue {}",
                entry.item_id,
                entry_status,
                entry.owner,
                entry.item_kind,
                entry.route_kind,
                entry.next_required_action,
                if entry.prohibited_in_studio {
                    "yes"
                } else {
                    "no"
                },
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
        .handoff_checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    format!(
        "shell Hostess staging acceptance {status}; issue {issue}\n  checklist: {}\n  handoff: {}\n  file plan: {}\n  envelope id: {}\n  project: {} rev {}\n  checksum: {} ({})\n  requests ready {}; blocked {}; instructions ready {}; blocked {}\n  items ready {}; blocked {}; rejected {}\n  authority: command {}; host {}; studio {}; policy {}; checklist owner {}; handoff owner {}; staging owner {}\n  entries:\n  {}\n  prohibited: {}\n  checks: {}; failed {}",
        output_path.display(),
        handoff_path,
        file_plan_path,
        report.envelope_id,
        report.project_id.as_deref().unwrap_or("unknown"),
        report
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.plan_checksum,
        report.checksum_algorithm,
        report.ready_request_count,
        report.blocked_request_count,
        report.ready_instruction_count,
        report.blocked_instruction_count,
        report.ready_item_count,
        report.blocked_item_count,
        report.rejected_item_count,
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
        report.checklist_owner,
        report.handoff_owner,
        report.staging_owner,
        if entries.is_empty() {
            "none".to_string()
        } else {
            entries
        },
        prohibited,
        report.handoff_checks.len(),
        failed_checks
    )
}

pub(crate) fn shell_hostess_staging_acceptance_append_status(
    acceptance: &StudioShellHostessStagingAcceptanceManifest,
    index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_hostess_staging_acceptance_index_selection(
        index,
        Some(index_path),
        Some(&acceptance.acceptance_id),
    );
    format!(
        "Hostess staging acceptance archived\n  acceptance: {} ({})\n  identity: {}\n  checklist: {}\n{}\n{}",
        acceptance.acceptance_id,
        acceptance.label,
        acceptance_path.display(),
        acceptance.checklist_path,
        shell_hostess_staging_acceptance_selection_status(&selection),
        shell_hostess_staging_acceptance_index_status(index, index_path)
    )
}

pub(crate) fn shell_hostess_staging_acceptance_summary_status(
    acceptance: &StudioShellHostessStagingAcceptanceManifest,
    index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_path: &Path,
    index_path: &Path,
) -> String {
    let selection =
        summarize_shell_hostess_staging_acceptance_index_selection(index, Some(index_path), None);
    let status = shell_hostess_staging_acceptance_status_label(acceptance.status);
    let issue = acceptance.issue_code.as_deref().unwrap_or("none");
    format!(
        "Hostess staging acceptance summary {status}; acceptance {} ({}); project {} rev {}; envelope {}; issue {issue}\n  identity: {}\n  checklist: {}\n  items ready {}; blocked {}; rejected {}; requests {}; instructions {}\n  authority: command {}; host {}; studio {}; policy {}; checklist owner {}; handoff owner {}; staging owner {}\n{}\n{}",
        acceptance.acceptance_id,
        acceptance.label,
        acceptance.project_id.as_deref().unwrap_or("unknown"),
        acceptance
            .project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        acceptance.envelope_id,
        acceptance_path.display(),
        acceptance.checklist_path,
        acceptance.ready_item_count,
        acceptance.blocked_item_count,
        acceptance.rejected_item_count,
        acceptance.request_count,
        acceptance.instruction_count,
        acceptance
            .command_session_authority
            .as_deref()
            .unwrap_or("unknown"),
        acceptance
            .install_launch_evidence_authority
            .as_deref()
            .unwrap_or("unknown"),
        acceptance.studio_role.as_deref().unwrap_or("unknown"),
        acceptance.execution_policy,
        acceptance.checklist_owner,
        acceptance.handoff_owner,
        acceptance.staging_owner,
        shell_hostess_staging_acceptance_selection_status(&selection),
        shell_hostess_staging_acceptance_index_status(index, index_path)
    )
}

pub(crate) fn shell_hostess_staging_acceptance_promote_status(
    acceptance: &StudioShellHostessStagingAcceptanceManifest,
    index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_hostess_staging_acceptance_index_selection(
        index,
        Some(index_path),
        Some(&acceptance.acceptance_id),
    );
    format!(
        "Hostess staging acceptance default promoted\n  acceptance: {} ({})\n  identity: {}\n{}\n{}",
        acceptance.acceptance_id,
        acceptance.label,
        acceptance_path.display(),
        shell_hostess_staging_acceptance_selection_status(&selection),
        shell_hostess_staging_acceptance_index_status(index, index_path)
    )
}

pub(crate) fn shell_hostess_staging_acceptance_select_status(
    acceptance: &StudioShellHostessStagingAcceptanceManifest,
    index: &StudioShellHostessStagingAcceptanceIndex,
    acceptance_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_hostess_staging_acceptance_index_selection(
        index,
        Some(index_path),
        Some(&acceptance.acceptance_id),
    );
    format!(
        "Hostess staging acceptance default selected\n  acceptance: {} ({})\n  identity: {}\n{}\n{}",
        acceptance.acceptance_id,
        acceptance.label,
        acceptance_path.display(),
        shell_hostess_staging_acceptance_selection_status(&selection),
        shell_hostess_staging_acceptance_index_status(index, index_path)
    )
}

pub(crate) fn shell_hostess_staging_acceptance_index_status(
    index: &StudioShellHostessStagingAcceptanceIndex,
    index_path: &Path,
) -> String {
    let default = index.default_acceptance_id.as_deref().unwrap_or("none");
    let projects = if index.project_ids.is_empty() {
        "none".to_string()
    } else {
        index.project_ids.join(", ")
    };
    let envelopes = if index.envelope_ids.is_empty() {
        "none".to_string()
    } else {
        index.envelope_ids.join(", ")
    };
    let rows = index
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let status = shell_hostess_staging_acceptance_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry
                .acceptance_manifest_path
                .as_deref()
                .unwrap_or("unknown");
            format!(
                "{} [{}] project {} rev {}; envelope {}; items ready {}; blocked {}; rejected {}; manifest {}; issue {}",
                entry.acceptance_id,
                status,
                entry.project_id.as_deref().unwrap_or("unknown"),
                entry
                    .project_revision
                    .map(|revision| revision.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
                entry.envelope_id,
                entry.ready_item_count,
                entry.blocked_item_count,
                entry.rejected_item_count,
                manifest_path,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "Hostess staging acceptance index slots {}; default {}; ready {}; blocked {}; rejected {}\n  index: {}\n  projects: {}\n  envelopes: {}\n  entries:\n  {}",
        index.acceptance_count,
        default,
        index.ready_acceptance_count,
        index.blocked_acceptance_count,
        index.rejected_acceptance_count,
        index_path.display(),
        projects,
        envelopes,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

pub(crate) fn shell_hostess_staging_acceptance_selection_status(
    report: &StudioShellHostessStagingAcceptanceSelectionReport,
) -> String {
    let status = shell_hostess_staging_acceptance_selection_status_label(report.status);
    let requested = report.requested_acceptance_id.as_deref().unwrap_or("none");
    let default = report.default_acceptance_id.as_deref().unwrap_or("none");
    let selected = report.selected_acceptance_id.as_deref().unwrap_or("none");
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let index_path = report.index_path.as_deref().unwrap_or("not saved");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_hostess_staging_acceptance_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry
                .acceptance_manifest_path
                .as_deref()
                .unwrap_or("unknown");
            let selected_flag = if entry.selected { "yes" } else { "no" };
            let default_flag = if entry.default { "yes" } else { "no" };
            format!(
                "{} [{}] selected {}; default {}; items ready {}; blocked {}; rejected {}; manifest {}; issue {}",
                entry.acceptance_id,
                entry_status,
                selected_flag,
                default_flag,
                entry.ready_item_count,
                entry.blocked_item_count,
                entry.rejected_item_count,
                manifest_path,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "Hostess staging acceptance selection {status}; requested {requested}; default {default}; selected {selected}; slots {}; ready {}; blocked {}; rejected {}; issue {issue}\n  index: {}\n  entries:\n  {}",
        report.acceptance_count,
        report.ready_acceptance_count,
        report.blocked_acceptance_count,
        report.rejected_acceptance_count,
        index_path,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

pub(crate) fn shell_hostess_staging_acceptance_comparison_status(
    report: &StudioShellHostessStagingAcceptanceComparisonReport,
    acceptance_path: &Path,
    output_path: &Path,
) -> String {
    let status = shell_hostess_staging_acceptance_comparison_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let acceptance_id = report
        .baseline_acceptance_id
        .as_deref()
        .unwrap_or("unnamed");
    let acceptance_label = report.baseline_label.as_deref().unwrap_or("unlabeled");
    let baseline_checklist = report
        .baseline_checklist_path
        .as_deref()
        .unwrap_or("unknown");
    let index_path = report.baseline_index_path.as_deref().unwrap_or("not used");
    let index_default = report
        .baseline_index_default_acceptance_id
        .as_deref()
        .unwrap_or("none");
    let index_selected = report
        .baseline_index_selected_acceptance_id
        .as_deref()
        .unwrap_or("none");
    let failed_checks = report
        .checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let baseline = entry
                .baseline_status
                .map(shell_hostess_staging_acceptance_status_label)
                .unwrap_or("missing");
            let candidate = entry
                .candidate_status
                .map(shell_hostess_staging_acceptance_status_label)
                .unwrap_or("missing");
            let change = shell_hostess_staging_acceptance_comparison_change_label(entry.change);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let route = entry
                .candidate_route_kind
                .as_deref()
                .or(entry.baseline_route_kind.as_deref())
                .unwrap_or("unknown");
            format!(
                "{} owner {}; {baseline}->{candidate}; change {change}; delta {}; route {}; issue {}",
                entry.item_id, entry.owner, entry.score_delta, route, issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "Hostess staging acceptance comparison {status}; ready {}->{}, delta {}; blocked {}->{}, delta {}; rejected {}->{}, delta {}; issue {issue}\n  acceptance: {} ({})\n  baseline source: project {} rev {}; envelope {}; manifest {}\n  candidate source: project {} rev {}; envelope {}; manifest {}\n  baseline identity: {}\n  baseline checklist: {}\n  baseline index: {}; default {}; selected {}\n  comparison: {}\n  checks: {}; failed {}\n  entries:\n  {}",
        report.baseline_ready_item_count,
        report.candidate_ready_item_count,
        report.ready_item_delta,
        report.baseline_blocked_item_count,
        report.candidate_blocked_item_count,
        report.blocked_item_delta,
        report.baseline_rejected_item_count,
        report.candidate_rejected_item_count,
        report.rejected_item_delta,
        acceptance_id,
        acceptance_label,
        report.baseline_project_id.as_deref().unwrap_or("unknown"),
        report
            .baseline_project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.baseline_envelope_id,
        report.baseline_manifest_id.as_deref().unwrap_or("unknown"),
        report.candidate_project_id.as_deref().unwrap_or("unknown"),
        report
            .candidate_project_revision
            .map(|revision| revision.to_string())
            .unwrap_or_else(|| "unknown".to_string()),
        report.candidate_envelope_id,
        report.candidate_manifest_id.as_deref().unwrap_or("unknown"),
        acceptance_path.display(),
        baseline_checklist,
        index_path,
        index_default,
        index_selected,
        output_path.display(),
        report.checks.len(),
        failed_checks,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
