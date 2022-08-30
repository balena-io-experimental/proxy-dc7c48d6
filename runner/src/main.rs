use actix_rt::time;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use std::time::Duration;

#[derive(FromPrimitive)]
enum Command {
    CheckUbuntu,
    CheckGnome,
    CheckFedora,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut counter: usize = 0;

    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        println!("Awaiting Tick...");
        interval.tick().await;

        match Command::from_usize(counter % 3).expect("Enum out of range") {
            Command::CheckUbuntu => {
                println!("Check Ubuntu");
            }
            Command::CheckGnome => {
                println!("Check Gnome");
            }
            Command::CheckFedora => {
                println!("Check Fedora");
            }
        };

        counter += 1;
    }
}
