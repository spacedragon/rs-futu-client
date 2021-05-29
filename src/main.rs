use rs_futu_api::client::Client;
use actix::System;
use actix_rt::time::sleep;
use std::time::Duration;

fn main() {
    env_logger::init();

    System::new().block_on(async {
        let client = Client::connect("127.0.0.1:11111").await;
        sleep(Duration::from_secs(20)).await;
    });
}
