use std::{thread, io};
use rs_futu_api::tcpclient::Client;

#[actix_web::main]
async fn main() {
    let client = Client::connect("127.0.0.1:11111").await;
    thread::spawn(move || loop {
        let mut cmd = String::new();
        if io::stdin().read_line(&mut cmd).is_err() {
            println!("error");
            return;
        }
    });
}
