use std::{io, time::Duration};

use anyhow::Result;

use crossterm::{
    event::KeyCode,
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
};

use ratatui::{backend::CrosstermBackend, Terminal};

use tokio::sync::mpsc;

use crate::{
    config::Config,
    events::AppEvent,
    input,
    pianobar::process,
    state::AppState,
    theme::Theme,
    ui,
};

pub async fn run() -> Result<()> {
    enable_raw_mode()?;

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let mut state = AppState {
        should_quit: false,
        logs: vec![],
    };

    let theme = Theme::default();

    let config = Config::load()?;

    state.logs.push(format!(
        "Loaded config: {:?}",
        config.pianobar.executable
    ));

    let (tx, mut rx) = mpsc::unbounded_channel();

    let mut pianobar =
        process::spawn_pianobar(&config.pianobar.executable, tx).await?;

    loop {
        while let Ok(line) = rx.try_recv() {
            state.logs.push(line);

            if state.logs.len() > 100 {
                state.logs.remove(0);
            }
        }

        terminal.draw(|frame| {
            ui::render(frame, &state, &theme);
        })?;

        match input::next_event(Duration::from_millis(100))? {
            AppEvent::Tick => {}

            AppEvent::Key(key) => match key.code {
                KeyCode::Char('q') => {
                    state.should_quit = true;
                }

                KeyCode::Char('n') => {
                    let _ =
                        process::send_command(&mut pianobar.stdin, "n").await;
                }

                KeyCode::Char('p') => {
                    let _ =
                        process::send_command(&mut pianobar.stdin, "p").await;
                }

                KeyCode::Char('+') => {
                    let _ =
                        process::send_command(&mut pianobar.stdin, "+").await;
                }

                KeyCode::Char('-') => {
                    let _ =
                        process::send_command(&mut pianobar.stdin, "-").await;
                }

                KeyCode::Char(c) if c.is_ascii_digit() => {
                    let cmd = c.to_string();

                    let _ =
                        process::send_command(&mut pianobar.stdin, &cmd).await;
                }

                _ => {}
            },
        }

        if state.should_quit {
            break;
        }
    }

    let _ = pianobar.child.kill().await;

    disable_raw_mode()?;

    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    terminal.show_cursor()?;

    Ok(())
}