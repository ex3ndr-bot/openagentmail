#![allow(missing_docs)]

pub mod client;
pub mod error;
pub mod resources;
pub mod types;
pub mod utils;

pub use client::{ClientConfig, OpenAgentMail};
pub use error::{Error, Result};
pub use types::*;
