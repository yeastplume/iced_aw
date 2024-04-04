#![allow(clippy::doc_markdown)]

//! A [`table`] widget for displaying [`TableRow`]s and [`TableHeader`]s
//!
//! TODO
//!
//! 

mod table_row;
mod table_header;

pub use crate::style::table_header::{Appearance as TableHeaderAppearance, StyleSheet as TableHeaderStyleSheet};
pub use crate::style::table_row::{Appearance as TableRowAppearance, StyleSheet as TableRowStyleSheet};

/// A selectable table row
pub type TableRow<'a, Message, Theme, Renderer> = table_row::TableRow<'a, Message, Theme, Renderer>;

/// A table header
pub type TableHeader<'a, Message, Theme, Renderer> = table_header::TableHeader<'a, Message, Theme, Renderer>;

/// Table header resize event
pub type TableHeaderResizeEvent = table_header::ResizeEvent;