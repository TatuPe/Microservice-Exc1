use axum::{routing::{get, post}, Router};
use tokio::fs;
use std::{env};

const LOGFILE: &str = "log.txt";

fn get_port() -> i32 {
    let port = match env::var("PORT").map(|port| port.parse::<i32>()) {
        Ok(port) => port,
        Err(e) => panic!("{}", e)
    };

    return port.unwrap();
}

#[tokio::main]
async fn main() {
    let port = get_port();

    let app = Router::new()
        .route("/log", get(get_log))
        .route("/log", post(post_log));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    println!("Running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_log() -> String {
    println!("GET log");
    let f: String = match fs::read_to_string(LOGFILE).await {
                            Ok(file) => file,
                            Err(e) => return format!("Could not open log!\n{}", e)
                        };

    return f;
}

async fn post_log(req: String) -> String {
    println!("POST log");
    let f: String = match fs::read_to_string(LOGFILE).await {
                            Ok(file) => file,
                            Err(_) => "".to_string()
                        };
    
    let log = format!("{}{}\n", f, req);

    match fs::write(LOGFILE, log).await {
        Ok(_) => return format!("Request OK!"),
        Err(e) => return format!("Error: {}", e)
    };
}