use directories::ProjectDirs;
use nix::unistd::execv;
use std::{env, ffi::CString, path::PathBuf, process::Command};

use crossterm::event::KeyCode;
use ratatui::widgets::ListState;

use crate::repo::{Repo, Tracking};

const EDITOR_PATH: &str = "/opt/homebrew/bin/nvim";
const SHELL_PATH: &str = "/bin/zsh";

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

pub enum ClosingAction {
    Exit,
    Cd,
    Editor,
    // Lazygit,
}

/// Object to hold the applicaion state.
pub struct App {
    pub state_list: StatefulList,
    pub action: ClosingAction,
    pub quit: bool,
}

impl App {
    pub fn new(tracking: Tracking) -> App {
        App {
            state_list: StatefulList::with_items(tracking.active),
            action: ClosingAction::Exit,
            quit: false,
        }
    }

    /// This should choose the previous repo from tracking
    pub fn on_up(&mut self) {
        self.state_list.previous();
    }

    /// This should choose the next repo from tracking
    pub fn on_down(&mut self) {
        self.state_list.next();
    }

    /// This handles the different events occuring for key strokes.
    pub fn on_key(&mut self, k: KeyCode) {
        match k {
            KeyCode::Char('q') => {
                self.quit = true;
                self.action = ClosingAction::Exit;
            }
            KeyCode::Char('e') => {
                self.quit = true;
                self.action = ClosingAction::Editor;
            }
            KeyCode::Enter => {
                if self.state_list.items.is_empty() {
                    // TODO: Show pop-up in UI when pressing Enter and there are no repos to choose
                    // from.
                    self.quit = false;
                } else {
                    self.quit = true;
                    self.action = ClosingAction::Cd;
                }
            }
            _ => (),
        }
    }

    fn goto_selected_dir(&self) {
        let selected_index = self.state_list.state.selected().unwrap_or(0);
        let selected_path = &self.state_list.items[selected_index].path;

        env::set_current_dir(selected_path);
    }

    pub fn goto_dir(&self) {
        self.goto_selected_dir();
        let shell = CString::new(SHELL_PATH).unwrap();
        let args = [shell.clone()];
        execv(&shell, &args).unwrap();
    }

    /// Open the selected repo in the editor
    /// NOTE: Currently you don't make a new shell session then CD, so when you close you are still
    /// in the same directory as you opened coal, so maybe this is desired behaviour?
    pub fn open_in_editor(&self) {
        self.goto_selected_dir();
        /// NOTE: Hard coded path to neovim, will need to be changed to be more dynamic
        let path = CString::new(EDITOR_PATH).unwrap();
        let args = vec![
            CString::new("nvim").unwrap(),
            // Add any arguments you want to pass to neovim here
        ];
        execv(&path, &args).expect("execv failed");
    }
}
