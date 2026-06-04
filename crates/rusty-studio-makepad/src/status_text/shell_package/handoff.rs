use super::super::*;

pub(crate) fn shell_handoff_status(report: &StudioShellHandoffReport, output_dir: &Path) -> String {
    let status = validation_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    if report.status == StudioValidationStatus::Pass {
        let args = if report.consumer_args.is_empty() {
            "none".to_string()
        } else {
            report.consumer_args.join(" ")
        };
        let authority = report
            .runtime_authority
            .as_ref()
            .map(|authority| {
                format!(
                    "{} / {} / {}",
                    authority.command_session_authority,
                    authority.install_launch_evidence_authority,
                    authority.studio_role
                )
            })
            .unwrap_or_else(|| "none".to_string());
        return format!(
            "shell handoff {status}; issue {issue}\n  graph: {}\n  output: {}\n  consumer: {}\n  target: {}\n  args: {}\n  authority: {}",
            report.graph_id,
            output_dir.display(),
            report.consumer_id,
            shell_target_kind_label(report.target_kind),
            args,
            authority
        );
    }
    format!(
        "shell handoff {status}; issue {issue}\n  graph: {}\n  output: {}\n  target: {}\n  message: {}",
        report.graph_id,
        output_dir.display(),
        shell_target_kind_label(report.target_kind),
        report.message
    )
}

pub(crate) fn shell_handoff_readiness_status(
    report: &StudioShellHandoffReadinessReport,
    bundle_root: &Path,
) -> String {
    let status = validation_status_label(report.status);
    let target_rows = report
        .target_summaries
        .iter()
        .map(|summary| {
            let ready_path = summary
                .template_index_paths
                .first()
                .map(|path| format!("; templates {path}"))
                .unwrap_or_default();
            let missing_path = summary
                .missing_bundle_dirs
                .first()
                .map(|path| format!("; missing bundle {path}"))
                .unwrap_or_default();
            format!(
                "{}: ready {}/{}; missing {}; packages {}; modules {}; shells {}{}{}",
                shell_target_kind_label(summary.target_kind),
                summary.ready_count,
                summary.graph_count,
                summary.missing_bundle_count,
                summary.package_count,
                summary.module_count,
                summary.operator_shell_count,
                ready_path,
                missing_path
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = validation_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            format!(
                "{} [{}] profile {}; packages {}; modules {}; shell {}; -> {} / {}; issue {}",
                entry.graph_id,
                shell_target_kind_label(entry.target_kind),
                entry.target_host_profile,
                entry.package_count,
                entry.module_count,
                entry.operator_shell_count,
                entry.consumer_id,
                entry_status,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    format!(
        "handoff readiness {status}; ready {}/{}; failed {}; missing {}\n  root: {}\n  targets:\n  {}\n  graphs:\n  {}",
        report.ready_count,
        report.graph_count,
        report.failed_count,
        report.missing_bundle_count,
        bundle_root.display(),
        if target_rows.is_empty() {
            "none".to_string()
        } else {
            target_rows
        },
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}

pub(crate) fn shell_runbook_status(
    report: &StudioShellRunbookReport,
    bundle_root: &Path,
) -> String {
    let status = shell_runbook_status_label(report.status);
    let issue = report.issue_code.as_deref().unwrap_or("none");
    let prohibited = if report.prohibited_actions.is_empty() {
        "none".to_string()
    } else {
        report.prohibited_actions.join(", ")
    };
    let target_rows = report
        .target_summaries
        .iter()
        .map(|target| {
            let consumers = if target.consumer_ids.is_empty() {
                "none".to_string()
            } else {
                target.consumer_ids.join(", ")
            };
            let owners = if target.responsible_owners.is_empty() {
                "none".to_string()
            } else {
                target.responsible_owners.join(", ")
            };
            let routes = if target.runtime_route_kinds.is_empty() {
                "none".to_string()
            } else {
                target.runtime_route_kinds.join(", ")
            };
            let issues = if target.issue_codes.is_empty() {
                "none".to_string()
            } else {
                target.issue_codes.join(", ")
            };
            format!(
                "{}: ready {}; blocked {}; rejected {}; consumers {}; owners {}; routes {}; issues {}",
                shell_target_kind_label(target.target_kind),
                target.ready_count,
                target.blocked_count,
                target.rejected_count,
                consumers,
                owners,
                routes,
                issues
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let rows = report
        .entries
        .iter()
        .take(6)
        .map(|entry| {
            let entry_status = shell_runbook_status_label(entry.status);
            let issue = entry.issue_code.as_deref().unwrap_or("none");
            let install = entry.host_routes.install_route.as_deref().unwrap_or("none");
            let launch = entry.host_routes.launch_route.as_deref().unwrap_or("none");
            let bridge = entry.host_routes.command_bridge.as_deref().unwrap_or("none");
            let evidence = entry
                .host_routes
                .evidence_pull_route
                .as_deref()
                .unwrap_or("none");
            let cli = if entry.cli_request.is_empty() {
                "none".to_string()
            } else {
                entry.cli_request.join(" ")
            };
            format!(
                "{} [{}] target {}; owner {}; action {}; policy {}; route {}; install {}; launch {}; bridge {}; evidence {}; cli {}; issue {}",
                entry.graph_id,
                entry_status,
                shell_target_kind_label(entry.target_kind),
                entry.responsible_owner,
                entry.next_required_action,
                entry.execution_policy,
                entry.runtime_route_kind,
                install,
                launch,
                bridge,
                evidence,
                cli,
                issue
            )
        })
        .collect::<Vec<_>>()
        .join("\n  ");

    format!(
        "shell runbook {status}; ready {}; blocked {}; rejected {}; issue {issue}\n  root: {}\n  bundle root: {}\n  prohibited: {}\n  targets:\n  {}\n  entries:\n  {}",
        report.ready_count,
        report.blocked_count,
        report.rejected_count,
        report.bundle_root,
        bundle_root.display(),
        prohibited,
        if target_rows.is_empty() {
            "none".to_string()
        } else {
            target_rows
        },
        if rows.is_empty() {
            "none".to_string()
        } else {
            rows
        }
    )
}
