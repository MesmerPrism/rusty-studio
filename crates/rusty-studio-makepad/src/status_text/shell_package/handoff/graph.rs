use super::super::super::*;

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
