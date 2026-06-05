use super::*;

pub(super) fn export_package(
    args: ShellExportPackageArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let report = if let Some(manifest_path) = args.manifest.as_ref() {
        let manifest = load_shell_handoff_manifest(manifest_path)?;
        shell_export_package_for_manifest(&manifest)
    } else {
        let project_path = args.project.as_ref().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "--project is required unless --manifest is supplied",
            )
        })?;
        let bundle_root = args.bundle_root.as_ref().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "--bundle-root is required unless --manifest is supplied",
            )
        })?;
        let project = load_project(project_path)?;
        shell_export_package_for_project(&project, project_path.parent(), bundle_root)
    };
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn baseline(
    args: ShellExportPackageBaselineArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let package = load_shell_export_package_report(&args.package_report)?;
    let report = shell_export_package_baseline_manifest_for_report(
        &package,
        &args.package_report,
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
    args: ShellExportPackageBaselineIndexArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let baselines = args
        .baseline_manifests
        .iter()
        .map(|path| {
            load_shell_export_package_baseline_manifest(path)
                .map(|baseline| (baseline, Some(path.clone())))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let report = shell_export_package_baseline_index_for_manifests(
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
    args: ShellExportPackageBaselineIndexAppendArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_export_package_baseline_index(&args.baseline_index)?;
    let baselines = args
        .baseline_manifests
        .iter()
        .map(|path| {
            load_shell_export_package_baseline_manifest(path)
                .map(|baseline| (baseline, Some(path.clone())))
        })
        .collect::<Result<Vec<_>, _>>()?;
    let report = append_shell_export_package_baseline_index_manifests(
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
    args: ShellExportPackageBaselineIndexPromoteArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_export_package_baseline_index(&args.baseline_index)?;
    let report = promote_shell_export_package_baseline_index_default(&index, &args.baseline_id)
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
    args: ShellExportPackageBaselineSelectionArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let index = load_shell_export_package_baseline_index(&args.baseline_index)?;
    let report = summarize_shell_export_package_baseline_index_selection(
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
    args: ShellExportPackageComparisonArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let candidate = load_shell_export_package_report(&args.candidate)?;
    let report = if let Some(baseline_index_path) = args.baseline_index.as_ref() {
        if args.baseline.is_some() || args.baseline_manifest.is_some() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "--baseline-index cannot be combined with --baseline or --baseline-manifest",
            )
            .into());
        }
        let baseline_index = load_shell_export_package_baseline_index(baseline_index_path)?;
        let baseline_index_entry = select_shell_export_package_baseline_index_entry(
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
            load_shell_export_package_baseline_manifest(&baseline_manifest_path)?;
        let baseline_path = PathBuf::from(&baseline_manifest.package_path);
        let baseline = load_shell_export_package_report(&baseline_path)?;
        compare_shell_export_packages_against_baseline_index_entry(
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
            .map(|path| load_shell_export_package_baseline_manifest(path))
            .transpose()?;
        let baseline_path = baseline_manifest
            .as_ref()
            .map(|identity| PathBuf::from(&identity.package_path))
            .or(args.baseline.clone())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "--baseline, --baseline-manifest, or --baseline-index is required",
                )
            })?;
        let baseline = load_shell_export_package_report(&baseline_path)?;
        if let Some(baseline_manifest) = baseline_manifest.as_ref() {
            compare_shell_export_packages_against_baseline_manifest(
                baseline_manifest,
                &baseline,
                &candidate,
            )
        } else {
            compare_shell_export_packages(&baseline, &candidate)
        }
    };
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
