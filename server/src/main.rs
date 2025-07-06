use actix_cors::Cors;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Deserialize)]
struct Config {
    uptimerobot: UptimeRobotConfig,
}

#[derive(Deserialize)]
struct UptimeRobotConfig {
    api_token: String,
    monitor_id: String,
}

#[derive(Serialize, Deserialize)]
struct UptimeResponse {
    status: String,
    duration: u64,
    ratio: Option<f64>,
}

#[get("/api/uptime")]
async fn get_uptime() -> impl Responder {
    let config: Config = toml::from_str(
        &fs::read_to_string("config.toml").expect("Failed to read config.toml"),
    )
    .expect("Failed to parse config.toml");

    let client = Client::new();
    let response = client
        .get(format!(
            "https://api.uptimerobot.com/v3/monitors/{}",
            config.uptimerobot.monitor_id
        ))
        .bearer_auth(config.uptimerobot.api_token.as_str())
        .send()
        .await;

    match response {
        Ok(resp) => {
            let json: serde_json::Value =
                resp.json().await.expect("Failed to parse JSON");

            HttpResponse::Ok().json(UptimeResponse {
                status: json
                    .get("status")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_string(),
                duration: json
                    .get("currentStateDuration")
                    .unwrap()
                    .as_u64()
                    .unwrap(),
                ratio: 100.0.into(),
            })
        }

        Err(_) => HttpResponse::InternalServerError().json(UptimeResponse {
            status: "Failed".to_string(),
            duration: 0,
            ratio: None,
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin() // For development; replace with specific origin in production
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        actix_web::http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .service(get_uptime)
            .service(
                actix_files::Files::new("/", "../client/dist")
                    .index_file("index.html"),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
