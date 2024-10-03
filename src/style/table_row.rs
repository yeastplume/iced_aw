//! Table Row Styling
//!
//! *This API requires the following crate features to be activated: `table`*;
//!

use std::rc::Rc;

use iced::{border::Radius, Background, Color, Theme};

/// The appearance of a [`TableRow`](crate::native::table::TableRow) Row itself or
/// individual cell.
///

#[derive(Clone, Copy, Debug)]
pub struct RowOrCellAppearance {
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

impl Default for RowOrCellAppearance {
    fn default() -> Self {
        Self {
            text_color: [1.0, 1.0, 1.0].into(),
            background: None,
            border_radius: 0.0.into(),
            border_width: 0.0,
            border_color: Color::TRANSPARENT,
            offset_left: 0.0,
            offset_right: 0.0,
        }
    }
}

/// The appearance of a [`TableRow`](crate::native::table::TableRow)
/// containing separate appearances for the entire row and individual cells.
#[derive(Clone, Copy, Debug)]
pub struct Appearance {
    /// Appearance of an entire row
    pub row: RowOrCellAppearance,
    /// Appearance of an individual cell
    pub cell: RowOrCellAppearance,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            row: RowOrCellAppearance::default(),
            cell: RowOrCellAppearance::default(),
        }
    }
}

/// A set of rules that dictate the style of a table row.
pub trait StyleSheet {
    /// Style for the trait to use.
    type Style: Default;

    /// The default appearance of a table row.
    fn appearance(&self, style: &Self::Style, row_id: u16) -> Appearance;

    /// The appearance of a table row when it is hovered over.
    fn hovered(&self, style: &Self::Style, row_id: u16) -> Appearance;
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

    fn appearance(&self, style: &Self::Style, row_id: u16) -> Appearance {
        let palette = self.extended_palette();

        match style {
            TableRowStyles::Default => Appearance {
                row: RowOrCellAppearance {
                    text_color: palette.primary.strong.color,
                    background: Some(Background::Color(palette.primary.base.color)),
                    border_radius: 0.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    offset_left: 0.0,
                    offset_right: 0.0,
                },
                cell: RowOrCellAppearance {
                    text_color: palette.primary.strong.color,
                    background: Some(Background::Color(palette.primary.base.color)),
                    border_radius: 0.0.into(),
                    border_width: 1.0,
                    border_color: Color::BLACK,
                    offset_left: 0.0,
                    offset_right: 0.0,
                },
            },
            TableRowStyles::TableRowAlternate => Appearance {
                row: RowOrCellAppearance {
                    text_color: palette.primary.strong.color,
                    background: Some(Background::Color(Color {
                        a: 0.50,
                        ..palette.primary.base.color
                    })),
                    ..RowOrCellAppearance::default()
                },
                cell: RowOrCellAppearance {
                    text_color: palette.primary.strong.color,
                    background: Some(Background::Color(Color {
                        a: 0.50,
                        ..palette.primary.base.color
                    })),
                    ..RowOrCellAppearance::default()
                },
            },
            TableRowStyles::TableRowHighlight => Appearance {
                row: RowOrCellAppearance {
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
                cell: RowOrCellAppearance {
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
            },
            TableRowStyles::TableRowLowlight => Appearance {
                row: RowOrCellAppearance {
                    text_color: palette.primary.base.color,
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    border_radius: 0.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    offset_left: 0.0,
                    offset_right: 0.0,
                },
                cell: RowOrCellAppearance {
                    text_color: palette.primary.base.color,
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    border_radius: 0.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    offset_left: 0.0,
                    offset_right: 0.0,
                },
            },
            TableRowStyles::TableRowSelected => Appearance {
                row: RowOrCellAppearance {
                    text_color: palette.primary.strong.color,
                    background: Some(Background::Color(palette.primary.base.color)),
                    border_radius: 0.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    offset_left: 0.0,
                    offset_right: 0.0,
                },
                cell: RowOrCellAppearance {
                    text_color: palette.primary.strong.color,
                    background: Some(Background::Color(palette.primary.base.color)),
                    border_radius: 0.0.into(),
                    border_width: 0.0,
                    border_color: Color::TRANSPARENT,
                    offset_left: 0.0,
                    offset_right: 0.0,
                },
            },
            TableRowStyles::Custom(custom) => return custom.appearance(self, row_id),
        }
    }

    fn hovered(&self, style: &Self::Style, row_id: u16) -> Appearance {
        let palette = self.extended_palette();
        match style {
            TableRowStyles::Default => Appearance {
                row: RowOrCellAppearance {
                    background: Some(Background::Color(Color {
                        a: 0.60,
                        ..palette.primary.base.color
                    })),
                    ..self.appearance(style, row_id).row
                },
                cell: RowOrCellAppearance {
                    background: Some(Background::Color(Color {
                        a: 0.60,
                        ..palette.primary.base.color
                    })),
                    ..self.appearance(style, row_id).cell
                },
            },
            TableRowStyles::TableRowAlternate => Appearance {
                row: RowOrCellAppearance {
                    background: Some(Background::Color(Color {
                        a: 0.25,
                        ..palette.primary.base.color
                    })),
                    ..self.appearance(style, row_id).row
                },
                cell: RowOrCellAppearance {
                    background: Some(Background::Color(Color {
                        a: 0.25,
                        ..palette.primary.base.color
                    })),
                    ..self.appearance(style, row_id).cell
                },
            },
            TableRowStyles::TableRowHighlight => Appearance {
                row: RowOrCellAppearance {
                    background: Some(Background::Color(Color {
                        a: 0.60,
                        ..palette.primary.base.color
                    })),
                    ..self.appearance(style, row_id).row
                },
                cell: RowOrCellAppearance {
                    background: Some(Background::Color(Color {
                        a: 0.60,
                        ..palette.primary.base.color
                    })),
                    ..self.appearance(style, row_id).cell
                },
            },
            TableRowStyles::TableRowLowlight => Appearance {
                row: RowOrCellAppearance {
                    background: Some(Background::Color(Color {
                        a: 0.60,
                        ..palette.primary.base.color
                    })),
                    ..self.appearance(style, row_id).row
                },
                cell: RowOrCellAppearance {
                    background: Some(Background::Color(Color {
                        a: 0.60,
                        ..palette.primary.base.color
                    })),
                    ..self.appearance(style, row_id).cell
                },
            },
            TableRowStyles::TableRowSelected => Appearance {
                row: RowOrCellAppearance {
                    background: Some(Background::Color(Color {
                        a: 0.60,
                        ..palette.primary.base.color
                    })),
                    ..self.appearance(style, row_id).row
                },
                cell: RowOrCellAppearance {
                    background: Some(Background::Color(Color {
                        a: 0.60,
                        ..palette.primary.base.color
                    })),
                    ..self.appearance(style, row_id).cell
                },
            },
            TableRowStyles::Custom(custom) => custom.hovered(self, row_id),
        }
    }
}
