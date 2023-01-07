pub trait TextTraits {
    fn word_count(&self) -> i32;
    fn sentence_count(&self) -> i32;
    fn characters_count(&self) -> i32;
}

impl TextTraits for String {
    fn word_count(&self) -> i32 {
        self.split(" ").count() as i32
    }

    fn sentence_count(&self) -> i32 {
        self.split(".").count() as i32
    }

    fn characters_count(&self) -> i32 {
        self.chars().collect::<Vec<char>>().len() as i32
    }
}