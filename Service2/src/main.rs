use axum::{routing::get, Router};
use sysinfo::{Disks, System};
use std::{env};
use chrono::prelude::*;
use tokio::fs;

const VSTORAGE: &str = "/vstorage";

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
        .route("/status", get(get_status));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();
    println!("Running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn get_status() -> String {
    println!("GET status");

    // Reports uptime in seconds, divide to hours
    // Note: Reports system uptime, which in Docker equals to app uptime.
    let uptime = System::uptime() / 3600;
    let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%S.%3fZ");

    let disks = Disks::new_with_refreshed_list();
    let mut space= 0;
    for disk in disks.list() {
        if disk.mount_point().to_str().unwrap() == "/" {
            // Reports disk space in bytes, divide to Mega bytes
            space = disk.available_space()/1000000;
            break;
        }
    }
    
    let status = format!("{}: uptime {} hours, free disk in root: {} Mbytes", timestamp, uptime, space);

    write_vstorage(status.clone()).await;

    let client = reqwest::Client::new();
    let res = client.post("http://storage:3002/log")
                    .body(status.clone())
                    .send()
                    .await;

    match res {
        Ok(res) => println!("{}", res.text().await.unwrap()),
        Err(e) => println!("{}", e)
    }
    
    return status;
}

async fn write_vstorage(status: String) {
    let f: String = match fs::read_to_string(VSTORAGE).await {
                            Ok(file) => file,
                            Err(_) => "".to_string()
                        };

    let log = format!("{}{}\n", f, status);

    match fs::write(VSTORAGE, log).await {
        Ok(_) => println!("vStorage write success!"),
        Err(e) => println!("vStorage error: {} {}", VSTORAGE, e)
    };
}