mod app;
mod learning_data;
mod util;

fn main() {
	app::run();
}

// use iced::{Sandbox, Settings};

// fn main() -> iced::Result {
//     gui::LanguageLearningToolApp::run(Settings::default())
// }

// fn main() {
// 	let mut data = learning_data::LearningData::load();

// 	for _ in 0..20 {
// 		let task = data.next_task();
// 		let mut replaced = task.sentence.original.clone();
// 		let word_range = task.word_position..task.word_position+task.word.word.len();
// 		replaced.replace_range(word_range, &"_".repeat(task.word.word.chars().count()));
// 		println!("Sentence: {}", replaced);
// 		println!("Translations:");
// 		for (_id, translation) in &task.sentence.translations {
// 			println!("{}", translation);
// 		}
// 		println!("The word was \"{}\".\n", task.word.word);
// 	}
	
// 	// println!("{}", serde_json::to_string_pretty(&data.sentences.0.iter().take(10).collect::<HashMap<_, _>>()).unwrap());
// }
