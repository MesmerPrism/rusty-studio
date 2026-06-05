use super::super::*;

mod archive;
mod selection;
mod write;

pub(crate) use archive::*;
pub(crate) use selection::*;
pub(crate) use write::*;

pub(crate) type ShellHandoffAcceptanceBaselineWriteResult = Result<
    (
        StudioShellHandoffAcceptanceChecklistReport,
        StudioShellHandoffAcceptanceBaselineManifest,
        StudioShellHandoffAcceptanceBaselineIndex,
        PathBuf,
        PathBuf,
        PathBuf,
        PathBuf,
    ),
    String,
>;
