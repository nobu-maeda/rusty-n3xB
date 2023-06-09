use crate::order::MakerOrderBuilder;
use crate::{nostr::*, order::TradeEngineSpecfiicsTrait};
use serde::Serialize;

use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

pub struct Manager<EngineSpecificsType: TradeEngineSpecfiicsTrait + Clone + Serialize> {
    event_msg_client: ArcClient,
    subscription_client: ArcClient,
    // TODO: Local DB
    _phantom_engine_specifics: PhantomData<EngineSpecificsType>,
}

impl<EngineSpecificsType: TradeEngineSpecfiicsTrait + Clone + Serialize>
    Manager<EngineSpecificsType>
{
    // Public Functions

    // Constructors

    pub async fn new_with_keys(keys: Keys) -> Self {
        Manager {
            event_msg_client: Self::new_nostr_client(&keys).await,
            subscription_client: Self::new_nostr_client(&keys).await,
            // TODO: Create Local DB
            _phantom_engine_specifics: PhantomData,
        }
    }

    pub async fn new() -> Self {
        let keys = Keys::generate();

        Manager {
            event_msg_client: Self::new_nostr_client(&keys).await,
            subscription_client: Self::new_nostr_client(&keys).await,
            // TODO: Create Local DB
            _phantom_engine_specifics: PhantomData,
        }
    }

    pub fn new_with_nostr(event_msg_client: Client, subscription_client: Client) -> Self {
        Manager {
            event_msg_client: Arc::new(Mutex::new(event_msg_client)),
            subscription_client: Arc::new(Mutex::new(subscription_client)),
            // TODO: Create Local DB
            _phantom_engine_specifics: PhantomData,
        }
    }

    // Order Management

    pub fn build_maker_order(&self) -> MakerOrderBuilder<EngineSpecificsType> {
        MakerOrderBuilder::new(&self.event_msg_client)
    }

    // Private Functions

    async fn new_nostr_client(keys: &Keys) -> ArcClient {
        let opts = Options::new()
            .wait_for_connection(true)
            .wait_for_send(true)
            .difficulty(8);
        let client = Client::with_opts(&keys, opts);

        client.add_relay("ws://localhost:8008", None).await.unwrap(); // TODO: Should add to existing list of relay, or default relay list, vs localhost test mode?
        client.connect().await;
        Arc::new(Mutex::new(client))
    }
}
