//! Displays a [`TableHeader`] displaying child content.
//!
//! *This API requires the following crate features to be activated: `table`*

use iced::{
    advanced::{
        layout::{Limits, Node, flex},
        renderer,
        widget::Tree,
        Clipboard, Layout, Shell, Widget,
    },
	widget::Space,
    event,
    mouse,
    Alignment, Background, Border, Color, Element, Event, Length, Padding, Point, Rectangle,
    Size
};

#[derive(Clone, Debug, Default)]
/// The state of the [`TableHeader`]
pub struct TableHeaderState {
	/// Whether the cursor is hovering over the resize area
	pub resize_hovering: bool,

	/// Whether the [`TableHeader`] is currently resizing
	pub resizing: bool,

	/// The starting cursor position when resizing
	pub starting_cursor_pos: Option<Point>,

	/// The starting width of the left column when resizing 
	pub starting_left_width: f32,

	/// The starting width of the right column when resizing
	pub starting_right_width: f32,

	/// the index of the column being resized 
	pub resizing_idx: usize,
}

pub use crate::style::table_header::StyleSheet;

/// A table header
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

pub struct TableHeader<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
	Renderer: 'a + renderer::Renderer,
	Theme: StyleSheet,
{
	spacing: u16,
	padding: Padding,
	width: Length,
	height: Length,
	state: TableHeaderState,
	leeway: u16,
	on_resize: Option<(u16, Box<dyn Fn(ResizeEvent) -> Message + 'a>)>,
	children: Vec<Element<'a, Message, Theme, Renderer>>,
	left_margin: bool,
	right_margin: bool,
	names: Vec<String>,
	style: <Theme as StyleSheet>::Style,
}

impl<'a, Message, Theme, Renderer> TableHeader<'a, Message, Theme, Renderer>
where
	Renderer: 'a + renderer::Renderer,
	Theme: StyleSheet,
	Message: 'a,
{
	/// Creates a new [`TableHeader`] displaying the provided content.
	pub fn new(
		state: TableHeaderState,
		headers: Vec<(String, Element<'a, Message, Theme, Renderer>)>,
		left_margin: Option<Length>,
		right_margin: Option<Length>,
	) -> Self
	where
		Renderer: 'a + renderer::Renderer,
		Theme: StyleSheet,
	{
		let mut names = vec![];
		let mut left = false;
		let mut right = false;

		let mut children = vec![];

		if let Some(margin) = left_margin {
			children.push(Space::with_width(margin).into());
			left = true;
		}

		for (key, container) in headers {
			names.push(key);

			// add container to children
			children.push(container.into());
		}

		if let Some(margin) = right_margin {
			children.push(Space::with_width(margin).into());
			right = true;
		}

		Self {
			spacing: 0,
			padding: Padding::ZERO,
			width: Length::Fill,
			height: Length::Shrink,
			leeway: 0,
			state,
			on_resize: None,
			children,
			left_margin: left,
			right_margin: right,
			names,
			style: Default::default(),
		}
	}

	/// Sets the style of the [`TableHeader`].
	pub fn style<S>(mut self, style: S) -> Self
	where
		S: Into<<Theme as StyleSheet>::Style>,
	{
		self.style = style.into();
		self
	}

	/// Sets the spacing between the children of the [`TableHeader`].
	pub fn spacing(mut self, units: u16) -> Self {
		self.spacing = units;
		self
	}

	/// Sets the width of the [`TableHeader`].
	pub fn width(mut self, width: Length) -> Self {
		self.width = width;
		self
	}

	/// sets the height of the [`TableHeader`].
	pub fn height(mut self, height: Length) -> Self {
		self.height = height;
		self
	}

	/// Sets the padding of the [`TableHeader`].
	pub fn padding(mut self, padding: Padding) -> Self {
		self.padding = padding;
		self
	}

	/// Sets the leeway for resizing the columns
	pub fn on_resize<F>(mut self, leeway: u16, f: F) -> Self
	where
		F: 'a + Fn(ResizeEvent) -> Message,
	{
		self.leeway = leeway;
		self.on_resize = Some((leeway, Box::new(f)));
		self
	}

	/// Triggers the resize event
	fn trigger_resize(
		&self,
		left_name: String,
		left_width: u16,
		right_name: String,
		right_width: u16,
		shell: &mut Shell<'_, Message>,
	) {
		if let Some((_, on_resize)) = &self.on_resize {
			//TODO: Update
			shell.publish(on_resize(ResizeEvent::ResizeColumn {
				left_name,
				left_width,
				right_name,
				right_width,
			}));
		}
	}

	fn _trigger_finished(&self, shell: &mut Shell<'_, Message>) {
		if let Some((_, on_resize)) = &self.on_resize {
			shell.publish(on_resize(ResizeEvent::Finished));
		}
	}
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
	for TableHeader<'a, Message, Theme, Renderer>
where
	Renderer: 'a + renderer::Renderer,
	Theme: StyleSheet,
{
	fn children(&self) -> Vec<Tree> {
		self.children.iter().map(Tree::new).collect()
	}

	fn diff(&self, tree: &mut Tree) {
		tree.diff_children(&self.children);
	}

	fn size(&self) -> Size<Length> {
		Size {
			width: self.width,
			height: self.height,
		}
	}

	fn layout(
		&self,
		tree: &mut Tree,
		renderer: &Renderer,
		limits: &Limits,
	) -> Node {

		let limits = limits
			.loose()
			.width(self.width)
			.height(self.height)
			.shrink(self.padding);

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
		let bounds = layout.bounds();
		let cursor_position = cursor.position().unwrap_or_default();
		let is_mouse_over = bounds.contains(cursor_position);

		let appearance = if is_mouse_over {
			theme.hovered(&self.style)
		} else {
			theme.appearance(&self.style)
		};

		let background = renderer::Quad {
			bounds: Rectangle {
				x: bounds.x + appearance.offset_left as f32,
				y: bounds.y,
				width: bounds.width - appearance.offset_right as f32,
				height: bounds.height,
			},
			border: Border {
				width: appearance.border_width,
				color: appearance.border_color,
				radius: appearance.border_radius.into(),
			},
			shadow: Default::default(),
		};

		renderer.fill_quad(
			background.into(),
			appearance.background.unwrap_or(Background::Color(Color::TRANSPARENT)),
		);

		for ((child, state), layout) in self
			.children
			.iter()
			.zip(&tree.children)
			.zip(layout.children())
		{
			child
				.as_widget()
				.draw(state, renderer, theme, style, layout, cursor, viewport)
		}
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
		let in_bounds = layout.bounds().contains(cursor_position);

		if self.state.resizing || in_bounds {
			let child_len = self.children.len();
			let start_offset = if self.left_margin { 1 } else { 0 };
			let end_offset = if self.right_margin { 1 } else { 0 };

			let dividers = self
				.children
				.iter()
				.enumerate()
				.zip(layout.children())
				.filter_map(|((idx, _), layout)| {
					if idx >= start_offset && idx < (child_len - 1 - end_offset) {
						Some((idx, layout.position().x + layout.bounds().width))
					} else {
						None
					}
				})
				.collect::<Vec<_>>();

			if self.on_resize.is_some() {
				if !self.state.resizing {
					self.state.resize_hovering = false;
				}

				for (idx, divider) in dividers.iter() {
					if cursor_position.x > (divider - self.leeway as f32)
						&& cursor_position.x < (divider + self.leeway as f32)
					{
						if !self.state.resize_hovering {
							self.state.resizing_idx = *idx;
						}

						self.state.resize_hovering = true;
					}
				}
			}

			match event {
				Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
					if self.state.resize_hovering {
						self.state.resizing = true;
						self.state.starting_cursor_pos = Some(cursor_position);
						self.state.starting_left_width = layout
							.children()
							.nth(self.state.resizing_idx)
							.unwrap()
							.bounds()
							.width;
						self.state.starting_right_width = layout
							.children()
							.nth(self.state.resizing_idx + 1)
							.unwrap()
							.bounds()
							.width;
						return event::Status::Captured;
					}
				}
				Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
					if self.state.resizing {
						self.state.resizing = false;
						let _ = self.state.starting_cursor_pos.take();
						// TODO: UPDATE
						//shell.publish(messages);
						return event::Status::Captured;
					}
				}
				Event::Mouse(mouse::Event::CursorMoved { position }) => {
					if self.state.resizing {
						let delta = position.x - self.state.starting_cursor_pos.unwrap().x;

						let left_width = self.state.starting_left_width;
						let right_width = self.state.starting_right_width;

						let max_width = left_width + right_width - 30.0;

						let left_width = (left_width + delta).max(30.0).min(max_width) as u16;
						let left_name = &self.names[self.state.resizing_idx - start_offset];
						let right_width = (right_width - delta).max(30.0).min(max_width) as u16;
						let right_name = &self.names[self.state.resizing_idx + 1 - start_offset];

						self.trigger_resize(
							left_name.clone(),
							left_width,
							right_name.clone(),
							right_width,
							shell,
						);
						return event::Status::Captured;
					}
				}
				_ => {}
			}
		} else {
			self.state.resize_hovering = false;
		}

		self.children
			.iter_mut()
			.zip(&mut tree.children)
			.zip(layout.children())
			.map(|((child, state), layout)| {
				child.as_widget_mut().on_event(
					state,
					event.clone(),
					layout,
					cursor,
					renderer,
					clipboard,
					shell,
					viewport,
				)
			})
			.fold(event::Status::Ignored, event::Status::merge)
	}


	fn mouse_interaction(
		&self,
		_tree: &Tree,
		layout: Layout<'_>,
		cursor: mouse::Cursor,
		_viewport: &Rectangle,
		_renderer: &Renderer,
	) -> mouse::Interaction {
		let bounds = layout.bounds();
		let cursor_position = cursor.position().unwrap_or_default();
		let is_mouse_over = bounds.contains(cursor_position);

		if is_mouse_over {
			mouse::Interaction::Pointer
		} else {
			mouse::Interaction::default()
		}
	}

}

impl<'a, Message, Theme, Renderer> From<TableHeader<'a, Message, Theme, Renderer>>
	for Element<'a, Message, Theme, Renderer>
where
	Message: 'a,
	Renderer: 'a + renderer::Renderer,
	Theme: 'a + StyleSheet
{
	fn from(header: TableHeader<'a, Message, Theme, Renderer>) -> Self {
		Self::new(header)
	}
}

#[derive(Debug, Clone)]
pub enum ResizeEvent {
	ResizeColumn {
		left_name: String,
		left_width: u16,
		right_name: String,
		right_width: u16,
	},
	Finished,
}
