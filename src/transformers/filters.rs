use super::ChatFilter;

pub struct Filters {
    pub chat_filter: ChatFilter,
}

impl Filters {
    pub fn new() -> Self {
        Self {
            chat_filter: ChatFilter::new(),
        }
    }
}
