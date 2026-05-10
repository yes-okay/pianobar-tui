use std::time::Duration;

use anyhow::Result;
use crossterm::event::{self, Event as CrosstermEvent};

use crate::events::AppEvent;

pub fn next_event(tick_rate: Duration) -> Result<AppEvent> {
    if event::poll(tick_rate)? {
        if let CrosstermEvent::Key(key) = event::read()? {
            return Ok(AppEvent::Key(key));
        }
    }

    Ok(AppEvent::Tick)
}