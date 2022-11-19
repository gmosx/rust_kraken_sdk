use serde::{Deserialize, Serialize};
use crate::{client::{IRequest, Event}, types::{OrderSide, OrderType, Channel}};

/// - <https://docs.kraken.com/websockets-v2/#trade>
#[derive(Debug, Serialize)]
pub struct SubscribeTradeRequest<'a> {
    pub channel: Channel,
    pub symbol: &'a [&'a str],
    /// Request a snapshot after subscribing.
    /// Default: true
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
}

impl IRequest for SubscribeTradeRequest<'_> {
    fn method(&self) -> &'static str {
        "subscribe"
    }
}

impl SubscribeTradeRequest<'_> {
    pub fn new<'a>(symbol: &'a[&'a str]) -> SubscribeTradeRequest<'a> {
        SubscribeTradeRequest {
            channel: Channel::Trade,
            symbol,
            snapshot: None,
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            snapshot: Some(snapshot),
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TradeData {
    pub ord_type: OrderType,
    pub price: f64,
    pub qty: f64,
    pub side: OrderSide,
    pub symbol: String,
    pub timestamp: String,
    pub trade_id: i64,
}

pub type TradeEvent = Event<Vec<TradeData>>;