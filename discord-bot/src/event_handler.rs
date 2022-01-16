use crate::primitives::Error;
use subxt::{ClientBuilder, DefaultConfig, DefaultExtra, EventSubscription};
use tokio::spawn;

#[subxt::subxt(runtime_metadata_path = "../blockchain/metadata.scale")]
pub mod polkadot {}

pub async fn handler() -> Result<(), Error> {
    env_logger::init();

    let api: polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>> =
        ClientBuilder::new()
            .set_url("wss://rpc.polkadot.io:443")
            .build()
            .await
            .map_err(Error::Subxt)?
            .to_runtime_api();

    spawn(async move {
        let sub = api
            .client
            .rpc()
            .subscribe_events()
            .await
            .map_err(Error::Subxt)
            .unwrap();
        let decoder = api.client.events_decoder();
        let mut sub = EventSubscription::<DefaultConfig>::new(sub, decoder);
        loop {
            let _raw = sub.next().await.unwrap().unwrap();
        }
    });

    Ok(())
}
