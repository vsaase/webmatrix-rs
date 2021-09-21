use matrix_sdk::{
    // config::SyncSettings,
    ruma::UserId,
    Client,
};
use std::convert::TryFrom;

pub async fn main() {
    let user = UserId::try_from("@vsaase_test:matrix.org").unwrap();
    let _client = Client::new_from_user_id(user).await.unwrap();
}
