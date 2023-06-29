use clap::Subcommand;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Subcommand, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Command {
    /// Return "ok"
    Ping,

    /// Open the GTK inspector
    Inspect,

    /// Set an `ironvar` value.
    /// This creates it if it does not already exist, and updates it if it does.
    /// Any references to this variable are automatically and immediately updated.
    /// Keys and values can be any valid UTF-8 string.
    Set {
        /// Variable key. Can be any valid UTF-8 string.
        key: Box<str>,
        /// Variable value. Can be any valid UTF-8 string.
        value: String,
    },

    /// Get the current value of an `ironvar`.
    Get {
        /// Variable key.
        key: Box<str>,
    },

    /// Load an additional CSS stylesheet.
    /// The sheet is automatically hot-reloaded.
    LoadCss {
        /// The path to the sheet.
        path: PathBuf,
    },
}