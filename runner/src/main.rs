use actix_rt::time;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use serde::Deserialize;

use std::time::Duration;

const USER_AGENT: &str = "Connectivity-Runner";

#[derive(Debug, Deserialize)]
struct Check {
    status: String,
}

#[derive(FromPrimitive)]
enum Command {
    Ubuntu,
    Gnome,
    Fedora,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("starting...");

    let mut counter: usize = 0;

    let mut interval = time::interval(Duration::from_secs(10));

    loop {
        interval.tick().await;
        let os = match Command::from_usize(counter % 3).expect("Enum out of range") {
            Command::Ubuntu => "ubuntu",
            Command::Gnome => "gnome",
            Command::Fedora => "fedora",
        };

        println!("request {}", os);

        let url = format!("http://checker:8080/check/{}", os);

        let result = get(&url).await;

        let status = match result {
            Ok(mut response) => match response.json::<Check>().await {
                Ok(check) => check.status,
                Err(err) => format!("{:?}", err),
            },
            Err(err) => {
                format!("{}", err)
            }
        };

        println!("{}: {}", os, status);

        counter += 1;
    }
}

fn get(url: &str) -> awc::SendClientRequest {
    let client = awc::Client::new();

    client
        .get(url)
        .append_header(("User-Agent", USER_AGENT))
        .timeout(std::time::Duration::from_secs(30))
        .send()
}
