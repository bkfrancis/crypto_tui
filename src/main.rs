use anyhow::Result;
use cli_log::*;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    Terminal,
};
use std::env;
use std::io::stdout;
use tokio::sync::mpsc::{self, Receiver, Sender};

mod tui;
use tui::Tui;
mod ws_client;
use ws_client::WsClient;
mod models;
use models::TkrResult;
mod components;

const WS_URL: &str = "wss://stream.crypto.com/exchange/v1/market";

#[tokio::main]
async fn main() -> Result<()> {
    init_cli_log!();
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    let (tx, rx): (Sender<TkrResult>, Receiver<TkrResult>) = mpsc::channel(5);

    // Get watchlist from args
    let mut watchlist = vec!["BTCUSD-PERP", "ETHUSD-PERP"];
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        watchlist = args[1..].iter().map(|tkr| tkr.as_str()).collect();
    }

    let mut ws_client = match WsClient::new(WS_URL, tx, watchlist.clone()).connect().await {
        Ok(ws) => ws,
        Err(e) => {
            debug!("WS Error: {}", e);
            // clean up terminal on websocket connection error
            stdout().execute(LeaveAlternateScreen)?;
            disable_raw_mode()?;

            return Err(e);
        }
    };

    let tui = Tui::new(rx, &watchlist);

    // Run concurrent
    let result = tokio::try_join!(tui.run(terminal), ws_client.run());
    match result {
        Ok((_tui, _ws)) => {}
        Err(e) => debug!("Tasks interrupted: {}", e),
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
