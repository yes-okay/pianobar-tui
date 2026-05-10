use std::{path::Path, process::Stdio};

use anyhow::{Context, Result};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    process::{Child, ChildStdin, Command},
    sync::mpsc,
};

pub struct PianobarProcess {
    pub child: Child,
    pub stdin: ChildStdin,
}

pub async fn spawn_pianobar(
    executable: &Path,
    sender: mpsc::UnboundedSender<String>,
) -> Result<PianobarProcess> {
    let mut child = Command::new(executable)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("failed to spawn pianobar")?;

    let mut stdout = child
        .stdout
        .take()
        .context("failed to capture stdout")?;

    let mut stderr = child
        .stderr
        .take()
        .context("failed to capture stderr")?;

    let stdin = child
        .stdin
        .take()
        .context("failed to capture stdin")?;

    let tx_stdout = sender.clone();

    tokio::spawn(async move {
        let mut buffer = [0u8; 1024];

        loop {
            match stdout.read(&mut buffer).await {
                Ok(0) => break,

                Ok(n) => {
                    let text =
                        String::from_utf8_lossy(&buffer[..n]);

                    for part in text.split(['\n', '\r']) {
                        let trimmed = part.trim();

                        if !trimmed.is_empty() {
                            let _ = tx_stdout.send(format!(
                                "[stdout] {}",
                                trimmed
                            ));
                        }
                    }
                }

                Err(_) => break,
            }
        }
    });

    tokio::spawn(async move {
        let mut buffer = [0u8; 1024];

        loop {
            match stderr.read(&mut buffer).await {
                Ok(0) => break,

                Ok(n) => {
                    let text =
                        String::from_utf8_lossy(&buffer[..n]);

                    for part in text.split(['\n', '\r']) {
                        let trimmed = part.trim();

                        if !trimmed.is_empty() {
                            let _ = sender.send(format!(
                                "[stderr] {}",
                                trimmed
                            ));
                        }
                    }
                }

                Err(_) => break,
            }
        }
    });

    Ok(PianobarProcess { child, stdin })
}

pub async fn send_command(
    stdin: &mut ChildStdin,
    command: &str,
) -> Result<()> {
    stdin.write_all(command.as_bytes()).await?;

    stdin.write_all(b"\n").await?;

    stdin.flush().await?;

    Ok(())
}