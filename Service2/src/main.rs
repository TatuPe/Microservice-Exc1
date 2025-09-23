use axum::{routing::get, Router};
use sysinfo::{Disks, System};
use std::{env};

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

    // Reports uptime in seconds, divide to hours
    // Note: Reports system uptime, which in Docker equals to app uptime.
    let uptime = System::uptime() / 3600;

    let disks = Disks::new_with_refreshed_list();
    let mut space= 0;
    for disk in disks.list() {
        if disk.mount_point().to_str().unwrap() == "/" {
            // Reports disk space in bytes, divide to Mega bytes
            space = disk.available_space()/1000000;
            break;
        }
    }
    
    format!("Timestamp2: uptime {} hours, free disk in root: {} Mbytes", uptime, space)
}