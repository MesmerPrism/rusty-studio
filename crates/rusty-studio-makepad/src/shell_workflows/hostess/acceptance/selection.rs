use super::super::super::*;

mod default_id;
mod promote;
mod select;
mod summary;

pub(crate) use promote::*;
pub(crate) use select::*;
pub(crate) use summary::*;

pub(crate) type ShellHostessStagingAcceptanceSelectionResult = Result<
    (
        StudioShellHostessStagingAcceptanceManifest,
        StudioShellHostessStagingAcceptanceIndex,
        PathBuf,
        PathBuf,
    ),
    String,
>;
