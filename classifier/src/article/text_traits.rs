pub trait TextTraits {
    fn word_count(&self) -> i32;
    fn sentence_count(&self) -> i32;
    fn characters_count(&self) -> i32;
    fn sentence_average_characters_amount(&self) -> f32;
    fn sentence_average_words_amount(&self) -> f32;
}

impl TextTraits for String {
    fn word_count(&self) -> i32 {
        (self.split(" ").count()) as i32
    }

    fn sentence_count(&self) -> i32 {
        (self.split(".").count()) as i32
    }

    fn characters_count(&self) -> i32 {
        self.chars().collect::<Vec<char>>().len() as i32
    }

    fn sentence_average_characters_amount(&self) -> f32 {
        let sentences: Vec<String> = self.split(".").map(|s| s.to_string()).collect();
        let chars: Vec<i32> = sentences.iter().map(|sentence| sentence.characters_count()).collect();
        let sum: i32 = chars.iter().sum();
        sum as f32 / sentences.len() as f32
    }

    fn sentence_average_words_amount(&self) -> f32 {
        let sentences: Vec<String> = self.split(".").map(|s| s.to_string()).collect();
        let words: Vec<i32> = sentences.iter().map(|sentence| sentence.word_count()).collect();
        let sum: i32 = words.iter().sum();
        sum as f32 / sentences.len() as f32
    }
}