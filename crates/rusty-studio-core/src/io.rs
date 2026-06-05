use super::*;

pub fn load_project(path: &Path) -> Result<StudioProject, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseProject {
        path: path.display().to_string(),
        source,
    })
}

pub fn save_project(path: &Path, project: &StudioProject) -> Result<(), StudioCoreError> {
    save_json(path, project)
}

pub fn load_shell_descriptor(path: &Path) -> Result<StudioShellDescriptor, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellDescriptor {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_artifact_manifest(
    path: &Path,
) -> Result<StudioShellArtifactManifest, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellArtifactManifest {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_template_index(path: &Path) -> Result<StudioShellTemplateIndex, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellTemplateIndex {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_template_manifest(
    path: &Path,
) -> Result<StudioShellTemplateManifest, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellTemplateManifest {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_handoff_manifest(
    path: &Path,
) -> Result<StudioShellHandoffManifest, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellHandoffManifest {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_handoff_intake_report(
    path: &Path,
) -> Result<StudioShellHandoffIntakeReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellHandoffIntakeReport {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_handoff_acceptance_checklist(
    path: &Path,
) -> Result<StudioShellHandoffAcceptanceChecklistReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellHandoffAcceptanceChecklist {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_handoff_acceptance_baseline_manifest(
    path: &Path,
) -> Result<StudioShellHandoffAcceptanceBaselineManifest, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellHandoffAcceptanceBaselineManifest {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_handoff_acceptance_baseline_index(
    path: &Path,
) -> Result<StudioShellHandoffAcceptanceBaselineIndex, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellHandoffAcceptanceBaselineIndex {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_export_package_report(
    path: &Path,
) -> Result<StudioShellExportPackageReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseShellExportPackageReport {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_export_package_baseline_manifest(
    path: &Path,
) -> Result<StudioShellExportPackageBaselineManifest, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellExportPackageBaselineManifest {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_export_package_baseline_index(
    path: &Path,
) -> Result<StudioShellExportPackageBaselineIndex, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellExportPackageBaselineIndex {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_release_candidate_review_report(
    path: &Path,
) -> Result<StudioShellReleaseCandidateReviewReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellReleaseCandidateReviewReport {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_release_candidate_review_manifest(
    path: &Path,
) -> Result<StudioShellReleaseCandidateReviewManifest, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellReleaseCandidateReviewManifest {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_release_candidate_review_index(
    path: &Path,
) -> Result<StudioShellReleaseCandidateReviewIndex, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellReleaseCandidateReviewIndex {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_hostess_handoff_package_report(
    path: &Path,
) -> Result<StudioShellHostessHandoffPackageReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellHostessHandoffPackageReport {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_hostess_owner_intake_report(
    path: &Path,
) -> Result<StudioShellHostessOwnerIntakeReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellHostessOwnerIntakeReport {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_manifold_package_validation_report(
    path: &Path,
) -> Result<StudioManifoldPackageValidationReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseManifoldPackageValidationReport {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_package_evidence_intake_report(
    path: &Path,
) -> Result<StudioPackageEvidenceIntakeReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParsePackageEvidenceIntakeReport {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_projected_motion_breath_authoring_review_report(
    path: &Path,
) -> Result<StudioProjectedMotionBreathAuthoringReviewReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseProjectedMotionBreathAuthoringReview {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_projected_motion_breath_source_adapter_descriptors(
    path: &Path,
) -> Result<Value, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseProjectedMotionBreathSourceAdapters {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_projected_motion_breath_source_adapter_selection_review_report(
    path: &Path,
) -> Result<StudioProjectedMotionBreathSourceAdapterSelectionReviewReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseProjectedMotionBreathSourceAdapterSelection {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_projected_motion_breath_source_binding_document(
    path: &Path,
) -> Result<Value, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseProjectedMotionBreathSourceBinding {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_projected_motion_breath_adapter_normalization_case_document(
    path: &Path,
) -> Result<Value, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseProjectedMotionBreathAdapterNormalization {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_projected_motion_breath_shell_handoff_evidence(
    path: &Path,
) -> Result<Value, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseProjectedMotionBreathShellHandoffEvidence {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_projected_motion_breath_shell_handoff_review_report(
    path: &Path,
) -> Result<StudioProjectedMotionBreathShellHandoffReviewReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseProjectedMotionBreathShellHandoffReview {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_motion_breath_profile_document(path: &Path) -> Result<Value, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| StudioCoreError::ParseMotionBreathProfile {
        path: path.display().to_string(),
        source,
    })
}

pub fn load_shell_hostess_staging_preview_manifest(
    path: &Path,
) -> Result<StudioShellHostessStagingPreviewManifest, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellHostessStagingPreviewManifest {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_hostess_staging_file_plan(
    path: &Path,
) -> Result<StudioShellHostessStagingFilePlan, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellHostessStagingFilePlan {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_hostess_staging_handoff_envelope(
    path: &Path,
) -> Result<StudioShellHostessStagingHandoffEnvelope, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellHostessStagingHandoffEnvelope {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_hostess_staging_acceptance_checklist(
    path: &Path,
) -> Result<StudioShellHostessStagingAcceptanceChecklistReport, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellHostessStagingAcceptanceChecklist {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_hostess_staging_acceptance_manifest(
    path: &Path,
) -> Result<StudioShellHostessStagingAcceptanceManifest, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellHostessStagingAcceptanceManifest {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn load_shell_hostess_staging_acceptance_index(
    path: &Path,
) -> Result<StudioShellHostessStagingAcceptanceIndex, StudioCoreError> {
    let text = std::fs::read_to_string(path).map_err(|source| StudioCoreError::ReadProject {
        path: path.display().to_string(),
        source,
    })?;
    serde_json::from_str(&text).map_err(|source| {
        StudioCoreError::ParseShellHostessStagingAcceptanceIndex {
            path: path.display().to_string(),
            source,
        }
    })
}

pub fn save_json<T: Serialize>(path: &Path, value: &T) -> Result<(), StudioCoreError> {
    let mut text = serde_json::to_string_pretty(value).map_err(|source| {
        StudioCoreError::SerializeProject {
            path: path.display().to_string(),
            source,
        }
    })?;
    text.push('\n');
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent).map_err(|source| StudioCoreError::WriteProject {
            path: path.display().to_string(),
            source,
        })?;
    }
    std::fs::write(path, text).map_err(|source| StudioCoreError::WriteProject {
        path: path.display().to_string(),
        source,
    })
}
