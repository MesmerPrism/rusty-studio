use super::*;

pub(super) fn checklist(
    args: ShellHandoffAcceptanceChecklistArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let intake = load_shell_handoff_intake_report(&args.intake)?;
    let report = shell_handoff_acceptance_checklist_for_intake(&intake);
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn snapshot(
    args: ShellHandoffAcceptanceSnapshotArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let project = load_project(&args.project)?;
    let report = shell_handoff_acceptance_checklist_for_project(
        &project,
        args.project.parent(),
        &args.bundle_root,
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn summary(
    args: ShellHandoffAcceptanceSummaryArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let checklist = load_shell_handoff_acceptance_checklist(&args.checklist)?;
    let report = summarize_shell_handoff_acceptance_checklist(&checklist, Some(&args.checklist));
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn baseline(
    args: ShellHandoffAcceptanceBaselineArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let checklist = load_shell_handoff_acceptance_checklist(&args.checklist)?;
    let report = shell_handoff_acceptance_baseline_manifest_for_checklist(
        &checklist,
        &args.checklist,
        args.baseline_id.as_deref(),
        args.label.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn baseline_index(
    args: ShellHandoffAcceptanceBaselineIndexArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let baselines = args
        .baseline_manifests
        .iter()
        .map(|path| {
            load_shell_handoff_acceptance_baseline_manifest(path)
                .map(|baseline| (baseline, Some(path.clone())))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let report = shell_handoff_acceptance_baseline_index_for_manifests(
        baselines,
        args.default_baseline_id.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn baseline_index_append(
    args: ShellHandoffAcceptanceBaselineIndexAppendArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_handoff_acceptance_baseline_index(&args.baseline_index)?;
    let baselines = args
        .baseline_manifests
        .iter()
        .map(|path| {
            load_shell_handoff_acceptance_baseline_manifest(path)
                .map(|baseline| (baseline, Some(path.clone())))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let report = append_shell_handoff_acceptance_baseline_index_manifests(
        &index,
        baselines,
        args.default_baseline_id.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn baseline_index_promote(
    args: ShellHandoffAcceptanceBaselineIndexPromoteArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_handoff_acceptance_baseline_index(&args.baseline_index)?;
    let report = promote_shell_handoff_acceptance_baseline_index_default(&index, &args.baseline_id)
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "--baseline-id was not found in --baseline-index",
            )
        })?;
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn baseline_selection(
    args: ShellHandoffAcceptanceBaselineSelectionArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_handoff_acceptance_baseline_index(&args.baseline_index)?;
    let report = summarize_shell_handoff_acceptance_baseline_index_selection(
        &index,
        Some(&args.baseline_index),
        args.baseline_id.as_deref(),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn comparison(
    args: ShellHandoffAcceptanceComparisonArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let candidate = load_shell_handoff_acceptance_checklist(&args.candidate)?;
    let report = if let Some(baseline_index_path) = args.baseline_index.as_ref() {
        if args.baseline.is_some() || args.baseline_manifest.is_some() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "--baseline-index cannot be combined with --baseline or --baseline-manifest",
            )
            .into());
        }
        let baseline_index = load_shell_handoff_acceptance_baseline_index(baseline_index_path)?;
        let baseline_index_entry = select_shell_handoff_acceptance_baseline_index_entry(
            &baseline_index,
            args.baseline_id.as_deref(),
        )
        .ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "--baseline-id was not found in --baseline-index",
            )
        })?;
        let baseline_manifest_path = baseline_index_entry
            .baseline_manifest_path
            .as_ref()
            .map(PathBuf::from)
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "selected baseline index entry does not include baseline_manifest_path",
                )
            })?;
        let baseline_manifest =
            load_shell_handoff_acceptance_baseline_manifest(&baseline_manifest_path)?;
        let baseline_path = PathBuf::from(&baseline_manifest.checklist_path);
        let baseline = load_shell_handoff_acceptance_checklist(&baseline_path)?;
        compare_shell_handoff_acceptance_against_baseline_index_entry(
            &baseline_index,
            Some(baseline_index_path),
            baseline_index_entry,
            Some(&baseline_manifest_path),
            &baseline_manifest,
            &baseline,
            &candidate,
        )
    } else {
        if args.baseline_id.is_some() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "--baseline-id requires --baseline-index",
            )
            .into());
        }
        let baseline_manifest = args
            .baseline_manifest
            .as_ref()
            .map(|path| load_shell_handoff_acceptance_baseline_manifest(path))
            .transpose()?;
        let baseline_path = baseline_manifest
            .as_ref()
            .map(|identity| PathBuf::from(&identity.checklist_path))
            .or(args.baseline.clone())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "--baseline, --baseline-manifest, or --baseline-index is required",
                )
            })?;
        let baseline = load_shell_handoff_acceptance_checklist(&baseline_path)?;
        if let Some(baseline_manifest) = baseline_manifest.as_ref() {
            compare_shell_handoff_acceptance_against_baseline_manifest(
                baseline_manifest,
                &baseline,
                &candidate,
            )
        } else {
            compare_shell_handoff_acceptance_checklists(&baseline, &candidate)
        }
    };
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
