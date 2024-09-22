use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
    WebSocketStream,
    MaybeTlsStream,
};
use tokio::net::TcpStream;
use tokio::sync::mpsc::Sender;
use futures_util::{SinkExt, StreamExt};
use serde::{Serialize, Deserialize};
use serde_json;
use anyhow::{anyhow, Result};
use cli_log::*;
use crate::models::{TkrResponse, TkrResult};


#[derive(Deserialize)]
struct Heartbeat {
    id: u64,
    // method: String,
    // code: i64,
}


pub struct WsClient<'a> {
    url: &'a str,
    tx: Sender<TkrResult>,
}

impl<'a> WsClient<'a> {
    pub fn new(url: &'a str, tx: Sender<TkrResult>) -> Self {
        Self {
            url,
            tx,
        }
    }

    pub async fn connect(self) -> Result<WsClientConnected<'a>> {
        let (ws_stream, _resp) = connect_async(self.url).await?;

        Ok(WsClientConnected {
            url: self.url,
            ws_stream,
            tx: self.tx,
        })
    }
}


pub struct WsClientConnected<'a> {
    url: &'a str,
    tx: Sender<TkrResult>,
    ws_stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

impl<'a> WsClientConnected<'a> {
    pub async fn run(&mut self) -> Result<()> {
        debug!("Connected ws_client");

        self.subscribe_tkr().await?;
    
        while let Some(msg) = self.ws_stream.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    let json_value: serde_json::Value = serde_json::from_str(&text)?;
                   
                    let method = json_value.get("method").expect("none");
                    if method == "subscribe" {
                        let tkr_resp: TkrResponse = serde_json::from_value(json_value)?;
                        // debug!("{:#?}", tkr_resp);
                        self.tx.send(tkr_resp.result).await?;
                    } else if method == "public/heartbeat" {
                        let heartbeat: Heartbeat = serde_json::from_value(json_value)?;
                        self.heartbeat_response(heartbeat.id).await?;

                    } else {
                        debug!("Unmatched json: {:#?}", json_value);

                    }
                }
                Err(e) => return Err(anyhow!(e)),
                _ => {},    // binary, pong, ping, etc
            }
        }
        Ok(())
    }

    async fn subscribe_tkr(&mut self) -> Result<()> {
        let tkr_sub = serde_json::json!({
            "id": 1,
            "method": "subscribe",
            "params": {"channels": ["ticker.BTCUSD-PERP"]},
            "nonce": 1000,
        });
        self.ws_stream.send(Message::Text(tkr_sub.to_string())).await?;
        
        let tkr_sub = serde_json::json!({
            "id": 1,
            "method": "subscribe",
            "params": {"channels": ["ticker.ETHUSD-PERP"]},
            "nonce": 1001,
        });
        self.ws_stream.send(Message::Text(tkr_sub.to_string())).await?;
        Ok(())
    }

    async fn heartbeat_response(&mut self, id: u64) -> Result<()> {
        let heartbeat = serde_json::json!({
            "id": id,
            "method": "public/respond-heartbeat",
        });
        self.ws_stream.send(Message::Text(heartbeat.to_string())).await?;
        Ok(())
    }
}
