// Copyright 2019-2022 Parity Technologies (UK) Ltd.
// This file is part of subxt.
//
// subxt is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// subxt is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with subxt.  If not, see <http://www.gnu.org/licenses/>.

//! To run this example, a local polkadot node should be running.
//!
//! E.g.
//! ```bash
//! curl "https://github.com/paritytech/polkadot/releases/download/v0.9.11/polkadot" --output /usr/local/bin/polkadot --location
//! polkadot --dev --tmp
//! ```

use sp_keyring::AccountKeyring;
use subxt::{
    ClientBuilder,
    DefaultConfig,
    DefaultExtra,
    EventSubscription,
    PairSigner,
};

#[subxt::subxt(runtime_metadata_path = "examples/polkadot_metadata.scale")]
pub mod polkadot {}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let signer = PairSigner::new(AccountKeyring::Alice.pair());
    let dest = AccountKeyring::Bob.to_account_id().into();

    let api = ClientBuilder::new()
        .build()
        .await?
        .to_runtime_api::<polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>>>();

    let sub = api.client.rpc().subscribe_events().await?;
    let decoder = api.client.events_decoder();
    let mut sub = EventSubscription::<DefaultConfig>::new(sub, decoder);
    sub.filter_event::<polkadot::balances::events::Transfer>();

    api.tx()
        .balances()
        .transfer(dest, 10_000)
        .sign_and_submit(&signer)
        .await?;

    let raw = sub.next().await.unwrap().unwrap();
    let event = <polkadot::balances::events::Transfer as codec::Decode>::decode(
        &mut &raw.data[..],
    );
    if let Ok(e) = event {
        println!("Balance transfer success: value: {:?}", e.2);
    } else {
        println!("Failed to subscribe to Balances::Transfer Event");
    }
    Ok(())
}
