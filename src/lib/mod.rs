//! [RSS](https://www.rssboard.org/rss-specification) reader for the terminal. Built using [ratatui](https://docs.rs/ratatui/latest/ratatui/) and
//! [rss](https://docs.rs/rss/latest/rss/). Currently supports only [Upwork](https://upwork.com).

#![feature(error_iter)]
#![deny(missing_docs)]

/// contains the `clap`-based [`Args`] struct
///
/// [`Args`]: args::Args
pub mod args;

/// Constants
pub mod consts;

/// contains the [`App`] struct (global program state)
///
/// [`App`]: app::App
pub mod app;

/// gets a remote RSS channel (if [`Args.get_from_url`] is set) and parses the local/remote RSS channel
///
/// [`Args.get_from_url`]: args/struct.Args.html#structfield.get_from_url
pub mod channel;

/// defines the event handler [`EventHandler`]
///
/// [`EventHandler`]: event::EventHandler
pub mod event;

/// defines the [`Tui`] struct (a representation of a terminal user interface)
///
/// [`Tui`]: tui::Tui
pub mod tui;

/// UI rendering functions
pub mod ui;

/// handles key and mouse events
pub mod update;
