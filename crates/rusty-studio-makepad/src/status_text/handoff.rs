use super::*;

pub(crate) fn shell_handoff_manifest_status(
    manifest: &StudioShellHandoffManifest,
    output_path: &Path,
) -> String {
    let status = validation_status_label(manifest.status);
    let target_rows = manifest
        .targets
        .iter()
        .map(|target| {
            let ready_path = target
                .ready_bundle_dirs
                .first()
                .map(|path| format!("; ready {path}"))
                .unwrap_or_default();
            let missing_path = target
                .missing_bundle_dirs
                .first()
                .map(|path| format!("; missing {path}"))
                .unwrap_or_default();
            format!(
                "{}: ready {}/{}; failed {}; missing {}; templates {}{}{}",
                shell_target_kind_label(target.target_kind),
                target.ready_count,
                target.graph_count,
                target.failed_count,
                target.missing_bundle_count,
                target.template_index_paths.len(),
                ready_path,
                missing_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    format!(
        "handoff manifest {status}; ready {}/{}; failed {}; missing {}\n  path: {}\n  authority: {} / {} / {}\n  targets:\n  {}",
        manifest.ready_count,
        manifest.graph_count,
        manifest.failed_count,
        manifest.missing_bundle_count,
        output_path.display(),
        manifest.runtime_authority.command_session_authority,
        manifest.runtime_authority.install_launch_evidence_authority,
        manifest.runtime_authority.studio_role,
        if target_rows.is_empty() {
            "none".to_string()
        } else {
            target_rows
        }
    )
}

pub(crate) fn shell_handoff_acceptance_status(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    bundle_root: &Path,
) -> String {
    let status = shell_handoff_acceptance_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let failed_intake_checks = report
        .intake_checks
        .iter()
        .filter(|check| check.status == StudioValidationStatus::Fail)
        .count();
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_handoff_acceptance_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let failed_checks = entry
                .checks
                .iter()
                .filter(|check| check.status == StudioValidationStatus::Fail)
                .count();
            format!(
                "{} [{}] -> {} / {}; action {}; route {}; owners {}; failed {}; issue {}",
                entry.graph_id,
                shell_target_kind_label(entry.target_kind),
                entry.consumer_id,
                entry_status,
                entry.next_required_action,
                entry.runtime_route_kind,
                shell_handoff_acceptance_owner_summary(report, &entry.graph_id),
                failed_checks,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    format!(
        "handoff acceptance {status}; ready {}; blocked {}; rejected {}; issue {issue}\n  root: {}\n  prohibited: {}\n  intake checks: {}; failed {}\n  entries:\n  {}",
        report.ready_count,
        report.blocked_count,
        report.rejected_count,
        bundle_root.display(),
        prohibited,
        report.intake_checks.len(),
        failed_intake_checks,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

pub(crate) fn shell_handoff_acceptance_baseline_status(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    checklist_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection =
        summarize_shell_handoff_acceptance_baseline_index_selection(index, Some(index_path), None);
    format!(
        "acceptance baseline written\n  baseline: {} ({})\n  identity: {}\n  checklist: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        checklist_path.display(),
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path),
        shell_handoff_acceptance_status(report, bundle_root)
    )
}

pub(crate) fn shell_handoff_acceptance_baseline_append_status(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    checklist_path: &Path,
    baseline_path: &Path,
    index_path: &Path,
    bundle_root: &Path,
) -> String {
    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "acceptance baseline archived\n  baseline: {} ({})\n  identity: {}\n  checklist: {}\n{}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        checklist_path.display(),
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path),
        shell_handoff_acceptance_status(report, bundle_root)
    )
}

pub(crate) fn shell_handoff_acceptance_baseline_index_status(
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    index_path: &Path,
) -> String {
    let default = index.default_baseline_id.as_deref().unwrap_or("none");
    let projects = if index.project_ids.is_empty() {
        "none".to_string()
    } else {
        index.project_ids.join(", ")
    };
    let manifests = if index.manifest_ids.is_empty() {
        "none".to_string()
    } else {
        index.manifest_ids.join(", ")
    };
    let rows = index
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let status = shell_handoff_acceptance_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry.baseline_manifest_path.as_deref().unwrap_or("unknown");
            format!(
                "{} [{}] project {} rev {}; ready {}; blocked {}; rejected {}; manifest {}; issue {}",
                entry.baseline_id,
                status,
                entry.project_id,
                entry.project_revision,
                entry.ready_count,
                entry.blocked_count,
                entry.rejected_count,
                manifest_path,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "baseline index slots {}; default {}; ready {}; blocked {}; rejected {}\n  index: {}\n  projects: {}\n  manifests: {}\n  entries:\n  {}",
        index.baseline_count,
        default,
        index.ready_baseline_count,
        index.blocked_baseline_count,
        index.rejected_baseline_count,
        index_path.display(),
        projects,
        manifests,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

pub(crate) fn shell_handoff_acceptance_baseline_selection_status(
    report: &StudioShellHandoffAcceptanceBaselineSelectionReport,
) -> String {
    let status = shell_handoff_acceptance_baseline_selection_status_label(report.status);
    let requested = report.requested_baseline_id.as_deref().unwrap_or("none");
    let default = report.default_baseline_id.as_deref().unwrap_or("none");
    let selected = report.selected_baseline_id.as_deref().unwrap_or("none");
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let index_path = report.index_path.as_deref().unwrap_or("not saved");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_handoff_acceptance_status_label(entry.status);
            let entry_issue = entry.issue_code.as_deref().unwrap_or("none");
            let manifest_path = entry.baseline_manifest_path.as_deref().unwrap_or("unknown");
            let selected_flag = if entry.selected { "yes" } else { "no" };
            let default_flag = if entry.default { "yes" } else { "no" };
            format!(
                "{} [{}] selected {}; default {}; ready {}; blocked {}; rejected {}; manifest {}; issue {}",
                entry.baseline_id,
                entry_status,
                selected_flag,
                default_flag,
                entry.ready_count,
                entry.blocked_count,
                entry.rejected_count,
                manifest_path,
                entry_issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "baseline selection {status}; requested {requested}; default {default}; selected {selected}; slots {}; ready {}; blocked {}; rejected {}; issue {issue}\n  index: {}\n  entries:\n  {}",
        report.baseline_count,
        report.ready_baseline_count,
        report.blocked_baseline_count,
        report.rejected_baseline_count,
        index_path,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

pub(crate) fn shell_handoff_acceptance_summary_status(
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection =
        summarize_shell_handoff_acceptance_baseline_index_selection(index, Some(index_path), None);
    let summary = &baseline.summary;
    let status = shell_handoff_acceptance_status_label(summary.status);
    let issue = summary.issue_code.as_deref().unwrap_or("none");
    let target_rows = summary
        .targets
        .iter()
        .map(|target| {
            let consumers = if target.consumer_ids.is_empty() {
                "none".to_string()
            } else {
                target.consumer_ids.join(", ")
            };
            let routes = if target.route_kinds.is_empty() {
                "none".to_string()
            } else {
                target.route_kinds.join(", ")
            };
            let issues = if target.issue_codes.is_empty() {
                "none".to_string()
            } else {
                target.issue_codes.join(", ")
            };
            format!(
                "{}: ready {}/{}; blocked {}; rejected {}; consumers {}; routes {}; issues {}",
                shell_target_kind_label(target.target_kind),
                target.ready_count,
                target.graph_count,
                target.blocked_count,
                target.rejected_count,
                consumers,
                routes,
                issues
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    format!(
        "acceptance baseline summary {status}; baseline {} ({}); project {} rev {}; manifest {}; ready {}; blocked {}; rejected {}; entries {}; issue {issue}\n  identity: {}\n  checklist: {}\n  intake checks: {}; failed {}\n  targets:\n  {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        summary.project_id,
        summary.project_revision,
        summary.manifest_id,
        summary.ready_count,
        summary.blocked_count,
        summary.rejected_count,
        summary.entry_count,
        baseline_path.display(),
        baseline.checklist_path,
        summary.intake_check_count,
        summary.failed_intake_check_count,
        if target_rows.is_empty() {
            "none".to_string()
        } else {
            target_rows
        },
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path)
    )
}

pub(crate) fn shell_handoff_acceptance_baseline_promote_status(
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "acceptance baseline default promoted\n  baseline: {} ({})\n  identity: {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path)
    )
}

pub(crate) fn shell_handoff_acceptance_baseline_select_status(
    baseline: &StudioShellHandoffAcceptanceBaselineManifest,
    index: &StudioShellHandoffAcceptanceBaselineIndex,
    baseline_path: &Path,
    index_path: &Path,
) -> String {
    let selection = summarize_shell_handoff_acceptance_baseline_index_selection(
        index,
        Some(index_path),
        Some(&baseline.baseline_id),
    );
    format!(
        "acceptance baseline default selected\n  baseline: {} ({})\n  identity: {}\n{}\n{}",
        baseline.baseline_id,
        baseline.label,
        baseline_path.display(),
        shell_handoff_acceptance_baseline_selection_status(&selection),
        shell_handoff_acceptance_baseline_index_status(index, index_path)
    )
}

pub(crate) fn shell_handoff_acceptance_owner_summary(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    graph_id: &str,
) -> String {
    let Some(entry) = report
        .entries
        .iter()
        .find(|entry| entry.graph_id == graph_id)
    else {
        return "none".to_string();
    };
    ["rusty.manifold", "rusty.hostess", "rusty.studio"]
        .iter()
        .map(|owner| {
            let owner_checks = entry
                .checks
                .iter()
                .filter(|check| check.owner.as_str() == *owner)
                .collect::<Vec<_>>();
            let status = if owner_checks.is_empty() {
                "none"
            } else if owner_checks
                .iter()
                .any(|check| check.status == StudioValidationStatus::Fail)
            {
                "fail"
            } else {
                "pass"
            };
            format!("{owner}:{status}")
        })
        .collect::<Vec<_>>()
        .join(", ")
}

pub(crate) fn shell_handoff_acceptance_comparison_status(
    report: &StudioShellHandoffAcceptanceComparisonReport,
    baseline_path: &Path,
    bundle_root: &Path,
) -> String {
    let status = shell_handoff_acceptance_comparison_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let baseline_id = report.baseline_id.as_deref().unwrap_or("unnamed");
    let baseline_label = report.baseline_label.as_deref().unwrap_or("unlabeled");
    let baseline_checklist = report
        .baseline_checklist_path
        .as_deref()
        .unwrap_or("unknown");
    let baseline_index_path = report.baseline_index_path.as_deref().unwrap_or("not used");
    let baseline_index_default = report
        .baseline_index_default_baseline_id
        .as_deref()
        .unwrap_or("none");
    let baseline_index_selected = report
        .baseline_index_selected_baseline_id
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
            let target = entry
                .target_kind
                .map(shell_target_kind_label)
                .unwrap_or("unknown");
            let baseline = entry
                .baseline_status
                .map(shell_handoff_acceptance_status_label)
                .unwrap_or("missing");
            let candidate = entry
                .candidate_status
                .map(shell_handoff_acceptance_status_label)
                .unwrap_or("missing");
            let change = shell_handoff_acceptance_comparison_change_label(entry.change);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let route = entry
                .candidate_route_kind
                .as_deref()
                .or(entry.baseline_route_kind.as_deref())
                .unwrap_or("unknown");
            format!(
                "{} [{}] {baseline}->{candidate}; change {change}; delta {}; route {}; issue {}",
                entry.graph_id, target, entry.score_delta, route, issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "handoff acceptance comparison {status}; ready {}->{}, delta {}; blocked {}->{}, delta {}; rejected {}->{}, delta {}; issue {issue}\n  baseline: {} ({})\n  baseline source: {} rev {}; manifest {}\n  candidate: {} rev {}; manifest {}\n  baseline identity: {}\n  baseline checklist: {}\n  baseline index: {}; default {}; selected {}\n  current root: {}\n  checks: {}; failed {}\n  entries:\n  {}",
        report.baseline_ready_count,
        report.candidate_ready_count,
        report.ready_delta,
        report.baseline_blocked_count,
        report.candidate_blocked_count,
        report.blocked_delta,
        report.baseline_rejected_count,
        report.candidate_rejected_count,
        report.rejected_delta,
        baseline_id,
        baseline_label,
        report.baseline_project_id,
        report.baseline_project_revision,
        report.baseline_manifest_id,
        report.candidate_project_id,
        report.candidate_project_revision,
        report.candidate_manifest_id,
        baseline_path.display(),
        baseline_checklist,
        baseline_index_path,
        baseline_index_default,
        baseline_index_selected,
        bundle_root.display(),
        report.checks.len(),
        failed_checks,
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
