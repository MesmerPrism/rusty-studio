use super::super::super::super::*;
use super::super::super::acceptance::*;
use super::super::ShellHandoffAcceptanceBaselineWriteResult;
use super::identity::next_shell_handoff_acceptance_baseline_archive_identity;

pub(crate) fn append_shell_handoff_acceptance_baseline_for_project_source(
    project_path: &Path,
) -> ShellHandoffAcceptanceBaselineWriteResult {
    let (report, bundle_root) = shell_handoff_acceptance_for_project_source(project_path)?;
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let existing_index = if index_path.is_file() {
        Some(
            load_shell_handoff_acceptance_baseline_index(&index_path)
                .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?,
        )
    } else {
        None
    };
    let (baseline_id, label) =
        next_shell_handoff_acceptance_baseline_archive_identity(&report, existing_index.as_ref());
    let checklist_path =
        shell_handoff_acceptance_baseline_archive_checklist_output_path(project_path, &baseline_id);
    save_json(&checklist_path, &report)
        .map_err(|error| format!("Shell handoff acceptance baseline save failed: {error}"))?;
    let baseline = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &report,
        &checklist_path,
        Some(&baseline_id),
        Some(&label),
    );
    let baseline_path =
        shell_handoff_acceptance_baseline_archive_manifest_output_path(project_path, &baseline_id);
    save_json(&baseline_path, &baseline).map_err(|error| {
        format!("Shell handoff acceptance baseline identity save failed: {error}")
    })?;
    let index = if let Some(index) = existing_index.as_ref() {
        append_shell_handoff_acceptance_baseline_index_manifests(
            index,
            vec![(baseline.clone(), Some(baseline_path.clone()))],
            Some(&baseline.baseline_id),
        )
    } else {
        shell_handoff_acceptance_baseline_index_for_manifests(
            vec![(baseline.clone(), Some(baseline_path.clone()))],
            Some(&baseline.baseline_id),
        )
    };
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell handoff acceptance baseline index save failed: {error}"))?;
    Ok((
        report,
        baseline,
        index,
        checklist_path,
        baseline_path,
        index_path,
        bundle_root,
    ))
}
