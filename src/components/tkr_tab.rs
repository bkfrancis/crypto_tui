use crate::models::DataList;
use chrono::{DateTime, Local};
use ratatui::{
    buffer::Buffer,
    prelude::{Constraint, Rect},
    style::{Color, Stylize},
    widgets::{Block, Cell, Row, Table, Tabs, Widget},
};
use std::collections::HashMap;

#[derive(Default)]
pub struct TkrTabs {
    pub selected_tab: SelectedTab,
}

impl TkrTabs {
    pub fn select(&mut self, i: usize) {
        match i {
            1 => self.selected_tab = SelectedTab::Tab1,
            2 => self.selected_tab = SelectedTab::Tab2,
            3 => self.selected_tab = SelectedTab::Tab3,
            4 => self.selected_tab = SelectedTab::Tab4,
            5 => self.selected_tab = SelectedTab::Tab5,
            _ => {}
        }
    }

    pub fn widget(&self, watchlist: &Vec<&str>) -> impl Widget {
        let tab_names: Vec<String> = watchlist
            .iter()
            .enumerate()
            .map(|(i, name)| format!("({}) {}", i + 1, name))
            .collect();
        Tabs::new(tab_names).select(self.selected_tab as usize)
    }
}

#[derive(Default, Clone, Copy)]
pub enum SelectedTab {
    #[default]
    Tab1,
    Tab2,
    Tab3,
    Tab4,
    Tab5,
}

impl SelectedTab {
    pub fn widget(
        &self,
        // area: Rect,
        // buf: &mut Buffer,
        data: &HashMap<String, DataList>,
        watchlist: &Vec<&str>,
    ) -> impl Widget {
        match self {
            SelectedTab::Tab1 => self.render_tab(&data[watchlist[0]]),
            SelectedTab::Tab2 => self.render_tab(&data[watchlist[1]]),
            SelectedTab::Tab3 => self.render_tab(&data[watchlist[2]]),
            SelectedTab::Tab4 => self.render_tab(&data[watchlist[3]]),
            SelectedTab::Tab5 => self.render_tab(&data[watchlist[4]]),
        }
    }

    fn render_tab(self, data: &DataList) -> impl Widget {
        let block = Block::bordered().title("Trades");
        let headers = Row::new(["Time", "Price", "Qty 24h", "Bid", "Ask"])
            .bg(Color::Rgb(205, 214, 244))
            .fg(Color::Rgb(17, 17, 27));

        // Color scheme
        let green_color = Color::Rgb(166, 227, 161);
        let red_color = Color::Rgb(243, 139, 168);
        let fg_color = Color::Rgb(24, 24, 27);

        let capacity = data.capacity;
        let mut rows: Vec<Row> = Vec::with_capacity(capacity);

        // Construct rows
        for i in data.get_order() {
            // Reset color
            let mut row_bg_color = Color::Reset;
            let mut row_fg_color = Color::Reset;
            let mut bid_fg_color = Color::Reset;
            let mut ask_fg_color = Color::Reset;

            let i_prior = (capacity - 1) - ((capacity - i) % capacity);
            let row_i = &data.data[i];
            let row_prior = &data.data[i_prior];

            let qty_i = row_i.v.parse::<f64>().unwrap_or(0.0);
            let qty_prior = row_prior.v.parse::<f64>().unwrap_or(0.0);
            let p_i = row_i.a.parse::<f64>().unwrap_or(0.0);
            let p_prior = row_prior.a.parse::<f64>().unwrap_or(0.0);
            let bid_i = row_i.b.parse::<f64>().unwrap_or(0.0);
            let bid_prior = row_prior.b.parse::<f64>().unwrap_or(0.0);
            let ask_i = row_i.k.parse::<f64>().unwrap_or(0.0);
            let ask_prior = row_prior.k.parse::<f64>().unwrap_or(0.0);

            // Bid/ask color
            if bid_i > bid_prior {
                bid_fg_color = green_color;
            }
            if bid_i < bid_prior {
                bid_fg_color = red_color;
            }
            if ask_i > ask_prior {
                ask_fg_color = green_color;
            }
            if ask_i < ask_prior {
                ask_fg_color = red_color;
            }

            // Row color
            if qty_i > qty_prior {
                if p_i > p_prior {
                    row_bg_color = green_color;
                    row_fg_color = fg_color;
                    bid_fg_color = fg_color;
                    ask_fg_color = fg_color;
                } else {
                    row_bg_color = red_color;
                    row_fg_color = fg_color;
                    bid_fg_color = fg_color;
                    ask_fg_color = fg_color;
                }
            }

            rows.push(
                Row::new([
                    Cell::new(
                        DateTime::from_timestamp_millis(row_i.t)
                            .unwrap()
                            .with_timezone(&Local)
                            .format("%H:%M:%S")
                            .to_string(),
                    ),
                    Cell::new(format!("{:.2}", p_i)),   // last price
                    Cell::new(format!("{:.4}", qty_i)), // 24h volume
                    Cell::new(row_i.b.clone()).fg(bid_fg_color), // best bid
                    Cell::new(row_i.k.clone()).fg(ask_fg_color), // best ask
                ])
                .bg(row_bg_color)
                .fg(row_fg_color),
            );
        }

        Table::new(
            rows,
            [
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Length(12),
            ],
        )
        .header(headers)
        .block(block)
    }
}
