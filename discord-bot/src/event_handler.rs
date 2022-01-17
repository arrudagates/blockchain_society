use crate::primitives::{Error, MemberBanned};
use codec::Decode;
use subxt::{ClientBuilder, DefaultConfig, DefaultExtra, EventSubscription};
use tokio::spawn;

#[subxt::subxt(runtime_metadata_path = "../blockchain/metadata.scale")]
pub mod polkadot {}

type MemberAdded = polkadot::discord::events::MemberAdded;
type RoleCreated = polkadot::discord::events::RoleCreated;
type RoleAssigned = polkadot::discord::events::RoleAssigned;

pub async fn handler() -> Result<(), Error> {
    env_logger::init();

    let api: polkadot::RuntimeApi<DefaultConfig, DefaultExtra<DefaultConfig>> =
        ClientBuilder::new()
            .set_url("ws://127.0.0.1:9944")
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
            match sub.next().await {
                Some(sub_result) => match sub_result {
                    Ok(raw) => {
                        if raw.pallet == String::from("Discord") {
                            match raw.variant.as_str() {
                                "BotAdded" => println!("{:?}", raw),
                                "MemberBanned" => member_banned(
                                    <MemberBanned as Decode>::decode(&mut &raw.data[..]).unwrap(),
                                )
                                .await
                                .unwrap(),
                                "RoleCreated" => role_created(
                                    <RoleCreated as Decode>::decode(&mut &raw.data[..]).unwrap(),
                                )
                                .await
                                .unwrap(),
                                "RoleAssigned" => role_assigned(
                                    <RoleAssigned as Decode>::decode(&mut &raw.data[..]).unwrap(),
                                )
                                .await
                                .unwrap(),
                                "MemberAdded" => member_added(
                                    <MemberAdded as Decode>::decode(&mut &raw.data[..]).unwrap(),
                                )
                                .await
                                .unwrap(),
                                _ => todo!(),
                            }
                        }
                    }
                    Err(why) => eprintln!("{:?}", Error::Subxt(why)),
                },
                _ => eprintln!(
                    "{:?}",
                    Error::Custom(String::from("Empty raw event received from socket"))
                ),
            }
        }
    });

    Ok(())
}

async fn member_banned(event: MemberBanned) -> Result<(), Error> {
    println!("{:?}", event.0);
    Ok(())
}

async fn member_added(event: MemberAdded) -> Result<(), Error> {
    println!("{:?}", event.0);
    Ok(())
}

async fn role_created(event: RoleCreated) -> Result<(), Error> {
    println!("{:?}", event.0);
    Ok(())
}

async fn role_assigned(event: RoleAssigned) -> Result<(), Error> {
    println!("{:?}", event.0);
    Ok(())
}
