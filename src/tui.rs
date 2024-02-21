use std::{
    io::{self},
    panic,
};

use anyhow::Result;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    terminal::{self, size, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

pub type CrosstermTerminal = ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stderr>>;

use crate::{
    app::App,
    event::{Event, EventHandler},
    ui,
    update::{self},
};

/// Representation of a terminal user interface.
///
/// It is responsible for setting up the terminal,
/// initializing the interface and handling the draw events.
pub struct Tui {
    /// Interface to the Terminal.
    terminal: CrosstermTerminal,
    /// Terminal event handler.
    pub events: EventHandler,
}

impl Tui {
    /// Constructs a new instance of [`Tui`].
    pub fn new(terminal: CrosstermTerminal, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    /// Initializes the terminal interface.
    ///
    /// It enables the raw mode and sets terminal properties.
    pub fn enter(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        // Define a custom panic hook to reset the terminal properties.
        // This way, you won't have your terminal messed up if an unexpected error happens.
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    /// [`Draw`] the terminal interface by [`rendering`] the widgets.
    ///
    /// [`Draw`]: tui::Terminal::draw
    /// [`rendering`]: crate::ui:render
    pub fn draw(&mut self, app: &mut App) -> Result<()> {
        self.terminal.draw(|frame| ui::render(app, frame))?;
        Ok(())
    }

    /// Resets the terminal interface.
    ///
    /// This function is also used for the panic hook to revert
    /// the terminal properties if unexpected errors occur.
    fn reset() -> Result<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    /// Exits the terminal interface.
    ///
    /// It disables the raw mode and reverts back the terminal properties.
    pub fn exit(&mut self) -> Result<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

pub fn start_terminal(app: &mut App) -> Result<()> {
    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(200);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    // Start the main loop.
    while !app.should_quit {
        // Render the user interface.
        tui.draw(app)?;
        // Handle events.
        let height = size().unwrap().1;
        match tui.events.next()? {
            Event::Tick => {}
            Event::Key(key_event) => update::update_on_key(app, key_event),
            Event::Mouse(mouse_event) => update::update_on_mouse(app, mouse_event, height / 2),
            Event::Resize(_, _) => {}
        };
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
