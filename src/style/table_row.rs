//! Table Row Styling
//!
//! *This API requires the following crate features to be activated: `table`*;
//!

use std::rc::Rc;

use iced::{border::Radius, Background, Color, Theme};

/// The appearance of a [`TableRow`](crate::native::table::TableRow).
///

#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// The table row's background.
    pub background: Option<Background>,

    /// The table row's text color.
    pub text_color: Color,

    /// The table row's border radius.
    pub border_radius: Radius,

    /// The table row's border width.
    pub border_width: f32,

    /// The table row's border color.
    pub border_color: Color,

    /// The table row's left offset.
    pub offset_left: f32,

    /// The table row's right offset.
    pub offset_right: f32,
}

/// A set of rules that dictate the style of a table row.
pub trait StyleSheet {
    /// Style for the trait to use.
    type Style: Default;

    /// The default appearance of a table row.
    fn appearance(&self, style: &Self::Style, row_id: u32) -> Appearance;

    /// The appearance of a table row when it is hovered over.
    fn hovered(&self, style: &Self::Style, row_id: u32) -> Appearance;
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
/// Default Prebuilt ``TableRow`` Styles
pub enum TableRowStyles {
    #[default]
    Default,
    TableRowAlternate,
    TableRowHighlight,
    TableRowLowlight,
    TableRowSelected,
	Custom(Rc<dyn StyleSheet<Style = Theme>>),
}

impl TableRowStyles {
    /// Creates a custom [`TableRowStyles`] style variant.
    pub fn custom(style_sheet: impl StyleSheet<Style = Theme> + 'static) -> Self {
        Self::Custom(Rc::new(style_sheet))
    }
}

impl StyleSheet for Theme {
    type Style = TableRowStyles;

    fn appearance(&self, style: &Self::Style, row_id: u32) -> Appearance {
        let palette = self.extended_palette();

        match style {
            TableRowStyles::Default => Appearance {
                text_color: palette.primary.strong.color,
                background: Some(Background::Color(palette.primary.base.color)),
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                offset_left: 0.0,
                offset_right: 0.0,
            },
            TableRowStyles::TableRowAlternate => Appearance {
                text_color: palette.primary.strong.color,
                background: Some(Background::Color(Color {
                    a: 0.50,
                    ..palette.primary.base.color
                })),
                ..Appearance::default()
            },
            TableRowStyles::TableRowHighlight => Appearance {
                text_color: palette.primary.strong.color,
                background: Some(Background::Color(Color {
                    a: 0.30,
                    ..palette.primary.base.color
                })),
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                offset_left: 0.0,
                offset_right: 0.0,
            },
            TableRowStyles::TableRowLowlight => Appearance {
                text_color: palette.primary.base.color,
                background: Some(Background::Color(Color::TRANSPARENT)),
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                offset_left: 0.0,
                offset_right: 0.0,
            },
            TableRowStyles::TableRowSelected => Appearance {
                text_color: palette.primary.strong.color,
                background: Some(Background::Color(palette.primary.base.color)),
                border_radius: 0.0.into(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
                offset_left: 0.0,
                offset_right: 0.0,
            },
            TableRowStyles::Custom(custom) => return custom.appearance(self, row_id),
        }
    }

    fn hovered(&self, style: &Self::Style, row_id: u32) -> Appearance {
        let palette = self.extended_palette();
        match style {
            TableRowStyles::Default => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.60,
                    ..palette.primary.base.color
                })),
                ..self.appearance(style, row_id)
            },
            TableRowStyles::TableRowAlternate => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.25,
                    ..palette.primary.base.color
                })),
                ..self.appearance(style, row_id)
            },
            TableRowStyles::TableRowHighlight => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.60,
                    ..palette.primary.base.color
                })),
                ..self.appearance(style, row_id)
            },
            TableRowStyles::TableRowLowlight => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.60,
                    ..palette.primary.base.color
                })),
                ..self.appearance(style, row_id)
            },
            TableRowStyles::TableRowSelected => Appearance {
                background: Some(Background::Color(Color {
                    a: 0.60,
                    ..palette.primary.base.color
                })),
                ..self.appearance(style, row_id)
            },
            TableRowStyles::Custom(custom) => custom.hovered(self, row_id),
        }
    }
}
