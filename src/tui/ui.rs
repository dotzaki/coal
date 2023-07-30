use anyhow::Context;
use last_git_commit::LastGitCommit;
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::app::App;

/// Draws the UI
/// Split into two chunks.
/// Chunk 0 is just the list of tracking repos
/// Chunk 1 is dependent on the currently "selected" item from the list of tracking repos.
pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // TODO: IF no repositories in tracking then hide the block content and just show the block
    // outlines with a message as the contents.

    // Render the LHS
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(40), Constraint::Percentage(60)].as_ref())
        .margin(2)
        .split(f.size());

    let block_lhs = Block::default()
        .borders(Borders::ALL)
        .title("Repository names");

    let block_rhs = Block::default()
        .borders(Borders::ALL)
        .title("Repository Information");

    // TODO: Handle for when there are no repositories in tracking in a better way?
    if app.state_list.items.is_empty() {
        let text = vec![Line::from("No repositories in tracking")];
        let paragraph = Paragraph::new(text)
            .block(block_lhs)
            .alignment(ratatui::layout::Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[0]);
    } else {
        let repos: Vec<ListItem> = app
            .state_list
            .items
            .iter()
            .map(|i| {
                ListItem::new(i.name.clone())
                    .style(Style::default().fg(Color::Black).bg(Color::White))
            })
            .collect();

        let repos = List::new(repos)
            .block(block_lhs)
            // TODO: Change the highlight color to be dynamic depending on the current background
            // color?
            .highlight_style(Style::default().add_modifier(Modifier::BOLD).bg(Color::Red));

        f.render_stateful_widget(repos, chunks[0], &mut app.state_list.state);
    }

    // Render RHS

    if app.state_list.items.is_empty() {
        let text = vec![Line::from("No repositories in tracking")];
        let info = Paragraph::new(text)
            .block(block_rhs)
            .alignment(ratatui::layout::Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(info, chunks[1]);
    } else {
        let selected_index: usize = app.state_list.state.selected().unwrap_or(0);

        let current_path: &str = match app.state_list.items[selected_index].path.to_str() {
            Some(i) => i,
            None => "n/a", //FIXME: Change this to something better wtf?
        };

        // TODO: Clean this up
        let lgc = LastGitCommit::new().set_path(current_path).build().unwrap();
        let current_branch: &str = lgc.branch();
        let message: &str = lgc.message().unwrap();
        let author: &str = lgc.author().name().unwrap();

        let text = vec![
            Line::from(current_path),
            Line::from(""),
            Line::from(vec![
                Span::raw("Branch"),
                Span::raw(": "),
                Span::raw(current_branch),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw(author),
                Span::raw(": "),
                Span::styled(message, Style::default().add_modifier(Modifier::ITALIC)),
                Span::raw("."),
            ]),
        ];

        let info = Paragraph::new(text)
            .block(block_rhs)
            .alignment(ratatui::layout::Alignment::Center)
            .wrap(Wrap { trim: true });
        f.render_widget(info, chunks[1]);
    }
}
