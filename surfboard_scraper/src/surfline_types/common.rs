#[derive(Clone)]
pub struct FetchParams {
    pub days: u32,
    pub interval_hours: u32,
}

impl Default for FetchParams {
    fn default() -> Self {
        Self {
            days: 2,
            interval_hours: 1,
        }
    }
}
