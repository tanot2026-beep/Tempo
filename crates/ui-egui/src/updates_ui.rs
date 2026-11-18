//! This offline build never looks for, downloads or installs updates. The types below stay so the
//! rest of the app needs no change, and every function does nothing.

use crate::CadApp;

/// Nothing runs between frames.
#[derive(Default)]
pub struct Runtime;

impl CadApp {
    /// There is no startup update check to turn on.
    pub fn enable_update_check(&mut self) {}
}

/// No update checks run.
pub fn tick(_app: &mut CadApp, _ctx: &egui::Context) {}

/// There is no update prompt.
pub fn show(_app: &mut CadApp, _ctx: &egui::Context) {}
