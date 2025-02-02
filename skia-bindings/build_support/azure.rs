use std::env;
use std::path::PathBuf;

/// Are we running on azure-pipelines?
pub fn is_active() -> bool {
    artifact_staging_directory().is_some()
}

/// Returns the artifact staging directory.
pub fn artifact_staging_directory() -> Option<PathBuf> {
    env::var("BUILD_ARTIFACTSTAGINGDIRECTORY")
        .map(|dir| PathBuf::from(dir))
        .ok()
}
