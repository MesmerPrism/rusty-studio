use super::*;

pub(crate) fn shell_hostess_handoff_package_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessHandoffPackageReport, PathBuf), String> {
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let index = load_shell_release_candidate_review_index(&index_path)
        .map_err(|error| format!("Shell release candidate index load failed: {error}"))?;
    let report =
        shell_hostess_handoff_package_for_release_candidate_index(&index, Some(&index_path), None);
    let output_path = shell_hostess_handoff_package_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess handoff package save failed: {error}"))?;
    Ok((report, output_path))
}

pub(crate) fn shell_hostess_owner_intake_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessOwnerIntakeReport, PathBuf), String> {
    let package_path = shell_hostess_handoff_package_output_path(project_path);
    let package = load_shell_hostess_handoff_package_report(&package_path)
        .map_err(|error| format!("Shell Hostess handoff package load failed: {error}"))?;
    let report = shell_hostess_owner_intake_for_handoff_package(&package, Some(&package_path));
    let output_path = shell_hostess_owner_intake_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess owner intake save failed: {error}"))?;
    Ok((report, output_path))
}

pub(crate) fn shell_hostess_staging_preview_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessStagingPreviewManifest, PathBuf), String> {
    let intake_path = shell_hostess_owner_intake_output_path(project_path);
    let intake = load_shell_hostess_owner_intake_report(&intake_path)
        .map_err(|error| format!("Shell Hostess owner intake load failed: {error}"))?;
    let report = shell_hostess_staging_preview_for_owner_intake(&intake, Some(&intake_path));
    let output_path = shell_hostess_staging_preview_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess staging preview save failed: {error}"))?;
    Ok((report, output_path))
}

pub(crate) fn shell_hostess_staging_file_plan_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessStagingFilePlan, PathBuf), String> {
    let preview_path = shell_hostess_staging_preview_output_path(project_path);
    let preview = load_shell_hostess_staging_preview_manifest(&preview_path)
        .map_err(|error| format!("Shell Hostess staging preview load failed: {error}"))?;
    let report = shell_hostess_staging_file_plan_for_preview(&preview, Some(&preview_path));
    let output_path = shell_hostess_staging_file_plan_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess staging file plan save failed: {error}"))?;
    Ok((report, output_path))
}

pub(crate) fn shell_hostess_staging_handoff_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessStagingHandoffEnvelope, PathBuf), String> {
    let file_plan_path = shell_hostess_staging_file_plan_output_path(project_path);
    let file_plan = load_shell_hostess_staging_file_plan(&file_plan_path)
        .map_err(|error| format!("Shell Hostess staging file plan load failed: {error}"))?;
    let report =
        shell_hostess_staging_handoff_envelope_for_file_plan(&file_plan, Some(&file_plan_path));
    let output_path = shell_hostess_staging_handoff_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess staging handoff save failed: {error}"))?;
    Ok((report, output_path))
}

pub(crate) fn shell_hostess_staging_acceptance_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessStagingAcceptanceChecklistReport, PathBuf), String> {
    let handoff_path = shell_hostess_staging_handoff_output_path(project_path);
    let handoff = load_shell_hostess_staging_handoff_envelope(&handoff_path)
        .map_err(|error| format!("Shell Hostess staging handoff load failed: {error}"))?;
    let report =
        shell_hostess_staging_acceptance_checklist_for_handoff(&handoff, Some(&handoff_path));
    let output_path = shell_hostess_staging_acceptance_output_path(project_path);
    save_json(&output_path, &report).map_err(|error| {
        format!("Shell Hostess staging acceptance checklist save failed: {error}")
    })?;
    Ok((report, output_path))
}

pub(crate) fn append_shell_hostess_staging_acceptance_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHostessStagingAcceptanceManifest,
        StudioShellHostessStagingAcceptanceIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let (checklist, _) = shell_hostess_staging_acceptance_for_project_source(project_path)?;
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let existing_index = if index_path.is_file() {
        Some(
            load_shell_hostess_staging_acceptance_index(&index_path).map_err(|error| {
                format!("Shell Hostess staging acceptance index load failed: {error}")
            })?,
        )
    } else {
        None
    };
    let (acceptance_id, label) =
        next_shell_hostess_staging_acceptance_archive_identity(&checklist, existing_index.as_ref());
    let checklist_path = shell_hostess_staging_acceptance_archive_checklist_output_path(
        project_path,
        &acceptance_id,
    );
    save_json(&checklist_path, &checklist).map_err(|error| {
        format!("Shell Hostess staging acceptance checklist archive save failed: {error}")
    })?;
    let acceptance = shell_hostess_staging_acceptance_manifest_for_checklist(
        &checklist,
        &checklist_path,
        Some(&acceptance_id),
        Some(&label),
    );
    let acceptance_path =
        shell_hostess_staging_acceptance_archive_manifest_output_path(project_path, &acceptance_id);
    save_json(&acceptance_path, &acceptance).map_err(|error| {
        format!("Shell Hostess staging acceptance identity save failed: {error}")
    })?;
    save_json(
        &shell_hostess_staging_acceptance_manifest_output_path(project_path),
        &acceptance,
    )
    .map_err(|error| {
        format!("Shell Hostess staging acceptance current identity save failed: {error}")
    })?;
    let index = if let Some(index) = existing_index.as_ref() {
        append_shell_hostess_staging_acceptance_index_manifests(
            index,
            vec![(acceptance.clone(), Some(acceptance_path.clone()))],
            Some(&acceptance.acceptance_id),
        )
    } else {
        shell_hostess_staging_acceptance_index_for_manifests(
            vec![(acceptance.clone(), Some(acceptance_path.clone()))],
            Some(&acceptance.acceptance_id),
        )
    };
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell Hostess staging acceptance index save failed: {error}"))?;
    Ok((acceptance, index, acceptance_path, index_path))
}

pub(crate) fn next_shell_hostess_staging_acceptance_archive_identity(
    checklist: &StudioShellHostessStagingAcceptanceChecklistReport,
    index: Option<&StudioShellHostessStagingAcceptanceIndex>,
) -> (String, String) {
    let status = shell_hostess_staging_acceptance_status_label(checklist.status);
    let project_id = checklist.project_id.as_deref().unwrap_or("unknown_project");
    let revision = checklist
        .project_revision
        .map(|revision| revision.to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let base_id = format!("studio.hostess_staging_acceptance.{project_id}.rev{revision}.{status}");
    let next_slot = index
        .map(|index| {
            index
                .entries
                .iter()
                .filter(|entry| {
                    entry.acceptance_id == base_id
                        || entry
                            .acceptance_id
                            .strip_prefix(base_id.as_str())
                            .is_some_and(|suffix| suffix.starts_with(".archive"))
                })
                .count()
                + 1
        })
        .unwrap_or(1);
    let acceptance_id = if next_slot == 1 {
        base_id
    } else {
        format!("{base_id}.archive{next_slot}")
    };
    let label = if next_slot == 1 {
        format!("{project_id} revision {revision} {status} Hostess staging acceptance")
    } else {
        format!(
            "{project_id} revision {revision} {status} Hostess staging acceptance archive {next_slot}"
        )
    };
    (acceptance_id, label)
}

pub(crate) fn shell_hostess_staging_acceptance_summary_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHostessStagingAcceptanceManifest,
        StudioShellHostessStagingAcceptanceIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let index = load_shell_hostess_staging_acceptance_index(&index_path)
        .map_err(|error| format!("Shell Hostess staging acceptance index load failed: {error}"))?;
    let acceptance_id = index
        .default_acceptance_id
        .as_deref()
        .ok_or_else(|| "Shell Hostess staging acceptance index has no default entry".to_string())?;
    let acceptance_path = index
        .entries
        .iter()
        .find(|entry| entry.acceptance_id == acceptance_id)
        .and_then(|entry| entry.acceptance_manifest_path.as_ref())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "Shell Hostess staging acceptance index entry {acceptance_id} does not include a manifest path"
            )
        })?;
    let acceptance =
        load_shell_hostess_staging_acceptance_manifest(&acceptance_path).map_err(|error| {
            format!("Shell Hostess staging acceptance identity load failed: {error}")
        })?;
    Ok((acceptance, index, acceptance_path, index_path))
}

pub(crate) fn promote_shell_hostess_staging_acceptance_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHostessStagingAcceptanceManifest,
        StudioShellHostessStagingAcceptanceIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let acceptance_path = shell_hostess_staging_acceptance_manifest_output_path(project_path);
    let acceptance =
        load_shell_hostess_staging_acceptance_manifest(&acceptance_path).map_err(|error| {
            format!("Shell Hostess staging acceptance identity load failed: {error}")
        })?;
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let index = load_shell_hostess_staging_acceptance_index(&index_path)
        .map_err(|error| format!("Shell Hostess staging acceptance index load failed: {error}"))?;
    let promoted =
        promote_shell_hostess_staging_acceptance_index_default(&index, &acceptance.acceptance_id)
            .ok_or_else(|| {
            format!(
                "Shell Hostess staging acceptance index does not contain acceptance {}",
                acceptance.acceptance_id
            )
        })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Shell Hostess staging acceptance index save failed: {error}"))?;
    Ok((acceptance, promoted, acceptance_path, index_path))
}

pub(crate) fn select_next_shell_hostess_staging_acceptance_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHostessStagingAcceptanceManifest,
        StudioShellHostessStagingAcceptanceIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let index = load_shell_hostess_staging_acceptance_index(&index_path)
        .map_err(|error| format!("Shell Hostess staging acceptance index load failed: {error}"))?;
    let acceptance_id = next_shell_hostess_staging_acceptance_default_id(&index)?;
    let archive_path = index
        .entries
        .iter()
        .find(|entry| entry.acceptance_id == acceptance_id)
        .and_then(|entry| entry.acceptance_manifest_path.as_ref())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "Shell Hostess staging acceptance index entry {acceptance_id} does not include a manifest path"
            )
        })?;
    let acceptance =
        load_shell_hostess_staging_acceptance_manifest(&archive_path).map_err(|error| {
            format!("Shell Hostess staging acceptance identity load failed: {error}")
        })?;
    let promoted =
        promote_shell_hostess_staging_acceptance_index_default(&index, &acceptance.acceptance_id)
            .ok_or_else(|| {
            format!(
                "Shell Hostess staging acceptance index does not contain acceptance {}",
                acceptance.acceptance_id
            )
        })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Shell Hostess staging acceptance index save failed: {error}"))?;
    let current_path = shell_hostess_staging_acceptance_manifest_output_path(project_path);
    save_json(&current_path, &acceptance).map_err(|error| {
        format!("Shell Hostess staging acceptance current identity save failed: {error}")
    })?;
    Ok((acceptance, promoted, current_path, index_path))
}

pub(crate) fn next_shell_hostess_staging_acceptance_default_id(
    index: &StudioShellHostessStagingAcceptanceIndex,
) -> Result<String, String> {
    if index.entries.is_empty() {
        return Err("Shell Hostess staging acceptance index has no selectable entries".to_string());
    }
    let default_position = index
        .default_acceptance_id
        .as_deref()
        .and_then(|default_id| {
            index
                .entries
                .iter()
                .position(|entry| entry.acceptance_id == default_id)
        });
    let selected_position = default_position.map_or(0, |position| {
        if position + 1 >= index.entries.len() {
            0
        } else {
            position + 1
        }
    });
    Ok(index.entries[selected_position].acceptance_id.clone())
}

pub(crate) fn shell_hostess_staging_acceptance_comparison_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellHostessStagingAcceptanceComparisonReport,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let index = load_shell_hostess_staging_acceptance_index(&index_path)
        .map_err(|error| format!("Shell Hostess staging acceptance index load failed: {error}"))?;
    let Some(acceptance_index_entry) =
        select_shell_hostess_staging_acceptance_index_entry(&index, None)
    else {
        return Err(
            "Shell Hostess staging acceptance index does not contain a selected acceptance"
                .to_string(),
        );
    };
    let acceptance_path = acceptance_index_entry
        .acceptance_manifest_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Selected acceptance index entry does not include an acceptance manifest path"
                .to_string()
        })?;
    let baseline_identity = load_shell_hostess_staging_acceptance_manifest(&acceptance_path)
        .map_err(|error| {
            format!("Shell Hostess staging acceptance identity load failed: {error}")
        })?;
    let checklist_path = PathBuf::from(&baseline_identity.checklist_path);
    let baseline =
        load_shell_hostess_staging_acceptance_checklist(&checklist_path).map_err(|error| {
            format!("Shell Hostess staging acceptance checklist load failed: {error}")
        })?;
    let (candidate, _) = shell_hostess_staging_acceptance_for_project_source(project_path)?;
    let report = compare_shell_hostess_staging_acceptance_against_index_entry(
        &index,
        Some(&index_path),
        acceptance_index_entry,
        Some(&acceptance_path),
        &baseline_identity,
        &baseline,
        &candidate,
    );
    let output_path = shell_hostess_staging_acceptance_comparison_output_path(project_path);
    save_json(&output_path, &report).map_err(|error| {
        format!("Shell Hostess staging acceptance comparison save failed: {error}")
    })?;
    Ok((report, acceptance_path, output_path))
}

pub(crate) fn shell_hostess_staging_execution_request_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellHostessStagingExecutionRequestReport, PathBuf), String> {
    let index_path = shell_hostess_staging_acceptance_index_output_path(project_path);
    let index = load_shell_hostess_staging_acceptance_index(&index_path)
        .map_err(|error| format!("Shell Hostess staging acceptance index load failed: {error}"))?;
    let Some(acceptance_index_entry) =
        select_shell_hostess_staging_acceptance_index_entry(&index, None)
    else {
        return Err(
            "Shell Hostess staging acceptance index does not contain a selected acceptance"
                .to_string(),
        );
    };
    let acceptance_path = acceptance_index_entry
        .acceptance_manifest_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Selected acceptance index entry does not include an acceptance manifest path"
                .to_string()
        })?;
    let acceptance =
        load_shell_hostess_staging_acceptance_manifest(&acceptance_path).map_err(|error| {
            format!("Shell Hostess staging acceptance identity load failed: {error}")
        })?;
    let checklist_path = PathBuf::from(&acceptance.checklist_path);
    let checklist =
        load_shell_hostess_staging_acceptance_checklist(&checklist_path).map_err(|error| {
            format!("Shell Hostess staging acceptance checklist load failed: {error}")
        })?;
    let handoff_path = checklist
        .handoff_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            "Selected acceptance checklist does not include a handoff path".to_string()
        })?;
    let handoff = load_shell_hostess_staging_handoff_envelope(&handoff_path)
        .map_err(|error| format!("Shell Hostess staging handoff load failed: {error}"))?;
    let report = shell_hostess_staging_execution_request_for_acceptance_index_entry(
        &index,
        Some(&index_path),
        acceptance_index_entry,
        Some(&acceptance_path),
        &acceptance,
        &checklist,
        Some(&handoff_path),
        &handoff,
    );
    let output_path = shell_hostess_staging_execution_request_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell Hostess staging execution request save failed: {error}"))?;
    Ok((report, output_path))
}
