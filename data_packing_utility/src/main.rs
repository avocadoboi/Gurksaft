use std::error::Error;
use std::fs;

use std::collections::HashMap;

fn pack_sentences_with_audio() -> Result<(), Box<dyn Error>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t').has_headers(false)
        .from_path("sentences_with_audio.csv")?;

    let rows = reader.deserialize();

    let mut result: HashMap<u32, Vec<u32>> = HashMap::new();
    
    for row in rows {
        let row: (u32, u32, String, String, String) = row?;
        if let Some(audio_id_vector) = result.get_mut(&row.0) {
            audio_id_vector.push(row.1);
        }
        else {
            result.insert(row.0, vec![row.1]);
        }
    }

    fs::write("audio_ids", bincode::serialize(&result)?)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    pack_sentences_with_audio()?;
    Ok(())
}
