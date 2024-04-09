use iced::{
    alignment, font,
    futures::stream::Count,
    mouse::Interaction,
    widget::{container, text, Button, Column, Container, Row, Text, TextInput},
    Alignment, Application, Command, Element, Length, Settings, Theme,
};
use iced_aw::{style::table_row, TableHeader, TableHeaderState, TableHeaderStyles, TableRow, TableRowStyles};

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
    pub fn sample_cell<'a>(row_id: u16, col_id: u16) -> Text<'a> {
        Text::new(format!("Column {}_{}", row_id, col_id))
            .width(Length::Fill)
            .height(Length::Fixed(50.0.into()))
    }

    pub fn sample_row<'a>(col_count: u16, row_id: u16) -> Vec<Text<'a>> {
        let mut col_data = vec![];
        for i in 0..col_count {
            col_data.push(TableExample::sample_cell(i, row_id));
        }
        col_data
    }

    pub fn sample_table_rows<'a>(
        &self,
        row_count: u16,
        col_count: u16,
    ) -> Vec<TableRow<'a, Message, Theme, iced::Renderer>> {
        let mut table_rows = vec![];
        for i in 0..row_count {
            let row = TableExample::sample_row(col_count, i);
            table_rows.push(TableRow::new(row, i).style(TableRowStyles::Default));
        }
        table_rows
    }

    pub fn sample_header<'a>(
        &self,
        state: &State,
    ) -> TableHeader<'a, Message, Theme, iced::Renderer> {
        let column_keys = vec!["Column 1", "Column 2", "Column 3"];
        let mut column_headers = vec![];
        for column_key in column_keys.iter() {
            let column_header_button = Button::new(Text::new(*column_key).size(20.0));
            // TODO: On press

            let column_header_container = Container::new(column_header_button);

            column_headers.push(((*column_key).to_owned(), column_header_container.into()));
        }

        TableHeader::new(state.header_state.clone(), column_headers, None, None)
            .spacing(5)
            .width(Length::Fill)
            .style(TableHeaderStyles::Default)
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
            TableExample::Loaded(state) => {
                let mut col = Column::new().push(self.sample_header(state));
                for row in self.sample_table_rows(12, 3) {
                    col = col.push(row);
                }
                col.into()
            }
        }
    }
}
