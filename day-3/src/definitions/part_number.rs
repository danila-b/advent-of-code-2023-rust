#[derive(Debug)]
pub struct PartNumber {
    pub value: String,
    pub start_idx: usize,
    pub end_idx: usize,
}

impl PartNumber {
    pub fn add_value_char(&mut self, char: char) {
        self.value.push(char);
    }

    pub fn set_end_idx(&mut self, end_idx: usize) {
        self.end_idx = end_idx;
    }
}
