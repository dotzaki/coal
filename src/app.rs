use ratatui::widgets::ListState;

use crate::repo::{Repo, Tracking};

/// This is used for the TUI implementation
pub struct StatefulList {
    pub state: ListState,
    pub items: Vec<Repo>,
}

impl StatefulList {
    fn with_items(items: Vec<Repo>) -> StatefulList {
        // TODO: Instead of the default implementation have it so that the first item is always
        // selected by default.
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

/// Object to hold the applicaion state.
pub struct App {
    pub repo_list: StatefulList,
    pub should_quit: bool,
}

impl App {
    pub fn new(tracking: Tracking) -> App {
        App {
            repo_list: StatefulList::with_items(tracking.active),
            should_quit: false,
        }
    }

    /// This should choose the previous repo from tracking
    pub fn on_up(&mut self) {
        self.repo_list.previous();
    }

    /// This should choose the next repo from tracking
    pub fn on_down(&mut self) {
        self.repo_list.next();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            }
            _ => {}
        }
    }
}
