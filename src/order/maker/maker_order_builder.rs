use super::{maker_order::*, obligation::*, trade_details::*, trade_engine_details::*};
use crate::{error::*, nostr::*};
use serde::Serialize;

pub struct MakerOrderBuilder<'a, T: TradeEngineSpecfiicsTrait + Clone + Serialize> {
    event_msg_client: &'a ArcClient,
    // DB

    // Trade Specific Parameters
    trade_uuid: Option<String>, // TODO: Change to UUID type
    maker_obligation: Option<MakerObligation>,
    taker_obligation: Option<TakerObligation>,
    trade_details: Option<TradeDetails>,
    engine_details: Option<TradeEngineDetails<T>>,
    pow_difficulty: Option<u64>,
}

impl<'a, T: TradeEngineSpecfiicsTrait + Clone + Serialize> MakerOrderBuilder<'a, T> {
    pub fn new(event_msg_client: &'a ArcClient, // DB
    ) -> Self {
        MakerOrderBuilder {
            event_msg_client,
            trade_uuid: Option::<String>::None,
            maker_obligation: Option::<MakerObligation>::None,
            taker_obligation: Option::<TakerObligation>::None,
            trade_details: Option::<TradeDetails>::None,
            engine_details: Option::<TradeEngineDetails<T>>::None,
            pow_difficulty: Option::<u64>::None,
        }
    }

    pub fn trade_uuid(&mut self, trade_uuid: impl Into<String>) -> &mut Self {
        self.trade_uuid = Some(trade_uuid.into());
        self
    }

    pub fn maker_obligation(&mut self, maker_obligation: impl Into<MakerObligation>) -> &mut Self {
        self.maker_obligation = Some(maker_obligation.into());
        self
    }

    pub fn taker_obligation(&mut self, taker_obligation: impl Into<TakerObligation>) -> &mut Self {
        self.taker_obligation = Some(taker_obligation.into());
        self
    }

    pub fn trade_details(&mut self, trade_details: impl Into<TradeDetails>) -> &mut Self {
        self.trade_details = Some(trade_details.into());
        self
    }

    pub fn engine_details(
        &mut self,
        engine_details: impl Into<TradeEngineDetails<T>>,
    ) -> &mut Self {
        self.engine_details = Some(engine_details.into());
        self
    }

    pub fn pow_difficulty(&mut self, pow_difficulty: impl Into<u64>) -> &mut Self {
        self.pow_difficulty = Some(pow_difficulty.into());
        self
    }

    pub fn build(&self) -> std::result::Result<MakerOrder<T>, N3xbError> {
        let Some(trade_uuid) = self.trade_uuid.as_ref() else {
      return Err(N3xbError::Other("No Trade UUID".to_string()));  // TODO: Error handling?
    };

        let Some(maker_obligation) = self.maker_obligation.as_ref() else {
      return Err(N3xbError::Other("No Maker Obligations defined".to_string()));  // TODO: Error handling?
    };

        let Some(taker_obligation) = self.taker_obligation.as_ref() else {
      return Err(N3xbError::Other("No Taker Obligations defined".to_string()));  // TODO: Error handling?
    };

        let Some(trade_details) = self.trade_details.as_ref() else {
      return Err(N3xbError::Other("No Trade Details defined".to_string()));  // TODO: Error handling?
    };

        let Some(engine_details) = self.engine_details.as_ref() else {
      return Err(N3xbError::Other("No Engine Details defined".to_string()));  // TODO: Error handling?
    };

        let pow_difficulty = self.pow_difficulty.unwrap_or_else(|| 0);

        Ok(MakerOrder::new(
            self.event_msg_client,
            trade_uuid.to_owned(),
            maker_obligation.to_owned(),
            taker_obligation.to_owned(),
            trade_details.to_owned(),
            engine_details.to_owned(),
            pow_difficulty,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::super::common::test::*;
    use super::*;
    use core::panic;
    use std::sync::{Arc, Mutex};

    #[tokio::test]
    async fn maker_order_builder_build() {
        let client = new_event_msg_client();
        let mut builder: MakerOrderBuilder<SomeTradeEngineSpecifics> =
            MakerOrderBuilder::new(&client);

        builder.trade_uuid(SomeTestParams::some_uuid_string());

        builder.maker_obligation(MakerObligation {
            kind: SomeTestParams::maker_obligation_kind(),
            content: SomeTestParams::maker_obligation_content(),
        });

        builder.taker_obligation(TakerObligation {
            kind: SomeTestParams::taker_obligation_kind(),
            content: SomeTestParams::taker_obligation_content(),
        });

        builder.trade_details(TradeDetails {
            parameters: SomeTestParams::trade_parameters(),
            content: SomeTestParams::trade_details_content(),
        });

        builder.engine_details(TradeEngineDetails {
            trade_engine_name: SomeTestParams::engine_name_str(),
            trade_engine_specifics: SomeTradeEngineSpecifics {
                test_specific_field: SomeTestParams::engine_specific_str(),
            },
        });

        builder.pow_difficulty(SomeTestParams::pow_difficulty());

        let result = builder.build();

        match result {
            Ok(maker_order) => {
                assert_eq!(maker_order.trade_uuid, SomeTestParams::some_uuid_string());
                assert_eq!(
                    maker_order.maker_obligation.kind,
                    SomeTestParams::maker_obligation_kind()
                );
                assert_eq!(
                    maker_order.maker_obligation.content,
                    SomeTestParams::maker_obligation_content()
                );
                assert_eq!(
                    maker_order.taker_obligation.kind,
                    SomeTestParams::taker_obligation_kind()
                );
                assert_eq!(
                    maker_order.taker_obligation.content,
                    SomeTestParams::taker_obligation_content()
                );
                assert_eq!(
                    maker_order.trade_details.parameters,
                    SomeTestParams::trade_parameters()
                );
                assert_eq!(
                    maker_order.trade_details.content,
                    SomeTestParams::trade_details_content()
                );
                assert_eq!(
                    maker_order.engine_details.trade_engine_name,
                    SomeTestParams::engine_name_str()
                );
                assert_eq!(
                    maker_order
                        .engine_details
                        .trade_engine_specifics
                        .test_specific_field,
                    SomeTestParams::engine_specific_str()
                );
                assert_eq!(maker_order.pow_difficulty, SomeTestParams::pow_difficulty());
            }
            Err(error) => {
                panic!(
                    "maker_order_builder_build failed on builder.build() - {}",
                    error.to_string()
                );
            }
        }
    }

    #[tokio::test]
    async fn maker_order_builder_build_trade_uuid_missing() {
        let client = new_event_msg_client();
        let mut builder: MakerOrderBuilder<SomeTradeEngineSpecifics> =
            MakerOrderBuilder::new(&client);

        builder.maker_obligation(MakerObligation {
            kind: SomeTestParams::maker_obligation_kind(),
            content: SomeTestParams::maker_obligation_content(),
        });

        builder.taker_obligation(TakerObligation {
            kind: SomeTestParams::taker_obligation_kind(),
            content: SomeTestParams::taker_obligation_content(),
        });

        builder.trade_details(TradeDetails {
            parameters: SomeTestParams::trade_parameters(),
            content: SomeTestParams::trade_details_content(),
        });

        builder.engine_details(TradeEngineDetails {
            trade_engine_name: SomeTestParams::engine_name_str(),
            trade_engine_specifics: SomeTradeEngineSpecifics {
                test_specific_field: SomeTestParams::engine_specific_str(),
            },
        });

        builder.pow_difficulty(SomeTestParams::pow_difficulty());

        let result = builder.build();

        match result {
            Ok(_) => {
                panic!("maker_order_builder_build should not contain trade_uuid and should not result in Ok");
            }
            Err(_) => {} // TODO: Some way to check on Error returned, without hard coupling to Error handling methodology
        }
    }

    #[tokio::test]
    async fn maker_order_builder_build_maker_obligation_missing() {
        let client = new_event_msg_client();
        let mut builder: MakerOrderBuilder<SomeTradeEngineSpecifics> =
            MakerOrderBuilder::new(&client);

        builder.trade_uuid(SomeTestParams::some_uuid_string());

        builder.taker_obligation(TakerObligation {
            kind: SomeTestParams::taker_obligation_kind(),
            content: SomeTestParams::taker_obligation_content(),
        });

        builder.trade_details(TradeDetails {
            parameters: SomeTestParams::trade_parameters(),
            content: SomeTestParams::trade_details_content(),
        });

        builder.engine_details(TradeEngineDetails {
            trade_engine_name: SomeTestParams::engine_name_str(),
            trade_engine_specifics: SomeTradeEngineSpecifics {
                test_specific_field: SomeTestParams::engine_specific_str(),
            },
        });

        builder.pow_difficulty(SomeTestParams::pow_difficulty());

        let result = builder.build();

        match result {
            Ok(_) => {
                panic!("maker_order_builder_build should not contain maker_obligation and should not result in Ok");
            }
            Err(_) => {} // TODO: Some way to check on Error returned, without hard coupling to Error handling methodology
        }
    }

    #[tokio::test]
    async fn maker_order_builder_build_taker_obligation_missing() {
        let client = new_event_msg_client();
        let mut builder: MakerOrderBuilder<SomeTradeEngineSpecifics> =
            MakerOrderBuilder::new(&client);

        builder.trade_uuid(SomeTestParams::some_uuid_string());

        builder.maker_obligation(MakerObligation {
            kind: SomeTestParams::maker_obligation_kind(),
            content: SomeTestParams::maker_obligation_content(),
        });

        builder.trade_details(TradeDetails {
            parameters: SomeTestParams::trade_parameters(),
            content: SomeTestParams::trade_details_content(),
        });

        builder.engine_details(TradeEngineDetails {
            trade_engine_name: SomeTestParams::engine_name_str(),
            trade_engine_specifics: SomeTradeEngineSpecifics {
                test_specific_field: SomeTestParams::engine_specific_str(),
            },
        });

        builder.pow_difficulty(SomeTestParams::pow_difficulty());

        let result = builder.build();

        match result {
            Ok(_) => {
                panic!("maker_order_builder_build should not contain taker_obligation and should not result in Ok");
            }
            Err(_) => {} // TODO: Some way to check on Error returned, without hard coupling to Error handling methodology
        }
    }

    #[tokio::test]
    async fn maker_order_builder_build_trade_details_missing() {
        let client = new_event_msg_client();
        let mut builder: MakerOrderBuilder<SomeTradeEngineSpecifics> =
            MakerOrderBuilder::new(&client);

        builder.trade_uuid(SomeTestParams::some_uuid_string());

        builder.maker_obligation(MakerObligation {
            kind: SomeTestParams::maker_obligation_kind(),
            content: SomeTestParams::maker_obligation_content(),
        });

        builder.taker_obligation(TakerObligation {
            kind: SomeTestParams::taker_obligation_kind(),
            content: SomeTestParams::taker_obligation_content(),
        });

        builder.engine_details(TradeEngineDetails {
            trade_engine_name: SomeTestParams::engine_name_str(),
            trade_engine_specifics: SomeTradeEngineSpecifics {
                test_specific_field: SomeTestParams::engine_specific_str(),
            },
        });

        builder.pow_difficulty(SomeTestParams::pow_difficulty());

        let result = builder.build();

        match result {
            Ok(_) => {
                panic!("maker_order_builder_build should not contain trade_details and should not result in Ok");
            }
            Err(_) => {} // TODO: Some way to check on Error returned, without hard coupling to Error handling methodology
        }
    }

    #[tokio::test]
    async fn maker_order_builder_build_engine_details_missing() {
        let client = new_event_msg_client();
        let mut builder: MakerOrderBuilder<SomeTradeEngineSpecifics> =
            MakerOrderBuilder::new(&client);

        builder.trade_uuid(SomeTestParams::some_uuid_string());

        builder.maker_obligation(MakerObligation {
            kind: SomeTestParams::maker_obligation_kind(),
            content: SomeTestParams::maker_obligation_content(),
        });

        builder.taker_obligation(TakerObligation {
            kind: SomeTestParams::taker_obligation_kind(),
            content: SomeTestParams::taker_obligation_content(),
        });

        builder.trade_details(TradeDetails {
            parameters: SomeTestParams::trade_parameters(),
            content: SomeTestParams::trade_details_content(),
        });

        builder.pow_difficulty(SomeTestParams::pow_difficulty());

        let result = builder.build();

        match result {
            Ok(_) => {
                panic!("maker_order_builder_build should not contain engine_details and should not result in Ok");
            }
            Err(_) => {} // TODO: Some way to check on Error returned, without hard coupling to Error handling methodology
        }
    }

    #[tokio::test]
    async fn maker_order_builder_build_pow_difficulty_default() {
        let client = new_event_msg_client();
        let mut builder: MakerOrderBuilder<SomeTradeEngineSpecifics> =
            MakerOrderBuilder::new(&client);

        builder.trade_uuid(SomeTestParams::some_uuid_string());

        builder.maker_obligation(MakerObligation {
            kind: SomeTestParams::maker_obligation_kind(),
            content: SomeTestParams::maker_obligation_content(),
        });

        builder.taker_obligation(TakerObligation {
            kind: SomeTestParams::taker_obligation_kind(),
            content: SomeTestParams::taker_obligation_content(),
        });

        builder.trade_details(TradeDetails {
            parameters: SomeTestParams::trade_parameters(),
            content: SomeTestParams::trade_details_content(),
        });

        builder.engine_details(TradeEngineDetails {
            trade_engine_name: SomeTestParams::engine_name_str(),
            trade_engine_specifics: SomeTradeEngineSpecifics {
                test_specific_field: SomeTestParams::engine_specific_str(),
            },
        });

        let result = builder.build();

        match result {
            Ok(maker_order) => {
                assert_eq!(maker_order.trade_uuid, SomeTestParams::some_uuid_string());
                assert_eq!(
                    maker_order.maker_obligation.kind,
                    SomeTestParams::maker_obligation_kind()
                );
                assert_eq!(
                    maker_order.maker_obligation.content,
                    SomeTestParams::maker_obligation_content()
                );
                assert_eq!(
                    maker_order.taker_obligation.kind,
                    SomeTestParams::taker_obligation_kind()
                );
                assert_eq!(
                    maker_order.taker_obligation.content,
                    SomeTestParams::taker_obligation_content()
                );
                assert_eq!(
                    maker_order.trade_details.parameters,
                    SomeTestParams::trade_parameters()
                );
                assert_eq!(
                    maker_order.trade_details.content,
                    SomeTestParams::trade_details_content()
                );
                assert_eq!(
                    maker_order.engine_details.trade_engine_name,
                    SomeTestParams::engine_name_str()
                );
                assert_eq!(
                    maker_order
                        .engine_details
                        .trade_engine_specifics
                        .test_specific_field,
                    SomeTestParams::engine_specific_str()
                );
                assert_eq!(maker_order.pow_difficulty, 0);
            }
            Err(error) => {
                panic!(
                    "maker_order_builder_build failed on builder.build() - {}",
                    error.to_string()
                );
            }
        }
    }

    // Helper Functions

    fn new_event_msg_client() -> ArcClient {
        let client = Client::new();
        Arc::new(Mutex::new(client))
    }
}
