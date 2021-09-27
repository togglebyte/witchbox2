pub struct Filter;

impl Filter {
    pub fn new() -> Self {
        Self {
            
        }
    }

    pub fn filter(&mut self, event: crate::Event) -> Option<crate::Event> {
        Some(event)
    }
}
