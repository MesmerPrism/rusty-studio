use std::path::PathBuf;

pub(super) fn normalize_verbatim_path(path: PathBuf) -> PathBuf {
    #[cfg(windows)]
    {
        let path_text = path.to_string_lossy();
        if let Some(rest) = path_text.strip_prefix(r"\\?\UNC\") {
            return PathBuf::from(format!(r"\\{rest}"));
        }
        if let Some(rest) = path_text.strip_prefix(r"\\?\") {
            return PathBuf::from(rest);
        }
    }
    path
}
