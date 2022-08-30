use actix_http::body::BoxBody;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

use serde::Serialize;

const USER_AGENT: &str = "Connectivity-Checker";

#[derive(Serialize)]
struct Check {
    status: String,
}

impl Check {
    pub fn new(status: String) -> Self {
        Self { status }
    }
}

impl Responder for Check {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        HttpResponse::Ok().json(self)
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

#[get("/check/ubuntu")]
async fn check_ubuntu() -> impl Responder {
    let result = get("http://connectivity-check.ubuntu.com").await;

    let status = match result {
        Ok(response) => response
            .headers()
            .get("x-networkmanager-status")
            .map_or("missing-status", |v| {
                v.to_str().unwrap_or("status-not-string")
            })
            .to_owned(),
        Err(err) => {
            format!("{}", err)
        }
    };

    Check::new(status)
}

#[get("/check/gnome")]
async fn check_gnome() -> impl Responder {
    let result = get("https://nmcheck.gnome.org/check_network_status.txt").await;

    let status = match result {
        Ok(mut response) => match response.body().await {
            Ok(body) => if body == "NetworkManager is online\n" {
                "online"
            } else {
                println!("{:?}", body);
                "unknown-body"
            }
            .to_owned(),
            Err(err) => {
                format!("{}", err)
            }
        },
        Err(err) => {
            format!("{}", err)
        }
    };

    Check::new(status)
}

#[get("/check/fedora")]
async fn check_fedora() -> impl Responder {
    let result = get("https://fedoraproject.org/static/hotspot.txt").await;

    let status = match result {
        Ok(mut response) => match response.body().await {
            Ok(body) => if body == "OK" {
                "online"
            } else {
                println!("{:?}", body);
                "unknown-body"
            }
            .to_owned(),
            Err(err) => {
                format!("{}", err)
            }
        },
        Err(err) => {
            format!("{}", err)
        }
    };

    Check::new(status)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(check_ubuntu)
            .service(check_gnome)
            .service(check_fedora)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
