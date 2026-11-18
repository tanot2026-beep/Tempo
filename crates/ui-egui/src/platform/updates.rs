//! This offline build never looks for, downloads or installs updates. The type below stays so the
//! rest of the app (menus, dialogs, quit path) needs no change, and every part of it does nothing.

use crate::PdfCraftApp;

#[derive(Default)]
pub(crate) struct Updates {
    /// Always false: there is no Updates dialog.
    pub(crate) open: bool,
}

impl Updates {
    /// There is no update prompt.
    pub(crate) fn prompt_open(&self) -> bool {
        false
    }

    /// Nothing waits for the app to exit.
    pub(crate) fn exiting(&mut self) {}
}

impl PdfCraftApp {
    /// No update checks run (called each frame).
    pub(crate) fn poll_updates(&mut self) {}
}

/// No startup prompt and no Updates dialog.
pub(crate) fn dialog(_app: &mut PdfCraftApp, _ctx: &egui::Context) {}
