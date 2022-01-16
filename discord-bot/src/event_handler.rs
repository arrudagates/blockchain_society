use crate::primitives::Error;
use sp_keyring::AccountKeyring;
use subxt::{ClientBuilder, DefaultConfig, DefaultExtra, EventSubscription, PairSigner};
use tokio::spawn;

type Config = polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>;

#[subxt::subxt(runtime_metadata_path = "../blockchain/metadata.scale")]
pub mod polkadot {}

pub async fn handler() -> Result<(), Error> {
    env_logger::init();
    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id();

    let api: polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>> =
        ClientBuilder::new()
            .set_url("wss://rpc.polkadot.io:443")
            .build()
            .await
            .map_err(|e| Error::Client(e.to_string()))?
            .to_runtime_api();

    let sub = api
        .client
        .rpc()
        .subscribe_events()
        .await
        .map_err(Error::Subxt)?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<DefaultConfig>::new(sub, decoder);
    sub.filter_event::<polkadot::balances::events::Transfer>();

    spawn(async move {
        loop {
            let raw = sub.next().await.unwrap().unwrap();
        }
    });
    Ok(())
}
