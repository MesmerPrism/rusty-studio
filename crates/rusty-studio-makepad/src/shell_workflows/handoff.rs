use super::*;

pub(crate) fn write_shell_handoff_manifest_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHandoffManifest, PathBuf), String> {
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let bundle_root = selected_shell_bundle_root_dir(project_path);
    let manifest =
        shell_handoff_manifest_for_project(&project, project_path.parent(), &bundle_root);
    let output_path = shell_handoff_manifest_output_path(project_path);
    save_json(&output_path, &manifest)
        .map_err(|error| format!("Shell handoff manifest save failed: {error}"))?;
    Ok((manifest, output_path))
}

pub(crate) fn shell_handoff_acceptance_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHandoffAcceptanceChecklistReport, PathBuf), String> {
    let project =
        load_project(project_path).map_err(|error| format!("Project reload failed: {error}"))?;
    let bundle_root = selected_shell_bundle_root_dir(project_path);
    let report = shell_handoff_acceptance_checklist_for_project(
        &project,
        project_path.parent(),
        &bundle_root,
    );
    Ok((report, bundle_root))
}

pub(crate) fn write_shell_handoff_acceptance_baseline_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceChecklistReport,
        StudioShellHandoffAcceptanceBaselineManifest,
        StudioShellHandoffAcceptanceBaselineIndex,
        PathBuf,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let (report, bundle_root) = shell_handoff_acceptance_for_project_source(project_path)?;
    let output_path = shell_handoff_acceptance_checklist_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell handoff acceptance baseline save failed: {error}"))?;
    let baseline =
        shell_handoff_acceptance_baseline_manifest_for_checklist(&report, &output_path, None, None);
    let baseline_path = shell_handoff_acceptance_baseline_manifest_output_path(project_path);
    save_json(&baseline_path, &baseline).map_err(|error| {
        format!("Shell handoff acceptance baseline identity save failed: {error}")
    })?;
    let index = shell_handoff_acceptance_baseline_index_for_manifests(
        vec![(baseline.clone(), Some(baseline_path.clone()))],
        Some(&baseline.baseline_id),
    );
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell handoff acceptance baseline index save failed: {error}"))?;
    Ok((
        report,
        baseline,
        index,
        output_path,
        baseline_path,
        index_path,
        bundle_root,
    ))
}

pub(crate) fn append_shell_handoff_acceptance_baseline_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceChecklistReport,
        StudioShellHandoffAcceptanceBaselineManifest,
        StudioShellHandoffAcceptanceBaselineIndex,
        PathBuf,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
> {
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

pub(crate) fn next_shell_handoff_acceptance_baseline_archive_identity(
    report: &StudioShellHandoffAcceptanceChecklistReport,
    index: Option<&StudioShellHandoffAcceptanceBaselineIndex>,
) -> (String, String) {
    let status = shell_handoff_acceptance_status_label(report.status);
    let base_id = format!(
        "{}.rev{}.{}",
        report.project_id, report.project_revision, status
    );
    let next_slot = index
        .map(|index| {
            index
                .entries
                .iter()
                .filter(|entry| {
                    entry.baseline_id == base_id
                        || entry
                            .baseline_id
                            .strip_prefix(base_id.as_str())
                            .is_some_and(|suffix| suffix.starts_with(".archive"))
                })
                .count()
                + 1
        })
        .unwrap_or(1);
    let baseline_id = if next_slot == 1 {
        base_id
    } else {
        format!("{base_id}.archive{next_slot}")
    };
    let label = if next_slot == 1 {
        format!(
            "{} revision {} {} acceptance baseline",
            report.project_id, report.project_revision, status
        )
    } else {
        format!(
            "{} revision {} {} acceptance baseline archive {}",
            report.project_id, report.project_revision, status, next_slot
        )
    };
    (baseline_id, label)
}

pub(crate) fn shell_handoff_acceptance_baseline_summary_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceBaselineManifest,
        StudioShellHandoffAcceptanceBaselineIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let baseline_path = shell_handoff_acceptance_baseline_manifest_output_path(project_path);
    let baseline = load_shell_handoff_acceptance_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Baseline acceptance identity load failed: {error}"))?;
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let index = load_shell_handoff_acceptance_baseline_index(&index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    Ok((baseline, index, baseline_path, index_path))
}

pub(crate) fn promote_shell_handoff_acceptance_baseline_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceBaselineManifest,
        StudioShellHandoffAcceptanceBaselineIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let baseline_path = shell_handoff_acceptance_baseline_manifest_output_path(project_path);
    let baseline = load_shell_handoff_acceptance_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Baseline acceptance identity load failed: {error}"))?;
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let index = load_shell_handoff_acceptance_baseline_index(&index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    let promoted =
        promote_shell_handoff_acceptance_baseline_index_default(&index, &baseline.baseline_id)
            .ok_or_else(|| {
                format!(
                    "Baseline acceptance index does not contain baseline {}",
                    baseline.baseline_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Baseline acceptance index save failed: {error}"))?;
    Ok((baseline, promoted, baseline_path, index_path))
}

pub(crate) fn select_next_shell_handoff_acceptance_baseline_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceBaselineManifest,
        StudioShellHandoffAcceptanceBaselineIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let index = load_shell_handoff_acceptance_baseline_index(&index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    let baseline_id = next_shell_handoff_acceptance_baseline_default_id(&index)?;
    let baseline_path = index
        .entries
        .iter()
        .find(|entry| entry.baseline_id == baseline_id)
        .and_then(|entry| entry.baseline_manifest_path.as_ref())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "Baseline acceptance index entry {baseline_id} does not include a manifest path"
            )
        })?;
    let baseline = load_shell_handoff_acceptance_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Baseline acceptance identity load failed: {error}"))?;
    let promoted =
        promote_shell_handoff_acceptance_baseline_index_default(&index, &baseline.baseline_id)
            .ok_or_else(|| {
                format!(
                    "Baseline acceptance index does not contain baseline {}",
                    baseline.baseline_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Baseline acceptance index save failed: {error}"))?;
    Ok((baseline, promoted, baseline_path, index_path))
}

pub(crate) fn next_shell_handoff_acceptance_baseline_default_id(
    index: &StudioShellHandoffAcceptanceBaselineIndex,
) -> Result<String, String> {
    if index.entries.is_empty() {
        return Err("Baseline acceptance index has no selectable entries".to_string());
    }
    let default_position = index.default_baseline_id.as_deref().and_then(|default_id| {
        index
            .entries
            .iter()
            .position(|entry| entry.baseline_id == default_id)
    });
    let selected_position = default_position.map_or(0, |position| {
        if position + 1 >= index.entries.len() {
            0
        } else {
            position + 1
        }
    });
    Ok(index.entries[selected_position].baseline_id.clone())
}

pub(crate) fn shell_handoff_acceptance_comparison_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHandoffAcceptanceComparisonReport,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let index = load_shell_handoff_acceptance_baseline_index(&index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    let Some(baseline_index_entry) =
        select_shell_handoff_acceptance_baseline_index_entry(&index, None)
    else {
        return Err("Baseline acceptance index does not contain a selected baseline".to_string());
    };
    let baseline_path = baseline_index_entry
        .baseline_manifest_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Selected baseline index entry does not include a baseline manifest path".to_string()
        })?;
    let baseline_identity = load_shell_handoff_acceptance_baseline_manifest(&baseline_path)
        .map_err(|error| format!("Baseline acceptance identity load failed: {error}"))?;
    let checklist_path = PathBuf::from(&baseline_identity.checklist_path);
    let baseline = load_shell_handoff_acceptance_checklist(&checklist_path)
        .map_err(|error| format!("Baseline acceptance checklist load failed: {error}"))?;
    let (candidate, bundle_root) = shell_handoff_acceptance_for_project_source(project_path)?;
    let report = compare_shell_handoff_acceptance_against_baseline_index_entry(
        &index,
        Some(&index_path),
        baseline_index_entry,
        Some(&baseline_path),
        &baseline_identity,
        &baseline,
        &candidate,
    );
    Ok((report, baseline_path, bundle_root))
}
