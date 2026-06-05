use thiserror::Error;

#[derive(Debug, Error)]
pub enum StudioCoreError {
    #[error("{path}: {source}")]
    ReadProject {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("{path}: {source}")]
    ParseProject {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellDescriptor {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellArtifactManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellTemplateIndex {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellTemplateManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHandoffManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHandoffIntakeReport {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHandoffAcceptanceChecklist {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHandoffAcceptanceBaselineManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHandoffAcceptanceBaselineIndex {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellExportPackageReport {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellExportPackageBaselineManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellExportPackageBaselineIndex {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellReleaseCandidateReviewReport {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellReleaseCandidateReviewManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellReleaseCandidateReviewIndex {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHostessHandoffPackageReport {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHostessOwnerIntakeReport {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseManifoldPackageValidationReport {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParsePackageEvidenceIntakeReport {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseProjectedMotionBreathAuthoringReview {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseProjectedMotionBreathSourceAdapters {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseProjectedMotionBreathSourceAdapterSelection {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseProjectedMotionBreathSourceBinding {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseProjectedMotionBreathAdapterNormalization {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseProjectedMotionBreathShellHandoffEvidence {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseProjectedMotionBreathShellHandoffReview {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseMotionBreathProfile {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHostessStagingPreviewManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHostessStagingFilePlan {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHostessStagingHandoffEnvelope {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHostessStagingAcceptanceChecklist {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHostessStagingAcceptanceManifest {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    ParseShellHostessStagingAcceptanceIndex {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    SerializeProject {
        path: String,
        #[source]
        source: serde_json::Error,
    },
    #[error("{path}: {source}")]
    WriteProject {
        path: String,
        #[source]
        source: std::io::Error,
    },
}
