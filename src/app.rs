
use std::sync::Mutex;

use crate::learning_data::{LearningData, LearningTask};

struct App {
	learning_data: Mutex<LearningData>,
}

impl App {
	fn new() -> Self {
		App {
			learning_data: Mutex::new(LearningData::load()),
		}
	}
}

#[tauri::command]
fn next_task(app: tauri::State<App>) -> LearningTask {
	let mut data = app.learning_data.lock().unwrap();
	data.next_task()
}

pub fn run() {
	tauri::Builder::default()
		.manage(App::new())
		.invoke_handler(tauri::generate_handler![
			next_task
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

// use iced::{Element, Length, Sandbox, Theme};
// use iced::alignment;
// use iced::widget::{button, column, container, row, text_input, text};

// use crate::learning_data::{LearningData, LearningTask};

// pub struct LanguageLearningToolApp {
//     learning_data: LearningData,
//     current_task: LearningTask,
//     word_input: String,
// }

// #[derive(Debug, Clone)]
// pub enum Message {
//     NewTask,
//     WordInput(String)
// }

// impl Sandbox for LanguageLearningToolApp {
//     type Message = Message;

//     fn new() -> Self {
//         let mut learning_data = LearningData::load();
//         let current_task = learning_data.next_task();
//         let word_input = String::from(&learning_data.words.0[current_task.word_index].word);
//         LanguageLearningToolApp {
//             learning_data,
//             current_task,
//             word_input,
//         }
//     }

//     fn title(&self) -> String {
//         String::from("Language learning tool")
//     }

//     fn theme(&self) -> Theme {
//         Theme::Dark
//     }

//     fn update(&mut self, message: Message) {
//         match message {
//             Message::NewTask => {
//                 self.current_task = self.learning_data.next_task();
//             }
//             Message::WordInput(new_input) => {
//                 self.word_input = new_input;
//             }
//         }
//     }

//     fn view(&self) -> Element<Message> {
//         let original = &self.learning_data.sentences.0[&self.current_task.sentence_id].original;
//         let text_before = text(&original[..self.current_task.word_position])
//             .size(30).horizontal_alignment(alignment::Horizontal::Center);
		
//         let input = text_input("", &self.word_input, Message::WordInput).padding(5).size(20);

//         let word = &self.learning_data.words.0[self.current_task.word_index].word;
//         let text_after = text(&original[self.current_task.word_position+word.len()..])
//             .size(30).horizontal_alignment(alignment::Horizontal::Center);

//         container(row![text_before, input, text_after].align_items(iced::Alignment::Fill).width(Length::Shrink))
//             .width(Length::Fill).height(Length::Fill)
//             .align_x(alignment::Horizontal::Center).align_y(alignment::Vertical::Center)
//             .into()
//     }
// }


