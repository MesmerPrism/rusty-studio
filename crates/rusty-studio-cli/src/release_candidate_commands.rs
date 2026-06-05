use super::*;

pub(super) fn review(
    args: ShellReleaseCandidateReviewArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let manifest = load_shell_handoff_manifest(&args.manifest)?;
    let acceptance_baseline_index =
        load_shell_handoff_acceptance_baseline_index(&args.acceptance_baseline_index)?;
    let export_package_baseline_index =
        load_shell_export_package_baseline_index(&args.export_package_baseline_index)?;
    let report = shell_release_candidate_review_for_manifest(
        &manifest,
        Some(&args.manifest),
        &acceptance_baseline_index,
        Some(&args.acceptance_baseline_index),
        args.acceptance_baseline_id.as_deref(),
        &export_package_baseline_index,
        Some(&args.export_package_baseline_index),
        args.export_package_baseline_id.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn manifest(
    args: ShellReleaseCandidateReviewManifestArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let review = load_shell_release_candidate_review_report(&args.review)?;
    let report = shell_release_candidate_review_manifest_for_report(
        &review,
        &args.review,
        args.candidate_id.as_deref(),
        args.label.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn index(
    args: ShellReleaseCandidateReviewIndexArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let candidates = args
        .candidate_manifests
        .iter()
        .map(|path| {
            load_shell_release_candidate_review_manifest(path)
                .map(|candidate| (candidate, Some(path.clone())))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let report = shell_release_candidate_review_index_for_manifests(
        candidates,
        args.default_candidate_id.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn index_append(
    args: ShellReleaseCandidateReviewIndexAppendArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_release_candidate_review_index(&args.review_index)?;
    let candidates = args
        .candidate_manifests
        .iter()
        .map(|path| {
            load_shell_release_candidate_review_manifest(path)
                .map(|candidate| (candidate, Some(path.clone())))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let report = append_shell_release_candidate_review_index_manifests(
        &index,
        candidates,
        args.default_candidate_id.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn index_promote(
    args: ShellReleaseCandidateReviewIndexPromoteArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_release_candidate_review_index(&args.review_index)?;
    let report = promote_shell_release_candidate_review_index_default(&index, &args.candidate_id)
        .ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "--candidate-id was not found in --review-index",
        )
    })?;
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn selection(
    args: ShellReleaseCandidateReviewSelectionArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_release_candidate_review_index(&args.review_index)?;
    let report = summarize_shell_release_candidate_review_index_selection(
        &index,
        Some(&args.review_index),
        args.candidate_id.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
