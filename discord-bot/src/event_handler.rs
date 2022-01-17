use crate::primitives::Error;
use subxt::{ClientBuilder, DefaultConfig, DefaultExtra, EventSubscription};
use tokio::spawn;

#[subxt::subxt(runtime_metadata_path = "../blockchain/metadata.scale")]
pub mod polkadot {}

pub async fn handler() -> Result<(), Error> {
    env_logger::init();

    let api: polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>> =
        ClientBuilder::new()
            .set_url("ws://127.0.0.1:9944")
            .build()
            .await
            .map_err(|e| Error::Client(e.to_string()))?
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
            let raw = sub.next().await.unwrap().unwrap();
            if raw.pallet == String::from("Discord") {
                todo!()
            }
        }
    });

    Ok(())
}
