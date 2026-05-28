// ─── Local Modules ────────────────────────────────────────────────────
mod config;
mod db;
// ─── END: =============================================================

fn main() -> anyhow::Result<()> {
    let app_env_config = config::env::AppConfigEnv::from_env()?;

    todo!();
}
