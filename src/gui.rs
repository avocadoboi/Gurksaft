use iced::{Element, Length, Sandbox, Theme};
use iced::alignment;
use iced::widget::{button, column, container, text_input, text};

use crate::learning_data::{LearningData, LearningTask};

pub struct LanguageLearningToolApp {
    learning_data: LearningData,
    current_task: LearningTask,
}

#[derive(Debug, Clone)]
pub enum Message {
    NewTask,
    WordInput(String)
}

impl Sandbox for LanguageLearningToolApp {
    type Message = Message;

    fn new() -> Self {
        let mut learning_data = LearningData::load();
        let current_task = learning_data.next_task();
        LanguageLearningToolApp {
            learning_data,
            current_task,
        }
    }

    fn title(&self) -> String {
        String::from("Language learning tool")
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::NewTask => {
                self.current_task = self.learning_data.next_task();
            }
            Message::WordInput(_string) => {

            }
        }
    }

    fn view(&self) -> Element<Message> {
        let original = &self.learning_data.sentences.0[&self.current_task.sentence_id].original;
        let text_before = text(&original[..self.current_task.word_position])
            .size(30).horizontal_alignment(alignment::Horizontal::Center);
        let word = &self.learning_data.words.0[self.current_task.word_index].word;
        let input = text_input(word, "", Message::WordInput);
        let text_after = text(&original[self.current_task.word_position+word.len()..])
            .size(30).horizontal_alignment(alignment::Horizontal::Center);
		
        container(column![text_before, input, text_after])
            .width(Length::Fill).height(Length::Fill)
            .align_x(alignment::Horizontal::Center).align_y(alignment::Vertical::Center)
            .into()
    }
}


