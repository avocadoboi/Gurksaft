use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // let response = reqwest::blocking::get("https://en.wiktionary.org/wiki/siellä")?.text()?;
    // let dom = tl::parse(&response, tl::ParserOptions::default())?;
    // let parser = dom.parser();
    // let lists = dom.query_selector("ol").unwrap();

    // for list in lists {
    //     println!("{}", list.get(parser).unwrap().outer_html(parser));
    // }

    let response = reqwest::blocking::get("https://en.wiktionary.org/api/rest_v1/page/definition/yksi?redirect=true")?.text()?;
    let value: serde_json::Value = serde_json::from_str(&response)?;
    for word in value["fi"].as_array().unwrap() {
        // println!("waowwoew: {}\n\n", serde_json::to_string_pretty(word)?);
        println!("{}:", word["partOfSpeech"].as_str().unwrap());
        for definition in word["definitions"].as_array().unwrap() {
            let definition = format!("<html>{}</html>", definition["definition"].as_str().unwrap());
            let html = tl::parse(&definition, tl::ParserOptions::default())?;
            println!("* {}", html.nodes()[0].inner_text(html.parser()));
        }
        // word[""]
    }
    // std::fs::write("word.html", response)?;
    // reqwest::get("https://en.wiktionary.org/wiki/siellä").
    Ok(())
}
