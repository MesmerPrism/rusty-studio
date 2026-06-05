use super::super::*;

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
