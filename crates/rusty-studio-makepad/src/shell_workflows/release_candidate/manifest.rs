use super::super::*;

mod archive;
mod identity;
mod write;

pub(crate) use archive::*;
pub(crate) use write::*;

pub(crate) type ShellReleaseCandidateReviewManifestWriteResult = Result<
    (
        StudioShellReleaseCandidateReviewReport,
        StudioShellReleaseCandidateReviewManifest,
        StudioShellReleaseCandidateReviewIndex,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
>;
