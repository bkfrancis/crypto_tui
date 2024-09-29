use crate::models::DataList;
use ratatui::{
    buffer::Buffer,
    prelude::{Constraint, Rect},
    style::{Color, Stylize},
    widgets::{Block, Cell, Row, Table, Widget},
};
use std::collections::HashMap;

pub fn render(area: Rect, buf: &mut Buffer, data: &HashMap<String, DataList>) {
    let block = Block::bordered().title("Summary");
    let headers = Row::new(["Tkr", "Price", "Qty 24h", "Bid", "Ask"])
        .bg(Color::Rgb(205, 214, 244))
        .fg(Color::Rgb(17, 17, 27));

    let mut rows: Vec<Row> = Vec::with_capacity(data.capacity());
    for (key, data_i) in data.iter() {
        let curr_i = data_i.curr_i;
        rows.push(Row::new([
            Cell::new(key.to_string()),
            Cell::new(data_i.data[curr_i].a.clone()),
            Cell::new(data_i.data[curr_i].v.clone()),
            Cell::new(data_i.data[curr_i].b.clone()),
            Cell::new(data_i.data[curr_i].k.clone()),
        ]));
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
    .block(block)
    .header(headers)
    .render(area, buf);
}
