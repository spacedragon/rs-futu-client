use rs_futu_api::client::Client;
use actix::System;
use actix_rt::time::sleep;
use std::time::Duration;
use rs_futu_api::commands::GetGlobalState;

fn main() {
    env_logger::init();

    System::new().block_on(async {
        let mut client = Client::connect("127.0.0.1:11111").await;

        let get_global_state = GetGlobalState {
            user_id: 0
        };
        if let Ok(Some(resp)) = client.send_command(get_global_state).await {
            let resp = resp.s2c.unwrap();
            log::info!("global state is {:?}", resp);
        };

        sleep(Duration::from_secs(20)).await;
    });
}
