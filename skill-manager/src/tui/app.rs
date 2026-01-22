//! TUI Application state

/// The main TUI application
pub struct App {
    /// Whether the app should quit
    pub should_quit: bool,

    /// Current screen
    pub current_screen: Screen,

    /// Whether help overlay is shown
    pub show_help: bool,
}

/// Available screens
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Dashboard,
    Skills,
    Updates,
    Conflicts,
    Search,
    Settings,
}

impl App {
    /// Create a new app instance
    pub fn new() -> Self {
        Self {
            should_quit: false,
            current_screen: Screen::default(),
            show_help: false,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
