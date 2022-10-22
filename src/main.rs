use std::collections::HashMap;

mod app;
mod learning_data;
mod options;
mod source_data;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let client = reqwest::Client::new();
	let response = client.post("https://tatoeba.org/en/exports/add")
		.form(&[
			("fields[]", "id"), 
			("fields[]", "text"), 
			("fields[]", "trans_id"), 
			("fields[]", "trans_text"), 
			("format", "tsv"),
			("from", "ita"), 
			("to", "eng"), 
			("type", "pairs")
		])
		.header("cookie", "csrfToken=a")
		.header("x-csrf-token", "a")
		.send().await?
		.json::<serde_json::Value>().await?;

	println!("{}", response["export"]["id"]);

	Ok(())
	
	// app::run();
}
