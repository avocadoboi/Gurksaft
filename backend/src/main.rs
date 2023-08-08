#![feature(let_chains)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod learning_data;
mod options;
mod sentence_audio;
mod source_data;
mod util;

fn main() {
	app::run();
}
