use ratatui::{backend::Backend, Frame};

use crate::app::App;

/// Draws the UI
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Split into two chunks.
    // Chunk 0 is just the list of tracking repos
    // Chunk 1 is dependent on the currently "selected" item from the list of tracking repos.
}

