//! Displays a selectable [`TableRow`] displaying child content.
//!
//! *This API requires the following crate features to be activated: `table`*

use iced::{
    advanced::{
        layout::{flex, Limits, Node},
        overlay, renderer,
        widget::Tree,
        Clipboard, Layout, Shell, Widget,
    },
    event, mouse, Alignment, Border, Element, Event, Length, Padding, Point, Rectangle, Size,
    Vector,
};

pub use crate::style::table_row::StyleSheet;

/// A selectable table row
///
/// TODO: Add correct documentation
///
/// # Example
/// ```ignore
/// # use iced_aw::{TabLabel, TabBar};
/// #
/// #[derive(Debug, Clone)]
/// enum Message {
///     TabSelected(TabId),
/// }
///
/// #[derive(PartialEq, Hash)]
/// enum TabId {
///    One,
///    Two,
///    Three,
/// }
///
/// let tab_bar = TabBar::new(
///     Message::TabSelected,
/// )
/// .push(TabId::One, TabLabel::Text(String::from("One")))
/// .push(TabId::Two, TabLabel::Text(String::from("Two")))
/// .push(TabId::Three, TabLabel::Text(String::from("Three")))
/// .set_active_tab(&TabId::One);
/// ```
#[allow(missing_debug_implementations)]

pub struct TableRow<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer,
    Theme: StyleSheet,
{
    padding: Padding,
    spacing: u16,
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    row_id: u16,
    inner_row_height: u32,
    horizontal_alignment: Alignment,
    vertical_alignment: Alignment,
    style: <Theme as StyleSheet>::Style,
    children: Vec<Element<'a, Message, Theme, Renderer>>,
    on_press: Option<Box<dyn Fn(Event) -> Message + 'a>>,
}

impl<'a, Message, Theme, Renderer> TableRow<'a, Message, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer,
    Theme: StyleSheet,
{
    /// Creates an new [`TableRow`] with the given content.
    pub fn new<T>(row_content: Vec<T>, row_id: u16) -> Self
    where
        T: Into<Element<'a, Message, Theme, Renderer>>,
    {
        let mut left = false;
        let mut right = false;
        let mut children = vec![];

        for container in row_content {
            // add container to children
            children.push(container.into());
        }

        Self {
            padding: Padding::ZERO,
            spacing: 0,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            row_id: row_id,
            inner_row_height: u32::MAX,
            horizontal_alignment: Alignment::Start,
            vertical_alignment: Alignment::Start,
            style: Default::default(),
            children: children,
            on_press: None,
        }
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    /// Sets the style of the [`TableRow`].
    pub fn style<S>(mut self, style: S) -> Self
    where
        S: Into<<Theme as StyleSheet>::Style>,
    {
        self.style = style.into();
        self
    }

    /// Sets the height of the [`TableRow`].
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    /// Sets the maximum width of the [`TableRow`].
    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    /// Sets the maximum height of the [`TableRow`] in pixels.
    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    /// Sets the height of the area that will be used to define the event capture area of [`TableRow`] in pixels.
    pub fn inner_row_height(mut self, inner_row_height: u32) -> Self {
        self.inner_row_height = inner_row_height;
        self
    }

    /// Sets the content alignment for the horizontal axis of the [`TableRow`].
    pub fn align_x(mut self, alignment: Alignment) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the content alignment for the vertical axis of the [`TableRow`].
    pub fn align_y(mut self, alignment: Alignment) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Centers the contents in the horizontal axis of the [`TableRow`].
    pub fn center_x(mut self) -> Self {
        self.horizontal_alignment = Alignment::Center;
        self
    }

    /// Centers the contents in the vertical axis of the [`TableRow`].
    pub fn center_y(mut self) -> Self {
        self.vertical_alignment = Alignment::Center;
        self
    }

    /// Sets the message that will be produced when the [`TableRow`] is pressed.
    pub fn on_press<T>(mut self, f: T) -> Self
    where
        T: 'a + Fn(Event) -> Message,
    {
        self.on_press = Some(Box::new(f));
        self
    }

    pub fn padding(mut self, p: Padding) -> Self {
        self.padding = p;
        self
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for TableRow<'a, Message, Theme, Renderer>
where
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer,
    Theme: StyleSheet,
{
    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: self.height,
        }
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let limits = limits
            .loose()
            .width(self.width)
            .height(self.height)
            .shrink(self.padding);

        /*let mut content = self
            .children
            .as_widget()
            .layout(tree, renderer, &limits.loose());
        let size = limits.resolve(self.width, self.height, content.size());

        // TODO: MODIFIED COORDINATES, CHECK
        content = content.move_to(Point::new(
            self.padding.top as f32,
            self.padding.left as f32,
        ));
        content = content.align(self.horizontal_alignment, self.vertical_alignment, size);

        Node::with_children(size.expand(self.padding), vec![content])*/
        flex::resolve(
            flex::Axis::Horizontal,
            renderer,
            &limits,
            self.width,
            self.height,
            self.padding,
            self.spacing as f32,
            Alignment::Start,
            &self.children,
            &mut tree.children,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let cursor_position = cursor.position().unwrap_or_default();
        let bounds = layout.bounds();
        let mut custom_bounds = layout.bounds();

        // inner_row_height set?
        if self.inner_row_height != u32::MAX {
            custom_bounds.height = self.inner_row_height as f32;
        }

        let is_mouse_over = custom_bounds.contains(cursor_position);

        let appearance = if is_mouse_over {
            theme.hovered(&self.style, self.row_id)
        } else {
            theme.appearance(&self.style, self.row_id)
        };

        let background = renderer::Quad {
            bounds: Rectangle {
                x: bounds.x + appearance.row.offset_left as f32,
                y: bounds.y,
                width: bounds.width - appearance.row.offset_right as f32,
                height: custom_bounds.height,
            },
            border: Border {
                width: appearance.row.border_width,
                color: appearance.row.border_color,
                radius: appearance.row.border_radius.into(),
            },
            shadow: Default::default(),
        };

        renderer.fill_quad(
            background.into(),
            appearance.row.background.unwrap(), //.unwrap_or(Background::Color(Color::TRANSPARENT)),
        );

        /*self.content.as_widget().draw(
            &tree,
            renderer,
            theme,
            style,
            content_layout,
            cursor,
            viewport,
        );*/

        for ((child, state), layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
			// Draw cell background, if required
            let cell_background = renderer::Quad {
                bounds: Rectangle {
                    x: layout.bounds().x,
                    y: layout.bounds().y,
                    width: layout.bounds().width,
                    height: layout.bounds().height,
                },
                border: Border {
                    width: appearance.cell.border_width,
                    color: appearance.cell.border_color,
                    radius: appearance.cell.border_radius.into(),
                },
                shadow: Default::default(),
            };
            renderer.fill_quad(
                cell_background.into(),
                appearance.cell.background.unwrap(), //.unwrap_or(Background::Color(Color::TRANSPARENT)),
            );

            child
                .as_widget()
                .draw(state, renderer, theme, style, layout, cursor, viewport)
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let cursor_position = cursor.position().unwrap_or_default();
        let bounds = layout.bounds();
        let is_mouse_over = bounds.contains(cursor_position);

        let mut mouse_interaction = if is_mouse_over {
            mouse::Interaction::Pointer
        } else {
            mouse::Interaction::default()
        };

        let children = layout.children();

        for layout in children {
            let is_mouse_over = layout.bounds().contains(cursor_position);
            let new_mouse_interaction = if is_mouse_over {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::default()
            };

            if new_mouse_interaction > mouse_interaction {
                mouse_interaction = new_mouse_interaction;
            }
        }

        mouse_interaction
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        let cursor_position = cursor.position().unwrap_or_default();
        /*let status_from_content = layout.on_event(
            &mut tree,
            event.clone(),
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        match status_from_content {
            event::Status::Ignored => {
                if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
                    if let Some(on_press) = &self.on_press {
                        let mut bounds = layout.bounds();

                        // was inner row height set?
                        if self.inner_row_height != u32::MAX {
                            //We can face issues if the row is expanded, so we manage it by having a reduced bounds area to check for pointer
                            bounds.height = self.inner_row_height as f32;
                        }

                        if bounds.contains(cursor_position) {
                            shell.publish(on_press(event));
                        }
                    }
                }
                status_from_content
            }
            _ => status_from_content,
        }*/
        event::Status::Ignored
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        cursor: Vector, // Change the type of the `cursor` parameter
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        /*self.overlay(
            tree,
            layout.children().next().unwrap(),
            renderer,
            cursor,
        )*/
        None
    }
}

impl<'a, Message, Theme, Renderer> From<TableRow<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: 'a + renderer::Renderer + iced::advanced::text::Renderer,
    Theme: 'a + StyleSheet,
{
    fn from(row: TableRow<'a, Message, Theme, Renderer>) -> Self {
        Self::new(row)
    }
}
