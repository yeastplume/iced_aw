use iced::{
    alignment, font,
    mouse::Interaction,
    widget::{container, text, Button, Column, Row, Text, TextInput},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};
use iced_aw::TableRow;

fn main() -> iced::Result {
    TableExample::run(Settings {
        default_text_size: 12.into(),
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    TabSelected(usize),
    TabClosed(usize),
    TabLabelInputChanged(String),
    TabContentInputChanged(String),
    NewTab,
    #[allow(dead_code)]
    Loaded(Result<(), String>),
    FontLoaded(Result<(), font::Error>),
}

#[derive(Debug)]
enum TableExample {
    Loading,
    Loaded(State),
}

#[derive(Debug)]
struct State {
    active_tab: usize,
    new_tab_label: String,
    new_tab_content: String,
    tabs: Vec<(String, String)>,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl TableExample {
    /*pub fn SampleRowData() -> Element<'static, Message, Theme, Renderer> {
        let row = TableRow::new(
            Text::new("Row 1"),
            1,
        )
        .padding(10.into())
        .width(Length::Fill)
        .height(Length::Fixed(50.0.into()));
        //.on_press(|_| Interaction::RowSelected(1));
        row.into()
    }*/
}

impl Application for TableExample {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (TableExample, Command<Message>) {
        (
            TableExample::Loading,
            Command::batch(vec![
                font::load(iced_aw::BOOTSTRAP_FONT_BYTES).map(Message::FontLoaded),
                Command::perform(load(), Message::Loaded),
            ]),
        )
        /*TableExample {
            active_tab: 0,
            new_tab_label: String::new(),
            new_tab_content: String::new(),
            tabs: Vec::new(),
        }*/
    }

    fn title(&self) -> String {
        String::from("Table example")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match self {
            TableExample::Loading => {
                if let Message::Loaded(_) = message {
                    *self = TableExample::Loaded(State {
                        active_tab: 0,
                        new_tab_label: String::new(),
                        new_tab_content: String::new(),
                        tabs: Vec::new(),
                    })
                }
            }
            TableExample::Loaded(state) => match message {
                Message::TabSelected(index) => {
                    println!("Tab selected: {}", index);
                    state.active_tab = index
                }
                Message::TabClosed(index) => {
                    state.tabs.remove(index);
                    println!("active tab before: {}", state.active_tab);
                    state.active_tab = if state.tabs.is_empty() {
                        0
                    } else {
                        usize::max(0, usize::min(state.active_tab, state.tabs.len() - 1))
                    };
                    println!("active tab after: {}", state.active_tab);
                }
                Message::TabLabelInputChanged(value) => state.new_tab_label = value,
                Message::TabContentInputChanged(value) => state.new_tab_content = value,
                Message::NewTab => {
                    println!("New");
                    if !state.new_tab_label.is_empty() && !state.new_tab_content.is_empty() {
                        println!("Create");
                        state.tabs.push((
                            state.new_tab_label.to_owned(),
                            state.new_tab_content.to_owned(),
                        ));
                        state.new_tab_label.clear();
                        state.new_tab_content.clear();
                    }
                }
                _ => {}
            },
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        match self {
            TableExample::Loading => container(
                text("Loading...")
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .size(50),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .center_x()
            .into(),
            TableExample::Loaded(state) => {
                Column::new()
                    .push(
                        Row::new()
                            .push(
                                TextInput::new("Tab label", &state.new_tab_label)
                                    .on_input(Message::TabLabelInputChanged)
                                    .size(16)
                                    .padding(5.0),
                            )
                            .push(
                                TextInput::new("Tab content", &state.new_tab_content)
                                    .on_input(Message::TabContentInputChanged)
                                    .size(12)
                                    .padding(5.0),
                            )
                            .push(Button::new(Text::new("New")).on_press(Message::NewTab))
                            .align_items(Alignment::Center)
                            .padding(10.0)
                            .spacing(5.0),
                    )
                    .push(
                        /*state
                        .tabs
                        .iter()
                        .fold(
                            TableRow::new(Message::TabSelected),
                            |tab_bar, (tab_label, _)| {
                                // manually create a new index for the new tab
                                // starting from 0, when there is no tab created yet
                                let idx = tab_bar.size();
                                tab_bar.push(idx, TabLabel::Text(tab_label.to_owned()))
                            },
                        )
                        .on_close(Message::TabClosed)*/
                        TableRow::new(Text::new("Row 1"), 1)
                            .padding(10.into())
                            .width(Length::Fill)
                            .height(Length::Fixed(50.0.into())),
                    )
                    .push(
                        if let Some((_, content)) = state.tabs.get(state.active_tab) {
                            Text::new(content)
                        } else {
                            Text::new("Please create a new tab")
                        }
                        .size(25),
                    )
                    .into()
            }
        }
    }
}
