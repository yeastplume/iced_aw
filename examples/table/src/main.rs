use iced::{
    alignment, font,
    futures::stream::Count,
    mouse::Interaction,
    widget::{container, text, Button, Column, Container, Row, Text, TextInput},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};
use iced_aw::{TableHeader, TableHeaderState, TableRow};

fn main() -> iced::Result {
    TableExample::run(Settings {
        default_text_size: 12.into(),
        ..Default::default()
    })
}

#[derive(Debug, Clone)]
enum Message {
    BlankMessage,
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
    header_state: TableHeaderState,
}

async fn load() -> Result<(), String> {
    Ok(())
}

impl TableExample {
    /*pub fn sample_header() -> Element<'static, Message, Theme, Renderer> {
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

    pub fn sample_header<'a>(
        &self,
        state: &State,
    ) -> TableHeader<'a, Message, Theme, iced::Renderer> {
        let column_keys = vec!["Column 1", "Column 2", "Column 3"];
        let mut column_headers = vec![];
        for column_key in column_keys.iter() {
            let column_header_button = Button::new(
                Text::new(*column_key)
                    .size(30.0)
            ).width(Length::Fill);
            // TODO: On press

            let column_header_container = Container::new(column_header_button)
                .width(Length::Fixed(200.0))
                .height(Length::Fixed(100.0));

            column_headers.push(((*column_key).to_owned(), column_header_container.into()));
        }

        TableHeader::new(state.header_state.clone(), column_headers, None, None).spacing(10).width(Length::Fill)
    }
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
                        header_state: TableHeaderState::default(),
                    })
                }
            }
            TableExample::Loaded(state) => match message {
                Message::BlankMessage => {
                    println!("Loaded")
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
            TableExample::Loaded(state) => Column::new().push(self.sample_header(state)).into(),
        }
    }
}
