use anyhow::Result;

mod app;
mod config;
mod events;
mod input;
mod parser;
mod state;
mod theme;
mod ui;

mod pianobar;

#[tokio::main]
async fn main() -> Result<()> {
    app::run().await
}