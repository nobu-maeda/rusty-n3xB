use serde::Serialize;
use std::fmt::*;

#[typetag::serde(tag = "type")]
pub trait TradeEngineSpecfiicsTrait: Debug {}

#[derive(Clone, Debug)]
pub struct TradeEngineDetails<T: TradeEngineSpecfiicsTrait + Clone + Serialize> {
    pub trade_engine_name: String,
    pub trade_engine_specifics: T,
}
