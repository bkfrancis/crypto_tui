use crate::models::DataList;
use chrono::{DateTime, Local};
use ratatui::{
    buffer::Buffer,
    prelude::Rect,
    style::{Color, Modifier, Style, Stylize},
    symbols,
    widgets::{Axis, Block, Chart, Dataset, Widget},
};
use std::collections::HashMap;

pub struct TkrChart {
    upticks: Vec<(f64, f64)>,
    downticks: Vec<(f64, f64)>,
}

impl TkrChart {
    pub fn new(tkr_data: &DataList) -> Self {
        let mut data: Vec<(i64, f64)> = Vec::new();
        for i in tkr_data.get_order() {
            data.push((
                (tkr_data.data[i].t as f64 / 5_000.0).round() as i64,
                tkr_data.data[i].a.parse::<f64>().unwrap_or(0.0),
            ));
        }

        // create candlestick data
        let mut candle_data: HashMap<i64, [f64; 2]> = HashMap::new();
        for (x, y) in data {
            if let Some([y_open, _]) = candle_data.get(&x) {
                candle_data.insert(x, [*y_open, y]);
            } else {
                candle_data.insert(x, [y, y]);
            }
        }

        let mut upticks: Vec<(f64, f64)> = Vec::new();
        let mut downticks: Vec<(f64, f64)> = Vec::new();
        for (key, value) in candle_data {
            if value[1] > value[0] {
                downticks.push((key as f64, value[1]));
            } else {
                upticks.push((key as f64, value[0]));
            }
        }

        Self { upticks, downticks }
    }
}

impl Widget for TkrChart {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let datasets = vec![
            Dataset::default()
                .marker(symbols::Marker::Block)
                .style(Style::default().fg(Color::Green))
                .data(&self.upticks),
            Dataset::default()
                .marker(symbols::Marker::Block)
                .style(Style::default().fg(Color::Red))
                .data(&self.downticks),
        ];

        let max_x_up = self
            .upticks
            .iter()
            .map(|(x, _)| *x)
            .fold(f64::NEG_INFINITY, f64::max);
        let max_x_down = self
            .downticks
            .iter()
            .map(|(x, _)| *x)
            .fold(f64::NEG_INFINITY, f64::max);
        let max_x = if max_x_up > max_x_down {
            max_x_up
        } else {
            max_x_down
        };
        let min_x = max_x - 50.0;

        let min_y_up = self
            .upticks
            .iter()
            .filter(|(_, y)| *y != 0.0)
            .map(|(_, y)| *y)
            .fold(f64::INFINITY, f64::min);
        let min_y_down = self
            .downticks
            .iter()
            .filter(|(_, y)| *y != 0.0)
            .map(|(_, y)| *y)
            .fold(f64::INFINITY, f64::min);
        let mut min_y = if min_y_up > min_y_down {
            min_y_down
        } else {
            min_y_up
        };
        min_y = (min_y / 10.0).floor() * 10.0;

        let mut max_y_up = self
            .upticks
            .iter()
            .map(|(_, y)| *y)
            .fold(f64::NEG_INFINITY, f64::max);
        let mut max_y_down = self
            .downticks
            .iter()
            .map(|(_, y)| *y)
            .fold(f64::NEG_INFINITY, f64::max);
        let mut max_y = if max_y_up > max_y_down {
            max_y_up
        } else {
            max_y_down
        };
        max_y = (max_y / 10.0).ceil() * 10.0;

        Chart::new(datasets)
            .block(Block::bordered())
            .x_axis(
                Axis::default()
                    .title(format!("min: {}; max: {}", min_x, max_x))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([min_x, max_x]),
            )
            .y_axis(
                Axis::default()
                    .title(format!("min: {}; max: {}", min_y, max_y))
                    .style(Style::default().fg(Color::Gray))
                    .bounds([min_y, max_y]),
            )
            .render(area, buf);
    }
}
