use chrono::Local;

pub struct Note {
    pub text: String,
    pub timestamp: String,
}

impl Note {
    pub fn new(text: String) -> Self {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        Self {
            text,
            timestamp: now,
        }
    }

    pub fn display(&self) {
        println!("[{}] {}", self.timestamp, self.text);
    }
}
