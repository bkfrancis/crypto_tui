use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::{Constraint, Layout},
    buffer::Buffer,
    prelude::Rect,
    widgets::{
        Widget,
        Paragraph,
    },
    DefaultTerminal,
};
use tokio::sync::mpsc::Receiver;
use anyhow::Result;
use cli_log::*;
use crate::components::summary;
use crate::components::tkr_tab::TkrTabs;
use crate::models::{CryptoData, DataList, TkrResult};


#[derive(PartialEq)]
enum AppState {
    Running,
    Quitting,
}

pub struct Tui<'a> {
    rx: Receiver<TkrResult>,
    state: AppState,
    tkr_tabs: TkrTabs,
    tkr_data: Vec<CryptoData<'a>>,
}

impl<'a> Tui<'a> {
    pub fn new(rx: Receiver<TkrResult>) -> Self{
        Self {
            rx,
            state: AppState::Running,
            tkr_tabs: TkrTabs::default(),
            tkr_data: Vec::new()
        }
    }

    pub async fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        info!("Starting Tui");
        let tkr_data_1 = CryptoData {tkr: "BTC", data_list: DataList::new(100)};
        let tkr_data_2 = CryptoData {tkr: "ETH", data_list: DataList::new(100)};
        self.tkr_data.push(tkr_data_1);
        self.tkr_data.push(tkr_data_2);

        while self.state == AppState::Running {
            match self.rx.try_recv() {
                Ok(tkr_result) => {
                    if tkr_result.tkr == "BTCUSD-PERP" {
                        self.tkr_data[0].data_list.insert(&tkr_result);
                    }
                    if tkr_result.tkr == "ETHUSD-PERP" {
                        self.tkr_data[1].data_list.insert(&tkr_result);
                    }
                    info!("tkr_result: {:#?}", tkr_result);
                },
                Err(_e) => {},
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
                        },
                        KeyCode::Char('1') => {
                            self.tkr_tabs.select(1);
                        },
                        KeyCode::Char('2') => {
                            self.tkr_tabs.select(2);
                        },
                        _ => {},
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
        ]).areas(area);

        let [_title_area, tabs_area] = Layout::horizontal([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ]).areas(header_area);

        let [left_area, right_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(main_area);

        Paragraph::new("Crypto Dashboard").render(header_area, buf); 
        Paragraph::new("Press (q) to quit...").render(footer_area, buf); 
        summary::render(left_area, buf, &self.tkr_data);
        self.tkr_tabs.render_tabs(tabs_area, buf);
        self.tkr_tabs.selected_tab.render(right_area, buf, &self.tkr_data);
    }
}
