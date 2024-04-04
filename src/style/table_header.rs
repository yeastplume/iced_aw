//! Table Header Styling
//!
//! *This API requires the following crate features to be activated: `table`*;
//!
 
use std::rc::Rc;

use iced::{border::Radius, Background, Color, Theme};

/// The appearance of a [`TableHeader`](crate::native::table::TableHeader).
///

#[derive(Debug, Clone, Copy)]
pub struct Appearance {
	/// The table header's background
	pub background: Option<Background>,

	/// The table header's text color
	pub text_color: Color,

	/// The table header's border radius
	pub border_radius: Radius,

	/// The table header's border width
	pub border_width: f32,

	/// The table header's border color
	pub border_color: Color,

	/// The table header's left offset
	pub offset_left: f32,

	/// The table header's right offset
	pub offset_right: f32,
}

/// A set of rules that dictate the style of a table header.
pub trait StyleSheet {
	/// Style for the trait to use.
	type Style: Default;

	/// Produces the style of a header.
	fn appearance(&self, style: &Self::Style) -> Appearance;

	/// Produces the a hovered appearance for header.
	fn hovered(&self, style: &Self::Style) -> Appearance;
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            text_color: [0.1, 0.1, 0.1].into(),
            background: None,
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 0.0,
            offset_right: 0.0,
        }
    }
}

#[derive(Clone, Default)]
#[allow(missing_docs, clippy::missing_docs_in_private_items)]
/// Default Prebuilt ``TableHeader`` Styles
/// TODO: Just the default for now
 
pub enum TableHeaderStyles {
	#[default]
	Default,
	Custom(Rc<dyn StyleSheet<Style = Theme>>),
}

impl StyleSheet for Theme {
	type Style = TableHeaderStyles;

	fn appearance(&self, style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();

		match style {
			TableHeaderStyles::Default => Appearance {
				//text_color: Some(self.palette.bright.surface),
				text_color: palette.primary.strong.color,
				background: Some(Background::Color(palette.primary.base.color)),
				border_radius: 0.0.into(),
				border_width: 0.0,
				border_color: Color::TRANSPARENT,
				offset_right: 0.0,
				offset_left: 0.0,
			},
            TableHeaderStyles::Custom(custom) => return custom.appearance(self),
		}
	}

	fn hovered(&self, style: &Self::Style) -> Appearance {
        let palette = self.extended_palette();
		match style {
			TableHeaderStyles::Default => Appearance {
				text_color: palette.primary.strong.color,
				background: Some(Background::Color(Color {
					a: 0.50,
					..palette.primary.base.color
				})),
				..Appearance::default()
			},
			TableHeaderStyles::Custom(custom) => return custom.hovered(self),
		}
	}
}
