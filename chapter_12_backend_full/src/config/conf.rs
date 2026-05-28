// FILE: src/main.rs

// ─── Crate Imports ────────────────────────────────────────────────────

// use ratatui::widgets::Paragraph;

// use color_eyre::eyre::Result;
// ─── Standard Library ─────────────────────────────────────────────────
//
//
// ─── Local Modules ────────────────────────────────────────────────────
// ─── END: =============================================================

use crate::config::AppConfigEnv;

#[derive(Debug, Clone)]
pub struct AppConfig {
    env: AppConfigEnv,
    db: String,
}
