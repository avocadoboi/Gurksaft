
pub fn create_parent_directory_if_nonexistent(path: &str) {
	if let Some(directory) = std::path::Path::new(path).parent() {
		if !directory.exists() {
			std::fs::create_dir(directory).unwrap();
		}
	}
}
pub fn contains_word(sentence: &str, word_to_find: &str) -> bool {
	sentence.to_lowercase().split_whitespace().any(|word| word == word_to_find)
}
pub fn find_word_position(sentence: &str, word_to_find: &str) -> Option<usize> {
	let lowercase = sentence.to_lowercase();
	if let Some(word) = lowercase.split_whitespace().find(|&word| word == word_to_find) {
		let byte_offset = word.as_ptr() as usize - lowercase.as_ptr() as usize;
		Some(lowercase[..byte_offset].chars().count())
	}
	else {
		None
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_find_word_position() {
		assert_eq!(find_word_position("åäöasdf hej", "hej"), Some(8));
		assert_eq!(find_word_position("åäöasdf hej", "asdf"), None);
		assert_eq!(find_word_position("åäöasdf hej", "åäöasdf"), Some(0));
		assert_eq!(find_word_position("hej åäöasdf", "åäöasdf"), Some(4));
	}
}
