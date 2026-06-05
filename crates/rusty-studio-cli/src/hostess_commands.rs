use super::*;

pub(super) fn handoff_package(
    args: ShellHostessHandoffPackageArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_release_candidate_review_index(&args.review_index)?;
    let report = shell_hostess_handoff_package_for_release_candidate_index(
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

pub(super) fn owner_intake(
    args: ShellHostessOwnerIntakeArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let package = load_shell_hostess_handoff_package_report(&args.package)?;
    let report = shell_hostess_owner_intake_for_handoff_package(&package, Some(&args.package));
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn staging_preview(
    args: ShellHostessStagingPreviewArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let intake = load_shell_hostess_owner_intake_report(&args.intake)?;
    let report = shell_hostess_staging_preview_for_owner_intake(&intake, Some(&args.intake));
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn staging_file_plan(
    args: ShellHostessStagingFilePlanArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let preview = load_shell_hostess_staging_preview_manifest(&args.preview)?;
    let report = shell_hostess_staging_file_plan_for_preview(&preview, Some(&args.preview));
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn staging_handoff(
    args: ShellHostessStagingHandoffArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_plan = load_shell_hostess_staging_file_plan(&args.file_plan)?;
    let report =
        shell_hostess_staging_handoff_envelope_for_file_plan(&file_plan, Some(&args.file_plan));
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn staging_acceptance_checklist(
    args: ShellHostessStagingAcceptanceChecklistArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let handoff = load_shell_hostess_staging_handoff_envelope(&args.handoff)?;
    let report =
        shell_hostess_staging_acceptance_checklist_for_handoff(&handoff, Some(&args.handoff));
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn staging_acceptance_manifest(
    args: ShellHostessStagingAcceptanceManifestArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let checklist = load_shell_hostess_staging_acceptance_checklist(&args.checklist)?;
    let report = shell_hostess_staging_acceptance_manifest_for_checklist(
        &checklist,
        &args.checklist,
        args.acceptance_id.as_deref(),
        args.label.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn staging_acceptance_index(
    args: ShellHostessStagingAcceptanceIndexArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let acceptances = args
        .acceptance_manifests
        .iter()
        .map(|path| {
            load_shell_hostess_staging_acceptance_manifest(path)
                .map(|acceptance| (acceptance, Some(path.clone())))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let report = shell_hostess_staging_acceptance_index_for_manifests(
        acceptances,
        args.default_acceptance_id.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn staging_acceptance_index_append(
    args: ShellHostessStagingAcceptanceIndexAppendArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_hostess_staging_acceptance_index(&args.acceptance_index)?;
    let acceptances = args
        .acceptance_manifests
        .iter()
        .map(|path| {
            load_shell_hostess_staging_acceptance_manifest(path)
                .map(|acceptance| (acceptance, Some(path.clone())))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let report = append_shell_hostess_staging_acceptance_index_manifests(
        &index,
        acceptances,
        args.default_acceptance_id.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn staging_acceptance_index_promote(
    args: ShellHostessStagingAcceptanceIndexPromoteArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_hostess_staging_acceptance_index(&args.acceptance_index)?;
    let report =
        promote_shell_hostess_staging_acceptance_index_default(&index, &args.acceptance_id)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "--acceptance-id was not found in --acceptance-index",
                )
            })?;
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn staging_acceptance_selection(
    args: ShellHostessStagingAcceptanceSelectionArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_hostess_staging_acceptance_index(&args.acceptance_index)?;
    let report = summarize_shell_hostess_staging_acceptance_index_selection(
        &index,
        Some(&args.acceptance_index),
        args.acceptance_id.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn staging_acceptance_comparison(
    args: ShellHostessStagingAcceptanceComparisonArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let candidate = load_shell_hostess_staging_acceptance_checklist(&args.candidate)?;
    let report = if let Some(acceptance_index_path) = args.acceptance_index.as_ref() {
        if args.baseline.is_some() || args.baseline_manifest.is_some() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "--acceptance-index cannot be combined with --baseline or --baseline-manifest",
            )
            .into());
        }
        let acceptance_index = load_shell_hostess_staging_acceptance_index(acceptance_index_path)?;
        let acceptance_index_entry = select_shell_hostess_staging_acceptance_index_entry(
            &acceptance_index,
            args.acceptance_id.as_deref(),
        )
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "--acceptance-id was not found in --acceptance-index",
            )
        })?;
        let acceptance_manifest_path = acceptance_index_entry
            .acceptance_manifest_path
            .as_ref()
            .map(PathBuf::from)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "selected acceptance index entry does not include acceptance_manifest_path",
                )
            })?;
        let baseline_manifest =
            load_shell_hostess_staging_acceptance_manifest(&acceptance_manifest_path)?;
        let baseline_path = PathBuf::from(&baseline_manifest.checklist_path);
        let baseline = load_shell_hostess_staging_acceptance_checklist(&baseline_path)?;
        compare_shell_hostess_staging_acceptance_against_index_entry(
            &acceptance_index,
            Some(acceptance_index_path),
            acceptance_index_entry,
            Some(&acceptance_manifest_path),
            &baseline_manifest,
            &baseline,
            &candidate,
        )
    } else {
        if args.acceptance_id.is_some() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "--acceptance-id requires --acceptance-index",
            )
            .into());
        }
        let baseline_manifest = args
            .baseline_manifest
            .as_ref()
            .map(|path| load_shell_hostess_staging_acceptance_manifest(path))
            .transpose()?;
        let baseline_path = baseline_manifest
            .as_ref()
            .map(|identity| PathBuf::from(&identity.checklist_path))
            .or(args.baseline.clone())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "--baseline, --baseline-manifest, or --acceptance-index is required",
                )
            })?;
        let baseline = load_shell_hostess_staging_acceptance_checklist(&baseline_path)?;
        if let Some(baseline_manifest) = baseline_manifest.as_ref() {
            compare_shell_hostess_staging_acceptance_against_manifest(
                baseline_manifest,
                &baseline,
                &candidate,
            )
        } else {
            compare_shell_hostess_staging_acceptance_checklists(&baseline, &candidate)
        }
    };
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn staging_execution_request(
    args: ShellHostessStagingExecutionRequestArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let acceptance_index = load_shell_hostess_staging_acceptance_index(&args.acceptance_index)?;
    let acceptance_index_entry = select_shell_hostess_staging_acceptance_index_entry(
        &acceptance_index,
        args.acceptance_id.as_deref(),
    )
    .ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "--acceptance-id was not found in --acceptance-index",
        )
    })?;
    let acceptance_manifest_path = acceptance_index_entry
        .acceptance_manifest_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "selected acceptance index entry does not include acceptance_manifest_path",
            )
        })?;
    let acceptance = load_shell_hostess_staging_acceptance_manifest(&acceptance_manifest_path)?;
    let checklist_path = PathBuf::from(&acceptance.checklist_path);
    let checklist = load_shell_hostess_staging_acceptance_checklist(&checklist_path)?;
    let handoff_path = checklist
        .handoff_path
        .as_ref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "selected acceptance checklist does not include handoff_path",
            )
        })?;
    let handoff = load_shell_hostess_staging_handoff_envelope(&handoff_path)?;
    let pmb_shell_handoff_review_path = args
        .pmb_shell_handoff_review
        .clone()
        .or_else(|| default_pmb_shell_handoff_review_path(&args.acceptance_index));
    let pmb_shell_handoff_review_path = match pmb_shell_handoff_review_path {
        Some(path) => Some(canonical_existing_path(&path)?),
        None => None,
    };
    let pmb_shell_handoff_review = pmb_shell_handoff_review_path
        .as_ref()
        .map(|path| load_projected_motion_breath_shell_handoff_review_report(path))
        .transpose()?;
    let pmb_shell_handoff_review_required =
        args.require_pmb_shell_handoff_review || pmb_shell_handoff_review_path.is_some();
    let report = shell_hostess_staging_execution_request_for_acceptance_index_entry_with_pmb_review(
        &acceptance_index,
        Some(&args.acceptance_index),
        acceptance_index_entry,
        Some(&acceptance_manifest_path),
        &acceptance,
        &checklist,
        Some(&handoff_path),
        &handoff,
        pmb_shell_handoff_review_path.as_deref(),
        pmb_shell_handoff_review.as_ref(),
        pmb_shell_handoff_review_required,
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
