
// Case sensitive.
pub fn contains_word(sentence: &str, word_to_find: &str) -> bool {
	sentence.split_ascii_whitespace().any(|word| word == word_to_find)
}

// pub fn find_word_position(sentence: &str, word_to_find: &str) -> Option<usize> {
// 	if let Some(word) = sentence.split_ascii_whitespace().find(|&word| word == word_to_find) {
// 		let byte_offset = word.as_ptr() as usize - sentence.as_ptr() as usize;
// 		Some(sentence[..byte_offset].chars().count())
// 	}
// 	else {
// 		None
// 	}
// }

// This assumes that 'word_in_sentence' is a slice within 'sentence'
pub fn get_word_position(sentence: &str, word_in_sentence: &str) -> usize {
	let byte_offset = word_in_sentence.as_ptr() as usize - sentence.as_ptr() as usize;
	sentence[..byte_offset].chars().count()
}

// #[cfg(test)]
// mod tests {
// 	use super::*;
	
// 	#[test]
// 	fn test_find_word_position() {
// 		assert_eq!(find_word_position("åäöasdf hej", "hej"), Some(8));
// 		assert_eq!(find_word_position("åäöasdf hej", "asdf"), None);
// 		assert_eq!(find_word_position("åäöasdf hej", "åäöasdf"), Some(0));
// 		assert_eq!(find_word_position("hej åäöasdf", "åäöasdf"), Some(4));
// 	}
// }
