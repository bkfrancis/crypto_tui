use crate::components::summary;
use crate::components::tkr_tab::TkrTabs;
use crate::models::{DataList, TkrResult};
use anyhow::Result;
use cli_log::*;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    prelude::Rect,
    widgets::{Paragraph, Widget},
    DefaultTerminal,
};
use std::cmp::min;
use std::collections::HashMap;
use tokio::sync::mpsc::Receiver;

#[derive(PartialEq)]
enum AppState {
    Running,
    Quitting,
}

pub struct Tui<'a> {
    rx: Receiver<TkrResult>,
    state: AppState,
    tkr_tabs: TkrTabs,
    tkr_data: HashMap<String, DataList>,
    watchlist: Vec<&'a str>,
}

impl<'a> Tui<'a> {
    pub fn new(rx: Receiver<TkrResult>, watchlist: Vec<&'a str>) -> Self {
        Self {
            rx,
            state: AppState::Running,
            tkr_tabs: TkrTabs::default(),
            tkr_data: HashMap::new(),
            watchlist,
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        info!("Starting Tui");
        for tkr in &self.watchlist {
            let tkr_data = DataList::new(100);
            self.tkr_data.insert(tkr.to_string(), tkr_data);
        }

        while self.state == AppState::Running {
            match self.rx.try_recv() {
                Ok(tkr_result) => {
                    if let Some(data) = self.tkr_data.get_mut(&tkr_result.tkr) {
                        data.insert(&tkr_result);
                    }
                    info!("tkr_result: {:#?}", tkr_result);
                }
                Err(_e) => {}
            }
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_event()?;

            tokio::task::yield_now().await;
        }
        Ok(())
    }

    fn handle_event(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => {
                            self.state = AppState::Quitting;
                        }
                        KeyCode::Char('1') => {
                            self.tkr_tabs.select(min(1, self.watchlist.len()));
                        }
                        KeyCode::Char('2') => {
                            self.tkr_tabs.select(min(2, self.watchlist.len()));
                        }
                        KeyCode::Char('3') => {
                            self.tkr_tabs.select(min(3, self.watchlist.len()));
                        }
                        KeyCode::Char('4') => {
                            self.tkr_tabs.select(min(4, self.watchlist.len()));
                        }
                        KeyCode::Char('5') => {
                            self.tkr_tabs.select(min(5, self.watchlist.len()));
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}

impl<'a> Widget for &Tui<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(area);

        let [_title_area, tabs_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(header_area);

        let [left_area, right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area);

        Paragraph::new("Crypto Dashboard").render(header_area, buf);
        Paragraph::new("Press (q) to quit...").render(footer_area, buf);
        summary::render(left_area, buf, &self.tkr_data);
        self.tkr_tabs.render(tabs_area, buf, &self.watchlist);
        self.tkr_tabs
            .selected_tab
            .render(right_area, buf, &self.tkr_data, &self.watchlist);
    }
}
