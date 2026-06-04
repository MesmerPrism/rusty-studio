use super::*;

pub(crate) fn shell_release_candidate_review_for_project_source(
    project_path: &Path,
) -> Result<(StudioShellReleaseCandidateReviewReport, PathBuf), String> {
    let manifest_path = shell_handoff_manifest_output_path(project_path);
    let manifest = load_shell_handoff_manifest(&manifest_path)
        .map_err(|error| format!("Shell handoff manifest load failed: {error}"))?;
    let acceptance_index_path = shell_handoff_acceptance_baseline_index_output_path(project_path);
    let acceptance_index = load_shell_handoff_acceptance_baseline_index(&acceptance_index_path)
        .map_err(|error| format!("Baseline acceptance index load failed: {error}"))?;
    let export_package_index_path = shell_export_package_baseline_index_output_path(project_path);
    let export_package_index = load_shell_export_package_baseline_index(&export_package_index_path)
        .map_err(|error| format!("Export package baseline index load failed: {error}"))?;
    let report = shell_release_candidate_review_for_manifest(
        &manifest,
        Some(&manifest_path),
        &acceptance_index,
        Some(&acceptance_index_path),
        None,
        &export_package_index,
        Some(&export_package_index_path),
        None,
    );
    let output_path = shell_release_candidate_review_output_path(project_path);
    save_json(&output_path, &report)
        .map_err(|error| format!("Shell release candidate review save failed: {error}"))?;
    Ok((report, output_path))
}

pub(crate) fn write_shell_release_candidate_review_manifest_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellReleaseCandidateReviewReport,
        StudioShellReleaseCandidateReviewManifest,
        StudioShellReleaseCandidateReviewIndex,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let (review, review_path) = shell_release_candidate_review_for_project_source(project_path)?;
    let candidate =
        shell_release_candidate_review_manifest_for_report(&review, &review_path, None, None);
    let candidate_path = shell_release_candidate_review_manifest_output_path(project_path);
    save_json(&candidate_path, &candidate)
        .map_err(|error| format!("Shell release candidate identity save failed: {error}"))?;
    let index = shell_release_candidate_review_index_for_manifests(
        vec![(candidate.clone(), Some(candidate_path.clone()))],
        Some(&candidate.candidate_id),
    );
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell release candidate index save failed: {error}"))?;
    Ok((
        review,
        candidate,
        index,
        review_path,
        candidate_path,
        index_path,
    ))
}

pub(crate) fn append_shell_release_candidate_review_manifest_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellReleaseCandidateReviewReport,
        StudioShellReleaseCandidateReviewManifest,
        StudioShellReleaseCandidateReviewIndex,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let (review, _) = shell_release_candidate_review_for_project_source(project_path)?;
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let existing_index = if index_path.is_file() {
        Some(
            load_shell_release_candidate_review_index(&index_path)
                .map_err(|error| format!("Shell release candidate index load failed: {error}"))?,
        )
    } else {
        None
    };
    let (candidate_id, label) =
        next_shell_release_candidate_archive_identity(&review, existing_index.as_ref());
    let review_path =
        shell_release_candidate_review_archive_report_output_path(project_path, &candidate_id);
    save_json(&review_path, &review)
        .map_err(|error| format!("Shell release candidate review archive save failed: {error}"))?;
    let candidate = shell_release_candidate_review_manifest_for_report(
        &review,
        &review_path,
        Some(&candidate_id),
        Some(&label),
    );
    let candidate_path =
        shell_release_candidate_review_archive_manifest_output_path(project_path, &candidate_id);
    save_json(&candidate_path, &candidate)
        .map_err(|error| format!("Shell release candidate identity save failed: {error}"))?;
    let index = if let Some(index) = existing_index.as_ref() {
        append_shell_release_candidate_review_index_manifests(
            index,
            vec![(candidate.clone(), Some(candidate_path.clone()))],
            Some(&candidate.candidate_id),
        )
    } else {
        shell_release_candidate_review_index_for_manifests(
            vec![(candidate.clone(), Some(candidate_path.clone()))],
            Some(&candidate.candidate_id),
        )
    };
    save_json(&index_path, &index)
        .map_err(|error| format!("Shell release candidate index save failed: {error}"))?;
    Ok((
        review,
        candidate,
        index,
        review_path,
        candidate_path,
        index_path,
    ))
}

pub(crate) fn next_shell_release_candidate_archive_identity(
    review: &StudioShellReleaseCandidateReviewReport,
    index: Option<&StudioShellReleaseCandidateReviewIndex>,
) -> (String, String) {
    let status = shell_release_candidate_review_status_label(review.status);
    let base_id = format!(
        "{}.rev{}.{}",
        review.project_id, review.project_revision, status
    );
    let next_slot = index
        .map(|index| {
            index
                .entries
                .iter()
                .filter(|entry| {
                    entry.candidate_id == base_id
                        || entry
                            .candidate_id
                            .strip_prefix(base_id.as_str())
                            .is_some_and(|suffix| suffix.starts_with(".archive"))
                })
                .count()
                + 1
        })
        .unwrap_or(1);
    let candidate_id = if next_slot == 1 {
        base_id
    } else {
        format!("{base_id}.archive{next_slot}")
    };
    let label = if next_slot == 1 {
        format!(
            "{} revision {} {} release candidate",
            review.project_id, review.project_revision, status
        )
    } else {
        format!(
            "{} revision {} {} release candidate archive {}",
            review.project_id, review.project_revision, status, next_slot
        )
    };
    (candidate_id, label)
}

pub(crate) fn shell_release_candidate_review_manifest_summary_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellReleaseCandidateReviewManifest,
        StudioShellReleaseCandidateReviewIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let candidate_path = shell_release_candidate_review_manifest_output_path(project_path);
    let candidate = load_shell_release_candidate_review_manifest(&candidate_path)
        .map_err(|error| format!("Shell release candidate identity load failed: {error}"))?;
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let index = load_shell_release_candidate_review_index(&index_path)
        .map_err(|error| format!("Shell release candidate index load failed: {error}"))?;
    Ok((candidate, index, candidate_path, index_path))
}

pub(crate) fn promote_shell_release_candidate_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellReleaseCandidateReviewManifest,
        StudioShellReleaseCandidateReviewIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let candidate_path = shell_release_candidate_review_manifest_output_path(project_path);
    let candidate = load_shell_release_candidate_review_manifest(&candidate_path)
        .map_err(|error| format!("Shell release candidate identity load failed: {error}"))?;
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let index = load_shell_release_candidate_review_index(&index_path)
        .map_err(|error| format!("Shell release candidate index load failed: {error}"))?;
    let promoted =
        promote_shell_release_candidate_review_index_default(&index, &candidate.candidate_id)
            .ok_or_else(|| {
                format!(
                    "Shell release candidate index does not contain candidate {}",
                    candidate.candidate_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Shell release candidate index save failed: {error}"))?;
    Ok((candidate, promoted, candidate_path, index_path))
}

pub(crate) fn select_next_shell_release_candidate_default_for_project_source(
    project_path: &Path,
) -> Result<
    (
        StudioShellReleaseCandidateReviewManifest,
        StudioShellReleaseCandidateReviewIndex,
        PathBuf,
        PathBuf,
    ),
    String,
> {
    let index_path = shell_release_candidate_review_index_output_path(project_path);
    let index = load_shell_release_candidate_review_index(&index_path)
        .map_err(|error| format!("Shell release candidate index load failed: {error}"))?;
    let candidate_id = next_shell_release_candidate_default_id(&index)?;
    let candidate_path = index
        .entries
        .iter()
        .find(|entry| entry.candidate_id == candidate_id)
        .and_then(|entry| entry.candidate_manifest_path.as_ref())
        .map(PathBuf::from)
        .ok_or_else(|| {
            format!(
                "Shell release candidate index entry {candidate_id} does not include a manifest path"
            )
        })?;
    let candidate = load_shell_release_candidate_review_manifest(&candidate_path)
        .map_err(|error| format!("Shell release candidate identity load failed: {error}"))?;
    let promoted =
        promote_shell_release_candidate_review_index_default(&index, &candidate.candidate_id)
            .ok_or_else(|| {
                format!(
                    "Shell release candidate index does not contain candidate {}",
                    candidate.candidate_id
                )
            })?;
    save_json(&index_path, &promoted)
        .map_err(|error| format!("Shell release candidate index save failed: {error}"))?;
    Ok((candidate, promoted, candidate_path, index_path))
}

pub(crate) fn next_shell_release_candidate_default_id(
    index: &StudioShellReleaseCandidateReviewIndex,
) -> Result<String, String> {
    if index.entries.is_empty() {
        return Err("Shell release candidate index has no selectable entries".to_string());
    }
    let default_position = index
        .default_candidate_id
        .as_deref()
        .and_then(|default_id| {
            index
                .entries
                .iter()
                .position(|entry| entry.candidate_id == default_id)
        });
    let selected_position = default_position.map_or(0, |position| {
        if position + 1 >= index.entries.len() {
            0
        } else {
            position + 1
        }
    });
    Ok(index.entries[selected_position].candidate_id.clone())
}
