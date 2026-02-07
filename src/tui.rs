use crate::components::chart;
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
    DefaultTerminal, Frame,
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
    watchlist: &'a Vec<&'a str>,
}

impl<'a> Tui<'a> {
    pub fn new(rx: Receiver<TkrResult>, watchlist: &'a Vec<&'a str>) -> Self {
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
        for tkr in self.watchlist {
            let tkr_data = DataList::new(1_000);
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
            terminal.draw(|frame| self.render(frame))?;
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

    // fn render(self, area: Rect, buf: &mut Buffer) {
    fn render(&mut self, frame: &mut Frame) {
        let [header_area, main_area, footer_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let [_title_area, tabs_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(header_area);

        let [left_area, right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area);
        let [top_left_area, bottom_left_area] =
            Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(left_area);

        frame.render_widget(Paragraph::new("Crypto Dashboard"), frame.area());
        frame.render_widget(Paragraph::new("Press (q) to quit..."), frame.area());
        frame.render_widget(summary::Summary::new(&self.tkr_data), top_left_area);
        frame.render_widget(
            chart::TkrChart::new(
                &self.tkr_data[self.watchlist[self.tkr_tabs.selected_tab as usize]],
            ),
            right_area,
        );
        frame.render_widget(self.tkr_tabs.widget(&self.watchlist), tabs_area);
        frame.render_widget(
            self.tkr_tabs
                .selected_tab
                .widget_trades(&self.tkr_data, &self.watchlist),
            bottom_left_area,
        );
    }
}
