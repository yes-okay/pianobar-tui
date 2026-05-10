#[derive(Default)]
pub struct AppState {
    pub should_quit: bool,
    pub logs: Vec<String>,
}