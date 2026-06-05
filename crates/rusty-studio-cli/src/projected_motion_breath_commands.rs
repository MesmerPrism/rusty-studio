use super::*;

pub(super) fn package_evidence_intake(
    args: PackageEvidenceIntakeArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let source_report = load_manifold_package_validation_report(&args.report)?;
    let report = package_evidence_intake_for_validation_report(
        &source_report,
        Some(&args.report),
        &args.package,
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn authoring_review(
    args: ProjectedMotionBreathAuthoringReviewArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let intake = load_package_evidence_intake_report(&args.intake)?;
    let profile = load_motion_breath_profile_document(&args.profile)?;
    let report = projected_motion_breath_authoring_review_for_intake(
        &intake,
        Some(&args.intake),
        &profile,
        Some(&args.profile),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn source_adapter_selection(
    args: ProjectedMotionBreathSourceAdapterSelectionArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let authoring_review =
        load_projected_motion_breath_authoring_review_report(&args.authoring_review)?;
    let source_descriptors =
        load_projected_motion_breath_source_adapter_descriptors(&args.source_descriptors)?;
    let report = projected_motion_breath_source_adapter_selection_review_for_authoring(
        &authoring_review,
        Some(&args.authoring_review),
        &source_descriptors,
        Some(&args.source_descriptors),
        &args.adapter,
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn adapter_normalization_evidence_review(
    args: ProjectedMotionBreathAdapterNormalizationEvidenceReviewArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let selection_review = load_projected_motion_breath_source_adapter_selection_review_report(
        &args.selection_review,
    )?;
    let package_report = load_manifold_package_validation_report(&args.package_report)?;
    let source_binding =
        load_projected_motion_breath_source_binding_document(&args.source_binding)?;
    let normalization_case =
        load_projected_motion_breath_adapter_normalization_case_document(&args.normalization_case)?;
    let report = projected_motion_breath_adapter_normalization_evidence_review_for_selection(
        &selection_review,
        Some(&args.selection_review),
        &package_report,
        Some(&args.package_report),
        &source_binding,
        Some(&args.source_binding),
        &normalization_case,
        Some(&args.normalization_case),
    );
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

pub(super) fn shell_handoff_review(
    args: ProjectedMotionBreathShellHandoffReviewArgs,
) -> Result<(), Box<dyn std::error::Error>> {
    let evidence = load_projected_motion_breath_shell_handoff_evidence(&args.evidence)?;
    let report =
        projected_motion_breath_shell_handoff_review_for_evidence(&evidence, Some(&args.evidence));
    if let Some(output) = args.output.as_ref() {
        save_json(output, &report)?;
    }
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
