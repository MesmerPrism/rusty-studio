use super::*;

pub fn shell_runbook_for_project(
    project: &StudioProject,
    base_dir: Option<&Path>,
    bundle_root: &Path,
) -> StudioShellRunbookReport {
    let manifest = shell_handoff_manifest_for_project(project, base_dir, bundle_root);
    shell_runbook_for_manifest(&manifest)
}

pub fn shell_runbook_for_manifest(
    manifest: &StudioShellHandoffManifest,
) -> StudioShellRunbookReport {
    let intake = shell_handoff_intake_for_manifest(manifest);
    let entries = intake
        .entries
        .iter()
        .map(shell_runbook_entry)
        .collect::<Vec<_>>();
    let ready_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellRunbookStatus::Ready)
        .count();
    let blocked_count = entries
        .iter()
        .filter(|entry| entry.status == StudioShellRunbookStatus::Blocked)
        .count();
    let rejected_count = if intake.status == StudioShellHandoffIntakeStatus::Rejected {
        1
    } else {
        entries
            .iter()
            .filter(|entry| entry.status == StudioShellRunbookStatus::Rejected)
            .count()
    };
    let status = if intake.status == StudioShellHandoffIntakeStatus::Rejected {
        StudioShellRunbookStatus::Rejected
    } else if blocked_count > 0 || entries.is_empty() {
        StudioShellRunbookStatus::Blocked
    } else {
        StudioShellRunbookStatus::Ready
    };
    let issue_code = match status {
        StudioShellRunbookStatus::Ready => None,
        StudioShellRunbookStatus::Blocked => entries
            .iter()
            .find(|entry| entry.status == StudioShellRunbookStatus::Blocked)
            .and_then(|entry| entry.issue_code.clone()),
        StudioShellRunbookStatus::Rejected => intake.issue_code.clone(),
    };

    StudioShellRunbookReport {
        schema_id: SHELL_RUNBOOK_REPORT_SCHEMA.to_string(),
        source_manifest_schema: manifest.schema_id.clone(),
        source_intake_schema: intake.schema_id.clone(),
        manifest_id: manifest.manifest_id.clone(),
        project_id: manifest.project_id.clone(),
        project_revision: manifest.project_revision,
        bundle_root: manifest.bundle_root.clone(),
        status,
        issue_code,
        ready_count,
        blocked_count,
        rejected_count,
        target_summaries: shell_runbook_target_summaries(&entries),
        prohibited_actions: shell_handoff_acceptance_prohibited_actions(),
        entries,
    }
}

fn shell_runbook_entry(entry: &StudioShellHandoffIntakeEntry) -> StudioShellRunbookEntry {
    let (host_routes, route_status, route_issue_code) = shell_runbook_host_routes(entry);
    let status = if route_status == StudioValidationStatus::Pass
        && entry.decision == StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner
    {
        StudioShellRunbookStatus::Ready
    } else {
        StudioShellRunbookStatus::Blocked
    };
    let issue_code = match status {
        StudioShellRunbookStatus::Ready => None,
        StudioShellRunbookStatus::Blocked => route_issue_code
            .clone()
            .or_else(|| entry.issue_code.clone())
            .or_else(|| Some("studio.issue.shell_runbook_blocked".to_string())),
        StudioShellRunbookStatus::Rejected => entry.issue_code.clone(),
    };
    let responsible_owner = if status == StudioShellRunbookStatus::Ready {
        entry.install_launch_evidence_authority.clone()
    } else {
        "rusty.studio".to_string()
    };
    let cli_request =
        if status == StudioShellRunbookStatus::Ready && !entry.consumer_args.is_empty() {
            ["cargo", "run", "-p", entry.consumer_id.as_str(), "--"]
                .into_iter()
                .map(str::to_string)
                .chain(entry.consumer_args.iter().cloned())
                .collect()
        } else {
            Vec::new()
        };

    StudioShellRunbookEntry {
        export_bundle_id: entry.export_bundle_id.clone(),
        graph_id: entry.graph_id.clone(),
        display_name: entry.display_name.clone(),
        target_host_profile: entry.target_host_profile.clone(),
        target_kind: entry.target_kind,
        handoff_kind: entry.handoff_kind,
        status,
        issue_code,
        decision: entry.decision,
        responsible_owner,
        handoff_request_kind: entry.handoff_request_kind.clone(),
        runtime_route_kind: entry.runtime_route_kind.clone(),
        next_required_action: entry.next_required_action.clone(),
        execution_policy: "not_executed.request_only".to_string(),
        command_session_authority: entry.command_session_authority.clone(),
        install_launch_evidence_authority: entry.install_launch_evidence_authority.clone(),
        studio_role: entry.studio_role.clone(),
        consumer_id: entry.consumer_id.clone(),
        bundle_dir: entry.bundle_dir.clone(),
        template_index_path: entry.template_index_path.clone(),
        consumer_args: entry.consumer_args.clone(),
        cli_request,
        host_routes,
        route_status,
        route_issue_code,
        package_ids: entry.package_ids.clone(),
        module_ids: entry.module_ids.clone(),
        operator_shell_ids: entry.operator_shell_ids.clone(),
    }
}

fn shell_runbook_host_routes(
    entry: &StudioShellHandoffIntakeEntry,
) -> (
    StudioShellHostRoutes,
    StudioValidationStatus,
    Option<String>,
) {
    if entry.decision != StudioShellHandoffIntakeDecision::ReadyForRuntimeOwner {
        return (
            empty_shell_host_routes(),
            StudioValidationStatus::Fail,
            entry.issue_code.clone(),
        );
    }

    let index = match load_shell_template_index(Path::new(&entry.template_index_path)) {
        Ok(index) => index,
        Err(_) => {
            return (
                empty_shell_host_routes(),
                StudioValidationStatus::Fail,
                Some("studio.issue.shell_runbook_template_index_load_failed".to_string()),
            );
        }
    };
    let Some(template_entry) = index
        .templates
        .iter()
        .find(|template| template.graph_id == entry.graph_id)
    else {
        return (
            empty_shell_host_routes(),
            StudioValidationStatus::Fail,
            Some("studio.issue.shell_runbook_template_missing".to_string()),
        );
    };
    let template_path =
        relative_output_path(Path::new(&entry.bundle_dir), &template_entry.template_path);
    match load_shell_template_manifest(&template_path) {
        Ok(template) => (template.host_routes, StudioValidationStatus::Pass, None),
        Err(_) => (
            empty_shell_host_routes(),
            StudioValidationStatus::Fail,
            Some("studio.issue.shell_runbook_template_manifest_load_failed".to_string()),
        ),
    }
}

pub(crate) fn empty_shell_host_routes() -> StudioShellHostRoutes {
    StudioShellHostRoutes {
        app_id: None,
        install_route: None,
        launch_route: None,
        command_bridge: None,
        evidence_pull_route: None,
    }
}

fn shell_runbook_target_summaries(
    entries: &[StudioShellRunbookEntry],
) -> Vec<StudioShellRunbookTargetSummary> {
    shell_target_kinds()
        .iter()
        .filter_map(|target_kind| shell_runbook_target_summary(entries, *target_kind))
        .collect()
}

fn shell_runbook_target_summary(
    entries: &[StudioShellRunbookEntry],
    target_kind: StudioShellTargetKind,
) -> Option<StudioShellRunbookTargetSummary> {
    let target_entries = entries
        .iter()
        .filter(|entry| entry.target_kind == target_kind)
        .collect::<Vec<_>>();
    if target_entries.is_empty() {
        return None;
    }

    Some(StudioShellRunbookTargetSummary {
        target_kind,
        ready_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellRunbookStatus::Ready)
            .count(),
        blocked_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellRunbookStatus::Blocked)
            .count(),
        rejected_count: target_entries
            .iter()
            .filter(|entry| entry.status == StudioShellRunbookStatus::Rejected)
            .count(),
        graph_ids: unique_strings(target_entries.iter().map(|entry| entry.graph_id.clone())),
        consumer_ids: unique_strings(target_entries.iter().map(|entry| entry.consumer_id.clone())),
        responsible_owners: unique_strings(
            target_entries
                .iter()
                .map(|entry| entry.responsible_owner.clone()),
        ),
        runtime_route_kinds: unique_strings(
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
