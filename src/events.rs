use crossterm::event::KeyEvent;

pub enum AppEvent {
    Tick,
    Key(KeyEvent),
}