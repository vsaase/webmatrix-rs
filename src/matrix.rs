use matrix_sdk::{
    config::SyncSettings,
    ruma::{
        events::{room::message::MessageEventContent, SyncMessageEvent},
        UserId,
    },
    Client, Result,
};
use std::convert::TryFrom;

pub async fn main() {
    let user = UserId::try_from("@vsaase_test:matrix.org").unwrap();
    let client = Client::new_from_user_id(&user).await.unwrap();

    // First we need to log in.
    client.login(user, "vsaase_test", None, None).await.unwrap();

    client
        .register_event_handler(|ev: SyncMessageEvent<MessageEventContent>| async move {
            println!("Received a message {:?}", ev);
        })
        .await;

    // Syncing is important to synchronize the client state with the server.
    // This method will never return.
    // client.sync(SyncSettings::default()).await;
}
