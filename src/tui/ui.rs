use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

use crate::app::App;

/// Draws the UI
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // Split into two chunks.
    // Chunk 0 is just the list of tracking repos
    // Chunk 1 is dependent on the currently "selected" item from the list of tracking repos.

    // Render the LHS
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .split(f.size());

    let repos: Vec<ListItem> = app
        .repo_list
        .items
        .iter()
        .map(|i| {
            ListItem::new(i.name.clone()).style(Style::default().fg(Color::Black).bg(Color::White))
        })
        .collect();

    let repos = List::new(repos)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Repository names"),
        )
        // TODO: Change the highlight color to be dynamic depending on the current background
        // color?
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Red));

    f.render_stateful_widget(repos, chunks[0], &mut app.repo_list.state);
}
