pub trait Cleaner {
    fn clean(&self) -> String;
}

impl Cleaner for String {
    fn clean(&self) -> String {
        self.trim().to_string()
    }
}