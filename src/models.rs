// TODO
// - parse TkrData out of String

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TkrData {
    pub h: String,  // Price of the 24h highest trade
    pub l: String,  // Price of the 24h lowest trade, null if there weren't any trades
    pub a: String,  // The price of the latest trade, null if there weren't any trades
    pub i: String,  // Instrument name
    pub v: String,  // The total 24h traded volume
    pub vv: String, // The total 24h traded volume value (in USD)
    pub oi: String, // Open interest
    pub c: String,  // 24-hour price change, null if there weren't any trades
    pub b: String,  // The current best bid price, null if there aren't any bids
    pub k: String,  // The current best ask price, null if there aren't any asks
    pub t: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TkrResult {
    #[serde(rename = "instrument_name")]
    pub tkr: String,
    subscription: String,
    channel: String,
    pub data: Vec<TkrData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TkrResponse {
    id: i64,
    method: String,
    code: i64,
    pub result: TkrResult,
}

#[derive(Clone)]
pub struct DataList {
    pub capacity: usize,
    insert_i: usize,
    pub curr_i: usize,
    pub data: Vec<TkrData>,
}

impl DataList {
    pub fn new(n: usize) -> Self {
        Self {
            capacity: n,
            insert_i: 0,
            curr_i: 0,
            data: vec![TkrData::default(); n],
        }
    }

    pub fn insert(&mut self, tkr_result: &TkrResult) {
        let data = tkr_result.data[0].clone();
        self.data[self.insert_i] = data;
        self.insert_i = (self.insert_i + 1) % self.capacity;
        self.curr_i = (self.capacity - 1) - ((self.capacity - self.insert_i) % self.capacity)
    }

    pub fn get_order(&self) -> Vec<usize> {
        let mut i = self.curr_i;
        let mut order = Vec::with_capacity(self.capacity);
        for _ in 0..(self.capacity - 1) {
            order.push(i);
            i = (self.capacity - 1) - ((self.capacity - i) % self.capacity);
        }
        order
    }
}
