use std::path::Path;

use rusty_studio_core::{
    summarize_shell_export_package_baseline_index_selection,
    summarize_shell_handoff_acceptance_baseline_index_selection,
    summarize_shell_hostess_staging_acceptance_index_selection,
    summarize_shell_release_candidate_review_index_selection,
};
use rusty_studio_model::{
    StudioEditReport, StudioEditStatus, StudioGraphView, StudioShellBundleReport,
    StudioShellBundleStatus, StudioShellBundleValidationReport, StudioShellDescriptorStatus,
    StudioShellExportPackageBaselineIndex, StudioShellExportPackageBaselineManifest,
    StudioShellExportPackageBaselineSelectionReport,
    StudioShellExportPackageBaselineSelectionStatus, StudioShellExportPackageComparisonChange,
    StudioShellExportPackageComparisonReport, StudioShellExportPackageComparisonStatus,
    StudioShellExportPackageReport, StudioShellExportPackageStatus,
    StudioShellHandoffAcceptanceBaselineIndex, StudioShellHandoffAcceptanceBaselineManifest,
    StudioShellHandoffAcceptanceBaselineSelectionReport,
    StudioShellHandoffAcceptanceBaselineSelectionStatus,
    StudioShellHandoffAcceptanceChecklistReport, StudioShellHandoffAcceptanceComparisonChange,
    StudioShellHandoffAcceptanceComparisonReport, StudioShellHandoffAcceptanceComparisonStatus,
    StudioShellHandoffAcceptanceStatus, StudioShellHandoffManifest,
    StudioShellHandoffReadinessReport, StudioShellHandoffReport,
    StudioShellHostessHandoffPackageActionStatus, StudioShellHostessHandoffPackageReport,
    StudioShellHostessHandoffPackageStatus, StudioShellHostessOwnerIntakeAssignmentStatus,
    StudioShellHostessOwnerIntakeReport, StudioShellHostessOwnerIntakeStatus,
    StudioShellHostessStagingAcceptanceChecklistReport,
    StudioShellHostessStagingAcceptanceComparisonChange,
    StudioShellHostessStagingAcceptanceComparisonReport,
    StudioShellHostessStagingAcceptanceComparisonStatus, StudioShellHostessStagingAcceptanceIndex,
    StudioShellHostessStagingAcceptanceManifest,
    StudioShellHostessStagingAcceptanceSelectionReport,
    StudioShellHostessStagingAcceptanceSelectionStatus, StudioShellHostessStagingAcceptanceStatus,
    StudioShellHostessStagingExecutionActionStatus,
    StudioShellHostessStagingExecutionRequestReport,
    StudioShellHostessStagingExecutionRequestStatus, StudioShellHostessStagingFilePlan,
    StudioShellHostessStagingFilePlanStatus, StudioShellHostessStagingFileRequestStatus,
    StudioShellHostessStagingHandoffEnvelope, StudioShellHostessStagingHandoffEnvelopeStatus,
    StudioShellHostessStagingHandoffInstructionStatus, StudioShellHostessStagingPreviewGroupStatus,
    StudioShellHostessStagingPreviewManifest, StudioShellHostessStagingPreviewStatus,
    StudioShellReleaseCandidateReviewIndex, StudioShellReleaseCandidateReviewManifest,
    StudioShellReleaseCandidateReviewReport, StudioShellReleaseCandidateReviewSelectionReport,
    StudioShellReleaseCandidateReviewSelectionStatus, StudioShellReleaseCandidateReviewStatus,
    StudioShellRunbookReport, StudioShellRunbookStatus, StudioShellTargetKind,
    StudioValidationStatus, StudioViewModel,
};
mod hostess;
pub(crate) use hostess::*;
mod shell_package;
pub(crate) use shell_package::*;
mod handoff;
pub(crate) use handoff::*;
mod release_candidate;
pub(crate) use release_candidate::*;

mod catalog_text;
pub(crate) use catalog_text::*;
mod edit_text;
pub(crate) use edit_text::*;
mod graph_text;
pub(crate) use graph_text::*;
mod labels;
pub(crate) use labels::*;
mod selection_text;
pub(crate) use selection_text::*;
mod shell_preview_text;
pub(crate) use shell_preview_text::*;
mod validation_text;
pub(crate) use validation_text::*;
