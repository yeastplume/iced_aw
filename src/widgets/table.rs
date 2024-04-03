#![allow(clippy::doc_markdown)]

//! A [`table`] widget for displaying [`TableRow`]s and [`TableHeader`]s
//!
//! TODO
//!
//! 

mod table_row;

pub use crate::style::table_row::{Appearance, StyleSheet};

/// A selectable table row
pub type TableRow<'a, Message, Theme, Renderer> = table_row::TableRow<'a, Message, Theme, Renderer>;
