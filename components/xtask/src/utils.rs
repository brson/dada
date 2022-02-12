use std::path::PathBuf;

pub fn cargo_path(env_var: &str) -> eyre::Result<PathBuf> {
    match std::env::var(env_var) {
        Ok(s) => {
            tracing::debug!("cargo_path({env_var}) = {s}");
            Ok(PathBuf::from(s))
        }
        Err(_) => eyre::bail!("`{}` not set", env_var),
    }
}

